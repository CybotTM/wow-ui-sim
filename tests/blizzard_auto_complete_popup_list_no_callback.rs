use crate::common;

use common::blizzard_addon_harness::with_blizzard_addon_smoke_shape;

const ROOT: &str = "Blizzard_AutoCompletePopupList";
const NO_CALLBACK_PROBE_LUA: &str = r#"
local failures = {}

local function expect(condition, message)
  if not condition then
    table.insert(failures, message)
  end
end

local popup = CreateFrame("Frame", "TestPopupListNoDisplayCallbackFrame",
                         UIParent, "AutoCompletePopupListTemplate")
expect(popup ~= nil, "AutoCompletePopupListTemplate must instantiate")

if popup ~= nil then
  popup:OnLoad()
  popup:SetSize(176, 100)
  popup.ScrollBox:SetSize(176, 100)
  popup.resultsListCallback = function()
    return 2, {
      { text = "Alpha" },
      { text = "Beta" },
    }, nil
  end

  popup:UpdateResults()

  local expectedNames = { "Alpha", "Beta" }
  local rowCount = 0
  popup.ScrollBox:ForEachFrame(function(row)
    rowCount = rowCount + 1
    expect(row.Name:GetText() == expectedNames[rowCount],
           "row " .. tostring(rowCount) .. " Name must fall back to " ..
           tostring(expectedNames[rowCount]) .. ", got " ..
           tostring(row.Name:GetText()))
  end)
  expect(rowCount == 2,
         "no-display-callback popup must initialize 2 rows, got " ..
         tostring(rowCount))
end

return table.concat(failures, "\n")
"#;

#[test]
fn blizzard_auto_complete_popup_list_uses_result_text_without_display_callback() {
    common::with_perf_lock(|| {
        common::with_timeout(240, || {
            with_blizzard_addon_smoke_shape(&[ROOT], &[], |env, loaded| {
                assert!(
                    loaded.iter().any(|name| name == ROOT),
                    "`{ROOT}` must load before PopupList no-callback fallback can be checked. \
                     Loaded set: {loaded:?}"
                );

                let failures: String = env
                    .eval(NO_CALLBACK_PROBE_LUA)
                    .expect("AutoCompletePopupList no-display-callback probe should run");
                assert!(
                    failures.is_empty(),
                    "`{ROOT}` no-display-callback mismatches:\n{failures}"
                );
            });
        });
    });
}
