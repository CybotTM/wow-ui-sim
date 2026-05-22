use wow_ui_sim::lua_api::WowLuaEnv;

fn env() -> WowLuaEnv {
    WowLuaEnv::new().expect("Failed to create Lua environment")
}

#[test]
fn scripted_animation_effects_default_empty() {
    let env = env();
    let (namespace_type, method_type, count): (String, String, i32) = env
        .eval(
            r#"
            local effects = C_ScriptedAnimations.GetAllScriptedAnimationEffects()
            return type(C_ScriptedAnimations),
                type(C_ScriptedAnimations.GetAllScriptedAnimationEffects),
                #effects
            "#,
        )
        .expect("scripted animation effect list should be queryable");

    assert_eq!(namespace_type, "table");
    assert_eq!(method_type, "function");
    assert_eq!(count, 0);
}
