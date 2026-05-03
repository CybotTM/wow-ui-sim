//! Behavior probe for ArrowCalloutFrame direction-to-anchor mappings.

use crate::common::blizzard_addon_harness::with_blizzard_addon_smoke_shape;
use crate::common::panel_fixtures::blizzard_ui_dir;
use wow_ui_sim::loader::load_addon;

const ROOT: &str = "Blizzard_ArrowCalloutFrame";
const DIRECTION_ANCHORS_PROBE: &str = r#"
local failures = {}

local function expect(condition, message)
    if not condition then
        table.insert(failures, message)
    end
end

local anchor = CreateFrame("Frame", "DirectionCalloutAnchor", UIParent)
anchor:SetSize(100, 40)
anchor:SetPoint("TOPLEFT", UIParent, "TOPLEFT", 200, -200)

local expectations = {
    {
        id = 61,
        name = "Up",
        direction = Enum.ArrowCalloutDirection.Up,
        point = "TOP",
        relativePoint = "BOTTOM",
        arrowTemplate = "ArrowCalloutPointerUp",
    },
    {
        id = 62,
        name = "Down",
        direction = Enum.ArrowCalloutDirection.Down,
        point = "BOTTOM",
        relativePoint = "TOP",
        arrowTemplate = "ArrowCalloutPointerDown",
    },
    {
        id = 63,
        name = "Left",
        direction = Enum.ArrowCalloutDirection.Left,
        point = "LEFT",
        relativePoint = "RIGHT",
        arrowTemplate = "ArrowCalloutPointerLeft",
    },
    {
        id = 64,
        name = "Right",
        direction = Enum.ArrowCalloutDirection.Right,
        point = "RIGHT",
        relativePoint = "LEFT",
        arrowTemplate = "ArrowCalloutPointerRight",
    },
}

local allArrowTemplates = {
    "ArrowCalloutPointerUp",
    "ArrowCalloutPointerDown",
    "ArrowCalloutPointerLeft",
    "ArrowCalloutPointerRight",
}

local function showCallout(expectation)
    return C_ArrowCalloutManager.ShowCallout({
        calloutID = expectation.id,
        calloutFrame = "DirectionCalloutAnchor",
        calloutDirection = expectation.direction,
        calloutType = Enum.ArrowCalloutType.Generic,
        calloutText = expectation.name,
        offsetX = 0,
        offsetY = 0,
    })
end

local function expectAnchor(callout, expectation)
    expect(callout:GetNumPoints() == 1, expectation.name .. " callout should have one anchor")

    local point, relativeTo, relativePoint, xOfs, yOfs = callout:GetPoint(1)
    expect(point == expectation.point, expectation.name .. " callout point")
    expect(relativeTo == anchor, expectation.name .. " callout relative frame")
    expect(relativePoint == expectation.relativePoint, expectation.name .. " callout relative point")
    expect(xOfs == 0, expectation.name .. " callout x offset")
    expect(yOfs == 0, expectation.name .. " callout y offset")
end

local function expectArrowTemplate(callout, expectation)
    for _, template in ipairs(allArrowTemplates) do
        local pool = callout.arrowPool:GetPool(template)
        local expectedActiveCount = template == expectation.arrowTemplate and 1 or 0
        expect(
            pool:GetNumActive() == expectedActiveCount,
            expectation.name .. " should activate only " .. expectation.arrowTemplate
        )
    end
end

for _, expectation in ipairs(expectations) do
    expect(showCallout(expectation), expectation.name .. " ShowCallout should accept payload")

    local callout = ArrowCalloutFrameManager.currentCallouts[expectation.id]
    expect(type(callout) == "table", expectation.name .. " callout should allocate a frame")

    if callout then
        expectAnchor(callout, expectation)
        expectArrowTemplate(callout, expectation)
    end
end

return table.concat(failures, "\n")
"#;

#[test]
fn anchor_directions_use_documented_anchor_points_and_arrow_templates() {
    with_blizzard_addon_smoke_shape(&[], &[], |env, _loaded| {
        load_arrow_callout_frame(env);

        let failures: String = env
            .eval(DIRECTION_ANCHORS_PROBE)
            .expect("direction-anchor behavior probe must run cleanly");
        assert!(
            failures.is_empty(),
            "ArrowCalloutFrame direction-anchor behavior mismatches:\n{failures}"
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
