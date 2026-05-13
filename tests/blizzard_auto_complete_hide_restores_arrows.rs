use crate::common;

use common::blizzard_addon_harness::with_blizzard_addon_smoke_shape;

const ROOT: &str = "Blizzard_AutoComplete";
const HIDE_RESTORES_ARROWS_PROBE_LUA: &str = r#"
local failures = {}

local function expect(condition, message)
  if not condition then
    table.insert(failures, message)
  end
end

local editBox = CreateFrame("EditBox", nil, UIParent)
editBox:SetSize(200, 20)
editBox:SetPoint("CENTER", UIParent, "CENTER", 0, 0)
editBox:SetAltArrowKeyMode(true)

local function sourceFn()
  return {
    { name = "able", priority = LE_AUTOCOMPLETE_PRIORITY_OTHER },
    { name = "about", priority = LE_AUTOCOMPLETE_PRIORITY_OTHER },
  }
end

AutoCompleteEditBox_SetAutoCompleteSource(editBox, sourceFn)
AutoComplete_Update(editBox, "ab", 2)

expect(AutoCompleteBox.parent == editBox, "AutoCompleteBox.parent must be the editBox")
expect(AutoCompleteBox.parentArrows == true, "AutoCompleteBox.parentArrows must capture true")
expect(editBox:GetAltArrowKeyMode() == false, "AutoComplete_Update must disable alt arrows")

AutoComplete_HideIfAttachedTo(editBox)

expect(editBox:GetAltArrowKeyMode() == true, "hide must restore the original alt-arrow mode")
expect(AutoCompleteBox.parentArrows == nil, "hide must clear captured parentArrows")
expect(AutoCompleteBox.parent == nil, "hide must clear AutoCompleteBox.parent")

return table.concat(failures, "\n")
"#;

#[test]
fn blizzard_auto_complete_hide_restores_arrow_key_mode() {
    common::with_perf_lock(|| {
        common::with_timeout(240, || {
            with_blizzard_addon_smoke_shape(&[ROOT], &[], |env, loaded| {
                assert!(
                    loaded.iter().any(|name| name == ROOT),
                    "`{ROOT}` must load before AutoComplete arrow restoration can be checked. \
                     Loaded set: {loaded:?}"
                );

                let failures: String = env
                    .eval(HIDE_RESTORES_ARROWS_PROBE_LUA)
                    .expect("AutoComplete arrow restoration probe should run");
                assert!(
                    failures.is_empty(),
                    "`{ROOT}` arrow restoration mismatches:\n{failures}"
                );
            });
        });
    });
}
