//! Behavior probe for looping ArrowCalloutFrame pointer animations.

use crate::common::blizzard_addon_harness::with_blizzard_addon_smoke_shape;
use crate::common::panel_fixtures::blizzard_ui_dir;
use wow_ui_sim::loader::load_addon;

const ROOT: &str = "Blizzard_ArrowCalloutFrame";
const SETUP_LOOP_PROBE: &str = r#"
local failures = {}

local function expect(condition, message)
    if not condition then
        table.insert(failures, message)
    end
end

local anchor = CreateFrame("Frame", "AnimLoopCalloutAnchor", UIParent)
anchor:SetSize(100, 40)
anchor:SetPoint("TOPLEFT", UIParent, "TOPLEFT", 200, -200)

expect(C_ArrowCalloutManager.ShowCallout({
    calloutID = 90,
    calloutFrame = "AnimLoopCalloutAnchor",
    calloutDirection = Enum.ArrowCalloutDirection.Up,
    calloutType = Enum.ArrowCalloutType.Generic,
    calloutText = "Loop",
    offsetX = 0,
    offsetY = 0,
}), "ShowCallout should accept the animation-loop payload")

local callout = ArrowCalloutFrameManager.currentCallouts[90]
local arrowPool = callout and callout.arrowPool and callout.arrowPool:GetPool("ArrowCalloutPointerUp")
local arrow = arrowPool and arrowPool:EnumerateActive()()
local anim = arrow and arrow.Anim

expect(type(arrow) == "table", "up-arrow pool should expose the active arrow")
expect(type(anim) == "table", "up arrow should expose its Anim animation group")
expect(anim and anim:IsPlaying(), "up arrow animation should be playing after Setup")

if anim then
    local originalOnFinished = anim:GetScript("OnFinished")
    local originalPlay = anim.Play

    expect(type(originalOnFinished) == "function", "Anim should keep the XML OnFinished script")
    expect(type(originalPlay) == "function", "Anim should expose Play")

    anim.__finishCount = 0
    anim.__playCount = 0
    anim:SetScript("OnFinished", function(self, ...)
        self.__finishCount = self.__finishCount + 1
        return originalOnFinished(self, ...)
    end)
    anim.Play = function(self, ...)
        self.__playCount = self.__playCount + 1
        return originalPlay(self, ...)
    end
end

ArrowCalloutAnimLoopProbe = {
    calloutID = 90,
    anim = anim,
}

return table.concat(failures, "\n")
"#;

const AFTER_FIRST_TICK_PROBE: &str = r#"
local failures = {}
local probe = ArrowCalloutAnimLoopProbe
local anim = probe and probe.anim

if not anim then
    table.insert(failures, "animation loop probe was not initialized")
else
    if anim.__finishCount < 1 then
        table.insert(failures, "animation OnFinished should fire after one duration tick: " .. tostring(anim.__finishCount))
    end
    if anim.__playCount < 1 then
        table.insert(failures, "animation OnFinished should replay the group at least once: " .. tostring(anim.__playCount))
    end
    if not anim:IsPlaying() then
        table.insert(failures, "animation should still be playing after its OnFinished replay")
    end
end

return table.concat(failures, "\n")
"#;

const AFTER_SECOND_TICK_PROBE: &str = r#"
local failures = {}
local probe = ArrowCalloutAnimLoopProbe
local anim = probe and probe.anim

if not anim then
    table.insert(failures, "animation loop probe was not initialized")
else
    if anim.__finishCount < 2 then
        table.insert(failures, "animation should keep looping across a second duration tick: " .. tostring(anim.__finishCount))
    end
    if anim.__playCount < 2 then
        table.insert(failures, "animation should replay again across a second duration tick: " .. tostring(anim.__playCount))
    end
    if not anim:IsPlaying() then
        table.insert(failures, "animation should remain playing while the callout is visible")
    end
end

return table.concat(failures, "\n")
"#;

const HIDE_LOOP_PROBE: &str = r#"
local failures = {}
local probe = ArrowCalloutAnimLoopProbe
local anim = probe and probe.anim

if not anim then
    table.insert(failures, "animation loop probe was not initialized")
else
    probe.finishCountAfterHide = anim.__finishCount
    probe.playCountAfterHide = anim.__playCount
    C_ArrowCalloutManager.HideCallout(probe.calloutID)

    if ArrowCalloutFrameManager.currentCallouts[probe.calloutID] ~= nil then
        table.insert(failures, "hiding the callout should clear the manager entry")
    end
end

return table.concat(failures, "\n")
"#;

const AFTER_HIDE_TICK_PROBE: &str = r#"
local failures = {}
local probe = ArrowCalloutAnimLoopProbe
local anim = probe and probe.anim

if not anim then
    table.insert(failures, "animation loop probe was not initialized")
else
    if anim.__finishCount ~= probe.finishCountAfterHide then
        table.insert(failures, "hidden callout animation should stop finishing: before=" .. tostring(probe.finishCountAfterHide) .. " after=" .. tostring(anim.__finishCount))
    end
    if anim.__playCount ~= probe.playCountAfterHide then
        table.insert(failures, "hidden callout animation should stop replaying: before=" .. tostring(probe.playCountAfterHide) .. " after=" .. tostring(anim.__playCount))
    end
end

return table.concat(failures, "\n")
"#;

#[test]
fn arrow_pointer_animation_replays_until_callout_is_hidden() {
    with_blizzard_addon_smoke_shape(&[], &[], |env, _loaded| {
        load_arrow_callout_frame(env);

        assert_lua_probe(env, SETUP_LOOP_PROBE, "setup animation-loop probe");

        env.fire_on_update(1.05)
            .expect("first animation tick should run cleanly");
        assert_lua_probe(env, AFTER_FIRST_TICK_PROBE, "first animation-loop tick");

        env.fire_on_update(1.05)
            .expect("second animation tick should run cleanly");
        assert_lua_probe(env, AFTER_SECOND_TICK_PROBE, "second animation-loop tick");

        assert_lua_probe(env, HIDE_LOOP_PROBE, "hide animation-loop callout");

        env.fire_on_update(1.05)
            .expect("post-hide animation tick should run cleanly");
        assert_lua_probe(env, AFTER_HIDE_TICK_PROBE, "post-hide animation-loop tick");
    });
}

fn assert_lua_probe(env: &wow_ui_sim::lua_api::WowLuaEnv, probe: &str, label: &str) {
    let failures: String = env
        .eval(probe)
        .expect("animation-loop probe must run cleanly");
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
