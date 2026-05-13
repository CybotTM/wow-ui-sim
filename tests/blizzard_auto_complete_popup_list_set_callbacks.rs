use crate::common;

use common::blizzard_addon_harness::with_blizzard_addon_smoke_shape;

const ROOT: &str = "Blizzard_AutoCompletePopupList";
const SET_CALLBACKS_PROBE_LUA: &str = r#"
local failures = {}

local function expect(condition, message)
  if not condition then
    table.insert(failures, message)
  end
end

local popup = CreateFrame("Frame", "TestPopupListCallbacksFrame", UIParent,
                         "AutoCompletePopupListTemplate")
expect(popup ~= nil, "AutoCompletePopupListTemplate must instantiate")

if popup ~= nil then
  popup:OnLoad()

  local selectCallback = function() end
  popup:SetSelectResultCallback(selectCallback)
  expect(popup.selectResultCallback == selectCallback,
         "SetSelectResultCallback must store the callback on the popup")

  local updateCalls = 0
  local originalUpdateResults = popup.UpdateResults
  popup.UpdateResults = function(self)
    updateCalls = updateCalls + 1
    return originalUpdateResults(self)
  end

  local resultsListCallback = function()
    return 0, {}
  end

  popup:SetResultsListCallback(resultsListCallback)
  expect(popup.resultsListCallback == resultsListCallback,
         "SetResultsListCallback must store the callback on the popup")
  expect(updateCalls == 1,
         "SetResultsListCallback must call UpdateResults immediately, got " ..
         tostring(updateCalls))
end

return table.concat(failures, "\n")
"#;

#[test]
fn blizzard_auto_complete_popup_list_set_callbacks_store_and_update() {
    common::with_perf_lock(|| {
        common::with_timeout(240, || {
            with_blizzard_addon_smoke_shape(&[ROOT], &[], |env, loaded| {
                assert!(
                    loaded.iter().any(|name| name == ROOT),
                    "`{ROOT}` must load before PopupList callback setters can be checked. \
                     Loaded set: {loaded:?}"
                );

                let failures: String = env
                    .eval(SET_CALLBACKS_PROBE_LUA)
                    .expect("AutoCompletePopupList callback setter probe should run");
                assert!(
                    failures.is_empty(),
                    "`{ROOT}` callback setter mismatches:\n{failures}"
                );
            });
        });
    });
}
