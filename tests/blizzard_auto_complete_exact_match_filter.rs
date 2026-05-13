use crate::common;

use common::blizzard_addon_harness::with_blizzard_addon_smoke_shape;

const ROOT: &str = "Blizzard_AutoComplete";
const EXACT_MATCH_PROBE_LUA: &str = r#"
local failures = {}

local function expect(condition, message)
  if not condition then
    table.insert(failures, message)
  end
end

local originalUpdateResults = AutoComplete_UpdateResults
local capturedResults

AutoComplete_UpdateResults = function(self, results)
  capturedResults = results
end

local function makeEditBox(sourceFn)
  local editBox = CreateFrame("EditBox", nil, UIParent)
  editBox:SetSize(200, 20)
  editBox:SetPoint("CENTER", UIParent, "CENTER", 0, 0)
  AutoCompleteEditBox_SetAutoCompleteSource(editBox, sourceFn)
  return editBox
end

local exactOnlyEditBox = makeEditBox(function()
  return {
    { name = "Alice", priority = LE_AUTOCOMPLETE_PRIORITY_OTHER },
  }
end)

AutoComplete_Update(exactOnlyEditBox, "Alice", strlen("Alice"))
expect(capturedResults[1] == nil,
       "single exact match must be dropped from possibilities")
expect(#capturedResults == 0, "single exact-match result list must be empty")

local mixedEditBox = makeEditBox(function()
  return {
    { name = "Alice", priority = LE_AUTOCOMPLETE_PRIORITY_OTHER },
    { name = "Alicia", priority = LE_AUTOCOMPLETE_PRIORITY_OTHER },
  }
end)

capturedResults = nil
AutoComplete_Update(mixedEditBox, "Alice", strlen("Alice"))

expect(#capturedResults == 2, "exact match must be preserved when multiple results exist")
expect(capturedResults[1].name == "Alice",
       "first multiple-result entry must remain the exact match")
expect(capturedResults[2].name == "Alicia",
       "second multiple-result entry must remain present")

AutoComplete_UpdateResults = originalUpdateResults

return table.concat(failures, "\n")
"#;

#[test]
fn blizzard_auto_complete_filters_single_exact_match_only() {
    common::with_perf_lock(|| {
        common::with_timeout(240, || {
            with_blizzard_addon_smoke_shape(&[ROOT], &[], |env, loaded| {
                assert!(
                    loaded.iter().any(|name| name == ROOT),
                    "`{ROOT}` must load before AutoComplete exact-match filtering can be checked. \
                     Loaded set: {loaded:?}"
                );

                let failures: String = env
                    .eval(EXACT_MATCH_PROBE_LUA)
                    .expect("AutoComplete exact-match probe should run");
                assert!(
                    failures.is_empty(),
                    "`{ROOT}` exact-match filtering mismatches:\n{failures}"
                );
            });
        });
    });
}
