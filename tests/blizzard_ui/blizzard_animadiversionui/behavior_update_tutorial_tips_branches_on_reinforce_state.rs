//! `AnimaDiversionFrameMixin:UpdateTutorialTips` branch probes.

use crate::common::blizzard_addon_harness::with_blizzard_addon_startup_shape;
use wow_ui_sim::loader::BlizzardAddonOverride;

const ROOT: &str = "Blizzard_AnimaDiversionUI";
const IMPLICIT_DEPS: &[&str] = &["Blizzard_MapCanvas", "Blizzard_SharedMapDataProviders"];
const CLOSURE_OVERRIDES: &[BlizzardAddonOverride<'_>] = &[BlizzardAddonOverride {
    addon: ROOT,
    extra_roots: IMPLICIT_DEPS,
}];
const UPDATE_TUTORIAL_TIPS_PROBE: &str = r#"
local originalShow = HelpTip.Show
local originalGetCVarBitfield = GetCVarBitfield
local showCalls = {}
local cvarCalls = {}
local showReturn = true
local canReinforce = true
local hasAvailableNode = true

HelpTip.Show = function(self, parent, info)
    table.insert(showCalls, { parent = parent, info = info })
    return showReturn
end
GetCVarBitfield = function(name, flag)
    table.insert(cvarCalls, { name = name, flag = flag })
    return false
end

local frame = {
    ReinforceProgressFrame = {},
    CanReinforceNode = function()
        return canReinforce
    end,
    HasAvailableNode = function()
        return hasAvailableNode
    end,
}
setmetatable(frame, { __index = AnimaDiversionFrameMixin })

frame:UpdateTutorialTips()
local reinforceCall = showCalls[1]
local reinforceShortCircuited = #showCalls == 1 and #cvarCalls == 0
local reinforceTipMatches = reinforceCall.parent == frame
    and reinforceCall.info.text == ANIMA_DIVERSION_TUTORIAL_SELECT_LOCATION_PERMANENT
    and reinforceCall.info.bitfieldFlag == LE_FRAME_TUTORIAL_ANIMA_DIVERSION_REINFORCE_LOCATION
    and reinforceCall.info.cvarBitfield == "closedInfoFrames"
    and reinforceCall.info.checkCVars == true

showCalls = {}
cvarCalls = {}
showReturn = false
canReinforce = false
hasAvailableNode = true

frame:UpdateTutorialTips()
local selectLocationCall = showCalls[1]
local fillBarCall = showCalls[2]
local activateCVarChecked = cvarCalls[1]
    and cvarCalls[1].name == "closedInfoFrames"
    and cvarCalls[1].flag == LE_FRAME_TUTORIAL_ANIMA_DIVERSION_ACTIVATE_LOCATION
local selectLocationTipMatches = selectLocationCall.parent == frame
    and selectLocationCall.info.text == ANIMA_DIVERSION_TUTORIAL_SELECT_LOCATION
    and selectLocationCall.info.bitfieldFlag == LE_FRAME_TUTORIAL_ANIMA_DIVERSION_ACTIVATE_LOCATION
    and selectLocationCall.info.cvarBitfield == "closedInfoFrames"
local fillBarTipMatches = fillBarCall.parent == frame.ReinforceProgressFrame
    and fillBarCall.info.text == ANIMA_DIVERSION_TUTORIAL_FILL_BAR
    and fillBarCall.info.targetPoint == HelpTip.Point.RightEdgeCenter

HelpTip.Show = originalShow
GetCVarBitfield = originalGetCVarBitfield

return reinforceShortCircuited,
       reinforceTipMatches,
       #showCalls,
       activateCVarChecked,
       selectLocationTipMatches,
       fillBarTipMatches
"#;

#[test]
fn update_tutorial_tips_branches_on_reinforce_state() {
    with_blizzard_addon_startup_shape(&[ROOT], CLOSURE_OVERRIDES, |env, _loaded| {
        let state: TutorialTipsState = env
            .eval(UPDATE_TUTORIAL_TIPS_PROBE)
            .expect("tutorial tips probe must run cleanly");

        assert_tutorial_tips_state(state);
    });
}

type TutorialTipsState = (bool, bool, i64, bool, bool, bool);

fn assert_tutorial_tips_state(state: TutorialTipsState) {
    assert_reinforce_tip((state.0, state.1));
    assert_available_node_tips((state.2, state.3, state.4, state.5));
}

fn assert_reinforce_tip(state: (bool, bool)) {
    let (short_circuited, tip_matches) = state;

    assert!(
        short_circuited,
        "Successful reinforce-location HelpTip must short-circuit"
    );
    assert!(
        tip_matches,
        "Reinforce path must show permanent-location tutorial with reinforce bitfield"
    );
}

fn assert_available_node_tips(state: (i64, bool, bool, bool)) {
    let (show_count, cvar_checked, select_tip_matches, fill_tip_matches) = state;

    assert_eq!(
        show_count, 2,
        "Available-node path must show select-location and fill-bar tips"
    );
    assert!(
        cvar_checked,
        "Available-node path must check the activate-location tutorial bit"
    );
    assert!(
        select_tip_matches,
        "Available-node path must show select-location tutorial with activate bitfield"
    );
    assert!(
        fill_tip_matches,
        "Available-node path must show fill-bar tutorial on ReinforceProgressFrame"
    );
}
