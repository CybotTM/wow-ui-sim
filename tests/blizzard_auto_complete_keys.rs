mod common;

use common::blizzard_addon_harness::with_blizzard_addon_smoke_shape;

const ROOT: &str = "Blizzard_AutoComplete";
const KEY_BACKSPACE_PROBE_LUA: &str = r#"
local failures = {}

local function expect(condition, message)
  if not condition then
    table.insert(failures, message)
  end
end

local editBox = CreateFrame("EditBox", nil, UIParent)
editBox.disallowAutoComplete = nil

AutoCompleteEditBox_OnKeyDown(editBox, "A")
expect(editBox.disallowAutoComplete == nil, "non-backspace keydown must leave nil flag untouched")

AutoCompleteEditBox_OnKeyDown(editBox, "BACKSPACE")
expect(editBox.disallowAutoComplete == true, "backspace keydown must set disallowAutoComplete")

AutoCompleteEditBox_OnKeyDown(editBox, "A")
expect(editBox.disallowAutoComplete == true, "non-backspace keydown must preserve true flag")

AutoCompleteEditBox_OnKeyUp(editBox, "A")
expect(editBox.disallowAutoComplete == true, "non-backspace keyup must preserve true flag")

AutoCompleteEditBox_OnKeyUp(editBox, "BACKSPACE")
expect(editBox.disallowAutoComplete == false, "backspace keyup must clear disallowAutoComplete")

AutoCompleteEditBox_OnKeyUp(editBox, "A")
expect(editBox.disallowAutoComplete == false, "non-backspace keyup must preserve false flag")

return table.concat(failures, "\n")
"#;

#[test]
fn blizzard_auto_complete_backspace_toggles_disallow_flag() {
    common::with_perf_lock(|| {
        common::with_timeout(240, || {
            with_blizzard_addon_smoke_shape(&[ROOT], &[], |env, loaded| {
                assert!(
                    loaded.iter().any(|name| name == ROOT),
                    "`{ROOT}` must load before AutoComplete key handlers can be checked. \
                     Loaded set: {loaded:?}"
                );

                let failures: String = env
                    .eval(KEY_BACKSPACE_PROBE_LUA)
                    .expect("AutoComplete key handler probe should run");
                assert!(
                    failures.is_empty(),
                    "`{ROOT}` key handler mismatches:\n{failures}"
                );
            });
        });
    });
}
