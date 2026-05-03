//! Behavior probe for ArrowCalloutFrame callouts targeting missing globals.

use crate::common::blizzard_addon_harness::with_blizzard_addon_smoke_shape;
use crate::common::panel_fixtures::{
    blizzard_ui_dir, clear_recorded_lua_errors, recorded_lua_errors,
};
use wow_ui_sim::loader::load_addon;

const ROOT: &str = "Blizzard_ArrowCalloutFrame";
const UNKNOWN_ANCHOR_PROBE: &str = r#"
local failures = {}

local function expect(condition, message)
    if not condition then
        table.insert(failures, message)
    end
end

local shown = C_ArrowCalloutManager.ShowCallout({
    calloutID = 2,
    calloutFrame = "DoesNotExist",
    calloutDirection = Enum.ArrowCalloutDirection.Down,
    calloutType = Enum.ArrowCalloutType.Generic,
    calloutText = "Hidden",
    offsetX = 0,
    offsetY = 0,
})
expect(shown == true, "ShowCallout should accept the state-backed payload")

local manager = ArrowCalloutFrameManager
expect(manager.currentCallouts[2] == nil, "unknown anchor should not allocate a callout frame")
expect(manager.calloutPool:GetNumActive() == 0, "unknown anchor should leave the callout pool idle")

return table.concat(failures, "\n")
"#;

#[test]
fn show_callout_with_unknown_anchor_returns_without_lua_error() {
    with_blizzard_addon_smoke_shape(&[], &[], |env, _loaded| {
        load_arrow_callout_frame(env);
        clear_recorded_lua_errors(env);

        let failures: String = env
            .eval(UNKNOWN_ANCHOR_PROBE)
            .expect("unknown-anchor behavior probe must run cleanly");
        assert!(
            failures.is_empty(),
            "ArrowCalloutFrame unknown-anchor behavior mismatches:\n{failures}"
        );

        let errors = recorded_lua_errors(env);
        assert!(
            errors.is_empty(),
            "unknown-anchor callout path must not record Lua errors:\n{}",
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
