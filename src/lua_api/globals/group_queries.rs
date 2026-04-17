//! Group / unit-query globals backed by `SimState`.
//!
//! Mirrors `unit_api_extra.rs`'s mlua implementations so that
//! `A_Admin.SetPartySize(N)` actually makes `UnitExists("party1")`,
//! `IsInGroup()`, `GetNumGroupMembers()`, `UnitName("party1")`,
//! `UnitClass("party1")`, and `UnitLevel("party1")` report real
//! simulated state. Without these overrides the env_init.rs stubs keep
//! returning the player/default values, and PartyFrame never leaves its
//! XML placeholder path.
//!
//! Called once from `admin::register_all` after the A_Admin table
//! is installed, so the globals are in place before any addon script runs.
//!
//! Registering these here keeps the admin module focused on the
//! A_Admin RustFn table; group/unit queries have different semantics and
//! share no code with it.

use crate::lua_api::game_data::{CLASS_LABELS, RACE_DATA};
use crate::lua_api::methods::{borrow_state, create_string, create_table, table_get, table_set};
use crate::lua_bridge::FromStack;
use rilua::vm::closure::{Closure, RustClosure};
use rilua::vm::state::LuaState;
use rilua::{LuaResult, RustFn, Val};

pub fn register_all(state: &mut LuaState) {
    set_global(state, "GetNumGroupMembers", get_num_group_members);
    set_global(state, "GetNumSubgroupMembers", get_num_subgroup_members);
    set_global(state, "GetNumPartyMembers", get_num_subgroup_members);
    set_global(state, "IsInGroup", is_in_group);
    set_global(state, "IsInRaid", is_in_raid);
    set_global(state, "IsPartyLFG", is_party_lfg);
    set_global(state, "IsGroupLeader", is_group_leader);
    set_global(state, "IsEveryoneAssistant", is_everyone_assistant);
    set_global(state, "UnitExists", unit_exists);
    set_global(state, "UnitName", unit_name);
    set_global(state, "UnitNameUnmodified", unit_name_unmodified);
    set_global(state, "UnitClass", unit_class);
    set_global(state, "UnitLevel", unit_level);
    set_global(state, "UnitClassification", unit_classification);
    set_global(state, "UnitCreatureType", unit_creature_type);
    set_global(
        state,
        "UnitTreatAsPlayerForDisplay",
        unit_treat_as_player_for_display,
    );
    set_global(state, "UnitIsFriend", unit_is_friend);
    set_global(state, "UnitCanAttack", unit_can_attack);
    set_global(state, "UnitCanAssist", unit_can_assist);
    set_global(state, "UnitCanCooperate", unit_can_cooperate);
    set_global(state, "UnitInParty", unit_in_party);
    set_global(state, "UnitInRaid", unit_in_raid);
    set_global(state, "UnitInOtherParty", unit_in_other_party);
    set_global(state, "UnitInRange", unit_in_range);
    set_global(state, "UnitInBattleground", unit_in_battleground);
    set_global(state, "UnitFactionGroup", unit_faction_group);
    set_global(state, "UnitIsGroupLeader", unit_is_group_leader);
    set_global(state, "UnitIsGroupAssistant", unit_is_group_assistant);
    set_global(state, "UnitSelectionColor", unit_selection_color);
    set_global(state, "IsPartyWorldPVP", always_false);
    set_global(state, "UnitHasLFGDeserter", always_false);
    ensure_string_join(state);
}

fn set_global(state: &mut LuaState, name: &'static str, func: RustFn) {
    let global = state.global;
    let function = make_function(state, name, func);
    table_set(state, Val::Table(global), name, function);
    state.gc.barrier_back(global);
}

fn make_function(state: &mut LuaState, name: &'static str, func: RustFn) -> Val {
    let closure = Closure::Rust(RustClosure::new(func, name));
    Val::Function(state.gc.alloc_closure(closure))
}

fn ensure_global_table(state: &mut LuaState, name: &'static str) -> Val {
    let global = Val::Table(state.global);
    match table_get(state, global, name) {
        table @ Val::Table(_) => table,
        _ => {
            let table = create_table(state);
            table_set(state, global, name, table);
            table
        }
    }
}

fn ensure_string_join(state: &mut LuaState) {
    let string_table = ensure_global_table(state, "string");
    if matches!(table_get(state, string_table, "join"), Val::Function(_)) {
        return;
    }
    let join = make_function(state, "string.join", string_join);
    table_set(state, string_table, "join", join);
}

fn string_join(state: &mut LuaState) -> LuaResult<u32> {
    super::utility_system_spell::strjoin(state)
}

fn get_num_subgroup_members(state: &mut LuaState) -> LuaResult<u32> {
    let n = active_party_count(state)? as f64;
    state.push(Val::Num(n));
    Ok(1)
}

fn get_num_group_members(state: &mut LuaState) -> LuaResult<u32> {
    let party_count = active_party_count(state)?;
    // Match mlua master: party_count + 1 (counting the player) when in a
    // group, else 0.
    let n = if party_count > 0 { party_count + 1 } else { 0 };
    state.push(Val::Num(n as f64));
    Ok(1)
}

fn is_in_group(state: &mut LuaState) -> LuaResult<u32> {
    let in_group = active_party_count(state)? > 0;
    state.push(Val::Bool(in_group));
    Ok(1)
}

fn is_in_raid(state: &mut LuaState) -> LuaResult<u32> {
    // Sim currently models party only; treat a party ≥ 6 as a raid.
    let in_raid = active_party_count(state)? >= 6;
    state.push(Val::Bool(in_raid));
    Ok(1)
}

fn is_party_lfg(state: &mut LuaState) -> LuaResult<u32> {
    let lfg = borrow_state(state)?.is_party_lfg;
    state.push(Val::Bool(lfg));
    Ok(1)
}

/// `IsGroupLeader()` — true when the player is in a group AND leads it.
/// `SimState.party_leader_index = None` means the player is the leader.
fn is_group_leader(state: &mut LuaState) -> LuaResult<u32> {
    let in_group = active_party_count(state)? > 0;
    let leader = in_group && borrow_state(state)?.party_leader_index.is_none();
    state.push(Val::Bool(leader));
    Ok(1)
}

fn is_everyone_assistant(state: &mut LuaState) -> LuaResult<u32> {
    let flag = borrow_state(state)?.everyone_assistant;
    state.push(Val::Bool(flag));
    Ok(1)
}

fn active_party_count(state: &mut LuaState) -> LuaResult<usize> {
    let st = borrow_state(state)?;
    Ok(if st.party_group_active {
        st.party_members.len()
    } else {
        0
    })
}

fn unit_exists(state: &mut LuaState) -> LuaResult<u32> {
    let unit = Option::<String>::from_stack(state, 1)?.unwrap_or_default();
    let exists = match unit.as_str() {
        "" => false,
        "player" | "vehicle" => true,
        "pet" => true,
        "target" => borrow_state(state)?.current_target.is_some(),
        "focus" => borrow_state(state)?.current_focus.is_some(),
        other => {
            let st = borrow_state(state)?;
            if let Some(idx) = crate::lua_api::globals::unit_api::parse_party_index(other) {
                st.party_group_active && idx < st.party_members.len()
            } else if let Some(rest) = other.strip_prefix("raid") {
                st.party_group_active
                    && rest
                        .parse::<usize>()
                        .ok()
                        .is_some_and(|n| n > 0 && n - 1 < st.party_members.len())
            } else {
                false
            }
        }
    };
    state.push(Val::Bool(exists));
    Ok(1)
}

fn unit_name(state: &mut LuaState) -> LuaResult<u32> {
    let unit = Option::<String>::from_stack(state, 1)?.unwrap_or_default();
    let name = unit_name_for(state, &unit)?;
    let value = create_string(state, &name);
    state.push(value);
    Ok(1)
}

fn unit_name_unmodified(state: &mut LuaState) -> LuaResult<u32> {
    let unit = Option::<String>::from_stack(state, 1)?.unwrap_or_default();
    let name = unit_name_for(state, &unit)?;
    let value = create_string(state, &name);
    state.push(value);
    Ok(1)
}

fn unit_name_for(state: &mut LuaState, unit: &str) -> LuaResult<String> {
    let name = {
        let st = borrow_state(state)?;
        match unit {
            "player" | "pet" | "vehicle" => st.player.name.clone(),
            "target" => st
                .current_target
                .as_ref()
                .map(|target| target.name.clone())
                .unwrap_or_else(|| "Unknown".to_string()),
            "focus" => st
                .current_focus
                .as_ref()
                .map(|target| target.name.clone())
                .unwrap_or_else(|| "Unknown".to_string()),
            other => visible_party_member(&st, other)
                .map(|member| member.name.clone())
                .unwrap_or_else(|| "Unknown".to_string()),
        }
    };
    Ok(name)
}

fn unit_level(state: &mut LuaState) -> LuaResult<u32> {
    let unit = Option::<String>::from_stack(state, 1)?.unwrap_or_default();
    let level = {
        let st = borrow_state(state)?;
        match unit.as_str() {
            "player" | "pet" | "vehicle" => st.player.level,
            "target" => st
                .current_target
                .as_ref()
                .map(|target| target.level)
                .unwrap_or(1),
            "focus" => st
                .current_focus
                .as_ref()
                .map(|target| target.level)
                .unwrap_or(1),
            other => visible_party_member(&st, other)
                .map(|member| member.level)
                .unwrap_or(1),
        }
    };
    state.push(Val::Num(level as f64));
    Ok(1)
}

fn unit_class(state: &mut LuaState) -> LuaResult<u32> {
    let unit = Option::<String>::from_stack(state, 1)?.unwrap_or_default();
    let class_index = {
        let st = borrow_state(state)?;
        match unit.as_str() {
            "player" | "pet" | "vehicle" => st.player.class_index,
            "target" => st
                .current_target
                .as_ref()
                .map(|target| target.class_index)
                .unwrap_or(2),
            "focus" => st
                .current_focus
                .as_ref()
                .map(|target| target.class_index)
                .unwrap_or(2),
            other => visible_party_member(&st, other)
                .map(|member| member.class_index)
                .unwrap_or(2),
        }
    };
    let (class_name, class_file, class_id) = class_info(class_index);
    let class_name_val = create_string(state, class_name);
    let class_file_val = create_string(state, class_file);
    state.push(class_name_val);
    state.push(class_file_val);
    state.push(Val::Num(class_id as f64));
    Ok(3)
}

fn visible_party_member<'a>(
    st: &'a crate::lua_api::state::SimState,
    unit: &str,
) -> Option<&'a crate::lua_api::state::PartyMember> {
    if !st.party_group_active {
        return None;
    }
    if let Some(idx) = crate::lua_api::globals::unit_api::parse_party_index(unit) {
        if idx < st.party_members.len() {
            st.party_members.get(idx)
        } else {
            None
        }
    } else if let Some(rest) = unit.strip_prefix("raid") {
        rest.parse::<usize>()
            .ok()
            .and_then(|n| n.checked_sub(1))
            .and_then(|idx| st.party_members.get(idx))
    } else {
        None
    }
}

fn unit_classification(state: &mut LuaState) -> LuaResult<u32> {
    let unit = Option::<String>::from_stack(state, 1)?.unwrap_or_default();
    let classification = {
        let st = borrow_state(state)?;
        match unit.as_str() {
            "target" => st
                .current_target
                .as_ref()
                .map(|target| target.classification.clone())
                .unwrap_or_else(|| "normal".to_string()),
            "focus" => st
                .current_focus
                .as_ref()
                .map(|target| target.classification.clone())
                .unwrap_or_else(|| "normal".to_string()),
            _ => "normal".to_string(),
        }
    };
    let value = create_string(state, &classification);
    state.push(value);
    Ok(1)
}

fn unit_creature_type(state: &mut LuaState) -> LuaResult<u32> {
    let unit = Option::<String>::from_stack(state, 1)?.unwrap_or_default();
    let creature_type = {
        let st = borrow_state(state)?;
        match unit.as_str() {
            "target" => st
                .current_target
                .as_ref()
                .map(|target| target.creature_type.clone())
                .unwrap_or_else(|| "Unknown".to_string()),
            "focus" => st
                .current_focus
                .as_ref()
                .map(|target| target.creature_type.clone())
                .unwrap_or_else(|| "Unknown".to_string()),
            _ => "Humanoid".to_string(),
        }
    };
    let value = create_string(state, &creature_type);
    state.push(value);
    Ok(1)
}

fn unit_treat_as_player_for_display(state: &mut LuaState) -> LuaResult<u32> {
    let unit = Option::<String>::from_stack(state, 1)?.unwrap_or_default();
    let treat_as_player = {
        let st = borrow_state(state)?;
        match unit.as_str() {
            "player" | "pet" | "vehicle" => true,
            "target" => st
                .current_target
                .as_ref()
                .map(|target| target.is_player)
                .unwrap_or(false),
            "focus" => st
                .current_focus
                .as_ref()
                .map(|target| target.is_player)
                .unwrap_or(false),
            other => visible_party_member(&st, other).is_some(),
        }
    };
    state.push(Val::Bool(treat_as_player));
    Ok(1)
}

fn unit_is_friend(state: &mut LuaState) -> LuaResult<u32> {
    let unit = Option::<String>::from_stack(state, 2)?.unwrap_or_default();
    let is_friend = is_friendly_unit(state, &unit)?;
    state.push(Val::Bool(is_friend));
    Ok(1)
}

fn unit_can_attack(state: &mut LuaState) -> LuaResult<u32> {
    let unit = Option::<String>::from_stack(state, 2)?.unwrap_or_default();
    let can_attack = is_attackable_unit(state, &unit)?;
    state.push(Val::Bool(can_attack));
    Ok(1)
}

fn unit_can_assist(state: &mut LuaState) -> LuaResult<u32> {
    let unit = Option::<String>::from_stack(state, 2)?.unwrap_or_default();
    let can_assist = is_friendly_unit(state, &unit)?;
    state.push(Val::Bool(can_assist));
    Ok(1)
}

fn unit_selection_color(state: &mut LuaState) -> LuaResult<u32> {
    let unit = Option::<String>::from_stack(state, 1)?.unwrap_or_default();
    let (r, g, b) = if is_attackable_unit(state, &unit)? {
        (1.0, 0.0, 0.0)
    } else if is_friendly_unit(state, &unit)? {
        (0.0, 1.0, 0.0)
    } else {
        (1.0, 1.0, 0.0)
    };
    state.push(Val::Num(r));
    state.push(Val::Num(g));
    state.push(Val::Num(b));
    Ok(3)
}

fn is_friendly_unit(state: &mut LuaState, unit: &str) -> LuaResult<bool> {
    let st = borrow_state(state)?;
    Ok(match unit {
        "player" | "pet" | "vehicle" => true,
        "target" => st
            .current_target
            .as_ref()
            .map(|target| !target.is_enemy || target.reaction >= 4)
            .unwrap_or(false),
        "focus" => st
            .current_focus
            .as_ref()
            .map(|target| !target.is_enemy || target.reaction >= 4)
            .unwrap_or(false),
        other => visible_party_member(&st, other).is_some(),
    })
}

fn is_attackable_unit(state: &mut LuaState, unit: &str) -> LuaResult<bool> {
    let st = borrow_state(state)?;
    Ok(match unit {
        "target" => st
            .current_target
            .as_ref()
            .map(|target| target.is_enemy || target.reaction < 4)
            .unwrap_or(false),
        "focus" => st
            .current_focus
            .as_ref()
            .map(|target| target.is_enemy || target.reaction < 4)
            .unwrap_or(false),
        _ => false,
    })
}

fn class_info(class_index: i32) -> (&'static str, &'static str, i32) {
    const CLASS_FILES: &[&str] = &[
        "WARRIOR",
        "PALADIN",
        "HUNTER",
        "ROGUE",
        "PRIEST",
        "DEATHKNIGHT",
        "SHAMAN",
        "MAGE",
        "WARLOCK",
        "MONK",
        "DRUID",
        "DEMONHUNTER",
        "EVOKER",
    ];

    let idx = class_index
        .max(1)
        .min(CLASS_LABELS.len() as i32)
        .saturating_sub(1) as usize;
    (CLASS_LABELS[idx], CLASS_FILES[idx], class_index.max(1))
}

fn always_false(state: &mut LuaState) -> LuaResult<u32> {
    state.push(Val::Bool(false));
    Ok(1)
}

// ── Unit-token relationship probes ───────────────────────────────────────────

/// `UnitInParty(unit)` — true when the unit is a member of the player's
/// party (including the player).
fn unit_in_party(state: &mut LuaState) -> LuaResult<u32> {
    let unit = Option::<String>::from_stack(state, 1)?.unwrap_or_default();
    let in_party = {
        let st = borrow_state(state)?;
        let active = st.party_group_active;
        match unit.as_str() {
            "player" | "pet" | "vehicle" => active,
            other => visible_party_member(&st, other).is_some(),
        }
    };
    state.push(Val::Bool(in_party));
    Ok(1)
}

/// `UnitInRaid(unit)` — true when the unit belongs to the player's raid
/// group (sim treats party ≥ 6 as raid).
fn unit_in_raid(state: &mut LuaState) -> LuaResult<u32> {
    let unit = Option::<String>::from_stack(state, 1)?.unwrap_or_default();
    let in_raid = {
        let st = borrow_state(state)?;
        if !st.party_group_active || st.party_members.len() < 6 {
            false
        } else {
            matches!(unit.as_str(), "player" | "pet" | "vehicle")
                || visible_party_member(&st, &unit).is_some()
        }
    };
    state.push(Val::Bool(in_raid));
    Ok(1)
}

/// `UnitInOtherParty(unit)` — sim does not model cross-party; always false.
fn unit_in_other_party(state: &mut LuaState) -> LuaResult<u32> {
    let _ = Option::<String>::from_stack(state, 1)?;
    state.push(Val::Bool(false));
    Ok(1)
}

/// `UnitInRange(unit)` — true for `player`, `pet`, current `target` /
/// `focus`, and visible party tokens. Everything else defaults to false.
fn unit_in_range(state: &mut LuaState) -> LuaResult<u32> {
    let unit = Option::<String>::from_stack(state, 1)?.unwrap_or_default();
    let in_range = {
        let st = borrow_state(state)?;
        match unit.as_str() {
            "player" | "pet" | "vehicle" => true,
            "target" => st.current_target.is_some(),
            "focus" => st.current_focus.is_some(),
            other => visible_party_member(&st, other).is_some(),
        }
    };
    state.push(Val::Bool(in_range));
    Ok(1)
}

/// `UnitInBattleground(unit)` — true when the player is currently in an
/// arena / battleground instance. Sim routes this through the world
/// flags set by the battlefield verbs.
fn unit_in_battleground(state: &mut LuaState) -> LuaResult<u32> {
    let _ = Option::<String>::from_stack(state, 1)?;
    let in_bg = {
        let st = borrow_state(state)?;
        st.world.battlefield_arena
            || matches!(st.world.instance_type.as_str(), "arena" | "pvp" | "bg")
    };
    state.push(Val::Bool(in_bg));
    Ok(1)
}

fn player_faction_name(st: &crate::lua_api::state::SimState) -> &'static str {
    RACE_DATA
        .get(st.player.race_index)
        .map(|(_, _, faction)| *faction)
        .filter(|faction| !faction.is_empty())
        .unwrap_or("Alliance")
}

fn opposing_faction_name(faction: &str) -> &'static str {
    match faction {
        "Horde" => "Alliance",
        _ => "Horde",
    }
}

/// `UnitFactionGroup(unit)` — returns `(english, localized)` faction tokens
/// for simple UI gating. Player / party / friendly target inherit the player's
/// faction; hostile targets return the opposing faction.
fn unit_faction_group(state: &mut LuaState) -> LuaResult<u32> {
    let unit = Option::<String>::from_stack(state, 1)?.unwrap_or_default();
    let faction = {
        let st = borrow_state(state)?;
        let player_faction = player_faction_name(&st);
        match unit.as_str() {
            "player" | "pet" | "vehicle" => Some(player_faction),
            "target" => st.current_target.as_ref().map(|target| {
                if target.is_enemy {
                    opposing_faction_name(player_faction)
                } else {
                    player_faction
                }
            }),
            "focus" => st.current_focus.as_ref().map(|target| {
                if target.is_enemy {
                    opposing_faction_name(player_faction)
                } else {
                    player_faction
                }
            }),
            other if visible_party_member(&st, other).is_some() => Some(player_faction),
            _ => None,
        }
    };

    match faction {
        Some(name) => {
            let english = create_string(state, name);
            let localized = create_string(state, name);
            state.push(english);
            state.push(localized);
        }
        None => {
            state.push(Val::Nil);
            state.push(Val::Nil);
        }
    }
    Ok(2)
}

/// `UnitCanCooperate(unit1, unit2)` — true when both units are friendly
/// (simple same-faction proxy for the sim).
fn unit_can_cooperate(state: &mut LuaState) -> LuaResult<u32> {
    let unit1 = Option::<String>::from_stack(state, 1)?.unwrap_or_default();
    let unit2 = Option::<String>::from_stack(state, 2)?.unwrap_or_default();
    let cooperate = is_friendly_unit(state, &unit1)? && is_friendly_unit(state, &unit2)?;
    state.push(Val::Bool(cooperate));
    Ok(1)
}

/// `UnitIsGroupLeader(unit)` — resolves unit to party index, then matches
/// against `SimState.party_leader_index`. Player is leader when the
/// index is `None`.
fn unit_is_group_leader(state: &mut LuaState) -> LuaResult<u32> {
    let unit = Option::<String>::from_stack(state, 1)?.unwrap_or_default();
    let leader = {
        let st = borrow_state(state)?;
        let player_tokens = matches!(unit.as_str(), "player" | "pet" | "vehicle");
        let active = st.party_group_active && !st.party_members.is_empty();
        if !active {
            false
        } else if player_tokens {
            st.party_leader_index.is_none()
        } else if let Some(idx) = resolve_unit_party_index(&st, &unit) {
            st.party_leader_index == Some(idx)
        } else {
            false
        }
    };
    state.push(Val::Bool(leader));
    Ok(1)
}

/// `UnitIsGroupAssistant(unit)` — true only when raid-wide
/// `everyone_assistant` is set and the unit resolves to a party member
/// or the player.
fn unit_is_group_assistant(state: &mut LuaState) -> LuaResult<u32> {
    let unit = Option::<String>::from_stack(state, 1)?.unwrap_or_default();
    let assistant = {
        let st = borrow_state(state)?;
        if !st.everyone_assistant {
            false
        } else {
            matches!(unit.as_str(), "player" | "pet" | "vehicle")
                || visible_party_member(&st, &unit).is_some()
        }
    };
    state.push(Val::Bool(assistant));
    Ok(1)
}

fn resolve_unit_party_index(st: &crate::lua_api::state::SimState, unit: &str) -> Option<usize> {
    if let Some(idx) = crate::lua_api::globals::unit_api::parse_party_index(unit) {
        if idx < st.party_members.len() {
            return Some(idx);
        }
    }
    if let Some(rest) = unit.strip_prefix("raid") {
        if let Some(n) = rest.parse::<usize>().ok().and_then(|n| n.checked_sub(1))
            && n < st.party_members.len()
        {
            return Some(n);
        }
    }
    None
}
