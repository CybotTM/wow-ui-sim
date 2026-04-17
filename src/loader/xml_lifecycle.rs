//! Lifecycle script firing for XML-created frames (OnLoad, OnShow).

use crate::loader::precompiled;
use crate::lua_api::LoaderEnv;
use crate::lua_api::methods::frame_ref;
use crate::lua_api::script_helpers::collect_lua_error;
use crate::lua_api::script_helpers::get_script;

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
        let frame_visible = if lifecycle.on_show {
            let Ok(sim) = crate::lua_api::methods::borrow_state(state) else {
                return Ok::<(), crate::Error>(());
            };
            sim.widgets.is_ancestor_visible(frame_id)
        } else {
            false
        };
        let Ok(frame) = frame_ref(state, frame_id) else {
            return Ok::<(), crate::Error>(());
        };
        if lifecycle.on_load {
            debug_onload_handler_state(state, frame_id, name);
            fire_handler(state, name, "OnLoad", precompiled::fire_onload, frame);
        }
        if lifecycle.on_show && frame_visible {
            fire_handler(state, name, "OnShow", precompiled::fire_onshow, frame);
        }
        Ok::<(), crate::Error>(())
    });
}

fn debug_onload_handler_state(
    state: &mut rilua::vm::state::LuaState,
    frame_id: u64,
    frame_name: &str,
) {
    if !frame_name.starts_with("__Blizzard_PlayerSpells_") {
        return;
    }
    let Some(handler) = get_script(state, frame_id, "OnLoad") else {
        eprintln!("[onload-debug] {frame_name} handler=nil");
        return;
    };
    match handler {
        rilua::Val::Function(func_ref) => {
            let closure_exists = state.gc.closures.get(func_ref).is_some();
            eprintln!(
                "[onload-debug] {frame_name} handler=function closure_exists={closure_exists}"
            );
        }
        other => {
            eprintln!(
                "[onload-debug] {frame_name} handler_type={}",
                other.type_name()
            );
        }
    }
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
