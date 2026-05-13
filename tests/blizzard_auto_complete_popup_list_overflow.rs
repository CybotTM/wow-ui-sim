use crate::common;

use common::blizzard_addon_harness::with_blizzard_addon_smoke_shape;

const ROOT: &str = "Blizzard_AutoCompletePopupList";
const OVERFLOW_PROBE_LUA: &str = r#"
local failures = {}

local function expect(condition, message)
  if not condition then
    table.insert(failures, message)
  end
end

local function resultList(count)
  local results = {}
  for index = 1, count do
    results[index] = { text = "Result " .. tostring(index) }
  end
  return results
end

local overflowPopup = CreateFrame("Frame", "TestPopupListOverflowFrame",
                                  UIParent, "AutoCompletePopupListTemplate")
expect(overflowPopup ~= nil, "AutoCompletePopupListTemplate must instantiate")

if overflowPopup ~= nil then
  overflowPopup:OnLoad()
  overflowPopup.resultsListCallback = function()
    return 12, resultList(12), nil
  end

  overflowPopup:UpdateResults()

  local expectedOverflow = TALENT_FRAME_SEARCH_PREVIEW_OVERFLOW_FORMAT:format(7)
  expect(overflowPopup.OverflowCount:IsShown(),
         "OverflowCount must be shown when numResults exceeds maximumEntries")
  expect(overflowPopup.OverflowCount.Text:GetText() == expectedOverflow,
         "OverflowCount text must be " .. expectedOverflow .. ", got " ..
         tostring(overflowPopup.OverflowCount.Text:GetText()))
end

local boundedPopup = CreateFrame("Frame", "TestPopupListBoundedFrame",
                                UIParent, "AutoCompletePopupListTemplate")
expect(boundedPopup ~= nil, "AutoCompletePopupListTemplate must instantiate again")

if boundedPopup ~= nil then
  boundedPopup:OnLoad()
  boundedPopup.resultsListCallback = function()
    return 5, resultList(5), nil
  end

  boundedPopup:UpdateResults()

  expect(not boundedPopup.OverflowCount:IsShown(),
         "OverflowCount must stay hidden when numResults fits maximumEntries")
end

return table.concat(failures, "\n")
"#;

#[test]
fn blizzard_auto_complete_popup_list_overflow_count_tracks_remaining_results() {
    common::with_perf_lock(|| {
        common::with_timeout(240, || {
            with_blizzard_addon_smoke_shape(&[ROOT], &[], |env, loaded| {
                assert!(
                    loaded.iter().any(|name| name == ROOT),
                    "`{ROOT}` must load before PopupList overflow can be checked. \
                     Loaded set: {loaded:?}"
                );

                let failures: String = env
                    .eval(OVERFLOW_PROBE_LUA)
                    .expect("AutoCompletePopupList overflow probe should run");
                assert!(
                    failures.is_empty(),
                    "`{ROOT}` overflow mismatches:\n{failures}"
                );
            });
        });
    });
}
