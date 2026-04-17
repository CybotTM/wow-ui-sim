//! `GuildControl*` rank-admin probes, backed by `SimState::world.guild_ranks`.
//!
//! Real signatures:
//!
//! - `GuildControlSetRank(rankIndex)`         — selects which rank subsequent
//!                                              getters reference; no-op if
//!                                              out of range.
//! - `GuildControlGetRankName(rankIndex?)`    — returns the rank's display
//!                                              name, or `""` if no guild /
//!                                              out of range.
//! - `GuildControlGetNumRanks()`              — total rank count; `0` when
//!                                              the player has no guild.
//! - `GuildControlGetRankFlags(rankIndex?)`   — flag table (`{ bool, bool,
//!                                              ... }`); `{}` when none.
//!
//! The sim models `world.guild_ranks: Vec<GuildRank>` plus a 1-based
//! `world.guild_selected_rank`. Admin API:
//! `A_Admin.SetGuildRanks({ {name, flags}, ... })` to install a roster.

use crate::lua_api::methods::{borrow_state, borrow_state_mut, create_string, create_table};
use crate::lua_bridge::{table_set_rust_fn, FromStack};
use rilua::vm::state::LuaState;
use rilua::{LuaResult, Val};

/// Resolve the 1-based rank index the caller asked about: explicit arg
/// first, else the "selected" rank. `0`/out-of-range → `None`.
fn resolve_rank_index(state: &mut LuaState, explicit_arg: i32) -> LuaResult<Option<usize>> {
    let from_arg = Option::<f64>::from_stack(state, explicit_arg)?
        .map(|n| n as i64)
        .filter(|n| *n > 0);
    let idx = match from_arg {
        Some(i) => i,
        None => borrow_state(state)?.world.guild_selected_rank as i64,
    };
    if idx <= 0 {
        return Ok(None);
    }
    let len = borrow_state(state)?.world.guild_ranks.len() as i64;
    if idx > len {
        return Ok(None);
    }
    Ok(Some((idx - 1) as usize))
}

pub fn guild_control_set_rank(state: &mut LuaState) -> LuaResult<u32> {
    let rank = Option::<f64>::from_stack(state, 1)?
        .map(|n| n as i64)
        .unwrap_or(0);
    let mut sim = borrow_state_mut(state)?;
    let len = sim.world.guild_ranks.len() as i64;
    if rank >= 1 && rank <= len {
        sim.world.guild_selected_rank = rank as i32;
    } else {
        // Out-of-range selection clears, matching WoW's "no rank" state.
        sim.world.guild_selected_rank = 0;
    }
    Ok(0)
}

pub fn guild_control_get_rank_name(state: &mut LuaState) -> LuaResult<u32> {
    let name = match resolve_rank_index(state, 1)? {
        Some(idx) => borrow_state(state)?
            .world
            .guild_ranks
            .get(idx)
            .map(|r| r.name.clone())
            .unwrap_or_default(),
        None => String::new(),
    };
    let val = create_string(state, &name);
    state.push(val);
    Ok(1)
}

pub fn guild_control_get_num_ranks(state: &mut LuaState) -> LuaResult<u32> {
    let count = borrow_state(state)?.world.guild_ranks.len() as f64;
    state.push(Val::Num(count));
    Ok(1)
}

pub fn guild_control_get_rank_flags(state: &mut LuaState) -> LuaResult<u32> {
    let flags = resolve_rank_index(state, 1)?.and_then(|idx| {
        borrow_state(state)
            .ok()
            .and_then(|sim| sim.world.guild_ranks.get(idx).map(|r| r.flags.clone()))
    });
    let table = create_table(state);
    let Val::Table(table_ref) = table else {
        unreachable!("create_table must return a table");
    };
    if let Some(flags) = flags {
        if let Some(t) = state.gc.tables.get_mut(table_ref) {
            for (i, flag) in flags.iter().enumerate() {
                let _ = t.raw_set(
                    Val::Num((i + 1) as f64),
                    Val::Bool(*flag),
                    &state.gc.string_arena,
                );
            }
        }
        state.gc.barrier_back(table_ref);
    }
    state.push(table);
    Ok(1)
}

pub fn register_all(lua: &mut rilua::Lua) -> LuaResult<()> {
    use rilua::LuaApiMut;
    let state = lua.state_mut();
    let g = state.global;
    table_set_rust_fn(state, g, "GuildControlSetRank", guild_control_set_rank)?;
    table_set_rust_fn(
        state,
        g,
        "GuildControlGetRankName",
        guild_control_get_rank_name,
    )?;
    table_set_rust_fn(
        state,
        g,
        "GuildControlGetNumRanks",
        guild_control_get_num_ranks,
    )?;
    table_set_rust_fn(
        state,
        g,
        "GuildControlGetRankFlags",
        guild_control_get_rank_flags,
    )?;
    Ok(())
}
