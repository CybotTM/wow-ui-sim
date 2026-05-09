#![cfg(feature = "gui")]

use wow_ui_sim::lua_api::WowLuaEnv;

#[test]
fn test_custom_word_wrap_min_width_expands_wrapped_only_tooltip_width() {
    let env = WowLuaEnv::new().unwrap();

    env.exec(
        r#"
        local owner = CreateFrame("Frame", "WrapMinWidthOwner", UIParent)
        GameTooltip:SetOwner(owner, "ANCHOR_NONE")
        GameTooltip:SetCustomWordWrapMinWidth(350)
        GameTooltip:AddLine("This is a wrapped-only tooltip body that should honor the custom minimum width instead of collapsing down to padding width.", 1, 1, 1, true)
        GameTooltip:Show()
    "#,
    )
    .unwrap();

    update_tooltip_sizes(&env);

    let state = env.state().borrow();
    let gt_id = state.widgets.get_id_by_name("GameTooltip").unwrap();
    let frame = state.widgets.get(gt_id).unwrap();

    assert!(
        frame.width >= 374.0,
        "SetCustomWordWrapMinWidth(350) should expand wrapped tooltip width, got {}",
        frame.width
    );
}

fn update_tooltip_sizes(env: &WowLuaEnv) {
    use wow_ui_sim::render::font::WowFontSystem;

    let mut font_sys = WowFontSystem::new();
    let mut state = env.state().borrow_mut();
    wow_ui_sim::iced_app::tooltip::update_tooltip_sizes(&mut state, &mut font_sys);
}
