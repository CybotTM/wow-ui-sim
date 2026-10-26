//! Social restrictions supplied by this simulator environment, not account policy.

use crate::client_profile::{ACTIVE, ClientProfile};
use crate::lua_api::methods::borrow_state;
use crate::lua_bridge::table_set_rust_fn_static;
use rilua::vm::state::LuaState;
use rilua::{LuaResult, Val};

pub(super) fn register(state: &mut LuaState) -> LuaResult<()> {
    if !matches!(
        ACTIVE,
        ClientProfile::Retail | ClientProfile::Ptr | ClientProfile::Mists
    ) {
        return Ok(());
    }
    let namespace = super::ensure_namespace(state, "C_SocialRestrictions")?;
    table_set_rust_fn_static(state, namespace, "IsSquelched", is_squelched)
}

fn is_squelched(state: &mut LuaState) -> LuaResult<u32> {
    let squelched = borrow_state(state)?.social_restrictions.is_squelched;
    state.push(Val::Bool(squelched));
    Ok(1)
}
