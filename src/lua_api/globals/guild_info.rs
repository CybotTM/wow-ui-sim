//! `C_GuildInfo.GetClubId` / `IsGuildOfficer` / `CanSpeakInGuildChat`.
//!
//! Backed by `SimState::world`:
//!
//! - `GetClubId()` — returns `world.guild_club_id` string, or nil.
//! - `IsGuildOfficer()` — `world.guild_is_officer` (default false).
//! - `CanSpeakInGuildChat()` — `world.guild_can_speak_in_chat` (default true;
//!   retail's "no explicit mute" baseline keeps addons' chat input enabled).
//!
//! Admin:
//! - `A_Admin.SetGuildClubId(id?)` — nil / empty clears.
//! - `A_Admin.SetGuildIsOfficer(b?)` / `A_Admin.SetGuildCanSpeakInChat(b?)` —
//!   no-arg defaults to true.

use crate::lua_api::methods::{borrow_state, borrow_state_mut, create_string, create_table};
use crate::lua_bridge::{FromStack, table_set_rust_fn};
use rilua::vm::gc::arena::GcRef;
use rilua::vm::state::LuaState;
use rilua::vm::table::Table;
use rilua::{LuaResult, Val};

pub fn get_club_id(state: &mut LuaState) -> LuaResult<u32> {
    let club_id = borrow_state(state)?.world.guild_club_id.clone();
    match club_id {
        Some(id) => {
            let val = create_string(state, &id);
            state.push(val);
        }
        None => state.push(Val::Nil),
    }
    Ok(1)
}

pub fn is_guild_officer(state: &mut LuaState) -> LuaResult<u32> {
    let v = borrow_state(state)?.world.guild_is_officer;
    state.push(Val::Bool(v));
    Ok(1)
}

pub fn can_speak_in_guild_chat(state: &mut LuaState) -> LuaResult<u32> {
    let v = borrow_state(state)?.world.guild_can_speak_in_chat;
    state.push(Val::Bool(v));
    Ok(1)
}

fn ensure_c_guild_info_table(state: &mut LuaState) -> GcRef<Table> {
    let key = state.gc.intern_string_static(b"C_GuildInfo");
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
    let table_ref = ensure_c_guild_info_table(state);
    table_set_rust_fn(state, table_ref, "GetClubId", get_club_id)?;
    table_set_rust_fn(state, table_ref, "IsGuildOfficer", is_guild_officer)?;
    table_set_rust_fn(
        state,
        table_ref,
        "CanSpeakInGuildChat",
        can_speak_in_guild_chat,
    )?;
    Ok(())
}

/// `A_Admin.SetGuildClubId(id?)` — pass nil or empty string to clear.
pub fn admin_set_guild_club_id(state: &mut LuaState) -> LuaResult<u32> {
    let id = Option::<String>::from_stack(state, 1)?.filter(|s| !s.is_empty());
    borrow_state_mut(state)?.world.guild_club_id = id;
    Ok(0)
}

/// `A_Admin.SetGuildIsOfficer(b?)` — no-arg defaults to true.
pub fn admin_set_guild_is_officer(state: &mut LuaState) -> LuaResult<u32> {
    let v = Option::<bool>::from_stack(state, 1)?.unwrap_or(true);
    borrow_state_mut(state)?.world.guild_is_officer = v;
    Ok(0)
}

/// `A_Admin.SetGuildCanSpeakInChat(b?)` — no-arg defaults to true.
pub fn admin_set_guild_can_speak_in_chat(state: &mut LuaState) -> LuaResult<u32> {
    let v = Option::<bool>::from_stack(state, 1)?.unwrap_or(true);
    borrow_state_mut(state)?.world.guild_can_speak_in_chat = v;
    Ok(0)
}
