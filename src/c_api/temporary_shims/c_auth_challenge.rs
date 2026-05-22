//! Temporary `C_AuthChallenge` fallback surface.
//!
//! Authenticator challenge state is not modeled yet. These no-op methods keep
//! the AuthChallenge UI callable while reporting no successful challenge.

use crate::c_api::ensure_namespace;
use crate::lua_bridge::table_set_rust_fn_static;
use rilua::vm::state::LuaState;
use rilua::{LuaResult, Val};

pub(crate) fn register_c_auth_challenge_shims(state: &mut LuaState) -> LuaResult<()> {
    let ns = ensure_namespace(state, "C_AuthChallenge")?;
    table_set_rust_fn_static(state, ns, "SetFrame", auth_challenge_noop)?;
    table_set_rust_fn_static(state, ns, "Submit", auth_challenge_noop)?;
    table_set_rust_fn_static(state, ns, "Cancel", auth_challenge_noop)?;
    table_set_rust_fn_static(state, ns, "OnTabPressed", auth_challenge_noop)?;
    table_set_rust_fn_static(state, ns, "DidChallengeSucceed", did_challenge_succeed)
}

fn auth_challenge_noop(_state: &mut LuaState) -> LuaResult<u32> {
    Ok(0)
}

fn did_challenge_succeed(state: &mut LuaState) -> LuaResult<u32> {
    state.push(Val::Bool(false));
    Ok(1)
}

#[cfg(test)]
mod tests {
    use crate::lua_api::WowLuaEnv;

    #[test]
    fn auth_challenge_defaults_to_unsuccessful_noops() {
        let env = WowLuaEnv::new().expect("lua env should initialize");
        let succeeded: bool = env
            .eval(
                r#"
                C_AuthChallenge.SetFrame({})
                C_AuthChallenge.Submit()
                C_AuthChallenge.Cancel()
                C_AuthChallenge.OnTabPressed(nil, false)
                return C_AuthChallenge.DidChallengeSucceed()
                "#,
            )
            .expect("auth challenge defaults should be callable");

        assert!(!succeeded);
    }
}
