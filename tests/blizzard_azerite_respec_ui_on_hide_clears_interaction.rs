use crate::common;

use common::blizzard_addon_harness::with_blizzard_addon_smoke_shape;
use common::panel_fixtures::{clear_recorded_lua_errors, recorded_lua_errors};

const ROOT: &str = "Blizzard_AzeriteRespecUI";

#[test]
fn blizzard_azerite_respec_ui_on_hide_clears_interaction_and_popups() {
    common::with_perf_lock(|| {
        common::with_timeout(240, || {
            with_blizzard_addon_smoke_shape(&[], &[], |env, _loaded| {
                clear_recorded_lua_errors(env);

                let (loaded, reason): (bool, Option<String>) = env
                    .eval(r#"return C_AddOns.LoadAddOn("Blizzard_AzeriteRespecUI")"#)
                    .expect("C_AddOns.LoadAddOn should return for Blizzard_AzeriteRespecUI");
                assert!(loaded, "`{ROOT}` should load: {reason:?}");

                let interaction_type: i64 = env
                    .eval("return Enum.PlayerInteractionType.AzeriteRespec")
                    .expect("AzeriteRespec interaction enum should resolve");
                env.state()
                    .borrow_mut()
                    .active_player_interactions
                    .insert(interaction_type as i32);

                env.exec(
                    r#"
                    if SetCVarBitfield and LE_FRAME_TUTORIAL_AZERITE_RESPEC then
                        SetCVarBitfield("closedInfoFrames", LE_FRAME_TUTORIAL_AZERITE_RESPEC, true)
                    end

                    local mixinEnv = debug.getfenv(AzeriteRespecMixin.OnHide)
                    _G.__azerite_respec_hidden_popups = {}
                    mixinEnv.StaticPopup_Hide = function(which)
                        table.insert(_G.__azerite_respec_hidden_popups, which)
                    end
                    "#,
                )
                .expect("OnHide probe hooks should install cleanly");

                env.exec(
                    r#"
                    AzeriteRespecMixin.OnShow(AzeriteRespecFrame)
                    AzeriteRespecMixin.OnHide(AzeriteRespecFrame)
                    "#,
                )
                .expect("AzeriteRespec show/hide lifecycle should run cleanly");

                let still_active = env
                    .state()
                    .borrow()
                    .active_player_interactions
                    .contains(&(interaction_type as i32));
                assert!(
                    !still_active,
                    "`{ROOT}` OnHide should clear Enum.PlayerInteractionType.AzeriteRespec"
                );

                let (first_popup, second_popup): (String, String) = env
                    .eval(
                        r#"
                        return _G.__azerite_respec_hidden_popups[1] or "",
                            _G.__azerite_respec_hidden_popups[2] or ""
                        "#,
                    )
                    .expect("StaticPopup_Hide calls should be readable");
                assert_eq!(first_popup, "CONFIRM_AZERITE_EMPOWERED_RESPEC");
                assert_eq!(second_popup, "CONFIRM_AZERITE_EMPOWERED_RESPEC_EXPENSIVE");

                let errors = recorded_lua_errors(env);
                assert!(
                    errors.is_empty(),
                    "`{ROOT}` emitted Lua errors while checking OnHide cleanup:\n{}",
                    errors.join("\n")
                );
            });
        });
    });
}
