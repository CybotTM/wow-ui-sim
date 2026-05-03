//! Tab switching behavior for `Blizzard_ArtifactUI`.

use crate::common::blizzard_addon_harness::with_blizzard_addon_smoke_shape;
use wow_ui_sim::lua_api::state::ArtifactInfo;

const ROOT: &str = "Blizzard_ArtifactUI";
const ARTIFACT_ICON: &str = "Interface/Icons/inv_sword_2h_artifactashbringer_d_01";

#[test]
fn set_tab_to_appearances_resizes_panel_and_closes_tutorial_gate() {
    with_blizzard_addon_smoke_shape(&[], &[], |env, _loaded| {
        seed_viewed_artifact_at_forge(env);
        load_artifact_ui(env);

        let mismatches: Vec<String> = env
            .eval(SET_APPEARANCES_TAB_PROBE)
            .expect("ArtifactUI appearance-tab probe should run cleanly");

        assert!(
            mismatches.is_empty(),
            "`{ROOT}` must resize and switch visible tabs when TAB_APPEARANCE is selected; \
             mismatches: {mismatches:?}"
        );
    });
}

fn seed_viewed_artifact_at_forge(env: &wow_ui_sim::lua_api::WowLuaEnv) {
    let mut state = env.state().borrow_mut();
    state.viewed_artifact.info = Some(sample_artifact());
    state.viewed_artifact.total_purchased_ranks = 3;
    state.viewed_artifact.is_at_forge = true;
}

fn sample_artifact() -> ArtifactInfo {
    ArtifactInfo {
        item_id: 128_910,
        alt_item_id: 128_911,
        name: "Ashbringer".to_string(),
        icon: ARTIFACT_ICON.to_string(),
        total_xp: 12_500,
        points_spent: 3,
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

fn load_artifact_ui(env: &wow_ui_sim::lua_api::WowLuaEnv) {
    let (loaded, error): (bool, Option<String>) = env
        .eval(r#"return C_AddOns.LoadAddOn("Blizzard_ArtifactUI")"#)
        .expect("C_AddOns.LoadAddOn probe should run cleanly");
    assert!(
        loaded,
        "`{ROOT}` must load before appearance-tab probe; error={error:?}"
    );
}

const SET_APPEARANCES_TAB_PROBE: &str = r#"
local mismatches = {}

local function expect(condition, message)
    if not condition then
        table.insert(mismatches, message)
    end
end

local originalPerksOnUIOpened = ArtifactFrame.PerksTab.OnUIOpened
ArtifactFrame.PerksTab.OnUIOpened = function() end

local showOk, showError = pcall(function()
    ShowUIPanel(ArtifactFrame)
end)
expect(showOk, "ShowUIPanel error:" .. tostring(showError))

local setTabOk, setTabError = pcall(function()
    ArtifactFrame:SetTab(2)
end)
ArtifactFrame.PerksTab.OnUIOpened = originalPerksOnUIOpened

expect(setTabOk, "SetTab error:" .. tostring(setTabError))
expect(ArtifactFrame:GetWidth() == 460, "ArtifactFrame width:" .. tostring(ArtifactFrame:GetWidth()))
expect(not ArtifactFrame.PerksTab:IsShown(), "PerksTab should be hidden")
expect(ArtifactFrame.AppearancesTab:IsShown(), "AppearancesTab should be shown")
expect(PanelTemplates_GetSelectedTab(ArtifactFrame) == 2, "selected tab:" .. tostring(PanelTemplates_GetSelectedTab(ArtifactFrame)))
expect(
    GetCVarBitfield("closedInfoFrames", LE_FRAME_TUTORIAL_ARTIFACT_APPEARANCE_TAB) == true,
    "appearance tutorial bitfield should be closed"
)

return mismatches
"#;
