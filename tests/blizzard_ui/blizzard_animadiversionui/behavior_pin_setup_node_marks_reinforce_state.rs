//! `AnimaDiversionPinMixin:SetupNode` reinforcement-state probes.

use crate::common::blizzard_addon_harness::with_blizzard_addon_startup_shape;
use wow_ui_sim::loader::BlizzardAddonOverride;

const ROOT: &str = "Blizzard_AnimaDiversionUI";
const IMPLICIT_DEPS: &[&str] = &["Blizzard_MapCanvas", "Blizzard_SharedMapDataProviders"];
const CLOSURE_OVERRIDES: &[BlizzardAddonOverride<'_>] = &[BlizzardAddonOverride {
    addon: ROOT,
    extra_roots: IMPLICIT_DEPS,
}];
const PIN_SETUP_NODE_PROBE: &str = r#"
local state = Enum.AnimaDiversionNodeState
local originalGetTalentUnlockWorldQuest = C_Garrison.GetTalentUnlockWorldQuest
local effectCalls = {}

C_Garrison.GetTalentUnlockWorldQuest = function()
    return nil
end

local owner = {
    bolsterProgress = 10,
    CanReinforceNode = AnimaDiversionFrameMixin.CanReinforceNode,
    AddEffectOnPin = function(self, effectID, pin, permanent)
        table.insert(effectCalls, { effectID = effectID, pin = pin, permanent = permanent })
    end,
    ClearEffectOnPin = function() end,
}

local function buildPin(nodeState)
    local pin = {}
    pin.owner = owner
    pin.textureKit = "Kyrian"
    pin.nodeData = { state = nodeState, talentID = 777 }
    pin.visualStateCalls = {}
    pin.borderHideCount = 0
    pin.showCount = 0
    pin.IconBorder = {
        Hide = function()
            pin.borderHideCount = pin.borderHideCount + 1
        end,
    }
    pin.Show = function(self)
        self.showCount = self.showCount + 1
    end

    setmetatable(pin, { __index = AnimaDiversionPinMixin })
    pin.SetVisualState = function(self, visualState)
        table.insert(self.visualStateCalls, visualState)
        self.visualState = visualState
    end
    return pin
end

local availablePin = buildPin(state.Available)
availablePin:SetupNode()

local permanentPin = buildPin(state.SelectedPermanent)
permanentPin:SetupNode()

local unavailablePin = buildPin(state.Unavailable)
unavailablePin:SetupNode()

C_Garrison.GetTalentUnlockWorldQuest = originalGetTalentUnlockWorldQuest

return effectCalls[1] and effectCalls[1].effectID,
       effectCalls[1] and effectCalls[1].pin == availablePin,
       effectCalls[1] and effectCalls[1].permanent == true,
       effectCalls[2] and effectCalls[2].effectID,
       effectCalls[2] and effectCalls[2].pin == permanentPin,
       effectCalls[2] and effectCalls[2].permanent == true,
       #effectCalls,
       availablePin.visualStateCalls[1],
       permanentPin.visualStateCalls[1],
       unavailablePin.visualStateCalls[1],
       availablePin.borderHideCount,
       permanentPin.borderHideCount,
       unavailablePin.borderHideCount,
       availablePin.showCount,
       permanentPin.showCount,
       unavailablePin.showCount
"#;

#[test]
fn setup_node_marks_reinforce_state_for_reinforceable_pins() {
    with_blizzard_addon_startup_shape(&[ROOT], CLOSURE_OVERRIDES, |env, _loaded| {
        let state: PinSetupNodeState = env
            .eval(PIN_SETUP_NODE_PROBE)
            .expect("pin setup node probe must run cleanly");

        assert_pin_setup_node_state(state);
    });
}

type PinSetupNodeState = (
    i64,
    bool,
    bool,
    i64,
    bool,
    bool,
    i64,
    i64,
    i64,
    i64,
    i64,
    i64,
    i64,
    i64,
    i64,
    i64,
);

type ReinforceEffectState = (i64, bool, bool, i64, bool, bool, i64);
type VisualState = (i64, i64, i64);
type PinCallCounts = (i64, i64, i64, i64, i64, i64);

fn assert_pin_setup_node_state(state: PinSetupNodeState) {
    assert_reinforce_effects(reinforce_effect_state(state));
    assert_visual_states(visual_state(state));
    assert_border_and_show_calls(pin_call_counts(state));
}

fn reinforce_effect_state(state: PinSetupNodeState) -> ReinforceEffectState {
    (
        state.0, state.1, state.2, state.3, state.4, state.5, state.6,
    )
}

fn visual_state(state: PinSetupNodeState) -> VisualState {
    (state.7, state.8, state.9)
}

fn pin_call_counts(state: PinSetupNodeState) -> PinCallCounts {
    (state.10, state.11, state.12, state.13, state.14, state.15)
}

fn assert_reinforce_effects(state: ReinforceEffectState) {
    let (
        available_effect_id,
        available_pin_matched,
        available_permanent,
        permanent_effect_id,
        permanent_pin_matched,
        permanent_permanent,
        effect_count,
    ) = state;

    assert_available_pin_reinforced(
        available_effect_id,
        available_pin_matched,
        available_permanent,
    );
    assert_permanent_pin_reinforced(
        permanent_effect_id,
        permanent_pin_matched,
        permanent_permanent,
    );
    assert_unavailable_pin_skips_reinforce(effect_count);
}

fn assert_available_pin_reinforced(effect_id: i64, pin_matched: bool, permanent: bool) {
    assert_eq!(effect_id, 22, "Kyrian reinforce effect ID must be used");
    assert!(
        pin_matched,
        "Available pin must receive the reinforce effect"
    );
    assert!(
        !permanent,
        "Available pin reinforce effect must not be marked permanent"
    );
}

fn assert_permanent_pin_reinforced(effect_id: i64, pin_matched: bool, permanent: bool) {
    assert_eq!(effect_id, 22, "Kyrian reinforce effect ID must be used");
    assert!(
        pin_matched,
        "SelectedPermanent pin must receive the reinforce effect"
    );
    assert!(
        permanent,
        "SelectedPermanent pin reinforce effect must be marked permanent"
    );
}

fn assert_unavailable_pin_skips_reinforce(effect_count: i64) {
    assert_eq!(
        effect_count, 2,
        "Unavailable pin must not call `SetReinforceState`"
    );
}

fn assert_visual_states(state: VisualState) {
    let (available_state, permanent_state, unavailable_state) = state;

    assert_eq!(
        available_state, 1,
        "Available pin visual must stay Available"
    );
    assert_eq!(
        permanent_state, 3,
        "SelectedPermanent pin visual must stay SelectedPermanent"
    );
    assert_eq!(
        unavailable_state, 0,
        "Unavailable pin visual must stay Unavailable"
    );
}

fn assert_border_and_show_calls(state: PinCallCounts) {
    let (
        available_border_hide_count,
        permanent_border_hide_count,
        unavailable_border_hide_count,
        available_show_count,
        permanent_show_count,
        unavailable_show_count,
    ) = state;

    assert_eq!(available_border_hide_count, 1, "Available pin hides border");
    assert_eq!(
        permanent_border_hide_count, 1,
        "SelectedPermanent pin hides border"
    );
    assert_eq!(
        unavailable_border_hide_count, 1,
        "Unavailable pin hides border"
    );
    assert_eq!(
        available_show_count, 1,
        "Available pin is shown after setup"
    );
    assert_eq!(
        permanent_show_count, 1,
        "SelectedPermanent pin is shown after setup"
    );
    assert_eq!(
        unavailable_show_count, 1,
        "Unavailable pin is shown after setup"
    );
}
