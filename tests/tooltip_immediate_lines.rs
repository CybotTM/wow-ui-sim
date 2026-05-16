#![cfg(feature = "gui")]

use wow_ui_sim::lua_api::WowLuaEnv;

#[test]
fn test_add_double_line_immediately_creates_named_fontstrings() {
    let env = WowLuaEnv::new().unwrap();

    env.exec(
        r#"
        local tooltip = CreateFrame("GameTooltip", "ImmediateLineTooltip", UIParent)
        tooltip:AddDoubleLine("Left", "Right")
        "#,
    )
    .unwrap();

    let (left_exists, right_exists): (bool, bool) = env
        .eval(
            r#"
            return ImmediateLineTooltipTextLeft1 ~= nil,
                ImmediateLineTooltipTextRight1 ~= nil
            "#,
        )
        .unwrap();
    assert!(left_exists, "left line fontstring should be globally named");
    assert!(right_exists, "right line fontstring should be globally named");
}
