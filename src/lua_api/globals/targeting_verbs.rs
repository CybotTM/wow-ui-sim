//! SimState-backed implementations of WoW player targeting globals.
//!
//! Migrated off `GLOBAL_NIL_STUBS` in `stubs/global_stubs.rs`. Each function
//! reads and writes `SimState::current_target`, `current_focus`,
//! `previous_target`, and `enemy_pool` then queues the matching WoW event.
//!
//! | Global               | Event fired                |
//! |----------------------|----------------------------|
//! | `TargetUnit`         | `PLAYER_TARGET_CHANGED`    |
//! | `FocusUnit`          | `PLAYER_FOCUS_CHANGED`     |
//! | `ClearTarget`        | `PLAYER_TARGET_CHANGED`    |
//! | `TargetLastTarget`   | `PLAYER_TARGET_CHANGED`    |
//! | `TargetNearestEnemy` | `PLAYER_TARGET_CHANGED`    |
//! | `TargetNearestFriend`| `PLAYER_TARGET_CHANGED`    |
//!
//! `TargetUnit` resolves the supplied unit token against the current sim
//! state using the same token set as `UnitExists` / `UnitName`. Unrecognised
//! tokens are a silent no-op (matches real WoW: targeting an invalid unit
//! does nothing).
//!
//! `TargetNearestEnemy` / `TargetNearestFriend` pick the first entry of
//! `enemy_pool` / party members respectively. Empty pool → silent no-op.

use crate::event::Event;
use crate::lua_api::game_data::{PartyMember, TargetInfo};
use crate::lua_api::methods::borrow_state_mut;
use crate::lua_bridge::{FromStack, table_set_rust_fn};
use rilua::LuaResult;
use rilua::vm::state::LuaState;

// ── Token resolution ──────────────────────────────────────────────────────────

/// Resolve a unit token to a `TargetInfo` snapshot from the current state.
/// Returns `None` for tokens that don't map to an existing unit.
fn resolve_token_to_target_info(
    state: &mut LuaState,
    token: &str,
) -> LuaResult<Option<TargetInfo>> {
    let st = borrow_state_mut(state)?;
    let result = match token.to_ascii_lowercase().as_str() {
        "player" | "self" => {
            let t = TargetInfo {
                unit_id: "player".to_string(),
                name: st.player.name.clone(),
                class_index: st.player.class_index,
                level: st.player.level,
                health: st.player.health,
                health_max: st.player.health_max,
                power: st.player.power,
                power_max: st.player.power_max,
                power_type: st.player.power_type,
                power_type_name: "MANA".to_string(),
                is_player: true,
                is_enemy: false,
                guid: "Player-0000-00000000".to_string(),
                classification: "normal".to_string(),
                creature_type: "Humanoid".to_string(),
                reaction: 5,
            };
            Some(t)
        }
        "target" => st.current_target.clone(),
        "focus" => st.current_focus.clone(),
        other => {
            if let Some(idx) = parse_party_slot(other) {
                if st.party_group_active {
                    st.party_members.get(idx).map(party_member_to_target_info)
                } else {
                    None
                }
            } else {
                None
            }
        }
    };
    Ok(result)
}

fn parse_party_slot(token: &str) -> Option<usize> {
    let b = token.as_bytes();
    if b.len() == 6 && b[..5].eq_ignore_ascii_case(b"party") {
        match b[5] {
            b'1' => Some(0),
            b'2' => Some(1),
            b'3' => Some(2),
            b'4' => Some(3),
            _ => None,
        }
    } else {
        None
    }
}

fn party_member_to_target_info(m: &PartyMember) -> TargetInfo {
    TargetInfo {
        unit_id: "target".to_string(),
        name: m.name.clone(),
        class_index: m.class_index,
        level: m.level,
        health: m.health,
        health_max: m.health_max,
        power: m.power,
        power_max: m.power_max,
        power_type: m.power_type,
        power_type_name: m.power_type_name.clone(),
        is_player: true,
        is_enemy: false,
        guid: "Player-0000-00000000".to_string(),
        classification: "normal".to_string(),
        creature_type: "Humanoid".to_string(),
        reaction: 5,
    }
}

// ── Event helpers ─────────────────────────────────────────────────────────────

fn push_target_changed(state: &mut LuaState) -> LuaResult<()> {
    borrow_state_mut(state)?.events.push(Event {
        name: "PLAYER_TARGET_CHANGED".to_string(),
        args: Vec::new(),
    });
    Ok(())
}

fn push_focus_changed(state: &mut LuaState) -> LuaResult<()> {
    borrow_state_mut(state)?.events.push(Event {
        name: "PLAYER_FOCUS_CHANGED".to_string(),
        args: Vec::new(),
    });
    Ok(())
}

// ── Globals ───────────────────────────────────────────────────────────────────

/// `TargetUnit(unit)` — resolve token, set `current_target`, snapshot previous.
pub fn target_unit(state: &mut LuaState) -> LuaResult<u32> {
    let token = match Option::<String>::from_stack(state, 1)? {
        Some(t) => t,
        None => return Ok(0),
    };
    let new_target = resolve_token_to_target_info(state, &token)?;
    {
        let mut st = borrow_state_mut(state)?;
        let old = st.current_target.take();
        st.previous_target = old;
        st.current_target = new_target;
    }
    push_target_changed(state)?;
    Ok(0)
}

/// `FocusUnit(unit)` — resolve token, set `current_focus`.
pub fn focus_unit(state: &mut LuaState) -> LuaResult<u32> {
    let token = match Option::<String>::from_stack(state, 1)? {
        Some(t) => t,
        None => return Ok(0),
    };
    let new_focus = resolve_token_to_target_info(state, &token)?;
    {
        let mut st = borrow_state_mut(state)?;
        st.current_focus = new_focus;
    }
    push_focus_changed(state)?;
    Ok(0)
}

/// `ClearTarget()` — clear `current_target`, snapshot to `previous_target`.
pub fn clear_target(state: &mut LuaState) -> LuaResult<u32> {
    {
        let mut st = borrow_state_mut(state)?;
        let old = st.current_target.take();
        st.previous_target = old;
    }
    push_target_changed(state)?;
    Ok(0)
}

/// `TargetLastTarget()` — swap `current_target` ↔ `previous_target`.
pub fn target_last_target(state: &mut LuaState) -> LuaResult<u32> {
    let has_previous = borrow_state_mut(state)?.previous_target.is_some();
    if !has_previous {
        return Ok(0);
    }
    {
        let mut st = borrow_state_mut(state)?;
        let old_current = st.current_target.take();
        let previous = st.previous_target.take();
        st.current_target = previous;
        st.previous_target = old_current;
    }
    push_target_changed(state)?;
    Ok(0)
}

/// `TargetNearestEnemy()` — pick first member of `enemy_pool`. No-op when empty.
pub fn target_nearest_enemy(state: &mut LuaState) -> LuaResult<u32> {
    let new_target = {
        let st = borrow_state_mut(state)?;
        st.enemy_pool.first().cloned()
    };
    if new_target.is_none() {
        return Ok(0);
    }
    {
        let mut st = borrow_state_mut(state)?;
        let old = st.current_target.take();
        st.previous_target = old;
        st.current_target = new_target;
    }
    push_target_changed(state)?;
    Ok(0)
}

/// `TargetNearestFriend()` — pick first party member. No-op when party is empty.
pub fn target_nearest_friend(state: &mut LuaState) -> LuaResult<u32> {
    let new_target = {
        let st = borrow_state_mut(state)?;
        if st.party_group_active {
            st.party_members.first().map(party_member_to_target_info)
        } else {
            None
        }
    };
    if new_target.is_none() {
        return Ok(0);
    }
    {
        let mut st = borrow_state_mut(state)?;
        let old = st.current_target.take();
        st.previous_target = old;
        st.current_target = new_target;
    }
    push_target_changed(state)?;
    Ok(0)
}

// ── Admin setter ──────────────────────────────────────────────────────────────

/// `A_Admin.SetEnemyPool({name, level, class_index, is_enemy=true}, ...)`.
///
/// Replaces `SimState::enemy_pool` entirely. Each entry in the Lua array must
/// be a table with at minimum a `name` key. `level` defaults to 63,
/// `class_index` to 1. `is_enemy` is always forced true regardless of what
/// the caller passes.
pub fn admin_set_enemy_pool(state: &mut LuaState) -> LuaResult<u32> {
    use crate::lua_api::globals::admin_party_target_helpers::make_target_info;
    use crate::lua_api::methods::table_get;
    use rilua::Val;

    let nargs = state.top as i32 - state.base as i32;
    let mut pool: Vec<TargetInfo> = Vec::new();
    for i in 1..=nargs {
        let entry_val = crate::lua_bridge::stack_val(state, i);
        let Val::Table(_) = entry_val else {
            continue;
        };

        let name_val = table_get(state, entry_val, "name");
        let name = match name_val {
            Val::Str(s) => state
                .gc
                .string_arena
                .get(s)
                .map(|ls| String::from_utf8_lossy(ls.data()).into_owned())
                .unwrap_or_else(|| "Unknown".to_string()),
            _ => "Unknown".to_string(),
        };

        let level_val = table_get(state, entry_val, "level");
        let level = match level_val {
            Val::Num(n) => n as i32,
            _ => 63,
        };

        let class_val = table_get(state, entry_val, "class_index");
        let class_index = match class_val {
            Val::Num(n) => n as i32,
            _ => 1,
        };

        pool.push(make_target_info("target", &name, level, class_index, true));
    }
    borrow_state_mut(state)?.enemy_pool = pool;
    Ok(0)
}

// ── Registration ──────────────────────────────────────────────────────────────

pub fn register_all(lua: &mut rilua::Lua) -> LuaResult<()> {
    use rilua::LuaApiMut;
    let state = lua.state_mut();
    let g = state.global;
    table_set_rust_fn(state, g, "TargetUnit", target_unit)?;
    table_set_rust_fn(state, g, "FocusUnit", focus_unit)?;
    table_set_rust_fn(state, g, "ClearTarget", clear_target)?;
    table_set_rust_fn(state, g, "TargetLastTarget", target_last_target)?;
    table_set_rust_fn(state, g, "TargetNearestEnemy", target_nearest_enemy)?;
    table_set_rust_fn(state, g, "TargetNearestFriend", target_nearest_friend)?;
    Ok(())
}
