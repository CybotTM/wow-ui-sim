use wow_ui_sim::lua_api::WowLuaEnv;

#[test]
fn keyboard_insert_refreshes_editbox_render_text_after_clear() {
    let env = WowLuaEnv::new().expect("Failed to create Lua environment");
    env.exec(
        r#"
        local box = CreateFrame("EditBox", "KeyboardRenderTextBox", UIParent)
        box:SetText("")
        box:SetFocus()
    "#,
    )
    .expect("editbox setup should succeed");

    env.send_key_press("A", Some("a"))
        .expect("key press should update focused editbox");

    let state = env.state().borrow();
    let box_id = state
        .widgets
        .get_id_by_name("KeyboardRenderTextBox")
        .expect("editbox should be registered");
    let editbox = state.widgets.get(box_id).expect("editbox should exist");

    assert_eq!(editbox.text.as_deref(), Some("a"));
    assert_eq!(
        editbox.text_stripped.as_deref(),
        Some("a"),
        "render text cache must match inserted text"
    );
}

#[test]
fn keyboard_delete_refreshes_editbox_render_text() {
    let env = WowLuaEnv::new().expect("Failed to create Lua environment");
    env.exec(
        r#"
        local box = CreateFrame("EditBox", "KeyboardDeleteRenderTextBox", UIParent)
        box:SetText("")
        box:SetFocus()
    "#,
    )
    .expect("editbox setup should succeed");

    env.send_key_press("A", Some("a"))
        .expect("first key press should update focused editbox");
    env.send_key_press("B", Some("b"))
        .expect("second key press should update focused editbox");
    env.send_key_press("BACKSPACE", None)
        .expect("backspace should update focused editbox");

    let state = env.state().borrow();
    let box_id = state
        .widgets
        .get_id_by_name("KeyboardDeleteRenderTextBox")
        .expect("editbox should be registered");
    let editbox = state.widgets.get(box_id).expect("editbox should exist");

    assert_eq!(editbox.text.as_deref(), Some("a"));
    assert_eq!(
        editbox.text_stripped.as_deref(),
        Some("a"),
        "render text cache must match text after deletion"
    );
}
