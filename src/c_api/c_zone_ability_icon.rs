//! Zone ability icons project existing spell-catalog file IDs, never a fallback path.

use crate::lua_bridge::{stack_val, table_set_rust_fn_static};
use rilua::vm::state::LuaState;
use rilua::{LuaResult, Val, runtime_error};

pub(crate) fn register(state: &mut LuaState) -> LuaResult<()> {
    let namespace = super::ensure_namespace(state, "C_ZoneAbility")?;
    table_set_rust_fn_static(state, namespace, "GetZoneAbilityIcon", query_icon)
}

fn read_spell_id(value: Val) -> LuaResult<u32> {
    let Val::Num(number) = value else {
        return Err(runtime_error(
            "C_ZoneAbility.GetZoneAbilityIcon: spellID must be a number",
        ));
    };
    let in_range = (0.0..=f64::from(u32::MAX)).contains(&number);
    let represented = number.is_finite() && number.fract() == 0.0 && in_range;
    if !represented {
        return Err(runtime_error(
            "C_ZoneAbility.GetZoneAbilityIcon: spellID must be a u32 integer",
        ));
    }
    Ok(number as u32)
}

fn query_icon(state: &mut LuaState) -> LuaResult<u32> {
    let value = stack_val(state, 1);
    #[cfg(feature = "retail-12-0-5")]
    let value = rilua::table_security::unwrap_secret(state, value)?;
    let spell_id = read_spell_id(value)?;
    let icon = crate::spells::get_spell(spell_id)
        .map(|spell| Val::Num(f64::from(spell.icon_file_data_id)))
        .unwrap_or(Val::Nil);
    state.push(icon);
    Ok(1)
}
