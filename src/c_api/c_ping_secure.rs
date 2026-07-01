//! Minimal `C_PingSecure` surface.
//!
//! Retail PingUI registers secure callbacks through this namespace during
//! startup. The simulator does not model protected ping-wheel creation yet, so
//! callbacks are accepted and retained as inert compatibility state while
//! `CreateFrame` is a no-op.

use crate::c_api::helpers::ensure_namespace;
use crate::lua_api::methods::{create_table, table_set};
use crate::lua_bridge::{stack_val, table_set_rust_fn_static};
use rilua::vm::state::LuaState;
use rilua::{LuaResult, Val};

const CALLBACK_TABLE_KEY: &str = "__wow_ping_secure_callbacks";

pub(crate) fn register_c_ping_secure_surface(state: &mut LuaState) -> LuaResult<()> {
    let table_ref = ensure_namespace(state, "C_PingSecure")?;
    table_set_rust_fn_static(state, table_ref, "CreateFrame", create_frame)?;
    table_set_rust_fn_static(
        state,
        table_ref,
        "SetPendingPingOffScreenCallback",
        set_pending_ping_off_screen_callback,
    )?;
    table_set_rust_fn_static(
        state,
        table_ref,
        "SetPingCooldownStartedCallback",
        set_ping_cooldown_started_callback,
    )?;
    table_set_rust_fn_static(
        state,
        table_ref,
        "SetPingRadialWheelCreatedCallback",
        set_ping_radial_wheel_created_callback,
    )?;
    table_set_rust_fn_static(
        state,
        table_ref,
        "SetTogglePingListenerCallback",
        set_toggle_ping_listener_callback,
    )?;
    Ok(())
}

fn create_frame(_state: &mut LuaState) -> LuaResult<u32> {
    Ok(0)
}

fn set_pending_ping_off_screen_callback(state: &mut LuaState) -> LuaResult<u32> {
    set_callback(state, "pendingPingOffScreen")
}

fn set_ping_cooldown_started_callback(state: &mut LuaState) -> LuaResult<u32> {
    set_callback(state, "pingCooldownStarted")
}

fn set_ping_radial_wheel_created_callback(state: &mut LuaState) -> LuaResult<u32> {
    set_callback(state, "pingRadialWheelCreated")
}

fn set_toggle_ping_listener_callback(state: &mut LuaState) -> LuaResult<u32> {
    set_callback(state, "togglePingListener")
}

fn set_callback(state: &mut LuaState, key: &str) -> LuaResult<u32> {
    let callbacks = callback_table(state);
    table_set(state, callbacks, key, stack_val(state, 1));
    Ok(0)
}

fn callback_table(state: &mut LuaState) -> Val {
    let key = state.gc.intern_string(CALLBACK_TABLE_KEY.as_bytes());
    let current = state
        .gc
        .tables
        .get(state.global)
        .map(|globals| globals.get_str(key, &state.gc.string_arena))
        .unwrap_or(Val::Nil);
    if matches!(current, Val::Table(_)) {
        return current;
    }

    let callbacks = create_table(state);
    if let Some(globals) = state.gc.tables.get_mut(state.global) {
        let _ = globals.raw_set(Val::Str(key), callbacks, &state.gc.string_arena);
    }
    state.gc.barrier_back(state.global);
    callbacks
}

#[cfg(test)]
mod tests {
    use crate::lua_api::WowLuaEnv;

    #[test]
    fn registers_ping_secure_api() {
        let env = WowLuaEnv::new().expect("lua env");
        let result: String = env
            .eval(
                r#"
                if type(C_PingSecure) ~= "table" then return "namespace" end
                if type(C_PingSecure.CreateFrame) ~= "function" then return "create_frame" end
                if type(C_PingSecure.SetPendingPingOffScreenCallback) ~= "function" then return "pending_callback" end
                if type(C_PingSecure.SetPingCooldownStartedCallback) ~= "function" then return "cooldown_callback" end
                if type(C_PingSecure.SetPingRadialWheelCreatedCallback) ~= "function" then return "radial_callback" end
                if type(C_PingSecure.SetTogglePingListenerCallback) ~= "function" then return "toggle_callback" end
                C_PingSecure.CreateFrame()
                C_PingSecure.SetPendingPingOffScreenCallback(function() end)
                C_PingSecure.SetPingCooldownStartedCallback(function() end)
                C_PingSecure.SetPingRadialWheelCreatedCallback(function() end)
                C_PingSecure.SetTogglePingListenerCallback(function() end)
                if type(rawget(__secureenv, "C_PingSecure")) ~= "table" then return "secure_namespace" end
                return "ok"
                "#,
            )
            .expect("C_PingSecure probe");

        assert_eq!(result, "ok");
    }
}
