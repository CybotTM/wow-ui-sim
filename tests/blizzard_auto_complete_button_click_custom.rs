use crate::common;

use common::blizzard_addon_harness::with_blizzard_addon_smoke_shape;

const ROOT: &str = "Blizzard_AutoComplete";
const BUTTON_CLICK_CUSTOM_PROBE_LUA: &str = r#"
local failures = {}

local function expect(condition, message)
  if not condition then
    table.insert(failures, message)
  end
end

local editBox = CreateFrame("EditBox", nil, UIParent)
editBox:SetSize(200, 20)
editBox:SetPoint("CENTER", UIParent, "CENTER", 0, 0)
editBox:SetText("original")
editBox:SetCursorPosition(3)
editBox.command = "/whisper"
editBox.addSpaceToAutoComplete = true

local captured = {}
editBox.customAutoCompleteFunction = function(callbackEditBox, newText, nameInfo, name)
  captured.editBox = callbackEditBox
  captured.newText = newText
  captured.nameInfo = nameInfo
  captured.name = name
  return true
end

local function sourceFn()
  return {
    { name = "Alice", priority = LE_AUTOCOMPLETE_PRIORITY_OTHER },
    { name = "Bob", priority = LE_AUTOCOMPLETE_PRIORITY_OTHER },
  }
end

AutoCompleteEditBox_SetAutoCompleteSource(editBox, sourceFn)
AutoComplete_Update(editBox, "a", 1)
AutoCompleteButton_OnClick(AutoCompleteButton2)

expect(captured.editBox == editBox, "custom function must receive the editBox")
expect(captured.newText == "/whisper Bob ", "custom function must receive default newText")
expect(captured.nameInfo == AutoCompleteButton2.nameInfo, "custom function must receive nameInfo")
expect(captured.name == "Bob", "custom function must receive the selected name")
expect(editBox:GetText() == "original", "custom function returning true must prevent SetText")
expect(editBox:GetCursorPosition() == 3, "custom function returning true must preserve cursor")
expect(not AutoCompleteBox:IsShown(), "AutoCompleteBox must still be hidden after custom click")

return table.concat(failures, "\n")
"#;

#[test]
fn blizzard_auto_complete_button_click_custom_function_takes_over() {
    common::with_perf_lock(|| {
        common::with_timeout(240, || {
            with_blizzard_addon_smoke_shape(&[ROOT], &[], |env, loaded| {
                assert!(
                    loaded.iter().any(|name| name == ROOT),
                    "`{ROOT}` must load before custom AutoCompleteButton_OnClick can be checked. \
                     Loaded set: {loaded:?}"
                );

                let failures: String = env
                    .eval(BUTTON_CLICK_CUSTOM_PROBE_LUA)
                    .expect("custom AutoCompleteButton_OnClick probe should run");
                assert!(
                    failures.is_empty(),
                    "`{ROOT}` custom button-click mismatches:\n{failures}"
                );
            });
        });
    });
}
