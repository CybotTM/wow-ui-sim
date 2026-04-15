//! Integration test for the Blizzard chat frame.
//!
//! Loads the Blizzard UI, clicks on ChatFrame1EditBox, types a message,
//! presses Enter, and verifies the message was submitted via
//! C_ChatInfo.SendChatMessage.

mod common;

use std::path::PathBuf;
use wow_ui_sim::loader::{discover_blizzard_addons, load_addon};
use wow_ui_sim::lua_api::WowLuaEnv;

const CHAT_LAYOUT_DEBUG_LUA: &str = r#"
    local frames = {
        {"ChatFrame1", ChatFrame1},
        {"ChatFrame1Background", ChatFrame1Background},
        {"ChatFrame1.ResizeButton", ChatFrame1.ResizeButton},
        {"ChatFrame1.ScrollToBottomButton", ChatFrame1.ScrollToBottomButton},
        {"ChatFrame1.ScrollBar", ChatFrame1.ScrollBar},
        {"ChatFrame1EditBox", ChatFrame1EditBox},
    }

    local out = {}
    for _, item in ipairs(frames) do
        local label, frame = item[1], item[2]
        if frame then
            local x, y, w, h = frame:GetRect()
            local points = {}
            for i = 1, frame:GetNumPoints() do
                local point, rel, relPoint, ox, oy = frame:GetPoint(i)
                local relName = rel and rel:GetName() or "$parent"
                table.insert(points, string.format("%s->%s:%s(%.0f,%.0f)", point, relName, relPoint, ox, oy))
            end
            table.sort(points)
            table.insert(
                out,
                string.format(
                    "%s rect=(%.0f,%.0f %.0fx%.0f) shown=%s points=%s",
                    label,
                    x or -1,
                    y or -1,
                    w or -1,
                    h or -1,
                    tostring(frame:IsShown()),
                    table.concat(points, " | ")
                )
            )
        else
            table.insert(out, label .. " <nil>")
        end
    end
    return table.concat(out, "\n")
"#;

fn blizzard_ui_dir() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("Interface/BlizzardUI")
}

/// Create a fully loaded environment with all Blizzard addons and startup events.
fn setup_env() -> WowLuaEnv {
    let env = WowLuaEnv::new().expect("Failed to create Lua environment");
    env.set_screen_size(1024.0, 768.0);

    let ui = blizzard_ui_dir();
    {
        let mut state = env.state().borrow_mut();
        state.addon_base_paths = vec![ui.clone()];
    }

    let addons = discover_blizzard_addons(&ui);
    for (name, toc_path) in &addons {
        if let Err(e) = load_addon(&env.loader_env(), toc_path) {
            eprintln!("[load {name}] FAILED: {e}");
        }
    }

    env.apply_post_load_workarounds();
    fire_startup_events(&env);
    env
}

/// Fire startup events (same sequence as main.rs).
fn fire_startup_events(env: &WowLuaEnv) {
    common::fire_addon_loaded(env, "WoWUISim");
    for event in ["VARIABLES_LOADED", "PLAYER_LOGIN"] {
        let _ = env.fire_event(event);
    }
    common::call_global_if_present(env, "RequestTimePlayed");
    common::fire_player_entering_world(env, true, false);
    for event in [
        "UPDATE_BINDINGS",
        "DISPLAY_SIZE_CHANGED",
        "UI_SCALE_CHANGED",
    ] {
        let _ = env.fire_event(event);
    }
}

fn chat_layout_debug(env: &WowLuaEnv) -> String {
    env.eval(CHAT_LAYOUT_DEBUG_LUA)
        .expect("chat layout debug eval failed")
}

/// Hook C_ChatInfo.SendChatMessage to capture submitted messages.
fn hook_send_chat_message(env: &WowLuaEnv) {
    env.exec(
        r#"
        _G.__test_sent_messages = {}
        local orig = C_ChatInfo.SendChatMessage
        C_ChatInfo.SendChatMessage = function(msg, chatType, language, target)
            table.insert(_G.__test_sent_messages, {
                message = msg,
                chatType = chatType,
                language = language,
                target = target,
            })
            if orig then orig(msg, chatType, language, target) end
        end
    "#,
    )
    .expect("Failed to hook SendChatMessage");
}

/// Type a string into the focused EditBox character by character.
fn type_text(env: &WowLuaEnv, text: &str) {
    for ch in text.chars() {
        let s = ch.to_string();
        let key = if ch == ' ' {
            "SPACE".to_string()
        } else {
            s.to_uppercase()
        };
        env.send_key_press(&key, Some(&s)).unwrap();
    }
}

/// Click on ChatFrame1EditBox and verify it gains focus.
fn click_chat_editbox(env: &WowLuaEnv) {
    let frame_id = env
        .state()
        .borrow()
        .widgets
        .get_id_by_name("ChatFrame1EditBox")
        .expect("ChatFrame1EditBox not found in widget registry");
    env.send_click(frame_id).expect("send_click failed");

    let has_focus: bool = env
        .eval("return ChatFrame1EditBox:HasFocus()")
        .expect("HasFocus failed");
    assert!(has_focus, "ChatFrame1EditBox should have focus after click");
}

/// Assert exactly one message was sent with expected text and chat type.
fn assert_message_sent(env: &WowLuaEnv, expected_text: &str, expected_type: &str) {
    let count: i32 = env
        .eval("return #_G.__test_sent_messages")
        .expect("eval failed");
    assert_eq!(count, 1, "Exactly one message should have been sent");

    let message: String = env
        .eval("return _G.__test_sent_messages[1].message")
        .expect("eval failed");
    assert_eq!(
        message, expected_text,
        "Sent message should match typed text"
    );

    let chat_type: String = env
        .eval("return _G.__test_sent_messages[1].chatType")
        .expect("eval failed");
    assert_eq!(chat_type, expected_type, "Chat type should match expected");

    let text_after: String = env
        .eval("return ChatFrame1EditBox:GetText() or ''")
        .expect("GetText failed");
    assert_eq!(text_after, "", "EditBox should be cleared after submit");
}

#[test]
fn test_chat_editbox_click_type_and_submit() {
    test_timeout! {
        let env = setup_env();

        let exists: bool = env
            .eval("return ChatFrame1EditBox ~= nil")
            .expect("eval failed");
        assert!(exists, "ChatFrame1EditBox should exist after loading Blizzard UI");

        hook_send_chat_message(&env);

        let has_focus: bool = env
            .eval("return ChatFrame1EditBox:HasFocus()")
            .expect("HasFocus failed");
        assert!(!has_focus, "ChatFrame1EditBox should not have focus initially");

        click_chat_editbox(&env);
        type_text(&env, "hello world");

        let text: String = env
            .eval("return ChatFrame1EditBox:GetText()")
            .expect("GetText failed");
        assert_eq!(text, "hello world", "EditBox should contain typed text");

        env.send_key_press("ENTER", None)
            .expect("ENTER key press failed");

        assert_message_sent(&env, "hello world", "SAY");

        let message: String = env
            .eval("return _G.__test_sent_messages[1].message")
            .expect("eval failed");
        assert_eq!(message, "hello world", "Sent message should match typed text");

        let chat_type: String = env
            .eval("return _G.__test_sent_messages[1].chatType")
            .expect("eval failed");
        assert_eq!(chat_type, "SAY", "Default chat type should be SAY");

        let text_after: String = env
            .eval("return ChatFrame1EditBox:GetText() or ''")
            .expect("GetText failed");
        assert_eq!(text_after, "", "EditBox should be cleared after submit");
    }
}

#[test]
fn test_chat_message_contains_timestamp() {
    test_timeout! {
        let env = setup_env();

        // Enable timestamps (default CVar is "none")
        env.exec(r#"SetCVar("showTimestamps", "%H:%M ")"#).unwrap();

        // Send a chat message — C_ChatInfo.SendChatMessage adds it to ChatFrame1
        env.exec(r#"C_ChatInfo.SendChatMessage("Test timestamp", "SAY")"#)
            .unwrap();

        // Get the last message text from ChatFrame1
        let msg: String = env
            .eval(
                r#"
        local n = ChatFrame1:GetNumMessages()
        local text = ChatFrame1:GetMessageInfo(n)
        return text
    "#,
            )
            .unwrap();

        // Message should start with a time like "14:32 " (HH:MM followed by space)
        let has_time = msg.len() >= 6
            && msg.as_bytes()[2] == b':'
            && msg.as_bytes()[0].is_ascii_digit()
            && msg.as_bytes()[1].is_ascii_digit()
            && msg.as_bytes()[3].is_ascii_digit()
            && msg.as_bytes()[4].is_ascii_digit()
            && msg.as_bytes()[5] == b' ';
        assert!(
            has_time,
            "Chat message should start with HH:MM timestamp, got: {msg:.40}"
        );
    }
}

#[test]
fn test_chat_editbox_text_color_after_activation() {
    test_timeout! {
        let env = setup_env();

        click_chat_editbox(&env);

        // After activation, ActivateChat should have called UpdateHeader
        // which sets text color to white (ChatTypeInfo default = 1.0, 1.0, 1.0)
        let (r, g, b): (f64, f64, f64) = env
            .eval("return ChatFrame1EditBox:GetTextColor()")
            .expect("GetTextColor failed");
        assert!(
            (r - 1.0).abs() < 0.01 && (g - 1.0).abs() < 0.01 && (b - 1.0).abs() < 0.01,
            "EditBox text color should be white after activation, got ({r}, {g}, {b})"
        );

        // Alpha should be 1.0 after activation
        let alpha: f64 = env
            .eval("return ChatFrame1EditBox:GetAlpha()")
            .expect("GetAlpha failed");
        assert!(
            (alpha - 1.0).abs() < 0.01,
            "EditBox alpha should be 1.0 after activation, got {alpha}"
        );
    }
}

#[test]
fn test_chat_scrollbar_stays_attached_to_chat_frame_right_edge() {
    test_timeout! {
        let env = setup_env();
        let layout = chat_layout_debug(&env);

        let (chat_x, _chat_y, chat_w, _chat_h): (f64, f64, f64, f64) = env
            .eval("local x, y, w, h = ChatFrame1:GetRect(); return x, y, w, h")
            .expect("ChatFrame1:GetRect failed");
        let (scroll_x, _scroll_y, scroll_w, _scroll_h): (f64, f64, f64, f64) = env
            .eval("local x, y, w, h = ChatFrame1.ScrollBar:GetRect(); return x, y, w, h")
            .expect("ChatFrame1.ScrollBar:GetRect failed");
        let (_edit_x, _edit_y, edit_w, _edit_h): (f64, f64, f64, f64) = env
            .eval("local x, y, w, h = ChatFrame1EditBox:GetRect(); return x, y, w, h")
            .expect("ChatFrame1EditBox:GetRect failed");
        let points: String = env
            .eval(
                r#"
            local out = {}
            for i = 1, ChatFrame1.ScrollBar:GetNumPoints() do
                local point, rel, relPoint, x, y = ChatFrame1.ScrollBar:GetPoint(i)
                local relName = rel and rel:GetName() or "$parent"
                table.insert(out, string.format("%s->%s:%s(%.0f,%.0f)", point, relName, relPoint, x, y))
            end
            table.sort(out)
            return table.concat(out, " | ")
        "#,
            )
            .expect("ChatFrame1.ScrollBar:GetPoint failed");

        let chat_right = chat_x + chat_w;
        assert!(
            (scroll_x - chat_right).abs() <= 30.0,
            "ChatFrame1.ScrollBar should stay near ChatFrame1 right edge. chat=({:.0}, w={:.0}) scroll=({:.0}, w={:.0}) anchors={}\n{}",
            chat_x,
            chat_w,
            scroll_x,
            scroll_w,
            points,
            layout
        );
        assert!(
            (4.0..=32.0).contains(&scroll_w),
            "ChatFrame1.ScrollBar width should stay sane, got {scroll_w}. anchors={points}\n{layout}"
        );
        assert!(
            (350.0..=600.0).contains(&edit_w),
            "ChatFrame1EditBox width should stay sane, got {edit_w}. scrollbar anchors={points}\n{layout}"
        );
        assert!(
            !points.contains("TOP->$parent:TOP") && !points.contains("BOTTOM->$parent:BOTTOM"),
            "ChatFrame1.ScrollBar should not keep inner Track anchors after startup: {points}\n{layout}"
        );
    }
}
