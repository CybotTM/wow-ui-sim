//! Profile-neutral widget types, identities and geometry.
pub mod atlas;
pub mod widget;

/// Blend mode for quad rendering.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[repr(u32)]
pub enum BlendMode {
    /// Standard alpha blending: src * alpha + dst * (1 - alpha)
    #[default]
    Alpha = 0,
    /// Additive blending: src + dst (for highlight textures)
    Additive = 1,
}

/// Computed layout position for a frame.
#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub struct LayoutRect {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
}
