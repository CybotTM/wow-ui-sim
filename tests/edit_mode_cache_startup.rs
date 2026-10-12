//! File-cache import before whole Game startup; no mocked EditMode consumer.
#![cfg(any(feature = "client-retail", feature = "client-ptr"))]

use std::path::Path;
use wow_ui_sim::loader::{discover_blizzard_startup_addons_for_screen, load_addon};
use wow_ui_sim::lua_api::WowLuaEnv;
use wow_ui_sim::saved_variables::{SavedVariablesManager, WtfConfig};
use wow_ui_sim::screen::ScreenKind;

const ACCOUNT_CACHE: &[u8] =
    include_bytes!("fixtures/frame_position_replay/edit-mode-cache-account.txt");
const CHARACTER_CACHE: &[u8] = b"2 2 2 2 0 0\0";

fn import_fixture_cache(env: &WowLuaEnv, root: &Path) {
    let wtf = root.join("WTF");
    let account = wtf.join("Account/TestAccount");
    let character = account.join("Test Realm/Testchar");
    std::fs::create_dir_all(&character).expect("create isolated cache tree");
    std::fs::write(account.join("edit-mode-cache-account.txt"), ACCOUNT_CACHE)
        .expect("write supplied account cache");
    std::fs::write(
        character.join("edit-mode-cache-character.txt"),
        CHARACTER_CACHE,
    )
    .expect("write native character-cache indices");
    let mut manager = SavedVariablesManager::with_storage_dir(root.join("local-sv"));
    manager.set_wtf_config(WtfConfig::new(
        &wtf,
        "TestAccount",
        "Test Realm",
        "Testchar",
    ));
    let loaded = env
        .loader_env()
        .with_state(|state| manager.load_edit_mode_cache(state, 2, None))
        .expect("production file-cache import");
    assert!(loaded, "provided cache must be imported");
    env.exec("assert(C_EditMode.GetLayouts().activeLayout == 2, 'native cache-index readiness')")
        .expect("unaltered character-cache active index");
}

fn load_whole_game_ui(env: &WowLuaEnv) {
    let ui = wow_ui_sim::client_profile::blizzard_ui_addons_dir();
    env.set_screen_mode(ScreenKind::Game);
    env.state().borrow_mut().addon_base_paths = vec![ui.clone()];
    wow_ui_sim::xml::register_intrinsic_templates();
    let addons = discover_blizzard_startup_addons_for_screen(&ui, ScreenKind::Game);
    for (name, toc) in addons {
        let result = load_addon(&env.loader_env(), &toc)
            .unwrap_or_else(|error| panic!("whole Game startup load {name}: {error}"));
        assert!(
            result.warnings.is_empty(),
            "startup {name}: {:?}",
            result.warnings
        );
    }
    env.apply_post_load_workarounds();
    wow_ui_sim::startup::fire_startup_events(env);
    wow_ui_sim::startup::process_pending_timers(env);
    wow_ui_sim::startup::fire_one_on_update_tick(env);
}

#[test]
fn native_preset_index_two_keeps_classic_and_horizontal_micro_menu() {
    test_timeout! {
        let temp = tempfile::tempdir().expect("create isolated cache fixture");
        let env = WowLuaEnv::new().expect("fresh pre-addon environment");
        env.set_display_size(1920.0, 1080.0).expect("display input");
        import_fixture_cache(&env, temp.path());
        load_whole_game_ui(&env);
        env.exec(r#"
            local layout = EditModeManagerFrame:GetActiveLayoutInfo()
            local classic = EditModePresetLayoutManager:GetCopyOfPresetLayouts()[2]
            assert(layout.layoutName == classic.layoutName,
                'native cached preset index 2 must select Classic, not saved Widescreen')
            local width, height = MicroMenu:GetSize()
            assert(width > 0 and height > 0, 'actual micro-menu geometry readiness')
            assert(width > height, 'Classic micro menu must have horizontal geometry')
            local _, firstY = CharacterMicroButton:GetCenter()
            local _, secondY = PlayerSpellsMicroButton:GetCenter()
            assert(firstY and secondY and math.abs(firstY - secondY) <= 1,
                'first microbuttons must share a horizontal row')
        "#).expect("unchanged Classic/micro-menu consumer result");
    }
}
