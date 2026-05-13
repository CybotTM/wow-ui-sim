//! Behavior pin: ActionBarController_UpdateAllSpellHighlights fans out to buttons.

use crate::common;
use crate::common::blizzard_addon_harness::with_blizzard_addon_startup_shape;

const ROOT: &str = "Blizzard_ActionBarController";

#[test]
fn update_all_spell_highlights_updates_each_action_bar_button_event_frame() {
    test_timeout! {
    with_blizzard_addon_startup_shape(&[ROOT], &[], |env, _loaded| {
        env.exec(
            r#"
            _G.spellHighlightUpdateCalls = 0

            local firstFrame = {}
            function firstFrame:UpdateSpellHighlightMark()
                _G.spellHighlightUpdateCalls = _G.spellHighlightUpdateCalls + 1
            end

            local secondFrame = {}
            function secondFrame:UpdateSpellHighlightMark()
                _G.spellHighlightUpdateCalls = _G.spellHighlightUpdateCalls + 1
            end

            ActionBarButtonEventsFrame.frames = {
                First = firstFrame,
                Second = secondFrame,
            }

            ActionBarController_UpdateAllSpellHighlights()
            "#,
        )
        .expect("ActionBarController_UpdateAllSpellHighlights must run cleanly");

        let calls: i32 = env
            .eval("return _G.spellHighlightUpdateCalls")
            .expect("post spell-highlight update probe must run cleanly");

        assert_eq!(
            calls, 2,
            "ActionBarController_UpdateAllSpellHighlights must update every button event frame"
        );
    });
    }
}
