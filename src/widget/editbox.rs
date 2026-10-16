//! EditBox presentation retained between dirty-state flushes.

use super::{Frame, TextJustify};

#[derive(Debug, Clone, Copy, PartialEq)]
pub(crate) struct EditBoxLayoutSettings {
    pub font_size: f32,
    pub width: f32,
    pub height: f32,
    pub insets: (f32, f32, f32, f32),
    pub word_wrap: bool,
    pub justify_h: TextJustify,
    pub justify_v: TextJustify,
    pub spacing: f32,
    pub smooth_scaling: bool,
    pub scale: f32,
}

impl EditBoxLayoutSettings {
    fn from_frame(frame: &Frame, size: (f32, f32)) -> Self {
        Self {
            font_size: frame.font_size * frame.effective_scale,
            width: size.0,
            height: size.1,
            insets: frame.editbox_text_insets,
            word_wrap: frame.editbox_multi_line && frame.word_wrap && !frame.width_is_text_auto,
            justify_h: frame.justify_h,
            justify_v: frame.justify_v,
            spacing: frame.text_line_spacing * frame.effective_scale,
            smooth_scaling: frame.font_string_smooth_scaling,
            scale: frame.effective_scale,
        }
    }

    /// Preserve the existing EditBox content padding used by the renderer.
    pub fn content_bounds(self) -> (f32, f32, f32, f32) {
        let (left, right, top, bottom) = self.insets;
        let left = if left > 0.0 { left } else { 4.0 };
        let right = if right > 0.0 { right } else { 4.0 };
        (
            left,
            top,
            (self.width - left - right).max(0.0),
            (self.height - top - bottom).max(0.0),
        )
    }
}

#[derive(Debug)]
pub(crate) struct EditBoxLayoutInput {
    pub text: Option<String>,
    pub stripped_text: String,
    pub font: Option<String>,
    pub cursor: usize,
    pub settings: EditBoxLayoutSettings,
}

impl EditBoxLayoutInput {
    pub fn from_frame(frame: &Frame, size: (f32, f32)) -> Self {
        let text = frame.text.clone();
        let stripped_text = crate::render::strip_wow_markup(text.as_deref().unwrap_or_default());
        Self {
            text,
            stripped_text,
            font: frame.font.clone(),
            cursor: frame.editbox_cursor_pos.max(0) as usize,
            settings: EditBoxLayoutSettings::from_frame(frame, size),
        }
    }

    pub fn matches_frame(&self, frame: &Frame, size: (f32, f32)) -> bool {
        let same_text = self.text == frame.text;
        let same_font = self.font == frame.font;
        let same_cursor = self.cursor == frame.editbox_cursor_pos.max(0) as usize;
        let same_settings = self.settings == EditBoxLayoutSettings::from_frame(frame, size);
        same_text && same_font && same_cursor && same_settings
    }

    pub fn storage_estimate_bytes(&self) -> usize {
        self.text.as_ref().map_or(0, String::capacity)
            + self.stripped_text.capacity()
            + self.font.as_ref().map_or(0, String::capacity)
    }
}

#[derive(Debug, Default)]
pub struct EditBoxPresentation {
    pub(crate) input: Option<EditBoxLayoutInput>,
    /// Frame-local x/y/width/height; y is negative below the frame's top edge.
    pub(crate) cursor_geometry: Option<(f32, f32, f32, f32)>,
    text_changed: bool,
    user_input: bool,
}

impl EditBoxPresentation {
    pub(crate) fn mark_text_changed(&mut self, user_input: Option<bool>) {
        self.text_changed = true;
        if let Some(user_input) = user_input {
            self.user_input = user_input;
        }
    }

    pub(crate) fn take_text_change(&mut self) -> Option<bool> {
        if !self.text_changed {
            return None;
        }
        self.text_changed = false;
        Some(std::mem::take(&mut self.user_input))
    }

    pub fn storage_estimate_bytes(&self) -> usize {
        std::mem::size_of::<Self>()
            + self
                .input
                .as_ref()
                .map_or(0, EditBoxLayoutInput::storage_estimate_bytes)
    }
}

impl Frame {
    pub(crate) fn editbox_presentation_mut(&mut self) -> &mut EditBoxPresentation {
        self.editbox_presentation
            .get_or_insert_with(Default::default)
    }

    pub(crate) fn mark_editbox_text_changed(&mut self, user_input: Option<bool>) {
        self.editbox_presentation_mut()
            .mark_text_changed(user_input);
    }
}
