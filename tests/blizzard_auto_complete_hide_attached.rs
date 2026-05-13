use crate::common;

use common::blizzard_addon_harness::with_blizzard_addon_smoke_shape;

const ROOT: &str = "Blizzard_AutoComplete";
const HIDE_ATTACHED_PROBE_LUA: &str = r#"
local failures = {}

local function expect(condition, message)
  if not condition then
    table.insert(failures, message)
  end
end

local editBoxA = CreateFrame("EditBox", nil, UIParent)
local editBoxB = CreateFrame("EditBox", nil, UIParent)

AutoCompleteBox.parent = editBoxA
AutoCompleteBox:Show()

AutoComplete_HideIfAttachedTo(editBoxB)
expect(AutoCompleteBox:IsShown(), "AutoCompleteBox must stay shown for a different parent")
expect(AutoCompleteBox.parent == editBoxA,
       "AutoCompleteBox.parent must remain editBoxA for a different parent")

AutoComplete_HideIfAttachedTo(editBoxA)
expect(not AutoCompleteBox:IsShown(), "AutoCompleteBox must be hidden for the attached parent")
expect(AutoCompleteBox.parent == nil, "AutoCompleteBox.parent must be nil after hiding")

return table.concat(failures, "\n")
"#;

#[test]
fn blizzard_auto_complete_hide_if_attached_to_matching_parent_only() {
    common::with_perf_lock(|| {
        common::with_timeout(240, || {
            with_blizzard_addon_smoke_shape(&[ROOT], &[], |env, loaded| {
                assert!(
                    loaded.iter().any(|name| name == ROOT),
                    "`{ROOT}` must load before AutoComplete_HideIfAttachedTo can be checked. \
                     Loaded set: {loaded:?}"
                );

                let failures: String = env
                    .eval(HIDE_ATTACHED_PROBE_LUA)
                    .expect("AutoComplete_HideIfAttachedTo probe should run");
                assert!(
                    failures.is_empty(),
                    "`{ROOT}` hide-if-attached mismatches:\n{failures}"
                );
            });
        });
    });
}
