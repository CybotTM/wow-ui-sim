use crate::common;

use common::blizzard_addon_harness::with_blizzard_addon_smoke_shape;

const ROOT: &str = "Blizzard_AutoComplete";
const UPDATE_NO_SOURCE_PROBE_LUA: &str = r#"
AutoCompleteBox:Hide()

local parentWithoutSource = CreateFrame("EditBox", nil, UIParent)
parentWithoutSource.autoCompleteSource = nil
parentWithoutSource.autoCompleteParams = nil

AutoComplete_Update(parentWithoutSource, "abc", 3)

if AutoCompleteBox:IsShown() then
  return false, "AutoCompleteBox must remain hidden without an autoCompleteSource"
end

return true, nil
"#;

#[test]
fn blizzard_auto_complete_update_without_source_returns_silently() {
    if wow_ui_sim::client_profile::ACTIVE == wow_ui_sim::client_profile::ClientProfile::Wrath {
        common::wrath_autocomplete::load_and_assert_native(
            r#"
local other = CreateFrame('EditBox', nil, UIParent)
AutoComplete_Update(other, 'F09', 3)
assert(not AutoCompleteBox:IsShown())
assert(AutoCompleteBox.parent == F09EditBox)
"#,
        );
        return;
    }

    common::with_perf_lock(|| {
        common::with_timeout(240, || {
            with_blizzard_addon_smoke_shape(&[ROOT], &[], |env, loaded| {
                assert!(
                    loaded.iter().any(|name| name == ROOT),
                    "`{ROOT}` must load before AutoComplete_Update can be checked. \
                     Loaded set: {loaded:?}"
                );

                let (ok, message): (bool, Option<String>) = env
                    .eval(UPDATE_NO_SOURCE_PROBE_LUA)
                    .expect("AutoComplete_Update no-source probe should run");
                assert!(
                    ok,
                    "`{ROOT}` no-source AutoComplete_Update mismatch: {}",
                    message.unwrap_or_else(|| "unknown mismatch".to_string())
                );
            });
        });
    });
}
