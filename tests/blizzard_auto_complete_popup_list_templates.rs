use crate::common;

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

local function expectAnchor(region, index, expectedPoint, expectedRelativeTo,
                            expectedRelativePoint, expectedX, expectedY, message)
  expect(region:GetNumPoints() >= index,
         message .. " must have anchor #" .. tostring(index))
  if region:GetNumPoints() < index then
    return
  end

  local point, relativeTo, relativePoint, x, y = region:GetPoint(index)
  expect(point == expectedPoint,
         message .. " point must be " .. expectedPoint .. ", got " ..
         tostring(point))
  expect(relativeTo == expectedRelativeTo,
         message .. " relativeTo mismatch")
  expect(relativePoint == expectedRelativePoint,
         message .. " relativePoint must be " .. expectedRelativePoint ..
         ", got " .. tostring(relativePoint))
  expect(x == expectedX,
         message .. " x offset must be " .. tostring(expectedX) ..
         ", got " .. tostring(x))
  expect(y == expectedY,
         message .. " y offset must be " .. tostring(expectedY) ..
         ", got " .. tostring(y))
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

local row = nil
if popup ~= nil then
  row = CreateFrame("Button", "TestPopupListResultTemplateButton", popup,
                    "AutoCompletePopupListResultTemplate")
  expect(row ~= nil, "AutoCompletePopupListResultTemplate must instantiate")
end

if row ~= nil then
  expect(row:GetObjectType() == "Button",
         "AutoCompletePopupListResultTemplate must instantiate a Button")
  expect(row:GetParent() == popup,
         "AutoCompletePopupListResultTemplate instance must be parented to popup")
  expectObjectType(row.HighlightTexture, "Texture", "row.HighlightTexture")
  expectObjectType(row.IconFrame, "Texture", "row.IconFrame")
  expectObjectType(row.Icon, "Texture", "row.Icon")
  expectObjectType(row.Name, "FontString", "row.Name")
  expectObjectType(row.Subtext, "FontString", "row.Subtext")

  if row.HighlightTexture ~= nil then
    expect(not row.HighlightTexture:IsShown(),
           "row.HighlightTexture must be hidden by default")
    expect(row.HighlightTexture:GetNumPoints() == 2,
           "row.HighlightTexture must have exactly 2 anchors")
    expectAnchor(row.HighlightTexture, 1, "TOPLEFT", row, "TOPLEFT", 0, 0,
                 "row.HighlightTexture anchor 1")
    expectAnchor(row.HighlightTexture, 2, "BOTTOMRIGHT", row, "BOTTOMRIGHT",
                 0, 0, "row.HighlightTexture anchor 2")
  end

  if row.IconFrame ~= nil then
    expect(row.IconFrame:GetNumPoints() == 1,
           "row.IconFrame must have exactly 1 anchor")
    expectAnchor(row.IconFrame, 1, "LEFT", row, "LEFT", 5, 1,
                 "row.IconFrame anchor")
  end

  if row.Icon ~= nil and row.IconFrame ~= nil then
    expect(row.Icon:GetNumPoints() == 2,
           "row.Icon must have exactly 2 anchors")
    expectAnchor(row.Icon, 1, "TOPLEFT", row.IconFrame, "TOPLEFT", 1, -1,
                 "row.Icon anchor 1")
    expectAnchor(row.Icon, 2, "BOTTOMRIGHT", row.IconFrame, "BOTTOMRIGHT",
                 -1, 1, "row.Icon anchor 2")
  end

  if row.Name ~= nil and row.Icon ~= nil then
    expect(row.Name:GetNumPoints() == 2,
           "row.Name must have exactly 2 anchors")
    expectAnchor(row.Name, 1, "LEFT", row.Icon, "RIGHT", 5, 1,
                 "row.Name anchor 1")
    expectAnchor(row.Name, 2, "RIGHT", row, "RIGHT", -5, 0,
                 "row.Name anchor 2")
  end

  if row.Subtext ~= nil and row.Name ~= nil then
    expect(row.Subtext:GetNumPoints() == 3,
           "row.Subtext must have exactly 3 anchors")
    expectAnchor(row.Subtext, 1, "TOP", row.Name, "BOTTOM", 0, -2,
                 "row.Subtext anchor 1")
    expectAnchor(row.Subtext, 2, "LEFT", row.Name, "LEFT", 0, 0,
                 "row.Subtext anchor 2")
    expectAnchor(row.Subtext, 3, "RIGHT", row.Name, "RIGHT", 0, 0,
                 "row.Subtext anchor 3")
  end
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
