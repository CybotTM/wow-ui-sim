//! Load smoke for `Blizzard_ArtifactUI`.

use crate::common::blizzard_addon_harness::with_blizzard_addon_smoke_shape;
use crate::common::panel_fixtures::{clear_recorded_lua_errors, recorded_lua_errors};
use wow_ui_sim::startup::settle_headless_startup;

const ROOT: &str = "Blizzard_ArtifactUI";

#[test]
fn artifact_ui_loads_cleanly_via_c_addons_load_addon() {
    with_blizzard_addon_smoke_shape(&[], &[], |env, _loaded| {
        clear_recorded_lua_errors(env);

        let (loaded, error): (bool, Option<String>) = env
            .eval(r#"return C_AddOns.LoadAddOn("Blizzard_ArtifactUI")"#)
            .expect("C_AddOns.LoadAddOn probe should run cleanly");
        assert!(
            loaded,
            "`{ROOT}` must load via `C_AddOns.LoadAddOn`; error={error:?}"
        );

        ensure_player_frame_stub(env);
        env.apply_post_load_workarounds();
        settle_headless_startup(env);

        let errors = recorded_lua_errors(env);
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
