//! Quest-choice provider removes accepted quest pins and fades the fog pin.

use crate::common::blizzard_addon_harness::with_blizzard_addon_startup_shape;

const ROOT: &str = "Blizzard_AdventureMap";
const QUEST_ID: i64 = 40519;
const OTHER_QUEST_ID: i64 = 40520;
const QUEST_ACCEPTED_PROBE: &str = r#"
local questID = __questChoiceAcceptedQuestID
local otherQuestID = __questChoiceAcceptedOtherQuestID
local removedPins = {}
local zoomOutCount = 0
local fogPlayCount = 0
local fogFinishedScript = nil
local mapCanvas = {}

local fogPin = {
    kind = "fog",
    OnQuestAcceptedAnim = {
        SetScript = function(self, scriptType, callback)
            fogFinishedScript = scriptType
            self.onFinished = callback
        end,
        Play = function(self)
            fogPlayCount = fogPlayCount + 1
            self.onFinished()
        end,
    },
}
local choicePin = { kind = "choice", questID = questID, fogPin = fogPin }
local otherPin = { kind = "other", questID = otherQuestID, fogPin = {} }

function mapCanvas:IsVisible()
    return true
end

function mapCanvas:EnumeratePinsByTemplate(template)
    local pins = { otherPin, choicePin }
    local index = 0
    return function()
        index = index + 1
        return pins[index]
    end
end

function mapCanvas:RemovePin(pin)
    table.insert(removedPins, pin.kind)
end

function mapCanvas:ZoomOut()
    zoomOutCount = zoomOutCount + 1
end

local provider = CreateFromMixins(AdventureMap_QuestChoiceDataProviderMixin)
provider.owningMap = mapCanvas

provider:OnEvent("QUEST_ACCEPTED", questID)

return fogPlayCount,
       fogFinishedScript,
       zoomOutCount,
       removedPins[1],
       removedPins[2],
       choicePin.fogPin == nil,
       #removedPins
"#;

#[test]
fn quest_choice_provider_quest_accepted_event_removes_choice_and_fog_pins() {
    with_blizzard_addon_startup_shape(&[ROOT], &[], |env, _loaded| {
        let surface = quest_accepted_surface(env);
        assert_quest_accepted_surface(surface);
    });
}

type QuestAcceptedSurface = (i64, String, i64, String, String, bool, i64);

fn quest_accepted_surface(env: &wow_ui_sim::lua_api::WowLuaEnv) -> QuestAcceptedSurface {
    env.exec(&format!(
        "__questChoiceAcceptedQuestID = {QUEST_ID}; \
         __questChoiceAcceptedOtherQuestID = {OTHER_QUEST_ID}"
    ))
    .expect("AdventureMap quest-accepted probe setup must run cleanly");
    env.eval(QUEST_ACCEPTED_PROBE)
        .expect("AdventureMap quest-accepted probe must run cleanly")
}

fn assert_quest_accepted_surface(surface: QuestAcceptedSurface) {
    let (
        fog_play_count,
        fog_finished_script,
        zoom_out_count,
        first_removed_pin,
        second_removed_pin,
        choice_fog_cleared,
        removed_pin_count,
    ) = surface;

    assert_fog_animation(fog_play_count, fog_finished_script, first_removed_pin);
    assert_choice_pin_removed(
        zoom_out_count,
        second_removed_pin,
        choice_fog_cleared,
        removed_pin_count,
    );
}

fn assert_fog_animation(
    fog_play_count: i64,
    fog_finished_script: String,
    first_removed_pin: String,
) {
    assert_eq!(
        fog_finished_script, "OnFinished",
        "`OnQuestAccepted` must hook the fog animation finish"
    );
    assert_eq!(
        fog_play_count, 1,
        "`OnQuestAccepted` must play the accepted-quest fog animation"
    );
    assert_eq!(
        first_removed_pin, "fog",
        "the fog pin must be removed by the animation finish callback"
    );
}

fn assert_choice_pin_removed(
    zoom_out_count: i64,
    second_removed_pin: String,
    choice_fog_cleared: bool,
    removed_pin_count: i64,
) {
    assert_eq!(
        zoom_out_count, 1,
        "`QUEST_ACCEPTED` must zoom the Adventure Map back out"
    );
    assert_eq!(
        second_removed_pin, "choice",
        "`QUEST_ACCEPTED` must remove the matching choice pin"
    );
    assert!(
        choice_fog_cleared,
        "`OnQuestAccepted` must detach the fog pin from the choice pin"
    );
    assert_eq!(
        removed_pin_count, 2,
        "`QUEST_ACCEPTED` must only remove the accepted quest's pins"
    );
}
