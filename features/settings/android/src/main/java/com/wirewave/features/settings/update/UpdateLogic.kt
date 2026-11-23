package com.wirewave.features.settings.update

import com.wirewave.core.config.AppConfig
import com.wirewave.core.config.BuildChannel
import com.wirewave.core.updater.ApkInstaller
import com.wirewave.core.updater.DownloadOutcome
import com.wirewave.core.updater.InstallOutcome
import com.wirewave.core.updater.UpdateCheckOutcome
import com.wirewave.core.updater.UpdateDownloader
import com.wirewave.core.updater.UpdateManifestClient
import com.wirewave.core.updater.UpdateRelease
import kotlinx.coroutines.CoroutineScope
import kotlinx.coroutines.flow.MutableStateFlow
import kotlinx.coroutines.flow.StateFlow
import kotlinx.coroutines.flow.asStateFlow
import kotlinx.coroutines.flow.update
import kotlinx.coroutines.launch
import java.io.File

sealed interface UpdatePhase {
    data object Idle : UpdatePhase
    data object Checking : UpdatePhase
    data object UpToDate : UpdatePhase
    data class Available(val release: UpdateRelease) : UpdatePhase
    data class Downloading(val versionName: String, val percent: Int) : UpdatePhase
    data class ReadyToInstall(val versionName: String, val apkFile: File) : UpdatePhase
    data class Failed(val message: String, val retry: RetryTarget) : UpdatePhase
}

sealed interface RetryTarget {
    data object Check : RetryTarget
    data class Download(val release: UpdateRelease) : RetryTarget
    data class Install(val versionName: String, val apkFile: File) : RetryTarget
    data object OpenStore : RetryTarget
}

enum class UpdateAction {
    CHECK,
    DOWNLOAD,
    INSTALL,
    OPEN_STORE,
    RETRY
}

data class UpdateActionSpec(
    val label: String,
    val action: UpdateAction
)

class UpdateController(
    private val scope: CoroutineScope,
    private val manifestClient: UpdateManifestClient,
    private val downloader: UpdateDownloader,
    private val installer: ApkInstaller
) {
    private val _phase = MutableStateFlow<UpdatePhase>(UpdatePhase.Idle)
    val phase: StateFlow<UpdatePhase> = _phase.asStateFlow()

    fun primaryActionFor(phase: UpdatePhase): UpdateActionSpec? = when (AppConfig.buildChannel) {
        BuildChannel.STORE -> UpdateActionSpec("Check for updates", UpdateAction.OPEN_STORE)
        BuildChannel.DEV -> when (phase) {
            UpdatePhase.Idle -> UpdateActionSpec("Check for updates", UpdateAction.CHECK)
            UpdatePhase.UpToDate -> UpdateActionSpec("Up to date · Check again", UpdateAction.CHECK)
            is UpdatePhase.Available -> UpdateActionSpec("Download update", UpdateAction.DOWNLOAD)
            is UpdatePhase.ReadyToInstall -> UpdateActionSpec("Install update", UpdateAction.INSTALL)
            is UpdatePhase.Failed -> UpdateActionSpec("Retry", UpdateAction.RETRY)
            UpdatePhase.Checking, is UpdatePhase.Downloading -> null
        }
    }

    fun dispatch(action: UpdateAction) {
        when (action) {
            UpdateAction.CHECK -> checkForUpdates()
            UpdateAction.DOWNLOAD -> downloadAvailableUpdate()
            UpdateAction.INSTALL -> installDownloadedUpdate()
            UpdateAction.OPEN_STORE -> openStore()
            UpdateAction.RETRY -> retryFailedAction()
        }
    }

    private fun checkForUpdates() {
        _phase.update { UpdatePhase.Checking }
        scope.launch {
            val next = when (val outcome = manifestClient.checkForUpdate()) {
                UpdateCheckOutcome.UpToDate -> UpdatePhase.UpToDate
                is UpdateCheckOutcome.Available -> UpdatePhase.Available(outcome.release)
                is UpdateCheckOutcome.Failure -> UpdatePhase.Failed(outcome.message, RetryTarget.Check)
            }
            _phase.update { next }
        }
    }

    private fun downloadAvailableUpdate() {
        val release = (_phase.value as? UpdatePhase.Available)?.release ?: return
        _phase.update { UpdatePhase.Downloading(release.versionName, percent = 0) }
        scope.launch {
            val outcome = downloader.download(release) { pct ->
                _phase.update { current ->
                    when (current) {
                        is UpdatePhase.Downloading -> current.copy(percent = pct)
                        else -> current
                    }
                }
            }
            val next = when (outcome) {
                is DownloadOutcome.Success -> UpdatePhase.ReadyToInstall(release.versionName, outcome.apkFile)
                is DownloadOutcome.Failure -> UpdatePhase.Failed(
                    outcome.message,
                    RetryTarget.Download(release)
                )
            }
            _phase.update { next }
        }
    }

    private fun installDownloadedUpdate() {
        val ready = _phase.value as? UpdatePhase.ReadyToInstall ?: return
        when (val outcome = installer.installApk(ready.apkFile)) {
            InstallOutcome.Launched -> Unit
            InstallOutcome.NeedsInstallPermission -> _phase.update {
                UpdatePhase.Failed(
                    "Enable 'Install unknown apps' permission, then tap Install again.",
                    RetryTarget.Install(ready.versionName, ready.apkFile)
                )
            }
            is InstallOutcome.Failure -> _phase.update {
                UpdatePhase.Failed(
                    outcome.message,
                    RetryTarget.Install(ready.versionName, ready.apkFile)
                )
            }
        }
    }

    private fun openStore() {
        when (val outcome = installer.openStoreListing()) {
            InstallOutcome.Launched, InstallOutcome.NeedsInstallPermission -> Unit
            is InstallOutcome.Failure -> _phase.update {
                UpdatePhase.Failed(outcome.message, RetryTarget.OpenStore)
            }
        }
    }

    private fun retryFailedAction() {
        val retry = (_phase.value as? UpdatePhase.Failed)?.retry ?: return
        when (retry) {
            RetryTarget.Check -> checkForUpdates()
            is RetryTarget.Download -> {
                _phase.update { UpdatePhase.Available(retry.release) }
                downloadAvailableUpdate()
            }
            is RetryTarget.Install -> {
                _phase.update { UpdatePhase.ReadyToInstall(retry.versionName, retry.apkFile) }
                installDownloadedUpdate()
            }
            RetryTarget.OpenStore -> openStore()
        }
    }
}
