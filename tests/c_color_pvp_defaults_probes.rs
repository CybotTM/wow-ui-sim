use wow_ui_sim::lua_api::WowLuaEnv;

fn env() -> WowLuaEnv {
    WowLuaEnv::new().expect("Failed to create Lua environment")
}

#[test]
fn color_override_quality_defaults_to_white_color() {
    let env = env();
    let (r, g, b, a): (f64, f64, f64, f64) = env
        .eval(
            r#"
            local color = C_ColorOverrides.GetColorForQuality(1)
            return color.r, color.g, color.b, color.a
            "#,
        )
        .expect("quality color should be queryable");

    assert_eq!((r, g, b, a), (1.0, 1.0, 1.0, 1.0));
}

#[test]
fn pvp_no_state_defaults_do_not_override_state_backed_pvp_methods() {
    let env = env();
    let result: String = env
        .eval(
            r#"
            if C_PvP.IsInBrawl() ~= false then return "brawl" end
            if C_PvP.IsSoloShuffle() ~= false then return "solo-shuffle" end
            local spellID, startTime, duration = C_PvP.GetArenaCrowdControlInfo("player")
            if spellID ~= nil or startTime ~= 0 or duration ~= 0 then return "cc" end
            if C_PvP.GetLocklistMap(1) ~= 0 then return "locklist" end
            C_PvP.SetLocklistMap(566)
            if C_PvP.GetLocklistMap(1) ~= 566 then return "state-backed" end
            return "ok"
            "#,
        )
        .expect("PvP defaults and state-backed methods should be callable");

    assert_eq!(result, "ok");
}
