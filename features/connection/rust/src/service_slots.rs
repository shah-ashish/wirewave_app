use gpui::AnyElement;

/// Abstraction allowing application shells to inject active service tiles into Connected Home.
///
/// Decouples `features/connection` from specific feature crates, enforcing the monorepo rule:
/// *"features may depend on core, NEVER on other features."*
pub trait ActiveServiceSlots: 'static {
    fn render_speaker_tile(&self) -> AnyElement;
    fn render_mic_tile(&self) -> AnyElement;
    fn render_mouse_tile(&self) -> AnyElement;
    fn render_camera_tile(&self) -> AnyElement;
    fn stop_all(&self);
    fn is_any_running(&self) -> bool;
}
