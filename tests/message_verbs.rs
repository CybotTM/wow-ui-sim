//! Integration tests for `src/lua_api/globals/message_verbs.rs`.

use wow_ui_sim::event::EventArg;
use wow_ui_sim::lua_api::WowLuaEnv;

fn env() -> WowLuaEnv {
    WowLuaEnv::new().expect("WowLuaEnv init")
}

// ── CancelAuction ─────────────────────────────────────────────────────────────

#[test]
fn cancel_auction_fires_event_with_index_arg() {
    let env = env();
    env.exec("CancelAuction(42)").unwrap();
    let st = env.state().borrow();
    let event = st
        .events
        .pending()
        .iter()
        .find(|e| e.name == "AUCTION_CANCELED")
        .expect("AUCTION_CANCELED must fire");
    assert_eq!(event.args.len(), 1);
    assert!(matches!(event.args[0], EventArg::Number(n) if (n - 42.0).abs() < 1e-6));
}

#[test]
fn cancel_auction_without_index_still_fires() {
    let env = env();
    env.exec("CancelAuction()").unwrap();
    let st = env.state().borrow();
    let event = st
        .events
        .pending()
        .iter()
        .find(|e| e.name == "AUCTION_CANCELED")
        .expect("must fire even without args");
    assert!(event.args.is_empty());
}

// ── SendAddonMessage ──────────────────────────────────────────────────────────

#[test]
fn send_addon_message_logs_entry_and_fires_chat_msg_addon() {
    let env = env();
    env.exec(r#"SendAddonMessage("MYADDON", "hello", "PARTY", "")"#)
        .unwrap();
    let st = env.state().borrow();
    let entry = st
        .message_log
        .last()
        .expect("message_log must have the entry");
    assert_eq!(entry.kind, "addon");
    assert_eq!(entry.prefix, "MYADDON");
    assert_eq!(entry.message, "hello");
    assert_eq!(entry.channel, "PARTY");

    let event = st
        .events
        .pending()
        .iter()
        .find(|e| e.name == "CHAT_MSG_ADDON")
        .expect("CHAT_MSG_ADDON must fire");
    // Args: prefix, message, channel, target.
    assert_eq!(event.args.len(), 4);
    assert!(matches!(&event.args[0], EventArg::String(s) if s == "MYADDON"));
    assert!(matches!(&event.args[1], EventArg::String(s) if s == "hello"));
    assert!(matches!(&event.args[2], EventArg::String(s) if s == "PARTY"));
}

#[test]
fn send_addon_message_preserves_target_on_whisper_channel() {
    let env = env();
    env.exec(r#"SendAddonMessage("MYADDON", "ping", "WHISPER", "Bob")"#)
        .unwrap();
    let st = env.state().borrow();
    let entry = st.message_log.last().unwrap();
    assert_eq!(entry.target, "Bob");
}

// ── SendChatMessage ───────────────────────────────────────────────────────────

#[test]
fn send_chat_message_logs_entry_without_firing_event() {
    let env = env();
    let events_before = env.state().borrow().events.pending().len();
    env.exec(r#"SendChatMessage("hi there", "SAY", nil, "")"#)
        .unwrap();
    let st = env.state().borrow();
    assert_eq!(
        st.events.pending().len(),
        events_before,
        "SendChatMessage must not queue an inbound event"
    );
    let entry = st.message_log.last().expect("message log updated");
    assert_eq!(entry.kind, "chat");
    assert_eq!(entry.message, "hi there");
    assert_eq!(entry.channel, "SAY");
}

#[test]
fn send_chat_message_whisper_preserves_target() {
    let env = env();
    env.exec(r#"SendChatMessage("secret", "WHISPER", nil, "Bob")"#)
        .unwrap();
    let st = env.state().borrow();
    let entry = st.message_log.last().unwrap();
    assert_eq!(entry.target, "Bob");
}

// ── Sequential log ordering ───────────────────────────────────────────────────

#[test]
fn multiple_messages_append_in_order() {
    let env = env();
    env.exec(
        r#"SendChatMessage("one", "SAY")
           SendChatMessage("two", "SAY")
           SendAddonMessage("X", "three", "PARTY")"#,
    )
    .unwrap();
    let st = env.state().borrow();
    assert_eq!(st.message_log.len(), 3);
    assert_eq!(st.message_log[0].message, "one");
    assert_eq!(st.message_log[1].message, "two");
    assert_eq!(st.message_log[2].message, "three");
    assert_eq!(st.message_log[2].kind, "addon");
}
