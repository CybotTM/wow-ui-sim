//! Behavior probe for ArrowCalloutFrame anchor replacement on reused frames.

use crate::common::blizzard_addon_harness::with_blizzard_addon_smoke_shape;
use crate::common::panel_fixtures::blizzard_ui_dir;
use wow_ui_sim::loader::load_addon;

const ROOT: &str = "Blizzard_ArrowCalloutFrame";
const ANCHOR_REUSE_PROBE: &str = r#"
local failures = {}

local function expect(condition, message)
    if not condition then
        table.insert(failures, message)
    end
end

local anchor = CreateFrame("Frame", "AnchorReuseCalloutAnchor", UIParent)
anchor:SetSize(100, 40)
anchor:SetPoint("TOPLEFT", UIParent, "TOPLEFT", 200, -200)

local function showCallout(direction, offsetX, offsetY, text)
    return C_ArrowCalloutManager.ShowCallout({
        calloutID = 50,
        calloutFrame = "AnchorReuseCalloutAnchor",
        calloutDirection = direction,
        calloutType = Enum.ArrowCalloutType.Generic,
        calloutText = text,
        offsetX = offsetX,
        offsetY = offsetY,
    })
end

expect(showCallout(Enum.ArrowCalloutDirection.Up, 0, 0, "First"), "first ShowCallout should accept payload")

local manager = ArrowCalloutFrameManager
local firstCallout = manager.currentCallouts[50]
expect(type(firstCallout) == "table", "first show should allocate a callout frame")

C_ArrowCalloutManager.HideCallout(50)
expect(manager.currentCallouts[50] == nil, "hide should clear the first callout")

expect(showCallout(Enum.ArrowCalloutDirection.Down, 7, -9, "Second"), "second ShowCallout should accept payload")
local secondCallout = manager.currentCallouts[50]
expect(secondCallout == firstCallout, "second show should reuse the released callout frame")
expect(secondCallout:GetNumPoints() == 1, "AnchorCallout should leave exactly one anchor after reuse")

local point, relativeTo, relativePoint, xOfs, yOfs = secondCallout:GetPoint(1)
expect(point == "BOTTOM", "reused callout should use the new Down anchor point")
expect(relativeTo == anchor, "reused callout should stay anchored to the named frame")
expect(relativePoint == "TOP", "reused callout should use the new Down relative point")
expect(xOfs == 7, "reused callout should use the new x offset")
expect(yOfs == -9, "reused callout should use the new y offset")

return table.concat(failures, "\n")
"#;

#[test]
fn anchor_callout_clears_previous_points_before_reanchoring_reused_frame() {
    with_blizzard_addon_smoke_shape(&[], &[], |env, _loaded| {
        load_arrow_callout_frame(env);

        let failures: String = env
            .eval(ANCHOR_REUSE_PROBE)
            .expect("anchor-reuse behavior probe must run cleanly");
        assert!(
            failures.is_empty(),
            "ArrowCalloutFrame anchor-reuse behavior mismatches:\n{failures}"
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
