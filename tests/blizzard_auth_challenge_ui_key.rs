use crate::common;

use common::blizzard_addon_harness::with_blizzard_addon_smoke_shape;
use common::panel_fixtures::{clear_recorded_lua_errors, recorded_lua_errors};

const ROOT: &str = "Blizzard_AuthChallengeUI";
const KEYDOWN_PROBE_LUA: &str = r##"
return select("#", AuthChallengeUI_OnKeyDown(AuthChallengeFrame, "ENTER"))
"##;

#[test]
fn auth_challenge_ui_key_down_is_silent_noop() {
    common::with_perf_lock(|| {
        common::with_timeout(240, || {
            with_blizzard_addon_smoke_shape(&[], &[], |env, _loaded| {
                clear_recorded_lua_errors(env);

                let (loaded, reason): (bool, Option<String>) = env
                    .eval(r#"return C_AddOns.LoadAddOn("Blizzard_AuthChallengeUI")"#)
                    .expect("C_AddOns.LoadAddOn should return");
                assert!(loaded, "`{ROOT}` should load: {reason:?}");

                let return_count: i32 = env
                    .eval(KEYDOWN_PROBE_LUA)
                    .expect("AuthChallengeUI_OnKeyDown probe should run");
                assert_eq!(
                    return_count, 0,
                    "`AuthChallengeUI_OnKeyDown` should trap keys without returning values"
                );

                let errors = recorded_lua_errors(env);
                assert!(
                    errors.is_empty(),
                    "`{ROOT}` OnKeyDown probe emitted Lua errors:\n{}",
                    errors.join("\n")
                );
            });
        });
    });
}
