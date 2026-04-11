//! Tooltip state data structures.

use std::sync::OnceLock;

use regex::Regex;

use crate::widget::{Anchor, AnchorPoint};

/// Inline texture/atlas icon embedded in a tooltip line.
#[derive(Clone)]
pub enum TooltipTexture {
    FileDataId(u32),
    Atlas(String),
}

/// A single line in a tooltip.
#[derive(Clone)]
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
    pub allow_show_with_no_lines: bool,
    pub frame_stack_index: usize,
    pub shrink_to_fit_wrapped: bool,
    pub anchor_x_offset: f32,
    pub anchor_y_offset: f32,
    pub custom_word_wrap_min_width: Option<f32>,
    pub min_width: f32,
    pub padding: f32,
    /// Spell ID set by `SetSpellByID`, returned by `GetSpell`.
    pub spell_id: Option<u32>,
    /// Custom line spacing set by `SetCustomLineSpacing` (default: 2px).
    pub line_spacing: Option<f32>,
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
            allow_show_with_no_lines: false,
            frame_stack_index: 0,
            shrink_to_fit_wrapped: true,
            anchor_x_offset: 0.0,
            anchor_y_offset: 0.0,
            custom_word_wrap_min_width: None,
            min_width: 0.0,
            padding: 0.0,
            spell_id: None,
            line_spacing: None,
            left_line_ids: Vec::new(),
            right_line_ids: Vec::new(),
        }
    }
}

pub(crate) const DEFAULT_CURSOR_Y_OFFSET: f32 = 20.0;

pub(crate) fn build_cursor_anchor(mx: f32, my: f32, x_offset: f32, y_offset: f32) -> Anchor {
    let cursor_y = if x_offset == 0.0 && y_offset == 0.0 {
        my + DEFAULT_CURSOR_Y_OFFSET
    } else {
        my + y_offset
    };
    Anchor {
        point: AnchorPoint::TopLeft,
        relative_to: None,
        relative_to_id: None,
        relative_point: AnchorPoint::TopLeft,
        x_offset: mx + x_offset,
        y_offset: cursor_y,
    }
}

pub(crate) fn parse_item_id_from_hyperlink(link: &str) -> Option<u32> {
    parse_hyperlink_id(link, &["item:"])
}

pub(crate) fn parse_spell_id_from_hyperlink(link: &str) -> Option<u32> {
    parse_hyperlink_id(link, &["Hspell:", "spell:"])
}

pub(crate) fn strip_html_tags(html: &str) -> String {
    let mut result = String::with_capacity(html.len());
    let mut in_tag = false;
    for ch in html.chars() {
        match ch {
            '<' => in_tag = true,
            '>' => in_tag = false,
            _ if !in_tag => result.push(ch),
            _ => {}
        }
    }
    result
}

pub(crate) fn format_spell_tooltip_description(spell_id: u32, description: &str) -> String {
    let replaced = replace_spell_amount_placeholders(spell_id, description);
    let without_conditionals = strip_trailing_conditional_branch(&replaced);
    let without_inline_formulas = strip_inline_formula_placeholders(&without_conditionals);
    let without_simple_tokens = strip_simple_placeholders(&without_inline_formulas);
    normalize_tooltip_spacing(&without_simple_tokens)
}

fn replace_spell_amount_placeholders(spell_id: u32, description: &str) -> String {
    static EFFECT_PLACEHOLDER_RE: OnceLock<Regex> = OnceLock::new();

    let amount = crate::lua_api::game_data::spell_effect_amount(spell_id).to_string();
    EFFECT_PLACEHOLDER_RE
        .get_or_init(|| Regex::new(r"\$s\d+").unwrap())
        .replace_all(description, amount.as_str())
        .into_owned()
}

fn strip_trailing_conditional_branch(description: &str) -> String {
    match description.find("$?") {
        Some(index) => description[..index].to_string(),
        None => description.to_string(),
    }
}

fn strip_inline_formula_placeholders(description: &str) -> String {
    static INLINE_FORMULA_RE: OnceLock<Regex> = OnceLock::new();

    INLINE_FORMULA_RE
        .get_or_init(|| Regex::new(r"\$\{[^}]+\}").unwrap())
        .replace_all(description, "")
        .into_owned()
}

fn strip_simple_placeholders(description: &str) -> String {
    static SIMPLE_PLACEHOLDER_RE: OnceLock<Regex> = OnceLock::new();

    SIMPLE_PLACEHOLDER_RE
        .get_or_init(|| Regex::new(r"\$[A-Za-z<][A-Za-z0-9<>]*").unwrap())
        .replace_all(description, "")
        .into_owned()
}

fn normalize_tooltip_spacing(description: &str) -> String {
    static SPACE_BEFORE_PUNCTUATION_RE: OnceLock<Regex> = OnceLock::new();
    static MULTI_SPACE_RE: OnceLock<Regex> = OnceLock::new();

    let without_space_before_punctuation = SPACE_BEFORE_PUNCTUATION_RE
        .get_or_init(|| Regex::new(r"\s+([,.;:])").unwrap())
        .replace_all(description, "$1")
        .into_owned();

    MULTI_SPACE_RE
        .get_or_init(|| Regex::new(r"\s{2,}").unwrap())
        .replace_all(without_space_before_punctuation.trim(), " ")
        .into_owned()
}

fn parse_hyperlink_id(link: &str, prefixes: &[&str]) -> Option<u32> {
    let (start, prefix) = prefixes
        .iter()
        .find_map(|prefix| link.find(prefix).map(|start| (start, *prefix)))?;
    let after = &link[start + prefix.len()..];
    let end = after
        .find(|c: char| c == ':' || c == '|')
        .unwrap_or(after.len());
    after[..end].parse::<u32>().ok()
}
