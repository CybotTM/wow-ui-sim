use crate::common;

use common::blizzard_addon_harness::with_blizzard_addon_smoke_shape;

const ROOT: &str = "Blizzard_AutoCompletePopupList";
const SELECT_DIRECT_PROBE_LUA: &str = r#"
local failures = {}

local function expect(condition, message)
  if not condition then
    table.insert(failures, message)
  end
end

local popup = CreateFrame("Frame", "TestPopupListSelectDirectFrame", UIParent,
                         "AutoCompletePopupListTemplate")
expect(popup ~= nil, "AutoCompletePopupListTemplate must instantiate")

if popup ~= nil then
  local resultInfo = { text = "Direct" }
  local callbackCount = 0
  local callbackResult = nil

  popup:SetSelectResultCallback(function(selectedResultInfo)
    callbackCount = callbackCount + 1
    callbackResult = selectedResultInfo
  end)

  popup:SelectResult(resultInfo)

  expect(callbackCount == 1,
         "SelectResult must call configured callback once, got " ..
         tostring(callbackCount))
  expect(callbackResult == resultInfo,
         "SelectResult must forward the resultInfo table")
end

local noCallbackPopup = CreateFrame("Frame", "TestPopupListSelectNoCallbackFrame",
                                   UIParent, "AutoCompletePopupListTemplate")
expect(noCallbackPopup ~= nil,
       "AutoCompletePopupListTemplate must instantiate no-callback popup")

if noCallbackPopup ~= nil then
  local ok, errorMessage = pcall(function()
    noCallbackPopup:SelectResult({ text = "No callback" })
  end)
  expect(ok, "SelectResult without callback must be a silent no-op, got " ..
             tostring(errorMessage))
end

return table.concat(failures, "\n")
"#;

#[test]
fn blizzard_auto_complete_popup_list_select_result_invokes_callback_directly() {
    common::with_perf_lock(|| {
        common::with_timeout(240, || {
            with_blizzard_addon_smoke_shape(&[ROOT], &[], |env, loaded| {
                assert!(
                    loaded.iter().any(|name| name == ROOT),
                    "`{ROOT}` must load before PopupList direct selection can be checked. \
                     Loaded set: {loaded:?}"
                );

                let failures: String = env
                    .eval(SELECT_DIRECT_PROBE_LUA)
                    .expect("AutoCompletePopupList direct-select probe should run");
                assert!(
                    failures.is_empty(),
                    "`{ROOT}` direct selection mismatches:\n{failures}"
                );
            });
        });
    });
}
