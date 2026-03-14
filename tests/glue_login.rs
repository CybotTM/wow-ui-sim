mod common;

use std::path::PathBuf;
use wow_ui_sim::loader::{discover_blizzard_addons_for_screen, load_addon};
use wow_ui_sim::lua_api::WowLuaEnv;
use wow_ui_sim::screen::ScreenKind;
use wow_ui_sim::startup::fire_startup_events_for_screen;

fn blizzard_ui_dir() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("Interface/BlizzardUI")
}

fn load_blizzard_screen(screen: ScreenKind) -> WowLuaEnv {
    let env = WowLuaEnv::new().expect("Failed to create Lua environment");
    env.set_screen_size(1024.0, 768.0);
    env.set_screen_mode(screen);

    let ui = blizzard_ui_dir();
    let addons = discover_blizzard_addons_for_screen(&ui, screen);
    for (name, toc_path) in &addons {
        if let Err(err) = load_addon(&env.loader_env(), toc_path) {
            panic!("[load {name}] FAILED: {err}");
        }
    }

    env.apply_post_load_workarounds();
    fire_startup_events_for_screen(&env, screen);
    env
}

#[test]
fn login_boot_hides_non_login_frontend_frames() {
    test_timeout! {
        let env = load_blizzard_screen(ScreenKind::Login);

        let stubs_present: bool = env
            .eval(
                r#"
                return type(GetSavedAccountName) == "function"
                    and type(SetSavedAccountName) == "function"
                    and type(GetSavedAccountList) == "function"
                    and type(SetUsesToken) == "function"
                    and type(WasScreenFirstDisplayed) == "function"
                    and type(C_Login.IsLoginReady) == "function"
                "#,
            )
            .expect("glue login stubs should be callable");
        assert!(stubs_present, "login boot should expose required glue account helpers");

        let missing_stub_errors: Vec<String> = env
            .state()
            .borrow()
            .lua_errors
            .iter()
            .filter(|msg| {
                msg.contains("GetSavedAccountName")
                    || msg.contains("IsLoginReady")
                    || msg.contains("WasScreenFirstDisplayed")
            })
            .cloned()
            .collect();
        assert!(
            missing_stub_errors.is_empty(),
            "login boot should not error on glue account helpers: {missing_stub_errors:#?}"
        );

        let account_login_visible: bool = env
            .eval("return AccountLogin ~= nil and AccountLogin:IsShown()")
            .expect("AccountLogin visibility should be queryable");
        assert!(account_login_visible, "login screen should show AccountLogin");

        let chat_frame_visible: bool = env
            .eval("return ChatFrame1 ~= nil and ChatFrame1:IsShown()")
            .expect("ChatFrame1 visibility should be queryable");
        assert!(
            !chat_frame_visible,
            "plain login screen should not show the front-end chat frame"
        );

        let chat_dock_visible: bool = env
            .eval("return GeneralDockManager ~= nil and GeneralDockManager:IsShown()")
            .expect("GeneralDockManager visibility should be queryable");
        assert!(
            !chat_dock_visible,
            "plain login screen should not show the chat dock"
        );

        let char_customize_visible: bool = env
            .eval("return CharCustomizeFrame ~= nil and CharCustomizeFrame:IsShown()")
            .expect("CharCustomizeFrame visibility should be queryable");
        assert!(
            !char_customize_visible,
            "plain login screen should not show character customization"
        );
    }
}
