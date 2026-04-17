use wow_ui_sim::lua_api::WowLuaEnv;

#[test]
fn c_tooltip_info_trade_skill_item_routes_recipe_outputs_and_reagents() {
    let env = WowLuaEnv::new().expect("should create WowLuaEnv");
    let (
        result_name,
        result_level,
        reagent_name,
        reagent_level,
        recipe_reagent_name,
        recipe_reagent_level,
    ): (String, String, String, String, String, String) = env
        .eval(
            r#"
            local result = C_TooltipInfo.GetTradeSkillItem(100005)
            local reagent = C_TooltipInfo.GetTradeSkillItem(100005, 1)
            local recipeReagent = C_TooltipInfo.GetRecipeReagentItem(100005, 1)
            return result.lines[1].leftText,
                   result.lines[2].leftText,
                   reagent.lines[1].leftText,
                   reagent.lines[2].leftText,
                   recipeReagent.lines[1].leftText,
                   recipeReagent.lines[2].leftText
            "#,
        )
        .unwrap();

    let (
        expected_result_name,
        expected_result_level,
        expected_reagent_name,
        expected_reagent_level,
    ): (String, String, String, String) = env
        .eval(
            r#"
            local baselineResult = C_TooltipInfo.GetItemByID(229181)
            local baselineReagent = C_TooltipInfo.GetItemByID(210934)
            return baselineResult.lines[1].leftText,
                   baselineResult.lines[2].leftText,
                   baselineReagent.lines[1].leftText,
                   baselineReagent.lines[2].leftText
            "#,
        )
        .unwrap();

    assert_eq!(result_name, expected_result_name);
    assert_eq!(result_level, expected_result_level);
    assert_eq!(reagent_name, expected_reagent_name);
    assert_eq!(reagent_level, expected_reagent_level);
    assert_eq!(recipe_reagent_name, expected_reagent_name);
    assert_eq!(recipe_reagent_level, expected_reagent_level);
}

#[test]
fn game_tooltip_set_trade_skill_item_populates_item_lines() {
    let env = WowLuaEnv::new().expect("should create WowLuaEnv");
    env.exec("GameTooltip:SetTradeSkillItem(100005)").unwrap();

    let state = env.state().borrow();
    let gt_id = state.widgets.get_id_by_name("GameTooltip").unwrap();
    let tooltip = state
        .tooltips
        .get(&gt_id)
        .expect("tooltip data should exist");
    assert_eq!(tooltip.lines[0].left_text, "Ordained Forge Maul");
    assert_eq!(tooltip.lines[1].left_text, "Item Level 610");
}
