use crate::common;

use common::blizzard_addon_harness::with_blizzard_addon_smoke_shape;
use common::panel_fixtures::{clear_recorded_lua_errors, recorded_lua_errors};

const ROOT: &str = "Blizzard_AzeriteRespecUI";

#[test]
fn blizzard_azerite_respec_ui_help_tip_shows_only_before_tutorial_bit_is_set() {
    common::with_perf_lock(|| {
        common::with_timeout(240, || {
            with_blizzard_addon_smoke_shape(&[], &[], |env, _loaded| {
                clear_recorded_lua_errors(env);

                let (loaded, reason): (bool, Option<String>) = env
                    .eval(r#"return C_AddOns.LoadAddOn("Blizzard_AzeriteRespecUI")"#)
                    .expect("C_AddOns.LoadAddOn should return for Blizzard_AzeriteRespecUI");
                assert!(loaded, "`{ROOT}` should load: {reason:?}");

                env.exec(
                    r#"
                    SetCVarBitfield("closedInfoFrames", LE_FRAME_TUTORIAL_AZERITE_RESPEC, false)

                    local mixinEnv = debug.getfenv(AzeriteRespecMixin.OnShow)
                    mixinEnv.PlaySound = function() end
                    mixinEnv.HelpTip = {
                        ButtonStyle = { Close = "Close" },
                        Point = { RightEdgeCenter = "RightEdgeCenter" },
                        Show = function(_, parent, info, relativeRegion)
                            _G.__azerite_respec_help_tip_count =
                                (_G.__azerite_respec_help_tip_count or 0) + 1
                            _G.__azerite_respec_help_tip_parent = parent
                            _G.__azerite_respec_help_tip_info = info
                            _G.__azerite_respec_help_tip_relative_region = relativeRegion
                        end,
                    }
                    _G.__azerite_respec_help_tip_count = 0
                    "#,
                )
                .expect("HelpTip recorder should install cleanly");

                env.exec("AzeriteRespecMixin.OnShow(AzeriteRespecFrame)")
                    .expect("AzeriteRespecMixin.OnShow should run with tutorial bit clear");

                let first_open: (i64, bool, bool, String, String, i64, String, i64) = env
                    .eval(
                        r#"
                        local info = _G.__azerite_respec_help_tip_info or {}
                        return _G.__azerite_respec_help_tip_count,
                            _G.__azerite_respec_help_tip_parent == AzeriteRespecFrame,
                            _G.__azerite_respec_help_tip_relative_region == AzeriteRespecFrame.ItemSlot,
                            info.text or "",
                            info.cvarBitfield or "",
                            info.bitfieldFlag or -1,
                            info.targetPoint or "",
                            info.offsetX or 0
                        "#,
                    )
                    .expect("first-open HelpTip call should be readable");
                assert_eq!(first_open.0, 1, "`{ROOT}` should show the tutorial once");
                assert!(first_open.1, "`{ROOT}` HelpTip parent should be the panel");
                assert!(
                    first_open.2,
                    "`{ROOT}` HelpTip should anchor to AzeriteRespecFrame.ItemSlot"
                );
                assert_eq!(
                    first_open.3,
                    "Drag a piece of Azerite Armor here to reforge its powers."
                );
                assert_eq!(first_open.4, "closedInfoFrames");
                assert_eq!(first_open.5, 57);
                assert_eq!(first_open.6, "RightEdgeCenter");
                assert_eq!(first_open.7, -10);

                env.exec(
                    r#"
                    SetCVarBitfield("closedInfoFrames", LE_FRAME_TUTORIAL_AZERITE_RESPEC, true)
                    _G.__azerite_respec_help_tip_count = 0
                    AzeriteRespecMixin.OnShow(AzeriteRespecFrame)
                    "#,
                )
                .expect("AzeriteRespecMixin.OnShow should run with tutorial bit set");
                let second_open_count: i64 = env
                    .eval("return _G.__azerite_respec_help_tip_count")
                    .expect("second-open HelpTip count should be readable");
                assert_eq!(
                    second_open_count, 0,
                    "`{ROOT}` should not show the tutorial after the closedInfoFrames bit is set"
                );

                let errors = recorded_lua_errors(env);
                assert!(
                    errors.is_empty(),
                    "`{ROOT}` emitted Lua errors while checking first-open HelpTip behavior:\n{}",
                    errors.join("\n")
                );
            });
        });
    });
}
