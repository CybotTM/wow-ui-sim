//! MessageFrame / ScrollingMessageFrame state data structures.

use std::collections::{HashMap, VecDeque};

/// A single message in a MessageFrame.
#[derive(Clone)]
pub struct Message {
    pub text: String,
    pub r: f32,
    pub g: f32,
    pub b: f32,
    pub a: f32,
    pub message_id: Option<i64>,
    /// Wrath chat-history grouping, distinct from the line identifier.
    pub access_id: Option<i64>,
    /// HistoryKeeper type token used to recover the transfer color.
    pub extra_data: Option<i64>,
    /// GetTime() value when the message was added (for fading calculations).
    pub timestamp: f64,
    /// Native Wrath MessageFrame row lifetime; scrolling chat keeps timestamp fading.
    pub wrath_timers: Option<MessageRowTimers>,
}

#[derive(Clone)]
pub struct MessageRowTimers {
    pub time_left: f64,
    pub fade_left: f64,
}

impl MessageRowTimers {
    fn advance(&mut self, elapsed: f64) {
        let elapsed = elapsed.max(0.0);
        let visible_elapsed = elapsed.min(self.time_left.max(0.0));
        self.time_left = (self.time_left - visible_elapsed).max(0.0);
        self.fade_left = (self.fade_left - (elapsed - visible_elapsed)).max(0.0);
    }
}

/// State for a MessageFrame or ScrollingMessageFrame.
pub struct MessageFrameData {
    pub messages: Vec<Message>,
    pub pending_wrath_messages: VecDeque<Message>,
    pub max_lines: usize,
    pub fading: bool,
    pub time_visible: f64,
    pub fade_duration: f64,
    pub fade_power: f64,
    pub insert_mode: String, // "TOP" or "BOTTOM"
    pub scroll_offset: i32,
    pub scroll_allowed: bool,
    pub text_copyable: bool,
    pub display_dirty: bool,
    pub override_fade_timestamp: f64,
    pub message_font_strings: HashMap<i64, u64>,
}

impl MessageFrameData {
    pub(crate) fn insert_message(&mut self, message: Message) {
        if self.insert_mode == "TOP" {
            self.messages.insert(0, message);
        } else {
            self.messages.push(message);
        }
    }

    fn update_wrath_rows(&mut self, elapsed: f64) {
        if self.fading {
            for message in &mut self.messages {
                if let Some(timers) = &mut message.wrath_timers {
                    timers.advance(elapsed);
                }
            }
        }
        if let Some(mut message) = self.pending_wrath_messages.pop_front() {
            message.wrath_timers = Some(MessageRowTimers {
                time_left: self.time_visible,
                fade_left: self.fade_duration,
            });
            self.insert_message(message);
            if self.messages.len() > self.max_lines {
                self.messages.remove(if self.insert_mode == "TOP" {
                    self.max_lines
                } else {
                    0
                });
            }
        }
    }
}

pub(crate) fn update_wrath_message_frames(sim: &mut crate::lua_api::SimState, elapsed: f64) {
    if crate::client_profile::ACTIVE != crate::client_profile::ClientProfile::Wrath {
        return;
    }
    for (&id, data) in &mut sim.message_frames {
        if sim
            .widgets
            .get(id)
            .is_some_and(|frame| frame.widget_type == crate::widget::WidgetType::MessageFrame)
        {
            data.update_wrath_rows(elapsed);
            sim.widgets.mark_visual_dirty(id);
        }
    }
}

impl Default for MessageFrameData {
    fn default() -> Self {
        Self {
            messages: Vec::new(),
            pending_wrath_messages: VecDeque::new(),
            max_lines: 120,
            fading: true,
            time_visible: 10.0,
            fade_duration: 3.0,
            fade_power: 1.0,
            insert_mode: "BOTTOM".to_string(),
            scroll_offset: 0,
            scroll_allowed: true,
            text_copyable: false,
            display_dirty: false,
            override_fade_timestamp: 0.0,
            message_font_strings: HashMap::new(),
        }
    }
}
