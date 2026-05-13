use crate::common;

use common::blizzard_addon_harness::with_blizzard_addon_smoke_shape;
use common::panel_fixtures::{clear_recorded_lua_errors, recorded_lua_errors};

const ROOT: &str = "Blizzard_AuthChallengeUI";
const INPUT_COUNT_PROBE_LUA: &str = r#"
local failures = {}

local function expect(condition, message)
  if not condition then
    table.insert(failures, message)
  end
end

local inputFrame = AuthChallengeFrame and AuthChallengeFrame.InputFrame
expect(inputFrame ~= nil, "AuthChallengeFrame.InputFrame must exist")
if inputFrame == nil then
  return table.concat(failures, "\n")
end

local editBoxCount = 0
for _, child in ipairs({ inputFrame:GetChildren() }) do
  if child:GetObjectType() == "EditBox" then
    editBoxCount = editBoxCount + 1
  end
end
expect(editBoxCount == 4, "InputFrame must have exactly four EditBox children")

for index = 1, 4 do
  local input = inputFrame["Input" .. index]
  expect(input ~= nil, "Input" .. index .. " must exist by parentKey")
  if input ~= nil then
    expect(input:GetParent() == inputFrame, "Input" .. index .. " must be parented to InputFrame")
    expect(input:GetObjectType() == "EditBox", "Input" .. index .. " must be an EditBox")
    expect(input:GetWidth() == 163, "Input" .. index .. " must inherit template width")
    expect(input:GetHeight() == 20, "Input" .. index .. " must inherit template height")
    expect(input.LeftTexture ~= nil, "Input" .. index .. " must inherit LeftTexture")
    expect(input.RightTexture ~= nil, "Input" .. index .. " must inherit RightTexture")
    expect(input.MiddleTexture ~= nil, "Input" .. index .. " must inherit MiddleTexture")
    expect(input.Label ~= nil, "Input" .. index .. " must inherit Label")
    expect(input:GetScript("OnEnterPressed") == AuthChallengeUI_Submit,
           "Input" .. index .. " must inherit submit OnEnterPressed")
    expect(input:GetScript("OnTabPressed") == AuthChallengeUI_OnTabPressed,
           "Input" .. index .. " must inherit tab OnTabPressed")
  end
end

return table.concat(failures, "\n")
"#;

#[test]
fn auth_challenge_ui_input_frame_has_four_template_edit_boxes() {
    common::with_perf_lock(|| {
        common::with_timeout(240, || {
            with_blizzard_addon_smoke_shape(&[], &[], |env, _loaded| {
                clear_recorded_lua_errors(env);

                let (loaded, reason): (bool, Option<String>) = env
                    .eval(r#"return C_AddOns.LoadAddOn("Blizzard_AuthChallengeUI")"#)
                    .expect("C_AddOns.LoadAddOn should return");
                assert!(loaded, "`{ROOT}` should load: {reason:?}");

                let failures: String = env
                    .eval(INPUT_COUNT_PROBE_LUA)
                    .expect("AuthChallengeUI input-count probe should run");
                assert!(
                    failures.is_empty(),
                    "`{ROOT}` InputFrame shape mismatches:\n{failures}"
                );

                let errors = recorded_lua_errors(env);
                assert!(
                    errors.is_empty(),
                    "`{ROOT}` input-count probe emitted Lua errors:\n{}",
                    errors.join("\n")
                );
            });
        });
    });
}
