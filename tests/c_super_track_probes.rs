use wow_ui_sim::lua_api::WowLuaEnv;

fn env() -> WowLuaEnv {
    WowLuaEnv::new().expect("Failed to create Lua environment")
}

#[test]
fn super_track_defaults_match_no_active_target_state() {
    let env = env();
    let result: String = env
        .eval(
            r#"
            if C_SuperTrack.GetSuperTrackedQuestID() ~= 0 then return "quest" end
            if C_SuperTrack.GetHighestPrioritySuperTrackingType() ~= nil then return "type" end
            if C_SuperTrack.GetSuperTrackedMapPin() ~= nil then return "map-pin" end
            C_SuperTrack.SetSuperTrackedQuestID(42)
            C_SuperTrack.ClearAllSuperTracked()
            C_SuperTrack.ClearSuperTrackedContent()
            C_SuperTrack.ClearSuperTrackedMapPin()
            return "ok"
            "#,
        )
        .expect("C_SuperTrack temporary defaults should be callable");

    assert_eq!(result, "ok");
}
