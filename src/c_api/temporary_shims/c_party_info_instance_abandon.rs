//! C_PartyInfo temporary instance-abandon shims — vote state is not modeled.
//!
//! The retail instance-abandon flow tracks an active vote, per-player
//! responses, and optional shutdown timers. The simulator has no backing vote
//! state yet, so these methods expose the inert "no vote in progress" shape.

use crate::c_api::ensure_namespace;
use crate::lua_bridge::{FromStack, table_set_rust_fn_static};
use rilua::vm::closure::RustFn;
use rilua::vm::state::LuaState;
use rilua::{LuaResult, Val};

const INSTANCE_ABANDON_METHODS: &[(&str, RustFn)] = &[
    ("GetInstanceAbandonVoteTime", get_instance_abandon_vote_time),
    (
        "GetInstanceAbandonShutdownTime",
        get_instance_abandon_shutdown_time,
    ),
    (
        "GetInstanceAbandonVoteResponse",
        get_instance_abandon_vote_response,
    ),
    (
        "SetInstanceAbandonVoteResponse",
        set_instance_abandon_vote_response,
    ),
    (
        "GetNumInstanceAbandonGroupVoteResponses",
        get_num_instance_abandon_group_vote_responses,
    ),
    (
        "CanStartInstanceAbandonVote",
        can_start_instance_abandon_vote,
    ),
    ("StartInstanceAbandonVote", start_instance_abandon_vote),
];

pub(crate) fn register_c_party_info_instance_abandon(state: &mut LuaState) -> LuaResult<()> {
    let ns = ensure_namespace(state, "C_PartyInfo")?;
    for &(name, handler) in INSTANCE_ABANDON_METHODS {
        table_set_rust_fn_static(state, ns, name, handler)?;
    }
    Ok(())
}

fn get_instance_abandon_vote_time(state: &mut LuaState) -> LuaResult<u32> {
    state.push(Val::Num(0.0));
    state.push(Val::Num(0.0));
    Ok(2)
}

fn get_instance_abandon_shutdown_time(state: &mut LuaState) -> LuaResult<u32> {
    state.push(Val::Num(0.0));
    state.push(Val::Num(0.0));
    Ok(2)
}

fn get_instance_abandon_vote_response(state: &mut LuaState) -> LuaResult<u32> {
    state.push(Val::Nil);
    Ok(1)
}

fn set_instance_abandon_vote_response(state: &mut LuaState) -> LuaResult<u32> {
    let _response = Option::<bool>::from_stack(state, 1)?;
    Ok(0)
}

fn get_num_instance_abandon_group_vote_responses(state: &mut LuaState) -> LuaResult<u32> {
    state.push(Val::Num(0.0));
    Ok(1)
}

fn can_start_instance_abandon_vote(state: &mut LuaState) -> LuaResult<u32> {
    state.push(Val::Bool(false));
    Ok(1)
}

fn start_instance_abandon_vote(_state: &mut LuaState) -> LuaResult<u32> {
    Ok(0)
}
