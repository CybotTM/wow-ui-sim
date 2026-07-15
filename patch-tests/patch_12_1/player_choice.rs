use wow_ui_sim::loader::load_addon;

use super::{load_game_ui_without_player_choice, player_choice_toc};

/// Proves both the LoD publication phase and the toggle helper's visible-button contract.
#[test]
fn toggle_is_lod_and_updates_visible_buttons() {
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
