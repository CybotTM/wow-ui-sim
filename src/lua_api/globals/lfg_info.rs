//! `C_LFGInfo.CanPlayerUsePremadeGroup` — SimState-backed.
//!
//! The namespace-stubs pass previously registered this as a `stub_false`
//! blanket entry. Moving it to its own backing flag lets tests exercise UI
//! paths that gate on premade-group availability (the Premade Group Finder
//! button in the LFG frame, for instance).
//!
//! Admin: `A_Admin.SetCanUsePremadeGroup(b?)` — no-arg defaults to true.

use crate::lua_api::methods::{borrow_state, borrow_state_mut, create_table};
use crate::lua_bridge::{FromStack, stack_val, table_set_rust_fn_static};
use rilua::vm::gc::arena::GcRef;
use rilua::vm::state::LuaState;
use rilua::vm::table::Table;
use rilua::{LuaResult, Val};

/// `C_LFGInfo.IsLFGFollowerDungeon(dungeonID)` → bool.
fn is_lfg_follower_dungeon(state: &mut LuaState) -> LuaResult<u32> {
    let dungeon_id = match stack_val(state, 1) {
        rilua::Val::Num(n) => n as i32,
        _ => {
            state.push(rilua::Val::Bool(false));
            return Ok(1);
        }
    };
    let is_follower = borrow_state(state)?
        .lfd_dungeons
        .iter()
        .find(|d| d.dungeon_id == dungeon_id)
        .map(|d| d.is_follower_dungeon)
        .unwrap_or(false);
    state.push(rilua::Val::Bool(is_follower));
    Ok(1)
}

pub fn can_player_use_premade_group(state: &mut LuaState) -> LuaResult<u32> {
    let v = borrow_state(state)?.can_use_premade_group;
    state.push(Val::Bool(v));
    Ok(1)
}

fn ensure_c_lfg_info_table(state: &mut LuaState) -> GcRef<Table> {
    let key = state.gc.intern_string_static(b"C_LFGInfo");
    let global = state.global;
    let existing = state
        .gc
        .tables
        .get(global)
        .map(|t| t.get_str(key, &state.gc.string_arena));
    if let Some(Val::Table(r)) = existing {
        return r;
    }
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

pub fn register_all(lua: &mut rilua::Lua) -> LuaResult<()> {
    use rilua::LuaApiMut;
    let state = lua.state_mut();
    let table_ref = ensure_c_lfg_info_table(state);
    table_set_rust_fn_static(
        state,
        table_ref,
        "CanPlayerUsePremadeGroup",
        can_player_use_premade_group,
    )?;
    table_set_rust_fn_static(
        state,
        table_ref,
        "IsLFGFollowerDungeon",
        is_lfg_follower_dungeon,
    )?;
    Ok(())
}

/// `A_Admin.SetCanUsePremadeGroup(b?)` — no-arg defaults to true.
pub fn admin_set_can_use_premade_group(state: &mut LuaState) -> LuaResult<u32> {
    let v = Option::<bool>::from_stack(state, 1)?.unwrap_or(true);
    borrow_state_mut(state)?.can_use_premade_group = v;
    Ok(0)
}
