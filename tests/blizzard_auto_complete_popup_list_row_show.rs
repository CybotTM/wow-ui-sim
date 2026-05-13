use crate::common;

use common::blizzard_addon_harness::with_blizzard_addon_smoke_shape;

const ROOT: &str = "Blizzard_AutoCompletePopupList";
const ROW_SHOW_PROBE_LUA: &str = r#"
local failures = {}

local function expect(condition, message)
  if not condition then
    table.insert(failures, message)
  end
end

local popup = CreateFrame("Frame", "TestPopupListRowShowOwner", UIParent,
                         "AutoCompletePopupListTemplate")
popup:SetFrameLevel(5)

local row = CreateFrame("Button", "TestPopupListRowShowButton", popup,
                       "AutoCompletePopupListResultTemplate")
row:SetFrameLevel(1)
row:OnShow()

expect(row:GetFrameLevel() == 15,
       "row OnShow must set frame level to parent + 10, got " ..
       tostring(row:GetFrameLevel()))

return table.concat(failures, "\n")
"#;

#[test]
fn blizzard_auto_complete_popup_list_row_show_sets_parent_relative_level() {
    common::with_perf_lock(|| {
        common::with_timeout(240, || {
            with_blizzard_addon_smoke_shape(&[ROOT], &[], |env, loaded| {
                assert!(
                    loaded.iter().any(|name| name == ROOT),
                    "`{ROOT}` must load before PopupList row show can be checked. \
                     Loaded set: {loaded:?}"
                );

                let failures: String = env
                    .eval(ROW_SHOW_PROBE_LUA)
                    .expect("AutoCompletePopupList row show probe should run");
                assert!(
                    failures.is_empty(),
                    "`{ROOT}` row show mismatches:\n{failures}"
                );
            });
        });
    });
}
