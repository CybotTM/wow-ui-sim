//! Frame-related enums: FrameStrata, DrawLayer.

/// Frame strata (draw order).
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Default)]
pub enum FrameStrata {
    World,
    Background,
    Low,
    #[default]
    Medium,
    High,
    Dialog,
    Fullscreen,
    FullscreenDialog,
    Tooltip,
}

impl FrameStrata {
    /// Number of strata variants (for fixed-size arrays/vecs).
    pub const COUNT: usize = 9;

    /// Convert to index (0..COUNT) for array indexing.
    pub fn as_index(self) -> usize {
        self as usize
    }

    #[allow(clippy::should_implement_trait)]
    pub fn from_str(s: &str) -> Option<Self> {
        Some(if s.eq_ignore_ascii_case("WORLD") {
            Self::World
        } else if s.eq_ignore_ascii_case("BACKGROUND") {
            Self::Background
        } else if s.eq_ignore_ascii_case("LOW") {
            Self::Low
        } else if s.eq_ignore_ascii_case("MEDIUM") {
            Self::Medium
        } else if s.eq_ignore_ascii_case("HIGH") {
            Self::High
        } else if s.eq_ignore_ascii_case("DIALOG") {
            Self::Dialog
        } else if s.eq_ignore_ascii_case("FULLSCREEN") {
            Self::Fullscreen
        } else if s.eq_ignore_ascii_case("FULLSCREEN_DIALOG") {
            Self::FullscreenDialog
        } else if s.eq_ignore_ascii_case("TOOLTIP") {
            Self::Tooltip
        } else {
            return None;
        })
    }

    pub fn as_str(&self) -> &'static str {
        match self {
            Self::World => "WORLD",
            Self::Background => "BACKGROUND",
            Self::Low => "LOW",
            Self::Medium => "MEDIUM",
            Self::High => "HIGH",
            Self::Dialog => "DIALOG",
            Self::Fullscreen => "FULLSCREEN",
            Self::FullscreenDialog => "FULLSCREEN_DIALOG",
            Self::Tooltip => "TOOLTIP",
        }
    }
}

/// Draw layer for regions (textures/fontstrings) within a frame.
/// Determines render order: BACKGROUND < BORDER < ARTWORK < OVERLAY < HIGHLIGHT
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Default)]
pub enum DrawLayer {
    Background = 1,
    Border = 2,
    #[default]
    Artwork = 3,
    Overlay = 4,
    Highlight = 5,
}

impl DrawLayer {
    #[allow(clippy::should_implement_trait)]
    pub fn from_str(s: &str) -> Option<Self> {
        Some(if s.eq_ignore_ascii_case("BACKGROUND") {
            Self::Background
        } else if s.eq_ignore_ascii_case("BORDER") {
            Self::Border
        } else if s.eq_ignore_ascii_case("ARTWORK") {
            Self::Artwork
        } else if s.eq_ignore_ascii_case("OVERLAY") {
            Self::Overlay
        } else if s.eq_ignore_ascii_case("HIGHLIGHT") {
            Self::Highlight
        } else {
            return None;
        })
    }

    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Background => "BACKGROUND",
            Self::Border => "BORDER",
            Self::Artwork => "ARTWORK",
            Self::Overlay => "OVERLAY",
            Self::Highlight => "HIGHLIGHT",
        }
    }
}
