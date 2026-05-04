mod common;

use common::blizzard_addon_harness::with_blizzard_addon_smoke_shape;

const ROOT: &str = "Blizzard_AutoCompletePopupList";
const TEMPLATE_PROBE_LUA: &str = r#"
local failures = {}

local function expect(condition, message)
  if not condition then
    table.insert(failures, message)
  end
end

local function expectObjectType(object, expectedType, message)
  expect(object ~= nil, message .. " must exist")
  if object ~= nil then
    expect(object:GetObjectType() == expectedType,
           message .. " must be a " .. expectedType ..
           ", got " .. tostring(object:GetObjectType()))
  end
end

local popup = CreateFrame("Frame", "TestPopupListTemplateFrame", UIParent,
                         "AutoCompletePopupListTemplate")
expect(popup ~= nil, "AutoCompletePopupListTemplate must instantiate")

if popup ~= nil then
  expect(popup:GetObjectType() == "Frame",
         "AutoCompletePopupListTemplate must instantiate a Frame")
  expect(popup.maximumEntries == 5,
         "AutoCompletePopupListTemplate maximumEntries must be 5, got " ..
         tostring(popup.maximumEntries))
  expectObjectType(popup.ScrollBox, "Frame", "popup.ScrollBox")
  expectObjectType(popup.OverflowCount, "Frame", "popup.OverflowCount")
  expectObjectType(popup.Background, "Texture", "popup.Background")
  expectObjectType(popup.BorderAnchor, "Texture", "popup.BorderAnchor")
  expectObjectType(popup.BotRightCorner, "Texture", "popup.BotRightCorner")
  expectObjectType(popup.BottomBorder, "Texture", "popup.BottomBorder")
  expectObjectType(popup.LeftBorder, "Texture", "popup.LeftBorder")
  expectObjectType(popup.RightBorder, "Texture", "popup.RightBorder")

  if popup.OverflowCount ~= nil then
    expect(not popup.OverflowCount:IsShown(),
           "popup.OverflowCount must be hidden by default")
    expectObjectType(popup.OverflowCount.Text, "FontString",
                     "popup.OverflowCount.Text")
  end
end

local row = CreateFrame("Button", "TestPopupListResultTemplateButton", UIParent,
                        "AutoCompletePopupListResultTemplate")
expect(row ~= nil, "AutoCompletePopupListResultTemplate must instantiate")

if row ~= nil then
  expect(row:GetObjectType() == "Button",
         "AutoCompletePopupListResultTemplate must instantiate a Button")
end

return table.concat(failures, "\n")
"#;

#[test]
fn blizzard_auto_complete_popup_list_templates_register_with_defaults() {
    common::with_perf_lock(|| {
        common::with_timeout(240, || {
            with_blizzard_addon_smoke_shape(&[ROOT], &[], |env, loaded| {
                assert!(
                    loaded.iter().any(|name| name == ROOT),
                    "`{ROOT}` must load before PopupList templates can be checked. \
                     Loaded set: {loaded:?}"
                );

                let failures: String = env
                    .eval(TEMPLATE_PROBE_LUA)
                    .expect("AutoCompletePopupList template probe should run");
                assert!(
                    failures.is_empty(),
                    "`{ROOT}` template mismatches:\n{failures}"
                );
            });
        });
    });
}
