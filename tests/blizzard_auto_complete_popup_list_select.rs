use crate::common;

use common::blizzard_addon_harness::with_blizzard_addon_smoke_shape;

const ROOT: &str = "Blizzard_AutoCompletePopupList";
const SELECT_PROBE_LUA: &str = r#"
local failures = {}

local function expect(condition, message)
  if not condition then
    table.insert(failures, message)
  end
end

local popup = CreateFrame("Frame", "TestPopupListSelectFrame", UIParent,
                         "AutoCompletePopupListTemplate")
expect(popup ~= nil, "AutoCompletePopupListTemplate must instantiate")

if popup ~= nil then
  local firstResult = { text = "First" }
  local secondResult = { text = "Second" }
  local callbackCount = 0
  local callbackResult = nil

  popup:OnLoad()
  popup:SetSize(176, 100)
  popup.ScrollBox:SetSize(176, 100)
  popup.resultsListCallback = function()
    return 2, { firstResult, secondResult }, nil
  end
  popup:SetSelectResultCallback(function(resultInfo)
    callbackCount = callbackCount + 1
    callbackResult = resultInfo
  end)

  popup:UpdateResults()
  popup:HighlightResult(1)

  local selected = popup:SelectHighlightedResult()
  expect(selected == true, "SelectHighlightedResult must return true")
  expect(callbackCount == 1,
         "selectResultCallback must be called once, got " ..
         tostring(callbackCount))
  expect(callbackResult == firstResult,
         "selectResultCallback must receive the highlighted result table")

  popup.highlightedIndex = 0
  local unhighlightedSelected = popup:SelectHighlightedResult()
  expect(unhighlightedSelected == false,
         "SelectHighlightedResult without highlight must return false")
  expect(callbackCount == 1,
         "SelectHighlightedResult without highlight must not call callback")
end

local emptyPopup = CreateFrame("Frame", "TestPopupListSelectEmptyFrame",
                              UIParent, "AutoCompletePopupListTemplate")
expect(emptyPopup ~= nil, "AutoCompletePopupListTemplate must instantiate again")

if emptyPopup ~= nil then
  local callbackCount = 0
  emptyPopup:OnLoad()
  emptyPopup:SetSelectResultCallback(function()
    callbackCount = callbackCount + 1
  end)
  emptyPopup.resultsListCallback = function()
    return 0, {}, nil
  end

  emptyPopup:UpdateResults()
  emptyPopup.highlightedIndex = 1

  local selected = emptyPopup:SelectHighlightedResult()
  expect(selected == false,
         "SelectHighlightedResult without results must return false")
  expect(callbackCount == 0,
         "SelectHighlightedResult without results must not call callback")
end

return table.concat(failures, "\n")
"#;

#[test]
fn blizzard_auto_complete_popup_list_selects_highlighted_result() {
    common::with_perf_lock(|| {
        common::with_timeout(240, || {
            with_blizzard_addon_smoke_shape(&[ROOT], &[], |env, loaded| {
                assert!(
                    loaded.iter().any(|name| name == ROOT),
                    "`{ROOT}` must load before PopupList selection can be checked. \
                     Loaded set: {loaded:?}"
                );

                let failures: String = env
                    .eval(SELECT_PROBE_LUA)
                    .expect("AutoCompletePopupList select probe should run");
                assert!(
                    failures.is_empty(),
                    "`{ROOT}` selection mismatches:\n{failures}"
                );
            });
        });
    });
}
