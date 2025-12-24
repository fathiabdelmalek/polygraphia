#[derive(Default, Debug, Clone, Copy, PartialEq, Eq)]
pub enum TextMode {
    #[default]
    PreserveAll,
    AlphaOnly,
}
