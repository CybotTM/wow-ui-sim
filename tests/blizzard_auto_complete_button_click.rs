use crate::common;

use common::blizzard_addon_harness::with_blizzard_addon_smoke_shape;

const ROOT: &str = "Blizzard_AutoComplete";
const BUTTON_CLICK_PROBE_LUA: &str = r#"
local failures = {}

local function expect(condition, message)
  if not condition then
    table.insert(failures, message)
  end
end

local editBox = CreateFrame("EditBox", nil, UIParent)
editBox:SetSize(200, 20)
editBox:SetPoint("CENTER", UIParent, "CENTER", 0, 0)
editBox.command = "/whisper"
editBox.addSpaceToAutoComplete = true

local function sourceFn()
  return {
    { name = "Alice", priority = LE_AUTOCOMPLETE_PRIORITY_OTHER },
    { name = "Bob", priority = LE_AUTOCOMPLETE_PRIORITY_OTHER },
  }
end

AutoCompleteEditBox_SetAutoCompleteSource(editBox, sourceFn)
AutoComplete_Update(editBox, "a", 1)
AutoCompleteButton_OnClick(AutoCompleteButton2)

local expectedText = "/whisper Bob "
expect(editBox:GetText() == expectedText, "click must write the command-prefixed name")
expect(editBox:GetCursorPosition() == strlen(expectedText), "cursor must move to text end")
expect(not AutoCompleteBox:IsShown(), "AutoCompleteBox must be hidden after click")

return table.concat(failures, "\n")
"#;

#[test]
fn blizzard_auto_complete_button_click_writes_selected_name() {
    common::with_perf_lock(|| {
        common::with_timeout(240, || {
            with_blizzard_addon_smoke_shape(&[ROOT], &[], |env, loaded| {
                assert!(
                    loaded.iter().any(|name| name == ROOT),
                    "`{ROOT}` must load before AutoCompleteButton_OnClick can be checked. \
                     Loaded set: {loaded:?}"
                );

                let failures: String = env
                    .eval(BUTTON_CLICK_PROBE_LUA)
                    .expect("AutoCompleteButton_OnClick probe should run");
                assert!(
                    failures.is_empty(),
                    "`{ROOT}` button-click mismatches:\n{failures}"
                );
            });
        });
    });
}
