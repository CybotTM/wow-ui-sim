#![cfg(feature = "gui")]

use wow_ui_sim::lua_api::WowLuaEnv;
use wow_ui_sim::render::font::WowFontSystem;

fn update_tooltip_sizes(env: &WowLuaEnv) {
    let mut font_sys = WowFontSystem::new();
    let mut state = env.state().borrow_mut();
    wow_ui_sim::iced_app::tooltip::update_tooltip_sizes(&mut state, &mut font_sys);
}

#[test]
fn test_set_owner_clears_stale_tooltip_min_width() {
    let env = WowLuaEnv::new().unwrap();

    env.exec(
        r#"
        local wideOwner = CreateFrame("Frame", "WideTooltipOwner", UIParent)
        GameTooltip:SetOwner(wideOwner, "ANCHOR_NONE")
        GameTooltip:SetMinimumWidth(1000)
        GameTooltip:AddLine("Wide")

        local itemOwner = CreateFrame("Frame", "NarrowTooltipOwner", UIParent)
        GameTooltip:SetOwner(itemOwner, "ANCHOR_NONE")
        GameTooltip:AddLine("Ring of Earthen Craftsmanship")
        GameTooltip:AddLine("Item Level 610")
        GameTooltip:AddLine("Finger")
        GameTooltip:AddLine("Vendor")
    "#,
    )
    .unwrap();

    update_tooltip_sizes(&env);

    let state = env.state().borrow();
    let gt_id = state.widgets.get_id_by_name("GameTooltip").unwrap();
    let frame = state.widgets.get(gt_id).unwrap();

    assert!(
        frame.width < 350.0,
        "SetOwner should clear stale minimum width before the next tooltip, got {}",
        frame.width
    );
}
