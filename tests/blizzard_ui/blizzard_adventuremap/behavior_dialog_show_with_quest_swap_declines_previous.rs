//! Quest dialog swap declines the currently shown quest before re-anchoring.

use crate::common::blizzard_addon_harness::with_blizzard_addon_startup_shape;

const ROOT: &str = "Blizzard_AdventureMap";
const FIRST_QUEST_ID: i64 = 40519;
const SECOND_QUEST_ID: i64 = 40520;
const QUEST_CHOICE_DIALOG_RESULT_ABSTAIN: i64 = 3;
const SWAP_PROBE: &str = r#"
local firstQuestID = __questDialogSwapFirstQuestID
local secondQuestID = __questDialogSwapSecondQuestID
local dialog = AdventureMapQuestChoiceDialog
local parent = CreateFrame("Frame")
local firstPin = CreateFrame("Frame", nil, parent)
local secondPin = CreateFrame("Frame", nil, parent)
local originalRefresh = dialog.Refresh
local originalSetPoint = dialog.SetPoint
local sequence = {}
local firstResult = nil
local secondResult = nil
local secondAnchorAppliedAfterAbstain = false

dialog.Refresh = function() end
dialog.SetPoint = function(self, point, relativeTo, ...)
    if relativeTo == secondPin then
        secondAnchorAppliedAfterAbstain = firstResult == QUEST_CHOICE_DIALOG_RESULT_ABSTAIN
        table.insert(sequence, "anchor-second")
    end
    return originalSetPoint(self, point, relativeTo, ...)
end

dialog:ShowWithQuest(parent, firstPin, firstQuestID, function(result)
    firstResult = result
    table.insert(sequence, "first-callback")
end, 0)
local shownBeforeSwap = dialog:IsShown()

dialog:ShowWithQuest(parent, secondPin, secondQuestID, function(result)
    secondResult = result
end, 0)

local point, relativeTo = dialog:GetPoint(1)
local shownAfterSwap = dialog:IsShown()
local secondQuestActive = dialog.questID == secondQuestID
local callbackSwapped = dialog.onClosedCallback ~= nil
local secondAnchorRegion = dialog.anchorRegion == secondPin
local secondPointAnchored = point == "CENTER" and relativeTo == secondPin
local callbackBeforeAnchor = sequence[1] == "first-callback" and sequence[2] == "anchor-second"

dialog.SetPoint = originalSetPoint
dialog.Refresh = originalRefresh

return shownBeforeSwap,
       firstResult,
       secondResult == nil,
       shownAfterSwap,
       secondQuestActive,
       callbackSwapped,
       secondAnchorRegion,
       secondPointAnchored,
       secondAnchorAppliedAfterAbstain,
       callbackBeforeAnchor
"#;

#[test]
fn dialog_show_with_quest_swap_declines_previous_before_reanchoring() {
    with_blizzard_addon_startup_shape(&[ROOT], &[], |env, _loaded| {
        let surface = swap_surface(env);
        assert_swap_surface(surface);
    });
}

type SwapSurface = (bool, i64, bool, bool, bool, bool, bool, bool, bool, bool);

fn swap_surface(env: &wow_ui_sim::lua_api::WowLuaEnv) -> SwapSurface {
    env.exec(&format!(
        "__questDialogSwapFirstQuestID = {FIRST_QUEST_ID}; \
         __questDialogSwapSecondQuestID = {SECOND_QUEST_ID}"
    ))
    .expect("AdventureMap quest-dialog swap setup must run cleanly");
    env.eval(SWAP_PROBE)
        .expect("AdventureMap quest-dialog swap probe must run cleanly")
}

fn assert_swap_surface(surface: SwapSurface) {
    let (
        shown_before_swap,
        first_result,
        second_result_pending,
        shown_after_swap,
        second_quest_active,
        callback_swapped,
        second_anchor_region,
        second_point_anchored,
        second_anchor_after_abstain,
        callback_before_anchor,
    ) = surface;

    assert_previous_quest_abstained(shown_before_swap, first_result);
    assert_new_quest_remains_active(
        second_result_pending,
        shown_after_swap,
        second_quest_active,
        callback_swapped,
    );
    assert_new_anchor_applied(
        second_anchor_region,
        second_point_anchored,
        second_anchor_after_abstain,
        callback_before_anchor,
    );
}

fn assert_previous_quest_abstained(shown_before_swap: bool, first_result: i64) {
    assert!(
        shown_before_swap,
        "first `ShowWithQuest` must show the dialog before the swap"
    );
    assert_eq!(
        first_result, QUEST_CHOICE_DIALOG_RESULT_ABSTAIN,
        "swapping quests must decline the previous quest with abstain"
    );
}

fn assert_new_quest_remains_active(
    second_result_pending: bool,
    shown_after_swap: bool,
    second_quest_active: bool,
    callback_swapped: bool,
) {
    assert!(
        second_result_pending,
        "the new quest callback must remain pending after the swap"
    );
    assert!(
        shown_after_swap,
        "the dialog must stay shown for the new quest"
    );
    assert!(
        second_quest_active,
        "`ShowWithQuest` must store the replacement quest id"
    );
    assert!(
        callback_swapped,
        "`ShowWithQuest` must install the replacement close callback"
    );
}

fn assert_new_anchor_applied(
    second_anchor_region: bool,
    second_point_anchored: bool,
    second_anchor_after_abstain: bool,
    callback_before_anchor: bool,
) {
    assert!(
        second_anchor_region,
        "`ShowWithQuest` must store the replacement anchor region"
    );
    assert!(
        second_point_anchored,
        "`ShowWithQuest` must re-anchor to the replacement pin"
    );
    assert!(
        second_anchor_after_abstain,
        "`ShowWithQuest` must re-anchor after the previous abstain callback"
    );
    assert!(
        callback_before_anchor,
        "previous callback must run before the replacement anchor is applied"
    );
}
