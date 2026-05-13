use crate::common;

use common::blizzard_addon_harness::with_blizzard_addon_smoke_shape;

const ROOT: &str = "Blizzard_AutoCompletePopupList";
const CYCLE_PROBE_LUA: &str = r#"
local failures = {}

local function expect(condition, message)
  if not condition then
    table.insert(failures, message)
  end
end

local function expectHighlightedIndex(popup, expected, label)
  expect(popup.highlightedIndex == expected,
         label .. " must set highlightedIndex to " .. tostring(expected) ..
         ", got " .. tostring(popup.highlightedIndex))
end

local popup = CreateFrame("Frame", "TestPopupListCycleFrame", UIParent,
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
  expectHighlightedIndex(popup, 1, "UpdateResults")

  popup:CycleHighlightedResultDown()
  expectHighlightedIndex(popup, 2, "first CycleHighlightedResultDown")

  popup:CycleHighlightedResultDown()
  expectHighlightedIndex(popup, 3, "second CycleHighlightedResultDown")

  popup:CycleHighlightedResultDown()
  expectHighlightedIndex(popup, 1, "third CycleHighlightedResultDown")

  popup:CycleHighlightedResultUp()
  expectHighlightedIndex(popup, 3, "first CycleHighlightedResultUp")

  popup:CycleHighlightedResultUp()
  expectHighlightedIndex(popup, 2, "second CycleHighlightedResultUp")
end

return table.concat(failures, "\n")
"#;

#[test]
fn blizzard_auto_complete_popup_list_cycles_highlight_with_wrap() {
    common::with_perf_lock(|| {
        common::with_timeout(240, || {
            with_blizzard_addon_smoke_shape(&[ROOT], &[], |env, loaded| {
                assert!(
                    loaded.iter().any(|name| name == ROOT),
                    "`{ROOT}` must load before PopupList highlight cycling can be checked. \
                     Loaded set: {loaded:?}"
                );

                let failures: String = env
                    .eval(CYCLE_PROBE_LUA)
                    .expect("AutoCompletePopupList cycle probe should run");
                assert!(
                    failures.is_empty(),
                    "`{ROOT}` highlight cycle mismatches:\n{failures}"
                );
            });
        });
    });
}
