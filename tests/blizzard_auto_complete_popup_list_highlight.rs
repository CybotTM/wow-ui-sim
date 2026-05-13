use crate::common;

use common::blizzard_addon_harness::with_blizzard_addon_smoke_shape;

const ROOT: &str = "Blizzard_AutoCompletePopupList";
const HIGHLIGHT_PROBE_LUA: &str = r#"
local failures = {}

local function expect(condition, message)
  if not condition then
    table.insert(failures, message)
  end
end

local popup = CreateFrame("Frame", "TestPopupListHighlightFrame", UIParent,
                         "AutoCompletePopupListTemplate")
expect(popup ~= nil, "AutoCompletePopupListTemplate must instantiate")

if popup ~= nil then
  popup:OnLoad()
  popup:SetSize(176, 100)
  popup.ScrollBox:SetSize(176, 100)
  popup.resultsListCallback = function()
    return 3, {
      { text = "One" },
      { text = "Two" },
      { text = "Three" },
    }, nil
  end

  popup:UpdateResults()
  popup:HighlightResult(2)

  expect(popup.highlightedIndex == 2,
         "popup highlightedIndex must be 2, got " ..
         tostring(popup.highlightedIndex))

  local rowCount = 0
  popup.ScrollBox:ForEachFrame(function(row)
    rowCount = rowCount + 1
    local shouldHighlight = row:GetIndex() == 2
    expect(row.HighlightTexture:IsShown() == shouldHighlight,
           "row " .. tostring(row:GetIndex()) ..
           " HighlightTexture shown state mismatch")
  end)
  expect(rowCount == 3, "highlight popup must initialize 3 rows, got " ..
                       tostring(rowCount))

  popup:HighlightResult(0)
  expect(popup.highlightedIndex == 3,
         "HighlightResult(0) must wrap highlightedIndex to 3, got " ..
         tostring(popup.highlightedIndex))

  popup.ScrollBox:ForEachFrame(function(row)
    local shouldHighlight = row:GetIndex() == 3
    expect(row.HighlightTexture:IsShown() == shouldHighlight,
           "row " .. tostring(row:GetIndex()) ..
           " HighlightTexture shown state mismatch after HighlightResult(0)")
  end)

  popup:HighlightResult(5)
  expect(popup.highlightedIndex == 2,
         "HighlightResult(5) must wrap highlightedIndex to 2, got " ..
         tostring(popup.highlightedIndex))

  popup.ScrollBox:ForEachFrame(function(row)
    local shouldHighlight = row:GetIndex() == 2
    expect(row.HighlightTexture:IsShown() == shouldHighlight,
           "row " .. tostring(row:GetIndex()) ..
           " HighlightTexture shown state mismatch after HighlightResult(5)")
  end)
end

local emptyPopup = CreateFrame("Frame", "TestPopupListHighlightEmptyFrame",
                              UIParent, "AutoCompletePopupListTemplate")
expect(emptyPopup ~= nil, "AutoCompletePopupListTemplate must instantiate again")

if emptyPopup ~= nil then
  emptyPopup:OnLoad()
  emptyPopup.resultsListCallback = function()
    return 0, {}, nil
  end

  emptyPopup:UpdateResults()
  emptyPopup:HighlightResult(1)

  expect(emptyPopup.highlightedIndex == 0,
         "HighlightResult with no results must leave highlightedIndex at 0")
  expect(not emptyPopup:HasResults(),
         "HighlightResult with no results must not create results")
end

return table.concat(failures, "\n")
"#;

#[test]
fn blizzard_auto_complete_popup_list_highlight_selects_one_row() {
    common::with_perf_lock(|| {
        common::with_timeout(240, || {
            with_blizzard_addon_smoke_shape(&[ROOT], &[], |env, loaded| {
                assert!(
                    loaded.iter().any(|name| name == ROOT),
                    "`{ROOT}` must load before PopupList highlight can be checked. \
                     Loaded set: {loaded:?}"
                );

                let failures: String = env
                    .eval(HIGHLIGHT_PROBE_LUA)
                    .expect("AutoCompletePopupList highlight probe should run");
                assert!(
                    failures.is_empty(),
                    "`{ROOT}` highlight mismatches:\n{failures}"
                );
            });
        });
    });
}
