//! `ANIMA_DIVERSION_CONFIRM_REINFORCE` popup accept behavior probes.

use crate::common::blizzard_addon_harness::with_blizzard_addon_startup_shape;
use wow_ui_sim::loader::BlizzardAddonOverride;

const ROOT: &str = "Blizzard_AnimaDiversionUI";
const IMPLICIT_DEPS: &[&str] = &["Blizzard_MapCanvas", "Blizzard_SharedMapDataProviders"];
const CLOSURE_OVERRIDES: &[BlizzardAddonOverride<'_>] = &[BlizzardAddonOverride {
    addon: ROOT,
    extra_roots: IMPLICIT_DEPS,
}];
const CONFIRM_REINFORCE_PROBE: &str = r#"
local originalPlaySound = PlaySound
local originalSelectAnimaNode = C_AnimaDiversion.SelectAnimaNode
local originalAcknowledge = HelpTip.Acknowledge
local playedSounds = {}
local selections = {}
local acknowledged = {}

PlaySound = function(soundKit)
    table.insert(playedSounds, soundKit)
end
C_AnimaDiversion.SelectAnimaNode = function(talentID, temporary)
    table.insert(selections, { talentID = talentID, temporary = temporary })
end
HelpTip.Acknowledge = function(self, parent, text)
    table.insert(acknowledged, { parent = parent, text = text })
end

local pin = {
    nodeData = {
        talentID = 904,
    },
}
local dialogInfo = StaticPopupDialogs["ANIMA_DIVERSION_CONFIRM_REINFORCE"]
local dialog = { dialogInfo = dialogInfo }
dialogInfo.OnAccept(dialog, pin)

local soundMatches = playedSounds[1] == SOUNDKIT.UI_COVENANT_ANIMA_DIVERSION_CONFIRM_REINFORCE
local selectionMatches = selections[1].talentID == 904 and selections[1].temporary == false
local acknowledgementMatches = acknowledged[1].parent == AnimaDiversionFrame
    and acknowledged[1].text == ANIMA_DIVERSION_TUTORIAL_SELECT_LOCATION_PERMANENT

PlaySound = originalPlaySound
C_AnimaDiversion.SelectAnimaNode = originalSelectAnimaNode
HelpTip.Acknowledge = originalAcknowledge

return #playedSounds,
       soundMatches,
       #selections,
       selectionMatches,
       #acknowledged,
       acknowledgementMatches
"#;

#[test]
fn confirm_reinforce_popup_accept_selects_permanent_node() {
    with_blizzard_addon_startup_shape(&[ROOT], CLOSURE_OVERRIDES, |env, _loaded| {
        let state: ConfirmReinforceState = env
            .eval(CONFIRM_REINFORCE_PROBE)
            .expect("confirm reinforce popup probe must run cleanly");

        assert_confirm_reinforce_state(state);
    });
}

type ConfirmReinforceState = (i64, bool, i64, bool, i64, bool);

fn assert_confirm_reinforce_state(state: ConfirmReinforceState) {
    assert_sound_call((state.0, state.1));
    assert_selection_call((state.2, state.3));
    assert_help_tip_acknowledgement((state.4, state.5));
}

fn assert_sound_call(state: (i64, bool)) {
    let (sound_count, sound_matches) = state;

    assert_eq!(sound_count, 1, "Accept must play exactly one sound");
    assert!(
        sound_matches,
        "Accept must play the reinforce confirmation sound"
    );
}

fn assert_selection_call(state: (i64, bool)) {
    let (selection_count, selection_matches) = state;

    assert_eq!(
        selection_count, 1,
        "Accept must select exactly one anima node"
    );
    assert!(
        selection_matches,
        "Accept must select the pin talent ID as a permanent reinforce"
    );
}

fn assert_help_tip_acknowledgement(state: (i64, bool)) {
    let (acknowledgement_count, acknowledgement_matches) = state;

    assert_eq!(
        acknowledgement_count, 1,
        "Accept must acknowledge exactly one tutorial tip"
    );
    assert!(
        acknowledgement_matches,
        "Accept must acknowledge the permanent-location tutorial on AnimaDiversionFrame"
    );
}
