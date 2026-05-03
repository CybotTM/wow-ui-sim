//! Close-event behavior for `Blizzard_ArtifactUI`.

use crate::common::blizzard_addon_harness::with_blizzard_addon_smoke_shape;
use wow_ui_sim::lua_api::state::ArtifactInfo;

const ROOT: &str = "Blizzard_ArtifactUI";
const ARTIFACT_ICON: &str = "Interface/Icons/inv_sword_2h_artifactashbringer_d_01";

#[test]
fn artifact_close_event_hides_panel_and_clears_viewed_artifact() {
    with_blizzard_addon_smoke_shape(&[], &[], |env, _loaded| {
        seed_viewed_artifact_at_forge(env);
        load_artifact_ui(env);

        let mismatches: Vec<String> = env
            .eval(ARTIFACT_CLOSE_EVENT_PROBE)
            .expect("ArtifactUI close-event probe should run cleanly");

        assert!(
            mismatches.is_empty(),
            "`{ROOT}` must hide when ARTIFACT_CLOSE fires; mismatches: {mismatches:?}"
        );
        assert!(
            env.state().borrow().viewed_artifact.info.is_none(),
            "`{ROOT}` OnHide must clear the simulator's viewed artifact state"
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
        "`{ROOT}` must load before close-event probe; error={error:?}"
    );
}

const ARTIFACT_CLOSE_EVENT_PROBE: &str = r#"
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
expect(ArtifactFrame:IsShown(), "ArtifactFrame should be shown before ARTIFACT_CLOSE")

local closeOk, closeError = pcall(function()
    FireEvent("ARTIFACT_CLOSE")
end)
ArtifactFrame.PerksTab.OnUIOpened = originalPerksOnUIOpened

expect(closeOk, "ARTIFACT_CLOSE dispatch error:" .. tostring(closeError))
expect(not ArtifactFrame:IsShown(), "ArtifactFrame should be hidden after ARTIFACT_CLOSE")

return mismatches
"#;
