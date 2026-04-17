//! Integration tests for `src/lua_api/globals/mail_verbs.rs`.

use wow_ui_sim::lua_api::WowLuaEnv;

fn env() -> WowLuaEnv {
    WowLuaEnv::new().expect("WowLuaEnv init")
}

fn fired_events(env: &WowLuaEnv, filter: &str) -> Vec<String> {
    env.state()
        .borrow()
        .events
        .pending()
        .iter()
        .map(|ev| ev.name.clone())
        .filter(|name| name.starts_with(filter))
        .collect()
}

// ── SendMail ──────────────────────────────────────────────────────────────────

#[test]
fn send_mail_appends_to_inbox_and_fires_send_success() {
    let env = env();
    env.exec(
        r#"A_Admin.ClearInbox()
           SendMail("Recipient", "Hello", "Body text")"#,
    )
    .unwrap();
    let st = env.state().borrow();
    let inbox = &st.player.inbox;
    assert_eq!(inbox.len(), 1);
    assert_eq!(inbox[0].sender, "Recipient");
    assert_eq!(inbox[0].subject, "Hello");
    assert_eq!(inbox[0].body, "Body text");
    drop(st);

    let events = fired_events(&env, "MAIL_");
    assert!(
        events.iter().any(|e| e == "MAIL_SEND_SUCCESS"),
        "MAIL_SEND_SUCCESS must fire, got {events:?}"
    );
    assert!(events.iter().any(|e| e == "MAIL_INBOX_UPDATE"));
}

#[test]
fn send_mail_with_empty_recipient_is_noop() {
    let env = env();
    let before = env.state().borrow().player.inbox.len();
    env.exec(r#"SendMail("", "Hello", "Body")"#).unwrap();
    assert_eq!(env.state().borrow().player.inbox.len(), before);
}

#[test]
fn send_mail_clears_pending_attachments_and_money() {
    let env = env();
    {
        let mut st = env.state().borrow_mut();
        st.player.send_mail_money = 5000;
        st.player.send_mail_cod = 100;
    }
    env.exec(r#"SendMail("Recipient", "Subj", "")"#).unwrap();
    let st = env.state().borrow();
    assert_eq!(st.player.send_mail_money, 0);
    assert_eq!(st.player.send_mail_cod, 0);
    assert_eq!(st.player.inbox.last().map(|m| m.money), Some(5000));
}

// ── DeleteMail ────────────────────────────────────────────────────────────────

#[test]
fn delete_mail_removes_indexed_entry_and_fires_inbox_update() {
    let env = env();
    env.exec(
        r#"A_Admin.ClearInbox()
               A_Admin.SetInboxCount(3)"#,
    )
    .unwrap();
    let before = env.state().borrow().player.inbox.len();
    assert_eq!(before, 3);
    env.exec("DeleteMail(2)").unwrap();
    let st = env.state().borrow();
    assert_eq!(st.player.inbox.len(), 2);
    assert!(
        st.events
            .pending()
            .iter()
            .any(|e| e.name == "MAIL_INBOX_UPDATE"),
        "MAIL_INBOX_UPDATE must fire"
    );
}

#[test]
fn delete_mail_out_of_range_is_noop() {
    let env = env();
    env.exec(
        r#"A_Admin.ClearInbox()
               A_Admin.SetInboxCount(1)"#,
    )
    .unwrap();
    env.exec("DeleteMail(99)").unwrap();
    assert_eq!(env.state().borrow().player.inbox.len(), 1);
}

// ── ForwardMail ───────────────────────────────────────────────────────────────

#[test]
fn forward_mail_replaces_sender_and_keeps_body() {
    let env = env();
    env.exec(
        r#"A_Admin.ClearInbox()
           A_Admin.AddMail("Thrall", "Warning", "Lok-tar!")"#,
    )
    .unwrap();
    env.exec(r#"ForwardMail(1, "Jaina")"#).unwrap();
    let st = env.state().borrow();
    let inbox = &st.player.inbox;
    assert_eq!(inbox.len(), 1, "forward keeps exactly one entry");
    assert_eq!(inbox[0].sender, "Jaina");
    assert!(
        inbox[0].subject.starts_with("Fwd: "),
        "default subject must prepend Fwd:, got {:?}",
        inbox[0].subject
    );
    assert_eq!(inbox[0].body, "Lok-tar!");
}

#[test]
fn forward_mail_custom_subject_wins() {
    let env = env();
    env.exec(
        r#"A_Admin.ClearInbox()
           A_Admin.AddMail("Thrall", "Warning", "Lok-tar!")
           ForwardMail(1, "Jaina", "Custom Subject")"#,
    )
    .unwrap();
    let st = env.state().borrow();
    assert_eq!(st.player.inbox[0].subject, "Custom Subject");
}

#[test]
fn forward_mail_missing_recipient_is_noop() {
    let env = env();
    env.exec(
        r#"A_Admin.ClearInbox()
           A_Admin.AddMail("Thrall", "S", "B")"#,
    )
    .unwrap();
    let before_sender = env.state().borrow().player.inbox[0].sender.clone();
    env.exec(r#"ForwardMail(1, "")"#).unwrap();
    assert_eq!(env.state().borrow().player.inbox[0].sender, before_sender);
}

// ── CloseInbox ────────────────────────────────────────────────────────────────

#[test]
fn close_inbox_fires_mail_closed_event_without_clearing_inbox() {
    let env = env();
    env.exec(
        r#"A_Admin.ClearInbox()
               A_Admin.SetInboxCount(2)"#,
    )
    .unwrap();
    env.exec("CloseInbox()").unwrap();
    let st = env.state().borrow();
    assert_eq!(st.player.inbox.len(), 2, "CloseInbox must not clear inbox");
    assert!(
        st.events.pending().iter().any(|e| e.name == "MAIL_CLOSED"),
        "MAIL_CLOSED must fire"
    );
}
