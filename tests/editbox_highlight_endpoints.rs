//! Clean-room selection policy from public text-editing behavior, not native parity.
use wow_ui_sim::lua_api::WowLuaEnv;

fn assert_inserted_text(text: &str, endpoints: &str, expected: (&str, i32, i32)) {
    let env = WowLuaEnv::new().expect("isolated EditBox environment");
    let actual: (String, i32, i32) = env
        .eval(&format!(
            r#"
            local box = CreateFrame('EditBox')
            box:SetText({text:?})
            box:HighlightText({endpoints})
            box:Insert('X')
            return box:GetText(), box:GetCursorPosition(), box:GetUTF8CursorPosition()
            "#,
        ))
        .expect("public HighlightText followed by Insert and cursor queries");
    assert_eq!(actual.0, expected.0, "{text:?}, endpoints {endpoints}");
    assert_eq!(actual.1, expected.1, "public UTF-8 byte cursor");
    assert_eq!(
        actual.2, expected.2,
        "internal Unicode scalar cursor projection"
    );
}

#[test]
fn highlight_reversed_end_selects_from_start_to_text_end_without_swapping() {
    assert_inserted_text("abcdef", "4, 2", ("abcdX", 5, 5));
}

#[test]
fn highlight_negative_end_is_end_of_text_sentinel() {
    assert_inserted_text("abcdef", "2, -1", ("abX", 3, 3));
}

#[test]
fn highlight_omitted_ordered_and_empty_endpoints_preserve_existing_editing() {
    assert_inserted_text("abcdef", "2", ("abX", 3, 3));
    assert_inserted_text("abcdef", "1, 3", ("aXdef", 2, 2));
    assert_inserted_text("abcdef", "2, 2", ("abXcdef", 3, 3));
}

#[test]
fn highlight_endpoint_policy_preserves_multibyte_byte_offsets_and_scalar_cursor() {
    assert_inserted_text("aé🙂z", "3, 1", ("aéX", 4, 3));
    assert_inserted_text("aé🙂z", "1, -1", ("aX", 2, 2));
    assert_inserted_text("aé🙂z", "3", ("aéX", 4, 3));
    assert_inserted_text("aé🙂z", "1, 3", ("aX🙂z", 2, 2));
    assert_inserted_text("aé🙂z", "3, 3", ("aéX🙂z", 4, 3));
}
