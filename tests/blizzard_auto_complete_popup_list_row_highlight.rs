use crate::common;

use common::blizzard_addon_harness::with_blizzard_addon_smoke_shape;

const ROOT: &str = "Blizzard_AutoCompletePopupList";
const ROW_HIGHLIGHT_PROBE_LUA: &str = r#"
local failures = {}

local function expect(condition, message)
  if not condition then
    table.insert(failures, message)
  end
end

local popup = CreateFrame("Frame", "TestPopupListRowHighlightOwner", UIParent,
                         "AutoCompletePopupListTemplate")
local row = CreateFrame("Button", "TestPopupListRowHighlightButton", popup,
                       "AutoCompletePopupListResultTemplate")

row:SetHighlighted(true)
expect(row.HighlightTexture:IsShown(),
       "SetHighlighted(true) must show HighlightTexture")

row:SetHighlighted(false)
expect(not row.HighlightTexture:IsShown(),
       "SetHighlighted(false) must hide HighlightTexture")

return table.concat(failures, "\n")
"#;

#[test]
fn blizzard_auto_complete_popup_list_row_highlight_toggles_texture() {
    common::with_perf_lock(|| {
        common::with_timeout(240, || {
            with_blizzard_addon_smoke_shape(&[ROOT], &[], |env, loaded| {
                assert!(
                    loaded.iter().any(|name| name == ROOT),
                    "`{ROOT}` must load before PopupList row highlight can be checked. \
                     Loaded set: {loaded:?}"
                );

                let failures: String = env
                    .eval(ROW_HIGHLIGHT_PROBE_LUA)
                    .expect("AutoCompletePopupList row highlight probe should run");
                assert!(
                    failures.is_empty(),
                    "`{ROOT}` row highlight mismatches:\n{failures}"
                );
            });
        });
    });
}
