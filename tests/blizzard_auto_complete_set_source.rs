use crate::common;

use common::blizzard_addon_harness::with_blizzard_addon_smoke_shape;

const ROOT: &str = "Blizzard_AutoComplete";
const SET_SOURCE_PROBE_LUA: &str = r#"
local editBox = CreateFrame("EditBox", nil, UIParent)
local function sourceFn()
end

AutoCompleteEditBox_SetAutoCompleteSource(editBox, sourceFn, "p1", "p2")

if editBox.autoCompleteSource ~= sourceFn then
  return false, "autoCompleteSource must be the supplied function"
end

if type(editBox.autoCompleteParams) ~= "table" then
  return false, "autoCompleteParams must be a table"
end

if #editBox.autoCompleteParams ~= 2 then
  return false, "autoCompleteParams must contain exactly two entries"
end

if editBox.autoCompleteParams[1] ~= "p1" or editBox.autoCompleteParams[2] ~= "p2" then
  return false, "autoCompleteParams must preserve vararg order"
end

return true, nil
"#;

#[test]
fn blizzard_auto_complete_set_source_stores_source_and_params() {
    common::with_perf_lock(|| {
        common::with_timeout(240, || {
            with_blizzard_addon_smoke_shape(&[ROOT], &[], |env, loaded| {
                assert!(
                    loaded.iter().any(|name| name == ROOT),
                    "`{ROOT}` must load before SetAutoCompleteSource can be checked. \
                     Loaded set: {loaded:?}"
                );

                let (ok, message): (bool, Option<String>) = env
                    .eval(SET_SOURCE_PROBE_LUA)
                    .expect("AutoComplete SetAutoCompleteSource probe should run");
                assert!(
                    ok,
                    "`{ROOT}` SetAutoCompleteSource mismatch: {}",
                    message.unwrap_or_else(|| "unknown mismatch".to_string())
                );
            });
        });
    });
}
