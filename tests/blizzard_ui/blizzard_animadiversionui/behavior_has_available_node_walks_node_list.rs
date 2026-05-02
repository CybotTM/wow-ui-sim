//! `AnimaDiversionFrameMixin:HasAvailableNode` behavior probes.

use crate::common::blizzard_addon_harness::with_blizzard_addon_startup_shape;
use wow_ui_sim::loader::BlizzardAddonOverride;

const ROOT: &str = "Blizzard_AnimaDiversionUI";
const IMPLICIT_DEPS: &[&str] = &["Blizzard_MapCanvas", "Blizzard_SharedMapDataProviders"];
const CLOSURE_OVERRIDES: &[BlizzardAddonOverride<'_>] = &[BlizzardAddonOverride {
    addon: ROOT,
    extra_roots: IMPLICIT_DEPS,
}];
const HAS_AVAILABLE_NODE_PROBE: &str = r#"
local frame = AnimaDiversionFrame
local state = Enum.AnimaDiversionNodeState
local originalGetNodes = C_AnimaDiversion.GetAnimaDiversionNodes

C_AnimaDiversion.GetAnimaDiversionNodes = function()
    return {}
end
local emptyListResult = frame:HasAvailableNode()

C_AnimaDiversion.GetAnimaDiversionNodes = function()
    return {
        { state = state.Unavailable },
        { state = state.Cooldown },
    }
end
local inactiveListResult = frame:HasAvailableNode()

C_AnimaDiversion.GetAnimaDiversionNodes = function()
    return {
        { state = state.Unavailable },
        { state = state.Available },
        { state = state.Cooldown },
    }
end
local availableListResult = frame:HasAvailableNode()

C_AnimaDiversion.GetAnimaDiversionNodes = originalGetNodes

return emptyListResult, inactiveListResult, availableListResult
"#;

#[test]
fn has_available_node_returns_true_only_for_available_nodes() {
    with_blizzard_addon_startup_shape(&[ROOT], CLOSURE_OVERRIDES, |env, _loaded| {
        let state: HasAvailableNodeState = env
            .eval(HAS_AVAILABLE_NODE_PROBE)
            .expect("HasAvailableNode probe must run cleanly");

        assert_has_available_node_state(state);
    });
}

type HasAvailableNodeState = (bool, bool, bool);

fn assert_has_available_node_state(state: HasAvailableNodeState) {
    let (empty_list_result, inactive_list_result, available_list_result) = state;

    assert!(
        !empty_list_result,
        "`HasAvailableNode` must return false for an empty anima node list"
    );
    assert!(
        !inactive_list_result,
        "`HasAvailableNode` must return false when all nodes are unavailable or cooling down"
    );
    assert!(
        available_list_result,
        "`HasAvailableNode` must return true when any node is Available"
    );
}
