//! Load smoke for `Blizzard_ArrowCalloutFrame`.

use crate::common::blizzard_addon_harness::with_blizzard_addon_smoke_shape;
use crate::common::panel_fixtures::blizzard_ui_dir;
use wow_ui_sim::loader::load_addon;
use wow_ui_sim::startup::settle_headless_startup;
use wow_ui_sim::toc::TocFile;

const ROOT: &str = "Blizzard_ArrowCalloutFrame";

#[test]
fn arrow_callout_frame_loads_cleanly_with_no_recorded_lua_errors() {
    with_blizzard_addon_smoke_shape(&[], &[], |env, _loaded| {
        load_addon(&env.loader_env(), &arrow_callout_toc())
            .expect("Blizzard_ArrowCalloutFrame should load directly from its TOC");

        ensure_player_frame_stub(env);
        env.apply_post_load_workarounds();
        settle_headless_startup(env);

        let errors = env.state().borrow().lua_errors.clone();
        assert!(
            errors.is_empty(),
            "`{ROOT}` must load and settle with zero recorded Lua errors:\n{}",
            errors.join("\n")
        );
    });
}

#[test]
fn arrow_callout_frame_toc_declares_ui_parent_and_help_plate_deps() {
    let toc = TocFile::from_file(&arrow_callout_toc())
        .expect("Blizzard_ArrowCalloutFrame TOC should parse");

    assert_eq!(
        toc.dependencies(),
        ["Blizzard_UIParent", "Blizzard_HelpPlate"],
        "`{ROOT}` must keep its source-declared load prerequisites explicit"
    );
    assert_eq!(
        toc.metadata.get("AllowLoad").map(String::as_str),
        Some("game")
    );
    assert_eq!(
        toc.metadata.get("AllowLoadGameType").map(String::as_str),
        Some("plunderstorm")
    );
    assert!(
        toc.is_game_type_restricted(),
        "`{ROOT}` is plunderstorm-restricted, so the smoke harness loads it directly after shared game UI setup"
    );
    assert_eq!(
        toc.files,
        [
            std::path::PathBuf::from("ArrowCalloutFrame.lua"),
            std::path::PathBuf::from("ArrowCalloutFrame.xml"),
        ]
    );
}

fn arrow_callout_toc() -> std::path::PathBuf {
    blizzard_ui_dir()
        .join(ROOT)
        .join("Blizzard_ArrowCalloutFrame.toc")
}

fn ensure_player_frame_stub(env: &wow_ui_sim::lua_api::WowLuaEnv) {
    env.exec(
        r#"
        if not PlayerFrame then
            PlayerFrame = CreateFrame("Frame", "PlayerFrame", UIParent)
        end
        PlayerFrame.unit = "player"
        "#,
    )
    .expect("failed to create PlayerFrame stub before settling startup events");
}
