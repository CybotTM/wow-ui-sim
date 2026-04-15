//! Lifecycle script firing for XML-created frames (OnLoad, OnShow).

use crate::lua_api::LoaderEnv;
use crate::lua_api::rilua_methods::{call_function_state, frame_ref, table_get};
use crate::lua_api::rilua_script_helpers::{collect_lua_error, get_script};
use rilua::Val;

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
pub fn fire_lifecycle_scripts(env: &LoaderEnv<'_>, name: &str, lifecycle: LifecycleScripts) {
    let Some(frame_id) = env.state().borrow().widgets.get_id_by_name(name) else {
        return;
    };

    let _ = env.with_state(|state| {
        if lifecycle.on_load {
            fire_intrinsic_handler(state, frame_id, "OnLoad_Intrinsic");
            fire_script_handler(state, frame_id, "OnLoad");
        }
        if lifecycle.on_show && is_frame_visible(state, frame_id) {
            fire_script_handler(state, frame_id, "OnShow");
            fire_intrinsic_handler(state, frame_id, "OnShow_Intrinsic");
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

fn fire_intrinsic_handler(state: &mut rilua::vm::state::LuaState, frame_id: u64, key: &str) {
    let Ok(frame) = frame_ref(state, frame_id) else {
        return;
    };
    let handler = table_get(state, frame, key);
    call_handler(state, frame_id, handler, key);
}

fn fire_script_handler(state: &mut rilua::vm::state::LuaState, frame_id: u64, handler_name: &str) {
    let Some(handler) = get_script(state, frame_id, handler_name) else {
        return;
    };
    call_handler(state, frame_id, handler, handler_name);
}

fn call_handler(
    state: &mut rilua::vm::state::LuaState,
    frame_id: u64,
    handler: Val,
    handler_name: &str,
) {
    if !matches!(handler, Val::Function(_)) {
        return;
    }

    let Ok(frame) = frame_ref(state, frame_id) else {
        return;
    };
    let frame_name = frame_display_name(state, frame_id);
    let result = call_function_state(state, handler, &[frame]);

    if let Err(error) = result {
        let message = format!("[{handler_name}] {frame_name}: {error}");
        if collect_lua_error(state, &message) {
            eprintln!("Lua error: {message}");
        }
    }
}

fn frame_display_name(state: &rilua::vm::state::LuaState, frame_id: u64) -> String {
    let Ok(sim) = crate::lua_api::rilua_methods::borrow_state(state) else {
        return format!("frame#{frame_id}");
    };
    sim.widgets
        .get(frame_id)
        .and_then(|frame| frame.name.clone())
        .unwrap_or_else(|| format!("frame#{frame_id}"))
}
