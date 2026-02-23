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
        match s.to_uppercase().as_str() {
            "WORLD" => Some(Self::World),
            "BACKGROUND" => Some(Self::Background),
            "LOW" => Some(Self::Low),
            "MEDIUM" => Some(Self::Medium),
            "HIGH" => Some(Self::High),
            "DIALOG" => Some(Self::Dialog),
            "FULLSCREEN" => Some(Self::Fullscreen),
            "FULLSCREEN_DIALOG" => Some(Self::FullscreenDialog),
            "TOOLTIP" => Some(Self::Tooltip),
            _ => None,
        }
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
        match s.to_uppercase().as_str() {
            "BACKGROUND" => Some(Self::Background),
            "BORDER" => Some(Self::Border),
            "ARTWORK" => Some(Self::Artwork),
            "OVERLAY" => Some(Self::Overlay),
            "HIGHLIGHT" => Some(Self::Highlight),
            _ => None,
        }
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
