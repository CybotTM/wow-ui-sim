use crate::common;

use common::blizzard_addon_harness::with_blizzard_addon_smoke_shape;

const ROOT: &str = "Blizzard_AutoComplete";
const UPDATE_RESULTS_PROBE_LUA: &str = r#"
local failures = {}

local function expect(condition, message)
  if not condition then
    table.insert(failures, message)
  end
end

local editBox = CreateFrame("EditBox", nil, UIParent)
editBox:SetSize(200, 20)
editBox:SetPoint("CENTER", UIParent, "CENTER", 0, 0)

local function sourceFn(text, maxResults, cursorPosition, allowFullMatch)
  expect(text == "ab", "source text must be forwarded")
  expect(maxResults == AUTOCOMPLETE_MAX_BUTTONS + 1, "source maxResults mismatch")
  expect(cursorPosition == 2, "source cursorPosition mismatch")
  expect(allowFullMatch == true, "source allowFullMatch mismatch")

  return {
    { name = "able", priority = LE_AUTOCOMPLETE_PRIORITY_OTHER },
    { name = "about", priority = LE_AUTOCOMPLETE_PRIORITY_OTHER },
    { name = "absent", priority = LE_AUTOCOMPLETE_PRIORITY_OTHER },
  }
end

AutoCompleteEditBox_SetAutoCompleteSource(editBox, sourceFn)
AutoComplete_Update(editBox, "ab", 2)

expect(AutoCompleteBox:IsShown(), "AutoCompleteBox must be shown")
expect(AutoCompleteBox.parent == editBox, "AutoCompleteBox.parent must be the editBox")
expect(AutoComplete_GetNumResults(AutoCompleteBox) == 3, "numResults must be 3")
expect(AutoComplete_GetSelectedIndex(AutoCompleteBox) == 1, "selected index must be 1")

for index = 1, 3 do
  expect(_G["AutoCompleteButton" .. index]:IsShown(),
         "AutoCompleteButton" .. index .. " must be shown")
end

for index = 4, 5 do
  expect(not _G["AutoCompleteButton" .. index]:IsShown(),
         "AutoCompleteButton" .. index .. " must be hidden")
end

return table.concat(failures, "\n")
"#;
const UPDATE_CONTINUED_RESULTS_PROBE_LUA: &str = r#"
local failures = {}

local function expect(condition, message)
  if not condition then
    table.insert(failures, message)
  end
end

local editBox = CreateFrame("EditBox", nil, UIParent)
editBox:SetSize(200, 20)
editBox:SetPoint("CENTER", UIParent, "CENTER", 0, 0)

local function sourceFn()
  return {
    { name = "able", priority = LE_AUTOCOMPLETE_PRIORITY_OTHER },
    { name = "about", priority = LE_AUTOCOMPLETE_PRIORITY_OTHER },
    { name = "absent", priority = LE_AUTOCOMPLETE_PRIORITY_OTHER },
    { name = "above", priority = LE_AUTOCOMPLETE_PRIORITY_OTHER },
    { name = "abroad", priority = LE_AUTOCOMPLETE_PRIORITY_OTHER },
    { name = "absolute", priority = LE_AUTOCOMPLETE_PRIORITY_OTHER },
  }
end

AutoCompleteEditBox_SetAutoCompleteSource(editBox, sourceFn)
AutoComplete_Update(editBox, "ab", 2)

expect(AutoCompleteButton5:IsShown(), "AutoCompleteButton5 must be shown")
expect(AutoCompleteButton5:GetText() == CONTINUED,
       "AutoCompleteButton5 must show CONTINUED")
expect(not AutoCompleteButton5:IsEnabled(), "AutoCompleteButton5 must be disabled")
expect(AutoComplete_GetNumResults(AutoCompleteBox) == 4, "numResults must be 4")

return table.concat(failures, "\n")
"#;

#[test]
fn blizzard_auto_complete_update_results_shows_matching_buttons() {
    common::with_perf_lock(|| {
        common::with_timeout(240, || {
            with_blizzard_addon_smoke_shape(&[ROOT], &[], |env, loaded| {
                assert!(
                    loaded.iter().any(|name| name == ROOT),
                    "`{ROOT}` must load before AutoComplete_Update can be checked. \
                     Loaded set: {loaded:?}"
                );

                let failures: String = env
                    .eval(UPDATE_RESULTS_PROBE_LUA)
                    .expect("AutoComplete_Update results probe should run");
                assert!(
                    failures.is_empty(),
                    "`{ROOT}` result AutoComplete_Update mismatches:\n{failures}"
                );
            });
        });
    });
}

#[test]
fn blizzard_auto_complete_update_results_uses_continued_indicator() {
    common::with_perf_lock(|| {
        common::with_timeout(240, || {
            with_blizzard_addon_smoke_shape(&[ROOT], &[], |env, loaded| {
                assert!(
                    loaded.iter().any(|name| name == ROOT),
                    "`{ROOT}` must load before AutoComplete_Update can be checked. \
                     Loaded set: {loaded:?}"
                );

                let failures: String = env
                    .eval(UPDATE_CONTINUED_RESULTS_PROBE_LUA)
                    .expect("AutoComplete_Update continued-results probe should run");
                assert!(
                    failures.is_empty(),
                    "`{ROOT}` continued AutoComplete_Update mismatches:\n{failures}"
                );
            });
        });
    });
}
