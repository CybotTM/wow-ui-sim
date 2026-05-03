//! Underlay forge-rotation behavior for `Blizzard_ArtifactUI`.

use crate::common::blizzard_addon_harness::with_blizzard_addon_smoke_shape;
use wow_ui_sim::lua_api::state::ArtifactInfo;

const ROOT: &str = "Blizzard_ArtifactUI";
const ARTIFACT_ICON: &str = "Interface/Icons/inv_sword_2h_artifactashbringer_d_01";
const CURSOR_X: f32 = 130.0;
const CURSOR_Y: f32 = 170.0;

#[test]
fn underlay_onupdate_rotates_when_idle_drags_from_cursor_and_honors_suppression() {
    with_blizzard_addon_smoke_shape(&[], &[], |env, _loaded| {
        seed_viewed_artifact_at_forge(env, false);
        env.state()
            .borrow_mut()
            .set_mouse_position(Some((CURSOR_X, CURSOR_Y)));
        load_artifact_ui(env);

        let mismatches: Vec<String> = env
            .eval(UNDERLAY_ROTATION_ACTIVE_PROBE)
            .expect("ArtifactUI underlay active-rotation probe should run cleanly");

        assert!(
            mismatches.is_empty(),
            "`{ROOT}` underlay must idle-rotate and drag from cursor deltas; \
             mismatches: {mismatches:?}"
        );

        env.state()
            .borrow_mut()
            .viewed_artifact
            .suppress_forge_rotation = true;
        let mismatches: Vec<String> = env
            .eval(UNDERLAY_ROTATION_SUPPRESSED_PROBE)
            .expect("ArtifactUI underlay suppressed-rotation probe should run cleanly");

        assert!(
            mismatches.is_empty(),
            "`{ROOT}` underlay must skip SetForgeRotation when rotation is suppressed; \
             mismatches: {mismatches:?}"
        );
    });
}

fn seed_viewed_artifact_at_forge(env: &wow_ui_sim::lua_api::WowLuaEnv, suppress_rotation: bool) {
    let mut state = env.state().borrow_mut();
    state.viewed_artifact.info = Some(sample_artifact());
    state.viewed_artifact.total_purchased_ranks = 3;
    state.viewed_artifact.is_at_forge = true;
    state.viewed_artifact.suppress_forge_rotation = suppress_rotation;
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
        "`{ROOT}` must load before underlay rotation probe; error={error:?}"
    );
}

const UNDERLAY_ROTATION_ACTIVE_PROBE: &str = r#"
local mismatches = {}

local function expect(condition, message)
    if not condition then
        table.insert(mismatches, message)
    end
end

local function expectAlmost(actual, expected, message)
    if math.abs((actual or 0) - expected) > 0.0001 then
        table.insert(mismatches, message .. ":" .. tostring(actual) .. " expected:" .. tostring(expected))
    end
end

local originalPerksOnUIOpened = ArtifactFrame.PerksTab.OnUIOpened
ArtifactFrame.PerksTab.OnUIOpened = function() end

local ok, errorMessage = pcall(function()
    ShowUIPanel(ArtifactFrame)
end)

ArtifactFrame.PerksTab.OnUIOpened = originalPerksOnUIOpened
expect(ok, "ShowUIPanel error:" .. tostring(errorMessage))
expect(ArtifactFrame:IsShown(), "ArtifactFrame should be shown")

local originalSetForgeRotation = C_ArtifactUI.SetForgeRotation
local calls = {}
C_ArtifactUI.SetForgeRotation = function(x, y, z)
    table.insert(calls, { x = x, y = y, z = z })
    return originalSetForgeRotation(x, y, z)
end

local ok, updateError = pcall(function()
    ArtifactFrameUnderlay.draggingStartX = nil
    ArtifactFrameUnderlay.draggingStartY = nil
    ArtifactFrameUnderlay.rotationOffsetX = nil
    ArtifactFrameUnderlay:OnUpdate(0.5)

    ArtifactFrameUnderlay.draggingStartX = 100
    ArtifactFrameUnderlay.draggingStartY = 200
    ArtifactFrameUnderlay.rotationOffsetX = 0.2
    ArtifactFrameUnderlay:OnUpdate(0.5)
end)

C_ArtifactUI.SetForgeRotation = originalSetForgeRotation

expect(ok, "OnUpdate error:" .. tostring(updateError))
expect(#calls == 2, "SetForgeRotation call count:" .. tostring(#calls))
expectAlmost(calls[1] and calls[1].x, 0, "idle x")
expectAlmost(calls[1] and calls[1].y, 0, "idle y")
expectAlmost(calls[1] and calls[1].z, 0.15 * 0.5, "idle z")
expectAlmost(calls[2] and calls[2].x, 0, "drag x")
expectAlmost(calls[2] and calls[2].y, 0, "drag y")
expectAlmost(calls[2] and calls[2].z, 0.2 + ((130 - 100) / UIParent:GetScale() * 0.0065), "drag z")

return mismatches
"#;

const UNDERLAY_ROTATION_SUPPRESSED_PROBE: &str = r#"
local mismatches = {}

local function expect(condition, message)
    if not condition then
        table.insert(mismatches, message)
    end
end

local originalSetForgeRotation = C_ArtifactUI.SetForgeRotation
local calls = {}
C_ArtifactUI.SetForgeRotation = function(x, y, z)
    table.insert(calls, { x = x, y = y, z = z })
end

local ok, errorMessage = pcall(function()
    ArtifactFrameUnderlay.draggingStartX = nil
    ArtifactFrameUnderlay.draggingStartY = nil
    ArtifactFrameUnderlay.rotationOffsetX = nil
    ArtifactFrameUnderlay:OnUpdate(0.5)

    ArtifactFrameUnderlay.draggingStartX = 100
    ArtifactFrameUnderlay.draggingStartY = 200
    ArtifactFrameUnderlay.rotationOffsetX = 0.2
    ArtifactFrameUnderlay:OnUpdate(0.5)
end)

C_ArtifactUI.SetForgeRotation = originalSetForgeRotation

expect(ok, "suppressed OnUpdate error:" .. tostring(errorMessage))
expect(#calls == 0, "suppressed SetForgeRotation call count:" .. tostring(#calls))

return mismatches
"#;
