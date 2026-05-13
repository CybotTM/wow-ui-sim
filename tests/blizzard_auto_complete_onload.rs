use crate::common;

use common::blizzard_addon_harness::with_blizzard_addon_smoke_shape;

const ROOT: &str = "Blizzard_AutoComplete";
const ONLOAD_PROBE_LUA: &str = r##"
local failures = {}

local function expect(condition, message)
  if not condition then
    table.insert(failures, message)
  end
end

expect(type(AutoComplete_OnLoad) == "function",
       "AutoComplete_OnLoad must be a function")
expect(AutoCompleteBox ~= nil, "AutoCompleteBox must exist")
expect(AutoCompleteButton1 ~= nil, "AutoCompleteButton1 must exist")
expect(AutoCompleteInstructions ~= nil, "AutoCompleteInstructions must exist")

if AutoCompleteBox == nil or AutoCompleteButton1 == nil or AutoCompleteInstructions == nil then
  return table.concat(failures, "\n")
end

AutoComplete_OnLoad(AutoCompleteBox)

local expectedHeight = 5 * AutoCompleteButton1:GetHeight()
expect(AutoCompleteBox.maxHeight == expectedHeight,
       "AutoCompleteBox.maxHeight must be 5 * AutoCompleteButton1:GetHeight(), got " ..
       tostring(AutoCompleteBox.maxHeight) .. " expected " .. tostring(expectedHeight))
expect(AutoCompleteInstructions:GetText() == "|cffbbbbbb" .. PRESS_TAB .. "|r",
       "AutoCompleteInstructions text mismatch")
expect(AutoCompleteBox:IsEventRegistered("GUILD_ROSTER_UPDATE"),
       "AutoCompleteBox must register GUILD_ROSTER_UPDATE")

return table.concat(failures, "\n")
"##;

#[test]
fn blizzard_auto_complete_onload_initializes_box_state() {
    common::with_perf_lock(|| {
        common::with_timeout(240, || {
            with_blizzard_addon_smoke_shape(&[ROOT], &[], |env, loaded| {
                assert!(
                    loaded.iter().any(|name| name == ROOT),
                    "`{ROOT}` must load before AutoComplete_OnLoad can be checked. \
                     Loaded set: {loaded:?}"
                );

                let failures: String = env
                    .eval(ONLOAD_PROBE_LUA)
                    .expect("AutoComplete OnLoad probe should run");
                assert!(
                    failures.is_empty(),
                    "`{ROOT}` OnLoad mismatches:\n{failures}"
                );
            });
        });
    });
}
