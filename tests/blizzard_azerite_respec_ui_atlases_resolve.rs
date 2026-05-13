use crate::common;

use common::blizzard_addon_harness::with_blizzard_addon_smoke_shape;
use common::panel_fixtures::{clear_recorded_lua_errors, recorded_lua_errors};

const ROOT: &str = "Blizzard_AzeriteRespecUI";

#[test]
fn blizzard_azerite_respec_ui_resolves_reforge_atlases_with_atlas_sizes() {
    common::with_perf_lock(|| {
        common::with_timeout(240, || {
            with_blizzard_addon_smoke_shape(&[], &[], |env, _loaded| {
                clear_recorded_lua_errors(env);

                let (loaded, reason): (bool, Option<String>) = env
                    .eval(r#"return C_AddOns.LoadAddOn("Blizzard_AzeriteRespecUI")"#)
                    .expect("C_AddOns.LoadAddOn should return for Blizzard_AzeriteRespecUI");
                assert!(loaded, "`{ROOT}` should load: {reason:?}");

                let (background_atlas, background_width, background_height): (String, f64, f64) =
                    env.eval(
                        r#"
                        return AzeriteRespecFrame.Background:GetAtlas(),
                            AzeriteRespecFrame.Background:GetWidth(),
                            AzeriteRespecFrame.Background:GetHeight()
                        "#,
                    )
                    .expect("AzeriteRespecFrame.Background atlas shape should be readable");
                assert_eq!(background_atlas, "azeritereforger-background");
                assert_eq!(background_width, 323.0);
                assert_eq!(background_height, 197.0);

                let (glow_atlas, glow_width, glow_height): (String, f64, f64) = env
                    .eval(
                        r#"
                        local glow = AzeriteRespecFrame.ItemSlot.GlowOverlay
                        return glow:GetAtlas(), glow:GetWidth(), glow:GetHeight()
                        "#,
                    )
                    .expect(
                        "AzeriteRespecFrame.ItemSlot.GlowOverlay atlas shape should be readable",
                    );
                assert_eq!(glow_atlas, "azeritereforger-glow");
                assert_eq!(glow_width, 133.0);
                assert_eq!(glow_height, 135.0);

                let errors = recorded_lua_errors(env);
                assert!(
                    errors.is_empty(),
                    "`{ROOT}` emitted Lua errors while checking atlas textures:\n{}",
                    errors.join("\n")
                );
            });
        });
    });
}
