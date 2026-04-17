use wow_ui_sim::lua_api::WowLuaEnv;

#[test]
fn c_tooltip_info_achievement_by_id_exposes_name_and_description() {
    let env = WowLuaEnv::new().expect("should create WowLuaEnv");
    let (tooltip_type, name, description): (i32, String, String) = env
        .eval(
            r#"
            local info = C_TooltipInfo.GetAchievementByID(776)
            return info.type, info.lines[1].leftText, info.lines[2].leftText
            "#,
        )
        .unwrap();

    assert_eq!(tooltip_type, 12);
    assert_eq!(name, "Explore Elwynn Forest");
    assert_eq!(
        description,
        "Explore Elwynn Forest, revealing the covered areas of the world map."
    );
}

#[test]
fn game_tooltip_set_achievement_by_id_populates_lines() {
    let env = WowLuaEnv::new().expect("should create WowLuaEnv");
    env.exec("GameTooltip:SetAchievementByID(776)").unwrap();

    let state = env.state().borrow();
    let gt_id = state.widgets.get_id_by_name("GameTooltip").unwrap();
    let tooltip = state
        .tooltips
        .get(&gt_id)
        .expect("tooltip data should exist");
    assert_eq!(tooltip.lines[0].left_text, "Explore Elwynn Forest");
    assert_eq!(
        tooltip.lines[1].left_text,
        "Explore Elwynn Forest, revealing the covered areas of the world map."
    );
}
