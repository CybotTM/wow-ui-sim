//! Temporary class-trial defaults.
//!
//! Class-trial account and character state is not modeled yet. Keep startup on
//! the regular-character path until that backing state exists.

const CLASS_TRIAL_DEFAULTS_LUA: &str = r#"
C_ClassTrial = C_ClassTrial or __wow_namespace()
if rawget(C_ClassTrial, "IsClassTrialCharacter") == nil then
    function C_ClassTrial.IsClassTrialCharacter()
        return false
    end
end
if rawget(C_ClassTrial, "GetClassTrialLogoutTimeSeconds") == nil then
    function C_ClassTrial.GetClassTrialLogoutTimeSeconds()
        return 0
    end
end
"#;

pub(crate) fn apply_bootstrap(lua: &mut rilua::Lua) -> crate::Result<()> {
    lua.exec(CLASS_TRIAL_DEFAULTS_LUA)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::lua_api::WowLuaEnv;

    #[test]
    fn class_trial_defaults_to_regular_character() {
        let env = WowLuaEnv::new().expect("lua env should initialize");
        let (is_trial, logout_time): (bool, i32) = env
            .eval(
                "return C_ClassTrial.IsClassTrialCharacter(), C_ClassTrial.GetClassTrialLogoutTimeSeconds()",
            )
            .expect("class trial defaults should be queryable");

        assert!(!is_trial);
        assert_eq!(logout_time, 0);
    }

    #[test]
    fn preserves_existing_class_trial_functions() {
        let env = WowLuaEnv::new().expect("lua env should initialize");
        env.exec(
            r#"
            function C_ClassTrial.IsClassTrialCharacter()
                return true
            end
            function C_ClassTrial.GetClassTrialLogoutTimeSeconds()
                return 30
            end
            "#,
        )
        .expect("fixture should install existing class-trial functions");

        super::apply_bootstrap(&mut env.rilua_mut()).expect("workaround should apply");

        let (is_trial, logout_time): (bool, i32) = env
            .eval(
                "return C_ClassTrial.IsClassTrialCharacter(), C_ClassTrial.GetClassTrialLogoutTimeSeconds()",
            )
            .expect("existing class-trial functions should remain callable");

        assert!(is_trial);
        assert_eq!(logout_time, 30);
    }
}
