use crate::common;

use common::blizzard_addon_harness::with_blizzard_addon_smoke_shape;
use common::panel_fixtures::{clear_recorded_lua_errors, recorded_lua_errors};

const ROOT: &str = "Blizzard_AzeriteRespecUI";

#[test]
fn blizzard_azerite_respec_ui_show_and_hide_play_reforge_window_sounds() {
    common::with_perf_lock(|| {
        common::with_timeout(240, || {
            with_blizzard_addon_smoke_shape(&[], &[], |env, _loaded| {
                clear_recorded_lua_errors(env);

                let (loaded, reason): (bool, Option<String>) = env
                    .eval(r#"return C_AddOns.LoadAddOn("Blizzard_AzeriteRespecUI")"#)
                    .expect("C_AddOns.LoadAddOn should return for Blizzard_AzeriteRespecUI");
                assert!(loaded, "`{ROOT}` should load: {reason:?}");

                let (open_sound, close_sound): (i64, i64) = env
                    .eval(
                        r#"
                        return SOUNDKIT.UI_80_AZERITEARMOR_REFORGE_ETHEREALWINDOW_OPEN,
                            SOUNDKIT.UI_80_AZERITEARMOR_REFORGE_ETHEREALWINDOW_CLOSE
                        "#,
                    )
                    .expect("Azerite respec SOUNDKIT constants should resolve");

                env.exec(
                    r#"
                    if SetCVarBitfield and LE_FRAME_TUTORIAL_AZERITE_RESPEC then
                        SetCVarBitfield("closedInfoFrames", LE_FRAME_TUTORIAL_AZERITE_RESPEC, true)
                    end
                    "#,
                )
                .expect("tutorial CVar setup should run");

                env.exec(
                    r#"
                    local mixinEnv = debug.getfenv(AzeriteRespecMixin.OnShow)
                    _G.__azerite_respec_played_sounds = {}
                    mixinEnv.PlaySound = function(soundKitID)
                        table.insert(_G.__azerite_respec_played_sounds, soundKitID)
                    end
                    "#,
                )
                .expect("PlaySound tracker should install into AzeriteRespecMixin environment");

                env.exec("AzeriteRespecMixin.OnShow(AzeriteRespecFrame)")
                    .expect("AzeriteRespecMixin.OnShow should run cleanly");
                assert_eq!(
                    played_sound_at(env, 1),
                    Some(open_sound),
                    "`{ROOT}` OnShow should play UI_80_AZERITEARMOR_REFORGE_ETHEREALWINDOW_OPEN"
                );

                env.exec("AzeriteRespecMixin.OnHide(AzeriteRespecFrame)")
                    .expect("AzeriteRespecMixin.OnHide should run cleanly");
                assert_eq!(
                    played_sound_at(env, 2),
                    Some(close_sound),
                    "`{ROOT}` OnHide should play UI_80_AZERITEARMOR_REFORGE_ETHEREALWINDOW_CLOSE"
                );

                let errors = recorded_lua_errors(env);
                assert!(
                    errors.is_empty(),
                    "`{ROOT}` emitted Lua errors while checking show/hide sounds:\n{}",
                    errors.join("\n")
                );
            });
        });
    });
}

fn played_sound_at(env: &wow_ui_sim::lua_api::WowLuaEnv, index: i64) -> Option<i64> {
    env.eval(&format!(
        "return _G.__azerite_respec_played_sounds[{index}]"
    ))
    .expect("recorded PlaySound entry should be readable")
}
