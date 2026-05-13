use crate::common;

use common::blizzard_addon_harness::with_blizzard_addon_smoke_shape;

const ROOT: &str = "Blizzard_AutoCompletePopupList";
const EXPECTED_ASSERTSAFE_HINT: &str =
    "AutoCompletePopupLists require a resultsListCallback. Use :SetResultsListCallback";
const UPDATE_NO_CALLBACK_PROBE_LUA: &str = r##"
local failures = {}

local function expect(condition, message)
  if not condition then
    table.insert(failures, message)
  end
end

local popup = CreateFrame("Frame", "TestPopupListNoCallbackFrame", UIParent,
                         "AutoCompletePopupListTemplate")
expect(popup ~= nil, "AutoCompletePopupListTemplate must instantiate")
local assertsafeMessages = {}

if popup ~= nil then
  popup:OnLoad()
  popup:SetResults(1, { { text = "before" } })
  expect(popup:HasResults(), "test precondition: popup must have seeded results")

  local originalAssertSafe = assertsafe
  assertsafe = function(condition, message, ...)
    if not condition then
      local formattedMessage = tostring(message)
      if select("#", ...) > 0 then
        formattedMessage = string.format(formattedMessage, ...)
      end
      table.insert(assertsafeMessages, formattedMessage)
    end
    return originalAssertSafe(condition, message, ...)
  end

  popup.resultsListCallback = nil
  popup:UpdateResults()
  assertsafe = originalAssertSafe

  expect(not popup:HasResults(), "UpdateResults without callback must clear results")
  expect(popup.highlightedIndex == 0,
         "UpdateResults without callback must reset highlightedIndex")
  expect(not popup.OverflowCount:IsShown(),
         "UpdateResults without callback must hide OverflowCount")
  expect(not popup:IsShown(), "UpdateResults without callback must hide popup")
end

_G.__popup_no_callback_failures = table.concat(failures, "\n")
_G.__popup_no_callback_assertsafe_messages = table.concat(assertsafeMessages or {}, "\n")
"##;

#[test]
fn blizzard_auto_complete_popup_list_update_without_callback_records_and_clears() {
    common::with_perf_lock(|| {
        common::with_timeout(240, || {
            with_blizzard_addon_smoke_shape(&[ROOT], &[], |env, loaded| {
                assert!(
                    loaded.iter().any(|name| name == ROOT),
                    "`{ROOT}` must load before PopupList UpdateResults can be checked. \
                     Loaded set: {loaded:?}"
                );

                common::panel_fixtures::clear_recorded_lua_errors(env);

                env.exec(UPDATE_NO_CALLBACK_PROBE_LUA)
                    .expect("AutoCompletePopupList no-callback probe should run");
                let failures: String = env
                    .eval("return _G.__popup_no_callback_failures or ''")
                    .expect("AutoCompletePopupList no-callback failures should be readable");
                assert!(
                    failures.is_empty(),
                    "`{ROOT}` no-callback mismatches:\n{failures}"
                );

                let assertsafe_messages: String = env
                    .eval("return _G.__popup_no_callback_assertsafe_messages or ''")
                    .expect("AutoCompletePopupList assertsafe messages should be readable");
                assert!(
                    assertsafe_messages.contains(EXPECTED_ASSERTSAFE_HINT),
                    "`{ROOT}` must record assertsafe hint {EXPECTED_ASSERTSAFE_HINT:?}; got: \
                     {assertsafe_messages:?}",
                );
            });
        });
    });
}
