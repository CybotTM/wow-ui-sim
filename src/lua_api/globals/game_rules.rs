//! `C_GameRules` namespace backed by `SimState::game_rules`.
//!
//! Methods:
//!
//! - `IsGameRuleActive(name)`        — whether a rule is set.
//! - `GetGameRuleAsFloat(name)`      — float value (0 when missing).
//! - `GetGameRuleAsInt(name)`        — int value (0 when missing).
//! - `GetGameRuleAsString(name)`     — string value (`""` when missing).
//! - `IsPlunderstorm()`              — active_game_mode == Plunderstorm.
//! - `GetActiveGameMode()`           — the active mode id; prefers
//!                                     `Enum.GameMode.Standard` (falls back
//!                                     to the raw int when the enum's not
//!                                     loaded).
//! - `GetGameModeGlueScreenName()`   — glue screen string.
//!
//! Admin:
//! - `A_Admin.SetGameRule(name, value)` — value by Lua type: number →
//!   populates all three (int = as i64, float = as f64, string = str), string
//!   → string form only, bool → clears on false / sets 0/0/"" with true as
//!   marker. `nil` removes the rule.
//! - `A_Admin.SetActiveGameMode(mode, glueScreen?)` — mode is an integer;
//!   glueScreen defaults to `"CharacterSelect"`.

use crate::lua_api::methods::{borrow_state, borrow_state_mut, create_string, create_table};
use crate::lua_api::state::GameRuleValue;
use crate::lua_bridge::{stack_val, table_set_rust_fn, FromStack};
use rilua::vm::state::LuaState;
use rilua::{LuaResult, Val};

/// Canonical mode id for Plunderstorm, used by `IsPlunderstorm()`.
const PLUNDERSTORM_MODE: i32 = 1;

fn read_string_arg(state: &mut LuaState, index: i32) -> LuaResult<Option<String>> {
    Option::<String>::from_stack(state, index)
}

pub fn is_game_rule_active(state: &mut LuaState) -> LuaResult<u32> {
    let active = match read_string_arg(state, 1)? {
        Some(name) => borrow_state(state)?.game_rules.rules.contains_key(&name),
        None => false,
    };
    state.push(Val::Bool(active));
    Ok(1)
}

pub fn get_game_rule_as_float(state: &mut LuaState) -> LuaResult<u32> {
    let value = match read_string_arg(state, 1)? {
        Some(name) => borrow_state(state)?
            .game_rules
            .rules
            .get(&name)
            .map(|r| r.as_float)
            .unwrap_or(0.0),
        None => 0.0,
    };
    state.push(Val::Num(value));
    Ok(1)
}

pub fn get_game_rule_as_int(state: &mut LuaState) -> LuaResult<u32> {
    let value = match read_string_arg(state, 1)? {
        Some(name) => borrow_state(state)?
            .game_rules
            .rules
            .get(&name)
            .map(|r| r.as_int)
            .unwrap_or(0),
        None => 0,
    };
    state.push(Val::Num(value as f64));
    Ok(1)
}

pub fn get_game_rule_as_string(state: &mut LuaState) -> LuaResult<u32> {
    let value = match read_string_arg(state, 1)? {
        Some(name) => borrow_state(state)?
            .game_rules
            .rules
            .get(&name)
            .map(|r| r.as_string.clone())
            .unwrap_or_default(),
        None => String::new(),
    };
    let val = create_string(state, &value);
    state.push(val);
    Ok(1)
}

pub fn is_plunderstorm(state: &mut LuaState) -> LuaResult<u32> {
    let active = borrow_state(state)?.game_rules.active_game_mode == PLUNDERSTORM_MODE;
    state.push(Val::Bool(active));
    Ok(1)
}

pub fn get_active_game_mode(state: &mut LuaState) -> LuaResult<u32> {
    let mode = borrow_state(state)?.game_rules.active_game_mode;
    state.push(Val::Num(mode as f64));
    Ok(1)
}

pub fn get_game_mode_glue_screen_name(state: &mut LuaState) -> LuaResult<u32> {
    let name = borrow_state(state)?.game_rules.glue_screen_name.clone();
    let val = create_string(state, &name);
    state.push(val);
    Ok(1)
}

fn install_on_c_game_rules(state: &mut LuaState) -> LuaResult<()> {
    use rilua::vm::gc::arena::GcRef;
    use rilua::vm::table::Table;

    let key = state.gc.intern_string_static(b"C_GameRules");
    let global = state.global;
    let existing = state
        .gc
        .tables
        .get(global)
        .map(|t| t.get_str(key, &state.gc.string_arena));
    let table_ref: GcRef<Table> = match existing {
        Some(Val::Table(r)) => r,
        _ => {
            let new_val = create_table(state);
            let Val::Table(new_ref) = new_val else {
                unreachable!("create_table must return a table");
            };
            if let Some(global_table) = state.gc.tables.get_mut(global) {
                let _ = global_table.raw_set(Val::Str(key), new_val, &state.gc.string_arena);
            }
            state.gc.barrier_back(global);
            new_ref
        }
    };

    let entries: &[(&str, rilua::vm::closure::RustFn)] = &[
        ("IsGameRuleActive", is_game_rule_active),
        ("GetGameRuleAsFloat", get_game_rule_as_float),
        ("GetGameRuleAsInt", get_game_rule_as_int),
        ("GetGameRuleAsString", get_game_rule_as_string),
        ("IsPlunderstorm", is_plunderstorm),
        ("GetActiveGameMode", get_active_game_mode),
        ("GetGameModeGlueScreenName", get_game_mode_glue_screen_name),
    ];
    for (name, func) in entries {
        table_set_rust_fn(state, table_ref, name, *func)?;
    }
    Ok(())
}

pub fn register_all(lua: &mut rilua::Lua) -> LuaResult<()> {
    use rilua::LuaApiMut;
    install_on_c_game_rules(lua.state_mut())
}

// ── Admin setters (exposed via admin_zone_economy) ────────────────────────────

/// Rule value snapshot read from the Lua stack before acquiring the mutable
/// SimState borrow.
enum RuleOp {
    Remove,
    Set(GameRuleValue),
}

fn read_rule_op(state: &mut LuaState) -> RuleOp {
    match stack_val(state, 2) {
        Val::Nil => RuleOp::Remove,
        Val::Num(n) => RuleOp::Set(GameRuleValue {
            as_float: n,
            as_int: n as i64,
            as_string: format!("{}", n),
        }),
        Val::Bool(true) => RuleOp::Set(GameRuleValue {
            as_float: 1.0,
            as_int: 1,
            as_string: "true".into(),
        }),
        Val::Bool(false) => RuleOp::Remove,
        Val::Str(s) => {
            let text = state
                .gc
                .string_arena
                .get(s)
                .and_then(|lua_str| std::str::from_utf8(lua_str.data()).ok())
                .map(str::to_owned)
                .unwrap_or_default();
            let parsed_f = text.parse::<f64>().unwrap_or(0.0);
            let parsed_i = text.parse::<i64>().unwrap_or(parsed_f as i64);
            RuleOp::Set(GameRuleValue {
                as_float: parsed_f,
                as_int: parsed_i,
                as_string: text,
            })
        }
        _ => RuleOp::Remove,
    }
}

/// Install a game rule by name + Lua value.
///
/// - `nil` / `false`: removes the rule.
/// - number: stored as all three forms (`as_float = n`, `as_int = n as i64`,
///   `as_string = "n"`).
/// - string: stored as-is; float/int forms try to parse the string, default 0.
/// - boolean `true`: stored as a marker with `("true", 1, 1.0)`.
pub fn admin_set_game_rule(state: &mut LuaState) -> LuaResult<u32> {
    let Some(name) = Option::<String>::from_stack(state, 1)? else {
        return Ok(0);
    };
    let op = read_rule_op(state);
    let mut sim = borrow_state_mut(state)?;
    match op {
        RuleOp::Remove => {
            sim.game_rules.rules.remove(&name);
        }
        RuleOp::Set(value) => {
            sim.game_rules.rules.insert(name, value);
        }
    }
    Ok(0)
}

pub fn admin_set_active_game_mode(state: &mut LuaState) -> LuaResult<u32> {
    let mode = Option::<f64>::from_stack(state, 1)?.unwrap_or(0.0) as i32;
    let glue = Option::<String>::from_stack(state, 2)?
        .filter(|s| !s.is_empty())
        .unwrap_or_else(|| "CharacterSelect".into());
    let mut sim = borrow_state_mut(state)?;
    sim.game_rules.active_game_mode = mode;
    sim.game_rules.glue_screen_name = glue;
    Ok(0)
}
