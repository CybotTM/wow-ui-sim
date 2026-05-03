//! Behavior probe for showing a generic ArrowCalloutFrame callout.

use crate::common::blizzard_addon_harness::with_blizzard_addon_smoke_shape;
use crate::common::panel_fixtures::blizzard_ui_dir;
use wow_ui_sim::loader::load_addon;

const ROOT: &str = "Blizzard_ArrowCalloutFrame";
const SHOW_CALLOUT_PROBE: &str = r#"
local failures = {}

local function expect(condition, message)
    if not condition then
        table.insert(failures, message)
    end
end

local anchor = CreateFrame("Frame", "TestCalloutAnchor", UIParent)
anchor:SetSize(100, 40)
anchor:ClearAllPoints()
anchor:SetPoint("TOPLEFT", UIParent, "TOPLEFT", 200, -200)

local shown = C_ArrowCalloutManager.ShowCallout({
    calloutID = 1,
    calloutFrame = "TestCalloutAnchor",
    calloutDirection = Enum.ArrowCalloutDirection.Up,
    calloutType = Enum.ArrowCalloutType.Generic,
    calloutText = "Hello",
    offsetX = 0,
    offsetY = 0,
})
expect(shown == true, "ShowCallout should accept the seeded callout info")

local manager = ArrowCalloutFrameManager
local callout = manager and manager.currentCallouts and manager.currentCallouts[1]
expect(type(callout) == "table", "manager currentCallouts[1] should hold the acquired frame")

if callout then
    local point, relativeTo, relativePoint, xOfs, yOfs = callout:GetPoint(1)
    expect(point == "TOP", "callout anchor point")
    expect(relativeTo == anchor, "callout relative frame")
    expect(relativePoint == "BOTTOM", "callout relative point")
    expect(xOfs == 0, "callout x offset")
    expect(yOfs == 0, "callout y offset")

    local text = callout.Content and callout.Content.Text and callout.Content.Text:GetText()
    expect(text == "Hello", "callout content text")

    local arrowPool = callout.arrowPool and callout.arrowPool:GetPool("ArrowCalloutPointerUp")
    expect(type(arrowPool) == "table", "callout should expose an up-arrow pool")
    expect(arrowPool and arrowPool:GetNumActive() == 1, "up-arrow pool active count")

    local arrow = arrowPool and arrowPool:EnumerateActive()()
    expect(type(arrow) == "table", "up-arrow pool should expose the active arrow")
    expect(arrow and arrow:IsShown(), "up arrow should be shown")
    expect(arrow and arrow.Anim and arrow.Anim:IsPlaying(), "up arrow animation should be playing")
end

return table.concat(failures, "\n")
"#;

#[test]
fn show_callout_anchors_generic_container_to_named_global_frame() {
    with_blizzard_addon_smoke_shape(&[], &[], |env, _loaded| {
        load_arrow_callout_frame(env);

        let failures: String = env
            .eval(SHOW_CALLOUT_PROBE)
            .expect("show-callout behavior probe must run cleanly");
        assert!(
            failures.is_empty(),
            "ArrowCalloutFrame show-callout behavior mismatches:\n{failures}"
        );
    });
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
