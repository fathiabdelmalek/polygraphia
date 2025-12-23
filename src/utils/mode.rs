#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TextMode {
    AlphaOnly,
    PreserveAll,
}

impl Default for TextMode {
    fn default() -> Self {
        TextMode::PreserveAll
    }
}
