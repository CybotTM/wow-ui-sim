//! Behavior probe for the currently-unused PLAYER_SOFT_INTERACT_CHANGED event.

use crate::common::blizzard_addon_harness::with_blizzard_addon_smoke_shape;
use crate::common::panel_fixtures::{
    blizzard_ui_dir, clear_recorded_lua_errors, recorded_lua_errors,
};
use wow_ui_sim::loader::load_addon;

const ROOT: &str = "Blizzard_ArrowCalloutFrame";
const SETUP_SOFT_INTERACT_PROBE: &str = r#"
local failures = {}

local function expect(condition, message)
    if not condition then
        table.insert(failures, message)
    end
end

local function countCallouts(callouts)
    local count = 0
    for _ in pairs(callouts) do
        count = count + 1
    end
    return count
end

local anchor = CreateFrame("Frame", "SoftInteractCalloutAnchor", UIParent)
anchor:SetSize(100, 40)
anchor:SetPoint("TOPLEFT", UIParent, "TOPLEFT", 200, -200)

expect(C_ArrowCalloutManager.ShowCallout({
    calloutID = 91,
    calloutFrame = "SoftInteractCalloutAnchor",
    calloutDirection = Enum.ArrowCalloutDirection.Up,
    calloutType = Enum.ArrowCalloutType.Generic,
    calloutText = "Soft interact should not change this",
    offsetX = 0,
    offsetY = 0,
}), "ShowCallout should accept the soft-interact payload")

local manager = ArrowCalloutFrameManager
local pool = manager and manager.calloutPool and manager.calloutPool:GetPool("ArrowCalloutContainerTemplate")
local callout = manager and manager.currentCallouts and manager.currentCallouts[91]

expect(type(callout) == "table", "setup should allocate the callout before soft-interact event")
expect(pool and pool:GetNumActive() == 1, "setup should leave one active generic container")

ArrowCalloutSoftInteractProbe = {
    callout = callout,
    calloutCount = manager and countCallouts(manager.currentCallouts) or -1,
    activeCount = pool and pool:GetNumActive() or -1,
}

return table.concat(failures, "\n")
"#;

const AFTER_SOFT_INTERACT_PROBE: &str = r#"
local failures = {}

local function expect(condition, message)
    if not condition then
        table.insert(failures, message)
    end
end

local function countCallouts(callouts)
    local count = 0
    for _ in pairs(callouts) do
        count = count + 1
    end
    return count
end

local manager = ArrowCalloutFrameManager
local pool = manager and manager.calloutPool and manager.calloutPool:GetPool("ArrowCalloutContainerTemplate")
local probe = ArrowCalloutSoftInteractProbe

expect(type(probe) == "table", "soft-interact probe should be initialized")
if probe then
    expect(manager.currentCallouts[91] == probe.callout, "soft-interact event should preserve the active callout")
    expect(countCallouts(manager.currentCallouts) == probe.calloutCount, "soft-interact event should not add or remove currentCallouts entries")
    expect(pool:GetNumActive() == probe.activeCount, "soft-interact event should not release or allocate generic containers")
    expect(pool:IsActive(probe.callout), "soft-interact event should leave the original container active")
end

return table.concat(failures, "\n")
"#;

#[test]
fn player_soft_interact_changed_event_is_registered_but_currently_noop() {
    with_blizzard_addon_smoke_shape(&[], &[], |env, _loaded| {
        load_arrow_callout_frame(env);
        clear_recorded_lua_errors(env);

        assert_lua_probe(env, SETUP_SOFT_INTERACT_PROBE, "setup soft-interact probe");

        env.fire_event("PLAYER_SOFT_INTERACT_CHANGED")
            .expect("soft-interact event should dispatch cleanly");

        assert_lua_probe(env, AFTER_SOFT_INTERACT_PROBE, "post soft-interact probe");

        let errors = recorded_lua_errors(env);
        assert!(
            errors.is_empty(),
            "soft-interact event path must not record Lua errors:\n{}",
            errors.join("\n")
        );
    });
}

fn assert_lua_probe(env: &wow_ui_sim::lua_api::WowLuaEnv, probe: &str, label: &str) {
    let failures: String = env
        .eval(probe)
        .expect("soft-interact probe must run cleanly");
    assert!(
        failures.is_empty(),
        "ArrowCalloutFrame {label} mismatches:\n{failures}"
    );
}

fn load_arrow_callout_frame(env: &wow_ui_sim::lua_api::WowLuaEnv) {
    load_addon(&env.loader_env(), &arrow_callout_toc())
        .expect("Blizzard_ArrowCalloutFrame should load directly from its TOC");
}

fn arrow_callout_toc() -> std::path::PathBuf {
    blizzard_ui_dir()
        .join(ROOT)
        .join("Blizzard_ArrowCalloutFrame.toc")
}
