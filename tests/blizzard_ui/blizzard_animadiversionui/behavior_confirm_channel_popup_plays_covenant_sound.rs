//! `ANIMA_DIVERSION_CONFIRM_CHANNEL` popup accept behavior probes.

use crate::common::blizzard_addon_harness::with_blizzard_addon_startup_shape;
use wow_ui_sim::loader::BlizzardAddonOverride;

const ROOT: &str = "Blizzard_AnimaDiversionUI";
const IMPLICIT_DEPS: &[&str] = &["Blizzard_MapCanvas", "Blizzard_SharedMapDataProviders"];
const CLOSURE_OVERRIDES: &[BlizzardAddonOverride<'_>] = &[BlizzardAddonOverride {
    addon: ROOT,
    extra_roots: IMPLICIT_DEPS,
}];
const CONFIRM_CHANNEL_PROBE: &str = r#"
local originalPlaySound = PlaySound
local originalSelectAnimaNode = C_AnimaDiversion.SelectAnimaNode
local originalAcknowledge = HelpTip.Acknowledge
local originalSetExclusiveSelectionNode = AnimaDiversionFrame.SetExclusiveSelectionNode
local originalClearExclusiveSelectionNode = AnimaDiversionFrame.ClearExclusiveSelectionNode
local playedSounds = {}
local selections = {}
local acknowledged = {}
local expectedSounds = {
    Kyrian = SOUNDKIT.UI_9_0_ANIMA_DIVERSION_BASTION_CONFIRM_CHANNEL,
    Venthyr = SOUNDKIT.UI_9_0_ANIMA_DIVERSION_REVENDRETH_CONFIRM_CHANNEL,
    NightFae = SOUNDKIT.UI_9_0_ANIMA_DIVERSION_ARDENWEALD_CONFIRM_CHANNEL,
    Necrolord = SOUNDKIT.UI_9_0_ANIMA_DIVERSION_MALDRAXXUS_CONFIRM_CHANNEL,
}
local textureKits = { "Kyrian", "Venthyr", "NightFae", "Necrolord" }

PlaySound = function(soundKit)
    table.insert(playedSounds, soundKit)
end
C_AnimaDiversion.SelectAnimaNode = function(talentID, temporary)
    table.insert(selections, { talentID = talentID, temporary = temporary })
end
HelpTip.Acknowledge = function(self, parent, text)
    table.insert(acknowledged, { parent = parent, text = text })
end
AnimaDiversionFrame.SetExclusiveSelectionNode = function() end
AnimaDiversionFrame.ClearExclusiveSelectionNode = function() end

local allSoundsMatch = true
local allSelectionsMatch = true
local allSelectLocationTipsMatch = true
local allFillBarTipsMatch = true

for index, textureKit in ipairs(textureKits) do
    local talentID = 700 + index
    local pin = {
        textureKit = textureKit,
        nodeData = {
            talentID = talentID,
        },
    }
    local dialogInfo = StaticPopupDialogs["ANIMA_DIVERSION_CONFIRM_CHANNEL"]
    local dialog = { dialogInfo = dialogInfo }
    dialogInfo.OnAccept(dialog, pin)

    allSoundsMatch = allSoundsMatch and playedSounds[index] == expectedSounds[textureKit]
    allSelectionsMatch = allSelectionsMatch
        and selections[index].talentID == talentID
        and selections[index].temporary == true

    local firstTip = acknowledged[(index - 1) * 2 + 1]
    local secondTip = acknowledged[(index - 1) * 2 + 2]
    allSelectLocationTipsMatch = allSelectLocationTipsMatch
        and firstTip.parent == AnimaDiversionFrame
        and firstTip.text == ANIMA_DIVERSION_TUTORIAL_SELECT_LOCATION
    allFillBarTipsMatch = allFillBarTipsMatch
        and secondTip.parent == AnimaDiversionFrame.ReinforceProgressFrame
        and secondTip.text == ANIMA_DIVERSION_TUTORIAL_FILL_BAR

end

PlaySound = originalPlaySound
C_AnimaDiversion.SelectAnimaNode = originalSelectAnimaNode
HelpTip.Acknowledge = originalAcknowledge
AnimaDiversionFrame.SetExclusiveSelectionNode = originalSetExclusiveSelectionNode
AnimaDiversionFrame.ClearExclusiveSelectionNode = originalClearExclusiveSelectionNode

return #playedSounds,
       allSoundsMatch,
       #selections,
       allSelectionsMatch,
       #acknowledged,
       allSelectLocationTipsMatch,
       allFillBarTipsMatch
"#;

#[test]
fn confirm_channel_popup_accept_plays_covenant_sound_and_selects_node() {
    with_blizzard_addon_startup_shape(&[ROOT], CLOSURE_OVERRIDES, |env, _loaded| {
        let state: ConfirmChannelState = env
            .eval(CONFIRM_CHANNEL_PROBE)
            .expect("confirm channel popup probe must run cleanly");

        assert_confirm_channel_state(state);
    });
}

type ConfirmChannelState = (i64, bool, i64, bool, i64, bool, bool);

fn assert_confirm_channel_state(state: ConfirmChannelState) {
    assert_sound_calls((state.0, state.1));
    assert_selection_calls((state.2, state.3));
    assert_help_tip_acknowledgements((state.4, state.5, state.6));
}

fn assert_sound_calls(state: (i64, bool)) {
    let (sound_count, all_sounds_match) = state;

    assert_eq!(
        sound_count, 4,
        "Each covenant texture kit must play one confirmation sound"
    );
    assert!(
        all_sounds_match,
        "Each texture kit must play its covenant confirmation sound"
    );
}

fn assert_selection_calls(state: (i64, bool)) {
    let (selection_count, all_selections_match) = state;

    assert_eq!(
        selection_count, 4,
        "Each accept must select exactly one anima node"
    );
    assert!(
        all_selections_match,
        "Each accept must select the pin talent ID as a temporary channel"
    );
}

fn assert_help_tip_acknowledgements(state: (i64, bool, bool)) {
    let (acknowledgement_count, select_location_tips_match, fill_bar_tips_match) = state;

    assert_eq!(
        acknowledgement_count, 8,
        "Each accept must acknowledge both channel-selection tutorial tips"
    );
    assert!(
        select_location_tips_match,
        "Each accept must acknowledge the select-location tip on AnimaDiversionFrame"
    );
    assert!(
        fill_bar_tips_match,
        "Each accept must acknowledge the fill-bar tip on ReinforceProgressFrame"
    );
}
