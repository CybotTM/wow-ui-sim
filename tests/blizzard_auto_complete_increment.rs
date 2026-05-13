use crate::common;

use common::blizzard_addon_harness::with_blizzard_addon_smoke_shape;

const ROOT: &str = "Blizzard_AutoComplete";
const INCREMENT_PROBE_LUA: &str = r#"
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
  }
end

AutoCompleteEditBox_SetAutoCompleteSource(editBox, sourceFn)
AutoComplete_Update(editBox, "ab", 2)

expect(AutoComplete_GetSelectedIndex(AutoCompleteBox) == 1,
       "initial selected index must be 1")

expect(AutoComplete_IncrementSelection(editBox, false), "down increment 1 must return true")
expect(AutoComplete_GetSelectedIndex(AutoCompleteBox) == 2, "down increment 1 must select 2")
expect(AutoComplete_IncrementSelection(editBox, false), "down increment 2 must return true")
expect(AutoComplete_GetSelectedIndex(AutoCompleteBox) == 3, "down increment 2 must select 3")
expect(AutoComplete_IncrementSelection(editBox, false), "down increment 3 must return true")
expect(AutoComplete_GetSelectedIndex(AutoCompleteBox) == 1, "down increment 3 must wrap to 1")

expect(AutoComplete_IncrementSelection(editBox, true), "up increment 1 must return true")
expect(AutoComplete_GetSelectedIndex(AutoCompleteBox) == 3, "up increment 1 must wrap to 3")
expect(AutoComplete_IncrementSelection(editBox, true), "up increment 2 must return true")
expect(AutoComplete_GetSelectedIndex(AutoCompleteBox) == 2, "up increment 2 must select 2")
expect(AutoComplete_IncrementSelection(editBox, true), "up increment 3 must return true")
expect(AutoComplete_GetSelectedIndex(AutoCompleteBox) == 1, "up increment 3 must select 1")

local otherEditBox = CreateFrame("EditBox", nil, UIParent)
expect(not AutoComplete_IncrementSelection(otherEditBox, false),
       "increment must return false for a different parent")

AutoCompleteBox:Hide()
expect(not AutoComplete_IncrementSelection(editBox, false),
       "increment must return false while AutoCompleteBox is hidden")

return table.concat(failures, "\n")
"#;

#[test]
fn blizzard_auto_complete_increment_selection_wraps_and_guards() {
    common::with_perf_lock(|| {
        common::with_timeout(240, || {
            with_blizzard_addon_smoke_shape(&[ROOT], &[], |env, loaded| {
                assert!(
                    loaded.iter().any(|name| name == ROOT),
                    "`{ROOT}` must load before AutoComplete_IncrementSelection can be checked. \
                     Loaded set: {loaded:?}"
                );

                let failures: String = env
                    .eval(INCREMENT_PROBE_LUA)
                    .expect("AutoComplete_IncrementSelection probe should run");
                assert!(
                    failures.is_empty(),
                    "`{ROOT}` increment-selection mismatches:\n{failures}"
                );
            });
        });
    });
}
