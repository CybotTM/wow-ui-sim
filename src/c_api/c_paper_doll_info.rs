use crate::c_api::helpers::ensure_namespace;
use crate::lua_api::methods::borrow_state;
use crate::lua_bridge::{FromStack, table_set_rust_fn_static};
use rilua::vm::state::LuaState;
use rilua::{LuaResult, Val};

const MAX_ARMOR_EFFECTIVENESS: f64 = 0.85;
const LOW_LEVEL_ARMOR_BASE: f64 = 400.0;
const LOW_LEVEL_ARMOR_PER_LEVEL: f64 = 85.0;
const HIGH_LEVEL_ARMOR_PER_LEVEL: f64 = 467.5;
const HIGH_LEVEL_ARMOR_OFFSET: f64 = 22_167.5;
const HIGH_LEVEL_ARMOR_FORMULA_MIN_LEVEL: f64 = 60.0;

pub(crate) fn register_c_paper_doll_info_surface(state: &mut LuaState) -> LuaResult<()> {
    let ns = ensure_namespace(state, "C_PaperDollInfo")?;
    table_set_rust_fn_static(state, ns, "GetArmorEffectiveness", get_armor_effectiveness)?;
    table_set_rust_fn_static(
        state,
        ns,
        "GetArmorEffectivenessAgainstTarget",
        get_armor_effectiveness_against_target,
    )?;
    Ok(())
}

fn get_armor_effectiveness(state: &mut LuaState) -> LuaResult<u32> {
    let armor = f64::from_stack(state, 1)?;
    let attacker_level = f64::from_stack(state, 2)?;
    state.push(Val::Num(armor_effectiveness(armor, attacker_level)));
    Ok(1)
}

fn get_armor_effectiveness_against_target(state: &mut LuaState) -> LuaResult<u32> {
    let armor = f64::from_stack(state, 1)?;
    let target_level = borrow_state(state)?
        .current_target
        .as_ref()
        .map(|target| target.level as f64);
    let Some(target_level) = target_level else {
        return Ok(0);
    };

    state.push(Val::Num(armor_effectiveness(armor, target_level)));
    Ok(1)
}

fn armor_effectiveness(armor: f64, attacker_level: f64) -> f64 {
    if armor <= 0.0 {
        return 0.0;
    }

    let mitigation_constant = armor_mitigation_constant(attacker_level);
    (armor / (armor + mitigation_constant)).clamp(0.0, MAX_ARMOR_EFFECTIVENESS)
}

fn armor_mitigation_constant(attacker_level: f64) -> f64 {
    let level = attacker_level.max(1.0);
    if level < HIGH_LEVEL_ARMOR_FORMULA_MIN_LEVEL {
        return LOW_LEVEL_ARMOR_BASE + LOW_LEVEL_ARMOR_PER_LEVEL * level;
    }

    (HIGH_LEVEL_ARMOR_PER_LEVEL * level - HIGH_LEVEL_ARMOR_OFFSET).max(1.0)
}
