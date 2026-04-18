//! Smoke tests for the rilua-ported MessageFrame method family.
//!
//! Exercises: AddMessage / GetNumMessages, BackFillMessage.

use wow_ui_sim::lua_api::WowLuaEnv;

#[test]
fn test_add_message_returns_num_messages_one() {
    let env = WowLuaEnv::new().unwrap();

    let count: i32 = env
        .eval(
            r#"
        local f = CreateFrame("MessageFrame")
        f:AddMessage("hello")
        return f:GetNumMessages()
    "#,
        )
        .unwrap();

    assert_eq!(count, 1);
}

#[test]
fn test_backfill_message_prepends_and_count_is_correct() {
    let env = WowLuaEnv::new().unwrap();

    let (count, first, second): (i32, String, String) = env
        .eval(
            r#"
        local f = CreateFrame("MessageFrame")
        f:AddMessage("second")
        f:BackFillMessage("first")
        local t1 = f:GetMessageInfo(1)
        local t2 = f:GetMessageInfo(2)
        return f:GetNumMessages(), t1, t2
    "#,
        )
        .unwrap();

    assert_eq!(count, 2);
    assert_eq!(first, "first", "BackFillMessage should prepend");
    assert_eq!(second, "second", "AddMessage should follow BackFill");
}
