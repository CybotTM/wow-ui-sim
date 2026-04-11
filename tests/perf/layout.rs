use std::time::{Duration, Instant};

use wow_ui_sim::lua_api::WowLuaEnv;

pub fn measure_full_root_layout_pass(env: &WowLuaEnv) -> Duration {
    let ui_parent_id = {
        let state = env.state().borrow();
        state
            .widgets
            .get_id_by_name("UIParent")
            .expect("UIParent should exist in the settled game UI")
    };

    let started = Instant::now();
    {
        let mut state = env.state().borrow_mut();
        state.widgets.mark_rect_dirty(ui_parent_id);
        state.invalidate_layout(ui_parent_id);
    }
    started.elapsed()
}
