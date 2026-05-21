//! C_Spell temporary static fallbacks — spell override, charges, visibility,
//! and Maw atlas state are not modeled yet.
//!
//! These functions keep addon startup code on inert values until the simulator
//! has backing state for spell override replacement, charge counters, spell
//! visibility rules, and Maw power border art.

use crate::c_api::helpers::ensure_namespace;
use crate::lua_api::methods::{create_table, table_set_static};
use crate::lua_bridge::{FromStack, table_set_rust_fn_static};
use rilua::vm::state::LuaState;
use rilua::{LuaResult, Val};

pub(crate) fn register_c_spell_static_fallbacks(state: &mut LuaState) -> LuaResult<()> {
    let ns = ensure_namespace(state, "C_Spell")?;
    table_set_rust_fn_static(state, ns, "GetSpellCharges", get_spell_charges)?;
    table_set_rust_fn_static(state, ns, "GetOverrideSpell", get_override_spell)?;
    table_set_rust_fn_static(state, ns, "GetVisibilityInfo", get_visibility_info)?;
    table_set_rust_fn_static(
        state,
        ns,
        "GetMawPowerBorderAtlasBySpellID",
        get_maw_power_border_atlas_by_spell_id,
    )?;
    Ok(())
}

fn get_spell_charges(state: &mut LuaState) -> LuaResult<u32> {
    let _spell_id = u32::from_stack(state, 1)?;
    let charges = create_table(state);
    table_set_static(state, charges, "currentCharges", Val::Num(0.0));
    table_set_static(state, charges, "maxCharges", Val::Num(0.0));
    table_set_static(state, charges, "cooldownStartTime", Val::Num(0.0));
    table_set_static(state, charges, "cooldownDuration", Val::Num(0.0));
    table_set_static(state, charges, "chargeModRate", Val::Num(1.0));
    state.push(charges);
    Ok(1)
}

fn get_override_spell(state: &mut LuaState) -> LuaResult<u32> {
    let spell_id = u32::from_stack(state, 1)?;
    state.push(Val::Num(spell_id as f64));
    Ok(1)
}

fn get_visibility_info(state: &mut LuaState) -> LuaResult<u32> {
    state.push(Val::Bool(false));
    state.push(Val::Bool(true));
    state.push(Val::Bool(false));
    Ok(3)
}

fn get_maw_power_border_atlas_by_spell_id(state: &mut LuaState) -> LuaResult<u32> {
    let _spell_id = u32::from_stack(state, 1)?;
    state.push(Val::Nil);
    Ok(1)
}
