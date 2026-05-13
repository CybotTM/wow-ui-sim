use crate::common;

use common::blizzard_addon_harness::with_blizzard_addon_smoke_shape;

const ROOT: &str = "Blizzard_AutoComplete";
const TEMPLATE_SURFACE_PROBE_LUA: &str = r#"
local failures = {}

local function expect(condition, message)
  if not condition then
    table.insert(failures, message)
  end
end

local editBox = CreateFrame("EditBox", "AutoCompleteEditBoxTemplateProbe", UIParent,
                            "AutoCompleteEditBoxTemplate")
expect(editBox:GetObjectType() == "EditBox",
       "AutoCompleteEditBoxTemplate must instantiate an EditBox")

local editBoxScripts = {
  OnTabPressed = AutoCompleteEditBox_OnTabPressed,
  OnEnterPressed = AutoCompleteEditBox_OnEnterPressed,
  OnTextChanged = AutoCompleteEditBox_OnTextChanged,
  OnChar = AutoCompleteEditBox_OnChar,
  OnEditFocusLost = AutoCompleteEditBox_OnEditFocusLost,
  OnEscapePressed = AutoCompleteEditBox_OnEscapePressed,
  OnArrowPressed = AutoCompleteEditBox_OnArrowPressed,
  OnKeyDown = AutoCompleteEditBox_OnKeyDown,
  OnKeyUp = AutoCompleteEditBox_OnKeyUp,
}

for scriptName, expectedFunction in pairs(editBoxScripts) do
  expect(editBox:GetScript(scriptName) == expectedFunction,
         "AutoCompleteEditBoxTemplate " .. scriptName .. " script mismatch")
end

local button = CreateFrame("Button", "AutoCompleteButtonTemplateProbe", UIParent,
                           "AutoCompleteButtonTemplate")
expect(button:GetObjectType() == "Button",
       "AutoCompleteButtonTemplate must instantiate a Button")
expect(button:GetScript("OnClick") == AutoCompleteButton_OnClick,
       "AutoCompleteButtonTemplate OnClick script mismatch")
expect(type(button:GetScript("OnLoad")) == "function",
       "AutoCompleteButtonTemplate must wire its inline OnLoad script")

return table.concat(failures, "\n")
"#;

#[test]
fn blizzard_auto_complete_virtual_templates_instantiate_with_scripts() {
    common::with_perf_lock(|| {
        common::with_timeout(240, || {
            with_blizzard_addon_smoke_shape(&[ROOT], &[], |env, loaded| {
                assert!(
                    loaded.iter().any(|name| name == ROOT),
                    "`{ROOT}` must load before virtual templates can be checked. \
                     Loaded set: {loaded:?}"
                );

                let failures: String = env
                    .eval(TEMPLATE_SURFACE_PROBE_LUA)
                    .expect("AutoComplete template-surface probe should run");
                assert!(
                    failures.is_empty(),
                    "`{ROOT}` template-surface mismatches:\n{failures}"
                );
            });
        });
    });
}
