//! Text measurement and rendering using iced canvas.
//!
//! This module provides text rendering with proper alignment for WoW UI frames.
//! Text measurement is handled by iced's text rendering via cosmic-text.

use iced::widget::canvas::{self, Frame};
use iced::{Color, Font, Pixels, Point, Rectangle, alignment};

use crate::widget::TextJustify;

/// Default WoW UI font (Friz Quadrata).
pub const WOW_FONT_DEFAULT: Font = Font::DEFAULT;

/// Text renderer with alignment capabilities.
pub struct TextRenderer;

impl TextRenderer {
    /// Draw text on a canvas frame with proper centering.
    ///
    /// Uses iced's built-in text centering via align_x and align_y.
    pub fn draw_centered_text(
        frame: &mut Frame,
        text: &str,
        bounds: Rectangle,
        font_size: f32,
        color: Color,
        font: Font,
    ) {
        if text.is_empty() {
            return;
        }

        // Position at center of bounds, let iced handle alignment
        let center = Point::new(
            bounds.x + bounds.width / 2.0,
            bounds.y + bounds.height / 2.0,
        );

        frame.fill_text(canvas::Text {
            content: text.to_string(),
            position: center,
            color,
            size: Pixels(font_size),
            line_height: iced::widget::text::LineHeight::default(),
            font,
            align_x: alignment::Horizontal::Center.into(),
            align_y: alignment::Vertical::Center,
            shaping: iced::widget::text::Shaping::Advanced,
            max_width: f32::INFINITY,
        });
    }

    /// Draw text on a canvas frame with WoW-style justification.
    pub fn draw_justified_text(frame: &mut Frame, text: JustifiedText<'_>) {
        if text.content.is_empty() {
            return;
        }

        // Convert WoW justification to iced alignment
        let (align_x, x_pos) = match text.justify_h {
            TextJustify::Left => (alignment::Horizontal::Left, text.bounds.x),
            TextJustify::Center => (
                alignment::Horizontal::Center,
                text.bounds.x + text.bounds.width / 2.0,
            ),
            TextJustify::Right => (
                alignment::Horizontal::Right,
                text.bounds.x + text.bounds.width,
            ),
        };

        let (align_y, y_pos) = match text.justify_v {
            TextJustify::Left => (alignment::Vertical::Top, text.bounds.y), // TOP
            TextJustify::Center => (
                alignment::Vertical::Center,
                text.bounds.y + text.bounds.height / 2.0,
            ), // MIDDLE
            TextJustify::Right => (
                alignment::Vertical::Bottom,
                text.bounds.y + text.bounds.height,
            ), // BOTTOM
        };

        frame.fill_text(canvas::Text {
            content: text.content.to_string(),
            position: Point::new(x_pos, y_pos),
            color: text.color,
            size: Pixels(text.font_size),
            line_height: iced::widget::text::LineHeight::default(),
            font: text.font,
            align_x: align_x.into(),
            align_y,
            shaping: iced::widget::text::Shaping::Advanced,
            max_width: text.bounds.width,
        });
    }
}

pub struct JustifiedText<'a> {
    pub content: &'a str,
    pub bounds: Rectangle,
    pub font_size: f32,
    pub color: Color,
    pub font: Font,
    pub justify_h: TextJustify,
    pub justify_v: TextJustify,
}

/// Map WoW font paths to system fonts.
/// Returns a Font that iced can use.
pub fn wow_font_to_iced(font_path: Option<&str>) -> Font {
    // For now, use the default font
    // In the future, we could load custom fonts via iced's font loading
    match font_path {
        Some(path) => {
            let path_upper = path.to_uppercase();
            if path_upper.contains("MONO") {
                Font::MONOSPACE
            } else {
                Font::DEFAULT
            }
        }
        None => Font::DEFAULT,
    }
}

/// Strip WoW markup from text: textures (`|T...|t`), atlases (`|A...|a`),
/// colors (`|cXXXXXXXX`/`|r`), and hyperlinks (`|H...|h`/`|h`).
/// Preserves plain text content visible to the player.
pub fn strip_wow_markup(text: &str) -> String {
    let mut result = String::with_capacity(text.len());
    let mut chars = text.chars().peekable();

    while let Some(c) = chars.next() {
        if c == '|' && consume_escape(&mut chars) {
            continue;
        }
        result.push(c);
    }

    result
}

/// Try to consume a WoW escape sequence after `|`. Returns true if consumed.
fn consume_escape(chars: &mut std::iter::Peekable<std::str::Chars>) -> bool {
    let Some(&next) = chars.peek() else {
        return false;
    };
    match next {
        'T' | 'A' => skip_delimited_span(chars, if next == 'T' { 't' } else { 'a' }),
        'H' => skip_delimited_span(chars, 'h'),
        'h' | 'r' => { chars.next(); true }
        'c' => { chars.next(); skip_n(chars, 8); true }
        _ => false,
    }
}

/// Skip from current position to `|{end_marker}` (e.g. `|T...|t`).
fn skip_delimited_span(chars: &mut std::iter::Peekable<std::str::Chars>, end_marker: char) -> bool {
    chars.next(); // consume the opening letter (T, A, H)
    while let Some(ch) = chars.next() {
        if ch == '|' && chars.peek() == Some(&end_marker) {
            chars.next();
            return true;
        }
    }
    true // consumed the opening, even if unclosed
}

/// Skip N characters.
fn skip_n(chars: &mut std::iter::Peekable<std::str::Chars>, n: usize) {
    for _ in 0..n {
        chars.next();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn plain_text_unchanged() {
        assert_eq!(strip_wow_markup("Hello World"), "Hello World");
    }

    #[test]
    fn strips_color_codes() {
        assert_eq!(strip_wow_markup("|cFF00FF00Green|r text"), "Green text");
    }

    #[test]
    fn strips_inline_texture() {
        assert_eq!(strip_wow_markup("Icon|TInterface\\Icons\\Spell:16|tEnd"), "IconEnd");
    }

    #[test]
    fn strips_inline_atlas() {
        assert_eq!(strip_wow_markup("Before|Aatlasname|aAfter"), "BeforeAfter");
    }

    #[test]
    fn strips_hyperlink_keeps_text() {
        assert_eq!(
            strip_wow_markup("|Hitem:12345|hCool Sword|h"),
            "Cool Sword"
        );
    }

    #[test]
    fn strips_nested_color_in_hyperlink() {
        assert_eq!(
            strip_wow_markup("|cFF0070DD|Hitem:123|h[Blade]|h|r"),
            "[Blade]"
        );
    }

    #[test]
    fn empty_string() {
        assert_eq!(strip_wow_markup(""), "");
    }

    #[test]
    fn lone_pipe_preserved() {
        assert_eq!(strip_wow_markup("a|b"), "a|b");
    }

    #[test]
    fn pipe_at_end_preserved() {
        assert_eq!(strip_wow_markup("text|"), "text|");
    }
}
