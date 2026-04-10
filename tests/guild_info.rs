use wow_ui_sim::lua_api::WowLuaEnv;

const GUILD_INFO_SCRIPT: &str = r#"
    local motd = C_GuildInfo.GetMOTD()
    if motd ~= "Raid invites tonight at 20:00 server. Repairs are on for progression." then
        return "wrong_motd:" .. tostring(motd)
    end

    local infoText = C_GuildInfo.GetInfoText()
    if infoText ~= "Mythic-focused guild recruiting healers and a warlock for weekend raids." then
        return "wrong_initial_info:" .. tostring(infoText)
    end

    C_GuildInfo.SetInfoText("Casual alt run on Sunday; sign up in Discord.")
    local updatedInfoText = C_GuildInfo.GetInfoText()
    if updatedInfoText ~= "Casual alt run on Sunday; sign up in Discord." then
        return "wrong_updated_info:" .. tostring(updatedInfoText)
    end

    if C_GuildInfo.GetMOTD() ~= motd then
        return "motd_changed_after_info_update"
    end

    return "ok"
"#;

fn env() -> WowLuaEnv {
    WowLuaEnv::new().expect("Failed to create Lua environment")
}

#[test]
fn guild_info_text_methods_use_seeded_runtime_state() {
    let env = env();
    let result: String = env
        .eval(GUILD_INFO_SCRIPT)
        .expect("C_GuildInfo guild text methods should be queryable");
    assert_eq!(result, "ok");
}
