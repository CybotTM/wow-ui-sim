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
//! - `C_GuildInfo.GuildControlGetRankFlags(rankIndex?)` — namespace alias for
//!                                                        the same data.
//! - `GetNumMembersInRank(rankIndex)`         — number of roster entries with
//!                                              the 1-based rank index.
//! - `GuildControlGetAllowedShifts(rankIndex)` — whether a rank can move up
//!                                               or down in the editable list.
//!
//! The sim models `world.guild_ranks: Vec<GuildRank>` plus a 1-based
//! `world.guild_selected_rank`. Admin API:
//! `A_Admin.SetGuildRanks({ {name, flags}, ... })` to install a roster.

use crate::lua_api::methods::{
    borrow_state, borrow_state_mut, create_string, create_table, val_to_string,
};
use crate::lua_bridge::{FromStack, stack_val, table_set_rust_fn_static};
use rilua::RustFn;
use rilua::vm::gc::arena::GcRef;
use rilua::vm::state::LuaState;
use rilua::vm::table::Table;
use rilua::{LuaResult, Val};

/// Resolve the 1-based rank index the caller asked about: explicit arg
/// first, else the "selected" rank. `0`/out-of-range → `None`.
fn resolve_rank_index(state: &mut LuaState, explicit_arg: i32) -> LuaResult<Option<usize>> {
    if borrow_state(state)?.world.guild_name.is_none() {
        return Ok(None);
    }
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
    if sim.world.guild_name.is_none() {
        sim.world.guild_selected_rank = 0;
        return Ok(0);
    }
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
    let count = {
        let sim = borrow_state(state)?;
        if sim.world.guild_name.is_some() {
            sim.world.guild_ranks.len() as f64
        } else {
            0.0
        }
    };
    state.push(Val::Num(count));
    Ok(1)
}

pub fn get_num_members_in_rank(state: &mut LuaState) -> LuaResult<u32> {
    let rank = Option::<f64>::from_stack(state, 1)?
        .map(|n| n as i32)
        .unwrap_or(0);
    let count = {
        let sim = borrow_state(state)?;
        if sim.world.guild_name.is_some() && rank > 0 {
            sim.world
                .guild_members
                .iter()
                .filter(|member| member.rank_index == rank)
                .count() as f64
        } else {
            0.0
        }
    };
    state.push(Val::Num(count));
    Ok(1)
}

pub fn guild_control_get_allowed_shifts(state: &mut LuaState) -> LuaResult<u32> {
    let rank = Option::<f64>::from_stack(state, 1)?
        .map(|n| n as i32)
        .unwrap_or(0);
    let (can_shift_up, can_shift_down) = {
        let sim = borrow_state(state)?;
        let rank_count = sim.world.guild_ranks.len() as i32;
        if sim.world.guild_name.is_some() && rank > 1 && rank <= rank_count {
            (rank > 2, rank < rank_count)
        } else {
            (false, false)
        }
    };
    state.push(Val::Bool(can_shift_up));
    state.push(Val::Bool(can_shift_down));
    Ok(2)
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

pub fn guild_control_save_rank(state: &mut LuaState) -> LuaResult<u32> {
    let name = val_to_string(state, stack_val(state, 1)).unwrap_or_default();
    let mut sim = borrow_state_mut(state)?;
    let index = sim.world.guild_selected_rank - 1;
    if index >= 0
        && let Some(rank) = sim.world.guild_ranks.get_mut(index as usize)
    {
        rank.name = name;
    }
    Ok(0)
}

pub fn guild_control_set_rank_flag(state: &mut LuaState) -> LuaResult<u32> {
    let flag_index = Option::<f64>::from_stack(state, 1)?
        .map(|n| n as i32)
        .unwrap_or(0);
    let checked = bool::from_stack(state, 2).unwrap_or(false);
    let mut sim = borrow_state_mut(state)?;
    let rank_index = sim.world.guild_selected_rank - 1;
    if flag_index > 0
        && rank_index >= 0
        && let Some(rank) = sim.world.guild_ranks.get_mut(rank_index as usize)
    {
        let slot = (flag_index - 1) as usize;
        if rank.flags.len() <= slot {
            rank.flags.resize(slot + 1, false);
        }
        rank.flags[slot] = checked;
    }
    Ok(0)
}

pub fn guild_control_add_rank(state: &mut LuaState) -> LuaResult<u32> {
    let name = val_to_string(state, stack_val(state, 1)).unwrap_or_else(|| "New Rank".to_string());
    let mut sim = borrow_state_mut(state)?;
    if sim.world.guild_name.is_some() && sim.world.guild_ranks.len() < 10 {
        sim.world
            .guild_ranks
            .push(crate::lua_api::state::GuildRank {
                name,
                flags: Vec::new(),
            });
    }
    Ok(0)
}

pub fn guild_control_del_rank(state: &mut LuaState) -> LuaResult<u32> {
    let rank_index = Option::<f64>::from_stack(state, 1)?
        .map(|n| n as i32)
        .unwrap_or(0);
    let mut sim = borrow_state_mut(state)?;
    if rank_index > 1 && (rank_index as usize) <= sim.world.guild_ranks.len() {
        sim.world.guild_ranks.remove((rank_index - 1) as usize);
        if sim.world.guild_selected_rank == rank_index {
            sim.world.guild_selected_rank = 0;
        }
    }
    Ok(0)
}

pub fn guild_control_shift_rank_up(state: &mut LuaState) -> LuaResult<u32> {
    let rank_index = Option::<f64>::from_stack(state, 1)?
        .map(|n| n as i32)
        .unwrap_or(0);
    shift_rank(state, rank_index, -1)
}

pub fn guild_control_shift_rank_down(state: &mut LuaState) -> LuaResult<u32> {
    let rank_index = Option::<f64>::from_stack(state, 1)?
        .map(|n| n as i32)
        .unwrap_or(0);
    shift_rank(state, rank_index, 1)
}

fn shift_rank(state: &mut LuaState, rank_index: i32, direction: i32) -> LuaResult<u32> {
    let mut sim = borrow_state_mut(state)?;
    let next_rank = rank_index + direction;
    if rank_index > 1
        && next_rank > 1
        && (rank_index as usize) <= sim.world.guild_ranks.len()
        && (next_rank as usize) <= sim.world.guild_ranks.len()
    {
        sim.world
            .guild_ranks
            .swap((rank_index - 1) as usize, (next_rank - 1) as usize);
        if sim.world.guild_selected_rank == rank_index {
            sim.world.guild_selected_rank = next_rank;
        } else if sim.world.guild_selected_rank == next_rank {
            sim.world.guild_selected_rank = rank_index;
        }
    }
    Ok(0)
}

fn ensure_c_guild_info_table(state: &mut LuaState) -> GcRef<Table> {
    let key = state.gc.intern_string_static(b"C_GuildInfo");
    let global = state.global;
    let existing = state
        .gc
        .tables
        .get(global)
        .map(|table| table.get_str(key, &state.gc.string_arena));
    if let Some(Val::Table(table_ref)) = existing {
        return table_ref;
    }

    let new_table = create_table(state);
    let Val::Table(table_ref) = new_table else {
        unreachable!("create_table must return a table");
    };
    if let Some(global_table) = state.gc.tables.get_mut(global) {
        let _ = global_table.raw_set(Val::Str(key), new_table, &state.gc.string_arena);
    }
    state.gc.barrier_back(global);
    table_ref
}

pub fn register_all(lua: &mut rilua::Lua) -> LuaResult<()> {
    use rilua::LuaApiMut;
    let state = lua.state_mut();
    let c_guild_info = ensure_c_guild_info_table(state);

    register_legacy_guild_control_globals(state)?;
    table_set_rust_fn_static(
        state,
        c_guild_info,
        "GuildControlGetRankFlags",
        guild_control_get_rank_flags,
    )?;
    Ok(())
}

fn register_legacy_guild_control_globals(state: &mut LuaState) -> LuaResult<()> {
    let g = state.global;
    for (name, function) in LEGACY_GUILD_CONTROL_GLOBALS {
        table_set_rust_fn_static(state, g, name, *function)?;
    }
    Ok(())
}

const LEGACY_GUILD_CONTROL_GLOBALS: &[(&str, RustFn)] = &[
    ("GuildControlSetRank", guild_control_set_rank),
    ("GuildControlGetRankName", guild_control_get_rank_name),
    ("GuildControlGetNumRanks", guild_control_get_num_ranks),
    ("GuildControlGetRankFlags", guild_control_get_rank_flags),
    ("GetNumMembersInRank", get_num_members_in_rank),
    (
        "GuildControlGetAllowedShifts",
        guild_control_get_allowed_shifts,
    ),
    ("GuildControlSaveRank", guild_control_save_rank),
    ("GuildControlSetRankFlag", guild_control_set_rank_flag),
    ("GuildControlAddRank", guild_control_add_rank),
    ("GuildControlDelRank", guild_control_del_rank),
    ("GuildControlShiftRankUp", guild_control_shift_rank_up),
    ("GuildControlShiftRankDown", guild_control_shift_rank_down),
];
