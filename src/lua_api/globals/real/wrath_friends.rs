//! Wrath legacy tuples and GUID selection over the shared friend model.

use crate::c_api::c_friend_list::{friends_in_display_order, show_friends};
use crate::lua_api::methods::{borrow_state, borrow_state_mut, create_string, val_to_string};
use crate::lua_api::state::SocialFriend;
use crate::lua_bridge::stack_val;
use rilua::vm::state::LuaState;
use rilua::{LuaApiMut, LuaResult, Val, runtime_error};

const FRIEND_SLOT_COUNT: usize = 100;

pub(crate) fn register_all(lua: &mut rilua::Lua) -> crate::Result<()> {
    LuaApiMut::register_function(lua, "ShowFriends", show_friends)?;
    LuaApiMut::register_function(lua, "GetFriendInfo", get_friend_info)?;
    LuaApiMut::register_function(lua, "GetSelectedFriend", get_selected_friend)?;
    LuaApiMut::register_function(lua, "SetSelectedFriend", set_selected_friend)?;
    // Use the VM's Lua 5.1 numeric-string conversion, not Rust string parsing.
    lua.exec(
        r#"
        local number, info, selectFriend = tonumber, GetFriendInfo, SetSelectedFriend
        function GetFriendInfo(selector)
            return info(number(selector) or selector)
        end
        function SetSelectedFriend(index)
            return selectFriend(number(index))
        end
        "#,
    )?;
    Ok(())
}

fn friend_slot(number: f64) -> Option<usize> {
    // The reconstructed 101st slot is an unsafe native boundary, not a contract.
    let slot = (number as u32).checked_sub(1)? as usize;
    (slot < FRIEND_SLOT_COUNT).then_some(slot)
}

fn find_friend(state: &mut LuaState) -> LuaResult<Option<SocialFriend>> {
    let selector = stack_val(state, 1);
    let sim = borrow_state(state)?;
    match selector {
        Val::Num(number) => Ok(friend_slot(number).and_then(|slot| {
            friends_in_display_order(&sim.social_friends)
                .get(slot)
                .cloned()
        })),
        Val::Str(_) => {
            let name = val_to_string(state, selector)
                .ok_or_else(|| runtime_error("Usage: GetFriendInfo(index or name)"))?;
            Ok(sim
                .social_friends
                .iter()
                .find(|friend| friend.name.eq_ignore_ascii_case(&name))
                .cloned())
        }
        _ => Err(runtime_error("Usage: GetFriendInfo(index or name)")),
    }
}

fn get_friend_info(state: &mut LuaState) -> LuaResult<u32> {
    let Some(friend) = find_friend(state)? else {
        return push_missing_friend(state);
    };
    let linked = borrow_state(state)?
        .refer_a_friend_linked_guids
        .contains(&friend.guid);
    let name = create_string(state, &friend.name);
    let class = create_string(state, &friend.class_name);
    let area = create_string(state, &friend.area);
    let note = if friend.note.is_empty() {
        Val::Nil
    } else {
        create_string(state, &friend.note)
    };
    let status = create_string(state, "");
    for value in [
        name,
        Val::Num(f64::from(friend.level)),
        class,
        area,
        native_flag(friend.is_online),
        status,
        note,
        native_flag(linked),
    ] {
        state.push(value);
    }
    Ok(8)
}

fn native_flag(enabled: bool) -> Val {
    if enabled { Val::Num(1.0) } else { Val::Nil }
}

fn push_missing_friend(state: &mut LuaState) -> LuaResult<u32> {
    let unknown = LuaApiMut::get_global_val(state, "UNKNOWN");
    let status = create_string(state, "");
    for value in [
        Val::Nil,
        Val::Num(0.0),
        unknown,
        unknown,
        Val::Nil,
        status,
        Val::Nil,
        Val::Nil,
    ] {
        state.push(value);
    }
    Ok(8)
}

fn get_selected_friend(state: &mut LuaState) -> LuaResult<u32> {
    let index = {
        let sim = borrow_state(state)?;
        sim.selected_friend_guid
            .as_ref()
            .and_then(|guid| {
                friends_in_display_order(&sim.social_friends)
                    .iter()
                    .take(FRIEND_SLOT_COUNT)
                    .position(|friend| &friend.guid == guid)
            })
            .map_or(0, |slot| slot + 1)
    };
    state.push(Val::Num(index as f64));
    Ok(1)
}

fn set_selected_friend(state: &mut LuaState) -> LuaResult<u32> {
    let Val::Num(number) = stack_val(state, 1) else {
        return Err(runtime_error("Usage: SetSelectedFriend(index)"));
    };
    let mut sim = borrow_state_mut(state)?;
    sim.selected_friend_guid = friend_slot(number).and_then(|slot| {
        friends_in_display_order(&sim.social_friends)
            .get(slot)
            .filter(|friend| !friend.guid.is_empty())
            .map(|friend| friend.guid.clone())
    });
    Ok(0)
}
