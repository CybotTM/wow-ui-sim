use crate::common;

use common::blizzard_addon_harness::with_blizzard_addon_smoke_shape;

const ROOT: &str = "Blizzard_AutoCompletePopupList";
const ONLOAD_PROBE_LUA: &str = r#"
local failures = {}

local function expect(condition, message)
  if not condition then
    table.insert(failures, message)
  end
end

local popup = CreateFrame("Frame", "TestPopupListOnLoadFrame", UIParent,
                         "AutoCompletePopupListTemplate")
expect(popup ~= nil, "AutoCompletePopupListTemplate must instantiate")

if popup ~= nil then
  popup:OnLoad()
  local view = popup.ScrollBox:GetView()
  expect(view ~= nil, "popup.ScrollBox:GetView() must be non-nil after OnLoad")
end

return table.concat(failures, "\n")
"#;

#[test]
fn blizzard_auto_complete_popup_list_onload_sets_scrollbox_view() {
    common::with_perf_lock(|| {
        common::with_timeout(240, || {
            with_blizzard_addon_smoke_shape(&[ROOT], &[], |env, loaded| {
                assert!(
                    loaded.iter().any(|name| name == ROOT),
                    "`{ROOT}` must load before PopupList OnLoad can be checked. \
                     Loaded set: {loaded:?}"
                );

                let failures: String = env
                    .eval(ONLOAD_PROBE_LUA)
                    .expect("AutoCompletePopupList OnLoad probe should run");
                assert!(
                    failures.is_empty(),
                    "`{ROOT}` OnLoad mismatches:\n{failures}"
                );
            });
        });
    });
}
