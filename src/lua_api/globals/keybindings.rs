//! Keybinding global registrations.
//!
//! Backs the user-set side of WoW's binding API against
//! `SimState.keybindings`. The sim has no `Bindings.xml` registry, so
//! `GetNumBindings` / `GetBinding(index)` only iterate bindings the
//! user has set via `SetBinding`; they do not expose a fixed command
//! list the way retail does.
//!
//! Override bindings (`SetOverrideBinding` / `ClearOverrideBindings`)
//! shadow base bindings during lookup and are matched by WoW's
//! `GetBindingAction(key, checkOverride=true)` / `GetBindingKey`
//! semantics.

use crate::lua_api::methods::{borrow_state, borrow_state_mut, create_string};
use crate::lua_bridge::{FromStack, IntoStack};
use rilua::vm::state::LuaState;
use rilua::{LuaApiMut, LuaResult, Val};

fn push_opt_string(state: &mut LuaState, val: Option<String>) {
    match val {
        Some(s) => {
            let v = create_string(state, &s);
            state.push(v);
        }
        None => state.push(Val::Nil),
    }
}

pub fn get_binding_key(state: &mut LuaState) -> LuaResult<u32> {
    let action = Option::<String>::from_stack(state, 1)?.unwrap_or_default();
    let (k1, k2) = {
        let sim = borrow_state(state)?;
        sim.keybindings.keys_for_action(&action)
    };
    push_opt_string(state, k1);
    push_opt_string(state, k2);
    Ok(2)
}

pub fn get_binding_key_for_action(state: &mut LuaState) -> LuaResult<u32> {
    let action = Option::<String>::from_stack(state, 1)?.unwrap_or_default();
    let (k1, _) = {
        let sim = borrow_state(state)?;
        sim.keybindings.keys_for_action(&action)
    };
    push_opt_string(state, k1);
    Ok(1)
}

pub fn get_binding_action(state: &mut LuaState) -> LuaResult<u32> {
    let key = Option::<String>::from_stack(state, 1)?.unwrap_or_default();
    let action = {
        let sim = borrow_state(state)?;
        sim.keybindings.action_for_key(&key)
    };
    create_string(state, &action).into_stack(state)
}

pub fn get_binding(state: &mut LuaState) -> LuaResult<u32> {
    let index = i32::from_stack(state, 1).unwrap_or(0);
    let sim = borrow_state(state)?;
    let idx_0 = (index - 1) as usize;
    let entry = sim.keybindings.base.get(idx_0).cloned();
    drop(sim);
    match entry {
        Some((key, action)) => {
            let action_v = create_string(state, &action);
            let key_v = create_string(state, &key);
            state.push(action_v);
            state.push(key_v);
            Ok(2)
        }
        None => {
            state.push(Val::Nil);
            state.push(Val::Nil);
            Ok(2)
        }
    }
}

pub fn get_num_bindings(state: &mut LuaState) -> LuaResult<u32> {
    let n = borrow_state(state)?.keybindings.base.len() as i32;
    n.into_stack(state)
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
    let key = Option::<String>::from_stack(state, 1)?.unwrap_or_default();
    let action = Option::<String>::from_stack(state, 2)?.unwrap_or_default();
    if key.is_empty() {
        return false.into_stack(state);
    }
    borrow_state_mut(state)?.keybindings.set(&key, &action);
    true.into_stack(state)
}

pub fn set_binding_click(state: &mut LuaState) -> LuaResult<u32> {
    let key = Option::<String>::from_stack(state, 1)?.unwrap_or_default();
    let button_name = Option::<String>::from_stack(state, 2)?.unwrap_or_default();
    if key.is_empty() || button_name.is_empty() {
        return false.into_stack(state);
    }
    let action = format!("CLICK {button_name}:LeftButton");
    borrow_state_mut(state)?.keybindings.set(&key, &action);
    true.into_stack(state)
}

pub fn set_binding_spell(state: &mut LuaState) -> LuaResult<u32> {
    let key = Option::<String>::from_stack(state, 1)?.unwrap_or_default();
    let spell = Option::<String>::from_stack(state, 2)?.unwrap_or_default();
    if key.is_empty() || spell.is_empty() {
        return false.into_stack(state);
    }
    let action = format!("SPELL {spell}");
    borrow_state_mut(state)?.keybindings.set(&key, &action);
    true.into_stack(state)
}

pub fn set_binding_item(state: &mut LuaState) -> LuaResult<u32> {
    let key = Option::<String>::from_stack(state, 1)?.unwrap_or_default();
    let item = Option::<String>::from_stack(state, 2)?.unwrap_or_default();
    if key.is_empty() || item.is_empty() {
        return false.into_stack(state);
    }
    let action = format!("ITEM {item}");
    borrow_state_mut(state)?.keybindings.set(&key, &action);
    true.into_stack(state)
}

pub fn set_binding_macro(state: &mut LuaState) -> LuaResult<u32> {
    let key = Option::<String>::from_stack(state, 1)?.unwrap_or_default();
    let macro_name = Option::<String>::from_stack(state, 2)?.unwrap_or_default();
    if key.is_empty() || macro_name.is_empty() {
        return false.into_stack(state);
    }
    let action = format!("MACRO {macro_name}");
    borrow_state_mut(state)?.keybindings.set(&key, &action);
    true.into_stack(state)
}

pub fn set_override_binding(state: &mut LuaState) -> LuaResult<u32> {
    // Args: owner (frame), isPriority (bool), key (string), action (string?)
    let key = Option::<String>::from_stack(state, 3)?.unwrap_or_default();
    let action = Option::<String>::from_stack(state, 4)?.unwrap_or_default();
    if key.is_empty() {
        return Ok(0);
    }
    borrow_state_mut(state)?
        .keybindings
        .set_override(&key, &action);
    Ok(0)
}

pub fn clear_override_bindings(state: &mut LuaState) -> LuaResult<u32> {
    borrow_state_mut(state)?.keybindings.clear_overrides();
    Ok(0)
}

pub fn save_bindings(_state: &mut LuaState) -> LuaResult<u32> {
    // No disk persistence in the sim — values live in SimState for the
    // lifetime of the env.
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
