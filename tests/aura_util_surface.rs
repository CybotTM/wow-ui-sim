use wow_ui_sim::lua_api::WowLuaEnv;

#[test]
fn aura_util_exposes_filter_table_and_filter_joiner() {
    let env = WowLuaEnv::new().expect("lua env should initialize");

    let result: (String, String, String) = env
        .eval(
            r#"
            return AuraUtil.AuraFilters.Helpful,
                   AuraUtil.AuraFilters.IncludeNameplateOnly,
                   AuraUtil.CreateFilterString(
                       AuraUtil.AuraFilters.Helpful,
                       "",
                       nil,
                       AuraUtil.AuraFilters.Raid
                   )
            "#,
        )
        .expect("AuraUtil filter surface should be native");

    assert_eq!(
        result,
        (
            "HELPFUL".to_string(),
            "INCLUDE_NAME_PLATE_ONLY".to_string(),
            "HELPFUL|RAID".to_string(),
        )
    );
}
