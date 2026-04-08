//! Unit-related WoW API functions.

use crate::lua_api::SimState;
use mlua::{Lua, MultiValue, Result, Value};
use std::cell::RefCell;
use std::rc::Rc;

/// Class data: (index, display_name, file_name).
const CLASS_DATA: &[(i32, &str, &str)] = &[
    (1, "Warrior", "WARRIOR"),
    (2, "Paladin", "PALADIN"),
    (3, "Hunter", "HUNTER"),
    (4, "Rogue", "ROGUE"),
    (5, "Priest", "PRIEST"),
    (6, "Death Knight", "DEATHKNIGHT"),
    (7, "Shaman", "SHAMAN"),
    (8, "Mage", "MAGE"),
    (9, "Warlock", "WARLOCK"),
    (10, "Monk", "MONK"),
    (11, "Druid", "DRUID"),
    (12, "Demon Hunter", "DEMONHUNTER"),
    (13, "Evoker", "EVOKER"),
];

/// Look up class name and file by 1-based index. Returns None for invalid indices.
fn class_info_by_index(index: i32) -> Option<(&'static str, &'static str)> {
    CLASS_DATA
        .iter()
        .find(|(i, _, _)| *i == index)
        .map(|(_, name, file)| (*name, *file))
}

/// Return true if a unit token is a recognized WoW unit ID.
fn is_known_unit(unit: &str) -> bool {
    matches!(unit, "player" | "target" | "pet" | "focus" | "mouseover")
        || parse_party_index(unit).is_some()
}

/// Resolve a unit name for known unit tokens. Returns None for unknown tokens.
fn resolve_unit_name_with_party(unit: &str, state: &SimState) -> Option<String> {
    if unit == "player" {
        return Some(state.player.name.clone());
    }
    if unit == "target" {
        return Some(
            state
                .current_target
                .as_ref()
                .map(|t| t.name.clone())
                .unwrap_or_else(|| "Unknown".to_string()),
        );
    }
    if let Some(idx) = parse_party_index(unit) {
        if let Some(m) = state.party_members.get(idx) {
            return Some(m.name.to_string());
        }
        return Some("SimUnit".to_string());
    }
    if !is_known_unit(unit) {
        return None;
    }
    Some("SimUnit".to_string())
}

/// Parse a "partyN" unit ID and return the 0-based index if valid.
pub fn parse_party_index(unit: &str) -> Option<usize> {
    unit.strip_prefix("party")
        .and_then(|n| n.parse::<usize>().ok())
        .filter(|&n| n >= 1)
        .map(|n| n - 1)
}

/// Register unit-related global functions.
pub fn register_unit_api(lua: &Lua, state: Rc<RefCell<SimState>>) -> Result<()> {
    register_identity_stubs(lua, state.clone())?;
    register_unit_guid(lua, state.clone())?;
    register_unit_level_exists(lua, state.clone())?;
    register_class_functions(lua, state.clone())?;
    register_name_functions(lua, state.clone())?;
    register_state_functions(lua, state.clone())?;
    register_group_functions(lua, state.clone())?;
    super::unit_health_power_api::register_health_power_functions(lua, state.clone())?;
    register_aura_functions(lua, state.clone())?;
    super::unit_api_extra::register_extra_unit_functions(lua, state.clone())?;
    super::targeting_api::register_targeting_functions(lua, state)?;
    super::unit_combat_api::register_unit_combat_stat_functions(lua)?;
    Ok(())
}

fn player_race_name_file(state: &SimState) -> (&'static str, &'static str) {
    let (name, file, _) = crate::lua_api::state::RACE_DATA
        .get(state.player.race_index)
        .copied()
        .unwrap_or(("Human", "Human", "Alliance"));
    (name, file)
}

fn player_race_faction(state: &SimState) -> &'static str {
    crate::lua_api::state::RACE_DATA
        .get(state.player.race_index)
        .copied()
        .map(|(_, _, f)| f)
        .unwrap_or("Alliance")
}

/// UnitSex returns legacy values (2=Male, 3=Female). UnitSexBase returns Enum.UnitSex (0=Male, 1=Female).
fn register_sex_stubs(lua: &Lua, state: Rc<RefCell<SimState>>) -> Result<()> {
    let g = lua.globals();
    let st = state.clone();
    g.set(
        "UnitSex",
        lua.create_function(move |_, _unit: Option<String>| Ok(st.borrow().player.sex))?,
    )?;
    let st = state;
    g.set(
        "UnitSexBase",
        lua.create_function(move |_, _unit: Option<String>| {
            Ok(match st.borrow().player.sex {
                2 => 0, // Male
                3 => 1, // Female
                _ => 2, // None/Unknown
            })
        })?,
    )?;
    Ok(())
}

fn register_identity_stubs(lua: &Lua, state: Rc<RefCell<SimState>>) -> Result<()> {
    let globals = lua.globals();
    let st = state.clone();
    globals.set(
        "UnitRace",
        lua.create_function(move |lua, _unit: Option<String>| {
            let (name, file) = player_race_name_file(&st.borrow());
            Ok(MultiValue::from_vec(vec![
                Value::String(lua.create_string(name)?),
                Value::String(lua.create_string(file)?),
            ]))
        })?,
    )?;
    register_sex_stubs(lua, state.clone())?;
    globals.set(
        "UnitEffectiveLevel",
        lua.create_function(|_, _unit: Option<String>| Ok(80))?,
    )?;
    globals.set(
        "UnitFactionGroup",
        lua.create_function(move |lua, _unit: Option<String>| {
            let faction = player_race_faction(&state.borrow());
            Ok(MultiValue::from_vec(vec![
                Value::String(lua.create_string(faction)?),
                Value::String(lua.create_string(faction)?),
            ]))
        })?,
    )?;
    Ok(())
}

/// Register UnitGUID with party/target awareness.
fn register_unit_guid(lua: &Lua, state: Rc<RefCell<SimState>>) -> Result<()> {
    lua.globals().set(
        "UnitGUID",
        lua.create_function(move |lua, unit: Option<String>| {
            let Some(unit) = unit else {
                return Ok(Value::Nil);
            };
            if unit == "player" {
                return Ok(Value::String(lua.create_string("Player-0000-00000001")?));
            }
            if unit == "target" {
                let s = state.borrow();
                let guid = s
                    .current_target
                    .as_ref()
                    .map(|t| t.guid.clone())
                    .unwrap_or_else(|| "Creature-0000-00000000".into());
                return Ok(Value::String(lua.create_string(&guid)?));
            }
            if let Some(idx) = parse_party_index(&unit)
                && idx < state.borrow().party_members.len()
            {
                let guid = format!("Player-0000-0000000{}", idx + 2);
                return Ok(Value::String(lua.create_string(&guid)?));
            }
            Ok(Value::String(lua.create_string("Creature-0000-00000000")?))
        })?,
    )
}

/// Register UnitLevel and UnitExists with party/target awareness.
fn register_unit_level_exists(lua: &Lua, state: Rc<RefCell<SimState>>) -> Result<()> {
    let globals = lua.globals();

    let st = state.clone();
    globals.set(
        "UnitLevel",
        lua.create_function(move |_, unit: Option<String>| {
            let Some(unit) = unit else { return Ok(0) };
            if unit == "target" {
                let s = st.borrow();
                return Ok(s.current_target.as_ref().map(|t| t.level).unwrap_or(1));
            }
            if let Some(idx) = parse_party_index(&unit) {
                let s = st.borrow();
                if let Some(m) = s.party_members.get(idx) {
                    return Ok(m.level);
                }
            }
            Ok(st.borrow().player.level)
        })?,
    )?;

    globals.set(
        "UnitExists",
        lua.create_function(move |_, unit: Value| {
            let unit = match unit {
                Value::String(s) => s.to_str()?.to_string(),
                _ => return Ok(false),
            };
            if matches!(unit.as_str(), "player" | "pet") {
                return Ok(true);
            }
            if unit == "target" {
                return Ok(state.borrow().current_target.is_some());
            }
            if let Some(idx) = parse_party_index(&unit) {
                return Ok(idx < state.borrow().party_members.len());
            }
            Ok(false)
        })?,
    )
}

/// Register UnitClass, UnitClassBase, GetNumClasses, GetClassInfo,
/// LocalizedClassList.
fn register_class_functions(lua: &Lua, state: Rc<RefCell<SimState>>) -> Result<()> {
    register_unit_class(lua, state.clone())?;
    register_class_lookup_functions(lua, state)
}

/// Resolve class (name, file, index) for the given unit token from state.
fn resolve_unit_class(unit: &str, state: &SimState) -> (&'static str, &'static str, i32) {
    if unit == "target" {
        if let Some(t) = &state.current_target {
            let (n, f) = class_info_by_index(t.class_index).unwrap_or(("Warrior", "WARRIOR"));
            return (n, f, t.class_index);
        }
        return ("Warrior", "WARRIOR", 1);
    }
    if let Some(i) = parse_party_index(unit) {
        if let Some(m) = state.party_members.get(i) {
            let (n, f) = class_info_by_index(m.class_index).unwrap_or(("Warrior", "WARRIOR"));
            return (n, f, m.class_index);
        }
        return ("Warrior", "WARRIOR", 1);
    }
    let (n, f) = class_info_by_index(state.player.class_index).unwrap_or(("Warrior", "WARRIOR"));
    (n, f, state.player.class_index)
}

/// Register UnitClass with party member awareness.
fn register_unit_class(lua: &Lua, state: Rc<RefCell<SimState>>) -> Result<()> {
    lua.globals().set(
        "UnitClass",
        lua.create_function(move |lua, unit: Option<String>| {
            let unit = unit.unwrap_or_default();
            let (name, file, idx) = resolve_unit_class(&unit, &state.borrow());
            Ok(MultiValue::from_vec(vec![
                Value::String(lua.create_string(name)?),
                Value::String(lua.create_string(file)?),
                Value::Integer(idx as i64),
            ]))
        })?,
    )
}

/// Register UnitClassBase and GetNumClasses.
fn register_class_base_functions(lua: &Lua, state: Rc<RefCell<SimState>>) -> Result<()> {
    let globals = lua.globals();
    globals.set(
        "UnitClassBase",
        lua.create_function(move |lua, _unit: Option<String>| {
            let s = state.borrow();
            let (_, file) =
                class_info_by_index(s.player.class_index).unwrap_or(("Warrior", "WARRIOR"));
            Ok(Value::String(lua.create_string(file)?))
        })?,
    )?;
    globals.set(
        "GetNumClasses",
        lua.create_function(|_, ()| Ok(CLASS_DATA.len() as i32))?,
    )?;
    Ok(())
}

/// Register GetClassInfo and LocalizedClassList.
fn register_class_info_functions(lua: &Lua) -> Result<()> {
    let globals = lua.globals();
    globals.set(
        "GetClassInfo",
        lua.create_function(|lua, class_index: i32| {
            let Some((name, file)) = class_info_by_index(class_index) else {
                return Ok(MultiValue::new());
            };
            Ok(MultiValue::from_vec(vec![
                Value::String(lua.create_string(name)?),
                Value::String(lua.create_string(file)?),
                Value::Integer(class_index as i64),
            ]))
        })?,
    )?;
    globals.set(
        "LocalizedClassList",
        lua.create_function(|lua, _is_female: Option<bool>| {
            let classes = lua.create_table()?;
            for &(_, name, file) in CLASS_DATA {
                classes.set(file, name)?;
            }
            Ok(classes)
        })?,
    )?;
    Ok(())
}

/// Register class lookup functions (some need state for player class).
fn register_class_lookup_functions(lua: &Lua, state: Rc<RefCell<SimState>>) -> Result<()> {
    register_class_base_functions(lua, state)?;
    register_class_info_functions(lua)
}

/// Register UnitName and UnitNameUnmodified.
///
/// Returns nil for unknown or invalid unit tokens instead of raising.
fn register_unit_name_strict(lua: &Lua, state: Rc<RefCell<SimState>>) -> Result<()> {
    let globals = lua.globals();
    for &fn_name in &["UnitName", "UnitNameUnmodified"] {
        let st = state.clone();
        globals.set(
            fn_name,
            lua.create_function(move |lua, unit: Value| {
                let unit_token = match unit {
                    Value::String(s) => s.to_str()?.to_string(),
                    _ => return Ok(MultiValue::from_vec(vec![Value::Nil, Value::Nil])),
                };

                let Some(name) = resolve_unit_name_with_party(&unit_token, &st.borrow()) else {
                    if let Some(fallback) = fallback_name_for_unknown_unit(&unit_token) {
                        return Ok(MultiValue::from_vec(vec![
                            Value::String(lua.create_string(fallback)?),
                            Value::Nil,
                        ]));
                    }
                    return Ok(MultiValue::from_vec(vec![Value::Nil, Value::Nil]));
                };
                Ok(MultiValue::from_vec(vec![
                    Value::String(lua.create_string(name)?),
                    Value::Nil,
                ]))
            })?,
        )?;
    }
    Ok(())
}

/// Register UnitFullName (returns name + realm).
fn register_unit_full_name(lua: &Lua, state: Rc<RefCell<SimState>>) -> Result<()> {
    lua.globals().set(
        "UnitFullName",
        lua.create_function(move |lua, unit: Option<String>| {
            let name = resolve_unit_name_with_party(&unit.unwrap_or_default(), &state.borrow())
                .unwrap_or_else(|| "SimUnit".to_string());
            Ok(MultiValue::from_vec(vec![
                Value::String(lua.create_string(name)?),
                Value::String(lua.create_string("SimRealm")?),
            ]))
        })?,
    )
}

/// Register GetUnitName and UnitPVPName (single string returns, fallback to SimUnit).
fn register_unit_name_display(lua: &Lua, state: Rc<RefCell<SimState>>) -> Result<()> {
    let globals = lua.globals();
    let st = state.clone();
    globals.set(
        "GetUnitName",
        lua.create_function(move |lua, (unit, _): (Option<String>, Option<bool>)| {
            let name = resolve_unit_name_with_party(&unit.unwrap_or_default(), &st.borrow())
                .unwrap_or_else(|| "SimUnit".to_string());
            Ok(Value::String(lua.create_string(name)?))
        })?,
    )?;
    globals.set(
        "UnitPVPName",
        lua.create_function(move |lua, unit: Option<String>| {
            let name = resolve_unit_name_with_party(&unit.unwrap_or_default(), &state.borrow())
                .unwrap_or_else(|| "SimUnit".to_string());
            Ok(Value::String(lua.create_string(name)?))
        })?,
    )?;
    Ok(())
}

fn fallback_name_for_unknown_unit(unit: &str) -> Option<&str> {
    if unit.is_empty() || unit == "target" || unit == "focus" || unit.starts_with("party") {
        return None;
    }
    Some(unit)
}

/// Register UnitFullName, GetUnitName, UnitPVPName (lenient: fallback to SimUnit).
fn register_unit_name_lenient(lua: &Lua, state: Rc<RefCell<SimState>>) -> Result<()> {
    register_unit_full_name(lua, state.clone())?;
    register_unit_name_display(lua, state)
}

/// Register UnitName, UnitNameUnmodified, UnitFullName, GetUnitName.
fn register_name_functions(lua: &Lua, state: Rc<RefCell<SimState>>) -> Result<()> {
    register_unit_name_strict(lua, state.clone())?;
    register_unit_name_lenient(lua, state)
}

/// Register unit state boolean functions: alive/dead, AFK/DND, combat
/// relations, visibility.
fn register_state_functions(lua: &Lua, state: Rc<RefCell<SimState>>) -> Result<()> {
    register_state_boolean_stubs(lua, state.clone())?;
    register_death_functions(lua, state.clone())?;
    register_state_comparisons(lua, state.clone())?;
    register_state_relations(lua, state)
}

/// Register UnitIsDead, UnitIsDeadOrGhost with player health awareness.
fn register_death_functions(lua: &Lua, state: Rc<RefCell<SimState>>) -> Result<()> {
    let globals = lua.globals();

    let st = state.clone();
    globals.set(
        "UnitIsDead",
        lua.create_function(move |_, unit: Option<String>| {
            if unit.as_deref() == Some("player") {
                return Ok(st.borrow().player.health <= 0);
            }
            Ok(false)
        })?,
    )?;
    globals.set(
        "UnitIsDeadOrGhost",
        lua.create_function(move |_, unit: Option<String>| {
            if unit.as_deref() == Some("player") {
                return Ok(state.borrow().player.health <= 0);
            }
            Ok(false)
        })?,
    )?;
    Ok(())
}

/// Register single-unit boolean stubs (always false or always true).
fn register_state_boolean_stubs(lua: &Lua, state: Rc<RefCell<SimState>>) -> Result<()> {
    let globals = lua.globals();
    for &name in &[
        "UnitIsGhost",
        "UnitIsAFK",
        "UnitIsDND",
        "UnitIsTapDenied",
        "UnitIsCorpse",
        "UnitIsWildBattlePet",
        "UnitIsBattlePetCompanion",
        "UnitIsBossMob",
        "UnitIsQuestBoss",
        "UnitLeadsAnyGroup",
        "UnitIsUnconscious",
        "UnitIsBattlePet",
        "UnitIsOtherPlayersBattlePet",
        "UnitIsOtherPlayersPet",
    ] {
        globals.set(
            name,
            lua.create_function(|_, _unit: Option<String>| Ok(false))?,
        )?;
    }
    globals.set(
        "UnitIsConnected",
        lua.create_function(|_, _unit: Option<String>| Ok(true))?,
    )?;
    globals.set(
        "UnitIsVisible",
        lua.create_function(move |_, unit: Option<String>| {
            Ok(match unit.as_deref() {
                Some("player" | "pet") => true,
                Some("target") => state.borrow().current_target.is_some(),
                _ => false,
            })
        })?,
    )?;
    globals.set(
        "UnitCanCooperate",
        lua.create_function(|_, (_u1, _u2): (String, String)| Ok(false))?,
    )?;
    Ok(())
}

/// Register unit comparison functions (player checks, unit identity).
fn register_state_comparisons(lua: &Lua, state: Rc<RefCell<SimState>>) -> Result<()> {
    let globals = lua.globals();
    globals.set(
        "UnitIsPlayer",
        lua.create_function(move |_, unit: Option<String>| {
            let Some(unit) = unit else { return Ok(false) };
            if unit == "player" {
                return Ok(true);
            }
            if unit == "target" {
                return Ok(state
                    .borrow()
                    .current_target
                    .as_ref()
                    .map(|t| t.is_player)
                    .unwrap_or(false));
            }
            if let Some(idx) = parse_party_index(&unit) {
                return Ok(idx < state.borrow().party_members.len());
            }
            Ok(false)
        })?,
    )?;
    globals.set(
        "UnitPlayerControlled",
        lua.create_function(|_, unit: Option<String>| {
            Ok(matches!(unit.as_deref(), Some("player" | "pet")))
        })?,
    )?;
    globals.set(
        "UnitIsUnit",
        lua.create_function(|_, (u1, u2): (Option<String>, Option<String>)| {
            Ok(u1.is_some() && u1 == u2)
        })?,
    )?;
    Ok(())
}

/// Register two-unit relation functions with target awareness.
fn register_state_relations(lua: &Lua, state: Rc<RefCell<SimState>>) -> Result<()> {
    let globals = lua.globals();

    let st = state.clone();
    globals.set(
        "UnitIsEnemy",
        lua.create_function(move |_, (_u1, u2): (Option<String>, Option<String>)| {
            if u2.as_deref() == Some("target") {
                return Ok(st
                    .borrow()
                    .current_target
                    .as_ref()
                    .map(|t| t.is_enemy)
                    .unwrap_or(false));
            }
            Ok(false)
        })?,
    )?;
    let st = state.clone();
    globals.set(
        "UnitCanAttack",
        lua.create_function(move |_, (_u1, u2): (Option<String>, Option<String>)| {
            if u2.as_deref() == Some("target") {
                return Ok(st
                    .borrow()
                    .current_target
                    .as_ref()
                    .map(|t| t.is_enemy)
                    .unwrap_or(false));
            }
            Ok(false)
        })?,
    )?;
    let st = state.clone();
    globals.set(
        "UnitIsFriend",
        lua.create_function(move |_, (_u1, u2): (Option<String>, Option<String>)| {
            if u2.as_deref() == Some("target") {
                return Ok(st
                    .borrow()
                    .current_target
                    .as_ref()
                    .map(|t| !t.is_enemy)
                    .unwrap_or(true));
            }
            Ok(true)
        })?,
    )?;
    globals.set(
        "UnitCanAssist",
        lua.create_function(move |_, (_u1, u2): (Option<String>, Option<String>)| {
            if u2.as_deref() == Some("target") {
                return Ok(state
                    .borrow()
                    .current_target
                    .as_ref()
                    .map(|t| !t.is_enemy)
                    .unwrap_or(true));
            }
            Ok(true)
        })?,
    )?;
    globals.set(
        "UnitInRange",
        lua.create_function(|_, _unit: Option<String>| Ok((true, true)))?,
    )?;
    Ok(())
}

/// Register UnitIsGroupLeader with party member awareness.
fn register_unit_is_group_leader(lua: &Lua, state: Rc<RefCell<SimState>>) -> Result<()> {
    lua.globals().set(
        "UnitIsGroupLeader",
        lua.create_function(move |_, unit: Option<String>| {
            let Some(unit) = unit else { return Ok(false) };
            if let Some(idx) = parse_party_index(&unit) {
                let s = state.borrow();
                if let Some(m) = s.party_members.get(idx) {
                    return Ok(m.is_leader);
                }
            }
            Ok(false)
        })?,
    )
}

/// Register UnitInParty, UnitInRaid, UnitIsGroupLeader, UnitIsGroupAssistant.
fn register_group_functions(lua: &Lua, state: Rc<RefCell<SimState>>) -> Result<()> {
    let globals = lua.globals();
    let st = state.clone();
    globals.set(
        "UnitInParty",
        lua.create_function(move |_, unit: Option<String>| {
            let Some(unit) = unit else { return Ok(false) };
            Ok(parse_party_index(&unit).map_or(false, |idx| idx < st.borrow().party_members.len()))
        })?,
    )?;
    globals.set(
        "UnitInRaid",
        lua.create_function(|_, _unit: Option<String>| Ok(Value::Nil))?,
    )?;
    register_unit_is_group_leader(lua, state)?;
    globals.set(
        "UnitIsGroupAssistant",
        lua.create_function(|_, _unit: Option<String>| Ok(false))?,
    )?;
    Ok(())
}

/// Register UnitAura, UnitBuff, UnitDebuff, GetPlayerAuraBySpellID,
/// and the AuraUtil namespace. Delegated to aura_api module.
fn register_aura_functions(lua: &Lua, state: Rc<RefCell<SimState>>) -> Result<()> {
    super::aura_api::register_aura_api(lua, state)
}
