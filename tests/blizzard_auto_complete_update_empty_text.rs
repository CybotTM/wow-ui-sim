use crate::common;

use common::blizzard_addon_harness::with_blizzard_addon_smoke_shape;

const ROOT: &str = "Blizzard_AutoComplete";
const UPDATE_EMPTY_TEXT_PROBE_LUA: &str = r#"
local editBox = CreateFrame("EditBox", nil, UIParent)
local function sourceFn()
end

AutoCompleteEditBox_SetAutoCompleteSource(editBox, sourceFn, "param")
AutoCompleteBox.parent = editBox
AutoCompleteBox:Show()

AutoComplete_Update(editBox, "", 0)

if AutoCompleteBox:IsShown() then
  return false, "AutoCompleteBox must be hidden for empty text"
end

if AutoCompleteBox.parent ~= nil then
  return false, "AutoCompleteBox.parent must be nil after empty-text update"
end

return true, nil
"#;

#[test]
fn blizzard_auto_complete_update_empty_text_hides_attached_box() {
    common::with_perf_lock(|| {
        common::with_timeout(240, || {
            with_blizzard_addon_smoke_shape(&[ROOT], &[], |env, loaded| {
                assert!(
                    loaded.iter().any(|name| name == ROOT),
                    "`{ROOT}` must load before AutoComplete_Update can be checked. \
                     Loaded set: {loaded:?}"
                );

                let (ok, message): (bool, Option<String>) = env
                    .eval(UPDATE_EMPTY_TEXT_PROBE_LUA)
                    .expect("AutoComplete_Update empty-text probe should run");
                assert!(
                    ok,
                    "`{ROOT}` empty-text AutoComplete_Update mismatch: {}",
                    message.unwrap_or_else(|| "unknown mismatch".to_string())
                );
            });
        });
    });
}
