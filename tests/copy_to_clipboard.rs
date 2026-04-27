//! Integration tests for the `CopyToClipboard(text, removeMarkup?)` global.
//!
//! Drives `Blizzard_APIDocumentation/Blizzard_APIDocumentation.lua`
//! (`APIDocumentationMixin:HandleCopyAPI` at line 66 calls
//! `CopyToClipboard(clipboardString)` after the user runs
//! `/api copyapi <name>`).

use wow_ui_sim::lua_api::WowLuaEnv;

fn env() -> WowLuaEnv {
    WowLuaEnv::new().expect("Failed to create Lua environment")
}

#[test]
fn copy_to_clipboard_records_plain_text_and_returns_true() {
    let env = env();
    let returned: bool = env
        .eval(r#"return CopyToClipboard("GetTime() — returns elapsedSeconds: number")"#)
        .unwrap();
    assert!(returned, "CopyToClipboard must return true on success");
    let sim = env.state().borrow();
    assert_eq!(
        sim.clipboard.last_text.as_deref(),
        Some("GetTime() — returns elapsedSeconds: number"),
        "plain string must round-trip into the simulator clipboard slot",
    );
    assert!(
        !sim.clipboard.last_remove_markup,
        "default removeMarkup is false when arg #2 is omitted",
    );
}

#[test]
fn copy_to_clipboard_overwrites_previous_capture() {
    let env = env();
    env.eval::<()>(
        r#"
        CopyToClipboard("first")
        CopyToClipboard("second")
        "#,
    )
    .unwrap();
    assert_eq!(
        env.state().borrow().clipboard.last_text.as_deref(),
        Some("second"),
        "only the latest CopyToClipboard payload is retained",
    );
}

#[test]
fn copy_to_clipboard_strips_markup_when_requested() {
    let env = env();
    env.eval::<()>(
        r#"
        CopyToClipboard("|cFFFFD200GetTime|r — |Hapidoc:default:GetTime|h[link]|h", true)
        "#,
    )
    .unwrap();
    let sim = env.state().borrow();
    assert_eq!(
        sim.clipboard.last_text.as_deref(),
        Some("GetTime — [link]"),
        "removeMarkup=true must drop |c…|r color codes and the |H…|h/|h hyperlink wrappers, leaving the visible link label",
    );
    assert!(
        sim.clipboard.last_remove_markup,
        "the requested removeMarkup flag must be recorded",
    );
}

#[test]
fn copy_to_clipboard_keeps_markup_when_flag_false() {
    let env = env();
    env.eval::<()>(r#"CopyToClipboard("|cFFFFD200kept|r", false)"#)
        .unwrap();
    let sim = env.state().borrow();
    assert_eq!(
        sim.clipboard.last_text.as_deref(),
        Some("|cFFFFD200kept|r"),
        "markup must be preserved verbatim when removeMarkup is explicitly false",
    );
    assert!(
        !sim.clipboard.last_remove_markup,
        "the explicit false flag must be recorded",
    );
}

#[test]
fn copy_to_clipboard_records_empty_string_for_nil_arg() {
    let env = env();
    let returned: bool = env.eval("return CopyToClipboard()").unwrap();
    assert!(returned, "nil-arg call still returns true");
    assert_eq!(
        env.state().borrow().clipboard.last_text.as_deref(),
        Some(""),
        "missing first arg coerces to an empty string capture, matching val_to_string",
    );
}
