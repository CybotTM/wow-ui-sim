use crate::common;

use common::blizzard_addon_harness::with_blizzard_addon_smoke_shape;

const ROOT: &str = "Blizzard_AutoCompletePopupList";
const MAX_ENTRIES_PROBE_LUA: &str = r#"
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

local popup = CreateFrame("Frame", "TestPopupListMaxEntriesFrame", UIParent,
                         "AutoCompletePopupListTemplate")
expect(popup:GetMaximumEntries() == 5,
       "GetMaximumEntries must default to 5, got " ..
       tostring(popup:GetMaximumEntries()))

popup:OnLoad()
popup.maximumEntries = 3
popup.resultsListCallback = function()
  return 7, resultList(7), nil
end

popup:UpdateResults()

local expectedOverflow = TALENT_FRAME_SEARCH_PREVIEW_OVERFLOW_FORMAT:format(4)
expect(popup.ScrollBox:GetDataProviderSize() == 4,
       "Blizzard SetResults renders maximumEntries + 1 rows, got " ..
       tostring(popup.ScrollBox:GetDataProviderSize()))
expect(popup.OverflowCount:IsShown(),
       "OverflowCount must show when 7 results exceed maximumEntries=3")
expect(popup.OverflowCount.Text:GetText() == expectedOverflow,
       "OverflowCount text must be " .. expectedOverflow .. ", got " ..
       tostring(popup.OverflowCount.Text:GetText()))

return table.concat(failures, "\n")
"#;

#[test]
fn blizzard_auto_complete_popup_list_max_entries_controls_render_limit() {
    common::with_perf_lock(|| {
        common::with_timeout(240, || {
            with_blizzard_addon_smoke_shape(&[ROOT], &[], |env, loaded| {
                assert!(
                    loaded.iter().any(|name| name == ROOT),
                    "`{ROOT}` must load before PopupList max entries can be checked. \
                     Loaded set: {loaded:?}"
                );

                let failures: String = env
                    .eval(MAX_ENTRIES_PROBE_LUA)
                    .expect("AutoCompletePopupList max entries probe should run");
                assert!(
                    failures.is_empty(),
                    "`{ROOT}` max entries mismatches:\n{failures}"
                );
            });
        });
    });
}
