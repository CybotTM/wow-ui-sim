use crate::common;

use common::blizzard_addon_harness::with_blizzard_addon_smoke_shape;
use common::panel_fixtures::{clear_recorded_lua_errors, recorded_lua_errors};

const ROOT: &str = "Blizzard_AuthChallengeUI";
const ONLOAD_PROBE_LUA: &str = r#"
local calls = {}
local originalSetFrame = C_AuthChallenge.SetFrame

C_AuthChallenge.SetFrame = function(frame)
  table.insert(calls, frame)
end

AuthChallengeUI_OnLoad(AuthChallengeFrame)
C_AuthChallenge.SetFrame = originalSetFrame

return #calls, calls[1] == AuthChallengeFrame
"#;

#[test]
fn auth_challenge_ui_onload_sets_auth_challenge_frame() {
    common::with_perf_lock(|| {
        common::with_timeout(240, || {
            with_blizzard_addon_smoke_shape(&[], &[], |env, _loaded| {
                clear_recorded_lua_errors(env);

                let (loaded, reason): (bool, Option<String>) = env
                    .eval(r#"return C_AddOns.LoadAddOn("Blizzard_AuthChallengeUI")"#)
                    .expect("C_AddOns.LoadAddOn should return");
                assert!(loaded, "`{ROOT}` should load: {reason:?}");

                let (call_count, received_auth_frame): (i32, bool) = env
                    .eval(ONLOAD_PROBE_LUA)
                    .expect("AuthChallengeUI_OnLoad probe should run");
                assert_eq!(
                    call_count, 1,
                    "`AuthChallengeUI_OnLoad` should call `C_AuthChallenge.SetFrame` once"
                );
                assert!(
                    received_auth_frame,
                    "`AuthChallengeUI_OnLoad` should pass `AuthChallengeFrame` to `SetFrame`"
                );

                let errors = recorded_lua_errors(env);
                assert!(
                    errors.is_empty(),
                    "`{ROOT}` OnLoad probe emitted Lua errors:\n{}",
                    errors.join("\n")
                );
            });
        });
    });
}
