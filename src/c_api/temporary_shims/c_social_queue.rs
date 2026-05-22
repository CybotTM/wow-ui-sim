//! Temporary `C_SocialQueue` fallback surface.
//!
//! Quick Join/social queue state is not modeled yet. These methods expose the
//! empty-result shapes Blizzard QuickJoin expects while no groups or queues are
//! available.

use crate::c_api::ensure_namespace;
use crate::lua_api::methods::create_table;
use crate::lua_bridge::table_set_rust_fn_static;
use rilua::LuaResult;
use rilua::vm::state::LuaState;

pub(crate) fn register_c_social_queue_shims(state: &mut LuaState) -> LuaResult<()> {
    let ns = ensure_namespace(state, "C_SocialQueue")?;
    table_set_rust_fn_static(state, ns, "GetAllGroups", social_queue_empty_table)?;
    table_set_rust_fn_static(state, ns, "GetConfig", social_queue_empty_table)?;
    table_set_rust_fn_static(state, ns, "GetGroupForPlayer", social_queue_no_result)?;
    table_set_rust_fn_static(state, ns, "GetGroupInfo", social_queue_no_result)?;
    table_set_rust_fn_static(state, ns, "GetGroupMembers", social_queue_empty_table)?;
    table_set_rust_fn_static(state, ns, "GetGroupQueues", social_queue_empty_table)?;
    table_set_rust_fn_static(state, ns, "RequestToJoin", social_queue_no_result)?;
    table_set_rust_fn_static(state, ns, "SignalToastDisplayed", social_queue_no_result)
}

fn social_queue_empty_table(state: &mut LuaState) -> LuaResult<u32> {
    let table = create_table(state);
    state.push(table);
    Ok(1)
}

fn social_queue_no_result(_state: &mut LuaState) -> LuaResult<u32> {
    Ok(0)
}

#[cfg(test)]
mod tests {
    use crate::lua_api::WowLuaEnv;

    #[test]
    fn social_queue_defaults_to_empty_groups_and_queues() {
        let env = WowLuaEnv::new().expect("lua env should initialize");
        let (groups, config, members, queues): (i32, i32, i32, i32) = env
            .eval(
                r#"
                C_SocialQueue.RequestToJoin(1)
                C_SocialQueue.SignalToastDisplayed(1)
                return #C_SocialQueue.GetAllGroups(),
                    #C_SocialQueue.GetConfig(),
                    #C_SocialQueue.GetGroupMembers(1),
                    #C_SocialQueue.GetGroupQueues(1)
                "#,
            )
            .expect("social queue defaults should be queryable");

        assert_eq!(groups, 0);
        assert_eq!(config, 0);
        assert_eq!(members, 0);
        assert_eq!(queues, 0);
    }

    #[test]
    fn social_queue_missing_groups_return_nil() {
        let env = WowLuaEnv::new().expect("lua env should initialize");
        let missing: bool = env
            .eval(
                "return C_SocialQueue.GetGroupForPlayer('Player') == nil and C_SocialQueue.GetGroupInfo(1) == nil",
            )
            .expect("missing social queue groups should return nil");

        assert!(missing);
    }
}
