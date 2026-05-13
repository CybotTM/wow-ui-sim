use crate::common;

use common::blizzard_addon_harness::with_blizzard_addon_smoke_shape;

const ROOT: &str = "Blizzard_AutoCompletePopupList";
const INDEPENDENCE_PROBE_LUA: &str = r#"
local failures = {}

local function expect(condition, message)
  if not condition then
    table.insert(failures, message)
  end
end

local function resultList(prefix, count)
  local results = {}
  for index = 1, count do
    results[index] = { text = prefix .. tostring(index) }
  end
  return results
end

local first = CreateFrame("Frame", "TestPopupListIndependentFirst", UIParent,
                         "AutoCompletePopupListTemplate")
local second = CreateFrame("Frame", "TestPopupListIndependentSecond", UIParent,
                          "AutoCompletePopupListTemplate")
first:OnLoad()
second:OnLoad()

local firstCallbackCount = 0
local secondCallbackCount = 0
first.resultsListCallback = function()
  firstCallbackCount = firstCallbackCount + 1
  return 2, resultList("First", 2), nil
end
second.resultsListCallback = function()
  secondCallbackCount = secondCallbackCount + 1
  return 3, resultList("Second", 3), nil
end

first:UpdateResults()

expect(firstCallbackCount == 1,
       "first UpdateResults must call only first callback once, got " ..
       tostring(firstCallbackCount))
expect(secondCallbackCount == 0,
       "first UpdateResults must not call second callback, got " ..
       tostring(secondCallbackCount))
expect(first.ScrollBox:GetDataProviderSize() == 2,
       "first popup must render 2 rows after its update, got " ..
       tostring(first.ScrollBox:GetDataProviderSize()))
expect(not second:HasResults(),
       "second popup must remain empty before its own UpdateResults")

second:UpdateResults()

expect(firstCallbackCount == 1,
       "second UpdateResults must not recall first callback, got " ..
       tostring(firstCallbackCount))
expect(secondCallbackCount == 1,
       "second UpdateResults must call second callback once, got " ..
       tostring(secondCallbackCount))
expect(first.ScrollBox:GetDataProviderSize() == 2,
       "first popup row count must remain 2 after second update, got " ..
       tostring(first.ScrollBox:GetDataProviderSize()))
expect(second.ScrollBox:GetDataProviderSize() == 3,
       "second popup must render 3 rows after its update, got " ..
       tostring(second.ScrollBox:GetDataProviderSize()))
expect(first.highlightedIndex == 1,
       "first highlightedIndex must remain 1, got " ..
       tostring(first.highlightedIndex))
expect(second.highlightedIndex == 1,
       "second highlightedIndex must be 1 after its own update, got " ..
       tostring(second.highlightedIndex))

return table.concat(failures, "\n")
"#;

#[test]
fn blizzard_auto_complete_popup_list_instances_keep_independent_results() {
    common::with_perf_lock(|| {
        common::with_timeout(240, || {
            with_blizzard_addon_smoke_shape(&[ROOT], &[], |env, loaded| {
                assert!(
                    loaded.iter().any(|name| name == ROOT),
                    "`{ROOT}` must load before PopupList independence can be checked. \
                     Loaded set: {loaded:?}"
                );

                let failures: String = env
                    .eval(INDEPENDENCE_PROBE_LUA)
                    .expect("AutoCompletePopupList independence probe should run");
                assert!(
                    failures.is_empty(),
                    "`{ROOT}` independence mismatches:\n{failures}"
                );
            });
        });
    });
}
