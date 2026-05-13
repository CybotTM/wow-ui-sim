use crate::common;

use common::blizzard_addon_harness::with_blizzard_addon_smoke_shape;
use common::panel_fixtures::{clear_recorded_lua_errors, recorded_lua_errors};

const ROOT: &str = "Blizzard_AuthChallengeUI";
const REQUIRED_STRINGS: &[&str] = &[
    "BLIZZARD_CHALLENGE_CONNECTING",
    "BLIZZARD_CHALLENGE_CANCEL",
    "BLIZZARD_CHALLENGE_SCREEN_EXPLANATION",
    "BLIZZARD_CHALLENGE_SUBMIT",
    "BLIZZARD_CHALLENGE_DENIED_TITLE",
    "BLIZZARD_CHALLENGE_DENIED_DESCRIPTION",
    "BLIZZARD_CHALLENGE_ERROR_TITLE",
    "BLIZZARD_CHALLENGE_ERROR_DESCRIPTION",
    "BLIZZARD_CHALLENGE_OKAY",
];

#[test]
fn auth_challenge_ui_localization_globals_are_seeded() {
    common::with_perf_lock(|| {
        common::with_timeout(240, || {
            with_blizzard_addon_smoke_shape(&[], &[], |env, _loaded| {
                clear_recorded_lua_errors(env);

                let (loaded, reason): (bool, Option<String>) = env
                    .eval(r#"return C_AddOns.LoadAddOn("Blizzard_AuthChallengeUI")"#)
                    .expect("C_AddOns.LoadAddOn should return");
                assert!(loaded, "`{ROOT}` should load: {reason:?}");

                for global_name in REQUIRED_STRINGS {
                    let value: Option<String> = env
                        .eval(&format!("return _G[{global_name:?}]"))
                        .unwrap_or_else(|err| {
                            panic!("string probe for `{global_name}` failed: {err}")
                        });
                    let value = value
                        .unwrap_or_else(|| panic!("`{global_name}` should resolve to a string"));
                    assert!(
                        !value.is_empty(),
                        "`{global_name}` should resolve to a non-empty string"
                    );
                }

                let errors = recorded_lua_errors(env);
                assert!(
                    errors.is_empty(),
                    "`{ROOT}` string probe emitted Lua errors:\n{}",
                    errors.join("\n")
                );
            });
        });
    });
}
