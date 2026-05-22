//! C_Scenario temporary shim — scenario participation state is not modeled.
//!
//! Blizzard objective-tracker load code expects numeric zeroes for stage and
//! step counts, not nils, when the player is not in a scenario. Real scenario
//! state should replace this surface.

use crate::c_api::ensure_global_table;
use crate::lua_api::methods::create_string_static;
use crate::lua_bridge::table_set_rust_fn_static;
use rilua::vm::state::LuaState;
use rilua::{LuaResult, Val};

pub fn register_c_scenario_shims(state: &mut LuaState) -> LuaResult<()> {
    let t = ensure_global_table(state, "C_Scenario");
    let Val::Table(t_ref) = t else {
        unreachable!("C_Scenario must be a table");
    };
    table_set_rust_fn_static(state, t_ref, "GetInfo", get_info)?;
    table_set_rust_fn_static(state, t_ref, "IsInScenario", is_in_scenario)?;
    table_set_rust_fn_static(state, t_ref, "GetStepInfo", get_step_info)?;
    Ok(())
}

fn get_info(state: &mut LuaState) -> LuaResult<u32> {
    let texture_kit = create_string_static(state, "evergreen-scenario");
    state.push(Val::Nil);
    state.push(Val::Num(0.0));
    state.push(Val::Num(0.0));
    state.push(Val::Num(0.0));
    state.push(Val::Nil);
    state.push(Val::Nil);
    state.push(Val::Nil);
    state.push(Val::Num(0.0));
    state.push(Val::Num(0.0));
    state.push(Val::Num(0.0));
    state.push(Val::Nil);
    state.push(texture_kit);
    state.push(Val::Num(0.0));
    Ok(13)
}

fn is_in_scenario(state: &mut LuaState) -> LuaResult<u32> {
    state.push(Val::Bool(false));
    Ok(1)
}

fn get_step_info(state: &mut LuaState) -> LuaResult<u32> {
    state.push(Val::Nil);
    state.push(Val::Num(0.0));
    state.push(Val::Num(0.0));
    state.push(Val::Bool(false));
    state.push(Val::Bool(false));
    state.push(Val::Num(0.0));
    state.push(Val::Num(0.0));
    state.push(Val::Num(0.0));
    state.push(Val::Num(0.0));
    state.push(Val::Bool(false));
    state.push(Val::Bool(false));
    Ok(11)
}
