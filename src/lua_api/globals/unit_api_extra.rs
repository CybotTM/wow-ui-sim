//! Additional unit-related WoW API functions kept separate from `unit_api`.

use crate::lua_api::SimState;
use mlua::{Lua, Result, Value};
use std::cell::RefCell;
use std::rc::Rc;

pub fn register_extra_unit_functions(lua: &Lua, state: Rc<RefCell<SimState>>) -> Result<()> {
    register_threat_functions(lua)?;
    register_classification_functions(lua, state.clone())?;
    register_casting_functions(lua)?;
    register_unit_casting_info(lua, state.clone())?;
    register_weapon_enchant_functions(lua)?;
    register_xp_functions(lua)?;
    register_pvp_vehicle_functions(lua, state.clone())?;
    register_group_roster_globals(lua, state.clone())?;
    register_misc_unit_functions(lua)?;
    Ok(())
}

pub(crate) fn register_group_roster_globals(lua: &Lua, state: Rc<RefCell<SimState>>) -> Result<()> {
    let globals = lua.globals();

    let state_for_group_check = state.clone();
    globals.set(
        "IsInGroup",
        lua.create_function(move |_, _flags: Option<i32>| {
            Ok(!state_for_group_check.borrow().party_members.is_empty())
        })?,
    )?;
    globals.set("IsInRaid", lua.create_function(|_, ()| Ok(false))?)?;

    let state_for_subgroup_count = state.clone();
    globals.set(
        "GetNumSubgroupMembers",
        lua.create_function(move |_, ()| {
            Ok(state_for_subgroup_count.borrow().party_members.len() as i32)
        })?,
    )?;
    globals.set(
        "GetNumGroupMembers",
        lua.create_function(move |_, _category: Option<Value>| {
            let party_count = state.borrow().party_members.len() as i32;
            Ok(if party_count > 0 { party_count + 1 } else { 0 })
        })?,
    )?;
    Ok(())
}

/// Register UnitThreatSituation, UnitDetailedThreatSituation.
fn register_threat_functions(lua: &Lua) -> Result<()> {
    let globals = lua.globals();

    globals.set(
        "UnitThreatSituation",
        lua.create_function(|_, (_unit, _mob): (String, Option<String>)| Ok(Value::Nil))?,
    )?;
    globals.set(
        "UnitDetailedThreatSituation",
        lua.create_function(|_, (_unit, _mob): (String, Option<String>)| {
            Ok((false, 0i32, 0.0f64, 0.0f64, 0i32))
        })?,
    )?;

    Ok(())
}

/// Register UnitClassification, UnitCreatureType, UnitCreatureFamily, UnitReaction.
fn register_classification_functions(lua: &Lua, state: Rc<RefCell<SimState>>) -> Result<()> {
    let globals = lua.globals();

    let state_for_classification = Rc::clone(&state);
    globals.set(
        "UnitClassification",
        lua.create_function(move |lua, unit: Option<String>| {
            let classification = lookup_target_field(
                &state_for_classification.borrow(),
                unit.as_deref(),
                |target| target.classification.clone(),
            )
            .unwrap_or_else(|| "normal".to_string());
            Ok(Value::String(lua.create_string(&classification)?))
        })?,
    )?;

    let state_for_creature_type = Rc::clone(&state);
    globals.set(
        "UnitCreatureType",
        lua.create_function(move |lua, unit: Option<String>| {
            let creature_type = lookup_target_field(
                &state_for_creature_type.borrow(),
                unit.as_deref(),
                |target| target.creature_type.clone(),
            )
            .unwrap_or_else(|| "Humanoid".to_string());
            Ok(Value::String(lua.create_string(&creature_type)?))
        })?,
    )?;

    globals.set(
        "UnitCreatureFamily",
        lua.create_function(|_, _unit: Option<String>| Ok(Value::Nil))?,
    )?;

    let state_for_reaction = Rc::clone(&state);
    globals.set(
        "UnitReaction",
        lua.create_function(move |_, (_unit1, unit2): (String, String)| {
            let reaction =
                lookup_target_field(&state_for_reaction.borrow(), Some(&unit2), |target| {
                    target.reaction
                })
                .unwrap_or(5);
            Ok(reaction)
        })?,
    )?;

    Ok(())
}

/// Look up a field from the TargetInfo matching a unit ID ("target", "focus", "player", etc.).
fn lookup_target_field<T>(
    state: &SimState,
    unit: Option<&str>,
    f: impl Fn(&super::super::game_data::TargetInfo) -> T,
) -> Option<T> {
    match unit.unwrap_or("player") {
        "target" => state.current_target.as_ref().map(&f),
        "focus" => state.current_focus.as_ref().map(&f),
        _ => None,
    }
}

/// Register UnitCastingInfo, UnitChannelInfo.
fn register_casting_functions(lua: &Lua) -> Result<()> {
    // UnitCastingInfo needs state and is registered separately.
    lua.globals().set(
        "UnitChannelInfo",
        lua.create_function(|_, _unit: Option<String>| Ok(Value::Nil))?,
    )
}

/// Register UnitCastingInfo with state access for active cast tracking.
fn register_unit_casting_info(lua: &Lua, state: Rc<RefCell<SimState>>) -> Result<()> {
    lua.globals().set(
        "UnitCastingInfo",
        lua.create_function(move |lua, unit: Option<String>| {
            if unit.as_deref() != Some("player") {
                return Ok(mlua::MultiValue::new());
            }

            let state = state.borrow();
            let Some(cast) = &state.casting else {
                return Ok(mlua::MultiValue::new());
            };

            Ok(mlua::MultiValue::from_vec(vec![
                Value::String(lua.create_string(&cast.spell_name)?),
                Value::String(lua.create_string(&cast.spell_name)?),
                Value::String(lua.create_string(&cast.icon_path)?),
                Value::Number(cast.start_time * 1000.0),
                Value::Number(cast.end_time * 1000.0),
                Value::Boolean(false), // isTradeSkill
                Value::Integer(cast.cast_id as i64),
                Value::Boolean(false), // notInterruptible
                Value::Integer(cast.spell_id as i64),
            ]))
        })?,
    )
}

/// Register GetWeaponEnchantInfo.
fn register_weapon_enchant_functions(lua: &Lua) -> Result<()> {
    let globals = lua.globals();

    globals.set(
        "GetWeaponEnchantInfo",
        lua.create_function(|_, ()| Ok((false, 0i32, 0i32, 0i32, false, 0i32, 0i32, 0i32)))?,
    )?;

    Ok(())
}

/// Register UnitXP, UnitXPMax, UnitTrialXP, GetXPExhaustion, GetRestState.
fn register_xp_functions(lua: &Lua) -> Result<()> {
    let globals = lua.globals();
    let xp_max = 89_750i32;
    let nanos = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default()
        .subsec_nanos();
    let xp_current = (nanos % xp_max as u32) as i32;

    globals.set(
        "UnitXP",
        lua.create_function(move |_, _unit: Option<String>| Ok(xp_current))?,
    )?;
    globals.set(
        "UnitXPMax",
        lua.create_function(move |_, _unit: Option<String>| Ok(xp_max))?,
    )?;
    globals.set(
        "UnitTrialXP",
        lua.create_function(|_, _unit: Option<String>| Ok(0i32))?,
    )?;
    globals.set(
        "GetXPExhaustion",
        lua.create_function(|_, ()| Ok(Value::Nil))?,
    )?;
    globals.set("GetRestState", lua.create_function(|_, ()| Ok(1i32))?)?;
    Ok(())
}

/// Register PvP and vehicle-related unit functions.
fn register_pvp_vehicle_functions(lua: &Lua, state: Rc<RefCell<SimState>>) -> Result<()> {
    let globals = lua.globals();

    for &name in &[
        "UnitIsPVP",
        "UnitIsPVPFreeForAll",
        "UnitIsMercenary",
        "UnitInVehicle",
        "UnitHasVehiclePlayerFrameUI",
        "UnitInVehicleHidesPetFrame",
        "UnitInPartyIsAI",
    ] {
        globals.set(
            name,
            lua.create_function(|_, _unit: Option<String>| Ok(false))?,
        )?;
    }

    let state_for_affecting_combat = state.clone();
    globals.set(
        "UnitAffectingCombat",
        lua.create_function(move |_, _unit: Option<String>| {
            Ok(state_for_affecting_combat.borrow().player.in_combat)
        })?,
    )?;

    let state_for_in_combat = state.clone();
    globals.set(
        "UnitInCombat",
        lua.create_function(move |_, _unit: Option<String>| {
            Ok(state_for_in_combat.borrow().player.in_combat)
        })?,
    )?;

    let state_for_honor_level = state.clone();
    globals.set(
        "UnitHonorLevel",
        lua.create_function(move |_, _unit: Option<String>| {
            Ok(state_for_honor_level.borrow().player.honor_level)
        })?,
    )?;

    globals.set(
        "UnitPartialPower",
        lua.create_function(|_, (_unit, _pt): (Option<String>, Option<i32>)| Ok(0i32))?,
    )?;
    globals.set(
        "UnitGroupRolesAssignedEnum",
        lua.create_function(|_, _unit: Option<String>| Ok(Value::Nil))?,
    )?;
    globals.set(
        "UnitRealmRelationship",
        lua.create_function(|_, _unit: Option<String>| Ok(Value::Nil))?,
    )?;

    globals.set(
        "UnitSelectionColor",
        lua.create_function(move |_, unit: Option<String>| {
            if unit.as_deref() != Some("target") {
                return Ok((1.0f64, 1.0f64, 1.0f64, 1.0f64));
            }

            let state = state.borrow();
            let Some(target) = &state.current_target else {
                return Ok((1.0f64, 1.0f64, 1.0f64, 1.0f64));
            };

            if target.is_enemy {
                return Ok((1.0f64, 0.0f64, 0.0f64, 1.0f64));
            }

            Ok((0.0f64, 1.0f64, 0.0f64, 1.0f64))
        })?,
    )?;

    Ok(())
}

/// Register miscellaneous unit query functions.
fn register_misc_unit_functions(lua: &Lua) -> Result<()> {
    let globals = lua.globals();

    globals.set(
        "UnitPhaseReason",
        lua.create_function(|_, _unit: Option<String>| Ok(Value::Nil))?,
    )?;
    globals.set(
        "UnitIsOwnerOrControllerOfUnit",
        lua.create_function(|_, (_u1, _u2): (String, String)| Ok(false))?,
    )?;
    globals.set(
        "UnitIsWarModePhased",
        lua.create_function(|_, _unit: Option<String>| Ok(false))?,
    )?;
    globals.set(
        "UnitIsWarModeDesired",
        lua.create_function(|_, _unit: Option<String>| Ok(false))?,
    )?;
    globals.set(
        "UnitIsWarModeActive",
        lua.create_function(|_, _unit: Option<String>| Ok(false))?,
    )?;
    globals.set(
        "UnitHasMana",
        lua.create_function(|_, _unit: Value| Ok(true))?,
    )?;
    globals.set(
        "UnitHasRelicSlot",
        lua.create_function(|_, _unit: Value| Ok(false))?,
    )?;
    globals.set(
        "IsActiveBattlefieldArena",
        lua.create_function(|_, ()| Ok(false))?,
    )?;
    globals.set(
        "GetUnitPowerBarInfo",
        lua.create_function(|_, _unit: Value| Ok(Value::Nil))?,
    )?;

    globals.set(
        "UnitStagger",
        lua.create_function(|_, _unit: Value| Ok(0i32))?,
    )?;
    Ok(())
}
