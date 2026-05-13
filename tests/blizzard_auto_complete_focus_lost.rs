use crate::common;

use common::blizzard_addon_harness::with_blizzard_addon_smoke_shape;

const ROOT: &str = "Blizzard_AutoComplete";
const FOCUS_LOST_PROBE_LUA: &str = r#"
local failures = {}

local function expect(condition, message)
  if not condition then
    table.insert(failures, message)
  end
end

local editBox = CreateFrame("EditBox", nil, UIParent)
local highlightStart
local highlightEnd

editBox.HighlightText = function(self, startOffset, endOffset)
  highlightStart = startOffset
  highlightEnd = endOffset
end

AutoCompleteBox.parent = editBox
AutoCompleteBox:Show()

AutoCompleteEditBox_OnEditFocusLost(editBox)

expect(highlightStart == 0, "focus lost must reset highlight start to 0")
expect(highlightEnd == 0, "focus lost must reset highlight end to 0")
expect(not AutoCompleteBox:IsShown(), "focus lost must hide AutoCompleteBox")
expect(AutoCompleteBox.parent == nil, "focus lost must detach AutoCompleteBox")

return table.concat(failures, "\n")
"#;

#[test]
fn blizzard_auto_complete_focus_lost_resets_highlight_and_hides() {
    common::with_perf_lock(|| {
        common::with_timeout(240, || {
            with_blizzard_addon_smoke_shape(&[ROOT], &[], |env, loaded| {
                assert!(
                    loaded.iter().any(|name| name == ROOT),
                    "`{ROOT}` must load before AutoComplete focus-lost handling can be checked. \
                     Loaded set: {loaded:?}"
                );

                let failures: String = env
                    .eval(FOCUS_LOST_PROBE_LUA)
                    .expect("AutoComplete focus-lost probe should run");
                assert!(
                    failures.is_empty(),
                    "`{ROOT}` focus-lost mismatches:\n{failures}"
                );
            });
        });
    });
}
