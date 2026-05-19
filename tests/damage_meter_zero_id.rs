use wow_ui_sim::lua_api::WowLuaEnv;

#[test]
fn damage_meter_zero_id_maps_to_seeded_session() {
    let env = WowLuaEnv::new().expect("Failed to create Lua environment");
    let result: String = env
        .eval(
            r#"
            local session = C_DamageMeter.GetCombatSessionFromID(
                0,
                Enum.DamageMeterType.DamageDone
            )
            if not session then
                return "missing_zero_id_session"
            end
            return tostring(session.durationSeconds)
            "#,
        )
        .unwrap();

    assert_eq!(result, "40");
}
