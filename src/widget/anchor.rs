//! Anchor system for widget positioning.

/// Anchor points on a widget (matches WoW's anchor system).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum AnchorPoint {
    #[default]
    Center,
    Top,
    Bottom,
    Left,
    Right,
    TopLeft,
    TopRight,
    BottomLeft,
    BottomRight,
}

impl AnchorPoint {
    #[allow(clippy::should_implement_trait)]
    pub fn from_str(s: &str) -> Option<Self> {
        Some(if s.eq_ignore_ascii_case("CENTER") {
            Self::Center
        } else if s.eq_ignore_ascii_case("TOP") {
            Self::Top
        } else if s.eq_ignore_ascii_case("BOTTOM") {
            Self::Bottom
        } else if s.eq_ignore_ascii_case("LEFT") {
            Self::Left
        } else if s.eq_ignore_ascii_case("RIGHT") {
            Self::Right
        } else if s.eq_ignore_ascii_case("TOPLEFT") || s.eq_ignore_ascii_case("TOPELFT") {
            Self::TopLeft
        } else if s.eq_ignore_ascii_case("TOPRIGHT") {
            Self::TopRight
        } else if s.eq_ignore_ascii_case("BOTTOMLEFT") {
            Self::BottomLeft
        } else if s.eq_ignore_ascii_case("BOTTOMRIGHT") {
            Self::BottomRight
        } else {
            return None;
        })
    }

    /// WoW canonical sort order for GetPoint indexing.
    pub fn sort_key(&self) -> u8 {
        match self {
            Self::TopLeft => 0,
            Self::Top => 1,
            Self::TopRight => 2,
            Self::Left => 3,
            Self::Center => 4,
            Self::Right => 5,
            Self::BottomLeft => 6,
            Self::Bottom => 7,
            Self::BottomRight => 8,
        }
    }

    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Center => "CENTER",
            Self::Top => "TOP",
            Self::Bottom => "BOTTOM",
            Self::Left => "LEFT",
            Self::Right => "RIGHT",
            Self::TopLeft => "TOPLEFT",
            Self::TopRight => "TOPRIGHT",
            Self::BottomLeft => "BOTTOMLEFT",
            Self::BottomRight => "BOTTOMRIGHT",
        }
    }

    /// Horizontal anchor factor: 0.0 for left edge, 0.5 for center, 1.0 for right edge.
    pub fn horizontal_factor(&self) -> f32 {
        match self {
            Self::TopLeft | Self::Left | Self::BottomLeft => 0.0,
            Self::Top | Self::Center | Self::Bottom => 0.5,
            Self::TopRight | Self::Right | Self::BottomRight => 1.0,
        }
    }

    pub fn pins_left_edge(&self) -> bool {
        matches!(self, Self::TopLeft | Self::Left | Self::BottomLeft)
    }

    pub fn pins_right_edge(&self) -> bool {
        matches!(self, Self::TopRight | Self::Right | Self::BottomRight)
    }
}

/// An anchor defines how a widget is positioned relative to another widget.
#[derive(Debug, Clone)]
pub struct Anchor {
    /// The point on this widget to anchor.
    pub point: AnchorPoint,
    /// The widget name to anchor to (used for XML parsing, None = parent).
    pub relative_to: Option<String>,
    /// The widget ID to anchor to (used for Lua API, takes precedence over name).
    pub relative_to_id: Option<usize>,
    /// The point on the relative widget to anchor to.
    pub relative_point: AnchorPoint,
    /// X offset from the anchor point.
    pub x_offset: f32,
    /// Y offset from the anchor point.
    pub y_offset: f32,
}

impl Anchor {
    pub fn from_relative_id(
        point: AnchorPoint,
        relative_to_id: Option<usize>,
        relative_point: AnchorPoint,
    ) -> Self {
        Self {
            point,
            relative_to: None,
            relative_to_id,
            relative_point,
            x_offset: 0.0,
            y_offset: 0.0,
        }
    }
}

impl Default for Anchor {
    fn default() -> Self {
        Self {
            point: AnchorPoint::TopLeft,
            relative_to: None,
            relative_to_id: None,
            relative_point: AnchorPoint::TopLeft,
            x_offset: 0.0,
            y_offset: 0.0,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::AnchorPoint;

    #[test]
    fn anchor_point_accepts_plumber_topelft_typo() {
        assert_eq!(AnchorPoint::from_str("TOPELFT"), Some(AnchorPoint::TopLeft));
    }
}
