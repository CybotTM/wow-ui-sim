//! Behavior probe for nil callout IDs in ArrowCalloutFrame hide logic.

use crate::common::blizzard_addon_harness::with_blizzard_addon_smoke_shape;
use crate::common::panel_fixtures::{
    blizzard_ui_dir, clear_recorded_lua_errors, recorded_lua_errors,
};
use wow_ui_sim::loader::load_addon;

const ROOT: &str = "Blizzard_ArrowCalloutFrame";
const NIL_HIDE_CALLOUT_PROBE: &str = r#"
local failures = {}

local function expect(condition, message)
    if not condition then
        table.insert(failures, message)
    end
end

local anchor = CreateFrame("Frame", "NilHideCalloutAnchor", UIParent)
anchor:SetSize(100, 40)
anchor:SetPoint("TOPLEFT", UIParent, "TOPLEFT", 200, -200)

local shown = C_ArrowCalloutManager.ShowCallout({
    calloutID = 30,
    calloutFrame = "NilHideCalloutAnchor",
    calloutDirection = Enum.ArrowCalloutDirection.Up,
    calloutType = Enum.ArrowCalloutType.Generic,
    calloutText = "Still visible",
    offsetX = 0,
    offsetY = 0,
})
expect(shown == true, "ShowCallout should accept the seeded payload")

local manager = ArrowCalloutFrameManager
local pool = manager.calloutPool:GetPool("ArrowCalloutContainerTemplate")
local originalCallout = manager.currentCallouts[30]
expect(type(originalCallout) == "table", "setup should allocate the callout before nil hide")
expect(pool:GetNumActive() == 1, "setup should leave one active container before nil hide")

manager:HideCallout(nil)
expect(manager.currentCallouts[30] == originalCallout, "nil hide should preserve currentCallouts")
expect(pool:GetNumActive() == 1, "nil hide should not release the active container")
expect(pool:IsActive(originalCallout), "nil hide should leave the container active")

return table.concat(failures, "\n")
"#;

#[test]
fn hide_callout_with_nil_id_leaves_existing_callouts_untouched() {
    with_blizzard_addon_smoke_shape(&[], &[], |env, _loaded| {
        load_arrow_callout_frame(env);
        clear_recorded_lua_errors(env);

        let failures: String = env
            .eval(NIL_HIDE_CALLOUT_PROBE)
            .expect("nil-hide behavior probe must run cleanly");
        assert!(
            failures.is_empty(),
            "ArrowCalloutFrame nil-hide behavior mismatches:\n{failures}"
        );

        let errors = recorded_lua_errors(env);
        assert!(
            errors.is_empty(),
            "nil-hide callout path must not record Lua errors:\n{}",
            errors.join("\n")
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
