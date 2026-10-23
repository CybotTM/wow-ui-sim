//! Physical history slots for the Wrath EditBox engine.

use super::Frame;

impl Frame {
    pub(crate) fn resize_wrath_history(&mut self, capacity: usize) {
        self.editbox_history.resize(capacity, None);
        self.editbox_history_cursor = self.editbox_history_cursor.min(capacity.saturating_sub(1));
        self.editbox_history_max = capacity as i32;
    }

    pub(crate) fn append_wrath_history(&mut self, text: String) {
        let capacity = self.editbox_history_max as usize;
        if capacity == 0 {
            return;
        }
        self.editbox_history[self.editbox_history_cursor] = Some(text);
        self.editbox_history_cursor = (self.editbox_history_cursor + 1) % capacity;
    }

    pub(crate) fn clear_wrath_history(&mut self) {
        self.editbox_history.fill(None);
        self.editbox_history_cursor = 0;
    }

    pub(crate) fn recall_wrath_history(&mut self, backward: bool) -> Option<String> {
        let capacity = self.editbox_history_max as usize;
        if capacity == 0 {
            return None;
        }
        let mut candidate = self.editbox_history_cursor;
        for _ in 0..capacity {
            candidate = if backward {
                (candidate + capacity - 1) % capacity
            } else {
                (candidate + 1) % capacity
            };
            if let Some(text) = &self.editbox_history[candidate] {
                self.editbox_history_cursor = candidate;
                return Some(text.clone());
            }
        }
        None
    }
}
