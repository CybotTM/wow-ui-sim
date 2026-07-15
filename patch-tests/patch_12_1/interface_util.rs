use super::load_game_ui_without_player_choice;

/// Proves the screen-scale helpers remain globals rather than InterfaceUtil methods.
#[test]
fn screen_scale_helpers_remain_global() {
    let env = load_game_ui_without_player_choice();

    let (height_type, width_type, namespaced_height, namespaced_width, height, width): (
        String,
        String,
        String,
        String,
        f64,
        f64,
    ) = env
        .eval(
            r#"
            return type(GetScreenHeightScale),
                type(GetScreenWidthScale),
                InterfaceUtil and type(InterfaceUtil.GetScreenHeightScale) or "nil",
                InterfaceUtil and type(InterfaceUtil.GetScreenWidthScale) or "nil",
                GetScreenHeightScale(),
                GetScreenWidthScale()
            "#,
        )
        .expect("screen scale snapshot probe succeeds");

    assert_eq!(height_type, "function");
    assert_eq!(width_type, "function");
    assert_eq!(namespaced_height, "nil");
    assert_eq!(namespaced_width, "nil");
    assert!((height - 1.0).abs() < f64::EPSILON);
    assert!((width - 1.0).abs() < f64::EPSILON);
}
