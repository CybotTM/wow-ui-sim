//! Behavior pin: every `Enum.AccountExportResult` failure variant
//! routes through `ProcessAccountSaveError` to the matching localized
//! `ACCOUNT_SAVE_ERROR_*` global string.
//!
//! Source (`Interface/BlizzardUI/Blizzard_AccountSaveUI/
//! Blizzard_AccountSaveUI.lua`, lines 144-167):
//!
//! ```lua
//! function AccountSaveFrameMixin:ProcessAccountSaveError(errorCode)
//!     local errorText;
//!     if errorCode == Enum.AccountExportResult.TimedOut then
//!         errorText = ACCOUNT_SAVE_ERROR_TIMEOUT;
//!     elseif errorCode == Enum.AccountExportResult.NoAccountFound then
//!         errorText = ACCOUNT_SAVE_ERROR_INVALID_ACCOUNT;
//!     elseif errorCode == Enum.AccountExportResult.RequestedInvalidCharacter then
//!         errorText = ACCOUNT_SAVE_ERROR_INVALID_CHARACTER;
//!     elseif errorCode == Enum.AccountExportResult.FileInvalid then
//!         errorText = ACCOUNT_SAVE_ERROR_FILE_INVALID;
//!     elseif errorCode == Enum.AccountExportResult.FileWriteFailed then
//!         errorText = ACCOUNT_SAVE_ERROR_FILE_WRITE;
//!     elseif errorCode == Enum.AccountExportResult.Unavailable then
//!         errorText = ACCOUNT_SAVE_ERROR_UNAVAILABLE;
//!     elseif errorCode == Enum.AccountExportResult.AlreadyInProgress then
//!         errorText = ACCOUNT_SAVE_ERROR_ALREADY_IN_PROGRESS;
//!     elseif errorCode == Enum.AccountExportResult.Cancelled then
//!         errorText = ACCOUNT_SAVE_ERROR_CANCELLED;
//!     else
//!         errorText = ACCOUNT_SAVE_ERROR_OTHER;
//!     end
//!
//!     StaticPopup_Hide("OKAY_MUST_ACCEPT", errorText);
//! end
//! ```
//!
//! `OnEvent` routes the non-Success branch of `ACCOUNT_SAVE_RESULT`
//! through `self:ProcessAccountSaveError(result)`
//! (Blizzard_AccountSaveUI.lua:137), so firing the event with an error
//! variant exercises the same string-selection table that the C-API
//! failure path also uses (`SaveAccountData`, line 119). Driving
//! through `OnEvent` rather than calling the method directly proves
//! the event payload threads the result through unchanged AND that the
//! string-selection table is reachable from the event-driven path.
//!
//! The addon calls `StaticPopup_Hide("OKAY_MUST_ACCEPT", errorText)`
//! (line 166) — Hide rather than Show, an upstream quirk that's
//! preserved verbatim from `Interface/BlizzardUI/`. The simulator stub
//! (`runtime_surface_bootstrap.lua`) is a no-op, so the test wraps
//! `StaticPopup_Hide` in a Lua-side capturing closure that records the
//! second argument when `which == "OKAY_MUST_ACCEPT"`. The wrapper
//! filters by `which` because `UpdateAccountState` (called at the tail
//! of `OnEvent`) also calls `StaticPopup_Hide("ACCOUNT_SAVE_IN_PROGRESS")`,
//! and we don't want that captured.
//!
//! Coverage:
//!   1. Each of the 8 explicit elseif branches drives a distinct
//!      Enum.AccountExportResult value and asserts its localized string.
//!   2. The fallthrough `else` branch is exercised with
//!      `Enum.AccountExportResult.UnknownError` (= 1), which has no
//!      explicit elseif and must land in `ACCOUNT_SAVE_ERROR_OTHER`.
//!      A regression that added a wrong elseif for UnknownError, or
//!      that lost the final `else`, would flip this single assertion
//!      while leaving the other 8 passing.

use crate::common::blizzard_addon_harness::with_blizzard_addon_glue_smoke_shape;

const ROOT: &str = "Blizzard_AccountSaveUI";

const ERROR_VARIANTS: &[(&str, i64, &str)] = &[
    ("TimedOut", 4, "Progression archive timed out"),
    (
        "NoAccountFound",
        5,
        "Progression archive download failed: Invalid account",
    ),
    (
        "RequestedInvalidCharacter",
        6,
        "Progression archive download failed: Invalid character",
    ),
    (
        "FileInvalid",
        8,
        "Progression archive download failed: Couldn't open the target output file",
    ),
    (
        "FileWriteFailed",
        9,
        "Progression archive download failed: Couldn't write to the target output file",
    ),
    (
        "Unavailable",
        10,
        "Progression archive downloading is currently unavailable",
    ),
    (
        "AlreadyInProgress",
        11,
        "A progression archive download is already in progress. Please try again later.",
    ),
    (
        "Cancelled",
        2,
        "Progression archive download was interrupted. Please try again later.",
    ),
    (
        "UnknownError-falls-through-to-Other",
        1,
        "An error has occurred. Please try again later.",
    ),
];

#[test]
fn account_save_result_error_variants_route_to_localized_strings() {
    with_blizzard_addon_glue_smoke_shape(&[ROOT], &[], |env, _loaded| {
        {
            let mut state = env.state().borrow_mut();
            state.account_save_enabled = true;
            state.account_locked_post_save = false;
            state.account_save_in_progress = false;
        }

        env.eval::<()>(
            r#"
            assert(AccountSaveFrame, "AccountSaveFrame global must exist after Blizzard_AccountSaveUI load")
            assert(Enum and Enum.AccountExportResult, "Enum.AccountExportResult must be populated by missing_enums.lua")

            captured_error_text = nil
            local original_hide = StaticPopup_Hide
            StaticPopup_Hide = function(which, text)
                if which == "OKAY_MUST_ACCEPT" then
                    captured_error_text = text
                end
                return original_hide(which, text)
            end
            "#,
        )
        .expect("StaticPopup_Hide wrapper install probe must run cleanly");

        for (variant_name, error_code, expected_text) in ERROR_VARIANTS {
            let probe_src = format!(
                r#"
                captured_error_text = nil
                FireEvent("ACCOUNT_SAVE_RESULT", {error_code}, "", "")
                return captured_error_text or "<missing>"
                "#,
            );

            let captured_text = env
                .eval::<String>(&probe_src)
                .unwrap_or_else(|_| panic!("ACCOUNT_SAVE_RESULT error variant `{variant_name}` (code {error_code}) probe must run cleanly"));

            assert_eq!(
                &captured_text, expected_text,
                "ProcessAccountSaveError must select the `ACCOUNT_SAVE_ERROR_*` global string \
                 corresponding to Enum.AccountExportResult.{variant_name} (= {error_code}) \
                 (Blizzard_AccountSaveUI.lua:144-164). The probe wraps StaticPopup_Hide and \
                 captures the second argument when `which == \"OKAY_MUST_ACCEPT\"`. A mismatch \
                 here means either: (a) the elseif chain selected the wrong branch (variant \
                 swap or comparison drift), or (b) the localized global string was retuned in \
                 data/global_strings.rs without updating the addon mapping, or (c) for the \
                 `UnknownError` case specifically, the final `else` clause was lost or replaced \
                 with an explicit elseif. Got: `{captured_text}`, expected: `{expected_text}`."
            );
        }
    });
}
