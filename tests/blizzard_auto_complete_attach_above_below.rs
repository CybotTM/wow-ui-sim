use crate::common;

use common::blizzard_addon_harness::with_blizzard_addon_smoke_shape;

const ROOT: &str = "Blizzard_AutoComplete";
const ATTACH_PROBE_LUA: &str = r#"
local failures = {}

local function expect(condition, message)
  if not condition then
    table.insert(failures, message)
  end
end

local function sourceFn()
  return {
    { name = "able", priority = LE_AUTOCOMPLETE_PRIORITY_OTHER },
    { name = "about", priority = LE_AUTOCOMPLETE_PRIORITY_OTHER },
  }
end

local function pointSummary()
  local point, relativeTo, relativePoint, xOfs, yOfs = AutoCompleteBox:GetPoint(1)
  return point, relativeTo, relativePoint, xOfs, yOfs
end

local function assertAnchor(parent, expectedAttach, expectedPoint, expectedRelativePoint,
                            expectedYOffset)
  local point, relativeTo, relativePoint, xOfs, yOfs = pointSummary()
  expect(AutoCompleteBox.attachPoint == expectedAttach,
         "attachPoint must be " .. expectedAttach .. ", got " .. tostring(AutoCompleteBox.attachPoint))
  expect(point == expectedPoint,
         "anchor point must be " .. expectedPoint .. ", got " .. tostring(point))
  expect(relativeTo == parent, "anchor relative frame must be the editBox")
  expect(relativePoint == expectedRelativePoint,
         "relative point must be " .. expectedRelativePoint .. ", got " .. tostring(relativePoint))
  expect(xOfs == 0, "x offset must be 0, got " .. tostring(xOfs))
  expect(yOfs == expectedYOffset,
         "y offset must be " .. tostring(expectedYOffset) .. ", got " .. tostring(yOfs))
end

local function makeEditBox(name, bottom)
  local editBox = CreateFrame("EditBox", name, UIParent)
  editBox:SetSize(200, 20)
  editBox:SetPoint("BOTTOMLEFT", UIParent, "BOTTOMLEFT", 50, bottom)
  AutoCompleteEditBox_SetAutoCompleteSource(editBox, sourceFn)
  return editBox
end

AutoComplete_OnLoad(AutoCompleteBox)

local originalSetPoint = AutoCompleteBox.SetPoint
local setPointCalls = 0
AutoCompleteBox.SetPoint = function(self, ...)
  setPointCalls = setPointCalls + 1
  return originalSetPoint(self, ...)
end

local bottomEditBox = makeEditBox("AutoCompleteBottomAttachEditBox", 4)
expect(bottomEditBox:GetBottom() - AutoCompleteBox.maxHeight <= AUTOCOMPLETE_DEFAULT_Y_OFFSET + 10,
       "bottom editBox must exercise ABOVE branch")

AutoComplete_Update(bottomEditBox, "ab", 2)
assertAnchor(bottomEditBox, "ABOVE", "BOTTOMLEFT", "TOPLEFT", -AUTOCOMPLETE_DEFAULT_Y_OFFSET)
expect(setPointCalls == 1, "first ABOVE update must anchor once")

AutoComplete_Update(bottomEditBox, "ab", 2)
expect(setPointCalls == 1, "second ABOVE update with same parent and attachPoint must not re-anchor")

AutoComplete_HideIfAttachedTo(bottomEditBox)

local topEditBox = makeEditBox("AutoCompleteTopAttachEditBox", 700)
expect(topEditBox:GetBottom() - AutoCompleteBox.maxHeight > AUTOCOMPLETE_DEFAULT_Y_OFFSET + 10,
       "top editBox must exercise BELOW branch")

AutoComplete_Update(topEditBox, "ab", 2)
assertAnchor(topEditBox, "BELOW", "TOPLEFT", "BOTTOMLEFT", AUTOCOMPLETE_DEFAULT_Y_OFFSET)
expect(setPointCalls == 2, "first BELOW update must anchor once")

AutoComplete_Update(topEditBox, "ab", 2)
expect(setPointCalls == 2, "second BELOW update with same parent and attachPoint must not re-anchor")

AutoCompleteBox.SetPoint = originalSetPoint

return table.concat(failures, "\n")
"#;

#[test]
fn blizzard_auto_complete_attaches_above_or_below_parent() {
    common::with_perf_lock(|| {
        common::with_timeout(240, || {
            with_blizzard_addon_smoke_shape(&[ROOT], &[], |env, loaded| {
                assert!(
                    loaded.iter().any(|name| name == ROOT),
                    "`{ROOT}` must load before AutoComplete attachment can be checked. \
                     Loaded set: {loaded:?}"
                );

                let failures: String = env
                    .eval(ATTACH_PROBE_LUA)
                    .expect("AutoComplete attach probe should run");
                assert!(
                    failures.is_empty(),
                    "`{ROOT}` attachment mismatches:\n{failures}"
                );
            });
        });
    });
}
