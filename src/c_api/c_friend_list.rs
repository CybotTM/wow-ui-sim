//! Exact current-profile friend-name query over environment-owned social state.
//! Indexed and earlier name queries share the existing simulator projection.

use crate::lua_api::methods::{create_string, create_table, table_set_static};
use crate::lua_api::state::SocialFriend;
use rilua::Val;
use rilua::vm::state::LuaState;

#[cfg(any(feature = "client-retail", feature = "client-ptr"))]
pub(crate) mod current {
    use super::build_friend_info_table;
    use crate::c_api::helpers::{ensure_namespace, mark_namespace_keys_removed};
    use crate::lua_api::methods::{borrow_state, val_to_string};
    use crate::lua_bridge::{stack_val, table_set_rust_fn_static};
    use rilua::vm::state::LuaState;
    use rilua::{LuaResult, Val};

    pub(crate) fn register(state: &mut LuaState) -> LuaResult<()> {
        let namespace = ensure_namespace(state, "C_FriendList")?;
        mark_namespace_keys_removed(state, namespace, &["GetFriendInfoByName"]);
        table_set_rust_fn_static(state, namespace, "GetFriendInfo", get_friend_info)
    }

    fn get_friend_info(state: &mut LuaState) -> LuaResult<u32> {
        let argument = rilua::table_security::unwrap_secret(state, stack_val(state, 1))?;
        let name = val_to_string(state, argument).ok_or_else(|| {
            rilua::runtime_error("C_FriendList.GetFriendInfo: argument 1 must be a UTF-8 string")
        })?;
        let friend = {
            let sim = borrow_state(state)?;
            sim.social_friends
                .iter()
                .find(|friend| friend.name == name)
                .cloned()
        };
        let info = match friend {
            Some(friend) => build_friend_info_table(state, &friend),
            None => Val::Nil,
        };
        state.push(info);
        Ok(1)
    }
}

// Existing simulator compatibility projection, not a native DTO claim.
// Synthetic race, gender and guild constants retain their previous semantics.
pub(crate) fn build_friend_info_table(state: &mut LuaState, friend: &SocialFriend) -> Val {
    let table = create_table(state);
    let name = create_string(state, &friend.name);
    let full_name = create_string(state, &friend.name);
    let area = create_string(state, &friend.area);
    let class_name = create_string(state, &friend.class_name);
    let class_str = create_string(state, &friend.class_name);
    let filename = create_string(state, &friend.class_name.to_uppercase());
    let notes = create_string(state, &friend.note);
    let guid = create_string(state, &friend.guid);
    let race_str = create_string(state, "Human");
    let full_guild_name = create_string(state, "Heroes of Azeroth");
    table_set_static(state, table, "name", name);
    table_set_static(state, table, "fullName", full_name);
    table_set_static(state, table, "level", Val::Num(friend.level as f64));
    table_set_static(state, table, "area", area);
    table_set_static(state, table, "className", class_name);
    table_set_static(state, table, "classStr", class_str);
    table_set_static(state, table, "filename", filename);
    table_set_static(state, table, "notes", notes);
    table_set_static(state, table, "connected", Val::Bool(friend.is_online));
    table_set_static(state, table, "guid", guid);
    table_set_static(state, table, "raceStr", race_str);
    table_set_static(state, table, "gender", Val::Num(2.0));
    table_set_static(state, table, "fullGuildName", full_guild_name);
    table
}
