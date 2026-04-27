//! Integration tests for the `ChatFrameUtil.OpenChat(text, chatType?, cursorPosition?)`
//! helper. Drives `Blizzard_APIDocumentation/Blizzard_APIDocumentation.lua`
//! (`APIDocumentationMixin:HandleOpenDump` at line 81 calls
//! `ChatFrameUtil.OpenChat(dumpString, nil, desiredCursorPosition)`
//! after `/api dump <name>`), and the broader chat-edit flows in
//! `Blizzard_ChatFrameBase/Shared/ChatFrameUtil.lua:358`.

use wow_ui_sim::lua_api::WowLuaEnv;

fn env() -> WowLuaEnv {
    WowLuaEnv::new().expect("Failed to create Lua environment")
}

#[test]
fn open_chat_records_text_and_cursor_with_nil_chat_type() {
    let env = env();
    env.eval::<()>(r#"ChatFrameUtil.OpenChat("/dump GetTime", nil, 7)"#)
        .unwrap();
    let sim = env.state().borrow();
    let captured = sim
        .chat_edit_open_state
        .as_ref()
        .expect("OpenChat must populate chat_edit_open_state");
    assert_eq!(
        captured.text, "/dump GetTime",
        "first arg must round-trip into the captured text",
    );
    assert!(
        captured.chat_type.is_none(),
        "passing nil for the second arg must yield None",
    );
    assert_eq!(
        captured.cursor_position,
        Some(7),
        "third arg must round-trip as the cursor byte offset",
    );
}

#[test]
fn open_chat_records_chat_type_string_when_provided() {
    let env = env();
    env.eval::<()>(r#"ChatFrameUtil.OpenChat("hello", "SAY")"#)
        .unwrap();
    let sim = env.state().borrow();
    let captured = sim.chat_edit_open_state.as_ref().unwrap();
    assert_eq!(captured.text, "hello");
    assert_eq!(
        captured.chat_type.as_deref(),
        Some("SAY"),
        "string second arg must be captured verbatim",
    );
    assert!(
        captured.cursor_position.is_none(),
        "missing cursor arg must yield None",
    );
}

#[test]
fn open_chat_overwrites_previous_state() {
    let env = env();
    env.eval::<()>(
        r#"
        ChatFrameUtil.OpenChat("first", nil, 1)
        ChatFrameUtil.OpenChat("second", nil, 2)
        "#,
    )
    .unwrap();
    let sim = env.state().borrow();
    let captured = sim.chat_edit_open_state.as_ref().unwrap();
    assert_eq!(captured.text, "second");
    assert_eq!(captured.cursor_position, Some(2));
}

#[test]
fn open_chat_shows_and_sets_text_on_default_edit_box_when_present() {
    let env = env();
    env.eval::<()>(
        r#"
        ChatFrame1EditBox = {
            __shown = false,
            __text = nil,
            Show = function(self) self.__shown = true end,
            SetText = function(self, text) self.__text = text end,
        }
        ChatFrameUtil.OpenChat("/dump GetTime", nil, 7)
        "#,
    )
    .unwrap();
    let shown: bool = env.eval("return ChatFrame1EditBox.__shown").unwrap();
    let text: String = env.eval("return ChatFrame1EditBox.__text").unwrap();
    assert!(
        shown,
        "OpenChat must call Show on ChatFrame1EditBox when it exists",
    );
    assert_eq!(
        text, "/dump GetTime",
        "OpenChat must call SetText with the supplied text",
    );
}

#[test]
fn open_chat_is_a_no_op_on_edit_box_when_globals_missing() {
    let env = env();
    // No ChatFrame1EditBox global seeded — call must still record
    // chat_edit_open_state without erroring.
    env.eval::<()>(r#"ChatFrameUtil.OpenChat("x")"#).unwrap();
    let sim = env.state().borrow();
    assert_eq!(
        sim.chat_edit_open_state.as_ref().unwrap().text,
        "x",
        "missing edit box must not stop the state capture",
    );
}
