//! Real startup/key-dispatch regression for Enter-to-chat, not VNC/native parity.
#![cfg(any(feature = "client-retail", feature = "client-ptr"))]

use wow_ui_sim::lua_api::WowLuaEnv;

#[cfg(feature = "client-ptr")]
fn run_ptr_chat_case(run_case: fn(&WowLuaEnv)) {
    test_timeout! {
        let env = WowLuaEnv::new().expect("fresh pre-addon environment");
        env.set_display_size(1920.0, 1080.0).expect("display input");
        let warnings = crate::edit_mode_cache_startup::load_whole_game_ui(&env);
        run_case(&env);
        assert!(warnings.is_empty(), "startup warnings retained: {warnings:?}");
    }
}

#[test]
#[cfg(feature = "client-ptr")]
fn ptr_default_enter_opens_real_chat_for_typing_and_submission() {
    run_ptr_chat_case(default_enter_opens_real_chat_for_typing_and_submission::run);
}

#[test]
#[cfg(feature = "client-ptr")]
fn ptr_explicit_enter_binding_overrides_default_chat_action() {
    run_ptr_chat_case(explicit_enter_binding_overrides_default_chat_action::run);
}

#[test]
#[cfg(feature = "client-ptr")]
fn ptr_explicitly_cleared_enter_binding_stays_unbound() {
    run_ptr_chat_case(explicitly_cleared_enter_binding_stays_unbound::run);
}

fn clear_chat_focus(env: &WowLuaEnv) {
    env.exec(
        r#"
        assert(type(ChatFrame_OpenChat) == 'function', 'whole chat startup readiness')
        local source = debug.getinfo(ChatFrame_OpenChat, 'S').source:gsub('\\', '/')
        assert(source:sub(-#'Blizzard_ChatFrameBase/Shared/ChatFrameUtil.lua') ==
            'Blizzard_ChatFrameBase/Shared/ChatFrameUtil.lua', 'original chat opener')
        ChatFrame1EditBox:ClearFocus()
        ChatFrame1EditBox:SetText('')
        assert(not ChatFrame1EditBox:HasFocus(), 'unfocused startup scenario')
        "#,
    )
    .expect("real chat readiness");
}

prefork_full_ui_case! {
fn default_enter_opens_real_chat_for_typing_and_submission(env: &WowLuaEnv) {
    clear_chat_focus(env);
    env.send_key_press("ENTER", None).expect("public Enter keypress");
    env.exec("assert(ChatFrame1EditBox:HasFocus(), 'Enter must activate original chat editor')")
        .expect("default Enter chat activation");
    env.send_key_press("A", Some("a")).expect("public text keypress");
    env.exec("assert(ChatFrame1EditBox:GetText() == 'a', 'typed text enters real chat editor')")
        .expect("real chat typing");
    env.send_key_press("ENTER", None).expect("original chat Enter handler");
    env.exec(
        "assert(ChatFrame1EditBox:GetText() == '', 'original submit clears text'); \
         assert(not ChatFrame1EditBox:HasFocus(), 'original submit releases focus')",
    ).expect("original chat submission editor lifecycle");
}
}

prefork_full_ui_case! {
fn explicit_enter_binding_overrides_default_chat_action(env: &WowLuaEnv) {
    clear_chat_focus(env);
    env.exec(
        "GameMenuFrame:Hide(); \
         assert(SetBinding('ENTER', 'TOGGLEGAMEMENU')); \
         assert(GetBindingAction('ENTER') == 'TOGGLEGAMEMENU')",
    ).expect("explicit user binding");
    env.send_key_press("ENTER", None).expect("configured Enter dispatch");
    env.exec(
        "assert(GameMenuFrame:IsShown(), 'configured action opens actual menu'); \
         assert(not ChatFrame1EditBox:HasFocus(), 'override must not open chat')",
    ).expect("configured binding preserved");
}
}

prefork_full_ui_case! {
fn explicitly_cleared_enter_binding_stays_unbound(env: &WowLuaEnv) {
    clear_chat_focus(env);
    env.exec("assert(SetBinding('ENTER')); assert(GetBindingAction('ENTER') == '')")
        .expect("explicit Enter unbinding");
    env.send_key_press("ENTER", None).expect("cleared Enter dispatch");
    env.exec("assert(not ChatFrame1EditBox:HasFocus(), 'explicit unbinding suppresses default chat')")
        .expect("explicit unbinding preserved");
}
}
