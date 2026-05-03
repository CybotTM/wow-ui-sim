//! Behavior probe for ArrowCalloutFrame callout type to pool selection.

use crate::common::blizzard_addon_harness::with_blizzard_addon_smoke_shape;
use crate::common::panel_fixtures::blizzard_ui_dir;
use wow_ui_sim::loader::load_addon;

const ROOT: &str = "Blizzard_ArrowCalloutFrame";
const POOL_BY_TYPE_PROBE: &str = r#"
local failures = {}

local function expect(condition, message)
    if not condition then
        table.insert(failures, message)
    end
end

local function makeAnchor(name, x)
    local anchor = CreateFrame("Frame", name, UIParent)
    anchor:SetSize(100, 40)
    anchor:SetPoint("TOPLEFT", UIParent, "TOPLEFT", x, -200)
    return anchor
end

makeAnchor("GenericCalloutAnchor", 100)
makeAnchor("TutorialCalloutAnchor", 250)
makeAnchor("WidgetCalloutAnchor", 400)

local function showCallout(calloutID, anchorName, calloutType, text)
    return C_ArrowCalloutManager.ShowCallout({
        calloutID = calloutID,
        calloutFrame = anchorName,
        calloutDirection = Enum.ArrowCalloutDirection.Up,
        calloutType = calloutType,
        calloutText = text,
        offsetX = 0,
        offsetY = 0,
    })
end

expect(showCallout(10, "GenericCalloutAnchor", Enum.ArrowCalloutType.Generic, "Generic"), "generic callout should show")
expect(showCallout(11, "TutorialCalloutAnchor", Enum.ArrowCalloutType.Tutorial, "Tutorial"), "tutorial callout should show")
expect(showCallout(12, "WidgetCalloutAnchor", Enum.ArrowCalloutType.WidgetContainerNoBorder, "Widget"), "widget callout should show")

local manager = ArrowCalloutFrameManager
local generic = manager.currentCallouts[10]
local tutorial = manager.currentCallouts[11]
local widget = manager.currentCallouts[12]

local genericPool = manager.calloutPool:GetPool("ArrowCalloutContainerTemplate")
local tutorialPool = manager.calloutPool:GetPool("ArrowCalloutContainerTemplateWithCloseButtonTemplate")
local widgetPool = manager.calloutPool:GetPool("WidgetContainerCalloutTemplate")

expect(genericPool:IsActive(generic), "Generic type should use ArrowCalloutContainerTemplate")
expect(tutorialPool:IsActive(tutorial), "Tutorial type should use ArrowCalloutContainerTemplateWithCloseButtonTemplate")
expect(widgetPool:IsActive(widget), "WidgetContainerNoBorder type should use WidgetContainerCalloutTemplate")
expect(genericPool:GetNumActive() == 1, "generic pool active count")
expect(tutorialPool:GetNumActive() == 1, "tutorial pool active count")
expect(widgetPool:GetNumActive() == 1, "widget pool active count")

expect(generic.CloseButton == nil, "generic callouts should not expose a CloseButton")
expect(type(tutorial.CloseButton) == "table", "tutorial callouts should expose a CloseButton")
expect(tutorial.CloseButton:GetParent() == tutorial, "tutorial CloseButton should be parented to its callout")

return table.concat(failures, "\n")
"#;

#[test]
fn show_callout_selects_container_pool_from_callout_type() {
    with_blizzard_addon_smoke_shape(&[], &[], |env, _loaded| {
        load_arrow_callout_frame(env);

        let failures: String = env
            .eval(POOL_BY_TYPE_PROBE)
            .expect("pool-by-type behavior probe must run cleanly");
        assert!(
            failures.is_empty(),
            "ArrowCalloutFrame pool-by-type behavior mismatches:\n{failures}"
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
