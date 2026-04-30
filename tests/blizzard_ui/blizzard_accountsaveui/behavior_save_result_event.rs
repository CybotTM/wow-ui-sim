//! Behavior pin: firing `ACCOUNT_SAVE_RESULT` with
//! `Enum.AccountExportResult.Success` shows the
//! `ACCOUNT_SAVE_SUCCESS` popup with a formatted file link.
//!
//! Source (`Interface/BlizzardUI/Blizzard_AccountSaveUI/
//! Blizzard_AccountSaveUI.lua`, lines 125-141):
//!
//! ```lua
//! function AccountSaveFrameMixin:OnEvent(event, ...)
//!     if event == "ACCOUNT_SAVE_ENABLED_UPDATE" or event == "ACCOUNT_LOCKED_POST_SAVE_UPDATE" then
//!         self:UpdateAccountState();
//!     elseif event == "ACCOUNT_SAVE_RESULT" then
//!         local result, outputFolderPath, outputFilePath = ...;
//!         if result == Enum.AccountExportResult.Success then
//!             local fileLink = "<a href=\"" .. outputFolderPath .. "\">".. outputFilePath .. "</a>";
//!             local successMessage = HTML_START_CENTERED .. ACCOUNT_SAVE_SUCCESS .. "|n|n"
//!                                    .. ACCOUNT_SAVE_SUCCESS_DETAILS .. "|n|n"
//!                                    .. fileLink .. HTML_END;
//!             local text2 = nil;
//!             StaticPopup_Show("ACCOUNT_SAVE_SUCCESS", successMessage, text2, outputFolderPath);
//!         else
//!             self:ProcessAccountSaveError(result);
//!         end
//!         self:UpdateAccountState();
//!     end
//! end
//! ```
//!
//! Three observables are pinned for the success path:
//!   1. `StaticPopup_Show` is called with `which == "ACCOUNT_SAVE_SUCCESS"`.
//!   2. The body text contains the formatted file link
//!      `<a href="<folder>"><file></a>` — the only fully dynamic
//!      portion of the message and the contract that proves both
//!      arguments survived the concatenation in order.
//!   3. The `data` argument equals `outputFolderPath` (the popup's
//!      OnAccept handler uses this to open the folder).
//!
//! The XML wires `<OnEvent method="OnEvent"/>`
//! (Blizzard_AccountSaveUI.xml:80), so `FireEvent("ACCOUNT_SAVE_RESULT", ...)`
//! dispatches through the addon's mixin handler. The
//! `surface_events` fixture pins the registration; this fixture
//! exercises the actual event payload routing end-to-end.
//!
//! `OnEvent` always calls `UpdateAccountState` at the end, which can
//! itself trigger `StaticPopup_Show("ACCOUNT_SAVE_IN_PROGRESS")` when
//! `account_save_in_progress` is true. The probe filters captured
//! calls by `which == "ACCOUNT_SAVE_SUCCESS"` so the assertion
//! tolerates that secondary popup; the precondition also clears
//! `account_save_in_progress` to keep the noise down.

use crate::common::blizzard_addon_harness::with_blizzard_addon_glue_smoke_shape;

const ROOT: &str = "Blizzard_AccountSaveUI";
const OUTPUT_FOLDER: &str = "/tmp/account-saves";
const OUTPUT_FILE: &str = "ProgressionArchive.zip";

#[test]
fn account_save_result_success_shows_popup_with_file_link() {
    with_blizzard_addon_glue_smoke_shape(&[ROOT], &[], |env, _loaded| {
        {
            let mut state = env.state().borrow_mut();
            state.account_save_enabled = true;
            state.account_locked_post_save = false;
            state.account_save_in_progress = false;
        }

        let probe_src = format!(
            r#"
            assert(AccountSaveFrame, "AccountSaveFrame global must exist after Blizzard_AccountSaveUI load")
            assert(Enum and Enum.AccountExportResult and Enum.AccountExportResult.Success == 0,
                   "Enum.AccountExportResult.Success must equal 0 (set by missing_enums.lua)")

            local captured = {{}}
            local original_show = StaticPopup_Show
            StaticPopup_Show = function(which, text, text2, data)
                captured[#captured + 1] = {{
                    which = which,
                    text = text,
                    text2 = text2,
                    data = data,
                }}
                return original_show(which, text, text2, data)
            end

            FireEvent("ACCOUNT_SAVE_RESULT",
                      Enum.AccountExportResult.Success,
                      "{folder}",
                      "{file}")

            local hit_which, hit_text, hit_data = nil, nil, nil
            for _, call in ipairs(captured) do
                if call.which == "ACCOUNT_SAVE_SUCCESS" then
                    hit_which, hit_text, hit_data = call.which, call.text, call.data
                    break
                end
            end

            return hit_which or "<missing>",
                   hit_text or "<missing>",
                   hit_data or "<missing>"
            "#,
            folder = OUTPUT_FOLDER,
            file = OUTPUT_FILE,
        );

        let (popup_name, popup_text, popup_data) = env
            .eval::<(String, String, String)>(&probe_src)
            .expect("ACCOUNT_SAVE_RESULT success-event probe must run cleanly");

        assert_eq!(
            popup_name, "ACCOUNT_SAVE_SUCCESS",
            "OnEvent must call StaticPopup_Show with `which = \"ACCOUNT_SAVE_SUCCESS\"` when \
             ACCOUNT_SAVE_RESULT fires with Enum.AccountExportResult.Success \
             (Blizzard_AccountSaveUI.lua:135). A `<missing>` here means the success branch \
             never reached StaticPopup_Show — either the OnEvent handler isn't wired \
             (XML `<OnEvent method=\"OnEvent\"/>` regression) or the `result == \
             Enum.AccountExportResult.Success` comparison is failing (enum value drift). Got: \
             `{popup_name}`."
        );

        let expected_link = format!("<a href=\"{OUTPUT_FOLDER}\">{OUTPUT_FILE}</a>");
        assert!(
            popup_text.contains(&expected_link),
            "ACCOUNT_SAVE_SUCCESS popup text must contain the formatted file link \
             `{expected_link}` (Blizzard_AccountSaveUI.lua:132 — \
             `\"<a href=\\\"\" .. outputFolderPath .. \"\\\">\".. outputFilePath .. \"</a>\"`). \
             This is the only fully-dynamic portion of the success message; pinning the \
             substring proves both event payload args were concatenated in order. A regression \
             that swapped the two args would still produce a visually-similar message but \
             with the folder and filename transposed inside the link, which would break \
             clicking the link to open the folder. Got popup_text: `{popup_text}`."
        );

        assert_eq!(
            popup_data, OUTPUT_FOLDER,
            "ACCOUNT_SAVE_SUCCESS popup `data` argument must equal `outputFolderPath` \
             (Blizzard_AccountSaveUI.lua:135 — `StaticPopup_Show(..., outputFolderPath)`). \
             The popup's OnAccept handler reads `data` to know which folder to open; if this \
             argument drops to nil, the user gets the success message but acknowledging it \
             does not open the saves folder. Got popup_data: `{popup_data}`, expected \
             `{OUTPUT_FOLDER}`."
        );
    });
}
