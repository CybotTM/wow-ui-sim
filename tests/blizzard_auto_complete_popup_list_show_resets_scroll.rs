use crate::common;

use common::blizzard_addon_harness::with_blizzard_addon_smoke_shape;

const ROOT: &str = "Blizzard_AutoCompletePopupList";
const SHOW_RESETS_SCROLL_PROBE_LUA: &str = r#"
local failures = {}

local function expect(condition, message)
  if not condition then
    table.insert(failures, message)
  end
end

local popup = CreateFrame("Frame", "TestPopupListShowResetsScroll", UIParent,
                         "AutoCompletePopupListTemplate")
popup:OnLoad()

local scrollToBeginCount = 0
local scrollToBeginInterpolation = nil
local originalScrollToBegin = popup.ScrollBox.ScrollToBegin
popup.ScrollBox.ScrollToBegin = function(self, interpolation)
  scrollToBeginCount = scrollToBeginCount + 1
  scrollToBeginInterpolation = interpolation
  if originalScrollToBegin then
    return originalScrollToBegin(self, interpolation)
  end
end

popup:Show()
popup:Hide()
popup:OnShow()

expect(scrollToBeginCount == 1,
       "OnShow must call ScrollBox:ScrollToBegin exactly once, got " ..
       tostring(scrollToBeginCount))
expect(scrollToBeginInterpolation == ScrollBoxConstants.NoScrollInterpolation,
       "OnShow must use ScrollBoxConstants.NoScrollInterpolation")

popup.ScrollBox.ScrollToBegin = originalScrollToBegin

return table.concat(failures, "\n")
"#;

#[test]
fn blizzard_auto_complete_popup_list_show_resets_scroll_to_beginning() {
    common::with_perf_lock(|| {
        common::with_timeout(240, || {
            with_blizzard_addon_smoke_shape(&[ROOT], &[], |env, loaded| {
                assert!(
                    loaded.iter().any(|name| name == ROOT),
                    "`{ROOT}` must load before PopupList OnShow can be checked. \
                     Loaded set: {loaded:?}"
                );

                let failures: String = env
                    .eval(SHOW_RESETS_SCROLL_PROBE_LUA)
                    .expect("AutoCompletePopupList show reset probe should run");
                assert!(
                    failures.is_empty(),
                    "`{ROOT}` show reset mismatches:\n{failures}"
                );
            });
        });
    });
}
