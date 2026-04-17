//! Lifecycle script firing for XML-created frames (OnLoad, OnShow).

use crate::lua_api::LoaderEnv;
use crate::lua_api::rilua_methods::{frame_ref, table_get_static};
use crate::lua_api::rilua_script_helpers::{
    call_void_function_with_fallback_state, collect_lua_error, get_script,
};
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

const ONLOAD_INTRINSIC_KEY: &str = "OnLoad_Intrinsic";
const ONSHOW_INTRINSIC_KEY: &str = "OnShow_Intrinsic";

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
            fire_intrinsic_handler(state, ONLOAD_INTRINSIC_KEY, "OnLoad_Intrinsic", frame);
            fire_script_handler(state, frame_id, name, "OnLoad", frame);
        }
        if lifecycle.on_show && is_frame_visible(state, frame_id) {
            fire_script_handler(state, frame_id, name, "OnShow", frame);
            fire_intrinsic_handler(state, ONSHOW_INTRINSIC_KEY, "OnShow_Intrinsic", frame);
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

fn fire_script_handler(
    state: &mut rilua::vm::state::LuaState,
    frame_id: u64,
    frame_name: &str,
    handler_name: &str,
    frame: Val,
) {
    let Some(handler) = get_script(state, frame_id, handler_name) else {
        return;
    };
    if let Err(error_text) = call_void_function_with_fallback_state(state, handler, &[frame]) {
        let message = format!("[{handler_name}] {frame_name}: {error_text}");
        if collect_lua_error(state, &message) {
            eprintln!("Lua error: {message}");
        }
    }
}

fn fire_intrinsic_handler(
    state: &mut rilua::vm::state::LuaState,
    field_name: &'static str,
    error_label: &str,
    frame: Val,
) {
    let handler = table_get_static(state, frame, field_name);
    let Val::Function(_) = handler else {
        return;
    };
    if let Err(error_text) = call_void_function_with_fallback_state(state, handler, &[frame]) {
        let message = format!("[{error_label}] {error_text}");
        if collect_lua_error(state, &message) {
            eprintln!("Lua error: {message}");
        }
    }
}
