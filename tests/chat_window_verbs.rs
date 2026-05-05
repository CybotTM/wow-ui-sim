//! Integration tests for `src/lua_api/globals/chat_window_verbs.rs`.

use wow_ui_sim::lua_api::WowLuaEnv;

fn env() -> WowLuaEnv {
    WowLuaEnv::new().expect("WowLuaEnv init")
}

// ── SetChatWindowAlpha ────────────────────────────────────────────────────────

#[test]
fn set_chat_window_alpha_creates_window_and_stores_value() {
    let env = env();
    env.exec("SetChatWindowAlpha(1, 0.75)").unwrap();
    let st = env.state().borrow();
    let w = st.chat_windows.get(&1).expect("window 1 must exist");
    assert!((w.alpha - 0.75).abs() < 1e-6);
}

#[test]
fn set_chat_window_alpha_clamps_out_of_range_values() {
    let env = env();
    env.exec(
        "SetChatWindowAlpha(1, 2.0)
               SetChatWindowAlpha(2, -1.0)",
    )
    .unwrap();
    let st = env.state().borrow();
    assert!((st.chat_windows[&1].alpha - 1.0).abs() < 1e-6);
    assert!((st.chat_windows[&2].alpha - 0.0).abs() < 1e-6);
}

// ── SetChatWindowColor ────────────────────────────────────────────────────────

#[test]
fn set_chat_window_color_stores_rgb_triple() {
    let env = env();
    env.exec("SetChatWindowColor(1, 0.1, 0.2, 0.3)").unwrap();
    let st = env.state().borrow();
    let w = &st.chat_windows[&1];
    assert!((w.r - 0.1).abs() < 1e-6);
    assert!((w.g - 0.2).abs() < 1e-6);
    assert!((w.b - 0.3).abs() < 1e-6);
}

// ── SetChatWindowLocked / Uninteractable ──────────────────────────────────────

#[test]
fn set_chat_window_locked_true_and_false() {
    let env = env();
    env.exec("SetChatWindowLocked(1, true)").unwrap();
    assert!(env.state().borrow().chat_windows[&1].locked);
    env.exec("SetChatWindowLocked(1, false)").unwrap();
    assert!(!env.state().borrow().chat_windows[&1].locked);
}

#[test]
fn set_chat_window_locked_default_is_true_when_arg_omitted() {
    let env = env();
    env.exec("SetChatWindowLocked(1)").unwrap();
    assert!(env.state().borrow().chat_windows[&1].locked);
}

#[test]
fn set_chat_window_uninteractable_round_trips() {
    let env = env();
    env.exec("SetChatWindowUninteractable(2, true)").unwrap();
    assert!(env.state().borrow().chat_windows[&2].uninteractable);
}

// ── AddChatWindowChannel ──────────────────────────────────────────────────────

#[test]
fn add_chat_window_channel_appends_unique_channels() {
    let env = env();
    env.exec(
        r#"AddChatWindowChannel(1, "Trade")
           AddChatWindowChannel(1, "General")
           AddChatWindowChannel(1, "Trade")"#,
    )
    .unwrap();
    let st = env.state().borrow();
    let w = &st.chat_windows[&1];
    assert_eq!(w.channels, vec!["Trade".to_string(), "General".to_string()]);
    assert!(w.channel_names.contains("Trade"));
    assert!(w.channel_names.contains("General"));
    assert_eq!(w.channel_names.len(), 2);
}

// ── ChangeChatColor ───────────────────────────────────────────────────────────

#[test]
fn change_chat_color_stores_uppercase_channel_key() {
    let env = env();
    env.exec(r#"ChangeChatColor("say", 1, 0, 0)"#).unwrap();
    let st = env.state().borrow();
    let color = st.chat_type_colors.get("SAY").copied().unwrap_or_default();
    assert!((color.0 - 1.0).abs() < 1e-6);
    assert!((color.1 - 0.0).abs() < 1e-6);
    assert!((color.2 - 0.0).abs() < 1e-6);
}

// ── GetChatWindowChannels ─────────────────────────────────────────────────────

#[test]
fn get_chat_window_channels_returns_flat_list() {
    let env = env();
    env.exec(
        r#"AddChatWindowChannel(1, "Trade")
           AddChatWindowChannel(1, "General")"#,
    )
    .unwrap();
    let (a, b): (String, String) = env
        .eval(
            r#"
            local t = GetChatWindowChannels(1)
            return t[1], t[2]
            "#,
        )
        .unwrap();
    assert_eq!(a, "Trade");
    assert_eq!(b, "General");
}

#[test]
fn get_chat_window_channels_empty_for_unknown_window() {
    let env = env();
    let count: i64 = env.eval("return #GetChatWindowChannels(99)").unwrap();
    assert_eq!(count, 0);
}

// ── GetChatWindowMessages ─────────────────────────────────────────────────────

#[test]
fn get_chat_window_messages_returns_subscribed_types() {
    let env = env();
    {
        let mut st = env.state().borrow_mut();
        let w = st.chat_windows.entry(1).or_default();
        w.messages.push("SAY".into());
        w.messages.push("YELL".into());
    }
    let (a, b): (String, String) = env
        .eval(
            r#"
            local t = GetChatWindowMessages(1)
            return t[1], t[2]
            "#,
        )
        .unwrap();
    assert_eq!(a, "SAY");
    assert_eq!(b, "YELL");
}
