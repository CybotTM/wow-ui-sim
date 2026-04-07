//! Tooltip state data structures.

/// Inline texture/atlas icon embedded in a tooltip line.
pub enum TooltipTexture {
    FileDataId(u32),
    Atlas(String),
}

/// A single line in a tooltip.
pub struct TooltipLine {
    pub left_text: String,
    pub left_color: (f32, f32, f32),
    pub right_text: Option<String>,
    pub right_color: (f32, f32, f32),
    pub wrap: bool,
    /// Inline texture icon (from `AddTexture` / `AddAtlas`).
    pub texture: Option<TooltipTexture>,
}

/// State for a GameTooltip frame.
pub struct TooltipData {
    pub lines: Vec<TooltipLine>,
    pub owner_id: Option<u64>,
    pub anchor_type: String,
    pub min_width: f32,
    pub padding: f32,
    /// Spell ID set by `SetSpellByID`, returned by `GetSpell`.
    pub spell_id: Option<u32>,
    /// Widget IDs for left-side FontString children (`{Name}TextLeft{N}`).
    pub left_line_ids: Vec<u64>,
    /// Widget IDs for right-side FontString children (`{Name}TextRight{N}`).
    pub right_line_ids: Vec<u64>,
}

impl Default for TooltipData {
    fn default() -> Self {
        Self {
            lines: Vec::new(),
            owner_id: None,
            anchor_type: "ANCHOR_NONE".to_string(),
            min_width: 0.0,
            padding: 0.0,
            spell_id: None,
            left_line_ids: Vec::new(),
            right_line_ids: Vec::new(),
        }
    }
}
