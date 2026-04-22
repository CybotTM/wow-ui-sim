//! Minimal global-frame helpers that still have live callers.

use crate::lua_api::SimState;
use crate::lua_api::methods::borrow_state_mut;
use rilua::LuaApi;

const STARTUP_HIDDEN_FRAME_NAMES: &[&str] = &["QuestInfoRequiredMoneyFrame", "QuestInfoGroupSize"];

fn hide_named_frame(sim: &mut SimState, frame_name: &str) {
    let Some(frame_id) = sim.widgets.get_id_by_name(frame_name) else {
        return;
    };
    if let Some(frame) = sim.widgets.get_mut(frame_id) {
        frame.visible = false;
    }
}

pub fn hide_runtime_hidden_frames<T>(lua: T) -> crate::Result<()>
where
    T: std::ops::Deref<Target = rilua::Lua>,
{
    let mut sim = borrow_state_mut(lua.state())?;
    for frame_name in STARTUP_HIDDEN_FRAME_NAMES {
        hide_named_frame(&mut sim, frame_name);
    }
    Ok(())
}
