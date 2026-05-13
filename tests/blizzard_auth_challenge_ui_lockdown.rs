use crate::common;

use common::blizzard_addon_harness::with_blizzard_addon_smoke_shape;
use common::panel_fixtures::{clear_recorded_lua_errors, recorded_lua_errors};

const ROOT: &str = "Blizzard_AuthChallengeUI";
const LOCKDOWN_PROBE_LUA: &str = r#"
local parentBefore = AuthChallengeFrame:GetParent()
local forbiddenBefore = AuthChallengeFrame:IsForbidden()
local mutationParent = CreateFrame("Frame", "AuthChallengeMutationParent", UIParent)

forceinsecure()
local setParentOk = pcall(function()
  AuthChallengeFrame:SetParent(mutationParent)
end)
local childOk, child = pcall(function()
  return CreateFrame("Frame", nil, AuthChallengeFrame)
end)
debug.setstacktaint(nil)

local parentAfter = AuthChallengeFrame:GetParent()
local forbiddenAfter = AuthChallengeFrame:IsForbidden()
local childMarkedForbidden = child ~= nil and child:IsForbidden()

if parentAfter ~= parentBefore then
  AuthChallengeFrame:SetParent(parentBefore)
end

return forbiddenBefore,
       setParentOk,
       parentAfter == parentBefore or forbiddenAfter,
       childOk,
       child == nil or childMarkedForbidden or forbiddenAfter
"#;

#[test]
fn auth_challenge_ui_full_lockdown_marks_tainted_mutation_attempts() {
    common::with_perf_lock(|| {
        common::with_timeout(240, || {
            with_blizzard_addon_smoke_shape(&[], &[], |env, _loaded| {
                clear_recorded_lua_errors(env);

                let (loaded, reason): (bool, Option<String>) = env
                    .eval(r#"return C_AddOns.LoadAddOn("Blizzard_AuthChallengeUI")"#)
                    .expect("C_AddOns.LoadAddOn should return");
                assert!(loaded, "`{ROOT}` should load: {reason:?}");

                let (
                    was_forbidden,
                    set_parent_ok,
                    set_parent_rejected_or_marked,
                    child_create_ok,
                    child_rejected_or_marked,
                ): (bool, bool, bool, bool, bool) = env
                    .eval(LOCKDOWN_PROBE_LUA)
                    .expect("AuthChallengeUI lockdown probe should run");

                assert!(
                    was_forbidden,
                    "`AuthChallengeFrame` should inherit fullLockdown as forbidden state"
                );
                assert!(
                    set_parent_ok,
                    "tainted SetParent attempt should not crash the simulator"
                );
                assert!(
                    set_parent_rejected_or_marked,
                    "tainted SetParent attempt should be rejected or leave the frame marked forbidden"
                );
                assert!(
                    child_create_ok,
                    "tainted child creation attempt should not crash the simulator"
                );
                assert!(
                    child_rejected_or_marked,
                    "tainted child creation should be rejected or produce a forbidden child/frame"
                );

                let errors = recorded_lua_errors(env);
                assert!(
                    errors.is_empty(),
                    "`{ROOT}` lockdown probe emitted Lua errors:\n{}",
                    errors.join("\n")
                );
            });
        });
    });
}
