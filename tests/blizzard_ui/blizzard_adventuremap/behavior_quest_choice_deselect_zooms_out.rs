//! Quest-choice deselection abstains the dialog and zooms the map out.

use crate::common::blizzard_addon_harness::with_blizzard_addon_startup_shape;

const ROOT: &str = "Blizzard_AdventureMap";
const QUEST_ID: i64 = 40519;
const DESELECT_PROBE: &str = r#"
local mapCanvas = {
    zoomOutCount = 0,
    ZoomOut = function(self)
        self.zoomOutCount = self.zoomOutCount + 1
    end,
}
local pin = {
    panCount = 0,
    selectedValues = {},
    PanAndZoomTo = function(self)
        self.panCount = self.panCount + 1
    end,
    SetSelected = function(self, selected)
        table.insert(self.selectedValues, selected)
    end,
}
local provider = CreateFromMixins(AdventureMap_QuestChoiceDataProviderMixin)
provider.owningMap = mapCanvas
provider.pinsByQuestID = { [__questChoiceDeselectQuestID] = pin }

local originalRefresh = AdventureMapQuestChoiceDialog.Refresh
local originalSetPortraitAtlas = AdventureMapQuestChoiceDialog.SetPortraitAtlas
local originalShowWithQuest = AdventureMapQuestChoiceDialog.ShowWithQuest
AdventureMapQuestChoiceDialog.Refresh = function() end
AdventureMapQuestChoiceDialog.SetPortraitAtlas = function() end

local callbackResult = nil
AdventureMapQuestChoiceDialog.ShowWithQuest = function(self, map, shownPin, questID, callback, scale)
    local function wrappedCallback(result)
        callbackResult = result
        callback(result)
    end
    originalShowWithQuest(self, map, shownPin, questID, wrappedCallback, scale)
end

provider:SelectQuestID(__questChoiceDeselectQuestID, "alliance")
provider:SelectQuestID(nil)

AdventureMapQuestChoiceDialog.Refresh = originalRefresh
AdventureMapQuestChoiceDialog.SetPortraitAtlas = originalSetPortraitAtlas
AdventureMapQuestChoiceDialog.ShowWithQuest = originalShowWithQuest

return pin.panCount,
       pin.selectedValues[1] == true,
       pin.selectedValues[2] == false,
       provider.selectedQuestID == nil,
       mapCanvas.zoomOutCount,
       callbackResult,
       AdventureMapQuestChoiceDialog.questID == nil,
       AdventureMapQuestChoiceDialog.onClosedCallback == nil
"#;

#[test]
fn quest_choice_deselect_abstains_dialog_and_zooms_out() {
    with_blizzard_addon_startup_shape(&[ROOT], &[], |env, _loaded| {
        let surface = deselect_surface(env);
        assert_deselect_surface(surface);
    });
}

type DeselectSurface = (i64, bool, bool, bool, i64, i64, bool, bool);

fn deselect_surface(env: &wow_ui_sim::lua_api::WowLuaEnv) -> DeselectSurface {
    env.exec(&format!("__questChoiceDeselectQuestID = {QUEST_ID}"))
        .expect("AdventureMap quest-choice deselect setup must run cleanly");
    env.eval(DESELECT_PROBE)
        .expect("AdventureMap quest-choice deselect probe must run cleanly")
}

fn assert_deselect_surface(surface: DeselectSurface) {
    let (
        pan_count,
        selected_before_deselect,
        deselected_old_pin,
        provider_selection_cleared,
        zoom_out_count,
        callback_result,
        dialog_quest_cleared,
        dialog_callback_cleared,
    ) = surface;

    assert_initial_selection(pan_count, selected_before_deselect);
    assert_deselect_result(
        deselected_old_pin,
        provider_selection_cleared,
        zoom_out_count,
        callback_result,
    );
    assert_dialog_finalized(dialog_quest_cleared, dialog_callback_cleared);
}

fn assert_initial_selection(pan_count: i64, selected_before_deselect: bool) {
    assert_eq!(
        pan_count, 1,
        "initial `SelectQuestID` must pan to the selected quest pin"
    );
    assert!(
        selected_before_deselect,
        "initial `SelectQuestID` must mark the quest pin selected"
    );
}

fn assert_deselect_result(
    deselected_old_pin: bool,
    provider_selection_cleared: bool,
    zoom_out_count: i64,
    callback_result: i64,
) {
    assert!(
        deselected_old_pin,
        "`SelectQuestID(nil)` must deselect the previously selected pin"
    );
    assert!(
        provider_selection_cleared,
        "`SelectQuestID(nil)` must clear the provider selected quest id"
    );
    assert_eq!(
        zoom_out_count, 1,
        "`SelectQuestID(nil)` must zoom the map out"
    );
    assert_eq!(
        callback_result, 3,
        "`SelectQuestID(nil)` must close the dialog with an abstain result"
    );
}

fn assert_dialog_finalized(dialog_quest_cleared: bool, dialog_callback_cleared: bool) {
    assert!(
        dialog_quest_cleared,
        "`DeclineQuest(true)` must finalize and clear the dialog quest id"
    );
    assert!(
        dialog_callback_cleared,
        "`DeclineQuest(true)` must finalize and clear the dialog callback"
    );
}
