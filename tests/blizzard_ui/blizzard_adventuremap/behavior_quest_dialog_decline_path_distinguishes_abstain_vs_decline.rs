//! Quest dialog decline distinguishes abstain from explicit decline.

use crate::common::blizzard_addon_harness::with_blizzard_addon_startup_shape;

const ROOT: &str = "Blizzard_AdventureMap";
const QUEST_ID: i64 = 40519;
const DECLINE_PROBE: &str = r#"
local questID = __questDialogDeclineQuestID
local abstain = __questDialogDeclineAbstain
local originalRefresh = AdventureMapQuestChoiceDialog.Refresh
local callbackResult = nil

AdventureMapQuestChoiceDialog.Refresh = function() end
AdventureMapQuestChoiceDialog:ShowWithQuest({}, {}, questID, function(result)
    callbackResult = result
end, 0)
local shownBeforeDecline = AdventureMapQuestChoiceDialog:IsShown()

AdventureMapQuestChoiceDialog:DeclineQuest(abstain)

AdventureMapQuestChoiceDialog.Refresh = originalRefresh

return shownBeforeDecline,
       callbackResult,
       AdventureMapQuestChoiceDialog:IsShown(),
       AdventureMapQuestChoiceDialog.result == nil,
       AdventureMapQuestChoiceDialog.questID == nil,
       AdventureMapQuestChoiceDialog.onClosedCallback == nil
"#;

#[test]
fn quest_dialog_decline_distinguishes_abstain_from_decline() {
    with_blizzard_addon_startup_shape(&[ROOT], &[], |env, _loaded| {
        let abstain_surface = decline_surface(env, true);
        assert_decline_surface(abstain_surface, 3);

        let decline_surface = decline_surface(env, false);
        assert_decline_surface(decline_surface, 2);
    });
}

type DeclineSurface = (bool, i64, bool, bool, bool, bool);

fn decline_surface(env: &wow_ui_sim::lua_api::WowLuaEnv, abstain: bool) -> DeclineSurface {
    env.exec(&format!(
        "__questDialogDeclineQuestID = {QUEST_ID}; __questDialogDeclineAbstain = {abstain}"
    ))
    .expect("AdventureMap quest-dialog decline setup must run cleanly");
    env.eval(DECLINE_PROBE)
        .expect("AdventureMap quest-dialog decline probe must run cleanly")
}

fn assert_decline_surface(surface: DeclineSurface, expected_result: i64) {
    let (
        shown_before_decline,
        callback_result,
        shown_after_decline,
        result_cleared,
        quest_id_cleared,
        callback_cleared,
    ) = surface;

    assert_decline_callback(shown_before_decline, callback_result, expected_result);
    assert_decline_finalized(
        shown_after_decline,
        result_cleared,
        quest_id_cleared,
        callback_cleared,
    );
}

fn assert_decline_callback(shown_before_decline: bool, callback_result: i64, expected_result: i64) {
    assert!(
        shown_before_decline,
        "`ShowWithQuest` must show the dialog before `DeclineQuest` runs"
    );
    assert_eq!(
        callback_result, expected_result,
        "`DeclineQuest` must finalize with the expected result"
    );
}

fn assert_decline_finalized(
    shown_after_decline: bool,
    result_cleared: bool,
    quest_id_cleared: bool,
    callback_cleared: bool,
) {
    assert!(
        !shown_after_decline,
        "`DeclineQuest` must hide the quest choice dialog"
    );
    assert!(result_cleared, "`Finalize` must clear the dialog result");
    assert!(
        quest_id_cleared,
        "`Finalize` must clear the dialog quest id"
    );
    assert!(
        callback_cleared,
        "`Finalize` must clear the dialog close callback"
    );
}
