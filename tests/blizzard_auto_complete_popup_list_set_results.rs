mod common;

use common::blizzard_addon_harness::with_blizzard_addon_smoke_shape;

const ROOT: &str = "Blizzard_AutoCompletePopupList";
const SET_RESULTS_PROBE_LUA: &str = r#"
local failures = {}

local function expect(condition, message)
  if not condition then
    table.insert(failures, message)
  end
end

local popup = CreateFrame("Frame", "TestPopupListSetResultsFrame", UIParent,
                         "AutoCompletePopupListTemplate")
expect(popup ~= nil, "AutoCompletePopupListTemplate must instantiate")

if popup ~= nil then
  popup:OnLoad()
  popup.resultsListCallback = function()
    return 3, {
      { text = "A" },
      { text = "B" },
      { text = "C" },
    }, nil
  end

  popup:UpdateResults()

  expect(popup:HasResults(), "popup must have results after UpdateResults")
  expect(popup:IsShown(), "popup must be shown after non-empty results")
  expect(popup.highlightedIndex == 1,
         "popup highlightedIndex must default to 1, got " ..
         tostring(popup.highlightedIndex))
  expect(popup.ScrollBox:GetDataProviderSize() == 3,
         "popup ScrollBox data provider size must be 3, got " ..
         tostring(popup.ScrollBox:GetDataProviderSize()))
end

return table.concat(failures, "\n")
"#;

#[test]
fn blizzard_auto_complete_popup_list_update_results_shows_non_empty_results() {
    common::with_perf_lock(|| {
        common::with_timeout(240, || {
            with_blizzard_addon_smoke_shape(&[ROOT], &[], |env, loaded| {
                assert!(
                    loaded.iter().any(|name| name == ROOT),
                    "`{ROOT}` must load before PopupList results can be checked. \
                     Loaded set: {loaded:?}"
                );

                let failures: String = env
                    .eval(SET_RESULTS_PROBE_LUA)
                    .expect("AutoCompletePopupList set-results probe should run");
                assert!(
                    failures.is_empty(),
                    "`{ROOT}` set-results mismatches:\n{failures}"
                );
            });
        });
    });
}
