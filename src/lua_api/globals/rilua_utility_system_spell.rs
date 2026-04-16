//! rilua RustFn equivalents of globals from utility_api, system_api, and spell_api.
//!
//! Each `pub fn` matches the `RustFn` signature:
//!   `fn(state: &mut LuaState) -> LuaResult<u32>`
//!
//! Arguments are extracted with `stack_val(state, n)` (1-based).
//! Return values are pushed with `state.push(val)` and counted in the return.
//!
//! Complex operations (pcall, xpcall, securecall) are stubbed with TODO.

use crate::lua_api::LoaderEnv;
use crate::lua_api::globals::unit_api::parse_party_index;
use crate::lua_api::rilua_methods::{
    borrow_lua, borrow_state, borrow_state_mut, call_function_state, create_string, create_table,
    frame_id_from_stack, frame_ref, registry_get, registry_set, state_handle, table_get, table_set,
    val_to_string,
};
use crate::lua_api::rilua_script_helpers::{
    call_error_handler_state, get_event_listeners, get_script, protected_call_state,
    protected_lua_pcall_state,
};
use crate::lua_bridge::{stack_val, table_set_rust_fn};
use crate::specializations;
use rilua::LuaApiMut;
use rilua::vm::state::LuaState;
use rilua::vm::table::Table;
use rilua::{LuaResult, Val, runtime_error};
use std::path::PathBuf;

#[derive(Clone)]
struct UnitVitals {
    health: i32,
    health_max: i32,
    power: i32,
    power_max: i32,
    power_type: i32,
    power_type_name: String,
}

fn set_global_val(state: &mut LuaState, name: &str, value: Val) {
    let key = state.gc.intern_string(name.as_bytes());
    let global = state.global;
    if let Some(g) = state.gc.tables.get_mut(global) {
        let _ = g.raw_set(Val::Str(key), value, &state.gc.string_arena);
    }
}

fn global_val(state: &mut LuaState, name: &str) -> Val {
    let key = state.gc.intern_string(name.as_bytes());
    state
        .gc
        .tables
        .get(state.global)
        .map(|table| table.get_str(key, &state.gc.string_arena))
        .unwrap_or(Val::Nil)
}

fn ensure_global_table(state: &mut LuaState, name: &str) -> Val {
    match global_val(state, name) {
        table @ Val::Table(_) => table,
        _ => {
            let table = create_table(state);
            set_global_val(state, name, table);
            table
        }
    }
}

fn lookup_unit_vitals(state: &LuaState, unit: &str) -> UnitVitals {
    let sim = borrow_state(state).expect("sim state should exist");
    if unit == "target"
        && let Some(target) = &sim.current_target
    {
        return UnitVitals {
            health: target.health,
            health_max: target.health_max,
            power: target.power,
            power_max: target.power_max,
            power_type: target.power_type,
            power_type_name: target.power_type_name.clone(),
        };
    }
    if let Some(index) = parse_party_index(unit)
        && let Some(member) = sim.party_members.get(index)
    {
        return UnitVitals {
            health: member.health,
            health_max: member.health_max,
            power: member.power,
            power_max: member.power_max,
            power_type: member.power_type,
            power_type_name: member.power_type_name.clone(),
        };
    }
    UnitVitals {
        health: sim.player.health,
        health_max: sim.player.health_max,
        power: sim.player.power,
        power_max: sim.player.power_max,
        power_type: sim.player.power_type,
        power_type_name: power_type_name(sim.player.power_type).to_string(),
    }
}

fn requested_power_type(state: &LuaState) -> Option<i64> {
    match stack_val(state, 2) {
        Val::Num(n) => Some(n as i64),
        _ => None,
    }
}

fn is_secondary_power_type(power_type: Option<i64>) -> bool {
    matches!(power_type, Some(power_type) if power_type != 0)
}

fn secondary_power_max(power_type: i64) -> i32 {
    match power_type {
        4 => 7,
        5 => 6,
        9 => 5,
        16 => 4,
        _ => 5,
    }
}

fn power_type_name(power_type: i32) -> &'static str {
    match power_type {
        0 => "MANA",
        1 => "RAGE",
        2 => "FOCUS",
        3 => "ENERGY",
        5 => "RUNES",
        6 => "RUNIC_POWER",
        7 => "SOUL_SHARDS",
        8 => "LUNAR_POWER",
        9 => "HOLY_POWER",
        11 => "MAELSTROM",
        13 => "INSANITY",
        17 => "FURY",
        18 => "PAIN",
        _ => "MANA",
    }
}

// ── Utility API ─────────────────────────────────────────────────────────────

/// wipe(t) — clear all entries from a table and return it.
pub fn wipe(state: &mut LuaState) -> LuaResult<u32> {
    let t = stack_val(state, 1);
    let Val::Table(table_ref) = t else {
        state.push(t);
        return Ok(1);
    };

    let mut keys = Vec::new();
    if let Some(table) = state.gc.tables.get(table_ref) {
        let mut key = Val::Nil;
        while let Some((next_key, _)) = table.next(key, &state.gc.string_arena)? {
            keys.push(next_key);
            key = next_key;
        }
    }

    if let Some(table) = state.gc.tables.get_mut(table_ref) {
        for key in keys {
            let _ = table.raw_set(key, Val::Nil, &state.gc.string_arena);
        }
    }

    state.push(t);
    Ok(1)
}

/// tinsert(t [, pos], value) — append or insert a value into an array table.
///
/// TODO: rilua table mutation API needed.
pub fn tinsert(_state: &mut LuaState) -> LuaResult<u32> {
    // TODO: implement via table array ops
    Ok(0)
}

/// tremove(t [, pos]) — remove and return a value from an array table.
///
/// TODO: rilua table mutation API needed.
pub fn tremove(state: &mut LuaState) -> LuaResult<u32> {
    // TODO: implement via table array ops
    state.push(Val::Nil);
    Ok(1)
}

/// tContains(t, value) — return true if value is present in the array part of t.
///
/// TODO: rilua table iteration API needed.
pub fn t_contains(state: &mut LuaState) -> LuaResult<u32> {
    // TODO: iterate array part and compare
    state.push(Val::Bool(false));
    Ok(1)
}

/// tIndexOf(t, value) — return the integer index of value in t, or nil.
///
/// TODO: rilua table iteration API needed.
pub fn t_index_of(state: &mut LuaState) -> LuaResult<u32> {
    // TODO: iterate array part and compare
    state.push(Val::Nil);
    Ok(1)
}

/// tInvert(t) — return a new table with keys/values swapped.
///
/// `tInvert(tbl)` — return `{[v] = k for k, v in pairs(tbl)}`.
///
/// Mirrors Blizzard_SharedXMLBase/TableUtil.lua. Implemented in Rust
/// because the stub version (which pushed `nil`) shadows the Lua
/// definition — `EnumUtil.MakeEnum(...)` goes through this and every
/// downstream enum (ObjectiveTrackerModuleState, etc.) ends up nil
/// when the stub wins.
pub fn t_invert(state: &mut LuaState) -> LuaResult<u32> {
    let input = stack_val(state, 1);
    let Val::Table(tbl_ref) = input else {
        state.push(Val::Nil);
        return Ok(1);
    };

    let array_values: Vec<Val> = state
        .gc
        .tables
        .get(tbl_ref)
        .map(|t| t.array_slice().to_vec())
        .unwrap_or_default();
    let hash_entries = state
        .gc
        .tables
        .get(tbl_ref)
        .map(|t| t.hash_entries())
        .unwrap_or_default();

    let inverted_ref = state.gc.alloc_table(rilua::vm::table::Table::new());
    for (index, value) in array_values.into_iter().enumerate() {
        if matches!(value, Val::Nil) {
            continue;
        }
        let key = Val::Num((index + 1) as f64);
        if let Some(t) = state.gc.tables.get_mut(inverted_ref) {
            let _ = t.raw_set(value, key, &state.gc.string_arena);
        }
    }
    for (key, value) in hash_entries {
        if matches!(value, Val::Nil) {
            continue;
        }
        if let Some(t) = state.gc.tables.get_mut(inverted_ref) {
            let _ = t.raw_set(value, key, &state.gc.string_arena);
        }
    }
    state.push(Val::Table(inverted_ref));
    Ok(1)
}

/// getglobal(name) — return the global named `name`.
pub fn getglobal(state: &mut LuaState) -> LuaResult<u32> {
    let name_val = stack_val(state, 1);
    let Val::Str(name_ref) = name_val else {
        return Err(runtime_error("getglobal: expected string argument"));
    };
    let name = {
        let lua_str = state
            .gc
            .string_arena
            .get(name_ref)
            .ok_or_else(|| runtime_error("getglobal: invalid string ref"))?;
        String::from_utf8(lua_str.data().to_vec())
            .map_err(|_| runtime_error("getglobal: non-UTF8 name"))?
    };
    let global = state.global;
    let key_ref = state.gc.intern_string(name.as_bytes());
    let val = state
        .gc
        .tables
        .get(global)
        .map(|t| t.get_str(key_ref, &state.gc.string_arena))
        .unwrap_or(Val::Nil);
    state.push(val);
    Ok(1)
}

/// setglobal(name, value) — set the global named `name` to `value`.
pub fn setglobal(state: &mut LuaState) -> LuaResult<u32> {
    let name_val = stack_val(state, 1);
    let value = stack_val(state, 2);
    let Val::Str(name_ref) = name_val else {
        return Err(runtime_error(
            "setglobal: expected string as first argument",
        ));
    };
    let name = {
        let lua_str = state
            .gc
            .string_arena
            .get(name_ref)
            .ok_or_else(|| runtime_error("setglobal: invalid string ref"))?;
        String::from_utf8(lua_str.data().to_vec())
            .map_err(|_| runtime_error("setglobal: non-UTF8 name"))?
    };
    let global = state.global;
    let key_ref = state.gc.intern_string(name.as_bytes());
    if let Some(t) = state.gc.tables.get_mut(global) {
        let _ = t.raw_set(Val::Str(key_ref), value, &state.gc.string_arena);
    }
    Ok(0)
}

/// nop(...) — no-operation, discards all arguments.
pub fn nop(_state: &mut LuaState) -> LuaResult<u32> {
    Ok(0)
}

/// `strsplit(delimiter, str [, limit])` — split `str` on any character in
/// `delimiter` and push each piece as a separate return value.
///
/// Blizzard uses the multi-return shape all over the place:
/// `local major, minor, revision = strsplit(".", "12.0.5")`. The former
/// stub pushed the original string back as a single return, so every
/// downstream variable past the first landed as nil.
pub fn strsplit(state: &mut LuaState) -> LuaResult<u32> {
    let delim = val_to_string_bytes(state, stack_val(state, 1));
    let input = val_to_string_bytes(state, stack_val(state, 2));
    let limit = match stack_val(state, 3) {
        Val::Num(n) if n > 0.0 => Some(n as usize),
        _ => None,
    };
    let (Some(delim), Some(input)) = (delim, input) else {
        state.push(Val::Nil);
        return Ok(1);
    };

    let pieces = split_on_delimiter_set(&input, &delim, limit);
    let count = pieces.len() as u32;
    for piece in pieces {
        let s = state.gc.intern_string(&piece);
        state.push(Val::Str(s));
    }
    Ok(count.max(1))
}

/// Split `input` on any byte present in `delim` (WoW's multi-char
/// delimiter semantics). `limit` caps the resulting piece count:
/// everything after `limit - 1` splits ends up in the last piece.
fn split_on_delimiter_set(input: &[u8], delim: &[u8], limit: Option<usize>) -> Vec<Vec<u8>> {
    let mut pieces = Vec::new();
    let mut current: Vec<u8> = Vec::new();
    let max_pieces = limit.unwrap_or(usize::MAX);
    for &byte in input {
        let is_delim = delim.contains(&byte);
        let can_split = pieces.len() + 1 < max_pieces;
        if is_delim && can_split {
            pieces.push(std::mem::take(&mut current));
        } else {
            current.push(byte);
        }
    }
    pieces.push(current);
    pieces
}

/// `strjoin(delimiter, ...)` — concatenate the variadic string arguments
/// separated by `delimiter`. Previous stub returned the empty string
/// unconditionally, silently dropping everything the caller passed in.
pub fn strjoin(state: &mut LuaState) -> LuaResult<u32> {
    let delim = val_to_string_bytes(state, stack_val(state, 1)).unwrap_or_default();
    let nargs = (state.top as i32 - state.base as i32) as usize;
    let mut out: Vec<u8> = Vec::new();
    for index in 2..=nargs {
        if !out.is_empty() {
            out.extend_from_slice(&delim);
        }
        let slot = state.base + index - 1;
        if let Some(bytes) = val_to_string_bytes(state, state.stack_get(slot)) {
            out.extend_from_slice(&bytes);
        }
    }
    let joined = state.gc.intern_string(&out);
    state.push(Val::Str(joined));
    Ok(1)
}

/// Convert a Lua value to its byte representation for string-like ops.
fn val_to_string_bytes(state: &LuaState, val: Val) -> Option<Vec<u8>> {
    match val {
        Val::Str(s) => state.gc.string_arena.get(s).map(|s| s.data().to_vec()),
        Val::Num(n) => Some(format!("{n}").into_bytes()),
        Val::Bool(b) => Some(if b {
            b"true".to_vec()
        } else {
            b"false".to_vec()
        }),
        _ => None,
    }
}

// ── System API ───────────────────────────────────────────────────────────────

/// type(v) — return the Lua type name of v as a string, reporting frame UserData as "table".
///
/// Note: in rilua, FrameRef is a backed table (Val::Table), so no special case
/// is needed — Val::Table already covers frame-backed tables.
pub fn type_fn(state: &mut LuaState) -> LuaResult<u32> {
    let val = stack_val(state, 1);
    let type_name: &str = match val {
        Val::Nil => "nil",
        Val::Bool(_) => "boolean",
        Val::Num(_) => "number",
        Val::Str(_) => "string",
        Val::Table(_) => "table",
        Val::Function(_) => "function",
        Val::Userdata(_) | Val::LightUserdata(_) | Val::Thread(_) => "userdata",
    };
    let s = state.gc.intern_string(type_name.as_bytes());
    state.push(Val::Str(s));
    Ok(1)
}

/// IsPublicTestClient() — always false in the simulator.
pub fn is_public_test_client(state: &mut LuaState) -> LuaResult<u32> {
    state.push(Val::Bool(false));
    Ok(1)
}

/// IsBetaBuild() — always false in the simulator.
pub fn is_beta_build(state: &mut LuaState) -> LuaResult<u32> {
    state.push(Val::Bool(false));
    Ok(1)
}

/// IsPublicBuild() — always true in the simulator.
pub fn is_public_build(state: &mut LuaState) -> LuaResult<u32> {
    state.push(Val::Bool(true));
    Ok(1)
}

/// BNFeaturesEnabled() — always false (no Battle.net in sim).
pub fn bn_features_enabled(state: &mut LuaState) -> LuaResult<u32> {
    state.push(Val::Bool(false));
    Ok(1)
}

/// BNFeaturesEnabledAndConnected() — always false.
pub fn bn_features_enabled_and_connected(state: &mut LuaState) -> LuaResult<u32> {
    state.push(Val::Bool(false));
    Ok(1)
}

/// BNConnected() — always true (sim pretends connected).
pub fn bn_connected(state: &mut LuaState) -> LuaResult<u32> {
    state.push(Val::Bool(true));
    Ok(1)
}

/// IsGMClient() — always false.
pub fn is_gm_client(state: &mut LuaState) -> LuaResult<u32> {
    state.push(Val::Bool(false));
    Ok(1)
}

/// RegisterStaticConstants(t) — no-op stub.
pub fn register_static_constants(_state: &mut LuaState) -> LuaResult<u32> {
    Ok(0)
}

/// pcall(f, ...) — protected call.
///
/// TODO: rilua does not expose a pcall surface from RustFn context; stub returns false.
pub fn pcall(state: &mut LuaState) -> LuaResult<u32> {
    let func = stack_val(state, 1);
    let args: Vec<_> = ((state.base + 1)..state.top)
        .map(|index| state.stack_get(index))
        .collect();
    match protected_call_state(state, func, &args) {
        Ok(results) => {
            state.push(Val::Bool(true));
            for result in &results {
                state.push(*result);
            }
            Ok(1 + results.len() as u32)
        }
        Err(error) => {
            state.push(Val::Bool(false));
            state.push(error);
            Ok(2)
        }
    }
}

/// xpcall(f, handler, ...) — protected call with error handler.
///
/// TODO: same limitation as pcall.
pub fn xpcall(state: &mut LuaState) -> LuaResult<u32> {
    let func = stack_val(state, 1);
    let handler = stack_val(state, 2);
    let args: Vec<_> = ((state.base + 2)..state.top)
        .map(|index| state.stack_get(index))
        .collect();
    match protected_call_state(state, func, &args) {
        Ok(results) => {
            state.push(Val::Bool(true));
            for result in &results {
                state.push(*result);
            }
            Ok(1 + results.len() as u32)
        }
        Err(error) => {
            let handled = if matches!(handler, Val::Function(_)) {
                protected_call_state(state, handler, &[error])
                    .ok()
                    .and_then(|results| results.into_iter().next())
                    .unwrap_or(error)
            } else {
                error
            };
            state.push(Val::Bool(false));
            state.push(handled);
            Ok(2)
        }
    }
}

/// securecall(name_or_func, ...) — call a function by name in a secure context.
///
/// Taint-aware dispatch is not implemented on the rilua path yet, but SharedXML
/// still depends on `securecall` returning the wrapped function's real results.
pub fn securecall(state: &mut LuaState) -> LuaResult<u32> {
    let func = match stack_val(state, 1) {
        Val::Str(name_ref) => state
            .gc
            .tables
            .get(state.global)
            .map(|table| table.get_str(name_ref, &state.gc.string_arena))
            .unwrap_or(Val::Nil),
        value => value,
    };

    if !matches!(func, Val::Function(_)) {
        state.push(Val::Nil);
        return Ok(1);
    }

    let arg_count = (state.top - state.base).saturating_sub(1);
    let args = (0..arg_count)
        .map(|index| state.stack_get(state.base + 1 + index))
        .collect::<Vec<_>>();

    match protected_lua_pcall_state(state, func, &args) {
        Ok(results) if results.is_empty() => {
            state.push(Val::Nil);
            Ok(1)
        }
        Ok(results) => {
            let count = results.len() as u32;
            for value in results {
                state.push(value);
            }
            Ok(count)
        }
        Err(error) => {
            call_error_handler_state(state, &error);
            state.push(Val::Nil);
            Ok(1)
        }
    }
}

const ERROR_HANDLER_KEY: &str = "__error_handler";
const ADDON_VERSION_CHECK_KEY: &str = "__addon_version_check_enabled";

pub fn seterrorhandler(state: &mut LuaState) -> LuaResult<u32> {
    let handler = stack_val(state, 1);
    if !matches!(handler, Val::Function(_)) {
        return Err(runtime_error("seterrorhandler: expected function"));
    }
    let previous = registry_value(state, ERROR_HANDLER_KEY);
    set_registry_value(state, ERROR_HANDLER_KEY, handler);
    state.push(previous);
    Ok(1)
}

pub fn geterrorhandler(state: &mut LuaState) -> LuaResult<u32> {
    let handler = ensure_error_handler(state)?;
    state.push(handler);
    Ok(1)
}

pub fn table_util_find_indexed_mismatch(state: &mut LuaState) -> LuaResult<u32> {
    let left = stack_val(state, 1);
    let right = stack_val(state, 2);
    let comparator = stack_val(state, 3);
    let (Val::Table(left_ref), Val::Table(right_ref)) = (left, right) else {
        state.push(Val::Nil);
        return Ok(1);
    };

    let left_values = state
        .gc
        .tables
        .get(left_ref)
        .map(|table| table.array_slice().to_vec())
        .unwrap_or_default();
    let right_values = state
        .gc
        .tables
        .get(right_ref)
        .map(|table| table.array_slice().to_vec())
        .unwrap_or_default();
    let count = left_values.len().max(right_values.len());

    for index in 0..count {
        let left_val = left_values.get(index).copied().unwrap_or(Val::Nil);
        let right_val = right_values.get(index).copied().unwrap_or(Val::Nil);
        let equal = if matches!(comparator, Val::Function(_)) {
            call_table_util_comparator(state, comparator, left_val, right_val, index + 1)?
        } else {
            left_val == right_val
        };
        if !equal {
            state.push(Val::Num((index + 1) as f64));
            return Ok(1);
        }
    }

    state.push(Val::Nil);
    Ok(1)
}

// ── Spell API ────────────────────────────────────────────────────────────────

fn unit_health(state: &mut LuaState) -> LuaResult<u32> {
    let unit = val_to_string(state, stack_val(state, 1)).unwrap_or_else(|| "player".to_string());
    let vitals = lookup_unit_vitals(state, &unit);
    state.push(Val::Num(vitals.health as f64));
    Ok(1)
}

fn unit_health_max(state: &mut LuaState) -> LuaResult<u32> {
    let unit = val_to_string(state, stack_val(state, 1)).unwrap_or_else(|| "player".to_string());
    let vitals = lookup_unit_vitals(state, &unit);
    state.push(Val::Num(vitals.health_max as f64));
    Ok(1)
}

fn unit_power(state: &mut LuaState) -> LuaResult<u32> {
    let unit = val_to_string(state, stack_val(state, 1)).unwrap_or_else(|| "player".to_string());
    if is_secondary_power_type(requested_power_type(state)) {
        state.push(Val::Num(0.0));
        return Ok(1);
    }
    let vitals = lookup_unit_vitals(state, &unit);
    state.push(Val::Num(vitals.power as f64));
    Ok(1)
}

fn unit_power_max(state: &mut LuaState) -> LuaResult<u32> {
    let unit = val_to_string(state, stack_val(state, 1)).unwrap_or_else(|| "player".to_string());
    if let Some(power_type) = requested_power_type(state)
        && is_secondary_power_type(Some(power_type))
    {
        state.push(Val::Num(secondary_power_max(power_type) as f64));
        return Ok(1);
    }
    let vitals = lookup_unit_vitals(state, &unit);
    state.push(Val::Num(vitals.power_max as f64));
    Ok(1)
}

fn unit_power_type(state: &mut LuaState) -> LuaResult<u32> {
    let unit = val_to_string(state, stack_val(state, 1)).unwrap_or_else(|| "player".to_string());
    let vitals = lookup_unit_vitals(state, &unit);
    let power_type_name = create_string(state, &vitals.power_type_name);
    state.push(Val::Num(vitals.power_type as f64));
    state.push(power_type_name);
    Ok(2)
}

fn unit_get_incoming_heals(state: &mut LuaState) -> LuaResult<u32> {
    state.push(Val::Num(0.0));
    Ok(1)
}

fn unit_get_total_absorbs(state: &mut LuaState) -> LuaResult<u32> {
    state.push(Val::Num(0.0));
    Ok(1)
}

fn unit_get_total_heal_absorbs(state: &mut LuaState) -> LuaResult<u32> {
    state.push(Val::Num(0.0));
    Ok(1)
}

/// CastSpellByID(spellId [, unit]) — cast a spell by ID.
///
/// TODO: cast_spell_by_id requires Rc<RefCell<SimState>> — use borrow_state_mut helper.
pub fn cast_spell_by_id(_state: &mut LuaState) -> LuaResult<u32> {
    // TODO: extract spell_id from arg 1, call cast logic via borrow_state_mut
    Ok(0)
}

/// CastSpellByName(name [, unit]) — cast a spell by name.
///
/// TODO: same dependency as CastSpellByID.
pub fn cast_spell_by_name(_state: &mut LuaState) -> LuaResult<u32> {
    // TODO: extract name from arg 1, look up spell_id, call cast logic
    Ok(0)
}

fn unit_casting_info(state: &mut LuaState) -> LuaResult<u32> {
    let unit = val_to_string(state, stack_val(state, 1)).unwrap_or_default();
    if unit != "player" {
        return Ok(0);
    }

    let cast = {
        let sim = borrow_state(state)?;
        sim.casting.as_ref().map(|cast| {
            (
                cast.spell_name.clone(),
                cast.icon_path.clone(),
                cast.start_time,
                cast.end_time,
                cast.cast_id,
                cast.spell_id,
            )
        })
    };

    let Some((spell_name, icon_path, start_time, end_time, cast_id, spell_id)) = cast else {
        return Ok(0);
    };

    let spell_name_val = create_string(state, &spell_name);
    let spell_name_display_val = create_string(state, &spell_name);
    let icon_path_val = create_string(state, &icon_path);
    state.push(spell_name_val);
    state.push(spell_name_display_val);
    state.push(icon_path_val);
    state.push(Val::Num(start_time * 1000.0));
    state.push(Val::Num(end_time * 1000.0));
    state.push(Val::Bool(false));
    state.push(Val::Num(cast_id as f64));
    state.push(Val::Bool(false));
    state.push(Val::Num(spell_id as f64));
    Ok(9)
}

fn unit_channel_info(_state: &mut LuaState) -> LuaResult<u32> {
    Ok(0)
}

fn c_spec_get_specialization(state: &mut LuaState) -> LuaResult<u32> {
    let active_spec_index = borrow_state(state)?.player.active_spec_index;
    state.push(Val::Num(active_spec_index as f64));
    Ok(1)
}

fn c_spec_get_specialization_info(state: &mut LuaState) -> LuaResult<u32> {
    let requested_index = match stack_val(state, 1) {
        Val::Num(n) => n as i32,
        _ => 1,
    };

    let (class_id, active_spec_index) = {
        let sim = borrow_state(state)?;
        (sim.player.class_index as u32, sim.player.active_spec_index)
    };

    let fallback = requested_index.max(1);
    let spec = specializations::specs_for_class(class_id)
        .nth((fallback - 1) as usize)
        .or_else(|| {
            let active = active_spec_index.max(1);
            specializations::specs_for_class(class_id).nth((active - 1) as usize)
        });

    let Some(spec) = spec else {
        return Ok(0);
    };

    state.push(Val::Num(spec.id as f64));
    let spec_name = create_string(state, spec.name);
    let spec_description = create_string(state, spec.description);
    let spec_role = create_string(state, spec.role);
    state.push(spec_name);
    state.push(spec_description);
    state.push(Val::Num(spec.icon_file_data_id as f64));
    state.push(spec_role);
    state.push(Val::Num(spec.primary_stat as f64));
    Ok(6)
}

fn c_spec_get_class_id_from_spec_id(state: &mut LuaState) -> LuaResult<u32> {
    let spec_id = match stack_val(state, 1) {
        Val::Num(n) => n as u32,
        _ => 0,
    };
    let class_id = specializations::spec_by_id(spec_id)
        .map(|spec| spec.class_id as f64)
        .unwrap_or(0.0);
    state.push(Val::Num(class_id));
    Ok(1)
}

fn c_spec_get_num_specializations_for_class_id(state: &mut LuaState) -> LuaResult<u32> {
    let class_id = match stack_val(state, 1) {
        Val::Num(n) => n as u32,
        _ => 0,
    };
    let count = specializations::specs_for_class(class_id).count() as f64;
    state.push(Val::Num(count));
    Ok(1)
}

fn c_model_info_get_model_scene_info_by_id(state: &mut LuaState) -> LuaResult<u32> {
    let camera_ids = create_table(state);
    let actor_ids = create_table(state);
    state.push(Val::Num(0.0));
    state.push(camera_ids);
    state.push(actor_ids);
    state.push(Val::Num(0.0));
    Ok(4)
}

fn c_model_info_get_empty_table(state: &mut LuaState) -> LuaResult<u32> {
    let table = create_table(state);
    state.push(table);
    Ok(1)
}

fn player_get_timerunning_season_id(state: &mut LuaState) -> LuaResult<u32> {
    state.push(Val::Num(0.0));
    Ok(1)
}

fn wowtoken_state_table(state: &mut LuaState) -> Val {
    match global_val(state, "__wowtoken_state") {
        table @ Val::Table(_) => table,
        _ => {
            let table = create_table(state);
            table_set(state, table, "tokenCount", Val::Num(2.0));
            table_set(state, table, "currentBalance", Val::Num(2500.0));
            table_set(state, table, "balanceRedeemAmount", Val::Num(1500.0));
            table_set(state, table, "cannotRedeemReason", Val::Num(0.0));
            table_set(state, table, "isSubscribed", Val::Bool(false));
            table_set(state, table, "remainingGameTime", Val::Num(1440.0));
            table_set(state, table, "pendingRedeemType", Val::Nil);
            table_set(state, table, "priceLockDuration", Val::Num(900.0));
            table_set(state, table, "willKickFromWorld", Val::Bool(false));
            set_global_val(state, "__wowtoken_state", table);
            table
        }
    }
}

fn wowtoken_num(state: &mut LuaState, key: &str, default: f64) -> f64 {
    let token_state = wowtoken_state_table(state);
    match table_get(state, token_state, key) {
        Val::Num(value) => value,
        _ => default,
    }
}

fn wowtoken_bool(state: &mut LuaState, key: &str, default: bool) -> bool {
    let token_state = wowtoken_state_table(state);
    match table_get(state, token_state, key) {
        Val::Bool(value) => value,
        _ => default,
    }
}

fn wowtoken_pending_redeem_type(state: &mut LuaState) -> Option<i32> {
    let token_state = wowtoken_state_table(state);
    match table_get(state, token_state, "pendingRedeemType") {
        Val::Num(value) => Some(value as i32),
        _ => None,
    }
}

fn wowtoken_set_num(state: &mut LuaState, key: &str, value: f64) {
    let token_state = wowtoken_state_table(state);
    table_set(state, token_state, key, Val::Num(value));
}

fn wowtoken_set_bool(state: &mut LuaState, key: &str, value: bool) {
    let token_state = wowtoken_state_table(state);
    table_set(state, token_state, key, Val::Bool(value));
}

fn wowtoken_set_pending_redeem_type(state: &mut LuaState, value: Option<i32>) {
    let token_state = wowtoken_state_table(state);
    match value {
        Some(value) => table_set(
            state,
            token_state,
            "pendingRedeemType",
            Val::Num(value as f64),
        ),
        None => table_set(state, token_state, "pendingRedeemType", Val::Nil),
    }
}

fn parse_balance_amount(text: &str) -> Option<i64> {
    let digits_only: String = text.chars().filter(|ch| ch.is_ascii_digit()).collect();
    if digits_only.len() >= 3 {
        return digits_only.parse().ok();
    }
    if digits_only.is_empty() {
        return None;
    }
    digits_only.parse::<i64>().ok().map(|dollars| dollars * 100)
}

fn first_bool_arg(state: &LuaState) -> bool {
    (1..=2)
        .find_map(|index| match stack_val(state, index) {
            Val::Bool(value) => Some(value),
            _ => None,
        })
        .unwrap_or(false)
}

fn first_num_arg(state: &LuaState) -> Option<i32> {
    (1..=2).find_map(|index| match stack_val(state, index) {
        Val::Num(value) => Some(value as i32),
        _ => None,
    })
}

fn first_string_arg(state: &LuaState) -> String {
    (1..=2)
        .find_map(|index| val_to_string(state, stack_val(state, index)))
        .unwrap_or_default()
}

fn fire_named_event(state: &mut LuaState, event_name: &str) {
    for widget_id in get_event_listeners(state, event_name) {
        let Some(handler) = get_script(state, widget_id, "OnEvent") else {
            continue;
        };
        let Ok(frame) = frame_ref(state, widget_id) else {
            continue;
        };
        let event_name_val = create_string(state, event_name);
        let _ = call_function_state(state, handler, &[frame, event_name_val]);
    }
}

fn c_wowtoken_can_redeem_for_balance(state: &mut LuaState) -> LuaResult<u32> {
    fire_named_event(state, "TOKEN_REDEEM_BALANCE_UPDATED");
    let result = if wowtoken_num(state, "tokenCount", 0.0) > 0.0 {
        0.0
    } else {
        1.0
    };
    state.push(Val::Num(result));
    Ok(1)
}

fn c_wowtoken_cancel_redeem(state: &mut LuaState) -> LuaResult<u32> {
    wowtoken_set_pending_redeem_type(state, None);
    state.push(Val::Bool(true));
    Ok(1)
}

fn c_wowtoken_confirm_buy_token(state: &mut LuaState) -> LuaResult<u32> {
    let accepted = first_bool_arg(state);
    if !accepted {
        state.push(Val::Bool(false));
        return Ok(1);
    }
    let token_count = wowtoken_num(state, "tokenCount", 0.0) + 1.0;
    wowtoken_set_num(state, "tokenCount", token_count);
    fire_named_event(state, "TOKEN_STATUS_CHANGED");
    state.push(Val::Bool(true));
    Ok(1)
}

fn c_wowtoken_confirm_sell_token(state: &mut LuaState) -> LuaResult<u32> {
    let accepted = first_bool_arg(state);
    if !accepted {
        state.push(Val::Bool(false));
        return Ok(1);
    }
    let token_count = wowtoken_num(state, "tokenCount", 0.0);
    if token_count > 0.0 {
        wowtoken_set_num(state, "tokenCount", token_count - 1.0);
    }
    fire_named_event(state, "TOKEN_STATUS_CHANGED");
    state.push(Val::Bool(true));
    Ok(1)
}

fn c_wowtoken_get_balance_redeem_amount(state: &mut LuaState) -> LuaResult<u32> {
    let balance_redeem_amount = wowtoken_num(state, "balanceRedeemAmount", 1500.0);
    state.push(Val::Num(balance_redeem_amount));
    Ok(1)
}

fn c_wowtoken_get_balance_redemption_info(state: &mut LuaState) -> LuaResult<u32> {
    let current_balance = wowtoken_num(state, "currentBalance", 2500.0);
    let balance_redeem_amount = wowtoken_num(state, "balanceRedeemAmount", 1500.0);
    let token_count = wowtoken_num(state, "tokenCount", 0.0);
    let cannot_redeem_reason = wowtoken_num(state, "cannotRedeemReason", 0.0);
    state.push(Val::Num(current_balance));
    state.push(Val::Num(balance_redeem_amount));
    state.push(Val::Bool(token_count > 0.0));
    state.push(Val::Num(cannot_redeem_reason));
    Ok(4)
}

fn c_wowtoken_get_game_time_redemption_info(state: &mut LuaState) -> LuaResult<u32> {
    let is_subscribed = wowtoken_bool(state, "isSubscribed", false);
    let remaining_game_time = wowtoken_num(state, "remainingGameTime", 1440.0);
    state.push(Val::Bool(is_subscribed));
    state.push(Val::Num(remaining_game_time));
    Ok(2)
}

fn c_wowtoken_get_price_lock_duration(state: &mut LuaState) -> LuaResult<u32> {
    let price_lock_duration = wowtoken_num(state, "priceLockDuration", 900.0);
    state.push(Val::Num(price_lock_duration));
    Ok(1)
}

fn c_wowtoken_get_remaining_game_time(state: &mut LuaState) -> LuaResult<u32> {
    fire_named_event(state, "TOKEN_REDEEM_GAME_TIME_UPDATED");
    let remaining_game_time = wowtoken_num(state, "remainingGameTime", 1440.0);
    state.push(Val::Num(remaining_game_time));
    Ok(1)
}

fn c_wowtoken_get_token_count(state: &mut LuaState) -> LuaResult<u32> {
    let token_count = wowtoken_num(state, "tokenCount", 2.0);
    state.push(Val::Num(token_count));
    Ok(1)
}

fn c_wowtoken_is_redemption_still_valid(state: &mut LuaState) -> LuaResult<u32> {
    let pending_redeem_type = wowtoken_pending_redeem_type(state);
    let token_count = wowtoken_num(state, "tokenCount", 0.0);
    state.push(Val::Bool(
        pending_redeem_type.is_some() && token_count > 0.0,
    ));
    Ok(1)
}

fn c_wowtoken_redeem_token(state: &mut LuaState) -> LuaResult<u32> {
    let redeem_type = first_num_arg(state).unwrap_or(0);
    if wowtoken_num(state, "tokenCount", 0.0) <= 0.0 {
        state.push(Val::Bool(false));
        return Ok(1);
    }
    wowtoken_set_pending_redeem_type(state, Some(redeem_type));
    state.push(Val::Bool(true));
    Ok(1)
}

fn confirm_game_time_redemption(state: &mut LuaState) {
    wowtoken_set_bool(state, "isSubscribed", true);
    let remaining_game_time = wowtoken_num(state, "remainingGameTime", 1440.0);
    wowtoken_set_num(
        state,
        "remainingGameTime",
        remaining_game_time + 30.0 * 24.0 * 60.0,
    );
    fire_named_event(state, "TOKEN_STATUS_CHANGED");
    fire_named_event(state, "TOKEN_REDEEM_GAME_TIME_UPDATED");
    state.push(Val::Bool(true));
}

fn confirm_balance_redemption(state: &mut LuaState) {
    let current_balance = wowtoken_num(state, "currentBalance", 2500.0);
    let balance_redeem_amount = wowtoken_num(state, "balanceRedeemAmount", 1500.0);
    wowtoken_set_num(
        state,
        "currentBalance",
        current_balance + balance_redeem_amount,
    );
    fire_named_event(state, "TOKEN_STATUS_CHANGED");
    fire_named_event(state, "TOKEN_REDEEM_BALANCE_UPDATED");
    state.push(Val::Bool(true));
}

fn c_wowtoken_redeem_token_confirm(state: &mut LuaState) -> LuaResult<u32> {
    let redeem_type = first_num_arg(state).unwrap_or(0);
    if wowtoken_pending_redeem_type(state) != Some(redeem_type)
        || wowtoken_num(state, "tokenCount", 0.0) <= 0.0
    {
        state.push(Val::Bool(false));
        return Ok(1);
    }

    wowtoken_set_pending_redeem_type(state, None);
    let token_count = wowtoken_num(state, "tokenCount", 0.0);
    wowtoken_set_num(state, "tokenCount", token_count - 1.0);

    match redeem_type {
        1 => confirm_game_time_redemption(state),
        2 => confirm_balance_redemption(state),
        _ => state.push(Val::Bool(false)),
    }
    Ok(1)
}

fn c_wowtoken_set_balance_amount_string(state: &mut LuaState) -> LuaResult<u32> {
    let value = first_string_arg(state);
    if let Some(parsed_amount) = parse_balance_amount(&value) {
        wowtoken_set_num(state, "balanceRedeemAmount", parsed_amount as f64);
    }
    Ok(0)
}

fn c_wowtoken_will_kick_from_world(state: &mut LuaState) -> LuaResult<u32> {
    let will_kick_from_world = wowtoken_bool(state, "willKickFromWorld", false);
    state.push(Val::Bool(will_kick_from_world));
    Ok(1)
}

const WOWTOKEN_SECURE_FUNCTIONS: &[(&str, rilua::vm::closure::RustFn)] = &[
    ("CanRedeemForBalance", c_wowtoken_can_redeem_for_balance),
    ("CancelRedeem", c_wowtoken_cancel_redeem),
    ("ConfirmBuyToken", c_wowtoken_confirm_buy_token),
    ("ConfirmSellToken", c_wowtoken_confirm_sell_token),
    (
        "GetBalanceRedeemAmount",
        c_wowtoken_get_balance_redeem_amount,
    ),
    (
        "GetBalanceRedemptionInfo",
        c_wowtoken_get_balance_redemption_info,
    ),
    (
        "GetGameTimeRedemptionInfo",
        c_wowtoken_get_game_time_redemption_info,
    ),
    ("GetPriceLockDuration", c_wowtoken_get_price_lock_duration),
    ("GetRemainingGameTime", c_wowtoken_get_remaining_game_time),
    ("GetTokenCount", c_wowtoken_get_token_count),
    (
        "IsRedemptionStillValid",
        c_wowtoken_is_redemption_still_valid,
    ),
    ("RedeemToken", c_wowtoken_redeem_token),
    ("RedeemTokenConfirm", c_wowtoken_redeem_token_confirm),
    (
        "SetBalanceAmountString",
        c_wowtoken_set_balance_amount_string,
    ),
    ("WillKickFromWorld", c_wowtoken_will_kick_from_world),
];

fn register_rust_fns(
    state: &mut LuaState,
    table_ref: rilua::vm::gc::arena::GcRef<Table>,
    functions: &[(&str, rilua::vm::closure::RustFn)],
) -> LuaResult<()> {
    for (name, func) in functions {
        table_set_rust_fn(state, table_ref, name, *func)?;
    }
    Ok(())
}

// ── Registration ─────────────────────────────────────────────────────────────

/// Register all functions in this module as rilua globals.
///
/// Call this once after the rilua `Lua` instance is created.
pub fn register_all(lua: &mut rilua::Lua) -> rilua::LuaResult<()> {
    use rilua::LuaApiMut;

    // Utility: table functions
    LuaApiMut::register_function(lua, "wipe", wipe)?;
    LuaApiMut::register_function(lua, "tinsert", tinsert)?;
    LuaApiMut::register_function(lua, "tremove", tremove)?;
    LuaApiMut::register_function(lua, "tContains", t_contains)?;
    LuaApiMut::register_function(lua, "tIndexOf", t_index_of)?;
    LuaApiMut::register_function(lua, "tInvert", t_invert)?;

    // Utility: global access
    LuaApiMut::register_function(lua, "getglobal", getglobal)?;
    LuaApiMut::register_function(lua, "setglobal", setglobal)?;

    // Utility: misc
    LuaApiMut::register_function(lua, "nop", nop)?;

    // Utility: string functions
    LuaApiMut::register_function(lua, "strsplit", strsplit)?;
    LuaApiMut::register_function(lua, "strjoin", strjoin)?;

    // System: type override
    LuaApiMut::register_function(lua, "type", type_fn)?;

    // System: build type checks
    LuaApiMut::register_function(lua, "IsPublicTestClient", is_public_test_client)?;
    LuaApiMut::register_function(lua, "IsBetaBuild", is_beta_build)?;
    LuaApiMut::register_function(lua, "IsPublicBuild", is_public_build)?;

    // System: Battle.net stubs
    LuaApiMut::register_function(lua, "BNFeaturesEnabled", bn_features_enabled)?;
    LuaApiMut::register_function(
        lua,
        "BNFeaturesEnabledAndConnected",
        bn_features_enabled_and_connected,
    )?;
    LuaApiMut::register_function(lua, "BNConnected", bn_connected)?;

    // System: secure stubs
    LuaApiMut::register_function(lua, "IsGMClient", is_gm_client)?;
    LuaApiMut::register_function(lua, "RegisterStaticConstants", register_static_constants)?;

    // System: protected calls (stubbed)
    LuaApiMut::register_function(lua, "pcall", pcall)?;
    LuaApiMut::register_function(lua, "xpcall", xpcall)?;
    LuaApiMut::register_function(lua, "securecall", securecall)?;
    LuaApiMut::register_function(lua, "seterrorhandler", seterrorhandler)?;
    LuaApiMut::register_function(lua, "geterrorhandler", geterrorhandler)?;

    // Unit health/power
    LuaApiMut::register_function(lua, "UnitHealth", unit_health)?;
    LuaApiMut::register_function(lua, "UnitHealthMax", unit_health_max)?;
    LuaApiMut::register_function(lua, "UnitPower", unit_power)?;
    LuaApiMut::register_function(lua, "UnitPowerMax", unit_power_max)?;
    LuaApiMut::register_function(lua, "UnitPowerType", unit_power_type)?;
    LuaApiMut::register_function(lua, "UnitGetIncomingHeals", unit_get_incoming_heals)?;
    LuaApiMut::register_function(lua, "UnitGetTotalAbsorbs", unit_get_total_absorbs)?;
    LuaApiMut::register_function(lua, "UnitGetTotalHealAbsorbs", unit_get_total_heal_absorbs)?;

    // Spell: cast globals (stubbed)
    LuaApiMut::register_function(lua, "CastSpellByID", cast_spell_by_id)?;
    LuaApiMut::register_function(lua, "CastSpellByName", cast_spell_by_name)?;
    LuaApiMut::register_function(lua, "UnitCastingInfo", unit_casting_info)?;
    LuaApiMut::register_function(lua, "UnitChannelInfo", unit_channel_info)?;
    LuaApiMut::register_function(
        lua,
        "PlayerGetTimerunningSeasonID",
        player_get_timerunning_season_id,
    )?;
    register_table_util(lua.state_mut())?;
    register_c_addons(lua.state_mut())?;
    register_c_addon_profiler(lua.state_mut())?;
    register_c_specialization_info(lua.state_mut())?;
    register_c_model_info(lua.state_mut())?;
    register_c_lfg_info(lua.state_mut())?;
    register_c_wowtoken_secure(lua.state_mut())?;
    register_c_texture(lua.state_mut())?;
    register_c_xml_util(lua.state_mut())?;
    register_legacy_addon_globals(lua.state_mut())?;
    register_widget_container_mixin(lua.state_mut())?;

    Ok(())
}

fn register_table_util(state: &mut LuaState) -> LuaResult<()> {
    let table_ref = state.gc.alloc_table(Table::new());
    table_set_rust_fn(
        state,
        table_ref,
        "FindIndexedMismatch",
        table_util_find_indexed_mismatch,
    )?;
    let key_ref = state.gc.intern_string(b"C_TableUtil");
    if let Some(global) = state.gc.tables.get_mut(state.global) {
        let _ = global.raw_set(
            Val::Str(key_ref),
            Val::Table(table_ref),
            &state.gc.string_arena,
        );
    }
    Ok(())
}

fn register_c_specialization_info(state: &mut LuaState) -> LuaResult<()> {
    let t = create_table(state);
    let Val::Table(t_ref) = t else {
        unreachable!("create_table must return a table");
    };
    table_set_rust_fn(state, t_ref, "GetSpecialization", c_spec_get_specialization)?;
    table_set_rust_fn(
        state,
        t_ref,
        "GetSpecializationInfo",
        c_spec_get_specialization_info,
    )?;
    table_set_rust_fn(
        state,
        t_ref,
        "GetClassIDFromSpecID",
        c_spec_get_class_id_from_spec_id,
    )?;
    table_set_rust_fn(
        state,
        t_ref,
        "GetNumSpecializationsForClassID",
        c_spec_get_num_specializations_for_class_id,
    )?;
    set_global_val(state, "C_SpecializationInfo", t);
    Ok(())
}

fn register_c_model_info(state: &mut LuaState) -> LuaResult<()> {
    let t = create_table(state);
    let Val::Table(t_ref) = t else {
        unreachable!("create_table must return a table");
    };
    table_set_rust_fn(state, t_ref, "AddActiveModelScene", |_state| Ok(0))?;
    table_set_rust_fn(state, t_ref, "AddActiveModelSceneActor", |_state| Ok(0))?;
    table_set_rust_fn(state, t_ref, "ClearActiveModelScene", |_state| Ok(0))?;
    table_set_rust_fn(state, t_ref, "ClearActiveModelSceneActor", |_state| Ok(0))?;
    table_set_rust_fn(
        state,
        t_ref,
        "GetModelSceneActorDisplayInfoByID",
        c_model_info_get_empty_table,
    )?;
    table_set_rust_fn(
        state,
        t_ref,
        "GetModelSceneActorInfoByID",
        c_model_info_get_empty_table,
    )?;
    table_set_rust_fn(
        state,
        t_ref,
        "GetModelSceneCameraInfoByID",
        c_model_info_get_empty_table,
    )?;
    table_set_rust_fn(
        state,
        t_ref,
        "GetModelSceneInfoByID",
        c_model_info_get_model_scene_info_by_id,
    )?;
    set_global_val(state, "C_ModelInfo", t);
    Ok(())
}

fn register_c_lfg_info(state: &mut LuaState) -> LuaResult<()> {
    let t = ensure_global_table(state, "C_LFGInfo");
    let Val::Table(t_ref) = t else {
        unreachable!("C_LFGInfo must be a table");
    };
    table_set_rust_fn(state, t_ref, "GetDungeonInfo", c_model_info_get_empty_table)?;
    table_set_rust_fn(
        state,
        t_ref,
        "GetLFDLockStates",
        c_model_info_get_empty_table,
    )?;
    table_set_rust_fn(
        state,
        t_ref,
        "GetAllEntriesForCategory",
        c_model_info_get_empty_table,
    )?;
    table_set_rust_fn(state, t_ref, "CanPlayerUseLFD", c_lfg_info_can_player_use)?;
    table_set_rust_fn(state, t_ref, "CanPlayerUseLFR", c_lfg_info_can_player_use)?;
    Ok(())
}

fn c_lfg_info_can_player_use(state: &mut LuaState) -> LuaResult<u32> {
    state.push(Val::Bool(true));
    state.push(Val::Nil);
    Ok(2)
}

fn register_c_wowtoken_secure(state: &mut LuaState) -> LuaResult<()> {
    wowtoken_state_table(state);
    let t = ensure_global_table(state, "C_WowTokenSecure");
    let Val::Table(t_ref) = t else {
        unreachable!("C_WowTokenSecure must be a table");
    };
    register_rust_fns(state, t_ref, WOWTOKEN_SECURE_FUNCTIONS)?;
    Ok(())
}

fn register_c_addons(state: &mut LuaState) -> LuaResult<()> {
    let c_addons = create_table(state);
    let Val::Table(c_addons_ref) = c_addons else {
        unreachable!("create_table must return a table");
    };
    table_set_rust_fn(state, c_addons_ref, "GetNumAddOns", c_addons_get_num_addons)?;
    table_set_rust_fn(state, c_addons_ref, "GetAddOnInfo", c_addons_get_addon_info)?;
    table_set_rust_fn(
        state,
        c_addons_ref,
        "IsAddOnLoaded",
        c_addons_is_addon_loaded,
    )?;
    table_set_rust_fn(
        state,
        c_addons_ref,
        "IsAddOnLoadOnDemand",
        c_addons_is_addon_load_on_demand,
    )?;
    table_set_rust_fn(
        state,
        c_addons_ref,
        "GetAddOnEnableState",
        c_addons_get_addon_enable_state,
    )?;
    table_set_rust_fn(state, c_addons_ref, "EnableAddOn", c_addons_enable_addon)?;
    table_set_rust_fn(state, c_addons_ref, "DisableAddOn", c_addons_disable_addon)?;
    table_set_rust_fn(
        state,
        c_addons_ref,
        "EnableAllAddOns",
        c_addons_enable_all_addons,
    )?;
    table_set_rust_fn(
        state,
        c_addons_ref,
        "DisableAllAddOns",
        c_addons_disable_all_addons,
    )?;
    table_set_rust_fn(
        state,
        c_addons_ref,
        "GetAddOnMetadata",
        c_addons_get_addon_metadata,
    )?;
    table_set_rust_fn(
        state,
        c_addons_ref,
        "DoesAddOnExist",
        c_addons_does_addon_exist,
    )?;
    table_set_rust_fn(state, c_addons_ref, "GetAddOnName", c_addons_get_addon_name)?;
    table_set_rust_fn(
        state,
        c_addons_ref,
        "GetAddOnTitle",
        c_addons_get_addon_title,
    )?;
    table_set_rust_fn(
        state,
        c_addons_ref,
        "GetAddOnNotes",
        c_addons_get_addon_notes,
    )?;
    table_set_rust_fn(
        state,
        c_addons_ref,
        "GetAddOnSecurity",
        c_addons_get_addon_security,
    )?;
    table_set_rust_fn(
        state,
        c_addons_ref,
        "IsAddonVersionCheckEnabled",
        c_addons_is_addon_version_check_enabled,
    )?;
    table_set_rust_fn(
        state,
        c_addons_ref,
        "SetAddonVersionCheck",
        c_addons_set_addon_version_check,
    )?;
    table_set_rust_fn(state, c_addons_ref, "LoadAddOn", c_addons_load_addon)?;

    let key_ref = state.gc.intern_string(b"C_AddOns");
    if let Some(global) = state.gc.tables.get_mut(state.global) {
        let _ = global.raw_set(
            Val::Str(key_ref),
            Val::Table(c_addons_ref),
            &state.gc.string_arena,
        );
    }
    Ok(())
}

fn register_legacy_addon_globals(state: &mut LuaState) -> LuaResult<()> {
    table_set_rust_fn(state, state.global, "GetNumAddOns", c_addons_get_num_addons)?;
    table_set_rust_fn(
        state,
        state.global,
        "IsAddOnLoaded",
        c_addons_is_addon_loaded,
    )?;
    table_set_rust_fn(
        state,
        state.global,
        "GetAddOnMetadata",
        c_addons_get_addon_metadata,
    )?;
    table_set_rust_fn(
        state,
        state.global,
        "GetAddOnEnableState",
        legacy_get_addon_enable_state,
    )?;
    table_set_rust_fn(
        state,
        state.global,
        "IsAddOnLoadOnDemand",
        c_addons_is_addon_load_on_demand,
    )?;
    table_set_rust_fn(state, state.global, "LoadAddOn", c_addons_load_addon)?;
    let blocked = create_table(state);
    let key_ref = state.gc.intern_string(b"ADDON_ACTIONS_BLOCKED");
    if let Some(global) = state.gc.tables.get_mut(state.global) {
        let _ = global.raw_set(Val::Str(key_ref), blocked, &state.gc.string_arena);
    }
    Ok(())
}

fn register_c_addon_profiler(state: &mut LuaState) -> LuaResult<()> {
    let profiler = create_table(state);
    let Val::Table(profiler_ref) = profiler else {
        unreachable!("create_table must return a table");
    };
    table_set_rust_fn(
        state,
        profiler_ref,
        "GetApplicationMetric",
        c_addon_profiler_get_application_metric,
    )?;
    table_set_rust_fn(
        state,
        profiler_ref,
        "GetOverallMetric",
        c_addon_profiler_get_overall_metric,
    )?;
    table_set_rust_fn(
        state,
        profiler_ref,
        "GetAddOnMetric",
        c_addon_profiler_get_addon_metric,
    )?;
    table_set_rust_fn(
        state,
        profiler_ref,
        "CheckForPerformanceMessage",
        c_addon_profiler_check_for_performance_message,
    )?;
    let key_ref = state.gc.intern_string(b"C_AddOnProfiler");
    if let Some(global) = state.gc.tables.get_mut(state.global) {
        let _ = global.raw_set(Val::Str(key_ref), profiler, &state.gc.string_arena);
    }
    Ok(())
}

fn register_c_texture(state: &mut LuaState) -> LuaResult<()> {
    let c_texture = create_table(state);
    let Val::Table(c_texture_ref) = c_texture else {
        unreachable!("create_table must return a table");
    };
    table_set_rust_fn(
        state,
        c_texture_ref,
        "GetAtlasInfo",
        c_texture_get_atlas_info,
    )?;
    table_set_rust_fn(
        state,
        c_texture_ref,
        "GetAtlasExists",
        c_texture_get_atlas_exists,
    )?;
    let key_ref = state.gc.intern_string(b"C_Texture");
    if let Some(global) = state.gc.tables.get_mut(state.global) {
        let _ = global.raw_set(
            Val::Str(key_ref),
            Val::Table(c_texture_ref),
            &state.gc.string_arena,
        );
    }
    Ok(())
}

fn register_c_xml_util(state: &mut LuaState) -> LuaResult<()> {
    let c_xml_util = create_table(state);
    let Val::Table(c_xml_util_ref) = c_xml_util else {
        unreachable!("create_table must return a table");
    };
    table_set_rust_fn(
        state,
        c_xml_util_ref,
        "GetTemplateInfo",
        c_xml_util_get_template_info,
    )?;
    let key_ref = state.gc.intern_string(b"C_XMLUtil");
    if let Some(global) = state.gc.tables.get_mut(state.global) {
        let _ = global.raw_set(
            Val::Str(key_ref),
            Val::Table(c_xml_util_ref),
            &state.gc.string_arena,
        );
    }
    Ok(())
}

fn register_widget_container_mixin(state: &mut LuaState) -> LuaResult<()> {
    let mixin = create_table(state);
    let Val::Table(mixin_ref) = mixin else {
        unreachable!("create_table must return a table");
    };
    table_set_rust_fn(
        state,
        mixin_ref,
        "GetNumWidgetsShowing",
        ui_widget_container_get_num_widgets_showing,
    )?;
    let key_ref = state.gc.intern_string(b"UIWidgetContainerMixin");
    if let Some(global) = state.gc.tables.get_mut(state.global) {
        let _ = global.raw_set(Val::Str(key_ref), mixin, &state.gc.string_arena);
    }
    Ok(())
}

fn c_texture_get_atlas_exists(state: &mut LuaState) -> LuaResult<u32> {
    let atlas_name = val_to_string(state, stack_val(state, 1));
    state.push(Val::Bool(
        atlas_name
            .as_deref()
            .and_then(crate::atlas::get_atlas_info)
            .is_some(),
    ));
    Ok(1)
}

fn c_xml_util_get_template_info(state: &mut LuaState) -> LuaResult<u32> {
    let Some(template_name) = val_to_string(state, stack_val(state, 1)) else {
        state.push(Val::Nil);
        return Ok(1);
    };
    let Some(info) = crate::xml::get_template_info(&template_name) else {
        state.push(Val::Nil);
        return Ok(1);
    };

    let info_table = create_table(state);
    let Val::Table(info_ref) = info_table else {
        unreachable!("create_table must return a table");
    };
    let key_values = create_table(state);
    let Val::Table(key_values_ref) = key_values else {
        unreachable!("create_table must return a table");
    };

    let set_str = |state: &mut LuaState, table_ref, key: &str, value: &str| {
        let key_ref = state.gc.intern_string(key.as_bytes());
        let value = create_string(state, value);
        if let Some(table) = state.gc.tables.get_mut(table_ref) {
            let _ = table.raw_set(Val::Str(key_ref), value, &state.gc.string_arena);
        }
    };
    let set_num = |state: &mut LuaState, table_ref, key: &str, value: f64| {
        let key_ref = state.gc.intern_string(key.as_bytes());
        if let Some(table) = state.gc.tables.get_mut(table_ref) {
            let _ = table.raw_set(Val::Str(key_ref), Val::Num(value), &state.gc.string_arena);
        }
    };

    for (index, key_value) in info.key_values.iter().enumerate() {
        let key_value_table = create_table(state);
        let Val::Table(key_value_ref) = key_value_table else {
            unreachable!("create_table must return a table");
        };
        set_str(state, key_value_ref, "key", &key_value.key);
        set_str(state, key_value_ref, "value", &key_value.value);
        if let Some(value_type) = &key_value.value_type {
            set_str(state, key_value_ref, "type", value_type);
        }
        if let Some(table) = state.gc.tables.get_mut(key_values_ref) {
            let _ = table.raw_set(
                Val::Num((index + 1) as f64),
                Val::Table(key_value_ref),
                &state.gc.string_arena,
            );
        }
    }

    set_str(state, info_ref, "type", &info.frame_type);
    set_str(state, info_ref, "frameType", &info.frame_type);
    set_str(state, info_ref, "frameTemplate", &info.template_name);
    set_str(state, info_ref, "template", &info.template_name);
    set_num(state, info_ref, "width", info.width as f64);
    set_num(state, info_ref, "height", info.height as f64);
    let key_ref = state.gc.intern_string(b"keyValues");
    if let Some(table) = state.gc.tables.get_mut(info_ref) {
        let _ = table.raw_set(
            Val::Str(key_ref),
            Val::Table(key_values_ref),
            &state.gc.string_arena,
        );
    }

    state.push(info_table);
    Ok(1)
}

fn c_texture_get_atlas_info(state: &mut LuaState) -> LuaResult<u32> {
    let Some(atlas_name) = val_to_string(state, stack_val(state, 1)) else {
        state.push(Val::Nil);
        return Ok(1);
    };
    let Some(lookup) = crate::atlas::get_atlas_info(&atlas_name) else {
        state.push(Val::Nil);
        return Ok(1);
    };

    let info = create_table(state);
    let Val::Table(info_ref) = info else {
        unreachable!("create_table must return a table");
    };
    let raw_size = create_table(state);
    let Val::Table(raw_size_ref) = raw_size else {
        unreachable!("create_table must return a table");
    };

    let set_str = |state: &mut LuaState, key: &str, value: &str| {
        let key = state.gc.intern_string(key.as_bytes());
        let value = create_string(state, value);
        if let Some(table) = state.gc.tables.get_mut(info_ref) {
            let _ = table.raw_set(Val::Str(key), value, &state.gc.string_arena);
        }
    };
    let set_num = |state: &mut LuaState, key: &str, value: f64| {
        let key = state.gc.intern_string(key.as_bytes());
        if let Some(table) = state.gc.tables.get_mut(info_ref) {
            let _ = table.raw_set(Val::Str(key), Val::Num(value), &state.gc.string_arena);
        }
    };
    let set_bool = |state: &mut LuaState, key: &str, value: bool| {
        let key = state.gc.intern_string(key.as_bytes());
        if let Some(table) = state.gc.tables.get_mut(info_ref) {
            let _ = table.raw_set(Val::Str(key), Val::Bool(value), &state.gc.string_arena);
        }
    };

    if let Some(table) = state.gc.tables.get_mut(raw_size_ref) {
        let _ = table.raw_set(
            Val::Num(1.0),
            Val::Num(lookup.width() as f64),
            &state.gc.string_arena,
        );
        let _ = table.raw_set(
            Val::Num(2.0),
            Val::Num(lookup.height() as f64),
            &state.gc.string_arena,
        );
    }

    set_str(state, "elementName", &atlas_name);
    set_num(state, "width", lookup.width() as f64);
    set_num(state, "height", lookup.height() as f64);
    set_num(state, "leftTexCoord", lookup.info.left_tex_coord as f64);
    set_num(state, "rightTexCoord", lookup.info.right_tex_coord as f64);
    set_num(state, "topTexCoord", lookup.info.top_tex_coord as f64);
    set_num(state, "bottomTexCoord", lookup.info.bottom_tex_coord as f64);
    set_bool(state, "tilesHorizontally", lookup.info.tiles_horizontally);
    set_bool(state, "tilesVertically", lookup.info.tiles_vertically);
    set_str(state, "filename", lookup.info.file);

    let raw_size_key = state.gc.intern_string(b"rawSize");
    if let Some(table) = state.gc.tables.get_mut(info_ref) {
        let _ = table.raw_set(Val::Str(raw_size_key), raw_size, &state.gc.string_arena);
    }

    state.push(info);
    Ok(1)
}

fn ensure_error_handler(state: &mut LuaState) -> LuaResult<Val> {
    let existing = registry_value(state, ERROR_HANDLER_KEY);
    if existing != Val::Nil {
        return Ok(existing);
    }
    let default_handler = build_default_error_handler(state)?;
    set_registry_value(state, ERROR_HANDLER_KEY, default_handler);
    Ok(default_handler)
}

fn build_default_error_handler(state: &mut LuaState) -> LuaResult<Val> {
    let func = state.load("return function(_msg) end")?;
    let call_base = state.top;
    state.ensure_stack(call_base + 2);
    state.stack_set(call_base, Val::Function(func.gc_ref()));
    state.top = call_base + 1;
    state.call_function(call_base, 1)?;
    let result = state.stack_get(call_base);
    state.top = call_base;
    Ok(result)
}

fn registry_value(state: &mut LuaState, key: &str) -> Val {
    let key_ref = state.gc.intern_string(key.as_bytes());
    state
        .gc
        .tables
        .get(state.registry)
        .map(|table| table.get_str(key_ref, &state.gc.string_arena))
        .unwrap_or(Val::Nil)
}

fn set_registry_value(state: &mut LuaState, key: &str, value: Val) {
    let key_ref = state.gc.intern_string(key.as_bytes());
    if let Some(table) = state.gc.tables.get_mut(state.registry) {
        let _ = table.raw_set(Val::Str(key_ref), value, &state.gc.string_arena);
    }
}

fn call_table_util_comparator(
    state: &mut LuaState,
    comparator: Val,
    left: Val,
    right: Val,
    index: usize,
) -> LuaResult<bool> {
    let call_base = state.top;
    state.ensure_stack(call_base + 5);
    state.stack_set(call_base, comparator);
    state.stack_set(call_base + 1, left);
    state.stack_set(call_base + 2, right);
    state.stack_set(call_base + 3, Val::Num(index as f64));
    state.top = call_base + 4;
    state.call_function(call_base, 1)?;
    let result = state.stack_get(call_base);
    state.top = call_base;
    Ok(!matches!(result, Val::Nil | Val::Bool(false)))
}

fn registry_bool(state: &mut LuaState, key: &str) -> bool {
    matches!(registry_get(state, key), Val::Bool(true))
}

fn set_registry_bool(state: &mut LuaState, key: &str, value: bool) {
    registry_set(state, key, Val::Bool(value));
}

fn addon_index_from_value(state: &LuaState, addon: Val) -> Option<usize> {
    match addon {
        Val::Num(index) if index.is_finite() && index.fract() == 0.0 && index >= 1.0 => {
            let index = index as usize;
            (index > 0).then_some(index - 1)
        }
        Val::Str(_) => {
            let name = val_to_string(state, addon)?;
            let sim = borrow_state(state).ok()?;
            sim.addons
                .iter()
                .position(|candidate| candidate.folder_name == name)
        }
        _ => None,
    }
}

fn addon_name_from_value(state: &LuaState, addon: Val) -> Option<String> {
    match addon {
        Val::Str(_) => val_to_string(state, addon),
        other => {
            let index = addon_index_from_value(state, other)?;
            let sim = borrow_state(state).ok()?;
            sim.addons.get(index).map(|addon| addon.folder_name.clone())
        }
    }
}

fn with_addon<R>(
    state: &LuaState,
    addon: Val,
    f: impl FnOnce(&crate::lua_api::AddonInfo) -> R,
) -> Option<R> {
    let index = addon_index_from_value(state, addon)?;
    let sim = borrow_state(state).ok()?;
    sim.addons.get(index).map(f)
}

fn default_runtime_addon_bases() -> Vec<PathBuf> {
    let root = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    vec![
        root.join("Interface/BlizzardUI"),
        root.join("Interface/AddOns"),
    ]
}

fn find_runtime_addon_toc(state: &LuaState, addon_name: &str) -> Option<PathBuf> {
    let bases = {
        let sim = borrow_state(state).ok()?;
        if sim.addon_base_paths.is_empty() {
            default_runtime_addon_bases()
        } else {
            sim.addon_base_paths.clone()
        }
    };
    for base in bases {
        let addon_dir = base.join(addon_name);
        if let Some(toc_path) = crate::loader::find_toc_file(&addon_dir) {
            return Some(toc_path);
        }
    }
    None
}

fn addon_exists(state: &LuaState, addon_name: &str) -> bool {
    let registered = {
        let sim = match borrow_state(state) {
            Ok(sim) => sim,
            Err(_) => return false,
        };
        sim.addons
            .iter()
            .any(|addon| addon.folder_name == addon_name)
    };
    registered || find_runtime_addon_toc(state, addon_name).is_some()
}

fn addon_metadata(addon: &crate::lua_api::AddonInfo, field: &str) -> Option<String> {
    match field {
        "Title" => Some(addon.title.clone()),
        "Notes" => (!addon.notes.is_empty()).then(|| addon.notes.clone()),
        "Version" => Some("@project-version@".to_string()),
        _ => None,
    }
}

fn push_addon_info(state: &mut LuaState, addon: &crate::lua_api::AddonInfo) -> u32 {
    let folder_name = create_string(state, &addon.folder_name);
    let title = create_string(state, &addon.title);
    let notes = (!addon.notes.is_empty()).then(|| create_string(state, &addon.notes));
    state.push(folder_name);
    state.push(title);
    if addon.notes.is_empty() {
        state.push(Val::Nil);
    } else {
        state.push(notes.unwrap_or(Val::Nil));
    }
    state.push(Val::Bool(addon.enabled));
    4
}

fn c_addons_get_num_addons(state: &mut LuaState) -> LuaResult<u32> {
    let count = borrow_state(state)?.addons.len() as f64;
    state.push(Val::Num(count));
    Ok(1)
}

fn c_addons_get_addon_info(state: &mut LuaState) -> LuaResult<u32> {
    let addon = stack_val(state, 1);
    let Some(index) = addon_index_from_value(state, addon) else {
        state.push(Val::Nil);
        return Ok(1);
    };
    let info = {
        let sim = borrow_state(state)?;
        sim.addons.get(index).cloned()
    };
    let Some(info) = info else {
        state.push(Val::Nil);
        return Ok(1);
    };
    Ok(push_addon_info(state, &info))
}

fn c_addons_is_addon_loaded(state: &mut LuaState) -> LuaResult<u32> {
    let loaded = with_addon(state, stack_val(state, 1), |addon| addon.loaded).unwrap_or(false);
    state.push(Val::Bool(loaded));
    Ok(1)
}

fn c_addons_is_addon_load_on_demand(state: &mut LuaState) -> LuaResult<u32> {
    let load_on_demand =
        with_addon(state, stack_val(state, 1), |addon| addon.load_on_demand).unwrap_or(false);
    state.push(Val::Bool(load_on_demand));
    Ok(1)
}

fn c_addons_get_addon_enable_state(state: &mut LuaState) -> LuaResult<u32> {
    let enabled = with_addon(state, stack_val(state, 1), |addon| addon.enabled).unwrap_or(false);
    state.push(Val::Num(if enabled { 2.0 } else { 0.0 }));
    Ok(1)
}

fn c_addons_enable_addon(state: &mut LuaState) -> LuaResult<u32> {
    if let Some(index) = addon_index_from_value(state, stack_val(state, 1))
        && let Some(addon) = borrow_state_mut(state)?.addons.get_mut(index)
    {
        addon.enabled = true;
    }
    Ok(0)
}

fn c_addons_disable_addon(state: &mut LuaState) -> LuaResult<u32> {
    if let Some(index) = addon_index_from_value(state, stack_val(state, 1))
        && let Some(addon) = borrow_state_mut(state)?.addons.get_mut(index)
        && addon.folder_name != "__BuiltIn"
    {
        addon.enabled = false;
    }
    Ok(0)
}

fn c_addons_enable_all_addons(state: &mut LuaState) -> LuaResult<u32> {
    let mut sim = borrow_state_mut(state)?;
    for addon in &mut sim.addons {
        addon.enabled = true;
    }
    Ok(0)
}

fn c_addons_disable_all_addons(state: &mut LuaState) -> LuaResult<u32> {
    let mut sim = borrow_state_mut(state)?;
    for addon in &mut sim.addons {
        if addon.folder_name != "__BuiltIn" {
            addon.enabled = false;
        }
    }
    Ok(0)
}

fn c_addons_get_addon_metadata(state: &mut LuaState) -> LuaResult<u32> {
    let addon_name = addon_name_from_value(state, stack_val(state, 1)).unwrap_or_default();
    let field = val_to_string(state, stack_val(state, 2)).unwrap_or_default();
    let value = with_addon(state, stack_val(state, 1), |addon| {
        addon_metadata(addon, &field)
    })
    .flatten()
    .or_else(|| (field == "Title").then_some(addon_name));
    match value {
        Some(value) => {
            let value = create_string(state, &value);
            state.push(value);
        }
        None => state.push(Val::Nil),
    }
    Ok(1)
}

fn c_addons_does_addon_exist(state: &mut LuaState) -> LuaResult<u32> {
    let addon_name = addon_name_from_value(state, stack_val(state, 1)).unwrap_or_default();
    state.push(Val::Bool(
        !addon_name.is_empty() && addon_exists(state, &addon_name),
    ));
    Ok(1)
}

fn c_addons_get_addon_name(state: &mut LuaState) -> LuaResult<u32> {
    let name = with_addon(state, stack_val(state, 1), |addon| {
        addon.folder_name.clone()
    });
    match name {
        Some(name) => {
            let name = create_string(state, &name);
            state.push(name);
        }
        None => state.push(Val::Nil),
    }
    Ok(1)
}

fn c_addons_get_addon_title(state: &mut LuaState) -> LuaResult<u32> {
    let title = with_addon(state, stack_val(state, 1), |addon| addon.title.clone());
    match title {
        Some(title) => {
            let title = create_string(state, &title);
            state.push(title);
        }
        None => state.push(Val::Nil),
    }
    Ok(1)
}

fn c_addons_get_addon_notes(state: &mut LuaState) -> LuaResult<u32> {
    let notes = with_addon(state, stack_val(state, 1), |addon| addon.notes.clone());
    match notes.filter(|notes| !notes.is_empty()) {
        Some(notes) => {
            let notes = create_string(state, &notes);
            state.push(notes);
        }
        None => state.push(Val::Nil),
    }
    Ok(1)
}

fn c_addons_get_addon_security(state: &mut LuaState) -> LuaResult<u32> {
    let security = with_addon(state, stack_val(state, 1), |addon| {
        if addon.folder_name == "__BuiltIn" || addon.folder_name.starts_with("Blizzard_") {
            "SECURE"
        } else {
            "INSECURE"
        }
    })
    .unwrap_or("INSECURE");
    let security = create_string(state, security);
    state.push(security);
    Ok(1)
}

fn c_addons_is_addon_version_check_enabled(state: &mut LuaState) -> LuaResult<u32> {
    let enabled = registry_bool(state, ADDON_VERSION_CHECK_KEY);
    state.push(Val::Bool(enabled));
    Ok(1)
}

fn c_addons_set_addon_version_check(state: &mut LuaState) -> LuaResult<u32> {
    set_registry_bool(
        state,
        ADDON_VERSION_CHECK_KEY,
        !matches!(stack_val(state, 1), Val::Nil | Val::Bool(false)),
    );
    Ok(0)
}

fn c_addons_load_addon(state: &mut LuaState) -> LuaResult<u32> {
    let Some(addon_name) = addon_name_from_value(state, stack_val(state, 1)) else {
        let missing = create_string(state, "MISSING");
        state.push(Val::Bool(false));
        state.push(missing);
        return Ok(2);
    };

    if with_addon(state, stack_val(state, 1), |addon| addon.loaded).unwrap_or(false) {
        state.push(Val::Bool(true));
        state.push(Val::Nil);
        return Ok(2);
    }

    let Some(toc_path) = find_runtime_addon_toc(state, &addon_name) else {
        let missing = create_string(state, "MISSING");
        state.push(Val::Bool(false));
        state.push(missing);
        return Ok(2);
    };

    let loader_env = LoaderEnv::from_parts_active(borrow_lua(state)?, state_handle(state)?, state);
    match crate::loader::load_addon(&loader_env, &toc_path) {
        Ok(_) => {
            {
                let mut sim = loader_env.state().borrow_mut();
                if let Some(addon) = sim
                    .addons
                    .iter_mut()
                    .find(|addon| addon.folder_name == addon_name)
                {
                    addon.loaded = true;
                    addon.enabled = true;
                }
            }
            let addon_name_val = create_string(state, &addon_name);
            let _ = loader_env.fire_event_with_args("ADDON_LOADED", &[addon_name_val]);
            state.push(Val::Bool(true));
            state.push(Val::Nil);
        }
        Err(error) => {
            let error = create_string(state, &error.to_string());
            state.push(Val::Bool(false));
            state.push(error);
        }
    }
    Ok(2)
}

fn legacy_get_addon_enable_state(state: &mut LuaState) -> LuaResult<u32> {
    state.push(Val::Num(2.0));
    Ok(1)
}

fn ui_widget_container_get_num_widgets_showing(state: &mut LuaState) -> LuaResult<u32> {
    let frame_id = frame_id_from_stack(state, 1)?;
    let count = {
        let sim = borrow_state(state)?;
        sim.widgets
            .get(frame_id)
            .map(|frame| {
                frame
                    .children
                    .iter()
                    .filter(|&&child_id| {
                        sim.widgets
                            .get(child_id)
                            .map(|child| child.visible)
                            .unwrap_or(false)
                    })
                    .count()
            })
            .unwrap_or(0) as f64
    };
    state.push(Val::Num(count));
    Ok(1)
}

fn profiler_metric_kind(_state: &LuaState, metric: Val) -> Option<i32> {
    match metric {
        Val::Num(value) if value.is_finite() && value.fract() == 0.0 => Some(value as i32),
        _ => None,
    }
}

fn average(values: impl Iterator<Item = f64>, count: usize) -> f64 {
    if count == 0 {
        0.0
    } else {
        values.sum::<f64>() / count as f64
    }
}

fn addon_metric_value(addon: &crate::lua_api::AddonInfo, metric: i32) -> f64 {
    match metric {
        0 => {
            if addon.runtime.session_frame_count == 0 {
                0.0
            } else {
                addon.runtime.session_total_ms / addon.runtime.session_frame_count as f64
            }
        }
        1 => average(
            addon.runtime.recent_frames.iter().copied(),
            addon.runtime.recent_frames.len(),
        ),
        4 => addon.runtime.peak_ms,
        _ => 0.0,
    }
}

fn application_metric_value(state: &crate::lua_api::SimState, metric: i32) -> f64 {
    match metric {
        0 => {
            if state.app_frame_metrics.session_frame_count == 0 {
                0.0
            } else {
                state.app_frame_metrics.session_total_ms
                    / state.app_frame_metrics.session_frame_count as f64
            }
        }
        1 => average(
            state.app_frame_metrics.recent_frame_ms.iter().copied(),
            state.app_frame_metrics.recent_frame_ms.len(),
        ),
        4 => state.app_frame_metrics.peak_ms,
        _ => 0.0,
    }
}

fn c_addon_profiler_get_application_metric(state: &mut LuaState) -> LuaResult<u32> {
    let metric = profiler_metric_kind(state, stack_val(state, 1)).unwrap_or(1);
    let value = {
        let sim = borrow_state(state)?;
        application_metric_value(&sim, metric)
    };
    state.push(Val::Num(value));
    Ok(1)
}

fn c_addon_profiler_get_overall_metric(state: &mut LuaState) -> LuaResult<u32> {
    let metric = profiler_metric_kind(state, stack_val(state, 1)).unwrap_or(1);
    let value = {
        let sim = borrow_state(state)?;
        sim.addons
            .iter()
            .filter(|addon| addon.folder_name != "__BuiltIn")
            .map(|addon| addon_metric_value(addon, metric))
            .sum::<f64>()
    };
    state.push(Val::Num(value));
    Ok(1)
}

fn c_addon_profiler_get_addon_metric(state: &mut LuaState) -> LuaResult<u32> {
    let addon_name = addon_name_from_value(state, stack_val(state, 1)).unwrap_or_default();
    let metric = profiler_metric_kind(state, stack_val(state, 2)).unwrap_or(1);
    let value = {
        let sim = borrow_state(state)?;
        sim.addons
            .iter()
            .find(|addon| addon.folder_name == addon_name)
            .map(|addon| addon_metric_value(addon, metric))
            .unwrap_or(0.0)
    };
    state.push(Val::Num(value));
    Ok(1)
}

fn c_addon_profiler_check_for_performance_message(_state: &mut LuaState) -> LuaResult<u32> {
    Ok(0)
}
