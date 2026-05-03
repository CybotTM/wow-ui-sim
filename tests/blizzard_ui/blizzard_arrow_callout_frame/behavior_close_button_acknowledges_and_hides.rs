//! Behavior probe for the ArrowCalloutFrame tutorial close button.

use crate::common::blizzard_addon_harness::with_blizzard_addon_smoke_shape;
use crate::common::panel_fixtures::blizzard_ui_dir;
use wow_ui_sim::loader::load_addon;

const ROOT: &str = "Blizzard_ArrowCalloutFrame";
const CLOSE_BUTTON_PROBE: &str = r#"
local failures = {}

local function expect(condition, message)
    if not condition then
        table.insert(failures, message)
    end
end

local anchor = CreateFrame("Frame", "CloseButtonCalloutAnchor", UIParent)
anchor:SetSize(100, 40)
anchor:SetPoint("TOPLEFT", UIParent, "TOPLEFT", 200, -200)

local shown = C_ArrowCalloutManager.ShowCallout({
    calloutID = 40,
    calloutFrame = "CloseButtonCalloutAnchor",
    calloutDirection = Enum.ArrowCalloutDirection.Up,
    calloutType = Enum.ArrowCalloutType.Tutorial,
    calloutText = "Closable",
    offsetX = 0,
    offsetY = 0,
})
expect(shown == true, "ShowCallout should accept the tutorial payload")

local manager = ArrowCalloutFrameManager
local pool = manager.calloutPool:GetPool("ArrowCalloutContainerTemplateWithCloseButtonTemplate")
local callout = manager.currentCallouts[40]
expect(type(callout) == "table", "tutorial callout should allocate a container")
expect(type(callout.CloseButton) == "table", "tutorial callout should expose a close button")
expect(pool:IsActive(callout), "tutorial container should start active")

callout.CloseButton:Click()
expect(C_ArrowCalloutManager.IsCalloutAcknowledged(40), "close button should acknowledge the callout")
expect(not C_ArrowCalloutManager.IsCalloutActive(40), "close button should clear C_ArrowCalloutManager active state")
expect(manager.currentCallouts[40] == nil, "close button should clear manager currentCallouts entry")
expect(not pool:IsActive(callout), "close button should release the container to the pool")
expect(pool:DoesObjectBelongToPool(callout), "released tutorial container should still belong to its pool")

return table.concat(failures, "\n")
"#;

#[test]
fn close_button_acknowledges_and_hides_tutorial_callout() {
    with_blizzard_addon_smoke_shape(&[], &[], |env, _loaded| {
        load_arrow_callout_frame(env);

        let failures: String = env
            .eval(CLOSE_BUTTON_PROBE)
            .expect("close-button behavior probe must run cleanly");
        assert!(
            failures.is_empty(),
            "ArrowCalloutFrame close-button behavior mismatches:\n{failures}"
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
