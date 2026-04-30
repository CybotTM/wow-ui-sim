//! Behavior pin: the `ACCOUNT_SAVE_SUCCESS` popup's `OnAccept` handler
//! invokes `LaunchURL(data)` so the user's "Open Folder" click reaches
//! the platform shell with the saved-archive folder path.
//!
//! Source (`Interface/BlizzardUI/Blizzard_AccountSaveUI/
//! Blizzard_AccountSaveUI.lua`, lines 12-23):
//!
//! ```lua
//! StaticPopupDialogs["ACCOUNT_SAVE_SUCCESS"] = {
//!     text = "",
//!     button1 = ACCOUNT_SAVE_FILE_BUTTON,
//!     button2 = ACCOUNT_SAVE_CLOSE_BUTTON,
//!     html = 1,
//!     explicitAcknowledge = true,
//!     OnAccept = function(dialog, data)
//!         LaunchURL(data);
//!     end,
//!     OnCancel = function(dialog, data)
//!     end,
//! }
//! ```
//!
//! `OnEvent` calls `StaticPopup_Show("ACCOUNT_SAVE_SUCCESS", successMessage,
//! text2, outputFolderPath)` (line 135) — the fourth argument is the
//! popup's `data` payload. When the popup's primary button
//! (`ACCOUNT_SAVE_FILE_BUTTON` = "Open Folder") is pressed, the popup
//! framework calls `OnAccept(dialog, data)`, which delegates to the
//! global `LaunchURL`. The simulator's `LaunchURL` stub
//! (`src/lua_api/globals/missing_surface.rs:364`) records the URL in
//! `SimState.last_launched_url` — it never opens an external browser,
//! but tests can assert what URL the addon would have launched.
//!
//! `behavior_save_result_event.rs` already pins that
//! `StaticPopup_Show`'s `data` argument equals `outputFolderPath`. This
//! fixture pins the *next* link in the chain — that pressing the
//! popup's primary button forwards `data` unchanged to `LaunchURL`.
//! Together the two fixtures pin the full
//! event-payload → popup-data → LaunchURL pipeline.
//!
//! The simulator has no live popup framework to dispatch button
//! clicks, so the test invokes the popup table's `OnAccept` directly
//! with a sentinel data path. This is the same call shape the real
//! popup framework uses (`info.OnAccept(dialog, dialog.data)`), so any
//! regression that broke the OnAccept contract — wrong arity, wrong
//! global name, swapped to `OnCancel`'s body — would surface here.
//!
//! Two contracts are pinned:
//!   1. `StaticPopupDialogs["ACCOUNT_SAVE_SUCCESS"].OnAccept` is a
//!      function (not nil, not a string). A regression that lost the
//!      handler entirely would surface as a Lua error rather than a
//!      silent no-op.
//!   2. Calling `OnAccept(nil, sentinel_path)` populates
//!      `SimState.last_launched_url` with `sentinel_path`. A regression
//!      that called `LaunchURL()` (no args), `LaunchURL(self)` (passing
//!      the dialog instead of data), or that swapped to `OnCancel`'s
//!      empty body, would all flip this assertion.

use crate::common::blizzard_addon_harness::with_blizzard_addon_glue_smoke_shape;

const ROOT: &str = "Blizzard_AccountSaveUI";
const SENTINEL_FOLDER: &str = "/tmp/account-saves/wow-archive-2026-04-30";

#[test]
fn account_save_success_popup_on_accept_launches_url_with_folder_path() {
    with_blizzard_addon_glue_smoke_shape(&[ROOT], &[], |env, _loaded| {
        {
            let mut state = env.state().borrow_mut();
            state.last_launched_url = None;
        }

        let on_accept_is_function = env
            .eval::<bool>(
                r#"
                assert(StaticPopupDialogs, "StaticPopupDialogs global must exist")
                local dialog = StaticPopupDialogs["ACCOUNT_SAVE_SUCCESS"]
                assert(dialog, "ACCOUNT_SAVE_SUCCESS popup must be registered after Blizzard_AccountSaveUI load")
                return type(dialog.OnAccept) == "function"
                "#,
            )
            .expect("OnAccept-shape probe must run cleanly");

        assert!(
            on_accept_is_function,
            "StaticPopupDialogs[\"ACCOUNT_SAVE_SUCCESS\"].OnAccept must be a function \
             (Blizzard_AccountSaveUI.lua:18-20). The popup framework calls this as \
             `info.OnAccept(dialog, dialog.data)` when the user clicks the primary button. \
             A nil here means the dialog table dropped the handler — clicking \"Open Folder\" \
             would do nothing and the user's saved-archive folder would never open."
        );

        let probe_src = format!(
            r#"
            local dialog = StaticPopupDialogs["ACCOUNT_SAVE_SUCCESS"]
            dialog.OnAccept(nil, "{SENTINEL_FOLDER}")
            return true
            "#,
        );

        env.eval::<bool>(&probe_src)
            .expect("ACCOUNT_SAVE_SUCCESS OnAccept invocation probe must run cleanly");

        let captured = env.state().borrow().last_launched_url.clone();

        assert_eq!(
            captured.as_deref(),
            Some(SENTINEL_FOLDER),
            "ACCOUNT_SAVE_SUCCESS OnAccept must call `LaunchURL(data)` with the folder path \
             passed in as `data` (Blizzard_AccountSaveUI.lua:19 — `LaunchURL(data)`). The \
             simulator's LaunchURL stub records the URL in SimState.last_launched_url \
             (src/lua_api/globals/missing_surface.rs:364-368). A `None` here means OnAccept \
             ran but never reached LaunchURL — possibly the body was emptied (matching \
             OnCancel's no-op shape), the global was renamed, or the closure swallowed the \
             call. A different value means LaunchURL was called with the wrong argument — \
             passing `self` (the dialog) or `nil` instead of `data` would both register here. \
             Got: `{captured:?}`, expected: `Some({SENTINEL_FOLDER:?})`."
        );
    });
}
