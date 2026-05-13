use crate::common;

use common::blizzard_addon_harness::with_blizzard_addon_smoke_shape;

const ROOT: &str = "Blizzard_AutoComplete";
const ESCAPE_PROBE_LUA: &str = r#"
local failures = {}

local function expect(condition, message)
  if not condition then
    table.insert(failures, message)
  end
end

local editBox = CreateFrame("EditBox", nil, UIParent)
local otherEditBox = CreateFrame("EditBox", nil, UIParent)

AutoCompleteBox.parent = editBox
AutoCompleteBox:Show()

local shownResult = AutoCompleteEditBox_OnEscapePressed(editBox)
expect(shownResult == true, "escape must return true when AutoCompleteBox is attached")
expect(not AutoCompleteBox:IsShown(), "escape must hide AutoCompleteBox")
expect(AutoCompleteBox.parent == nil, "escape must detach AutoCompleteBox")

AutoCompleteBox.parent = editBox
AutoCompleteBox:Hide()

local hiddenResult = AutoCompleteEditBox_OnEscapePressed(editBox)
expect(hiddenResult == false, "escape must return false while hidden")
expect(AutoCompleteBox.parent == editBox, "hidden escape must leave parent unchanged")

AutoCompleteBox.parent = editBox
AutoCompleteBox:Show()

local otherResult = AutoCompleteEditBox_OnEscapePressed(otherEditBox)
expect(otherResult == false, "escape must return false for a different parent")
expect(AutoCompleteBox:IsShown(), "different-parent escape must leave AutoCompleteBox shown")
expect(AutoCompleteBox.parent == editBox, "different-parent escape must leave parent unchanged")

return table.concat(failures, "\n")
"#;

#[test]
fn blizzard_auto_complete_escape_hides_only_attached_box() {
    common::with_perf_lock(|| {
        common::with_timeout(240, || {
            with_blizzard_addon_smoke_shape(&[ROOT], &[], |env, loaded| {
                assert!(
                    loaded.iter().any(|name| name == ROOT),
                    "`{ROOT}` must load before AutoComplete escape handling can be checked. \
                     Loaded set: {loaded:?}"
                );

                let failures: String = env
                    .eval(ESCAPE_PROBE_LUA)
                    .expect("AutoComplete escape probe should run");
                assert!(
                    failures.is_empty(),
                    "`{ROOT}` escape mismatches:\n{failures}"
                );
            });
        });
    });
}
