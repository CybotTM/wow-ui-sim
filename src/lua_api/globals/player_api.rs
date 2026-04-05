//! Player-related API functions.
//!
//! This module provides WoW API functions related to:
//! - BattleNet features (BNFeaturesEnabled, BNConnected, BNGetFriendInfo, etc.)
//! - Specialization info (GetSpecialization, GetSpecializationInfo, etc.)
//! - Action bar functions (HasAction, GetActionInfo, GetActionTexture, etc.)

use crate::lua_api::SimState;
use mlua::{Lua, Result, Value};
use std::cell::RefCell;
use std::rc::Rc;

/// Register all player-related API functions to the Lua globals table.
pub fn register_player_api(lua: &Lua, state: Rc<RefCell<SimState>>) -> Result<()> {
    register_battlenet_functions(lua)?;
    register_specialization_functions(lua, Rc::clone(&state))?;
    register_movement_functions(lua, Rc::clone(&state))?;
    super::action_bar_api::register_action_bar_functions(lua, state.clone())?;
    register_timerunning_functions(lua)?;
    register_economy_functions(lua, Rc::clone(&state))?;
    register_instance_functions(lua, Rc::clone(&state))?;
    register_character_functions(lua, Rc::clone(&state))?;
    register_character_stat_functions(lua)?;
    register_cinematic_functions(lua)?;
    register_unit_functions(lua)?;
    Ok(())
}

fn register_timerunning_functions(lua: &Lua) -> Result<()> {
    let globals = lua.globals();
    globals.set(
        "PlayerIsTimerunning",
        lua.create_function(|_, ()| Ok(false))?,
    )?;
    globals.set(
        "IsPlayerAtEffectiveMaxLevel",
        lua.create_function(|_, ()| Ok(false))?,
    )?;
    globals.set("IsXPUserDisabled", lua.create_function(|_, ()| Ok(false))?)?;
    Ok(())
}

/// Register BattleNet social functions.
fn register_battlenet_functions(lua: &Lua) -> Result<()> {
    let g = lua.globals();
    let bn_false = lua.create_function(|_, ()| Ok(false))?;
    for name in [
        "BNFeaturesEnabled",
        "BNFeaturesEnabledAndConnected",
        "BNConnected",
    ] {
        g.set(name, bn_false.clone())?;
    }
    g.set(
        "BNGetFriendInfo",
        lua.create_function(|_, _: i32| Ok(Value::Nil))?,
    )?;
    g.set(
        "BNGetNumFriends",
        lua.create_function(|_, ()| Ok((0, 0, 0, 0)))?,
    )?;
    g.set(
        "BNGetNumFriendInvites",
        lua.create_function(|_, ()| Ok(0i32))?,
    )?;
    g.set("BNGetInfo", lua.create_function(stub_bn_get_info)?)?;
    Ok(())
}

/// Returns (presenceID, battleTag, toonID, broadcast, AFK, DND, RIDEnabled).
fn stub_bn_get_info(lua: &Lua, _: ()) -> Result<(Value, Value, Value, Value, Value, Value, Value)> {
    Ok((
        Value::Integer(0),
        Value::String(lua.create_string("SimPlayer#0000")?),
        Value::Nil,
        Value::String(lua.create_string("")?),
        Value::Boolean(false),
        Value::Boolean(false),
        Value::Boolean(false),
    ))
}

/// Register specialization query functions.
fn register_specialization_functions(lua: &Lua, state: Rc<RefCell<SimState>>) -> Result<()> {
    register_spec_basic_queries(lua, Rc::clone(&state))?;
    register_spec_info_lookups(lua)?;
    register_spec_class_lookups(lua, state)?;
    Ok(())
}

use crate::specializations;

/// Paladin class ID.
const PALADIN_CLASS_ID: u32 = 2;

/// Basic spec queries: GetSpecialization, GetSpecializationInfo, GetNumSpecializations.
fn register_spec_basic_queries(lua: &Lua, state: Rc<RefCell<SimState>>) -> Result<()> {
    let globals = lua.globals();

    {
        let state = Rc::clone(&state);
        globals.set(
            "GetSpecialization",
            lua.create_function(move |_, ()| Ok(state.borrow().player.active_spec_index))?,
        )?;
    }
    globals.set(
        "GetSpecializationInfo",
        lua.create_function(|lua, spec_index: i32| {
            let specs: Vec<_> = specializations::specs_for_class(PALADIN_CLASS_ID).collect();
            let idx = (spec_index - 1).clamp(0, specs.len() as i32 - 1) as usize;
            spec_to_multivalue(lua, specs[idx])
        })?,
    )?;
    globals.set(
        "GetNumSpecializations",
        lua.create_function(|_, ()| {
            Ok(specializations::specs_for_class(PALADIN_CLASS_ID).count() as i32)
        })?,
    )?;
    register_spec_role_queries(&globals, lua, state)?;

    Ok(())
}

/// Role and count queries: GetSpecializationRole, GetSpecializationRoleByID, GetNumSpecializationsForClassID.
fn register_spec_role_queries(
    globals: &mlua::Table,
    lua: &Lua,
    state: Rc<RefCell<SimState>>,
) -> Result<()> {
    globals.set(
        "GetSpecializationRole",
        lua.create_function(move |lua, spec_index: Option<i32>| {
            let active = state.borrow().player.active_spec_index;
            let specs: Vec<_> = specializations::specs_for_class(PALADIN_CLASS_ID).collect();
            let idx = (spec_index.unwrap_or(active) - 1).clamp(0, specs.len() as i32 - 1) as usize;
            Ok(Value::String(lua.create_string(specs[idx].role)?))
        })?,
    )?;
    globals.set(
        "GetSpecializationRoleByID",
        lua.create_function(|lua, spec_id: i32| {
            let role = specializations::spec_by_id(spec_id as u32)
                .map(|s| s.role)
                .unwrap_or("DAMAGER");
            Ok(Value::String(lua.create_string(role)?))
        })?,
    )?;
    globals.set(
        "GetNumSpecializationsForClassID",
        lua.create_function(|_, (class_id, _sex): (Option<i32>, Option<i32>)| {
            Ok(class_id.map_or(0, |cid| {
                specializations::specs_for_class(cid as u32).count() as i32
            }))
        })?,
    )?;

    Ok(())
}

/// Convert a SpecInfo to the MultiValue returned by GetSpecializationInfo.
fn spec_to_multivalue(lua: &Lua, spec: &specializations::SpecInfo) -> Result<mlua::MultiValue> {
    Ok(mlua::MultiValue::from_vec(vec![
        Value::Integer(spec.id as i64),
        Value::String(lua.create_string(spec.name)?),
        Value::String(lua.create_string(spec.description)?),
        Value::Integer(spec.icon_file_data_id as i64),
        Value::String(lua.create_string(spec.role)?),
        Value::Integer(spec.primary_stat as i64),
    ]))
}

/// Spec info lookups by ID: GetSpecializationInfoByID, ForSpecID, NameForSpecID.
fn register_spec_info_lookups(lua: &Lua) -> Result<()> {
    let globals = lua.globals();
    globals.set(
        "GetSpecializationInfoByID",
        lua.create_function(spec_info_by_id)?,
    )?;
    globals.set(
        "GetSpecializationInfoForSpecID",
        lua.create_function(spec_info_by_id)?,
    )?;
    // GetSpecializationNameForSpecID(specID) -> name string
    globals.set(
        "GetSpecializationNameForSpecID",
        lua.create_function(|lua, spec_id: Option<i32>| {
            match spec_id.and_then(|spec_id| specializations::spec_by_id(spec_id as u32)) {
                Some(s) => Ok(Value::String(lua.create_string(s.name)?)),
                None => Ok(Value::Nil),
            }
        })?,
    )?;
    Ok(())
}

/// Spec info lookup by class+index: GetSpecializationInfoForClassID.
fn register_spec_class_lookups(lua: &Lua, state: Rc<RefCell<SimState>>) -> Result<()> {
    let globals = lua.globals();
    globals.set(
        "GetSpecializationInfoForClassID",
        lua.create_function(move |lua, (class_id, spec_index): (i32, i32)| {
            let active_spec_index = state.borrow().player.active_spec_index;
            let specs: Vec<_> = specializations::specs_for_class(class_id as u32).collect();
            if spec_index < 1 || spec_index as usize > specs.len() {
                return Ok(mlua::MultiValue::new());
            }
            let spec = specs[(spec_index - 1) as usize];
            let mut vals = spec_to_multivalue(lua, spec)?.into_vec();
            vals.push(Value::Boolean(false)); // isAllowed
            vals.push(Value::Boolean(spec_index == active_spec_index));
            Ok(mlua::MultiValue::from_vec(vals))
        })?,
    )?;
    Ok(())
}

fn spec_info_by_id(lua: &Lua, spec_id: Option<i32>) -> Result<mlua::MultiValue> {
    match spec_id.and_then(|spec_id| specializations::spec_by_id(spec_id as u32)) {
        Some(spec) => spec_to_multivalue(lua, spec),
        None => Ok(mlua::MultiValue::new()),
    }
}

/// Economy functions: money, trade, buyback.
fn register_economy_functions(lua: &Lua, state: Rc<RefCell<SimState>>) -> Result<()> {
    let globals = lua.globals();
    globals.set(
        "GetMoney",
        lua.create_function(move |_, ()| Ok(state.borrow().player.money))?,
    )?;
    globals.set(
        "GetTargetTradeMoney",
        lua.create_function(|_, ()| Ok(0i64))?,
    )?;
    globals.set("GetNumBuybackItems", lua.create_function(|_, ()| Ok(0i32))?)?;
    Ok(())
}

/// Instance/dungeon info functions.
fn register_instance_functions(lua: &Lua, state: Rc<RefCell<SimState>>) -> Result<()> {
    let g = lua.globals();
    g.set(
        "GetRaidDifficultyID",
        lua.create_function(|_, ()| Ok(14i32))?,
    )?;
    g.set(
        "GetLegacyRaidDifficultyID",
        lua.create_function(|_, ()| Ok(1i32))?,
    )?;
    g.set(
        "GetMirrorTimerProgress",
        lua.create_function(|_, _: String| Ok(0i32))?,
    )?;
    g.set(
        "GetInstanceInfo",
        lua.create_function(move |lua, ()| build_instance_info(lua, &state.borrow()))?,
    )?;
    Ok(())
}

/// Returns 10 values: name, type, difficultyID, difficultyName, maxPlayers, ...
fn build_instance_info(lua: &Lua, world: &SimState) -> Result<mlua::MultiValue> {
    Ok(mlua::MultiValue::from_vec(vec![
        Value::String(lua.create_string(&world.world.instance_name)?),
        Value::String(lua.create_string(&world.world.instance_type)?),
        Value::Integer(world.world.instance_difficulty as i64),
        Value::String(lua.create_string("")?),
        Value::Integer(world.world.instance_max_players as i64),
        Value::Integer(0),
        Value::Boolean(false),
        Value::Integer(0),
        Value::Integer(0),
        Value::Integer(0),
    ]))
}

/// Character info functions: titles, item level, RPE state, inventory.
fn register_character_functions(lua: &Lua, state: Rc<RefCell<SimState>>) -> Result<()> {
    register_character_info_stubs(lua, state)?;
    register_character_combat_stubs(lua)?;
    register_paperdoll_ui_stubs(lua)?;
    Ok(())
}

/// Character info stubs: title, item level, inventory quality.
fn register_character_info_stubs(lua: &Lua, state: Rc<RefCell<SimState>>) -> Result<()> {
    let g = lua.globals();
    register_character_info_simple_stubs(lua, &g)?;
    g.set(
        "GetAverageItemLevel",
        lua.create_function(move |_, ()| {
            let ilvl = state.borrow().player.item_level as f64;
            Ok((ilvl, ilvl, ilvl))
        })?,
    )?;
    g.set(
        "GetInventoryItemQuality",
        lua.create_function(|_, _: mlua::MultiValue| Ok(Value::Nil))?,
    )?;
    g.set(
        "GetPlayerTradeMoney",
        lua.create_function(|_, ()| Ok(0i64))?,
    )?;
    g.set(
        "GetRestrictedAccountData",
        lua.create_function(|_, ()| Ok((false, false, false)))?,
    )?;
    g.set("GetSheathState", lua.create_function(|_, ()| Ok(1i32))?)?;
    Ok(())
}

/// Bool and zero-returning character info stubs.
fn register_character_info_simple_stubs(lua: &Lua, g: &mlua::Table) -> Result<()> {
    let false_stub = lua.create_function(|_, _: mlua::MultiValue| Ok(false))?;
    for name in [
        "IsPlayerInRPE",
        "IsInventoryItemLocked",
        "IsActivePlayerNewcomer",
    ] {
        g.set(name, false_stub.clone())?;
    }
    g.set("IsAccountSecured", lua.create_function(|_, ()| Ok(true))?)?;
    let zero = lua.create_function(|_, ()| Ok(0i32))?;
    for name in [
        "GetCurrentTitle",
        "GetSpecializationRoleEnum",
        "GetResSicknessDuration",
    ] {
        g.set(name, zero.clone())?;
    }
    Ok(())
}

/// Combat stat functions reading from PlayerState.stats (computed from gear).
fn register_character_combat_stubs(lua: &Lua) -> Result<()> {
    use crate::lua_api::state::SimState;
    use std::cell::RefCell;
    use std::rc::Rc;

    let globals = lua.globals();
    // UnitStat(unit, statIndex) -> stat, effectiveStat, posBuff, negBuff
    // 1=Str, 2=Agi, 3=Sta, 4=Int
    globals.set(
        "UnitStat",
        lua.create_function(|lua, (_unit, stat_idx): (String, i32)| {
            let val = lua
                .app_data_ref::<Rc<RefCell<SimState>>>()
                .map(|s| {
                    let st = &s.borrow().player.stats;
                    match stat_idx {
                        1 => st.strength,
                        2 => st.agility,
                        3 => st.stamina,
                        4 => st.intellect,
                        _ => 0.0,
                    }
                })
                .unwrap_or(0.0);
            Ok((val, val, 0.0_f64, 0.0_f64))
        })?,
    )?;
    globals.set(
        "GetAttackPowerForStat",
        lua.create_function(|_, (_stat_idx, _stat): (Value, Value)| Ok(0.0_f64))?,
    )?;
    globals.set(
        "GetRangedAttackPowerForStat",
        lua.create_function(|_, (_stat_idx, _stat): (Value, Value)| Ok(0.0_f64))?,
    )?;
    Ok(())
}

/// PaperDoll UI helper stubs: merchant, equipment flyout, tooltip.
fn register_paperdoll_ui_stubs(lua: &Lua) -> Result<()> {
    let globals = lua.globals();
    globals.set(
        "MerchantFrame_UpdateGuildBankRepair",
        lua.create_function(|_, ()| Ok(()))?,
    )?;
    globals.set(
        "MerchantFrame_UpdateCanRepairAll",
        lua.create_function(|_, ()| Ok(()))?,
    )?;
    globals.set(
        "SetItemButtonDesaturated",
        lua.create_function(|_, _args: mlua::MultiValue| Ok(()))?,
    )?;
    globals.set(
        "EquipmentFlyout_UpdateFlyout",
        lua.create_function(|_, _args: mlua::MultiValue| Ok(()))?,
    )?;
    globals.set(
        "EquipmentFlyout_SetTooltipAnchor",
        lua.create_function(|_, _args: mlua::MultiValue| Ok(false))?,
    )?;
    globals.set(
        "GameTooltip_SuppressAutomaticCompareItem",
        lua.create_function(|_, _args: mlua::MultiValue| Ok(()))?,
    )?;
    Ok(())
}

/// Character stat query stubs for PaperDollFrame.
fn register_character_stat_functions(lua: &Lua) -> Result<()> {
    register_attack_power_stubs(lua)?;
    register_avoidance_and_crit_stubs(lua)?;
    register_combat_rating_stubs(lua)?;
    register_character_stat_functions_2(lua)?;
    Ok(())
}

/// AP/SP override stubs.
fn register_attack_power_stubs(lua: &Lua) -> Result<()> {
    let g = lua.globals();
    g.set(
        "HasAPEffectsSpellPower",
        lua.create_function(|_, ()| Ok(false))?,
    )?;
    g.set(
        "HasSPEffectsAttackPower",
        lua.create_function(|_, ()| Ok(false))?,
    )?;
    g.set(
        "GetOverrideAPBySpellPower",
        lua.create_function(|_, ()| Ok(0.0_f64))?,
    )?;
    g.set(
        "GetOverrideSpellPowerByAP",
        lua.create_function(|_, ()| Ok(0.0_f64))?,
    )?;
    Ok(())
}

/// Avoidance and crit chance — reads from PlayerState.stats.
fn register_avoidance_and_crit_stubs(lua: &Lua) -> Result<()> {
    use crate::lua_api::state::SimState;
    use std::cell::RefCell;
    use std::rc::Rc;
    let g = lua.globals();
    g.set("GetBlockChance", lua.create_function(|_, ()| Ok(12.5_f64))?)?;
    g.set("GetParryChance", lua.create_function(|_, ()| Ok(3.0_f64))?)?;
    g.set("GetDodgeChance", lua.create_function(|_, ()| Ok(3.0_f64))?)?;
    g.set(
        "GetCritChance",
        lua.create_function(|lua, ()| {
            Ok(5.0
                + lua
                    .app_data_ref::<Rc<RefCell<SimState>>>()
                    .map(|s| s.borrow().player.stats.crit_pct())
                    .unwrap_or(0.0))
        })?,
    )?;
    g.set(
        "GetRangedCritChance",
        lua.create_function(|_, ()| Ok(0.0_f64))?,
    )?;
    g.set(
        "GetSpellCritChance",
        lua.create_function(|_, _school: Value| Ok(0.0_f64))?,
    )?;
    Ok(())
}

/// Combat rating queries reading from PlayerState.stats.
fn register_combat_rating_stubs(lua: &Lua) -> Result<()> {
    use crate::lua_api::state::SimState;
    use std::cell::RefCell;
    use std::rc::Rc;
    let g = lua.globals();
    g.set(
        "GetCombatRating",
        lua.create_function(|lua, id: i32| {
            let rating = lua
                .app_data_ref::<Rc<RefCell<SimState>>>()
                .map(|s| {
                    let st = &s.borrow().player.stats;
                    match id {
                        9 => st.crit_rating,
                        6 => st.haste_rating,
                        26 => st.mastery_rating,
                        14 => st.versatility_rating,
                        15 => st.speed_rating,
                        17 => st.leech_rating,
                        18 => st.avoidance_rating,
                        _ => 0,
                    }
                })
                .unwrap_or(0);
            Ok(rating)
        })?,
    )?;
    g.set(
        "GetCombatRatingBonus",
        lua.create_function(|lua, id: i32| {
            let bonus = lua
                .app_data_ref::<Rc<RefCell<SimState>>>()
                .map(|s| {
                    let st = &s.borrow().player.stats;
                    match id {
                        9 => st.crit_pct(),
                        6 => st.haste_pct(),
                        26 => st.mastery_pct(),
                        14 => st.versatility_pct(),
                        _ => 0.0,
                    }
                })
                .unwrap_or(0.0);
            Ok(bonus)
        })?,
    )?;
    g.set(
        "GetMaxCombatRatingBonus",
        lua.create_function(|_, _rating_index: i32| Ok(100.0_f64))?,
    )?;
    Ok(())
}

/// Additional character stat query stubs: secondary stats, regen, spell power.
fn register_character_stat_functions_2(lua: &Lua) -> Result<()> {
    register_secondary_stat_stubs(lua)?;
    register_spell_and_defense_stat_stubs(lua)?;
    Ok(())
}

/// Secondary stats reading from PlayerState.stats.
fn register_secondary_stat_stubs(lua: &Lua) -> Result<()> {
    use crate::lua_api::state::SimState;
    use std::cell::RefCell;
    use std::rc::Rc;
    let g = lua.globals();
    g.set(
        "GetCombatRatingBonusForCombatRatingValue",
        lua.create_function(|_, _args: mlua::MultiValue| Ok(0.0_f64))?,
    )?;
    g.set(
        "GetMasteryEffect",
        lua.create_function(|lua, ()| {
            let m = lua
                .app_data_ref::<Rc<RefCell<SimState>>>()
                .map(|s| s.borrow().player.stats.mastery_pct())
                .unwrap_or(0.0);
            Ok((m + 8.0, m)) // base mastery + rating mastery
        })?,
    )?;
    g.set(
        "GetHaste",
        lua.create_function(|lua, ()| {
            Ok(lua
                .app_data_ref::<Rc<RefCell<SimState>>>()
                .map(|s| s.borrow().player.stats.haste_pct())
                .unwrap_or(0.0))
        })?,
    )?;
    g.set(
        "GetMeleeHaste",
        lua.create_function(|lua, ()| {
            Ok(lua
                .app_data_ref::<Rc<RefCell<SimState>>>()
                .map(|s| s.borrow().player.stats.haste_pct())
                .unwrap_or(0.0))
        })?,
    )?;
    g.set(
        "GetVersatilityBonus",
        lua.create_function(|lua, _id: Value| {
            Ok(lua
                .app_data_ref::<Rc<RefCell<SimState>>>()
                .map(|s| s.borrow().player.stats.versatility_pct())
                .unwrap_or(0.0))
        })?,
    )?;
    g.set(
        "GetLifesteal",
        lua.create_function(|lua, ()| {
            Ok(lua
                .app_data_ref::<Rc<RefCell<SimState>>>()
                .map(|s| s.borrow().player.stats.leech_rating as f64 / 100.0)
                .unwrap_or(0.0))
        })?,
    )?;
    g.set(
        "GetAvoidance",
        lua.create_function(|lua, ()| {
            Ok(lua
                .app_data_ref::<Rc<RefCell<SimState>>>()
                .map(|s| s.borrow().player.stats.avoidance_rating as f64 / 72.0)
                .unwrap_or(0.0))
        })?,
    )?;
    g.set(
        "GetSpeed",
        lua.create_function(|lua, ()| {
            Ok(lua
                .app_data_ref::<Rc<RefCell<SimState>>>()
                .map(|s| s.borrow().player.stats.speed_rating as f64 / 50.0)
                .unwrap_or(0.0))
        })?,
    )?;
    g.set(
        "GetStaggerPercentage",
        lua.create_function(|_, ()| Ok(0.0_f64))?,
    )?;
    g.set("GetBonusBarIndex", lua.create_function(|_, ()| Ok(0i32))?)?;
    g.set("GetShieldBlock", lua.create_function(|_, ()| Ok(0.0_f64))?)?;
    Ok(())
}

/// Spell power, regen, defense, and PVP stat stubs.
fn register_spell_and_defense_stat_stubs(lua: &Lua) -> Result<()> {
    let g = lua.globals();
    g.set(
        "GetSpellBonusDamage",
        lua.create_function(|_, _school: Value| Ok(0.0_f64))?,
    )?;
    g.set(
        "GetSpellBonusHealing",
        lua.create_function(|_, ()| Ok(0.0_f64))?,
    )?;
    g.set(
        "GetManaRegen",
        lua.create_function(|_, ()| Ok((0.0_f64, 0.0_f64)))?,
    )?;
    g.set(
        "GetPowerRegen",
        lua.create_function(|_, ()| Ok((0.0_f64, 0.0_f64)))?,
    )?;
    g.set(
        "GetPetSpellBonusDamage",
        lua.create_function(|_, ()| Ok(0.0_f64))?,
    )?;
    g.set(
        "GetArmorEffectiveness",
        lua.create_function(|_, _args: mlua::MultiValue| Ok(0.0_f64))?,
    )?;
    g.set(
        "GetDodgeChanceFromAttribute",
        lua.create_function(|_, ()| Ok(0.0_f64))?,
    )?;
    g.set(
        "GetParryChanceFromAttribute",
        lua.create_function(|_, ()| Ok(0.0_f64))?,
    )?;
    g.set(
        "GetCritChanceProvidesParryEffect",
        lua.create_function(|_, ()| Ok(false))?,
    )?;
    g.set(
        "GetExpertise",
        lua.create_function(|_, ()| Ok((0.0_f64, 0.0_f64, 0.0_f64)))?,
    )?;
    g.set(
        "GetModResilienceDamageReduction",
        lua.create_function(|_, ()| Ok(0.0_f64))?,
    )?;
    g.set(
        "GetPVPGearStatRules",
        lua.create_function(|_, ()| Ok(false))?,
    )?;
    g.set(
        "GetUnitMaxHealthModifier",
        lua.create_function(|_, _unit: Value| Ok(1.0_f64))?,
    )?;
    g.set(
        "UnitHPPerStamina",
        lua.create_function(|_, _unit: Value| Ok(20.0_f64))?,
    )?;
    Ok(())
}

/// Cinematic/cutscene control stubs.
fn register_cinematic_functions(lua: &Lua) -> Result<()> {
    let globals = lua.globals();
    globals.set(
        "MouseOverrideCinematicDisable",
        lua.create_function(|_, ()| Ok(()))?,
    )?;
    globals.set(
        "MouseOverrideCinematicEnable",
        lua.create_function(|_, ()| Ok(()))?,
    )?;
    globals.set("GetCursorMoney", lua.create_function(|_, ()| Ok(0i64))?)?;
    globals.set("GetNumTitles", lua.create_function(|_, ()| Ok(0i32))?)?;
    register_difficulty_and_utility_stubs(lua)
}

/// Difficulty queries and misc utility stubs.
fn register_difficulty_and_utility_stubs(lua: &Lua) -> Result<()> {
    let globals = lua.globals();
    // GetItemLevelColor(itemLevel) -> r, g, b
    globals.set(
        "GetItemLevelColor",
        lua.create_function(|_, _ilvl: Value| Ok((1.0_f64, 1.0_f64, 1.0_f64)))?,
    )?;
    // GetDifficultyInfo(id) -> name, groupType, isHeroic, isChallengeMode, toggleDifficultyID
    globals.set(
        "GetDifficultyInfo",
        lua.create_function(|lua, _id: Value| {
            Ok(mlua::MultiValue::from_vec(vec![
                Value::String(lua.create_string("")?),
                Value::String(lua.create_string("")?),
                Value::Boolean(false),
                Value::Boolean(false),
                Value::Integer(0),
            ]))
        })?,
    )?;
    // IsLegacyDifficulty(difficultyID) -> bool
    globals.set(
        "IsLegacyDifficulty",
        lua.create_function(|_, _id: Value| Ok(false))?,
    )?;
    // BreakUpLargeNumbers(amount) -> formatted string
    globals.set(
        "BreakUpLargeNumbers",
        lua.create_function(|_, amount: Value| {
            let s = match amount {
                Value::Integer(n) => n.to_string(),
                Value::Number(n) => format!("{:.0}", n),
                _ => "0".to_string(),
            };
            Ok(s)
        })?,
    )?;
    Ok(())
}

/// Player movement state functions (read from SimState.movement toggles).
fn register_movement_functions(lua: &Lua, state: Rc<RefCell<SimState>>) -> Result<()> {
    let globals = lua.globals();
    let mk = |field: fn(&super::super::state::MovementState) -> bool| {
        let st = Rc::clone(&state);
        lua.create_function(move |_, ()| Ok(field(&st.borrow().player.movement)))
    };
    globals.set("IsPlayerMoving", mk(|m| m.moving)?)?;
    globals.set("IsMounted", mk(|m| m.mounted)?)?;
    globals.set("IsFlying", mk(|m| m.flying)?)?;
    globals.set("IsFalling", mk(|m| m.falling)?)?;
    globals.set("IsSwimming", mk(|m| m.swimming)?)?;
    globals.set("IsSubmerged", lua.create_function(|_, ()| Ok(false))?)?;
    globals.set("IsFlyableArea", lua.create_function(|_, ()| Ok(false))?)?;
    globals.set(
        "IsAdvancedFlyableArea",
        lua.create_function(|_, ()| Ok(false))?,
    )?;
    globals.set("IsDrivableArea", lua.create_function(|_, ()| Ok(false))?)?;
    globals.set("IsOutOfBounds", lua.create_function(|_, ()| Ok(false))?)?;
    // GetPlayerFacing() -> radians (0.0 = north, increases counterclockwise)
    globals.set("GetPlayerFacing", lua.create_function(|_, ()| Ok(0.0_f64))?)?;
    Ok(())
}

/// Unit query functions used by UnitFrame/PetFrame code.
fn register_unit_functions(lua: &Lua) -> Result<()> {
    let globals = lua.globals();
    globals.set("PetUsesPetFrame", lua.create_function(|_, ()| Ok(false))?)?;
    globals.set(
        "UnitIsPossessed",
        lua.create_function(|_, _unit: Value| Ok(false))?,
    )?;
    globals.set(
        "GetNumShapeshiftForms",
        lua.create_function(|_, ()| Ok(0i32))?,
    )?;
    Ok(())
}
