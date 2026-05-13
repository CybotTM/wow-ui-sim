use crate::common;

use common::blizzard_addon_harness::with_blizzard_addon_smoke_shape;

const ROOT: &str = "Blizzard_AutoCompletePopupList";
const ROW_CLICK_PROBE_LUA: &str = r#"
local failures = {}

local function expect(condition, message)
  if not condition then
    table.insert(failures, message)
  end
end

local popup = CreateFrame("Frame", "TestPopupListRowClickOwner", UIParent,
                         "AutoCompletePopupListTemplate")
expect(popup ~= nil, "AutoCompletePopupListTemplate must instantiate")

if popup ~= nil then
  local row = CreateFrame("Button", "TestPopupListRowClickButton", popup,
                          "AutoCompletePopupListResultTemplate")
  expect(row ~= nil, "AutoCompletePopupListResultTemplate must instantiate")

  if row ~= nil then
    local resultInfo = { text = "Clicked" }
    local callbackCount = 0
    local callbackResult = nil

    popup:SetSelectResultCallback(function(selectedResultInfo)
      callbackCount = callbackCount + 1
      callbackResult = selectedResultInfo
    end)

    row:Init({
      resultInfo = resultInfo,
      index = 1,
      owner = popup,
      displayText = "Clicked",
      subtext = nil,
      displayTexture = nil,
    })
    row:OnClick()

    expect(callbackCount == 1,
           "row OnClick must invoke selectResultCallback once, got " ..
           tostring(callbackCount))
    expect(callbackResult == resultInfo,
           "row OnClick must forward row resultInfo table")
  end
end

return table.concat(failures, "\n")
"#;

#[test]
fn blizzard_auto_complete_popup_list_row_click_selects_result() {
    common::with_perf_lock(|| {
        common::with_timeout(240, || {
            with_blizzard_addon_smoke_shape(&[ROOT], &[], |env, loaded| {
                assert!(
                    loaded.iter().any(|name| name == ROOT),
                    "`{ROOT}` must load before PopupList row click can be checked. \
                     Loaded set: {loaded:?}"
                );

                let failures: String = env
                    .eval(ROW_CLICK_PROBE_LUA)
                    .expect("AutoCompletePopupList row click probe should run");
                assert!(
                    failures.is_empty(),
                    "`{ROOT}` row click mismatches:\n{failures}"
                );
            });
        });
    });
}
