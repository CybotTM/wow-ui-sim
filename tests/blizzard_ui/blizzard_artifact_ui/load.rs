//! Load smoke for `Blizzard_ArtifactUI`.

use crate::common::blizzard_addon_harness::with_blizzard_addon_smoke_shape;
use crate::common::panel_fixtures::{clear_recorded_lua_errors, recorded_lua_errors};
use wow_ui_sim::startup::settle_headless_startup;

const ROOT: &str = "Blizzard_ArtifactUI";

#[test]
fn artifact_ui_is_load_on_demand_until_explicitly_loaded() {
    with_blizzard_addon_smoke_shape(&[], &[], |env, _loaded| {
        let loaded_before: bool = env
            .eval(r#"return C_AddOns.IsAddOnLoaded("Blizzard_ArtifactUI")"#)
            .expect("pre-load IsAddOnLoaded probe should run cleanly");
        assert!(
            !loaded_before,
            "`{ROOT}` declares `LoadOnDemand: 1`, so startup must not load it eagerly"
        );

        let (loaded, error): (bool, Option<String>) = env
            .eval(r#"return C_AddOns.LoadAddOn("Blizzard_ArtifactUI")"#)
            .expect("C_AddOns.LoadAddOn probe should run cleanly");
        assert!(
            loaded,
            "`{ROOT}` must load after an explicit `C_AddOns.LoadAddOn`; error={error:?}"
        );

        let loaded_after: bool = env
            .eval(r#"return C_AddOns.IsAddOnLoaded("Blizzard_ArtifactUI")"#)
            .expect("post-load IsAddOnLoaded probe should run cleanly");
        assert!(
            loaded_after,
            "`{ROOT}` must report loaded after explicit `C_AddOns.LoadAddOn`"
        );
    });
}

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
