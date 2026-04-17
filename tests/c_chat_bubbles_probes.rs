//! Tests for `C_ChatBubbles` probes backed by `SimState.chat_bubbles`.

use wow_ui_sim::lua_api::WowLuaEnv;
use wow_ui_sim::lua_api::state::ChatBubble;

fn env() -> WowLuaEnv {
    WowLuaEnv::new().expect("Failed to create Lua environment")
}

#[test]
fn get_all_chat_bubbles_returns_empty_array_by_default() {
    let env = env();
    let count: i32 = env
        .eval("return #C_ChatBubbles.GetAllChatBubbles()")
        .unwrap();
    assert_eq!(count, 0, "default SimState has no chat bubbles");
}

#[test]
fn get_all_chat_bubbles_reflects_seeded_bubbles() {
    let env = env();
    env.state().borrow_mut().chat_bubbles.push(ChatBubble {
        message: "For the Horde!".to_string(),
        sender: "Thrall".to_string(),
        chat_type: "YELL".to_string(),
        frame_id: None,
    });
    env.state().borrow_mut().chat_bubbles.push(ChatBubble {
        message: "Greetings, traveller.".to_string(),
        sender: "Innkeeper".to_string(),
        chat_type: "SAY".to_string(),
        frame_id: None,
    });
    let (count, first_msg, second_sender): (i32, String, String) = env
        .eval(
            r#"
            local bubbles = C_ChatBubbles.GetAllChatBubbles()
            return #bubbles, bubbles[1].message, bubbles[2].sender
            "#,
        )
        .unwrap();
    assert_eq!(count, 2);
    assert_eq!(first_msg, "For the Horde!");
    assert_eq!(second_sender, "Innkeeper");
}

#[test]
fn get_all_chat_bubbles_returns_chat_type_field() {
    let env = env();
    env.state().borrow_mut().chat_bubbles.push(ChatBubble {
        message: "Hello".to_string(),
        sender: "Player".to_string(),
        chat_type: "SAY".to_string(),
        frame_id: None,
    });
    let chat_type: String = env
        .eval("return C_ChatBubbles.GetAllChatBubbles()[1].chatType")
        .unwrap();
    assert_eq!(chat_type, "SAY");
}

#[test]
fn get_all_chat_bubbles_includes_frame_id_when_set() {
    let env = env();
    env.state().borrow_mut().chat_bubbles.push(ChatBubble {
        message: "Bubble with frame".to_string(),
        sender: "NPC".to_string(),
        chat_type: "SAY".to_string(),
        frame_id: Some(42),
    });
    let frame_id: f64 = env
        .eval("return C_ChatBubbles.GetAllChatBubbles()[1].frameID")
        .unwrap();
    assert_eq!(frame_id as u64, 42);
}

#[test]
fn get_all_chat_bubbles_mutation_reflects_in_subsequent_calls() {
    let env = env();
    // Start empty
    let initial: i32 = env
        .eval("return #C_ChatBubbles.GetAllChatBubbles()")
        .unwrap();
    assert_eq!(initial, 0);
    // Seed one bubble
    env.state().borrow_mut().chat_bubbles.push(ChatBubble {
        message: "Added".to_string(),
        sender: "Someone".to_string(),
        chat_type: "EMOTE".to_string(),
        frame_id: None,
    });
    let after: i32 = env
        .eval("return #C_ChatBubbles.GetAllChatBubbles()")
        .unwrap();
    assert_eq!(after, 1);
}
