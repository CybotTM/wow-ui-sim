//! Quest-choice selection pans to the pin, shows the dialog, and picks a portrait.

use crate::common::blizzard_addon_harness::with_blizzard_addon_startup_shape;

const ROOT: &str = "Blizzard_AdventureMap";
const QUEST_ID: i64 = 40519;
const EXPECTED_DIALOG_SCALE: f64 = 0.5;
const SELECT_QUEST_PROBE: &str = r#"
local questID = __questChoiceSelectQuestID
local textureKit = __questChoiceSelectTextureKit
local mapCanvas = {}
local pin = {
    panCount = 0,
    selected = false,
    PanAndZoomTo = function(self)
        self.panCount = self.panCount + 1
    end,
    SetSelected = function(self, selected)
        self.selected = selected
    end,
}
local provider = CreateFromMixins(AdventureMap_QuestChoiceDataProviderMixin)
provider.owningMap = mapCanvas
provider.pinsByQuestID = { [questID] = pin }

local originalShowWithQuest = AdventureMapQuestChoiceDialog.ShowWithQuest
local originalSetPortraitAtlas = AdventureMapQuestChoiceDialog.SetPortraitAtlas
local dialogQuestID = nil
local dialogMapMatches = false
local dialogPinMatches = false
local callbackIsFunction = false
local dialogScale = nil
local portraitAtlas = nil

AdventureMapQuestChoiceDialog.ShowWithQuest = function(self, map, shownPin, questIDArg, callback, scale)
    dialogMapMatches = map == mapCanvas
    dialogPinMatches = shownPin == pin
    dialogQuestID = questIDArg
    callbackIsFunction = type(callback) == "function"
    dialogScale = scale
end
AdventureMapQuestChoiceDialog.SetPortraitAtlas = function(self, atlas)
    portraitAtlas = atlas
end

provider:SelectQuestID(questID, textureKit)

AdventureMapQuestChoiceDialog.ShowWithQuest = originalShowWithQuest
AdventureMapQuestChoiceDialog.SetPortraitAtlas = originalSetPortraitAtlas

return pin.panCount,
       pin.selected == true,
       provider.selectedQuestID == questID,
       dialogMapMatches,
       dialogQuestID,
       dialogPinMatches,
       callbackIsFunction,
       dialogScale,
       portraitAtlas
"#;

#[test]
fn quest_choice_select_quest_id_shows_dialog_for_each_texture_kit() {
    with_blizzard_addon_startup_shape(&[ROOT], &[], |env, _loaded| {
        for texture_kit in ["alliance", "horde", "neutral"] {
            let surface = select_quest_surface(env, texture_kit);
            assert_select_quest_surface(texture_kit, surface);
        }
    });
}

type SelectQuestSurface = (i64, bool, bool, bool, i64, bool, bool, f64, String);

fn select_quest_surface(
    env: &wow_ui_sim::lua_api::WowLuaEnv,
    texture_kit: &str,
) -> SelectQuestSurface {
    seed_select_quest_probe(env, texture_kit);
    env.eval(SELECT_QUEST_PROBE)
        .expect("AdventureMap quest-choice selection probe must run cleanly")
}

fn seed_select_quest_probe(env: &wow_ui_sim::lua_api::WowLuaEnv, texture_kit: &str) {
    env.exec(&format!(
        "__questChoiceSelectQuestID = {QUEST_ID}; __questChoiceSelectTextureKit = {texture_kit:?}"
    ))
    .expect("AdventureMap quest-choice selection probe setup must run cleanly");
}

fn assert_select_quest_surface(texture_kit: &str, surface: SelectQuestSurface) {
    let (
        pan_count,
        pin_selected,
        provider_selected_quest_id,
        dialog_map_matches,
        dialog_quest_id,
        dialog_pin_matches,
        callback_is_function,
        dialog_scale,
        portrait_atlas,
    ) = surface;

    assert_pin_selected(pan_count, pin_selected, provider_selected_quest_id);
    assert_dialog_invocation(
        dialog_map_matches,
        dialog_quest_id,
        dialog_pin_matches,
        callback_is_function,
        dialog_scale,
    );
    assert_eq!(
        portrait_atlas,
        expected_portrait_atlas(texture_kit),
        "`SelectQuestID` must pick the portrait atlas for `{texture_kit}`"
    );
}

fn assert_pin_selected(pan_count: i64, pin_selected: bool, provider_selected_quest_id: bool) {
    assert_eq!(pan_count, 1, "`SelectQuestID` must pan to the selected pin");
    assert!(
        pin_selected,
        "`SelectQuestID` must mark the selected pin as selected"
    );
    assert!(
        provider_selected_quest_id,
        "`SelectQuestID` must store the selected quest id"
    );
}

fn assert_dialog_invocation(
    dialog_map_matches: bool,
    dialog_quest_id: i64,
    dialog_pin_matches: bool,
    callback_is_function: bool,
    dialog_scale: f64,
) {
    assert!(
        dialog_map_matches,
        "`SelectQuestID` must show the dialog with the provider's map"
    );
    assert_eq!(
        dialog_quest_id, QUEST_ID,
        "`SelectQuestID` must show the dialog for the selected quest"
    );
    assert!(
        dialog_pin_matches,
        "`SelectQuestID` must show the dialog with the selected pin"
    );
    assert!(
        callback_is_function,
        "`SelectQuestID` must pass an OnClosed callback to the dialog"
    );
    assert_approx_eq(
        dialog_scale,
        EXPECTED_DIALOG_SCALE,
        "`SelectQuestID` must pass the dialog display scale",
    );
}

fn expected_portrait_atlas(texture_kit: &str) -> &'static str {
    match texture_kit {
        "alliance" => "QuestPortraitIcon-Alliance",
        "horde" => "QuestPortraitIcon-Horde",
        _ => "QuestPortraitIcon-SandboxQuest",
    }
}

fn assert_approx_eq(actual: f64, expected: f64, context: &str) {
    let delta = (actual - expected).abs();
    assert!(
        delta < 0.001,
        "{context}: expected {expected}, got {actual}"
    );
}
