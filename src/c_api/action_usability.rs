//! Supplied spell-action usability; no resource or range inference.

/// Host-supplied classification used by action-button presentation.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum ActionUsability {
    #[default]
    Usable,
    InsufficientPower,
    Unusable,
}
