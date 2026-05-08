use super::*;

#[test]
fn chat_frame_layout_stays_locked() {
    test_timeout! {
        let env = setup_env();
        let result: String = env
            .eval(include_str!("layout_lock.lua"))
            .expect("chat frame lock eval failed");

        assert_eq!(
            result, "ok",
            "ChatFrame1 layout should remain fully locked: {result}"
        );
    }
}
