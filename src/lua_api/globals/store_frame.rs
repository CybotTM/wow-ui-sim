//! `StoreFrame_IsShown` / `StoreFrame_SetShown` fallback globals.
//!
//! The sim doesn't render the in-game Store, but `MainMenuBarMicroButtons`
//! colours the Store micro-button as pushed when the probe returns true. A
//! SimState-backed flag lets tests drive that pushed-state rendering before the
//! Blizzard Store frame exists. Once `StoreFrame` is loaded, these globals defer
//! to the real frame attributes so Blizzard's `ToggleStoreUI` observes the same
//! state that `StoreFrame:SetAttribute("action", ...)` drives.

use crate::lua_api::methods::{borrow_state, borrow_state_mut, call_function_state, create_string};
use crate::lua_bridge::FromStack;
use crate::lua_bridge::table_set_rust_fn_static;
use rilua::vm::state::LuaState;
use rilua::{LuaApiMut, LuaResult, Val};

pub fn store_frame_is_shown(state: &mut LuaState) -> LuaResult<u32> {
    if let Some(shown) = actual_store_frame_is_shown(state)? {
        state.push(Val::Bool(shown));
        return Ok(1);
    }

    let shown = borrow_state(state)?.store_frame_shown;
    state.push(Val::Bool(shown));
    Ok(1)
}

pub fn store_frame_set_shown(state: &mut LuaState) -> LuaResult<u32> {
    let shown = Option::<bool>::from_stack(state, 1)?.unwrap_or(true);
    let context_key = Option::<String>::from_stack(state, 2)?;

    if set_actual_store_frame_shown(state, shown, context_key.as_deref())? {
        return Ok(0);
    }

    borrow_state_mut(state)?.store_frame_shown = shown;
    Ok(0)
}

fn actual_store_frame_is_shown(state: &mut LuaState) -> LuaResult<Option<bool>> {
    let probe = state.load(
        r#"
        local frame = rawget(_G, "StoreFrame")
        if type(frame) ~= "table" or type(frame.GetAttribute) ~= "function" then
            return nil
        end
        return frame:GetAttribute("isshown") and true or false
        "#,
    )?;
    let result = call_function_state(state, Val::Function(probe.gc_ref()), &[])?;
    Ok(match result {
        Val::Bool(shown) => Some(shown),
        _ => None,
    })
}

fn set_actual_store_frame_shown(
    state: &mut LuaState,
    shown: bool,
    context_key: Option<&str>,
) -> LuaResult<bool> {
    let setter = state.load(
        r#"
        local shown, contextKey = ...
        local frame = rawget(_G, "StoreFrame")
        if type(frame) ~= "table" or type(frame.SetAttribute) ~= "function" then
            return false
        end
        local wasShown = frame:GetAttribute("isshown") and true or false
        local contextKeyString = contextKey and tostring(contextKey) or nil
        if shown then
            frame:SetAttribute("contextkey", contextKeyString)
        end
        if wasShown ~= shown and C_StorePublic and type(C_StorePublic.EventStoreUISetShown) == "function" then
            C_StorePublic.EventStoreUISetShown(shown, contextKeyString)
        end
        frame:SetAttribute("action", shown and "Show" or "Hide")
        return true
        "#,
    )?;
    let context_key = context_key
        .map(|key| create_string(state, key))
        .unwrap_or(Val::Nil);
    let result = call_function_state(
        state,
        Val::Function(setter.gc_ref()),
        &[Val::Bool(shown), context_key],
    )?;
    Ok(matches!(result, Val::Bool(true)))
}

pub fn register_all(lua: &mut rilua::Lua) -> LuaResult<()> {
    use rilua::LuaApiMut;
    let state = lua.state_mut();
    table_set_rust_fn_static(
        state,
        state.global,
        "StoreFrame_IsShown",
        store_frame_is_shown,
    )?;
    table_set_rust_fn_static(
        state,
        state.global,
        "StoreFrame_SetShown",
        store_frame_set_shown,
    )?;
    Ok(())
}
