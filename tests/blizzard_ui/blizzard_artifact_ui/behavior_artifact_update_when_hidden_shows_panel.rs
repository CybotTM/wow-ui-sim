//! Hidden-panel update behavior for `Blizzard_ArtifactUI`.

use crate::common::blizzard_addon_harness::with_blizzard_addon_smoke_shape;
use wow_ui_sim::lua_api::state::ArtifactInfo;

const ROOT: &str = "Blizzard_ArtifactUI";
const ARTIFACT_ICON: &str = "Interface/Icons/inv_sword_2h_artifactashbringer_d_01";

#[test]
fn artifact_update_when_hidden_auto_shows_outside_relic_forge() {
    with_blizzard_addon_smoke_shape(&[], &[], |env, _loaded| {
        seed_hidden_artifact_panel(env, false);
        load_artifact_ui(env);

        let mismatches: Vec<String> = env
            .eval(HIDDEN_UPDATE_AUTO_SHOW_PROBE)
            .expect("ArtifactUI hidden update auto-show probe should run cleanly");

        assert!(
            mismatches.is_empty(),
            "`{ROOT}` must auto-show from hidden ARTIFACT_UPDATE outside the relic forge; \
             mismatches: {mismatches:?}"
        );
    });
}

#[test]
fn artifact_update_when_hidden_does_not_show_inside_relic_forge() {
    with_blizzard_addon_smoke_shape(&[], &[], |env, _loaded| {
        seed_hidden_artifact_panel(env, true);
        load_artifact_ui(env);

        let mismatches: Vec<String> = env
            .eval(HIDDEN_UPDATE_RELIC_FORGE_NO_SHOW_PROBE)
            .expect("ArtifactUI hidden update relic-forge probe should run cleanly");

        assert!(
            mismatches.is_empty(),
            "`{ROOT}` must stay hidden from ARTIFACT_UPDATE inside the relic forge; \
             mismatches: {mismatches:?}"
        );
    });
}

fn seed_hidden_artifact_panel(env: &wow_ui_sim::lua_api::WowLuaEnv, relic_forge_at_forge: bool) {
    let mut state = env.state().borrow_mut();
    state.viewed_artifact.info = Some(sample_artifact());
    state.viewed_artifact.total_purchased_ranks = 3;
    state.viewed_artifact.is_at_forge = true;
    state.relic_forge_at_forge = relic_forge_at_forge;
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
        "`{ROOT}` must load before hidden update probe; error={error:?}"
    );
}

const HIDDEN_UPDATE_AUTO_SHOW_PROBE: &str = r#"
local mismatches = {}

local function expect(condition, message)
    if not condition then
        table.insert(mismatches, message)
    end
end

local showPanelCount = 0
local originalShowUIPanel = ShowUIPanel
local originalPerksOnUIOpened = ArtifactFrame.PerksTab.OnUIOpened
ShowUIPanel = function(frame, ...)
    if frame == ArtifactFrame then
        showPanelCount = showPanelCount + 1
    end
    return originalShowUIPanel(frame, ...)
end
ArtifactFrame.PerksTab.OnUIOpened = function() end

expect(not ArtifactFrame:IsShown(), "ArtifactFrame should start hidden")
expect(not C_ArtifactRelicForgeUI.IsAtForge(), "relic forge state should be false")
local ok, errorMessage = pcall(function()
    ArtifactFrame:OnEvent("ARTIFACT_UPDATE", false)
end)

ShowUIPanel = originalShowUIPanel
ArtifactFrame.PerksTab.OnUIOpened = originalPerksOnUIOpened

expect(ok, "ARTIFACT_UPDATE handler error:" .. tostring(errorMessage))
expect(showPanelCount == 1, "ShowUIPanel call count:" .. tostring(showPanelCount))
expect(ArtifactFrame:IsShown(), "ArtifactFrame should auto-show outside the relic forge")

return mismatches
"#;

const HIDDEN_UPDATE_RELIC_FORGE_NO_SHOW_PROBE: &str = r#"
local mismatches = {}

local function expect(condition, message)
    if not condition then
        table.insert(mismatches, message)
    end
end

local showPanelCount = 0
local originalShowUIPanel = ShowUIPanel
ShowUIPanel = function(frame, ...)
    if frame == ArtifactFrame then
        showPanelCount = showPanelCount + 1
    end
    return originalShowUIPanel(frame, ...)
end

expect(not ArtifactFrame:IsShown(), "ArtifactFrame should start hidden")
expect(C_ArtifactRelicForgeUI.IsAtForge(), "relic forge state should be true")
local ok, errorMessage = pcall(function()
    ArtifactFrame:OnEvent("ARTIFACT_UPDATE", false)
end)

ShowUIPanel = originalShowUIPanel

expect(ok, "ARTIFACT_UPDATE handler error:" .. tostring(errorMessage))
expect(showPanelCount == 0, "ShowUIPanel call count:" .. tostring(showPanelCount))
expect(not ArtifactFrame:IsShown(), "ArtifactFrame should stay hidden inside the relic forge")

return mismatches
"#;
