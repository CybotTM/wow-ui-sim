use crate::common;

use common::blizzard_addon_harness::with_blizzard_addon_smoke_shape;

const ROOT: &str = "Blizzard_AutoCompletePopupList";
const CLEAR_PROBE_LUA: &str = r#"
local failures = {}

local function expect(condition, message)
  if not condition then
    table.insert(failures, message)
  end
end

local popup = CreateFrame("Frame", "TestPopupListClearFrame", UIParent,
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
  expect(popup:HasResults(), "test precondition: popup must have results")
  expect(popup:IsShown(), "test precondition: popup must be shown")

  popup:ClearResults()

  expect(not popup:HasResults(), "ClearResults must remove the data provider")
  expect(popup.highlightedIndex == 0,
         "ClearResults must reset highlightedIndex to 0, got " ..
         tostring(popup.highlightedIndex))
  expect(not popup.OverflowCount:IsShown(),
         "ClearResults must hide OverflowCount")
  expect(not popup:IsShown(), "ClearResults must hide the popup")
end

return table.concat(failures, "\n")
"#;

#[test]
fn blizzard_auto_complete_popup_list_clear_results_resets_visible_state() {
    common::with_perf_lock(|| {
        common::with_timeout(240, || {
            with_blizzard_addon_smoke_shape(&[ROOT], &[], |env, loaded| {
                assert!(
                    loaded.iter().any(|name| name == ROOT),
                    "`{ROOT}` must load before PopupList clearing can be checked. \
                     Loaded set: {loaded:?}"
                );

                let failures: String = env
                    .eval(CLEAR_PROBE_LUA)
                    .expect("AutoCompletePopupList clear probe should run");
                assert!(
                    failures.is_empty(),
                    "`{ROOT}` clear mismatches:\n{failures}"
                );
            });
        });
    });
}
