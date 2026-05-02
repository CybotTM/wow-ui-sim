//! `AnimaNodeReinforceButtonMixin:OnClick` popup-routing probes.

use crate::common::blizzard_addon_harness::with_blizzard_addon_startup_shape;
use wow_ui_sim::loader::BlizzardAddonOverride;

const ROOT: &str = "Blizzard_AnimaDiversionUI";
const IMPLICIT_DEPS: &[&str] = &["Blizzard_MapCanvas", "Blizzard_SharedMapDataProviders"];
const CLOSURE_OVERRIDES: &[BlizzardAddonOverride<'_>] = &[BlizzardAddonOverride {
    addon: ROOT,
    extra_roots: IMPLICIT_DEPS,
}];
const REINFORCE_BUTTON_CLICK_PROBE: &str = r#"
local originalPlaySound = PlaySound
local originalStaticPopupShow = StaticPopup_Show
local playedSounds = {}
local popupCalls = {}
local selectedNode = nil

PlaySound = function(soundKit)
    table.insert(playedSounds, soundKit)
end
StaticPopup_Show = function(which, textArg1, textArg2, data)
    table.insert(popupCalls, {
        which = which,
        textArg1 = textArg1,
        textArg2 = textArg2,
        data = data,
    })
end

local pin = {
    nodeData = {
        name = "Mirror Network",
    },
}
local parent = {
    GetSelectedNode = function()
        return selectedNode
    end,
}
local button = {
    GetParent = function()
        return parent
    end,
}
setmetatable(button, { __index = AnimaNodeReinforceButtonMixin })

button:OnClick()
local nilSelectionDidNothing = #playedSounds == 0 and #popupCalls == 0

selectedNode = pin
button:OnClick()
local popup = popupCalls[1]

PlaySound = originalPlaySound
StaticPopup_Show = originalStaticPopupShow

return nilSelectionDidNothing,
       #playedSounds,
       playedSounds[1] == SOUNDKIT.UI_COVENANT_ANIMA_DIVERSION_CLICK_REINFORCE_BUTTON,
       #popupCalls,
       popup and popup.which == "ANIMA_DIVERSION_CONFIRM_REINFORCE",
       popup and popup.textArg1 == "Mirror Network",
       popup and popup.textArg2 == nil,
       popup and popup.data == pin
"#;

#[test]
fn reinforce_button_click_shows_reinforce_popup_for_selected_node() {
    with_blizzard_addon_startup_shape(&[ROOT], CLOSURE_OVERRIDES, |env, _loaded| {
        let state: ReinforceButtonClickState = env
            .eval(REINFORCE_BUTTON_CLICK_PROBE)
            .expect("reinforce button click probe must run cleanly");

        assert_reinforce_button_click_state(state);
    });
}

type ReinforceButtonClickState = (bool, i64, bool, i64, bool, bool, bool, bool);

fn assert_reinforce_button_click_state(state: ReinforceButtonClickState) {
    assert!(
        state.0,
        "Button click with no selected node must not play sound or show popup"
    );
    assert_sound_call((state.1, state.2));
    assert_popup_call((state.3, state.4, state.5, state.6, state.7));
}

fn assert_sound_call(state: (i64, bool)) {
    let (sound_count, sound_matches) = state;

    assert_eq!(sound_count, 1, "Selected node click must play one sound");
    assert!(
        sound_matches,
        "Selected node click must play reinforce button sound"
    );
}

fn assert_popup_call(state: (i64, bool, bool, bool, bool)) {
    let (popup_count, which_matches, name_matches, text_arg2_is_nil, data_matches) = state;

    assert_eq!(popup_count, 1, "Selected node click must show one popup");
    assert!(
        which_matches,
        "Selected node click must show reinforce confirmation popup"
    );
    assert!(name_matches, "Popup must receive selected node name");
    assert!(text_arg2_is_nil, "Popup textArg2 must be nil");
    assert!(data_matches, "Popup data must be the selected pin");
}
