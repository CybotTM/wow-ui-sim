//! rilua RustFn equivalents of globals from utility_api, system_api, and spell_api.
//!
//! Each `pub fn` matches the `RustFn` signature:
//!   `fn(state: &mut LuaState) -> LuaResult<u32>`
//!
//! Arguments are extracted with `stack_val(state, n)` (1-based).
//! Return values are pushed with `state.push(val)` and counted in the return.
//!
//! Complex operations (pcall, xpcall, securecall) are stubbed with TODO.

mod c_addon_profiler;
mod c_addons;
mod c_model_info;
mod c_spec;
mod c_texture;
mod c_xml_util;
mod table_util;

use crate::lua_api::globals::unit_api::parse_party_index;
use crate::lua_api::rilua_methods::{borrow_state, create_string, create_table, val_to_string};
use crate::lua_api::rilua_script_helpers::{
    call_error_handler_state, protected_call_state, protected_lua_pcall_state,
};
use crate::lua_bridge::stack_val;
use rilua::LuaApiMut;
use rilua::vm::state::LuaState;
use rilua::{LuaResult, Val, runtime_error};

#[derive(Clone)]
struct UnitVitals {
    health: i32,
    health_max: i32,
    power: i32,
    power_max: i32,
    power_type: i32,
    power_type_name: String,
}

// ── Global table helpers ─────────────────────────────────────────────────────

pub(super) fn set_global_val(state: &mut LuaState, name: &str, value: Val) {
    let key = state.gc.intern_string(name.as_bytes());
    let global = state.global;
    if let Some(g) = state.gc.tables.get_mut(global) {
        let _ = g.raw_set(Val::Str(key), value, &state.gc.string_arena);
    }
    state.gc.barrier_back(global);
}

pub(super) fn global_val(state: &mut LuaState, name: &str) -> Val {
    let key = state.gc.intern_string(name.as_bytes());
    state
        .gc
        .tables
        .get(state.global)
        .map(|table| table.get_str(key, &state.gc.string_arena))
        .unwrap_or(Val::Nil)
}

pub(super) fn ensure_global_table(state: &mut LuaState, name: &str) -> Val {
    match global_val(state, name) {
        table @ Val::Table(_) => table,
        _ => {
            let table = create_table(state);
            set_global_val(state, name, table);
            table
        }
    }
}

// ── Unit vitals helpers ──────────────────────────────────────────────────────

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
    state.gc.barrier_back(table_ref);

    state.push(t);
    Ok(1)
}

/// tinsert(t [, pos], value) — append or insert a value into an array table.
pub fn tinsert(_state: &mut LuaState) -> LuaResult<u32> {
    // TODO: implement via table array ops
    Ok(0)
}

/// tremove(t [, pos]) — remove and return a value from an array table.
pub fn tremove(state: &mut LuaState) -> LuaResult<u32> {
    // TODO: implement via table array ops
    state.push(Val::Nil);
    Ok(1)
}

/// tContains(t, value) — return true if value is present in the array part of t.
pub fn t_contains(state: &mut LuaState) -> LuaResult<u32> {
    // TODO: iterate array part and compare
    state.push(Val::Bool(false));
    Ok(1)
}

/// tIndexOf(t, value) — return the integer index of value in t, or nil.
pub fn t_index_of(state: &mut LuaState) -> LuaResult<u32> {
    // TODO: iterate array part and compare
    state.push(Val::Nil);
    Ok(1)
}

pub use table_util::t_invert;
pub use table_util::table_util_find_indexed_mismatch;

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
    state.gc.barrier_back(global);
    Ok(0)
}

/// nop(...) — no-operation, discards all arguments.
pub fn nop(_state: &mut LuaState) -> LuaResult<u32> {
    Ok(0)
}

/// `strsplit(delimiter, str [, limit])` — split `str` on any character in
/// `delimiter` and push each piece as a separate return value.
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
/// separated by `delimiter`.
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

/// type(v) — return the Lua type name of v as a string.
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
pub fn pcall(state: &mut LuaState) -> LuaResult<u32> {
    let func = stack_val(state, 1);
    let args: Vec<Val> = ((state.base + 1)..state.top)
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
pub fn xpcall(state: &mut LuaState) -> LuaResult<u32> {
    let func = stack_val(state, 1);
    let handler = stack_val(state, 2);
    let args: Vec<Val> = ((state.base + 2)..state.top)
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
    let registry = state.registry;
    if let Some(table) = state.gc.tables.get_mut(registry) {
        let _ = table.raw_set(Val::Str(key_ref), value, &state.gc.string_arena);
    }
    state.gc.barrier_back(registry);
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
    let power_type_name_val = create_string(state, &vitals.power_type_name);
    state.push(Val::Num(vitals.power_type as f64));
    state.push(power_type_name_val);
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
pub fn cast_spell_by_id(_state: &mut LuaState) -> LuaResult<u32> {
    Ok(0)
}

/// CastSpellByName(name [, unit]) — cast a spell by name.
pub fn cast_spell_by_name(_state: &mut LuaState) -> LuaResult<u32> {
    Ok(0)
}

fn unit_casting_info(state: &mut LuaState) -> LuaResult<u32> {
    let unit = val_to_string(state, stack_val(state, 1)).unwrap_or_default();
    if unit != "player" {
        return Ok(0);
    }
    let cast = extract_cast_info(state)?;
    let Some((spell_name, icon_path, start_time, end_time, cast_id, spell_id)) = cast else {
        return Ok(0);
    };
    push_cast_info(
        state, spell_name, icon_path, start_time, end_time, cast_id, spell_id,
    );
    Ok(9)
}

fn extract_cast_info(
    state: &mut LuaState,
) -> LuaResult<Option<(String, String, f64, f64, u32, u32)>> {
    let sim = borrow_state(state)?;
    Ok(sim.casting.as_ref().map(|cast| {
        (
            cast.spell_name.clone(),
            cast.icon_path.clone(),
            cast.start_time,
            cast.end_time,
            cast.cast_id,
            cast.spell_id,
        )
    }))
}

fn push_cast_info(
    state: &mut LuaState,
    spell_name: String,
    icon_path: String,
    start_time: f64,
    end_time: f64,
    cast_id: u32,
    spell_id: u32,
) {
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
}

fn unit_channel_info(_state: &mut LuaState) -> LuaResult<u32> {
    Ok(0)
}

// ── Registration ─────────────────────────────────────────────────────────────

/// Register all functions in this module as rilua globals.
///
/// Call this once after the rilua `Lua` instance is created.
pub fn register_all(lua: &mut rilua::Lua) -> rilua::LuaResult<()> {
    register_utility_globals(lua)?;
    register_system_globals(lua)?;
    register_spell_globals(lua)?;

    let state = lua.state_mut();
    table_util::register_table_util(state)?;
    c_addons::register_c_addons(state)?;
    c_addon_profiler::register_c_addon_profiler(state)?;
    c_spec::register_c_specialization_info(state)?;
    c_model_info::register_c_model_info(state)?;
    c_model_info::register_c_lfg_info(state)?;
    c_model_info::register_c_wowtoken_secure(state)?;
    c_texture::register_c_texture(state)?;
    c_xml_util::register_c_xml_util(state)?;
    c_addons::register_legacy_addon_globals(state)?;
    c_spec::register_widget_container_mixin(state)?;

    Ok(())
}

fn register_utility_globals(lua: &mut rilua::Lua) -> LuaResult<()> {
    LuaApiMut::register_function(lua, "wipe", wipe)?;
    LuaApiMut::register_function(lua, "tinsert", tinsert)?;
    LuaApiMut::register_function(lua, "tremove", tremove)?;
    LuaApiMut::register_function(lua, "tContains", t_contains)?;
    LuaApiMut::register_function(lua, "tIndexOf", t_index_of)?;
    LuaApiMut::register_function(lua, "tInvert", t_invert)?;
    LuaApiMut::register_function(lua, "getglobal", getglobal)?;
    LuaApiMut::register_function(lua, "setglobal", setglobal)?;
    LuaApiMut::register_function(lua, "nop", nop)?;
    LuaApiMut::register_function(lua, "strsplit", strsplit)?;
    LuaApiMut::register_function(lua, "strjoin", strjoin)?;
    Ok(())
}

fn register_system_globals(lua: &mut rilua::Lua) -> LuaResult<()> {
    LuaApiMut::register_function(lua, "type", type_fn)?;
    LuaApiMut::register_function(lua, "IsPublicTestClient", is_public_test_client)?;
    LuaApiMut::register_function(lua, "IsBetaBuild", is_beta_build)?;
    LuaApiMut::register_function(lua, "IsPublicBuild", is_public_build)?;
    LuaApiMut::register_function(lua, "BNFeaturesEnabled", bn_features_enabled)?;
    LuaApiMut::register_function(
        lua,
        "BNFeaturesEnabledAndConnected",
        bn_features_enabled_and_connected,
    )?;
    LuaApiMut::register_function(lua, "BNConnected", bn_connected)?;
    LuaApiMut::register_function(lua, "IsGMClient", is_gm_client)?;
    LuaApiMut::register_function(lua, "RegisterStaticConstants", register_static_constants)?;
    LuaApiMut::register_function(lua, "pcall", pcall)?;
    LuaApiMut::register_function(lua, "xpcall", xpcall)?;
    LuaApiMut::register_function(lua, "securecall", securecall)?;
    LuaApiMut::register_function(lua, "seterrorhandler", seterrorhandler)?;
    LuaApiMut::register_function(lua, "geterrorhandler", geterrorhandler)?;
    Ok(())
}

fn register_spell_globals(lua: &mut rilua::Lua) -> LuaResult<()> {
    LuaApiMut::register_function(lua, "UnitHealth", unit_health)?;
    LuaApiMut::register_function(lua, "UnitHealthMax", unit_health_max)?;
    LuaApiMut::register_function(lua, "UnitPower", unit_power)?;
    LuaApiMut::register_function(lua, "UnitPowerMax", unit_power_max)?;
    LuaApiMut::register_function(lua, "UnitPowerType", unit_power_type)?;
    LuaApiMut::register_function(lua, "UnitGetIncomingHeals", unit_get_incoming_heals)?;
    LuaApiMut::register_function(lua, "UnitGetTotalAbsorbs", unit_get_total_absorbs)?;
    LuaApiMut::register_function(lua, "UnitGetTotalHealAbsorbs", unit_get_total_heal_absorbs)?;
    LuaApiMut::register_function(lua, "CastSpellByID", cast_spell_by_id)?;
    LuaApiMut::register_function(lua, "CastSpellByName", cast_spell_by_name)?;
    LuaApiMut::register_function(lua, "UnitCastingInfo", unit_casting_info)?;
    LuaApiMut::register_function(lua, "UnitChannelInfo", unit_channel_info)?;
    LuaApiMut::register_function(
        lua,
        "PlayerGetTimerunningSeasonID",
        c_spec::player_get_timerunning_season_id,
    )?;
    Ok(())
}
