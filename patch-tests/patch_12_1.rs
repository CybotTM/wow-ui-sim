use std::path::PathBuf;

use wow_ui_sim::loader::{discover_blizzard_addons_for_screen, load_addon};
use wow_ui_sim::lua_api::WowLuaEnv;
use wow_ui_sim::screen::ScreenKind;

fn blizzard_ui_dir() -> PathBuf {
    wow_ui_sim::paths::default_blizzard_ui_addons_path()
        .expect("Blizzard UI cache should be available")
}

fn player_choice_toc() -> PathBuf {
    blizzard_ui_dir()
        .join("Blizzard_PlayerChoice")
        .join("Blizzard_PlayerChoice.toc")
}

fn load_game_ui_without_player_choice() -> WowLuaEnv {
    let env = WowLuaEnv::new().expect("Failed to create Lua environment");
    env.set_screen_size(1024.0, 768.0);
    env.set_screen_mode(ScreenKind::Game);

    env.state().borrow_mut().addon_base_paths = vec![blizzard_ui_dir()];
    wow_ui_sim::xml::register_intrinsic_templates();

    for (name, toc_path) in
        discover_blizzard_addons_for_screen(&blizzard_ui_dir(), ScreenKind::Game)
    {
        load_addon(&env.loader_env(), &toc_path)
            .unwrap_or_else(|error| panic!("[load {name}] FAILED: {error}"));
    }

    env
}

/// Proves both the LoD publication phase and the toggle helper's visible-button contract.
#[test]
fn player_choice_toggle_is_lod_and_updates_visible_buttons() {
    let env = load_game_ui_without_player_choice();

    let before_load: String = env
        .eval("return type(PlayerChoiceToggle_TryShow)")
        .expect("pre-load PlayerChoiceToggle_TryShow probe succeeds");
    assert_eq!(before_load, "nil");

    load_addon(&env.loader_env(), &player_choice_toc())
        .expect("explicit load_addon for Blizzard_PlayerChoice succeeds");

    let (
        after_load,
        torghast_shown,
        cypher_shown,
        generic_shown,
        torghast_updates,
        cypher_updates,
        generic_updates,
        result,
    ): (String, bool, bool, bool, i64, i64, i64, String) = env
        .eval(
            r#"
            local torghastUpdates, cypherUpdates, genericUpdates = 0, 0, 0
            TorghastPlayerChoiceToggleButton.ShouldShow = function() return true end
            TorghastPlayerChoiceToggleButton.UpdateButtonState = function()
                torghastUpdates = torghastUpdates + 1
            end
            CypherPlayerChoiceToggleButton.ShouldShow = function() return false end
            CypherPlayerChoiceToggleButton.UpdateButtonState = function()
                cypherUpdates = cypherUpdates + 1
            end
            GenericPlayerChoiceToggleButton.ShouldShow = function() return true end
            GenericPlayerChoiceToggleButton.UpdateButtonState = function()
                genericUpdates = genericUpdates + 1
            end

            local result = PlayerChoiceToggle_TryShow()
            return type(PlayerChoiceToggle_TryShow),
                TorghastPlayerChoiceToggleButton:IsShown(),
                CypherPlayerChoiceToggleButton:IsShown(),
                GenericPlayerChoiceToggleButton:IsShown(),
                torghastUpdates,
                cypherUpdates,
                genericUpdates,
                type(result)
            "#,
        )
        .expect("PlayerChoiceToggle_TryShow behavior probe succeeds");

    assert_eq!(after_load, "function");
    assert!(torghast_shown);
    assert!(!cypher_shown);
    assert!(generic_shown);
    assert_eq!(torghast_updates, 2);
    assert_eq!(cypher_updates, 0);
    assert_eq!(generic_updates, 2);
    assert_eq!(result, "nil");
}

/// Proves the snapshot's legacy shake globals are absent while PTR publishes the
/// distinct ScriptAnimationUtil methods.
#[test]
fn shake_helpers_are_namespaced_not_legacy_globals() {
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

/// Pins the current PTRFeedback publication and its upstream undefined-state error.
#[test]
fn ptr_feedback_quest_progress_time_is_published_but_errors() {
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
