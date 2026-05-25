//! Temporary scripted-animation effect defaults.
//!
//! Scripted animation effect metadata is not modeled yet. Startup consumers
//! iterate the effect list, so expose an empty list until that data has backing
//! simulator state.

const SCRIPTED_ANIMATION_EFFECT_DEFAULTS_LUA: &str = r#"
C_ScriptedAnimations = C_ScriptedAnimations or __wow_namespace()
if rawget(C_ScriptedAnimations, "GetAllScriptedAnimationEffects") == nil then
    function C_ScriptedAnimations.GetAllScriptedAnimationEffects()
        return {}
    end
end
"#;

pub(crate) fn apply_bootstrap(lua: &mut rilua::Lua) -> crate::Result<()> {
    lua.exec(SCRIPTED_ANIMATION_EFFECT_DEFAULTS_LUA)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::lua_api::WowLuaEnv;

    #[test]
    fn installs_empty_effect_list_default() {
        let env = WowLuaEnv::new().expect("lua env should initialize");

        let count: i32 = env
            .eval("return #C_ScriptedAnimations.GetAllScriptedAnimationEffects()")
            .expect("scripted animation effects default should be callable");

        assert_eq!(count, 0);
    }

    #[test]
    fn preserves_existing_effect_list_function() {
        let env = WowLuaEnv::new().expect("lua env should initialize");
        env.exec(
            r#"
            function C_ScriptedAnimations.GetAllScriptedAnimationEffects()
                return { "modeled-effect" }
            end
            "#,
        )
        .expect("fixture should install existing function");

        super::apply_bootstrap(&mut env.rilua_mut()).expect("workaround should apply");

        let first_effect: String = env
            .eval("return C_ScriptedAnimations.GetAllScriptedAnimationEffects()[1]")
            .expect("existing scripted animation effects should remain callable");

        assert_eq!(first_effect, "modeled-effect");
    }
}
