//! Native 12340 cached talent-group selection (TalentInfo_Scripts.cpp:583–595).
use crate::lua_api::methods::borrow_state;
use crate::lua_bridge::stack_val;
use rilua::vm::state::LuaState;
use rilua::{LuaApiMut, LuaResult, Val};

pub fn register(lua: &mut rilua::Lua) -> LuaResult<()> {
    LuaApiMut::register_function(lua, "GetActiveTalentGroup", get_active_talent_group)
}

fn get_active_talent_group(state: &mut LuaState) -> LuaResult<u32> {
    let inspect = string_to_bool(state, stack_val(state, 1));
    let pet = string_to_bool(state, stack_val(state, 2));
    let group = {
        let sim = borrow_state(state)?;
        if inspect {
            sim.wrath_legacy.inspected_talent_group + 1
        } else if pet {
            1
        } else {
            sim.wrath_legacy.player_talent_group + 1
        }
    };
    state.push(Val::Num(group as f64));
    Ok(1)
}

/// LoadXML StringToBOOL, not Lua truthiness and not numeric-string coercion.
fn string_to_bool(state: &LuaState, value: Val) -> bool {
    match value {
        Val::Bool(value) => value,
        Val::Num(value) => value as i32 != 0,
        Val::Str(reference) => {
            let Some(string) = state.gc.string_arena.get(reference) else {
                return false;
            };
            let bytes = string.data();
            matches!(bytes.first(), Some(b'1'..=b'9' | b'T' | b'Y' | b't' | b'y'))
                || bytes.eq_ignore_ascii_case(b"on")
                || bytes.eq_ignore_ascii_case(b"enabled")
        }
        _ => false,
    }
}
