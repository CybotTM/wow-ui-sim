use wow_ui_sim::lua_api::WowLuaEnv;

fn env() -> WowLuaEnv {
    WowLuaEnv::new().expect("Failed to create Lua environment")
}

#[test]
fn calendar_default_guild_filter_has_expected_shape() {
    let env = env();
    let result: String = env
        .eval(
            r#"
            local filter = C_Calendar.GetDefaultGuildFilter()
            if filter.minLevel ~= 1 then return "min" end
            if filter.maxLevel ~= GetMaxLevelForLatestExpansion() then return "max" end
            if filter.rank ~= 1 then return "rank" end
            return "ok"
            "#,
        )
        .expect("calendar default guild filter should be callable");

    assert_eq!(result, "ok");
}
