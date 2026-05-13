use crate::common;

use common::blizzard_addon_harness::with_blizzard_addon_smoke_shape;
use common::panel_fixtures::{clear_recorded_lua_errors, recorded_lua_errors};

const ROOT: &str = "Blizzard_AuthChallengeUI";
const TAB_PROBE_LUA: &str = r#"
local calls = {}
local originalOnTabPressed = C_AuthChallenge.OnTabPressed
local originalIsShiftKeyDown = IsShiftKeyDown
local originalSecureIsShiftKeyDown = __secureenv and __secureenv.IsShiftKeyDown

C_AuthChallenge.OnTabPressed = function(frame, isShiftPressed)
  table.insert(calls, { frame = frame, isShiftPressed = isShiftPressed })
end

local function setShiftStub(value)
  IsShiftKeyDown = function()
    return value
  end
  if type(__secureenv) == "table" then
    __secureenv.IsShiftKeyDown = IsShiftKeyDown
  end
end

setShiftStub(false)
AuthChallengeUI_OnTabPressed(AuthChallengeFrame.InputFrame.Input1)

setShiftStub(true)
AuthChallengeUI_OnTabPressed(AuthChallengeFrame.InputFrame.Input1)

C_AuthChallenge.OnTabPressed = originalOnTabPressed
IsShiftKeyDown = originalIsShiftKeyDown
if type(__secureenv) == "table" then
  __secureenv.IsShiftKeyDown = originalSecureIsShiftKeyDown
end

return #calls,
       calls[1].frame == AuthChallengeFrame.InputFrame.Input1,
       calls[1].isShiftPressed,
       calls[2].frame == AuthChallengeFrame.InputFrame.Input1,
       calls[2].isShiftPressed
"#;

#[test]
fn auth_challenge_ui_tab_pressed_forwards_frame_and_shift_state() {
    common::with_perf_lock(|| {
        common::with_timeout(240, || {
            with_blizzard_addon_smoke_shape(&[], &[], |env, _loaded| {
                clear_recorded_lua_errors(env);

                let (loaded, reason): (bool, Option<String>) = env
                    .eval(r#"return C_AddOns.LoadAddOn("Blizzard_AuthChallengeUI")"#)
                    .expect("C_AddOns.LoadAddOn should return");
                assert!(loaded, "`{ROOT}` should load: {reason:?}");

                let result: (i32, bool, bool, bool, bool) = env
                    .eval(TAB_PROBE_LUA)
                    .expect("AuthChallengeUI_OnTabPressed probe should run");
                let (
                    call_count,
                    first_frame_matches,
                    first_shift_pressed,
                    second_frame_matches,
                    second_shift_pressed,
                ) = result;

                assert_eq!(
                    call_count, 2,
                    "`AuthChallengeUI_OnTabPressed` should call `C_AuthChallenge.OnTabPressed` twice"
                );
                assert!(
                    first_frame_matches,
                    "first OnTabPressed call should receive the original frame"
                );
                assert!(
                    second_frame_matches,
                    "second OnTabPressed call should receive the original frame"
                );
                assert!(
                    !first_shift_pressed,
                    "first OnTabPressed call should forward false shift state"
                );
                assert!(
                    second_shift_pressed,
                    "second OnTabPressed call should forward true shift state"
                );

                let errors = recorded_lua_errors(env);
                assert!(
                    errors.is_empty(),
                    "`{ROOT}` OnTabPressed probe emitted Lua errors:\n{}",
                    errors.join("\n")
                );
            });
        });
    });
}
