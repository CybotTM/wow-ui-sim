//! Behavior probe for duplicate ArrowCalloutFrame callout IDs.

use crate::common::blizzard_addon_harness::with_blizzard_addon_smoke_shape;
use crate::common::panel_fixtures::blizzard_ui_dir;
use wow_ui_sim::loader::load_addon;

const ROOT: &str = "Blizzard_ArrowCalloutFrame";
const DUPLICATE_CALLOUT_PROBE: &str = r#"
local failures = {}

local function expect(condition, message)
    if not condition then
        table.insert(failures, message)
    end
end

local anchor = CreateFrame("Frame", "DuplicateCalloutAnchor", UIParent)
anchor:SetSize(100, 40)
anchor:SetPoint("TOPLEFT", UIParent, "TOPLEFT", 200, -200)

local firstShown = C_ArrowCalloutManager.ShowCallout({
    calloutID = 3,
    calloutFrame = "DuplicateCalloutAnchor",
    calloutDirection = Enum.ArrowCalloutDirection.Up,
    calloutType = Enum.ArrowCalloutType.Generic,
    calloutText = "First",
    offsetX = 0,
    offsetY = 0,
})
expect(firstShown == true, "first ShowCallout should accept the payload")

local manager = ArrowCalloutFrameManager
local firstCallout = manager.currentCallouts[3]
expect(type(firstCallout) == "table", "first call should allocate a callout frame")
expect(manager.calloutPool:GetNumActive() == 1, "first call should allocate one container")

local secondShown = C_ArrowCalloutManager.ShowCallout({
    calloutID = 3,
    calloutFrame = "DuplicateCalloutAnchor",
    calloutDirection = Enum.ArrowCalloutDirection.Down,
    calloutType = Enum.ArrowCalloutType.Generic,
    calloutText = "Second",
    offsetX = 10,
    offsetY = 10,
})
expect(secondShown == true, "second ShowCallout should still accept the state-backed payload")

local secondCallout = manager.currentCallouts[3]
expect(secondCallout == firstCallout, "duplicate callout ID should keep the original frame")
expect(manager.calloutPool:GetNumActive() == 1, "duplicate callout ID should not allocate another container")

local text = secondCallout and secondCallout.Content and secondCallout.Content.Text:GetText()
expect(text == "First", "duplicate callout ID should not rerun container setup")

return table.concat(failures, "\n")
"#;

#[test]
fn show_callout_with_duplicate_id_keeps_single_container() {
    with_blizzard_addon_smoke_shape(&[], &[], |env, _loaded| {
        load_arrow_callout_frame(env);

        let failures: String = env
            .eval(DUPLICATE_CALLOUT_PROBE)
            .expect("duplicate-callout behavior probe must run cleanly");
        assert!(
            failures.is_empty(),
            "ArrowCalloutFrame duplicate-callout behavior mismatches:\n{failures}"
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
