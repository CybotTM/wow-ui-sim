//! Minimal `C_UnitAuras` surface backed by a fixed one-buff-one-debuff
//! fixture so TargetFrame / BuffFrame / party-frame aura code renders
//! something in headless tests.
//!
//! Real WoW clients return aura data per-unit and per-filter. We don't
//! model combat state, so the simulator just returns a single buff slot
//! for HELPFUL filters and a single debuff slot for HARMFUL filters on
//! any unit that `UnitExists` reports as alive. Slot IDs and aura
//! instance IDs are encoded so `GetAuraDataBySlot` /
//! `GetAuraDataByAuraInstanceID` can reconstruct the data without
//! per-unit bookkeeping.
//!
//! Registered after `stubs::register_all` so the `stub_nil`
//! registrations for the same methods are overwritten.
//!
//! NOTE: `GetAuraSlots` MUST NOT return an empty table as its first
//! value — `AuraUtil.ForEachAura` drives a `repeat ... until token == nil`
//! loop and treats any truthy first return as "more slots available",
//! spinning forever (see
//! docs/wiki/investigations/partyframe-tree.md).

use crate::lua_api::methods::{create_string, create_table, table_set};
use crate::lua_bridge::FromStack;
use rilua::vm::closure::{Closure, RustClosure};
use rilua::vm::state::LuaState;
use rilua::{LuaResult, RustFn, Val};

const BUFF_SLOT: f64 = 1.0;
const DEBUFF_SLOT: f64 = 2.0;
const BUFF_AURA_INSTANCE_ID: f64 = 1001.0;
const DEBUFF_AURA_INSTANCE_ID: f64 = 1002.0;

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

// ── GetAuraSlots ─────────────────────────────────────────────────────────────
//
// Signature:
//   (continuationToken, slot1, slot2, ...) = GetAuraSlots(unit, filter, batchSize, token)
//
// AuraUtil.ForEachAura passes the previous token back in; nil means "first
// call". We always emit the full single-slot result on the first call, so
// return a nil continuation to terminate the repeat-until loop.
fn get_aura_slots(state: &mut LuaState) -> LuaResult<u32> {
    let _unit: Option<String> = Option::<String>::from_stack(state, 1)?;
    let filter: String = Option::<String>::from_stack(state, 2)?.unwrap_or_default();
    let token = Option::<f64>::from_stack(state, 4)?;

    // Second call after we already produced our slot — terminate.
    if token.is_some() {
        return Ok(0);
    }

    let filter_upper = filter.to_uppercase();
    let include_helpful = filter_upper.contains("HELPFUL");
    let include_harmful = filter_upper.contains("HARMFUL");

    // continuationToken = nil (done after this batch)
    state.push(Val::Nil);
    let mut count = 1;
    if include_helpful {
        state.push(Val::Num(BUFF_SLOT));
        count += 1;
    }
    if include_harmful {
        state.push(Val::Num(DEBUFF_SLOT));
        count += 1;
    }
    if count == 1 {
        // No filter matched — only the nil continuation token. That's still
        // valid: loop exits immediately with zero slots processed.
        return Ok(1);
    }
    Ok(count)
}

// ── GetAuraDataBySlot ────────────────────────────────────────────────────────
fn get_aura_data_by_slot(state: &mut LuaState) -> LuaResult<u32> {
    let _unit = Option::<String>::from_stack(state, 1)?;
    let slot = Option::<f64>::from_stack(state, 2)?.unwrap_or_default();
    push_aura_for_slot(state, slot);
    Ok(1)
}

// ── GetAuraDataByIndex ───────────────────────────────────────────────────────
//
// (unit, index, filter) — filter decides whether the index addresses a buff
// or a debuff list. Only index 1 is populated.
fn get_aura_data_by_index(state: &mut LuaState) -> LuaResult<u32> {
    let _unit = Option::<String>::from_stack(state, 1)?;
    let index = Option::<f64>::from_stack(state, 2)?.unwrap_or_default();
    let filter = Option::<String>::from_stack(state, 3)?.unwrap_or_default();
    if index != 1.0 {
        state.push(Val::Nil);
        return Ok(1);
    }
    let slot = if filter.to_uppercase().contains("HARMFUL") {
        DEBUFF_SLOT
    } else {
        BUFF_SLOT
    };
    push_aura_for_slot(state, slot);
    Ok(1)
}

fn get_aura_data_by_aura_instance_id(state: &mut LuaState) -> LuaResult<u32> {
    let _unit = Option::<String>::from_stack(state, 1)?;
    let aura_id = Option::<f64>::from_stack(state, 2)?.unwrap_or_default();
    let slot = if aura_id == DEBUFF_AURA_INSTANCE_ID {
        DEBUFF_SLOT
    } else if aura_id == BUFF_AURA_INSTANCE_ID {
        BUFF_SLOT
    } else {
        state.push(Val::Nil);
        return Ok(1);
    };
    push_aura_for_slot(state, slot);
    Ok(1)
}

fn get_buff_data_by_index(state: &mut LuaState) -> LuaResult<u32> {
    let _unit = Option::<String>::from_stack(state, 1)?;
    let index = Option::<f64>::from_stack(state, 2)?.unwrap_or_default();
    if index == 1.0 {
        push_aura_for_slot(state, BUFF_SLOT);
    } else {
        state.push(Val::Nil);
    }
    Ok(1)
}

fn get_debuff_data_by_index(state: &mut LuaState) -> LuaResult<u32> {
    let _unit = Option::<String>::from_stack(state, 1)?;
    let index = Option::<f64>::from_stack(state, 2)?.unwrap_or_default();
    if index == 1.0 {
        push_aura_for_slot(state, DEBUFF_SLOT);
    } else {
        state.push(Val::Nil);
    }
    Ok(1)
}

// ── Aura payload builder ─────────────────────────────────────────────────────
struct AuraFixture {
    name: &'static str,
    icon: f64,
    duration: f64,
    expiration_offset: f64,
    applications: f64,
    spell_id: f64,
    dispel_name: &'static str,
    is_helpful: bool,
    is_harmful: bool,
    aura_instance_id: f64,
}

const BUFF_FIXTURE: AuraFixture = AuraFixture {
    name: "Power Word: Fortitude",
    icon: 135987.0, // INV_Misc_SpellShield_01 file-data-id placeholder
    duration: 3600.0,
    expiration_offset: 3600.0,
    applications: 0.0,
    spell_id: 21562.0,
    dispel_name: "Magic",
    is_helpful: true,
    is_harmful: false,
    aura_instance_id: BUFF_AURA_INSTANCE_ID,
};

const DEBUFF_FIXTURE: AuraFixture = AuraFixture {
    name: "Mortal Strike",
    icon: 132355.0, // ability_warrior_savageblow placeholder
    duration: 10.0,
    expiration_offset: 7.5,
    applications: 1.0,
    spell_id: 12294.0,
    dispel_name: "",
    is_helpful: false,
    is_harmful: true,
    aura_instance_id: DEBUFF_AURA_INSTANCE_ID,
};

fn push_aura_for_slot(state: &mut LuaState, slot: f64) {
    let fixture = if slot == DEBUFF_SLOT {
        &DEBUFF_FIXTURE
    } else {
        &BUFF_FIXTURE
    };
    let aura = build_aura_table(state, fixture);
    state.push(aura);
}

fn build_aura_table(state: &mut LuaState, f: &AuraFixture) -> Val {
    let aura = create_table(state);
    let now = current_time_seconds(state);
    let name = create_string(state, f.name);
    let dispel = create_string(state, f.dispel_name);
    let source = create_string(state, "player");
    let empty_points = create_table(state);

    table_set(state, aura, "name", name);
    table_set(state, aura, "icon", Val::Num(f.icon));
    table_set(state, aura, "applications", Val::Num(f.applications));
    table_set(state, aura, "charges", Val::Num(f.applications));
    table_set(state, aura, "stackCount", Val::Num(f.applications));
    table_set(state, aura, "dispelName", dispel);
    table_set(state, aura, "duration", Val::Num(f.duration));
    table_set(
        state,
        aura,
        "expirationTime",
        Val::Num(now + f.expiration_offset),
    );
    table_set(state, aura, "sourceUnit", source);
    table_set(state, aura, "isStealable", Val::Bool(false));
    table_set(state, aura, "nameplateShowPersonal", Val::Bool(false));
    table_set(state, aura, "spellId", Val::Num(f.spell_id));
    table_set(state, aura, "canApplyAura", Val::Bool(true));
    table_set(state, aura, "isBossAura", Val::Bool(false));
    table_set(state, aura, "isFromPlayerOrPlayerPet", Val::Bool(true));
    table_set(state, aura, "nameplateShowAll", Val::Bool(false));
    table_set(state, aura, "timeMod", Val::Num(1.0));
    table_set(state, aura, "points", empty_points);
    table_set(state, aura, "isHelpful", Val::Bool(f.is_helpful));
    table_set(state, aura, "isHarmful", Val::Bool(f.is_harmful));
    table_set(state, aura, "isNameplateOnly", Val::Bool(false));
    table_set(state, aura, "isRaid", Val::Bool(f.is_helpful));
    table_set(state, aura, "auraInstanceID", Val::Num(f.aura_instance_id));
    aura
}

fn current_time_seconds(state: &mut LuaState) -> f64 {
    // Match GetTime() by reading the simulator's elapsed clock so
    // UpdateDuration calculations (expirationTime - GetTime()) are
    // consistent. Falls back to 0 if state is unreachable.
    crate::lua_api::methods::borrow_state(state)
        .map(|st| st.start_time.elapsed().as_secs_f64())
        .unwrap_or(0.0)
}
