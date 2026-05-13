use crate::common;

use common::blizzard_addon_harness::with_blizzard_addon_smoke_shape;

const ROOT: &str = "Blizzard_AutoCompletePopupList";
const GLOBALS_PROBE_LUA: &str = r#"
local failures = {}

local function expect(condition, message)
  if not condition then
    table.insert(failures, message)
  end
end

expect(type(AutoCompletePopupListResultMixin) == "table",
       "AutoCompletePopupListResultMixin must be a table")
expect(type(AutoCompletePopupListMixin) == "table",
       "AutoCompletePopupListMixin must be a table")

return table.concat(failures, "\n")
"#;

#[test]
fn blizzard_auto_complete_popup_list_mixins_are_global_tables() {
    common::with_perf_lock(|| {
        common::with_timeout(240, || {
            with_blizzard_addon_smoke_shape(&[ROOT], &[], |env, loaded| {
                assert!(
                    loaded.iter().any(|name| name == ROOT),
                    "`{ROOT}` must load before PopupList globals can be checked. \
                     Loaded set: {loaded:?}"
                );

                let failures: String = env
                    .eval(GLOBALS_PROBE_LUA)
                    .expect("AutoCompletePopupList globals probe should run");
                assert!(
                    failures.is_empty(),
                    "`{ROOT}` global mismatches:\n{failures}"
                );
            });
        });
    });
}
