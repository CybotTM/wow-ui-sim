//! Reinforcement predicate behavior probes.

use crate::common::blizzard_addon_harness::with_blizzard_addon_startup_shape;
use wow_ui_sim::loader::BlizzardAddonOverride;

const ROOT: &str = "Blizzard_AnimaDiversionUI";
const IMPLICIT_DEPS: &[&str] = &["Blizzard_MapCanvas", "Blizzard_SharedMapDataProviders"];
const CLOSURE_OVERRIDES: &[BlizzardAddonOverride<'_>] = &[BlizzardAddonOverride {
    addon: ROOT,
    extra_roots: IMPLICIT_DEPS,
}];
const CAN_REINFORCE_PROBE: &str = r#"
local frame = AnimaDiversionFrame
local reinforceInfo = frame.ReinforceInfoFrame
local state = Enum.AnimaDiversionNodeState
local originalGetNodes = C_AnimaDiversion.GetAnimaDiversionNodes

frame.bolsterProgress = 9
local progressNine = frame:CanReinforceNode()

frame.bolsterProgress = 10
local progressTen = frame:CanReinforceNode()

C_AnimaDiversion.GetAnimaDiversionNodes = function()
    return {}
end
local emptyList = reinforceInfo:CanReinforceAnything()

C_AnimaDiversion.GetAnimaDiversionNodes = function()
    return {
        { state = state.Unavailable },
        { state = state.Cooldown },
        { state = state.SelectedPermanent },
    }
end
local inactiveList = reinforceInfo:CanReinforceAnything()

C_AnimaDiversion.GetAnimaDiversionNodes = function()
    return {
        { state = state.Cooldown },
        { state = state.Available },
    }
end
local availableList = reinforceInfo:CanReinforceAnything()

C_AnimaDiversion.GetAnimaDiversionNodes = function()
    return {
        { state = state.Unavailable },
        { state = state.SelectedTemporary },
    }
end
local selectedTemporaryList = reinforceInfo:CanReinforceAnything()

C_AnimaDiversion.GetAnimaDiversionNodes = originalGetNodes

return progressNine,
       progressTen,
       emptyList,
       inactiveList,
       availableList,
       selectedTemporaryList
"#;

#[test]
fn can_reinforce_gates_on_progress_and_reinforceable_nodes() {
    with_blizzard_addon_startup_shape(&[ROOT], CLOSURE_OVERRIDES, |env, _loaded| {
        let state: CanReinforceState = env
            .eval(CAN_REINFORCE_PROBE)
            .expect("reinforcement predicate probe must run cleanly");

        assert_can_reinforce_state(state);
    });
}

type CanReinforceState = (bool, bool, bool, bool, bool, bool);

fn assert_can_reinforce_state(state: CanReinforceState) {
    let (
        progress_nine,
        progress_ten,
        empty_list,
        inactive_list,
        available_list,
        selected_temporary_list,
    ) = state;

    assert_progress_gate(progress_nine, progress_ten);
    assert_node_list_gate(
        empty_list,
        inactive_list,
        available_list,
        selected_temporary_list,
    );
}

fn assert_progress_gate(progress_nine: bool, progress_ten: bool) {
    assert!(
        !progress_nine,
        "`CanReinforceNode` must return false below 10 bolster progress"
    );
    assert!(
        progress_ten,
        "`CanReinforceNode` must return true at 10 bolster progress"
    );
}

fn assert_node_list_gate(
    empty_list: bool,
    inactive_list: bool,
    available_list: bool,
    selected_temporary_list: bool,
) {
    assert!(
        !empty_list,
        "`CanReinforceAnything` must return false for an empty node list"
    );
    assert!(
        !inactive_list,
        "`CanReinforceAnything` must ignore Unavailable, Cooldown, and SelectedPermanent nodes"
    );
    assert!(
        available_list,
        "`CanReinforceAnything` must return true when any node is Available"
    );
    assert!(
        selected_temporary_list,
        "`CanReinforceAnything` must return true when any node is SelectedTemporary"
    );
}
