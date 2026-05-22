//! Temporary `C_ClassTrial` fallback surface.
//!
//! Class trial account/character state is not modeled yet. These methods keep
//! Blizzard startup code on the non-trial path until that backing model exists.

use crate::c_api::ensure_namespace;
use crate::lua_bridge::table_set_rust_fn_static;
use rilua::vm::state::LuaState;
use rilua::{LuaResult, Val};

pub(crate) fn register_c_class_trial_shims(state: &mut LuaState) -> LuaResult<()> {
    let ns = ensure_namespace(state, "C_ClassTrial")?;
    table_set_rust_fn_static(state, ns, "IsClassTrialCharacter", is_class_trial_character)?;
    table_set_rust_fn_static(
        state,
        ns,
        "GetClassTrialLogoutTimeSeconds",
        get_class_trial_logout_time_seconds,
    )
}

fn is_class_trial_character(state: &mut LuaState) -> LuaResult<u32> {
    state.push(Val::Bool(false));
    Ok(1)
}

fn get_class_trial_logout_time_seconds(state: &mut LuaState) -> LuaResult<u32> {
    state.push(Val::Num(0.0));
    Ok(1)
}

#[cfg(test)]
mod tests {
    use crate::lua_api::WowLuaEnv;

    #[test]
    fn class_trial_defaults_to_regular_character() {
        let env = WowLuaEnv::new().expect("lua env should initialize");
        let (is_trial, logout_time): (bool, i32) = env
            .eval("return C_ClassTrial.IsClassTrialCharacter(), C_ClassTrial.GetClassTrialLogoutTimeSeconds()")
            .expect("class trial defaults should be queryable");

        assert!(!is_trial);
        assert_eq!(logout_time, 0);
    }
}
