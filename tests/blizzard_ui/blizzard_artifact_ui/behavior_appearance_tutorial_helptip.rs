//! Appearance-tab tutorial HelpTip behavior for `Blizzard_ArtifactUI`.

use crate::common::blizzard_addon_harness::with_blizzard_addon_smoke_shape;
use wow_ui_sim::lua_api::state::{
    ArtifactAppearanceInfo, ArtifactAppearanceSetInfo, ArtifactInfo, ColorRgb,
};

const ROOT: &str = "Blizzard_ArtifactUI";
const ARTIFACT_ICON: &str = "Interface/Icons/inv_sword_2h_artifactashbringer_d_01";

#[test]
fn appearance_tutorial_helptip_shows_until_closed_info_frame_bit_is_set() {
    with_blizzard_addon_smoke_shape(&[], &[], |env, _loaded| {
        seed_viewed_artifact_with_unlocked_appearances(env);
        load_artifact_ui(env);

        let mismatches: Vec<String> = env
            .eval(APPEARANCE_TUTORIAL_HELPTIP_PROBE)
            .expect("ArtifactUI appearance tutorial HelpTip probe should run cleanly");

        assert!(
            mismatches.is_empty(),
            "`{ROOT}` must show the appearance tutorial HelpTip until its cvar bit is closed; \
             mismatches: {mismatches:?}"
        );
    });
}

fn seed_viewed_artifact_with_unlocked_appearances(env: &wow_ui_sim::lua_api::WowLuaEnv) {
    let mut state = env.state().borrow_mut();
    state.viewed_artifact.info = Some(sample_artifact());
    state.viewed_artifact.total_purchased_ranks = 1;
    state.viewed_artifact.is_at_forge = true;
    state
        .viewed_artifact
        .appearance_sets
        .push(sample_appearance_set());
    state
        .viewed_artifact
        .appearances
        .insert((1, 1), sample_appearance(101, "First Look"));
    state
        .viewed_artifact
        .appearances
        .insert((1, 2), sample_appearance(102, "Second Look"));
}

fn sample_artifact() -> ArtifactInfo {
    ArtifactInfo {
        item_id: 128_910,
        alt_item_id: 128_911,
        name: "Ashbringer".to_string(),
        icon: ARTIFACT_ICON.to_string(),
        total_xp: 12_500,
        points_spent: 1,
        quality: 6,
        artifact_appearance_id: 41,
        appearance_mod_id: 0,
        item_appearance_id: 0,
        alt_item_appearance_id: 0,
        alt_on_top: false,
        tier: 1,
        maxed: false,
        disabled: false,
        category: 1,
    }
}

fn sample_appearance_set() -> ArtifactAppearanceSetInfo {
    ArtifactAppearanceSetInfo {
        set_id: 10,
        name: "Base".to_string(),
        description: "Base artifact appearances".to_string(),
        num_appearances: 2,
    }
}

fn sample_appearance(appearance_id: i32, name: &str) -> ArtifactAppearanceInfo {
    ArtifactAppearanceInfo {
        set_id: 10,
        appearance_id,
        name: name.to_string(),
        display_index: appearance_id,
        unlocked: true,
        failure_description: None,
        ui_camera_id: 5,
        alt_hand_camera_id: None,
        swatch_color: ColorRgb {
            r: 0.2,
            g: 0.4,
            b: 0.6,
        },
        model_opacity: 1.0,
        model_saturation: 1.0,
        obtainable: true,
    }
}

fn load_artifact_ui(env: &wow_ui_sim::lua_api::WowLuaEnv) {
    let (loaded, error): (bool, Option<String>) = env
        .eval(r#"return C_AddOns.LoadAddOn("Blizzard_ArtifactUI")"#)
        .expect("C_AddOns.LoadAddOn probe should run cleanly");
    assert!(
        loaded,
        "`{ROOT}` must load before appearance tutorial HelpTip probe; error={error:?}"
    );
}

const APPEARANCE_TUTORIAL_HELPTIP_PROBE: &str = r#"
local mismatches = {}

local function expect(condition, message)
    if not condition then
        table.insert(mismatches, message)
    end
end

local originalPerksOnUIOpened = ArtifactFrame.PerksTab.OnUIOpened
local originalShow = HelpTip.Show
local originalHide = HelpTip.Hide
local showCalls = {}
local hideCalls = {}

ArtifactFrame.PerksTab.OnUIOpened = function() end
HelpTip.Show = function(self, parent, info, target)
    table.insert(showCalls, { parent = parent, info = info, target = target })
    return true
end
HelpTip.Hide = function(self, parent, text)
    table.insert(hideCalls, { parent = parent, text = text })
end

SetCVarBitfield("closedInfoFrames", LE_FRAME_TUTORIAL_ARTIFACT_APPEARANCE_TAB, false)
local showOk, showError = pcall(function()
    ShowUIPanel(ArtifactFrame)
end)
expect(showOk, "ShowUIPanel error:" .. tostring(showError))

showCalls = {}
hideCalls = {}
local openOk, openError = pcall(function()
    ArtifactFrame:EvaulateForgeState()
end)

SetCVarBitfield("closedInfoFrames", LE_FRAME_TUTORIAL_ARTIFACT_APPEARANCE_TAB, true)
local openBranchShowCall = showCalls[1]
showCalls = {}
hideCalls = {}
local closedOk, closedError = pcall(function()
    ArtifactFrame:EvaulateForgeState()
end)
local closedBranchHideCall = hideCalls[1]

ArtifactFrame.PerksTab.OnUIOpened = originalPerksOnUIOpened
HelpTip.Show = originalShow
HelpTip.Hide = originalHide

expect(openOk, "open EvaulateForgeState error:" .. tostring(openError))
expect(closedOk, "closed EvaulateForgeState error:" .. tostring(closedError))
expect(C_ArtifactUI.GetTotalPurchasedRanks() == 1, "purchased ranks:" .. tostring(C_ArtifactUI.GetTotalPurchasedRanks()))
expect(C_ArtifactUI.GetNumAppearanceSets() == 1, "appearance set count:" .. tostring(C_ArtifactUI.GetNumAppearanceSets()))
expect(select(4, C_ArtifactUI.GetAppearanceSetInfo(1)) == 2, "appearance slot count:" .. tostring(select(4, C_ArtifactUI.GetAppearanceSetInfo(1))))
expect(select(4, C_ArtifactUI.GetAppearanceInfo(1, 1)) == true, "first appearance should be unlocked")
expect(select(4, C_ArtifactUI.GetAppearanceInfo(1, 2)) == true, "second appearance should be unlocked")

expect(openBranchShowCall ~= nil, "open branch should show HelpTip")
expect(openBranchShowCall and openBranchShowCall.parent == ArtifactFrame, "show parent")
expect(openBranchShowCall and openBranchShowCall.target == ArtifactFrame.AppearancesTabButton, "show target")
expect(
    openBranchShowCall and openBranchShowCall.info.text == ARTIFACT_TUTORIAL_CUSTOMIZE_APPEARANCE,
    "show text:" .. tostring(openBranchShowCall and openBranchShowCall.info.text)
)
expect(
    openBranchShowCall and openBranchShowCall.info.cvarBitfield == "closedInfoFrames",
    "show cvar:" .. tostring(openBranchShowCall and openBranchShowCall.info.cvarBitfield)
)
expect(
    openBranchShowCall and openBranchShowCall.info.bitfieldFlag == LE_FRAME_TUTORIAL_ARTIFACT_APPEARANCE_TAB,
    "show bitfield:" .. tostring(openBranchShowCall and openBranchShowCall.info.bitfieldFlag)
)
expect(
    openBranchShowCall and openBranchShowCall.info.buttonStyle == HelpTip.ButtonStyle.Close,
    "show buttonStyle:" .. tostring(openBranchShowCall and openBranchShowCall.info.buttonStyle)
)
expect(
    openBranchShowCall and openBranchShowCall.info.targetPoint == HelpTip.Point.TopEdgeCenter,
    "show targetPoint:" .. tostring(openBranchShowCall and openBranchShowCall.info.targetPoint)
)
expect(openBranchShowCall and openBranchShowCall.info.offsetY == -7, "show offsetY:" .. tostring(openBranchShowCall and openBranchShowCall.info.offsetY))

expect(#showCalls == 0, "closed branch should not show HelpTip:" .. tostring(#showCalls))
expect(closedBranchHideCall ~= nil, "closed branch should hide HelpTip")
expect(closedBranchHideCall and closedBranchHideCall.parent == ArtifactFrame, "hide parent")
expect(
    closedBranchHideCall and closedBranchHideCall.text == ARTIFACT_TUTORIAL_CUSTOMIZE_APPEARANCE,
    "hide text:" .. tostring(closedBranchHideCall and closedBranchHideCall.text)
)

return mismatches
"#;
