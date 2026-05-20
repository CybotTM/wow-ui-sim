//! Temporary `C_ClickBindings` fallback.
//!
//! Real click-binding profiles are client/account state we do not model yet.
//! Without a backed namespace, Blizzard's secure unit-button handler sees Lua
//! nil-stubs and can fall through to stale click-cast secure attributes, making
//! normal party-frame left clicks do nothing. Retire this once click-binding
//! profile storage and macro/spell execution are modelled.

use crate::c_api::helpers::{ensure_namespace, global_val};
use crate::lua_api::methods::{call_function_state, create_string, val_to_string};
use crate::lua_bridge::{stack_val, table_set_rust_fn_static};
use rilua::vm::state::LuaState;
use rilua::{LuaResult, Val};

const BINDING_TYPE_MACRO: f64 = 2.0;
const BINDING_TYPE_INTERACTION: f64 = 3.0;

pub(crate) fn register_c_click_bindings_fallback(state: &mut LuaState) -> LuaResult<()> {
    let ns = ensure_namespace(state, "C_ClickBindings")?;
    table_set_rust_fn_static(state, ns, "CanSpellBeClickBound", can_spell_be_click_bound)?;
    table_set_rust_fn_static(state, ns, "ExecuteBinding", execute_binding)?;
    table_set_rust_fn_static(state, ns, "GetBindingType", get_binding_type)?;
    table_set_rust_fn_static(
        state,
        ns,
        "GetEffectiveInteractionButton",
        get_effective_interaction_button,
    )?;
    table_set_rust_fn_static(state, ns, "GetProfileInfo", get_profile_info)?;
    table_set_rust_fn_static(
        state,
        ns,
        "GetStringFromModifiers",
        get_string_from_modifiers,
    )?;
    table_set_rust_fn_static(state, ns, "GetTutorialShown", get_tutorial_shown)?;
    table_set_rust_fn_static(state, ns, "MakeModifiers", make_modifiers)?;
    table_set_rust_fn_static(state, ns, "ResetCurrentProfile", no_return)?;
    table_set_rust_fn_static(state, ns, "SetProfileByInfo", no_return)?;
    table_set_rust_fn_static(state, ns, "SetTutorialShown", no_return)
}

fn can_spell_be_click_bound(state: &mut LuaState) -> LuaResult<u32> {
    state.push(Val::Bool(true));
    Ok(1)
}

fn execute_binding(state: &mut LuaState) -> LuaResult<u32> {
    let target_token = stack_val(state, 1);
    let target_unit = global_val(state, "TargetUnit");
    if matches!(target_token, Val::Str(_)) && matches!(target_unit, Val::Function(_)) {
        let _ = call_function_state(state, target_unit, &[target_token]);
    }
    Ok(0)
}

fn get_binding_type(state: &mut LuaState) -> LuaResult<u32> {
    let binding_type = match val_to_string(state, stack_val(state, 1)).as_deref() {
        Some("LeftButton") => BINDING_TYPE_MACRO,
        _ => BINDING_TYPE_INTERACTION,
    };
    state.push(Val::Num(binding_type));
    Ok(1)
}

fn get_effective_interaction_button(state: &mut LuaState) -> LuaResult<u32> {
    state.push(stack_val(state, 1));
    Ok(1)
}

fn get_profile_info(state: &mut LuaState) -> LuaResult<u32> {
    let table = crate::lua_api::methods::create_table(state);
    state.push(table);
    Ok(1)
}

fn get_string_from_modifiers(state: &mut LuaState) -> LuaResult<u32> {
    let value = create_string(state, "");
    state.push(value);
    Ok(1)
}

fn get_tutorial_shown(state: &mut LuaState) -> LuaResult<u32> {
    state.push(Val::Bool(true));
    Ok(1)
}

fn make_modifiers(state: &mut LuaState) -> LuaResult<u32> {
    state.push(Val::Num(0.0));
    Ok(1)
}

fn no_return(_state: &mut LuaState) -> LuaResult<u32> {
    Ok(0)
}
