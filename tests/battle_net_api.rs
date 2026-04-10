use wow_ui_sim::lua_api::WowLuaEnv;

fn env() -> WowLuaEnv {
    WowLuaEnv::new().expect("Failed to create Lua environment")
}

#[test]
fn battle_net_high_res_texture_state_tracks_cvar_and_install_action() {
    let env = env();
    let result: String = env
        .eval(
            r#"
            if not C_BattleNet.AreHighResTexturesInstalled() then
                return "expected_default_high_res_textures_installed"
            end

            SetCVar("useHighResTextures", "0")
            if C_BattleNet.AreHighResTexturesInstalled() then
                return "cvar_zero_should_disable_high_res_textures"
            end

            C_BattleNet.InstallHighResTextures()
            if not C_BattleNet.AreHighResTexturesInstalled() then
                return "install_should_enable_high_res_textures"
            end

            if GetCVar("useHighResTextures") ~= "1" then
                return "install_should_set_high_res_texture_cvar"
            end

            return "ok"
            "#,
        )
        .unwrap();

    assert_eq!(
        result, "ok",
        "C_BattleNet high-res texture APIs should follow the runtime CVar state"
    );
}
