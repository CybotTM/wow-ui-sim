//! C_PartyInfo temporary static fallbacks — invite and Torghast state are not
//! modeled.
//!
//! These methods expose the inert baseline used by addon startup code until
//! the simulator has pending-invite and Torghast/Jailer's Tower backing state.

use crate::c_api::ensure_namespace;
use crate::lua_bridge::{FromStack, table_set_rust_fn_static};
use rilua::vm::state::LuaState;
use rilua::{LuaResult, Val};

pub(crate) fn register_c_party_info_static_fallbacks(state: &mut LuaState) -> LuaResult<()> {
    let ns = ensure_namespace(state, "C_PartyInfo")?;
    table_set_rust_fn_static(state, ns, "AllowedToDoPartyConversion", return_false)?;
    table_set_rust_fn_static(state, ns, "IsPartyInJailersTower", return_false)?;
    table_set_rust_fn_static(state, ns, "IsPartyWalkIn", return_false)?;
    table_set_rust_fn_static(
        state,
        ns,
        "GetInviteConfirmationInfo",
        get_invite_confirmation_info,
    )?;
    Ok(())
}

fn return_false(state: &mut LuaState) -> LuaResult<u32> {
    state.push(Val::Bool(false));
    Ok(1)
}

fn get_invite_confirmation_info(state: &mut LuaState) -> LuaResult<u32> {
    let _guid = Option::<String>::from_stack(state, 1)?;
    Ok(0)
}

#[cfg(test)]
mod tests {
    use crate::lua_api::WowLuaEnv;

    #[test]
    fn party_info_static_fallbacks_return_inert_values() {
        let env = WowLuaEnv::new().expect("lua env should initialize");
        let (can_convert, in_jailers_tower, walk_in, invite_results): (bool, bool, bool, i32) = env
            .eval(
                r##"
                return C_PartyInfo.AllowedToDoPartyConversion(),
                    C_PartyInfo.IsPartyInJailersTower(),
                    C_PartyInfo.IsPartyWalkIn(),
                    select("#", C_PartyInfo.GetInviteConfirmationInfo("Player-1234-ABCDEF"))
                "##,
            )
            .expect("party static fallbacks should be callable");

        assert!(!can_convert);
        assert!(!in_jailers_tower);
        assert!(!walk_in);
        assert_eq!(invite_results, 0);
    }
}
