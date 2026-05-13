use crate::common;

use common::blizzard_addon_harness::with_blizzard_addon_smoke_shape;

const ROOT: &str = "Blizzard_AutoCompletePopupList";
const RESIZE_PROBE_LUA: &str = r#"
local failures = {}

local function expect(condition, message)
  if not condition then
    table.insert(failures, message)
  end
end

local function expectEqual(expected, actual, message)
  expect(expected == actual,
         message .. ": expected " .. tostring(expected) ..
         ", got " .. tostring(actual))
end

local function resultList(count)
  local results = {}
  for index = 1, count do
    results[index] = { text = "Result " .. tostring(index) }
  end
  return results
end

local rowsOnlyPopup = CreateFrame("Frame", "TestPopupListRowsOnlyResize",
                                 UIParent, "AutoCompletePopupListTemplate")
rowsOnlyPopup:OnLoad()
rowsOnlyPopup.resultsListCallback = function()
  return 3, resultList(3), nil
end

rowsOnlyPopup:UpdateResults()

local rowsOnlyView = rowsOnlyPopup.ScrollBox:GetView()
expect(not rowsOnlyPopup.OverflowCount:IsShown(),
       "OverflowCount must stay hidden for 3 results")
expectEqual(rowsOnlyView:GetExtent(), rowsOnlyPopup:GetHeight(),
            "rows-only popup height must match view extent")

local overflowPopup = CreateFrame("Frame", "TestPopupListOverflowResize",
                                 UIParent, "AutoCompletePopupListTemplate")
overflowPopup:OnLoad()
overflowPopup.resultsListCallback = function()
  return 8, resultList(8), nil
end

overflowPopup:UpdateResults()

local overflowView = overflowPopup.ScrollBox:GetView()
local expectedOverflowHeight = overflowView:GetExtent() +
                               overflowPopup.OverflowCount:GetHeight()
expect(overflowPopup.OverflowCount:IsShown(),
       "OverflowCount must show for 8 results with maximumEntries=5")
expectEqual(expectedOverflowHeight, overflowPopup:GetHeight(),
            "overflow popup height must include view extent and overflow height")

return table.concat(failures, "\n")
"#;

#[test]
fn blizzard_auto_complete_popup_list_resize_tracks_rows_and_overflow() {
    common::with_perf_lock(|| {
        common::with_timeout(240, || {
            with_blizzard_addon_smoke_shape(&[ROOT], &[], |env, loaded| {
                assert!(
                    loaded.iter().any(|name| name == ROOT),
                    "`{ROOT}` must load before PopupList resize can be checked. \
                     Loaded set: {loaded:?}"
                );

                let failures: String = env
                    .eval(RESIZE_PROBE_LUA)
                    .expect("AutoCompletePopupList resize probe should run");
                assert!(
                    failures.is_empty(),
                    "`{ROOT}` resize mismatches:\n{failures}"
                );
            });
        });
    });
}
