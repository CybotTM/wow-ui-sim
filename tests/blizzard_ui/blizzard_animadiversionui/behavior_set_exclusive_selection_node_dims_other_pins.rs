//! `AnimaDiversionFrameMixin:SetExclusiveSelectionNode` behavior probes.

use crate::common::blizzard_addon_harness::with_blizzard_addon_startup_shape;
use wow_ui_sim::loader::BlizzardAddonOverride;

const ROOT: &str = "Blizzard_AnimaDiversionUI";
const IMPLICIT_DEPS: &[&str] = &["Blizzard_MapCanvas", "Blizzard_SharedMapDataProviders"];
const CLOSURE_OVERRIDES: &[BlizzardAddonOverride<'_>] = &[BlizzardAddonOverride {
    addon: ROOT,
    extra_roots: IMPLICIT_DEPS,
}];
const EXCLUSIVE_SELECTION_PROBE: &str = r#"
local frame = AnimaDiversionFrame
local state = Enum.AnimaDiversionNodeState
local refreshCount = 0

local function pinWithState(nodeState)
    return {
        visualState = state.Available,
        nodeData = { state = nodeState },
        visualStateCalls = {},
        SetVisualState = function(self, visualState)
            table.insert(self.visualStateCalls, visualState)
            self.visualState = visualState
        end,
    }
end

local targetPin = pinWithState(state.Available)
local availablePin = pinWithState(state.Available)
local selectedTemporaryPin = pinWithState(state.SelectedTemporary)
local unavailablePin = pinWithState(state.Unavailable)
unavailablePin.visualState = state.Unavailable

local pins = { targetPin, availablePin, selectedTemporaryPin, unavailablePin }
local originalEnumeratePins = frame.EnumeratePinsByTemplate
local originalRefreshAllDataProviders = frame.RefreshAllDataProviders

frame.EnumeratePinsByTemplate = function(self, template)
    local index = 0
    return function()
        index = index + 1
        return pins[index]
    end
end
frame.RefreshAllDataProviders = function(self)
    refreshCount = refreshCount + 1
end

frame:SetExclusiveSelectionNode(targetPin)
local availableCall = availablePin.visualStateCalls[1]
local selectedTemporaryCall = selectedTemporaryPin.visualStateCalls[1]
local targetCallCount = #targetPin.visualStateCalls
local unavailableCallCount = #unavailablePin.visualStateCalls
local disallowAfterSet = frame.disallowSelection

frame:ClearExclusiveSelectionNode()
local disallowAfterClear = frame.disallowSelection

frame.EnumeratePinsByTemplate = originalEnumeratePins
frame.RefreshAllDataProviders = originalRefreshAllDataProviders

return availableCall,
       selectedTemporaryCall,
       targetCallCount,
       unavailableCallCount,
       disallowAfterSet,
       refreshCount,
       disallowAfterClear
"#;

#[test]
fn exclusive_selection_dims_available_pins_and_clears_selection_lock() {
    with_blizzard_addon_startup_shape(&[ROOT], CLOSURE_OVERRIDES, |env, _loaded| {
        let state: ExclusiveSelectionState = env
            .eval(EXCLUSIVE_SELECTION_PROBE)
            .expect("exclusive selection probe must run cleanly");

        assert_exclusive_selection_state(state);
    });
}

type ExclusiveSelectionState = (i64, i64, i64, i64, bool, i64, bool);

fn assert_exclusive_selection_state(state: ExclusiveSelectionState) {
    let (
        available_call,
        selected_temporary_call,
        target_call_count,
        unavailable_call_count,
        disallow_after_set,
        refresh_count,
        disallow_after_clear,
    ) = state;

    assert_pin_visual_updates(
        available_call,
        selected_temporary_call,
        target_call_count,
        unavailable_call_count,
    );
    assert_selection_lock_lifecycle(disallow_after_set, refresh_count, disallow_after_clear);
}

fn assert_pin_visual_updates(
    available_call: i64,
    selected_temporary_call: i64,
    target_call_count: i64,
    unavailable_call_count: i64,
) {
    assert_eq!(
        available_call, 4,
        "plain Available non-target pins must be dimmed to Cooldown"
    );
    assert_eq!(
        selected_temporary_call, 2,
        "SelectedTemporary non-target pins must keep their SelectedTemporary visual"
    );
    assert_eq!(target_call_count, 0, "target pin must not be rewritten");
    assert_eq!(
        unavailable_call_count, 0,
        "pins whose current visual is not Available must not be rewritten"
    );
}

fn assert_selection_lock_lifecycle(
    disallow_after_set: bool,
    refresh_count: i64,
    disallow_after_clear: bool,
) {
    assert!(
        disallow_after_set,
        "`SetExclusiveSelectionNode` must block normal selection"
    );
    assert_eq!(
        refresh_count, 1,
        "`ClearExclusiveSelectionNode` must refresh all data providers"
    );
    assert!(
        !disallow_after_clear,
        "`ClearExclusiveSelectionNode` must clear the selection lock"
    );
}
