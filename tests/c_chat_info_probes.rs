use wow_ui_sim::lua_api::WowLuaEnv;

fn env() -> WowLuaEnv {
    WowLuaEnv::new().expect("Failed to create Lua environment")
}

#[test]
fn chat_info_temporary_defaults_are_available() {
    let env = env();
    let result: String = env
        .eval(
            r#"
            if C_ChatInfo.PerformEmote("wave") ~= false then return "perform" end
            if C_ChatInfo.IsValidChatLine(1) ~= false then return "valid" end
            if C_ChatInfo.ReplaceIconAndGroupExpressions("hello") ~= "hello" then return "replace" end
            if C_ChatInfo.AreOutgoingAddonChatMessagesRestricted() ~= false then return "restricted" end
            if C_ChatInfo.GetNumReservedChatWindows() ~= 0 then return "reserved" end
            if C_ChatInfo.GetChannelRulesetForChannelID(1) ~= 0 then return "ruleset-id" end
            if C_ChatInfo.GetChannelRuleset("General") ~= 0 then return "ruleset" end
            if C_ChatInfo.GetChatLineText(1, 1) ~= nil then return "line-text" end
            if C_ChatInfo.IsTimerunningPlayer() ~= false then return "timerunning" end
            if C_ChatInfo.GetChannelShortcutForChannelID(1) ~= "" then return "shortcut-id" end
            C_ChatInfo.CancelEmote()
            C_ChatInfo.UncensorChatLine(1)
            C_ChatInfo.DropCautionaryChatMessage(1)
            C_ChatInfo.SendCautionaryChatMessage(1)
            return "ok"
            "#,
        )
        .expect("C_ChatInfo temporary defaults should be callable");
    assert_eq!(result, "ok");
}

#[test]
fn chat_info_state_backed_channel_methods_still_win() {
    let env = env();
    let (count, shortcut, general_id): (i32, String, i32) = env
        .eval(
            r#"
            JoinChannelByName("General")
            return C_ChatInfo.GetNumActiveChannels(),
                C_ChatInfo.GetChannelShortcut(1),
                C_ChatInfo.GetGeneralChannelLocalID()
            "#,
        )
        .expect("C_ChatInfo state-backed channel methods should remain active");

    assert_eq!(count, 1);
    assert_eq!(shortcut, "1");
    assert_eq!(general_id, 1);
}
