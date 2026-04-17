//! Chat-channel moderation verbs that round-trip `SimState.chat_channels`.
//!
//! Migrates 9 entries off `GLOBAL_NIL_STUBS`:
//!
//! - `JoinChannelByName(name)`   — append new channel if missing.
//! - `JoinTemporaryChannel(name)` — alias of `JoinChannelByName`; retail
//!                                   auto-removes these on logout. Not
//!                                   modelled here.
//! - `ChannelLeave(name)`        — remove channel by name.
//! - `ChannelBan(name, player)`  — add player to channel's banned set,
//!                                   remove from members.
//! - `ChannelInvite(name, p)`    — add player to members (idempotent).
//! - `ChannelKick(name, p)`      — remove from members (no ban).
//! - `ChannelModerator(n, p)`    — add to moderators (player must be in
//!                                   channel first; silent no-op otherwise).
//! - `ChannelUnmoderator(n, p)`  — remove from moderators.
//! - `SwapChatChannelLinks(a, b)` — swap channels at positions a and b.
//!
//! Channel-number semantics mirror retail: slot 1 = channel #1.
//!
//! Registered from `register_tail_globals` after `missing_surface`.

use crate::lua_api::methods::borrow_state_mut;
use crate::lua_api::state::ChatChannel;
use crate::lua_bridge::{FromStack, stack_val};
use rilua::vm::state::LuaState;
use rilua::{LuaApiMut, LuaResult, Val};

fn required_string(state: &mut LuaState, index: i32) -> Option<String> {
    Option::<String>::from_stack(state, index)
        .ok()
        .flatten()
        .filter(|s| !s.is_empty())
}

fn stack_i32(state: &mut LuaState, index: i32) -> Option<i32> {
    match stack_val(state, index) {
        Val::Num(n) => Some(n as i32),
        _ => None,
    }
}

fn ensure_channel<'a>(
    channels: &'a mut Vec<ChatChannel>,
    name: &str,
) -> Option<&'a mut ChatChannel> {
    channels.iter_mut().find(|c| c.name == name)
}

fn channel_index(channels: &[ChatChannel], name: &str) -> Option<usize> {
    channels.iter().position(|c| c.name == name)
}

/// `JoinChannelByName(name)` — append a new channel when missing.
fn join_channel_by_name(state: &mut LuaState) -> LuaResult<u32> {
    let Some(name) = required_string(state, 1) else {
        return Ok(0);
    };
    let mut st = borrow_state_mut(state)?;
    if channel_index(&st.chat_channels, &name).is_none() {
        st.chat_channels.push(ChatChannel {
            name,
            ..ChatChannel::default()
        });
    }
    Ok(0)
}

/// `JoinTemporaryChannel(name)` — alias. Retail expires these on logout;
/// sim does not model session boundaries so the entry is permanent.
fn join_temporary_channel(state: &mut LuaState) -> LuaResult<u32> {
    join_channel_by_name(state)
}

/// `ChannelLeave(name)` — drop channel by name. Silent no-op when absent.
fn channel_leave(state: &mut LuaState) -> LuaResult<u32> {
    let Some(name) = required_string(state, 1) else {
        return Ok(0);
    };
    borrow_state_mut(state)?
        .chat_channels
        .retain(|c| c.name != name);
    Ok(0)
}

/// `ChannelBan(name, player)` — ban a player from a channel, also
/// evicting them if currently a member.
fn channel_ban(state: &mut LuaState) -> LuaResult<u32> {
    let (Some(channel), Some(player)) = (required_string(state, 1), required_string(state, 2))
    else {
        return Ok(0);
    };
    let mut st = borrow_state_mut(state)?;
    if let Some(ch) = ensure_channel(&mut st.chat_channels, &channel) {
        ch.banned.insert(player.clone());
        ch.members.remove(&player);
        ch.moderators.remove(&player);
    }
    Ok(0)
}

/// `ChannelInvite(name, player)` — add member; idempotent via BTreeSet.
fn channel_invite(state: &mut LuaState) -> LuaResult<u32> {
    let (Some(channel), Some(player)) = (required_string(state, 1), required_string(state, 2))
    else {
        return Ok(0);
    };
    let mut st = borrow_state_mut(state)?;
    if let Some(ch) = ensure_channel(&mut st.chat_channels, &channel)
        && !ch.banned.contains(&player)
    {
        ch.members.insert(player);
    }
    Ok(0)
}

/// `ChannelKick(name, player)` — remove from members (no ban record).
fn channel_kick(state: &mut LuaState) -> LuaResult<u32> {
    let (Some(channel), Some(player)) = (required_string(state, 1), required_string(state, 2))
    else {
        return Ok(0);
    };
    let mut st = borrow_state_mut(state)?;
    if let Some(ch) = ensure_channel(&mut st.chat_channels, &channel) {
        ch.members.remove(&player);
        ch.moderators.remove(&player);
    }
    Ok(0)
}

/// `ChannelModerator(name, player)` — grant moderator; player must be a
/// current member.
fn channel_moderator(state: &mut LuaState) -> LuaResult<u32> {
    let (Some(channel), Some(player)) = (required_string(state, 1), required_string(state, 2))
    else {
        return Ok(0);
    };
    let mut st = borrow_state_mut(state)?;
    if let Some(ch) = ensure_channel(&mut st.chat_channels, &channel)
        && ch.members.contains(&player)
    {
        ch.moderators.insert(player);
    }
    Ok(0)
}

/// `ChannelUnmoderator(name, player)` — revoke moderator.
fn channel_unmoderator(state: &mut LuaState) -> LuaResult<u32> {
    let (Some(channel), Some(player)) = (required_string(state, 1), required_string(state, 2))
    else {
        return Ok(0);
    };
    let mut st = borrow_state_mut(state)?;
    if let Some(ch) = ensure_channel(&mut st.chat_channels, &channel) {
        ch.moderators.remove(&player);
    }
    Ok(0)
}

/// `SwapChatChannelLinks(a, b)` — swap channels at positions a and b
/// (1-based). Silent no-op for out-of-range indices.
fn swap_chat_channel_links(state: &mut LuaState) -> LuaResult<u32> {
    let (Some(a), Some(b)) = (stack_i32(state, 1), stack_i32(state, 2)) else {
        return Ok(0);
    };
    let Some(a0) = a.checked_sub(1).and_then(|n| usize::try_from(n).ok()) else {
        return Ok(0);
    };
    let Some(b0) = b.checked_sub(1).and_then(|n| usize::try_from(n).ok()) else {
        return Ok(0);
    };
    let mut st = borrow_state_mut(state)?;
    let len = st.chat_channels.len();
    if a0 < len && b0 < len {
        st.chat_channels.swap(a0, b0);
    }
    Ok(0)
}

pub fn register_all(lua: &mut rilua::Lua) -> crate::Result<()> {
    LuaApiMut::register_function(lua, "JoinChannelByName", join_channel_by_name)?;
    LuaApiMut::register_function(lua, "JoinTemporaryChannel", join_temporary_channel)?;
    LuaApiMut::register_function(lua, "ChannelLeave", channel_leave)?;
    LuaApiMut::register_function(lua, "ChannelBan", channel_ban)?;
    LuaApiMut::register_function(lua, "ChannelInvite", channel_invite)?;
    LuaApiMut::register_function(lua, "ChannelKick", channel_kick)?;
    LuaApiMut::register_function(lua, "ChannelModerator", channel_moderator)?;
    LuaApiMut::register_function(lua, "ChannelUnmoderator", channel_unmoderator)?;
    LuaApiMut::register_function(lua, "SwapChatChannelLinks", swap_chat_channel_links)?;
    Ok(())
}
