use crate::common;

use common::blizzard_addon_harness::with_blizzard_addon_smoke_shape;
use common::panel_fixtures::{clear_recorded_lua_errors, recorded_lua_errors};

const ROOT: &str = "Blizzard_AuthChallengeUI";
const GLOBAL_SURFACE_PROBE_LUA: &str = r#"
local failures = {}

local expected = {
  "AuthChallengeUI_OnLoad",
  "AuthChallengeUI_Submit",
  "AuthChallengeUI_Cancel",
  "AuthChallengeUI_OnTabPressed",
  "AuthChallengeUI_OnKeyDown",
}

for _, name in ipairs(expected) do
  if type(_G[name]) ~= "function" then
    table.insert(failures, name .. " must be a function")
  end
end

return table.concat(failures, "\n")
"#;

#[test]
fn auth_challenge_ui_public_globals_exist_after_load() {
    common::with_perf_lock(|| {
        common::with_timeout(240, || {
            with_blizzard_addon_smoke_shape(&[], &[], |env, _loaded| {
                clear_recorded_lua_errors(env);

                let (loaded, reason): (bool, Option<String>) = env
                    .eval(r#"return C_AddOns.LoadAddOn("Blizzard_AuthChallengeUI")"#)
                    .expect("C_AddOns.LoadAddOn should return");
                assert!(loaded, "`{ROOT}` should load: {reason:?}");

                let failures: String = env
                    .eval(GLOBAL_SURFACE_PROBE_LUA)
                    .expect("AuthChallengeUI global surface probe should run");
                assert!(
                    failures.is_empty(),
                    "`{ROOT}` missing public globals:\n{failures}"
                );

                let errors = recorded_lua_errors(env);
                assert!(
                    errors.is_empty(),
                    "`{ROOT}` global-surface load emitted Lua errors:\n{}",
                    errors.join("\n")
                );
            });
        });
    });
}
