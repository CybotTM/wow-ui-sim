use wow_ui_sim::lua_api::WowLuaEnv;

#[test]
fn legacy_specialization_globals_are_not_c_api_surface() {
    let c_spec = include_str!("../src/c_api/c_spec.rs");
    let utility_registration = include_str!("../src/lua_api/globals/utility_system_spell/mod.rs");

    assert!(
        !c_spec.contains("GetNumSpecGroups")
            && !c_spec.contains("GetSpecializationInfoByID")
            && !c_spec.contains("GetSpecializationRole"),
        "legacy specialization globals should not live in c_api::c_spec"
    );
    assert!(
        utility_registration
            .contains("real::specialization_legacy::register_legacy_specialization_globals"),
        "legacy specialization globals should be registered from the Lua globals layer"
    );
}

#[test]
fn legacy_specialization_globals_remain_registered() {
    let env = WowLuaEnv::new().expect("failed to create Lua environment");
    let (num_groups, num_specs_type, lfg_role): (i32, String, String) = env
        .eval(
            r#"
            return GetNumSpecGroups(),
                   type(GetNumSpecializations),
                   GetLFGStringFromEnum(2)
            "#,
        )
        .expect("legacy specialization globals should run");

    assert_eq!(num_groups, 1);
    assert_eq!(num_specs_type, "function");
    assert_eq!(lfg_role, "DAMAGER");
}
