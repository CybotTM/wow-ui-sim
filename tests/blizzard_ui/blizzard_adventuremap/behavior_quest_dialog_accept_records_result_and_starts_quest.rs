//! Quest dialog accept starts the quest and finalizes with an accepted result.

use crate::common::blizzard_addon_harness::with_blizzard_addon_startup_shape;

const ROOT: &str = "Blizzard_AdventureMap";
const QUEST_ID: i64 = 40519;
const ACCEPT_PROBE: &str = r#"
local questID = __questDialogAcceptQuestID
local originalRefresh = AdventureMapQuestChoiceDialog.Refresh
local originalStartQuest = C_AdventureMap.StartQuest
local startedQuestID = nil
local callbackResult = nil

AdventureMapQuestChoiceDialog.Refresh = function() end
C_AdventureMap.StartQuest = function(startQuestID)
    startedQuestID = startQuestID
    return originalStartQuest(startQuestID)
end

AdventureMapQuestChoiceDialog:ShowWithQuest({}, {}, questID, function(result)
    callbackResult = result
end, 0)
local shownBeforeAccept = AdventureMapQuestChoiceDialog:IsShown()

AdventureMapQuestChoiceDialog:AcceptQuest()

C_AdventureMap.StartQuest = originalStartQuest
AdventureMapQuestChoiceDialog.Refresh = originalRefresh

return shownBeforeAccept,
       startedQuestID,
       callbackResult,
       AdventureMapQuestChoiceDialog:IsShown(),
       AdventureMapQuestChoiceDialog.result == nil,
       AdventureMapQuestChoiceDialog.questID == nil,
       AdventureMapQuestChoiceDialog.onClosedCallback == nil
"#;

#[test]
fn quest_dialog_accept_records_result_and_starts_quest() {
    with_blizzard_addon_startup_shape(&[ROOT], &[], |env, _loaded| {
        let surface = accept_surface(env);
        assert_accept_surface(surface);
        assert!(
            env.state().borrow().quest_log.contains(&(QUEST_ID as u32)),
            "`AcceptQuest` must start the selected adventure-map quest"
        );
    });
}

type AcceptSurface = (bool, i64, i64, bool, bool, bool, bool);

fn accept_surface(env: &wow_ui_sim::lua_api::WowLuaEnv) -> AcceptSurface {
    env.exec(&format!("__questDialogAcceptQuestID = {QUEST_ID}"))
        .expect("AdventureMap quest-dialog accept setup must run cleanly");
    env.eval(ACCEPT_PROBE)
        .expect("AdventureMap quest-dialog accept probe must run cleanly")
}

fn assert_accept_surface(surface: AcceptSurface) {
    let (
        shown_before_accept,
        started_quest_id,
        callback_result,
        shown_after_accept,
        result_cleared,
        quest_id_cleared,
        callback_cleared,
    ) = surface;

    assert_accept_started_quest(shown_before_accept, started_quest_id);
    assert_accept_finalized(
        callback_result,
        shown_after_accept,
        result_cleared,
        quest_id_cleared,
        callback_cleared,
    );
}

fn assert_accept_started_quest(shown_before_accept: bool, started_quest_id: i64) {
    assert!(
        shown_before_accept,
        "`ShowWithQuest` must show the dialog before `AcceptQuest` runs"
    );
    assert_eq!(
        started_quest_id, QUEST_ID,
        "`AcceptQuest` must call `C_AdventureMap.StartQuest` with `self.questID`"
    );
}

fn assert_accept_finalized(
    callback_result: i64,
    shown_after_accept: bool,
    result_cleared: bool,
    quest_id_cleared: bool,
    callback_cleared: bool,
) {
    assert_eq!(
        callback_result, 1,
        "`AcceptQuest` must finalize with `QUEST_CHOICE_DIALOG_RESULT_ACCEPTED`"
    );
    assert!(
        !shown_after_accept,
        "`AcceptQuest` must hide the quest choice dialog"
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
