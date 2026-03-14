//! Thin App wrappers over the unified dump module.

use super::app::App;

impl App {
    /// Dump WoW frames for debug server (compact format with warnings).
    pub(crate) fn dump_wow_frames(&self) -> String {
        let env = self.env.borrow();
        let state = env.state().borrow();
        let screen_width = self.screen_size.get().width;
        let screen_height = self.screen_size.get().height;
        let addon_names: Vec<String> = state.addons.iter().map(|a| a.folder_name.clone()).collect();
        crate::dump::build_warning_dump(&state.widgets, &addon_names, screen_width, screen_height)
            .join("\n")
    }

    /// Build a frame tree dump with computed layout rects (for connected dump-tree).
    pub(crate) fn build_frame_tree_dump(&self, filter: Option<&str>, visible_only: bool) -> String {
        let env = self.env.borrow();
        let state = env.state().borrow();
        let screen_width = self.screen_size.get().width;
        let screen_height = self.screen_size.get().height;
        let addon_names: Vec<String> = state.addons.iter().map(|a| a.folder_name.clone()).collect();
        let lines = crate::dump::build_tree(
            &state.widgets,
            &addon_names,
            filter,
            None,
            visible_only,
            screen_width,
            screen_height,
        );
        if lines.is_empty() {
            "No frames found".to_string()
        } else {
            lines.join("\n")
        }
    }
}
