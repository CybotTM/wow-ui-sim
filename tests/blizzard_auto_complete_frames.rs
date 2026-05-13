use crate::common;

use common::blizzard_addon_harness::with_blizzard_addon_smoke_shape;

const ROOT: &str = "Blizzard_AutoComplete";
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

expectObjectType(AutoCompleteBox, "Frame", "AutoCompleteBox")
if AutoCompleteBox == nil then
  return table.concat(failures, "\n")
end

expect(not AutoCompleteBox:IsShown(), "AutoCompleteBox must start hidden")
expect(AutoCompleteBox:IsMouseEnabled(), "AutoCompleteBox must enable mouse")
expect(AutoCompleteBox.NineSlice ~= nil,
       "AutoCompleteBox must inherit TooltipBackdropTemplate NineSlice child")
expect(type(AutoCompleteBox.TooltipBackdropOnLoad) == "function",
       "AutoCompleteBox must inherit TooltipBackdropTemplate mixin methods")

expectObjectType(AutoCompleteInstructions, "FontString", "AutoCompleteInstructions")

local buttonCount = 0
for _, child in ipairs({ AutoCompleteBox:GetChildren() }) do
  if child:GetObjectType() == "Button" then
    buttonCount = buttonCount + 1
  end
end
expect(buttonCount == AUTOCOMPLETE_MAX_BUTTONS,
       "AutoCompleteBox must have exactly " .. AUTOCOMPLETE_MAX_BUTTONS ..
       " Button children, got " .. buttonCount)

for index = 1, AUTOCOMPLETE_MAX_BUTTONS do
  local buttonName = "AutoCompleteButton" .. index
  local button = _G[buttonName]
  expectObjectType(button, "Button", buttonName)
  if button ~= nil then
    expect(button:GetParent() == AutoCompleteBox,
           buttonName .. " must be parented to AutoCompleteBox")
  end
end

expect(_G["AutoCompleteButton" .. (AUTOCOMPLETE_MAX_BUTTONS + 1)] == nil,
       "AutoCompleteButton" .. (AUTOCOMPLETE_MAX_BUTTONS + 1) .. " must not exist")

return table.concat(failures, "\n")
"#;

#[test]
fn blizzard_auto_complete_frame_surface_matches_xml_shape() {
    common::with_perf_lock(|| {
        common::with_timeout(240, || {
            with_blizzard_addon_smoke_shape(&[ROOT], &[], |env, loaded| {
                assert!(
                    loaded.iter().any(|name| name == ROOT),
                    "`{ROOT}` must load before frame globals can be checked. \
                     Loaded set: {loaded:?}"
                );

                let failures: String = env
                    .eval(FRAME_SURFACE_PROBE_LUA)
                    .expect("AutoComplete frame-surface probe should run");
                assert!(
                    failures.is_empty(),
                    "`{ROOT}` frame-surface mismatches:\n{failures}"
                );
            });
        });
    });
}
