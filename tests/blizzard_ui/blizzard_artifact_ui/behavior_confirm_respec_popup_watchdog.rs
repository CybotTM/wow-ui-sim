//! Respec popup behavior for `Blizzard_ArtifactUI`.

use crate::common::blizzard_addon_harness::with_blizzard_addon_smoke_shape;
use wow_ui_sim::lua_api::state::ArtifactInfo;

const ROOT: &str = "Blizzard_ArtifactUI";
const ARTIFACT_ICON: &str = "Interface/Icons/inv_sword_2h_artifactashbringer_d_01";

#[test]
fn confirm_respec_popup_watchdog_hides_popup_and_panel_when_npc_disappears() {
    with_blizzard_addon_smoke_shape(&[], &[], |env, _loaded| {
        load_artifact_ui(env);
        seed_respec_artifact(env, true);

        let mismatches: Vec<String> = env
            .eval(RESPEC_PANEL_OPEN_PROBE)
            .expect("ArtifactUI respec panel open probe should run cleanly");
        assert!(
            mismatches.is_empty(),
            "`{ROOT}` respec panel setup must show the panel; mismatches: {mismatches:?}"
        );

        seed_respec_artifact(env, true);

        let mismatches: Vec<String> = env
            .eval(RESPEC_WATCHDOG_SETUP_AND_ACTIVE_TICK_PROBE)
            .expect("ArtifactUI active respec watchdog probe should run cleanly");
        assert!(
            mismatches.is_empty(),
            "`{ROOT}` respec popup must stay shown while NPC access remains active; \
             mismatches: {mismatches:?}"
        );

        env.state().borrow_mut().viewed_artifact.respec_npc_active = false;

        let mismatches: Vec<String> = env
            .eval(RESPEC_WATCHDOG_INACTIVE_TICK_PROBE)
            .expect("ArtifactUI inactive respec watchdog probe should run cleanly");

        assert!(
            mismatches.is_empty(),
            "`{ROOT}` respec popup watchdog must hide popup and panel when NPC access ends; \
             mismatches: {mismatches:?}"
        );
        assert!(
            env.state().borrow().viewed_artifact.info.is_none(),
            "`{ROOT}` watchdog HideUIPanel path must clear the viewed artifact"
        );
    });
}

#[test]
fn confirm_respec_popup_accept_confirms_respec_before_hiding_panel() {
    with_blizzard_addon_smoke_shape(&[], &[], |env, _loaded| {
        load_artifact_ui(env);
        seed_respec_artifact(env, true);

        let mismatches: Vec<String> = env
            .eval(RESPEC_ACCEPT_PROBE)
            .expect("ArtifactUI respec accept probe should run cleanly");

        assert!(
            mismatches.is_empty(),
            "`{ROOT}` respec accept path must call ConfirmRespec and HideUIPanel; \
             mismatches: {mismatches:?}"
        );
        assert_eq!(
            env.state().borrow().viewed_artifact.total_purchased_ranks,
            0,
            "`{ROOT}` ConfirmRespec must zero purchased artifact ranks"
        );
    });
}

fn seed_respec_artifact(env: &wow_ui_sim::lua_api::WowLuaEnv, respec_npc_active: bool) {
    let mut state = env.state().borrow_mut();
    state.viewed_artifact.info = Some(sample_artifact());
    state.viewed_artifact.total_purchased_ranks = 3;
    state.viewed_artifact.points_remaining = 1;
    state.viewed_artifact.is_at_forge = true;
    state.viewed_artifact.respec_npc_active = respec_npc_active;
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
        "`{ROOT}` must load before respec popup probe; error={error:?}"
    );
}

const RESPEC_PANEL_OPEN_PROBE: &str = r#"
local mismatches = {}

local function expect(condition, message)
    if not condition then
        table.insert(mismatches, message)
    end
end

local originalPerksOnUIOpened = ArtifactFrame.PerksTab.OnUIOpened
ArtifactFrame.PerksTab.OnUIOpened = function() end

local ok, errorMessage = pcall(function()
    ShowUIPanel(ArtifactFrame)
end)

ArtifactFrame.PerksTab.OnUIOpened = originalPerksOnUIOpened

expect(ok, "ShowUIPanel error:" .. tostring(errorMessage))
expect(ArtifactFrame:IsShown(), "ArtifactFrame should be shown before popup watchdog")

return mismatches
"#;

const RESPEC_WATCHDOG_SETUP_AND_ACTIVE_TICK_PROBE: &str = r#"
local mismatches = {}

local function expect(condition, message)
    if not condition then
        table.insert(mismatches, message)
    end
end

__artifactRespecWatchdogProbe = {
    popupShown = false,
    hiddenPopup = nil,
}
local originalStaticPopupShow = StaticPopup_Show
local originalStaticPopupHide = StaticPopup_Hide
__artifactRespecWatchdogProbe.originalStaticPopupShow = originalStaticPopupShow
__artifactRespecWatchdogProbe.originalStaticPopupHide = originalStaticPopupHide

StaticPopup_Show = function(which, ...)
    if which == "CONFIRM_ARTIFACT_RESPEC" then
        __artifactRespecWatchdogProbe.popupShown = true
    end
    return StaticPopupDialogs[which]
end
StaticPopup_Hide = function(which, ...)
    if which == "CONFIRM_ARTIFACT_RESPEC" then
        __artifactRespecWatchdogProbe.popupShown = false
        __artifactRespecWatchdogProbe.hiddenPopup = which
    end
    return nil
end

local ok, errorMessage = pcall(function()
    expect(C_ArtifactUI.CheckRespecNPC(), "CheckRespecNPC should be true before active tick")
    StaticPopup_Show("CONFIRM_ARTIFACT_RESPEC")
    StaticPopupDialogs["CONFIRM_ARTIFACT_RESPEC"].OnUpdate({}, 0.016)
end)

expect(ok, "active respec watchdog error:" .. tostring(errorMessage))
expect(__artifactRespecWatchdogProbe.popupShown, "popup should remain shown while NPC is active")
expect(__artifactRespecWatchdogProbe.hiddenPopup == nil, "hidden popup:" .. tostring(__artifactRespecWatchdogProbe.hiddenPopup))
expect(ArtifactFrame:IsShown(), "ArtifactFrame should remain shown while NPC is active")

return mismatches
"#;

const RESPEC_WATCHDOG_INACTIVE_TICK_PROBE: &str = r#"
local mismatches = {}

local function expect(condition, message)
    if not condition then
        table.insert(mismatches, message)
    end
end

local ok, errorMessage = pcall(function()
    StaticPopupDialogs["CONFIRM_ARTIFACT_RESPEC"].OnUpdate({}, 0.016)
end)

StaticPopup_Show = __artifactRespecWatchdogProbe.originalStaticPopupShow
StaticPopup_Hide = __artifactRespecWatchdogProbe.originalStaticPopupHide

expect(ok, "inactive respec watchdog error:" .. tostring(errorMessage))
expect(
    __artifactRespecWatchdogProbe.hiddenPopup == "CONFIRM_ARTIFACT_RESPEC",
    "hidden popup:" .. tostring(__artifactRespecWatchdogProbe.hiddenPopup)
)
expect(not __artifactRespecWatchdogProbe.popupShown, "popup should be hidden after inactive NPC")
expect(not ArtifactFrame:IsShown(), "ArtifactFrame should hide after inactive NPC")

return mismatches
"#;

const RESPEC_ACCEPT_PROBE: &str = r#"
local mismatches = {}

local function expect(condition, message)
    if not condition then
        table.insert(mismatches, message)
    end
end

local hidePanelCount = 0
local originalHideUIPanel = HideUIPanel
HideUIPanel = function(frame, ...)
    if frame == ArtifactFrame then
        hidePanelCount = hidePanelCount + 1
        return nil
    end
    return originalHideUIPanel(frame, ...)
end

local ok, errorMessage = pcall(function()
    StaticPopupDialogs["CONFIRM_ARTIFACT_RESPEC"].OnAccept({}, nil)
end)

HideUIPanel = originalHideUIPanel

expect(ok, "respec accept error:" .. tostring(errorMessage))
expect(hidePanelCount == 1, "HideUIPanel call count:" .. tostring(hidePanelCount))
expect(C_ArtifactUI.GetTotalPurchasedRanks() == 0, "purchased ranks:" .. tostring(C_ArtifactUI.GetTotalPurchasedRanks()))

return mismatches
"#;
