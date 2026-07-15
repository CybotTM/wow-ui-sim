use super::load_game_ui_without_player_choice;

/// Proves the snapshot's legacy shake globals are absent while PTR publishes the
/// distinct ScriptAnimationUtil methods.
#[test]
fn helpers_are_namespaced_not_legacy_globals() {
    let env = load_game_ui_without_player_choice();

    let (legacy_shake, legacy_random, namespaced_shake, namespaced_random, safe_shake, safe_random): (
        String,
        String,
        String,
        String,
        String,
        String,
    ) = env
        .eval(
            r#"
            local region = CreateFrame("Frame")
            region.scriptedAnimatedAnchorLock = true
            return type(ShakeFrame),
                type(ShakeFrameRandom),
                type(ScriptAnimationUtil.ShakeFrame),
                type(ScriptAnimationUtil.ShakeFrameRandom),
                type(ScriptAnimationUtil.ShakeFrame(region, {}, 0, 0)),
                type(ScriptAnimationUtil.ShakeFrameRandom(region, 1, 0, 0))
            "#,
        )
        .expect("shake helper namespace probe succeeds");

    assert_eq!(legacy_shake, "nil");
    assert_eq!(legacy_random, "nil");
    assert_eq!(namespaced_shake, "function");
    assert_eq!(namespaced_random, "function");
    assert_eq!(safe_shake, "function");
    assert_eq!(safe_random, "function");
}
