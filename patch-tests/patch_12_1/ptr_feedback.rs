use super::load_game_ui_without_player_choice;

/// Pins the current PTRFeedback publication and its upstream undefined-state error.
#[test]
fn quest_progress_time_is_published_but_errors() {
    let env = load_game_ui_without_player_choice();

    let (function_type, succeeded, error): (String, bool, String) = env
        .eval(
            r#"
            local succeeded, result = pcall(GetTimeSinceLastQuestProgress)
            return type(GetTimeSinceLastQuestProgress), succeeded, tostring(result)
            "#,
        )
        .expect("PTRFeedback quest progress helper probe succeeds");

    assert_eq!(function_type, "function");
    assert!(!succeeded);
    assert!(error.contains("arithmetic"));
    assert!(error.contains("nil"));
}
