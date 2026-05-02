//! `ReinforceInfoFrameMixin:SelectNodeToReinforce` behavior probes.

use crate::common::blizzard_addon_harness::with_blizzard_addon_startup_shape;
use wow_ui_sim::loader::BlizzardAddonOverride;

const ROOT: &str = "Blizzard_AnimaDiversionUI";
const IMPLICIT_DEPS: &[&str] = &["Blizzard_MapCanvas", "Blizzard_SharedMapDataProviders"];
const CLOSURE_OVERRIDES: &[BlizzardAddonOverride<'_>] = &[BlizzardAddonOverride {
    addon: ROOT,
    extra_roots: IMPLICIT_DEPS,
}];
const SELECT_NODE_PROBE: &str = r#"
local originalPlaySound = PlaySound
local originalCovenantData = AnimaDiversionFrame.covenantData
local playedSounds = {}
local previousSelectionClears = 0
local reinforceCalls = {}
local selectedCalls = {}

PlaySound = function(soundKit)
    table.insert(playedSounds, soundKit)
end
AnimaDiversionFrame.covenantData = {
    animaReinforceSelectSoundKit = 98765,
}

local previousPin = {
    SetSelectedState = function(self, selected)
        if selected == false then
            previousSelectionClears = previousSelectionClears + 1
        end
    end,
}
local targetPin = {
    nodeData = {
        name = "Mirror Network",
    },
    SetReinforceState = function(self, reinforce)
        table.insert(reinforceCalls, reinforce)
    end,
    SetSelectedState = function(self, selected)
        table.insert(selectedCalls, selected)
    end,
}
local unavailablePin = {
    UnavailableState = true,
    nodeData = {
        name = "Unavailable",
    },
    SetReinforceState = function(self, reinforce)
        table.insert(reinforceCalls, reinforce)
    end,
    SetSelectedState = function(self, selected)
        table.insert(selectedCalls, selected)
    end,
}
local frame = {
    canReinforce = true,
    selectedNode = previousPin,
    Title = {
        SetText = function(self, text)
            self.text = text
        end,
        GetText = function(self)
            return self.text
        end,
    },
    AnimaNodeReinforceButton = {
        enabled = false,
        Enable = function(self)
            self.enabled = true
        end,
        IsEnabled = function(self)
            return self.enabled
        end,
    },
}
setmetatable(frame, { __index = ReinforceInfoFrameMixin })

frame:SelectNodeToReinforce(targetPin)
local selectedNodeMatches = frame.selectedNode == targetPin
local titleText = frame.Title:GetText()
local buttonEnabled = frame.AnimaNodeReinforceButton:IsEnabled()
local afterSuccessReinforceCount = #reinforceCalls
local afterSuccessSelectedCount = #selectedCalls
local afterSuccessSoundCount = #playedSounds

frame:SelectNodeToReinforce(unavailablePin)
local unavailableSkipped = frame.selectedNode == targetPin
    and #reinforceCalls == afterSuccessReinforceCount
    and #selectedCalls == afterSuccessSelectedCount
    and #playedSounds == afterSuccessSoundCount
    and frame.Title:GetText() == titleText

PlaySound = originalPlaySound
AnimaDiversionFrame.covenantData = originalCovenantData

return previousSelectionClears,
       selectedNodeMatches,
       #reinforceCalls,
       reinforceCalls[1] == true,
       #selectedCalls,
       selectedCalls[1] == true,
       titleText,
       buttonEnabled,
       #playedSounds,
       playedSounds[1] == 98765,
       unavailableSkipped
"#;

#[test]
fn reinforce_info_select_node_updates_title_button_and_selection() {
    with_blizzard_addon_startup_shape(&[ROOT], CLOSURE_OVERRIDES, |env, _loaded| {
        let state: SelectNodeState = env
            .eval(SELECT_NODE_PROBE)
            .expect("reinforce select node probe must run cleanly");

        assert_select_node_state(state);
    });
}

type SelectNodeState = (
    i64,
    bool,
    i64,
    bool,
    i64,
    bool,
    String,
    bool,
    i64,
    bool,
    bool,
);

fn assert_select_node_state(state: SelectNodeState) {
    assert_previous_selection_cleared(state.0);
    assert_target_pin_selected((state.1, state.2, state.3, state.4, state.5));
    assert_title_button_and_sound((state.6, state.7, state.8, state.9));
    assert!(
        state.10,
        "Unavailable pin must return early without mutating selection state"
    );
}

fn assert_previous_selection_cleared(clear_count: i64) {
    assert_eq!(
        clear_count, 1,
        "Selecting a fresh pin must clear the previous selected state"
    );
}

fn assert_target_pin_selected(state: (bool, i64, bool, i64, bool)) {
    let (selected_node_matches, reinforce_count, reinforce_arg, selected_count, selected_arg) =
        state;

    assert!(
        selected_node_matches,
        "`selectedNode` must become the target pin"
    );
    assert_eq!(reinforce_count, 1, "Target pin must be reinforced once");
    assert!(
        reinforce_arg,
        "Target pin must receive `SetReinforceState(true)`"
    );
    assert_eq!(selected_count, 1, "Target pin must be selected once");
    assert!(
        selected_arg,
        "Target pin must receive `SetSelectedState(true)`"
    );
}

fn assert_title_button_and_sound(state: (String, bool, i64, bool)) {
    let (title_text, button_enabled, sound_count, sound_matches) = state;

    assert_eq!(
        title_text, "Mirror Network",
        "Title must show selected node name"
    );
    assert!(button_enabled, "Reinforce button must be enabled");
    assert_eq!(sound_count, 1, "Selecting a node must play one sound");
    assert!(
        sound_matches,
        "Selecting a node must play the covenant reinforce-select sound"
    );
}
