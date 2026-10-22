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
    if wow_ui_sim::client_profile::ACTIVE == wow_ui_sim::client_profile::ClientProfile::Wrath {
        common::wrath_autocomplete::load_and_assert_native(
            r#"
AutoComplete_UpdateResults(AutoCompleteBox, 'F09Alpha', 'F09Beta')
AutoCompleteButton_OnClick(AutoCompleteButton2)
assert(F09EditBox:GetText() == 'F09Beta')
assert(F09EditBox:GetCursorPosition() == 7)
assert(not AutoCompleteBox:IsShown())
F09EditBox:SetText('/whisper F09 hello')
F09EditBox.autoCompleteRegex = '(/whisper )([^ ]+)(.*)'
F09EditBox.autoCompleteFormatRegex = '%2$s%1$s%4$s'
AutoCompleteButton_OnClick(AutoCompleteButton1)
assert(F09EditBox:GetText() == '/whisper F09Alpha hello')
F09EditBox:SetText('/whisper F09')
F09EditBox.addSpaceToAutoComplete = true
AutoComplete_UpdateResults(AutoCompleteBox, 'F09Alpha', 'F09Beta')
AutoCompleteButton_OnClick(AutoCompleteButton2)
local expectedText = '/whisper F09Beta '
assert(F09EditBox:GetText() == expectedText)
assert(F09EditBox:GetCursorPosition() == strlen(expectedText))
assert(not AutoCompleteBox:IsShown())
"#,
        );
        return;
    }

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
