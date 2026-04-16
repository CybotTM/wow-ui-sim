//! Lifecycle script firing for XML-created frames (OnLoad, OnShow).

use crate::loader::precompiled;
use crate::lua_api::LoaderEnv;
use crate::lua_api::rilua_methods::frame_ref;
use crate::lua_api::rilua_script_helpers::collect_lua_error;

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub(super) struct LifecycleScripts {
    pub(super) on_load: bool,
    pub(super) on_show: bool,
}

impl LifecycleScripts {
    pub(super) const fn any(self) -> bool {
        self.on_load || self.on_show
    }
}

/// Fire OnLoad and OnShow after XML creation has finished wiring children and properties.
pub fn fire_lifecycle_scripts(
    env: &LoaderEnv<'_>,
    frame_id: u64,
    name: &str,
    lifecycle: LifecycleScripts,
) {
    let _ = env.with_state(|state| {
        let Ok(frame) = frame_ref(state, frame_id) else {
            return Ok::<(), crate::Error>(());
        };
        if lifecycle.on_load {
            fire_handler(state, name, "OnLoad", precompiled::fire_onload, frame);
        }
        if lifecycle.on_show && is_frame_visible(state, frame_id) {
            fire_handler(state, name, "OnShow", precompiled::fire_onshow, frame);
        }
        Ok::<(), crate::Error>(())
    });
}

fn is_frame_visible(state: &rilua::vm::state::LuaState, frame_id: u64) -> bool {
    let Ok(sim) = crate::lua_api::rilua_methods::borrow_state(state) else {
        return false;
    };
    sim.widgets.is_ancestor_visible(frame_id)
}

fn fire_handler(
    state: &mut rilua::vm::state::LuaState,
    frame_name: &str,
    handler_name: &str,
    fire: fn(&mut rilua::vm::state::LuaState, rilua::Val) -> rilua::LuaResult<()>,
    frame: rilua::Val,
) {
    if let Err(error_text) = fire(state, frame) {
        let message = format!("[{handler_name}] {frame_name}: {error_text}");
        if collect_lua_error(state, &message) {
            eprintln!("Lua error: {message}");
        }
    }
}
