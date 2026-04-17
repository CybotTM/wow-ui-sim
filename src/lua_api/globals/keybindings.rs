//! Keybinding global registrations.
//!
//! These stub the WoW binding API surface (`GetBindingKey`, `SetBinding`,
//! etc.) with shapes that match the addon-side destructuring. Real
//! SimState-backed storage is TODO — most callers at startup just probe
//! "is there a binding for X" and take the "no binding" branch.

use crate::lua_api::methods::create_string;
use crate::lua_bridge::{FromStack, IntoStack};
use rilua::vm::state::LuaState;
use rilua::{LuaApiMut, LuaResult, Val};

pub fn get_binding_key(state: &mut LuaState) -> LuaResult<u32> {
    // TODO: keybindings::get_binding_key on SimState
    state.push(Val::Nil);
    state.push(Val::Nil);
    Ok(2)
}

pub fn get_binding_key_for_action(state: &mut LuaState) -> LuaResult<u32> {
    // TODO: keybindings lookup on SimState
    Val::Nil.into_stack(state)
}

pub fn get_binding_action(state: &mut LuaState) -> LuaResult<u32> {
    // TODO: reverse keybinding lookup on SimState
    create_string(state, "").into_stack(state)
}

pub fn get_binding(state: &mut LuaState) -> LuaResult<u32> {
    // TODO: keybindings::get_binding_at on SimState
    state.push(Val::Nil);
    state.push(Val::Nil);
    Ok(2)
}

pub fn get_num_bindings(state: &mut LuaState) -> LuaResult<u32> {
    // TODO: keybindings::get_num_bindings on SimState
    (0i32).into_stack(state)
}

pub fn get_current_binding_set(state: &mut LuaState) -> LuaResult<u32> {
    (1i32).into_stack(state)
}

pub fn get_binding_text(state: &mut LuaState) -> LuaResult<u32> {
    let key = Option::<String>::from_stack(state, 1)?;
    match key {
        Some(k) => create_string(state, &k).into_stack(state),
        None => create_string(state, "").into_stack(state),
    }
}

pub fn is_binding_for_game_pad(state: &mut LuaState) -> LuaResult<u32> {
    false.into_stack(state)
}

pub fn set_binding(state: &mut LuaState) -> LuaResult<u32> {
    // TODO: keybindings::set_binding on SimState
    true.into_stack(state)
}

pub fn set_binding_click(state: &mut LuaState) -> LuaResult<u32> {
    true.into_stack(state)
}

pub fn set_binding_spell(state: &mut LuaState) -> LuaResult<u32> {
    true.into_stack(state)
}

pub fn set_binding_item(state: &mut LuaState) -> LuaResult<u32> {
    true.into_stack(state)
}

pub fn set_binding_macro(state: &mut LuaState) -> LuaResult<u32> {
    true.into_stack(state)
}

pub fn set_override_binding(_state: &mut LuaState) -> LuaResult<u32> {
    Ok(0)
}

pub fn clear_override_bindings(_state: &mut LuaState) -> LuaResult<u32> {
    Ok(0)
}

pub fn save_bindings(_state: &mut LuaState) -> LuaResult<u32> {
    Ok(0)
}

pub fn load_bindings(_state: &mut LuaState) -> LuaResult<u32> {
    Ok(0)
}

/// Register all binding-related global functions on the rilua VM.
pub fn register_all(lua: &mut rilua::Lua) -> LuaResult<()> {
    LuaApiMut::register_function(lua, "GetBindingKey", get_binding_key)?;
    LuaApiMut::register_function(lua, "GetBindingKeyForAction", get_binding_key_for_action)?;
    LuaApiMut::register_function(lua, "GetBindingAction", get_binding_action)?;
    LuaApiMut::register_function(lua, "GetBinding", get_binding)?;
    LuaApiMut::register_function(lua, "GetNumBindings", get_num_bindings)?;
    LuaApiMut::register_function(lua, "GetCurrentBindingSet", get_current_binding_set)?;
    LuaApiMut::register_function(lua, "GetBindingText", get_binding_text)?;
    LuaApiMut::register_function(lua, "IsBindingForGamePad", is_binding_for_game_pad)?;
    LuaApiMut::register_function(lua, "SetBinding", set_binding)?;
    LuaApiMut::register_function(lua, "SetBindingClick", set_binding_click)?;
    LuaApiMut::register_function(lua, "SetBindingSpell", set_binding_spell)?;
    LuaApiMut::register_function(lua, "SetBindingItem", set_binding_item)?;
    LuaApiMut::register_function(lua, "SetBindingMacro", set_binding_macro)?;
    LuaApiMut::register_function(lua, "SetOverrideBinding", set_override_binding)?;
    LuaApiMut::register_function(lua, "ClearOverrideBindings", clear_override_bindings)?;
    LuaApiMut::register_function(lua, "SaveBindings", save_bindings)?;
    LuaApiMut::register_function(lua, "LoadBindings", load_bindings)?;
    Ok(())
}
