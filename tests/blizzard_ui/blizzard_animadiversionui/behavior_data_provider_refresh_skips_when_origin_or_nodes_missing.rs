//! `AnimaDiversionDataProviderMixin:RefreshAllData` early-return probes.

use crate::common::blizzard_addon_harness::with_blizzard_addon_startup_shape;
use wow_ui_sim::loader::BlizzardAddonOverride;

const ROOT: &str = "Blizzard_AnimaDiversionUI";
const IMPLICIT_DEPS: &[&str] = &["Blizzard_MapCanvas", "Blizzard_SharedMapDataProviders"];
const CLOSURE_OVERRIDES: &[BlizzardAddonOverride<'_>] = &[BlizzardAddonOverride {
    addon: ROOT,
    extra_roots: IMPLICIT_DEPS,
}];
const REFRESH_ALL_DATA_PROBE: &str = r#"
local originalGetProgress = C_AnimaDiversion.GetReinforceProgress
local originalGetTextureKit = C_AnimaDiversion.GetTextureKit
local originalGetOriginPosition = C_AnimaDiversion.GetOriginPosition
local originalGetNodes = C_AnimaDiversion.GetAnimaDiversionNodes
local originPosition = nil
local nodes = nil
local calls = {}

local function bump(name)
    calls[name] = (calls[name] or 0) + 1
end

C_AnimaDiversion.GetReinforceProgress = function()
    bump("progress")
    return 4
end
C_AnimaDiversion.GetTextureKit = function()
    bump("texture")
    return "Kyrian"
end
C_AnimaDiversion.GetOriginPosition = function()
    bump("origin_read")
    return originPosition
end
C_AnimaDiversion.GetAnimaDiversionNodes = function()
    bump("nodes_read")
    return nodes
end

local provider = {
    connectionPool = {
        ReleaseAll = function()
            bump("release")
        end,
    },
    RemoveAllData = function()
        bump("remove")
    end,
    AddModelScene = function()
        bump("model")
    end,
    AddOrigin = function(self, position)
        bump("origin")
        self.lastOrigin = position
    end,
    AddNode = function(self, node)
        bump("node")
        self.lastNode = node
    end,
}
setmetatable(provider, { __index = AnimaDiversionDataProviderMixin })

local function snapshot()
    return {
        remove = calls.remove or 0,
        progress = calls.progress or 0,
        release = calls.release or 0,
        texture = calls.texture or 0,
        model = calls.model or 0,
        originRead = calls.origin_read or 0,
        nodesRead = calls.nodes_read or 0,
        origin = calls.origin or 0,
        node = calls.node or 0,
    }
end

provider:RefreshAllData()
local missingOrigin = snapshot()

originPosition = { x = 0.25, y = 0.75 }
nodes = nil
provider:RefreshAllData()
local missingNodes = snapshot()

nodes = {
    { talentID = 1 },
    { talentID = 2 },
}
provider:RefreshAllData()
local complete = snapshot()

C_AnimaDiversion.GetReinforceProgress = originalGetProgress
C_AnimaDiversion.GetTextureKit = originalGetTextureKit
C_AnimaDiversion.GetOriginPosition = originalGetOriginPosition
C_AnimaDiversion.GetAnimaDiversionNodes = originalGetNodes

local function prefixCountsMatch(counts, expected)
    return counts.remove == expected
        and counts.progress == expected
        and counts.release == expected
        and counts.texture == expected
        and counts.model == expected
        and counts.originRead == expected
end

return prefixCountsMatch(missingOrigin, 1),
       missingOrigin.nodesRead == 0,
       missingOrigin.origin == 0 and missingOrigin.node == 0,
       prefixCountsMatch(missingNodes, 2),
       missingNodes.nodesRead == 1,
       missingNodes.origin == 0 and missingNodes.node == 0,
       prefixCountsMatch(complete, 3),
       complete.nodesRead == 2,
       complete.origin == 1,
       complete.node == 2,
       provider.bolsterProgress == 4,
       provider.textureKit == "Kyrian",
       provider.lastOrigin == originPosition,
       provider.lastNode == nodes[2]
"#;

#[test]
fn refresh_all_data_skips_when_origin_or_nodes_are_missing() {
    with_blizzard_addon_startup_shape(&[ROOT], CLOSURE_OVERRIDES, |env, _loaded| {
        let state: RefreshAllDataState = env
            .eval(REFRESH_ALL_DATA_PROBE)
            .expect("refresh all data probe must run cleanly");

        assert_refresh_all_data_state(state);
    });
}

type RefreshAllDataState = (
    bool,
    bool,
    bool,
    bool,
    bool,
    bool,
    bool,
    bool,
    bool,
    bool,
    bool,
    bool,
    bool,
    bool,
);

fn assert_refresh_all_data_state(state: RefreshAllDataState) {
    assert_missing_origin_path((state.0, state.1, state.2));
    assert_missing_nodes_path((state.3, state.4, state.5));
    assert_complete_path((state.6, state.7, state.8, state.9));
    assert_final_provider_state((state.10, state.11, state.12, state.13));
}

fn assert_missing_origin_path(state: (bool, bool, bool)) {
    let (prefix_counts_match, skips_nodes_read, skips_adds) = state;

    assert!(
        prefix_counts_match,
        "Missing origin path must run setup calls once"
    );
    assert!(skips_nodes_read, "Missing origin must skip reading nodes");
    assert!(
        skips_adds,
        "Missing origin must skip `AddOrigin` and `AddNode`"
    );
}

fn assert_missing_nodes_path(state: (bool, bool, bool)) {
    let (prefix_counts_match, reads_nodes, skips_adds) = state;

    assert!(
        prefix_counts_match,
        "Missing nodes path must run setup calls again"
    );
    assert!(reads_nodes, "Present origin must allow reading nodes");
    assert!(
        skips_adds,
        "Missing nodes must skip `AddOrigin` and `AddNode`"
    );
}

fn assert_complete_path(state: (bool, bool, bool, bool)) {
    let (prefix_counts_match, reads_nodes, adds_origin, adds_nodes) = state;

    assert!(
        prefix_counts_match,
        "Complete path must run setup calls a third time"
    );
    assert!(reads_nodes, "Complete path must read nodes");
    assert!(adds_origin, "Complete path must add the origin");
    assert!(adds_nodes, "Complete path must iterate all nodes");
}

fn assert_final_provider_state(state: (bool, bool, bool, bool)) {
    let (progress_matches, texture_matches, origin_matches, last_node_matches) = state;

    assert!(progress_matches, "Refresh must store reinforce progress");
    assert!(texture_matches, "Refresh must store texture kit");
    assert!(
        origin_matches,
        "Complete path must pass the origin position through"
    );
    assert!(last_node_matches, "Complete path must visit the final node");
}
