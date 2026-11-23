use gpui::{Context, Render};
use std::{
    path::PathBuf,
    sync::{
        atomic::{AtomicU8, Ordering},
        Arc,
    },
};
use wirewave_core_config::{BuildChannel, APP_CONFIG};
use wirewave_core_updater::{
    check_for_update, download_update, launch_installer, open_store_listing, UpdateCheckResult,
    UpdateError, UpdateRelease,
};

fn failed_phase(error: UpdateError, retry: RetryTarget) -> UpdatePhase {
    eprintln!("Updater action failed: {error}");
    UpdatePhase::Failed {
        message: error.user_message().to_string(),
        retry,
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RetryTarget {
    Check,
    Download(UpdateRelease),
    Install {
        version_name: String,
        installer_path: PathBuf,
    },
    OpenStore,
}

/// Finite state machine for the manual in-app updater.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum UpdatePhase {
    Idle,
    Checking,
    UpToDate,
    Available(UpdateRelease),
    Downloading {
        version_name: String,
        percent: u8,
    },
    ReadyToInstall {
        version_name: String,
        installer_path: PathBuf,
    },
    Failed {
        message: String,
        retry: RetryTarget,
    },
}

/// User-triggered intents mapped directly from `UpdatePhase`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UpdateAction {
    Check,
    Download,
    Install,
    OpenStore,
    Retry,
}

pub struct UpdateController {
    pub phase: UpdatePhase,
}

impl Default for UpdateController {
    fn default() -> Self {
        Self {
            phase: UpdatePhase::Idle,
        }
    }
}

impl UpdateController {
    /// Resolves the primary clickable action for the current phase & build channel.
    pub fn primary_action(&self) -> Option<(&'static str, UpdateAction)> {
        match (APP_CONFIG.build_channel, &self.phase) {
            (BuildChannel::Store, _) => Some(("Check for updates", UpdateAction::OpenStore)),
            (BuildChannel::Dev, UpdatePhase::Idle) => {
                Some(("Check for updates", UpdateAction::Check))
            }
            (BuildChannel::Dev, UpdatePhase::UpToDate) => {
                Some(("Up to date · Check again", UpdateAction::Check))
            }
            (BuildChannel::Dev, UpdatePhase::Available(_)) => {
                Some(("Download update", UpdateAction::Download))
            }
            (BuildChannel::Dev, UpdatePhase::ReadyToInstall { .. }) => {
                Some(("Install update", UpdateAction::Install))
            }
            (BuildChannel::Dev, UpdatePhase::Failed { .. }) => {
                Some(("Retry", UpdateAction::Retry))
            }
            (BuildChannel::Dev, UpdatePhase::Checking | UpdatePhase::Downloading { .. }) => None,
        }
    }

    pub fn trigger_check<V: Render + HasUpdateController>(&mut self, cx: &mut Context<V>) {
        if matches!(APP_CONFIG.build_channel, BuildChannel::Store) {
            self.open_store();
            return;
        }

        self.phase = UpdatePhase::Checking;
        cx.notify();

        cx.spawn(async move |this, cx| {
            let outcome = cx
                .background_executor()
                .spawn(async { check_for_update() })
                .await;

            let next_phase = match outcome {
                Ok(UpdateCheckResult::UpToDate) => UpdatePhase::UpToDate,
                Ok(UpdateCheckResult::Available(release)) => UpdatePhase::Available(release),
                Err(err) => failed_phase(err, RetryTarget::Check),
            };

            let _ = this.update(cx, |view, cx| {
                view.updater_mut().phase = next_phase;
                cx.notify();
            });
        })
        .detach();
    }

    pub fn trigger_download<V: Render + HasUpdateController>(&mut self, cx: &mut Context<V>) {
        let UpdatePhase::Available(release) = self.phase.clone() else {
            return;
        };

        let version_name = release.version_name.clone();
        self.phase = UpdatePhase::Downloading {
            version_name: version_name.clone(),
            percent: 0,
        };
        cx.notify();

        let progress_pct = Arc::new(AtomicU8::new(0));
        let progress_writer = Arc::clone(&progress_pct);

        cx.spawn(async move |this, cx| {
            let retry_release = release.clone();
            let mut download_task = cx.background_executor().spawn(async move {
                download_update(&release, |progress| {
                    progress_writer.store(progress.percent(), Ordering::Relaxed);
                })
            });

            // Poll progress smoothly on the foreground timer until the background download completes.
            let poll_handle = this.clone();
            let poll_pct = Arc::clone(&progress_pct);
            let poll_version = version_name.clone();
            let poller = cx.spawn(async move |cx| {
                loop {
                    cx.background_executor()
                        .timer(std::time::Duration::from_millis(120))
                        .await;
                    let pct = poll_pct.load(Ordering::Relaxed);
                    let keep_going = poll_handle
                        .update(cx, |view, cx| {
                            if let UpdatePhase::Downloading { percent, .. } =
                                &mut view.updater_mut().phase
                            {
                                if *percent != pct {
                                    *percent = pct;
                                    cx.notify();
                                }
                                return true;
                            }
                            false
                        })
                        .unwrap_or(false);
                    if !keep_going {
                        break;
                    }
                    let _ = &poll_version;
                }
            });

            let outcome = (&mut download_task).await;
            drop(poller);

            let next_phase = match outcome {
                Ok(installer_path) => UpdatePhase::ReadyToInstall {
                    version_name,
                    installer_path,
                },
                Err(err) => failed_phase(err, RetryTarget::Download(retry_release)),
            };

            let _ = this.update(cx, |view, cx| {
                view.updater_mut().phase = next_phase;
                cx.notify();
            });
        })
        .detach();
    }

    pub fn trigger_install(&mut self) {
        let (version_name, installer_path) = match &self.phase {
            UpdatePhase::ReadyToInstall {
                version_name,
                installer_path,
            } => (version_name.clone(), installer_path.clone()),
            _ => return,
        };
        if let Err(err) = launch_installer(&installer_path) {
            self.phase = failed_phase(
                err,
                RetryTarget::Install {
                    version_name,
                    installer_path,
                },
            );
        }
    }

    pub fn open_store(&mut self) {
        if let Err(err) = open_store_listing() {
            self.phase = failed_phase(err, RetryTarget::OpenStore);
        }
    }

    pub fn retry<V: Render + HasUpdateController>(&mut self, cx: &mut Context<V>) {
        let retry = match &self.phase {
            UpdatePhase::Failed { retry, .. } => retry.clone(),
            _ => return,
        };
        match retry {
            RetryTarget::Check => self.trigger_check(cx),
            RetryTarget::Download(release) => {
                self.phase = UpdatePhase::Available(release);
                self.trigger_download(cx);
            }
            RetryTarget::Install {
                version_name,
                installer_path,
            } => {
                self.phase = UpdatePhase::ReadyToInstall {
                    version_name,
                    installer_path,
                };
                self.trigger_install();
                cx.notify();
            }
            RetryTarget::OpenStore => {
                self.open_store();
                cx.notify();
            }
        }
    }
}

/// Trait implemented by any GPUI view hosting an `UpdateController`.
pub trait HasUpdateController: 'static {
    fn updater_mut(&mut self) -> &mut UpdateController;
}
