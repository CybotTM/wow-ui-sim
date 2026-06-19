use wow_ui_sim::loader::{discover_blizzard_addon_closure_for_screen, load_addon};
use wow_ui_sim::lua_api::WowLuaEnv;
use wow_ui_sim::screen::ScreenKind;

fn blizzard_ui_dir() -> std::path::PathBuf {
    wow_ui_sim::client_profile::blizzard_ui_addons_dir_under(std::path::Path::new(env!(
        "CARGO_MANIFEST_DIR"
    )))
}

#[test]
fn chat_frame_is_ready_before_third_party_addons_load() {
    let env = WowLuaEnv::new().expect("create Lua environment");
    env.set_screen_size(1600.0, 1200.0);
    let ui = blizzard_ui_dir();
    {
        let mut state = env.state().borrow_mut();
        state.addon_base_paths = vec![ui.clone()];
    }

    let addons = discover_blizzard_addon_closure_for_screen(
        &ui,
        ScreenKind::Game,
        &["Blizzard_FrameXMLBase", "Blizzard_ChatFrameBase"],
    );
    for (name, toc_path) in addons {
        load_addon(&env.loader_env(), &toc_path)
            .unwrap_or_else(|err| panic!("[load {name}] FAILED: {err}"));
    }

    wow_ui_sim::lua_api::chat_init::prepare_for_third_party_addons(&env);

    let ready: bool = env
        .eval(
            r#"
            return DEFAULT_CHAT_FRAME == ChatFrame1
                and DEFAULT_CHAT_FRAME.editBox == ChatFrame1EditBox
                and DEFAULT_CHAT_FRAME.editBox ~= nil
            "#,
        )
        .expect("inspect chat frame readiness");
    assert!(
        ready,
        "third-party addon startup expects DEFAULT_CHAT_FRAME.editBox to be real"
    );
}
