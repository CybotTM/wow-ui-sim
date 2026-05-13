use crate::common;

use common::blizzard_addon_harness::with_blizzard_addon_smoke_shape;

const ROOT: &str = "Blizzard_AutoCompletePopupList";
const MIXIN_PROBE_LUA: &str = r#"
local failures = {}

local function expect(condition, message)
  if not condition then
    table.insert(failures, message)
  end
end

local expectedMethods = {
  "OnLoad",
  "OnShow",
  "UpdateResults",
  "SetResults",
  "UpdateResultsDisplay",
  "ClearResults",
  "HighlightResult",
  "CycleHighlightedResultUp",
  "CycleHighlightedResultDown",
  "SelectHighlightedResult",
  "HasResults",
  "SelectResult",
  "GetMaximumEntries",
  "SetSelectResultCallback",
  "SetResultsListCallback",
}

expect(type(AutoCompletePopupListMixin) == "table",
       "AutoCompletePopupListMixin must be a table")

if type(AutoCompletePopupListMixin) == "table" then
  for _, methodName in ipairs(expectedMethods) do
    expect(type(AutoCompletePopupListMixin[methodName]) == "function",
           "AutoCompletePopupListMixin." .. methodName .. " must be a function")
  end
end

return table.concat(failures, "\n")
"#;

#[test]
fn blizzard_auto_complete_popup_list_mixin_exposes_methods() {
    common::with_perf_lock(|| {
        common::with_timeout(240, || {
            with_blizzard_addon_smoke_shape(&[ROOT], &[], |env, loaded| {
                assert!(
                    loaded.iter().any(|name| name == ROOT),
                    "`{ROOT}` must load before PopupList mixin methods can be checked. \
                     Loaded set: {loaded:?}"
                );

                let failures: String = env
                    .eval(MIXIN_PROBE_LUA)
                    .expect("AutoCompletePopupList mixin probe should run");
                assert!(
                    failures.is_empty(),
                    "`{ROOT}` mixin mismatches:\n{failures}"
                );
            });
        });
    });
}
