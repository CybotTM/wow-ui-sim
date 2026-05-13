use crate::common;

use common::blizzard_addon_harness::with_blizzard_addon_smoke_shape;

const ROOT: &str = "Blizzard_AutoComplete";
const ARROW_PROBE_LUA: &str = r#"
local failures = {}

local function expect(condition, message)
  if not condition then
    table.insert(failures, message)
  end
end

local editBox = CreateFrame("EditBox", nil, UIParent)
local calls = {}
local originalIncrementSelection = AutoComplete_IncrementSelection

AutoComplete_IncrementSelection = function(frame, up)
  table.insert(calls, { frame = frame, up = up })
  return true
end

local upResult = AutoCompleteEditBox_OnArrowPressed(editBox, "UP")
local downResult = AutoCompleteEditBox_OnArrowPressed(editBox, "DOWN")
local otherResult = AutoCompleteEditBox_OnArrowPressed(editBox, "LEFT")

AutoComplete_IncrementSelection = originalIncrementSelection

expect(upResult == true, "UP must return AutoComplete_IncrementSelection result")
expect(downResult == true, "DOWN must return AutoComplete_IncrementSelection result")
expect(otherResult == nil, "other arrow keys must return nil")
expect(#calls == 2, "UP and DOWN must be the only increment calls")
expect(calls[1].frame == editBox and calls[1].up == true,
       "UP must route with up=true")
expect(calls[2].frame == editBox and calls[2].up == false,
       "DOWN must route with up=false")

return table.concat(failures, "\n")
"#;

#[test]
fn blizzard_auto_complete_arrow_keys_route_to_increment_selection() {
    common::with_perf_lock(|| {
        common::with_timeout(240, || {
            with_blizzard_addon_smoke_shape(&[ROOT], &[], |env, loaded| {
                assert!(
                    loaded.iter().any(|name| name == ROOT),
                    "`{ROOT}` must load before AutoComplete arrow handling can be checked. \
                     Loaded set: {loaded:?}"
                );

                let failures: String = env
                    .eval(ARROW_PROBE_LUA)
                    .expect("AutoComplete arrow-key probe should run");
                assert!(
                    failures.is_empty(),
                    "`{ROOT}` arrow-key mismatches:\n{failures}"
                );
            });
        });
    });
}
