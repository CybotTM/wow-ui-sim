//! 12340 legacy action globals over supplied simulator snapshots.
use crate::lua_api::methods::{borrow_state, val_to_string};
use crate::lua_bridge::stack_val;
use rilua::{LuaApiMut, LuaResult, Val, vm::state::LuaState};

fn read_action_slot(state: &LuaState) -> Option<u32> {
    let value = stack_val(state, 1);
    let number = match value {
        Val::Num(number) => Some(number),
        Val::Str(_) => val_to_string(state, value)?.trim().parse::<f64>().ok(),
        _ => None,
    }?;
    let slot = number.trunc();
    (1.0..=144.0).contains(&slot).then_some(slot as u32)
}

fn is_action_in_range(state: &mut LuaState) -> LuaResult<u32> {
    let result = match read_action_slot(state) {
        Some(slot) => {
            let unit = match stack_val(state, 2) {
                Val::Nil => "target".to_string(),
                value => val_to_string(state, value).unwrap_or_default(),
            };
            let sim = borrow_state(state)?;
            crate::c_api::action_spell_range::lookup_range(&sim, slot, &unit)
        }
        None => None,
    };
    state.push(result.map_or(Val::Nil, |in_range| Val::Num(u8::from(in_range) as f64)));
    Ok(1)
}

pub(super) fn register(lua: &mut rilua::Lua) -> crate::Result<()> {
    LuaApiMut::register_function(lua, "IsActionInRange", is_action_in_range)?;
    Ok(())
}
