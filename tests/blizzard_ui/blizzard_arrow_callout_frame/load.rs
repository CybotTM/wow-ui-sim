//! Load smoke for `Blizzard_ArrowCalloutFrame`.

use crate::common::blizzard_addon_harness::with_blizzard_addon_smoke_shape;
use crate::common::panel_fixtures::blizzard_ui_dir;
use wow_ui_sim::loader::load_addon;
use wow_ui_sim::startup::settle_headless_startup;

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
