//! Profile-independent font measurement arithmetic.
const LINE_HEIGHT_MULTIPLIER: f32 = 1.2;

pub fn line_height_for_font_size(font_size: f32, smooth_scaling: bool) -> Option<f32> {
    if !font_size.is_finite() || font_size <= 0.0 {
        return None;
    }
    let height = font_size * LINE_HEIGHT_MULTIPLIER;
    Some(if smooth_scaling {
        height
    } else {
        height.ceil().max(1.0)
    })
}

pub fn spaced_text_height(base_height: f32, line_count: usize, spacing: f32) -> f32 {
    base_height + line_count.saturating_sub(1) as f32 * spacing
}
