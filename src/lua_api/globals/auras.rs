//! `C_UnitAuras` probe surface backed by `SimState.player.buffs`.
//!
//! Every method walks the live aura list rather than a hard-coded
//! fixture, so admin-added buffs pushed via `admin_buffs::add_buff`
//! are findable by all three keys the Blizzard UI uses:
//!
//! - **index** (`GetAuraDataByIndex`, `GetBuffDataByIndex`,
//!   `GetDebuffDataByIndex`)
//! - **aura instance id** (`GetAuraDataByAuraInstanceID`)
//! - **spell name** (`GetAuraDataBySpellName`)
//!
//! Slot ids round-trip through `aura_instance_id` so the
//! `GetAuraSlots` → `GetAuraDataBySlot` handshake stays consistent
//! with the per-aura data returned by index / name queries.
//!
//! Registered after `stubs::register_all` so the `stub_nil`
//! registrations for the same methods are overwritten.
//!
//! NOTE: `GetAuraSlots` MUST NOT return an empty table as its first
//! value — `AuraUtil.ForEachAura` drives a `repeat ... until
//! token == nil` loop and treats any truthy first return as "more
//! slots available", spinning forever (see
//! docs/wiki/investigations/partyframe-tree.md).

use crate::lua_api::game_data::AuraInfo;
use crate::lua_api::methods::{borrow_state, create_string, create_table, table_set};
use crate::lua_bridge::FromStack;
use rilua::vm::closure::{Closure, RustClosure};
use rilua::vm::state::LuaState;
use rilua::{LuaResult, RustFn, Val};

pub fn register_all(state: &mut LuaState) {
    let ns = ensure_c_unit_auras(state);
    install(state, ns, "GetAuraSlots", get_aura_slots);
    install(state, ns, "GetAuraDataBySlot", get_aura_data_by_slot);
    install(state, ns, "GetAuraDataByIndex", get_aura_data_by_index);
    install(
        state,
        ns,
        "GetAuraDataByAuraInstanceID",
        get_aura_data_by_aura_instance_id,
    );
    install(
        state,
        ns,
        "GetAuraDataBySpellName",
        get_aura_data_by_spell_name,
    );
    install(state, ns, "GetBuffDataByIndex", get_buff_data_by_index);
    install(state, ns, "GetDebuffDataByIndex", get_debuff_data_by_index);
}

fn ensure_c_unit_auras(state: &mut LuaState) -> Val {
    use crate::lua_api::methods::table_get;
    let global = Val::Table(state.global);
    match table_get(state, global, "C_UnitAuras") {
        ns @ Val::Table(_) => ns,
        _ => {
            let ns = create_table(state);
            table_set(state, global, "C_UnitAuras", ns);
            ns
        }
    }
}

fn install(state: &mut LuaState, ns: Val, name: &'static str, func: RustFn) {
    let closure = Closure::Rust(RustClosure::new(func, name));
    let closure_ref = state.gc.alloc_closure(closure);
    table_set(state, ns, name, Val::Function(closure_ref));
}

// ── Filter handling ──────────────────────────────────────────────────────────

#[derive(Clone, Copy)]
enum AuraFilter {
    Helpful,
    Harmful,
}

fn filter_from_str(filter: &str) -> AuraFilter {
    if filter.to_uppercase().contains("HARMFUL") {
        AuraFilter::Harmful
    } else {
        AuraFilter::Helpful
    }
}

fn aura_matches_filter(aura: &AuraInfo, filter: AuraFilter) -> bool {
    match filter {
        AuraFilter::Helpful => aura.is_helpful,
        AuraFilter::Harmful => !aura.is_helpful,
    }
}

fn collect_unit_auras(state: &mut LuaState, unit: &str, filter: AuraFilter) -> Vec<AuraInfo> {
    let Ok(sim) = borrow_state(state) else {
        return Vec::new();
    };
    use crate::lua_api::globals::unit_api::parse_party_index;
    let auras: Vec<&AuraInfo> =
        if let Some(idx) = parse_party_index(unit) {
            if let Some(member) = sim.party_members.get(idx) {
                match filter {
                    AuraFilter::Helpful => member.buffs.iter().collect(),
                    AuraFilter::Harmful => member.debuffs.iter().collect(),
                }
            } else {
                return Vec::new();
            }
        } else {
            sim.player
                .buffs
                .iter()
                .filter(|a| aura_matches_filter(a, filter))
                .collect()
        };
    auras.into_iter().cloned().collect()
}

// ── GetAuraSlots ─────────────────────────────────────────────────────────────
//
// Signature:
//   (continuationToken, slot1, slot2, ...) = GetAuraSlots(unit, filter, batchSize, token)
//
// Slot IDs map 1:1 to `aura_instance_id` so `GetAuraDataBySlot(slot)`
// is equivalent to `GetAuraDataByAuraInstanceID(slot)`.
fn get_aura_slots(state: &mut LuaState) -> LuaResult<u32> {
    let unit: String = Option::<String>::from_stack(state, 1)?.unwrap_or_default();
    let filter_str: String = Option::<String>::from_stack(state, 2)?.unwrap_or_default();
    let token = Option::<f64>::from_stack(state, 4)?;

    // Second call after we already produced our slots — terminate.
    if token.is_some() {
        return Ok(0);
    }

    let auras = collect_unit_auras(state, &unit, filter_from_str(&filter_str));

    // continuationToken = nil (done after this batch)
    state.push(Val::Nil);
    for aura in &auras {
        state.push(Val::Num(aura.aura_instance_id as f64));
    }
    Ok(auras.len() as u32 + 1)
}

// ── GetAuraDataBySlot ────────────────────────────────────────────────────────
fn get_aura_data_by_slot(state: &mut LuaState) -> LuaResult<u32> {
    let unit: String = Option::<String>::from_stack(state, 1)?.unwrap_or_default();
    let slot = Option::<f64>::from_stack(state, 2)?.unwrap_or_default() as i32;
    push_aura_by_instance_id(state, &unit, slot);
    Ok(1)
}

// ── GetAuraDataByIndex ───────────────────────────────────────────────────────
//
// `(unit, index, filter)` — filter decides whether the index addresses a buff
// or a debuff list. 1-based index into the filtered list.
fn get_aura_data_by_index(state: &mut LuaState) -> LuaResult<u32> {
    let unit: String = Option::<String>::from_stack(state, 1)?.unwrap_or_default();
    let index = Option::<f64>::from_stack(state, 2)?.unwrap_or_default() as i32;
    let filter_str = Option::<String>::from_stack(state, 3)?.unwrap_or_default();
    push_aura_at_filtered_index(state, &unit, filter_from_str(&filter_str), index);
    Ok(1)
}

fn get_aura_data_by_aura_instance_id(state: &mut LuaState) -> LuaResult<u32> {
    let unit: String = Option::<String>::from_stack(state, 1)?.unwrap_or_default();
    let aura_id = Option::<f64>::from_stack(state, 2)?.unwrap_or_default() as i32;
    push_aura_by_instance_id(state, &unit, aura_id);
    Ok(1)
}

fn get_aura_data_by_spell_name(state: &mut LuaState) -> LuaResult<u32> {
    let unit: String = Option::<String>::from_stack(state, 1)?.unwrap_or_default();
    let name = Option::<String>::from_stack(state, 2)?.unwrap_or_default();
    use crate::lua_api::globals::unit_api::parse_party_index;
    let found = {
        let Ok(sim) = borrow_state(state) else {
            state.push(Val::Nil);
            return Ok(1);
        };
        if let Some(idx) = parse_party_index(&unit) {
            sim.party_members.get(idx).and_then(|m| {
                m.buffs
                    .iter()
                    .chain(m.debuffs.iter())
                    .find(|a| a.name == name)
                    .cloned()
            })
        } else {
            sim.player.buffs.iter().find(|a| a.name == name).cloned()
        }
    };
    match found {
        Some(aura) => {
            let table = build_aura_table(state, &aura);
            state.push(table);
        }
        None => state.push(Val::Nil),
    }
    Ok(1)
}

fn get_buff_data_by_index(state: &mut LuaState) -> LuaResult<u32> {
    let unit: String = Option::<String>::from_stack(state, 1)?.unwrap_or_default();
    let index = Option::<f64>::from_stack(state, 2)?.unwrap_or_default() as i32;
    push_aura_at_filtered_index(state, &unit, AuraFilter::Helpful, index);
    Ok(1)
}

fn get_debuff_data_by_index(state: &mut LuaState) -> LuaResult<u32> {
    let unit: String = Option::<String>::from_stack(state, 1)?.unwrap_or_default();
    let index = Option::<f64>::from_stack(state, 2)?.unwrap_or_default() as i32;
    push_aura_at_filtered_index(state, &unit, AuraFilter::Harmful, index);
    Ok(1)
}

// ── Aura lookup helpers ──────────────────────────────────────────────────────

fn push_aura_by_instance_id(state: &mut LuaState, unit: &str, aura_instance_id: i32) {
    use crate::lua_api::globals::unit_api::parse_party_index;
    let found = {
        let Ok(sim) = borrow_state(state) else {
            state.push(Val::Nil);
            return;
        };
        if let Some(idx) = parse_party_index(unit) {
            sim.party_members.get(idx).and_then(|m| {
                m.buffs
                    .iter()
                    .chain(m.debuffs.iter())
                    .find(|a| a.aura_instance_id == aura_instance_id)
                    .cloned()
            })
        } else {
            sim.player
                .buffs
                .iter()
                .find(|a| a.aura_instance_id == aura_instance_id)
                .cloned()
        }
    };
    match found {
        Some(aura) => {
            let table = build_aura_table(state, &aura);
            state.push(table);
        }
        None => state.push(Val::Nil),
    }
}

fn push_aura_at_filtered_index(state: &mut LuaState, unit: &str, filter: AuraFilter, index: i32) {
    if index < 1 {
        state.push(Val::Nil);
        return;
    }
    let auras = collect_unit_auras(state, unit, filter);
    match auras.get((index - 1) as usize) {
        Some(aura) => {
            let table = build_aura_table(state, aura);
            state.push(table);
        }
        None => state.push(Val::Nil),
    }
}

// ── Aura table builder ───────────────────────────────────────────────────────

fn build_aura_table(state: &mut LuaState, aura: &AuraInfo) -> Val {
    let t = create_table(state);
    write_aura_identity(state, t, aura);
    write_aura_flags(state, t, aura);
    t
}

fn write_aura_identity(state: &mut LuaState, t: Val, aura: &AuraInfo) {
    let name = create_string(state, &aura.name);
    let source = create_string(state, &aura.source_unit);
    let empty_points = create_table(state);
    let dispel = create_string(state, "");

    table_set(state, t, "name", name);
    table_set(state, t, "icon", Val::Num(aura.icon as f64));
    table_set(state, t, "applications", Val::Num(aura.applications as f64));
    table_set(state, t, "charges", Val::Num(aura.applications as f64));
    table_set(state, t, "stackCount", Val::Num(aura.applications as f64));
    table_set(state, t, "dispelName", dispel);
    table_set(state, t, "duration", Val::Num(aura.duration));
    table_set(state, t, "expirationTime", Val::Num(aura.expiration_time));
    table_set(state, t, "sourceUnit", source);
    table_set(state, t, "spellId", Val::Num(aura.spell_id as f64));
    table_set(state, t, "timeMod", Val::Num(1.0));
    table_set(state, t, "points", empty_points);
    table_set(
        state,
        t,
        "auraInstanceID",
        Val::Num(aura.aura_instance_id as f64),
    );
}

fn write_aura_flags(state: &mut LuaState, t: Val, aura: &AuraInfo) {
    table_set(state, t, "isStealable", Val::Bool(aura.is_stealable));
    table_set(state, t, "nameplateShowPersonal", Val::Bool(false));
    table_set(state, t, "canApplyAura", Val::Bool(aura.can_apply_aura));
    table_set(state, t, "isBossAura", Val::Bool(false));
    table_set(
        state,
        t,
        "isFromPlayerOrPlayerPet",
        Val::Bool(aura.is_from_player_or_player_pet),
    );
    table_set(state, t, "nameplateShowAll", Val::Bool(false));
    table_set(state, t, "isHelpful", Val::Bool(aura.is_helpful));
    table_set(state, t, "isHarmful", Val::Bool(!aura.is_helpful));
    table_set(state, t, "isNameplateOnly", Val::Bool(false));
    table_set(state, t, "isRaid", Val::Bool(aura.is_helpful));
}
