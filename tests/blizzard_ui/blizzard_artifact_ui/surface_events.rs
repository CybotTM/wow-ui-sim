//! Event-registration probes for `Blizzard_ArtifactUI`.

use crate::common::blizzard_addon_harness::with_blizzard_addon_smoke_shape;

const ROOT: &str = "Blizzard_ArtifactUI";

#[test]
fn artifact_frame_event_lifecycle_matches_mixin_handlers() {
    with_blizzard_addon_smoke_shape(&[], &[], |env, _loaded| {
        load_artifact_ui(env);

        let mismatches: Vec<String> = env
            .eval(ARTIFACT_FRAME_EVENT_LIFECYCLE_PROBE)
            .expect("ArtifactFrame event lifecycle probe should run cleanly");

        assert!(
            mismatches.is_empty(),
            "`{ROOT}` must keep load-time events registered and show-only events scoped to \
             OnShow/OnHide; mismatches: {mismatches:?}"
        );
    });
}

fn load_artifact_ui(env: &wow_ui_sim::lua_api::WowLuaEnv) {
    let (loaded, error): (bool, Option<String>) = env
        .eval(r#"return C_AddOns.LoadAddOn("Blizzard_ArtifactUI")"#)
        .expect("C_AddOns.LoadAddOn probe should run cleanly");
    assert!(
        loaded,
        "`{ROOT}` must load before event surface probes; error={error:?}"
    );
}

const ARTIFACT_FRAME_EVENT_LIFECYCLE_PROBE: &str = r#"
local mismatches = {}

local function expect(condition, message)
    if not condition then
        table.insert(mismatches, message)
    end
end

local frame = ArtifactFrame
local loadEvents = {
    "ARTIFACT_UPDATE",
    "ARTIFACT_CLOSE",
}
local showEvents = {
    "ARTIFACT_XP_UPDATE",
    "ARTIFACT_RELIC_INFO_RECEIVED",
    "UI_SCALE_CHANGED",
    "DISPLAY_SIZE_CHANGED",
}

expect(type(frame) == "table", "ArtifactFrame:" .. type(frame))

local function expectRegistered(events, expected, phase)
    for _, event in ipairs(events) do
        local registered = frame and frame:IsEventRegistered(event) or false
        expect(
            registered == expected,
            phase .. " " .. event .. ":" .. tostring(registered)
        )
    end
end

expectRegistered(loadEvents, true, "after OnLoad")
expectRegistered(showEvents, false, "before OnShow")

local originalEvaluateForgeState = frame.EvaulateForgeState
local originalSetupPerArtifactData = frame.SetupPerArtifactData
local originalRefreshKnowledgeRanks = frame.RefreshKnowledgeRanks
local originalPerksOnUiOpened = frame.PerksTab.OnUIOpened
local originalUnderlayHide = ArtifactFrameUnderlay.Hide
local originalClear = C_ArtifactUI.Clear
local originalStaticPopupHide = StaticPopup_Hide

frame.EvaulateForgeState = function() end
frame.SetupPerArtifactData = function() end
frame.RefreshKnowledgeRanks = function() end
frame.PerksTab.OnUIOpened = function() end
ArtifactFrameUnderlay.Hide = function() end
C_ArtifactUI.Clear = function() end
StaticPopup_Hide = function() end

local ok, errorMessage = pcall(function()
    ArtifactUIMixin.OnShow(frame)
    expectRegistered(loadEvents, true, "after OnShow")
    expectRegistered(showEvents, true, "after OnShow")

    ArtifactUIMixin.OnHide(frame)
    expectRegistered(loadEvents, true, "after OnHide")
    expectRegistered(showEvents, false, "after OnHide")
end)

frame.EvaulateForgeState = originalEvaluateForgeState
frame.SetupPerArtifactData = originalSetupPerArtifactData
frame.RefreshKnowledgeRanks = originalRefreshKnowledgeRanks
frame.PerksTab.OnUIOpened = originalPerksOnUiOpened
ArtifactFrameUnderlay.Hide = originalUnderlayHide
C_ArtifactUI.Clear = originalClear
StaticPopup_Hide = originalStaticPopupHide

expect(ok, "OnShow/OnHide error:" .. tostring(errorMessage))

return mismatches
"#;
