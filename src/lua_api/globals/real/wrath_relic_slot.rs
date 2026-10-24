//! Wrath relic capability from supplied ChrClasses flags, not equipment.

use std::collections::HashMap;

use crate::client_profile::{ACTIVE, ClientProfile};
use crate::lua_api::game_data::TargetInfo;
use crate::lua_api::globals::targeting_verbs::resolve_unit_snapshot;
use crate::lua_api::globals::unit_misc::guid_for_unit;
use crate::lua_api::methods::borrow_state;
use crate::lua_api::state::SimState;
use crate::lua_bridge::{FromStack, stack_val};
use rilua::vm::state::LuaState;
use rilua::{LuaApiMut, LuaResult, Val, runtime_error};

/// Host inputs, empty until supplied. Native Q1 does not establish class mappings.
#[derive(Default)]
pub struct RelicSlotState {
    pub class_flags: HashMap<i32, u32>,
    /// Resolved native tokens/complete chains not represented by shared unit state.
    /// Keys are lowercase; snapshots carry player identity and class, not gear.
    pub units: HashMap<String, TargetInfo>,
}

fn recognizes_token(token: &str) -> bool {
    let mut base = token;
    while let Some(rest) = base
        .strip_suffix("-target")
        .or_else(|| base.strip_suffix("-pet"))
    {
        base = rest;
    }
    if matches!(
        base,
        "player" | "vehicle" | "pet" | "target" | "mouseover" | "focus" | "npc" | "questnpc"
    ) {
        return true;
    }
    [
        "partypet",
        "party",
        "raidpet",
        "raid",
        "boss",
        "arenapet",
        "arena",
        "commentator",
    ]
    .iter()
    .any(|prefix| {
        base.strip_prefix(prefix)
            .is_some_and(|index| !index.is_empty() && index.bytes().all(|b| b.is_ascii_digit()))
    })
}

fn resolve_native_unit(sim: &SimState, token: &str) -> Option<TargetInfo> {
    let token = token.to_ascii_lowercase();
    if token == "none" {
        return None;
    }
    if recognizes_token(&token) {
        if let Some(unit) = sim.wrath_relic_slot.units.get(&token) {
            return Some(unit.clone());
        }
        // Shared resolution's self/enemy/softinteract aliases are not native Q1 tokens.
        return resolve_unit_snapshot(sim, &token);
    }
    resolve_guid(sim, &token)
}

fn resolve_guid(sim: &SimState, guid: &str) -> Option<TargetInfo> {
    if guid_for_unit(sim, "player").eq_ignore_ascii_case(guid) {
        return resolve_unit_snapshot(sim, "player");
    }
    for index in 1..=sim.party_members.len() {
        let unit = resolve_unit_snapshot(sim, &format!("party{index}"));
        if unit
            .as_ref()
            .is_some_and(|unit| unit.guid.eq_ignore_ascii_case(guid))
        {
            return unit;
        }
    }
    sim.current_target
        .iter()
        .chain(sim.current_focus.iter())
        .chain(sim.enemy_pool.iter())
        .chain(sim.wrath_relic_slot.units.values())
        .find(|unit| unit.guid.eq_ignore_ascii_case(guid))
        .cloned()
}

fn unit_has_relic_slot(state: &mut LuaState) -> LuaResult<u32> {
    let argument = stack_val(state, 1);
    let token = match argument {
        Val::Str(_) => Some(String::from_stack(state, 1)?),
        // Lua 5.1 numbers pass lua_isstring. Their decimal text is neither
        // a native unit token nor a modeled GUID, so they resolve to no unit.
        Val::Num(_) => None,
        _ => return Err(runtime_error("Usage: UnitHasRelicSlot(\"unit\")")),
    };
    let sim = borrow_state(state)?;
    let capable = token
        .as_deref()
        .and_then(|token| resolve_native_unit(&sim, token))
        .filter(|unit| unit.is_player)
        .and_then(|unit| sim.wrath_relic_slot.class_flags.get(&unit.class_index))
        .is_some_and(|flags| flags & 8 != 0);
    drop(sim);
    state.push(if capable { Val::Num(1.0) } else { Val::Nil });
    Ok(1)
}

pub(crate) fn register_all(lua: &mut rilua::Lua) -> LuaResult<()> {
    if ACTIVE == ClientProfile::Wrath {
        LuaApiMut::register_function(lua, "UnitHasRelicSlot", unit_has_relic_slot)?;
    }
    Ok(())
}
