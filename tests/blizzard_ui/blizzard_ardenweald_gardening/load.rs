//! Load smoke for `Blizzard_ArdenwealdGardening`.

use crate::common::blizzard_addon_harness::with_blizzard_addon_smoke_shape;
use wow_ui_sim::startup::settle_headless_startup;

const ROOT: &str = "Blizzard_ArdenwealdGardening";

#[test]
fn ardenweald_gardening_loads_cleanly_with_no_recorded_lua_errors() {
    with_blizzard_addon_smoke_shape(&[ROOT], &[], |env, loaded| {
        assert!(
            loaded.iter().any(|name| name == ROOT),
            "`{ROOT}` must be loaded as the requested LoadOnDemand root"
        );

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
