#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MouseTileState {
    pub active: bool,
    pub subtitle: &'static str,
}

impl MouseTileState {
    pub fn new(active: bool) -> Self {
        Self {
            active,
            subtitle: if active { "Active" } else { "Inactive" },
        }
    }
}
