use crate::common;

use common::blizzard_addon_harness::with_blizzard_addon_smoke_shape;
use common::panel_fixtures::{clear_recorded_lua_errors, recorded_lua_errors};

const ROOT: &str = "Blizzard_AuthChallengeUI";
const FRAME_SURFACE_PROBE_LUA: &str = r#"
local failures = {}

local function expect(condition, message)
  if not condition then
    table.insert(failures, message)
  end
end

local function expectObjectType(frame, expectedType, message)
  expect(frame ~= nil, message .. " must exist")
  if frame ~= nil then
    expect(frame:GetObjectType() == expectedType,
           message .. " must be a " .. expectedType)
  end
end

expect(AuthChallengeFrame ~= nil, "AuthChallengeFrame must exist")
local parent = AuthChallengeFrame and AuthChallengeFrame:GetParent()
local parentName = parent and parent:GetName() or "nil"
expect(parent == UIParent,
       "AuthChallengeFrame must be parented to UIParent, got "
       .. parentName .. " (" .. tostring(parent) .. " vs " .. tostring(UIParent) .. ")")
expect(AuthChallengeFrame:GetFrameStrata() == UIParent:GetFrameStrata(),
       "AuthChallengeFrame must inherit UIParent's effective strata")
expect(not AuthChallengeFrame:HasFixedFrameStrata(),
       "AuthChallengeFrame BLIZZARD token must not fix its strata")
expect(not AuthChallengeFrame:IsShown(), "AuthChallengeFrame must start hidden")
expect(AuthChallengeFrame:IsKeyboardEnabled(), "AuthChallengeFrame must enable keyboard")
expect(AuthChallengeFrame:IsMouseEnabled(), "AuthChallengeFrame must enable mouse")

for _, childName in ipairs({ "WaitFrame", "InputFrame", "DeniedFrame", "ErrorFrame" }) do
  local child = AuthChallengeFrame[childName]
  expect(child ~= nil, "AuthChallengeFrame." .. childName .. " must exist")
  if child ~= nil then
    expect(not child:IsShown(), "AuthChallengeFrame." .. childName .. " must start hidden")
  end
end

local inputFrame = AuthChallengeFrame.InputFrame
if inputFrame ~= nil then
  for _, inputName in ipairs({ "Input1", "Input2", "Input3", "Input4" }) do
    expectObjectType(inputFrame[inputName], "EditBox", "AuthChallengeFrame.InputFrame." .. inputName)
  end

  expectObjectType(inputFrame.Submit, "Button", "AuthChallengeFrame.InputFrame.Submit")
  expectObjectType(inputFrame.Prompt, "FontString", "AuthChallengeFrame.InputFrame.Prompt")
  expectObjectType(inputFrame.Info, "FontString", "AuthChallengeFrame.InputFrame.Info")
  expectObjectType(inputFrame.Error, "FontString", "AuthChallengeFrame.InputFrame.Error")
end

return table.concat(failures, "\n")
"#;

#[test]
fn auth_challenge_ui_frame_surface_exists_after_load() {
    common::with_perf_lock(|| {
        common::with_timeout(240, || {
            with_blizzard_addon_smoke_shape(&[], &[], |env, _loaded| {
                clear_recorded_lua_errors(env);

                let (loaded, reason): (bool, Option<String>) = env
                    .eval(r#"return C_AddOns.LoadAddOn("Blizzard_AuthChallengeUI")"#)
                    .expect("C_AddOns.LoadAddOn should return");
                assert!(loaded, "`{ROOT}` should load: {reason:?}");

                let failures: String = env
                    .eval(FRAME_SURFACE_PROBE_LUA)
                    .expect("AuthChallengeUI frame surface probe should run");
                assert!(failures.is_empty(), "`{ROOT}` missing frames:\n{failures}");

                let errors = recorded_lua_errors(env);
                assert!(
                    errors.is_empty(),
                    "`{ROOT}` frame-surface load emitted Lua errors:\n{}",
                    errors.join("\n")
                );
            });
        });
    });
}
