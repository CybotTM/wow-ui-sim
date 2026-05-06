//! Integration tests for the key dispatch subsystem.
//!
//! Covers: ESC dispatch, keybinding lookup + execution, EditBox text input,
//! and unbound key as a silent no-op.

use wow_ui_sim::lua_api::WowLuaEnv;

fn env() -> WowLuaEnv {
    WowLuaEnv::new().expect("WowLuaEnv init")
}

// ── ESC dispatch ──────────────────────────────────────────────────────────────

#[test]
fn esc_with_focused_editbox_fires_on_escape_pressed_and_returns_early() {
    let env = env();
    // Create an editbox, set an OnEscapePressed that sets a flag and returns true.
    env.exec(
        r#"
        _G.escaped = false
        local eb = CreateFrame("EditBox", "TestEscEB", UIParent)
        eb:SetScript("OnEscapePressed", function(self)
            _G.escaped = true
            return true
        end)
        "#,
    )
    .unwrap();

    // Focus the editbox by clicking it.
    let eb_id = {
        let state = env.state();
        let sim = state.borrow();
        sim.widgets.get_id_by_name("TestEscEB")
    };
    if let Some(id) = eb_id {
        env.state().borrow_mut().focused_frame_id = Some(id);
    }

    env.send_key_press("ESCAPE", None).unwrap();

    let escaped: bool = env.eval("return _G.escaped").unwrap();
    assert!(escaped, "OnEscapePressed should have fired");
}

#[test]
fn esc_does_not_error_with_no_focus() {
    let env = env();
    // Sending ESC with no focus and no target should be a no-op (or toggle
    // GameMenuFrame) without panicking or returning an error.
    let result = env.send_key_press("ESCAPE", None);
    assert!(result.is_ok(), "ESC with no focus should not error");
}

#[test]
fn esc_closes_special_frame_that_is_visible() {
    let env = env();
    // Register a named frame in UISpecialFrames; ESC should hide it.
    env.exec(
        r#"
        local f = CreateFrame("Frame", "TestSpecialFrame", UIParent)
        f:Show()
        UISpecialFrames = UISpecialFrames or {}
        table.insert(UISpecialFrames, "TestSpecialFrame")
        -- Prevent CloseAllWindows from swallowing the result
        _G.CloseAllWindows = function() return nil end
        "#,
    )
    .unwrap();

    let visible_before: bool = env.eval(r#"return TestSpecialFrame:IsVisible()"#).unwrap();
    assert!(visible_before, "frame should start visible");

    env.send_key_press("ESCAPE", None).unwrap();

    let visible_after: bool = env
        .eval(r#"return TestSpecialFrame:IsVisible()"#)
        .unwrap_or(true);
    assert!(!visible_after, "ESC should have hidden the special frame");
}

// ── Keybinding dispatch ───────────────────────────────────────────────────────

#[test]
fn bound_key_dispatches_lua_code() {
    let env = env();
    // Set a custom binding whose Lua code sets a global flag.
    env.exec(
        r#"
        _G.binding_fired = false
        SetBinding("CTRL-X", "TOGGLEBACKPACK")
        -- Override ToggleBackpack to set our flag instead
        function ToggleBackpack() _G.binding_fired = true end
        "#,
    )
    .unwrap();

    env.send_key_press("CTRL-X", None).unwrap();

    let fired: bool = env.eval("return _G.binding_fired").unwrap();
    assert!(fired, "bound key should have dispatched its Lua code");
}

#[test]
fn unbound_key_is_silent_no_op() {
    let env = env();
    // A key that has no binding should do nothing and not error.
    let result = env.send_key_press("F12", None);
    assert!(result.is_ok(), "unbound key should be a silent no-op");
}

#[test]
fn default_keybindings_dispatch_on_init() {
    let env = env();
    env.exec(
        r#"
        _G.open_all_bags_fired = false
        function ToggleAllBags()
            _G.open_all_bags_fired = true
        end
        "#,
    )
    .unwrap();

    env.send_key_press("B", None).unwrap();

    let fired: bool = env.eval("return _G.open_all_bags_fired").unwrap();
    assert!(fired, "B should dispatch the default OPENALLBAGS binding");
}

// ── EditBox text input ────────────────────────────────────────────────────────

#[test]
fn editbox_receives_typed_text() {
    let env = env();
    env.exec(
        r#"
        local eb = CreateFrame("EditBox", "TestTypeEB", UIParent)
        eb:SetText("")
        "#,
    )
    .unwrap();

    // Focus the editbox.
    let eb_id = {
        let state = env.state();
        let sim = state.borrow();
        sim.widgets.get_id_by_name("TestTypeEB")
    };
    if let Some(id) = eb_id {
        env.state().borrow_mut().focused_frame_id = Some(id);
    }

    // Send individual characters.
    env.send_key_press("a", Some("a")).unwrap();
    env.send_key_press("b", Some("b")).unwrap();
    env.send_key_press("c", Some("c")).unwrap();

    // Read back via GetText.
    let text: String = env
        .eval(r#"return TestTypeEB:GetText()"#)
        .unwrap_or_default();
    assert_eq!(text, "abc", "EditBox should contain the typed characters");
}

#[test]
fn focused_editbox_inserts_printable_key_when_text_payload_is_missing() {
    let env = env();
    env.exec(
        r#"
        local eb = CreateFrame("EditBox", "TestFallbackTextEB", UIParent)
        eb:SetText("")
        eb:SetFocus()
        "#,
    )
    .unwrap();

    env.send_key_press("A", None).unwrap();
    env.send_key_press("B", None).unwrap();

    let text: String = env.eval(r#"return TestFallbackTextEB:GetText()"#).unwrap();
    assert_eq!(
        text, "ab",
        "focused EditBox should insert printable key names when the GUI event has no text payload"
    );
}

#[test]
fn editbox_on_text_changed_fires_on_input() {
    let env = env();
    env.exec(
        r#"
        _G.change_count = 0
        local eb = CreateFrame("EditBox", "TestChangedEB", UIParent)
        eb:SetScript("OnTextChanged", function(self, userInput)
            if userInput then _G.change_count = _G.change_count + 1 end
        end)
        "#,
    )
    .unwrap();

    let eb_id = {
        let state = env.state();
        let sim = state.borrow();
        sim.widgets.get_id_by_name("TestChangedEB")
    };
    if let Some(id) = eb_id {
        env.state().borrow_mut().focused_frame_id = Some(id);
    }

    env.send_key_press("x", Some("x")).unwrap();
    env.send_key_press("y", Some("y")).unwrap();

    let count: i64 = env.eval("return _G.change_count").unwrap_or(0);
    assert_eq!(count, 2, "OnTextChanged should fire once per character");
}

#[test]
fn editbox_backspace_removes_last_character() {
    let env = env();
    env.exec(
        r#"
        local eb = CreateFrame("EditBox", "TestBkspEB", UIParent)
        eb:SetText("hello")
        eb:SetCursorPosition(5)
        "#,
    )
    .unwrap();

    let eb_id = {
        let state = env.state();
        let sim = state.borrow();
        sim.widgets.get_id_by_name("TestBkspEB")
    };
    if let Some(id) = eb_id {
        env.state().borrow_mut().focused_frame_id = Some(id);
    }

    env.send_key_press("BACKSPACE", None).unwrap();

    let text: String = env
        .eval(r#"return TestBkspEB:GetText()"#)
        .unwrap_or_default();
    assert_eq!(text, "hell", "Backspace should remove the last character");
}

#[test]
fn editbox_cursor_keys_move_within_text_bounds() {
    let env = env();
    env.exec(
        r#"
        local eb = CreateFrame("EditBox", "TestCursorKeysEB", UIParent)
        eb:SetText("hello")
        eb:SetCursorPosition(2)
        eb:SetFocus()
        "#,
    )
    .unwrap();

    env.send_key_press("LEFT", None).unwrap();
    let left_pos: i64 = env
        .eval(r#"return TestCursorKeysEB:GetCursorPosition()"#)
        .unwrap();
    env.send_key_press("RIGHT", None).unwrap();
    let right_pos: i64 = env
        .eval(r#"return TestCursorKeysEB:GetCursorPosition()"#)
        .unwrap();
    env.send_key_press("END", None).unwrap();
    let end_pos: i64 = env
        .eval(r#"return TestCursorKeysEB:GetCursorPosition()"#)
        .unwrap();
    env.send_key_press("HOME", None).unwrap();
    let home_pos: i64 = env
        .eval(r#"return TestCursorKeysEB:GetCursorPosition()"#)
        .unwrap();

    assert_eq!(left_pos, 1);
    assert_eq!(right_pos, 2);
    assert_eq!(end_pos, 5);
    assert_eq!(home_pos, 0);
}

// ── OnKeyDown propagation ─────────────────────────────────────────────────────

#[test]
fn on_key_down_fires_on_focused_frame() {
    let env = env();
    env.exec(
        r#"
        _G.key_down = nil
        local f = CreateFrame("Frame", "TestKeyDownFrame", UIParent)
        f:EnableKeyboard(true)
        f:SetScript("OnKeyDown", function(self, key)
            _G.key_down = key
        end)
        "#,
    )
    .unwrap();

    let frame_id = {
        let state = env.state();
        let sim = state.borrow();
        sim.widgets.get_id_by_name("TestKeyDownFrame")
    };
    if let Some(id) = frame_id {
        env.state().borrow_mut().focused_frame_id = Some(id);
    }

    env.send_key_press("Q", None).unwrap();

    let key: Option<String> = env.eval("return _G.key_down").unwrap_or(None);
    assert_eq!(
        key.as_deref(),
        Some("Q"),
        "OnKeyDown should receive the key"
    );
}
