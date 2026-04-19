//! Misc unit globals that do not fit the core group-query bucket.

use crate::lua_api::methods::{borrow_state, create_string, create_table};
use crate::lua_bridge::FromStack;
use rilua::vm::state::LuaState;
use rilua::{LuaApiMut, LuaResult, Val};

const SIM_REALM: &str = "SimRealm";

fn unit_name_for(state: &mut LuaState, unit: &str) -> String {
    let Ok(sim) = borrow_state(state) else {
        return "Unknown".to_string();
    };
    match unit {
        "player" | "pet" | "vehicle" => sim.player.name.clone(),
        "target" => sim
            .current_target
            .as_ref()
            .map(|target| target.name.clone())
            .unwrap_or_else(|| "Unknown".to_string()),
        "focus" => sim
            .current_focus
            .as_ref()
            .map(|target| target.name.clone())
            .unwrap_or_else(|| "Unknown".to_string()),
        other => {
            if let Some(idx) = crate::lua_api::globals::unit_api::parse_party_index(other) {
                sim.party_members
                    .get(idx)
                    .map(|member| member.name.clone())
                    .unwrap_or_else(|| "Unknown".to_string())
            } else {
                "Unknown".to_string()
            }
        }
    }
}

fn unit_name_string(state: &mut LuaState) -> LuaResult<u32> {
    let unit = Option::<String>::from_stack(state, 1)?.unwrap_or_default();
    let name = unit_name_for(state, &unit);
    let name = create_string(state, &name);
    state.push(name);
    Ok(1)
}

fn get_unit_name(state: &mut LuaState) -> LuaResult<u32> {
    unit_name_string(state)
}

fn unit_full_name(state: &mut LuaState) -> LuaResult<u32> {
    let unit = Option::<String>::from_stack(state, 1)?.unwrap_or_default();
    let name = unit_name_for(state, &unit);
    let name = create_string(state, &name);
    let realm = create_string(state, SIM_REALM);
    state.push(name);
    state.push(realm);
    Ok(2)
}

fn unit_class_base(state: &mut LuaState) -> LuaResult<u32> {
    let unit = Option::<String>::from_stack(state, 1)?.unwrap_or_default();
    let class_index = {
        let Ok(sim) = borrow_state(state) else {
            state.push(Val::Nil);
            return Ok(1);
        };
        match unit.as_str() {
            "player" | "pet" | "vehicle" => sim.player.class_index,
            "target" => sim
                .current_target
                .as_ref()
                .map(|target| target.class_index)
                .unwrap_or(0),
            "focus" => sim
                .current_focus
                .as_ref()
                .map(|target| target.class_index)
                .unwrap_or(0),
            other => {
                if crate::lua_api::globals::unit_api::parse_party_index(other).is_some() {
                    1
                } else {
                    0
                }
            }
        }
    };
    let (_, class_file, _) = crate::lua_api::game_data::class_info_by_index(class_index);
    let class_file = create_string(state, class_file);
    state.push(class_file);
    Ok(1)
}

fn unit_guid(state: &mut LuaState) -> LuaResult<u32> {
    let unit = Option::<String>::from_stack(state, 1)?.unwrap_or_default();
    let guid = {
        let Ok(sim) = borrow_state(state) else {
            return Ok(0);
        };
        match unit.as_str() {
            "player" => "Player-0000-00000001".to_string(),
            "target" => sim
                .current_target
                .as_ref()
                .map(|target| target.guid.clone())
                .unwrap_or_else(|| "Creature-0000-00000000".to_string()),
            "focus" => sim
                .current_focus
                .as_ref()
                .map(|target| target.guid.clone())
                .unwrap_or_else(|| "Creature-0000-00000000".to_string()),
            other => {
                if let Some(idx) = crate::lua_api::globals::unit_api::parse_party_index(other) {
                    format!("Player-0000-000000{:02}", idx + 2)
                } else {
                    "Creature-0000-00000000".to_string()
                }
            }
        }
    };
    let guid = create_string(state, &guid);
    state.push(guid);
    Ok(1)
}

fn unit_creature_family(state: &mut LuaState) -> LuaResult<u32> {
    let _ = Option::<String>::from_stack(state, 1)?;
    state.push(Val::Nil);
    Ok(1)
}

fn unit_player_controlled(state: &mut LuaState) -> LuaResult<u32> {
    let unit = Option::<String>::from_stack(state, 1)?.unwrap_or_default();
    let controlled = match unit.as_str() {
        "player" | "pet" | "vehicle" => true,
        "target" => borrow_state(state)?
            .current_target
            .as_ref()
            .is_some_and(|target| target.is_player),
        "focus" => borrow_state(state)?
            .current_focus
            .as_ref()
            .is_some_and(|target| target.is_player),
        other => crate::lua_api::globals::unit_api::parse_party_index(other).is_some(),
    };
    state.push(Val::Bool(controlled));
    Ok(1)
}

fn unit_is_afk(state: &mut LuaState) -> LuaResult<u32> {
    let _ = Option::<String>::from_stack(state, 1)?;
    state.push(Val::Bool(false));
    Ok(1)
}

fn unit_is_dnd(state: &mut LuaState) -> LuaResult<u32> {
    let _ = Option::<String>::from_stack(state, 1)?;
    state.push(Val::Bool(false));
    Ok(1)
}

fn unit_is_unit(state: &mut LuaState) -> LuaResult<u32> {
    let lhs = Option::<String>::from_stack(state, 1)?.unwrap_or_default();
    let rhs = Option::<String>::from_stack(state, 2)?.unwrap_or_default();
    state.push(Val::Bool(lhs == rhs));
    Ok(1)
}

fn unit_threat_situation(state: &mut LuaState) -> LuaResult<u32> {
    let _ = Option::<String>::from_stack(state, 1)?;
    let _ = Option::<String>::from_stack(state, 2)?;
    state.push(Val::Nil);
    Ok(1)
}

fn unit_affecting_combat(state: &mut LuaState) -> LuaResult<u32> {
    let unit = Option::<String>::from_stack(state, 1)?.unwrap_or_default();
    let in_combat = matches!(unit.as_str(), "player" | "pet" | "vehicle")
        && borrow_state(state)?.player.in_combat;
    state.push(Val::Bool(in_combat));
    Ok(1)
}

fn get_corruption(state: &mut LuaState) -> LuaResult<u32> {
    state.push(Val::Num(0.0));
    Ok(1)
}

fn get_corruption_resistance(state: &mut LuaState) -> LuaResult<u32> {
    state.push(Val::Num(0.0));
    Ok(1)
}

fn get_negative_corruption_effect_info(state: &mut LuaState) -> LuaResult<u32> {
    let effects = create_table(state);
    state.push(effects);
    Ok(1)
}

pub fn register_all(lua: &mut rilua::Lua) -> crate::Result<()> {
    LuaApiMut::register_function(lua, "GetUnitName", get_unit_name)?;
    LuaApiMut::register_function(lua, "UnitFullName", unit_full_name)?;
    LuaApiMut::register_function(lua, "UnitClassBase", unit_class_base)?;
    LuaApiMut::register_function(lua, "UnitGUID", unit_guid)?;
    LuaApiMut::register_function(lua, "UnitCreatureFamily", unit_creature_family)?;
    LuaApiMut::register_function(lua, "UnitPlayerControlled", unit_player_controlled)?;
    LuaApiMut::register_function(lua, "UnitIsAFK", unit_is_afk)?;
    LuaApiMut::register_function(lua, "UnitIsDND", unit_is_dnd)?;
    LuaApiMut::register_function(lua, "UnitIsUnit", unit_is_unit)?;
    LuaApiMut::register_function(lua, "UnitThreatSituation", unit_threat_situation)?;
    LuaApiMut::register_function(lua, "UnitAffectingCombat", unit_affecting_combat)?;
    LuaApiMut::register_function(lua, "GetCorruption", get_corruption)?;
    LuaApiMut::register_function(lua, "GetCorruptionResistance", get_corruption_resistance)?;
    LuaApiMut::register_function(
        lua,
        "GetNegativeCorruptionEffectInfo",
        get_negative_corruption_effect_info,
    )?;
    Ok(())
}
