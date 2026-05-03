//! Behavior probe for ArrowCalloutFrame acknowledged-callout cvar persistence.

use crate::common::blizzard_addon_harness::new_blizzard_addon_env;
use crate::common::panel_fixtures::{
    blizzard_ui_dir, clear_recorded_lua_errors, load_panel_addons, recorded_lua_errors,
};
use wow_ui_sim::loader::load_addon;

const ROOT: &str = "Blizzard_ArrowCalloutFrame";
const ACKNOWLEDGE_CALLOUTS_PROBE: &str = r#"
C_ArrowCalloutManager.AcknowledgeCallout(7)
C_ArrowCalloutManager.AcknowledgeCallout(13)
C_ArrowCalloutManager.AcknowledgeCallout(99)

return C_ArrowCalloutManager.IsCalloutAcknowledged(7),
       C_ArrowCalloutManager.IsCalloutAcknowledged(13),
       C_ArrowCalloutManager.IsCalloutAcknowledged(99),
       GetCVar("acknowledgedArrowCallouts")
"#;
const RELOAD_ACKNOWLEDGED_CALLOUTS_PROBE: &str = r#"
local acknowledgedBeforeRewrite = {
    C_ArrowCalloutManager.IsCalloutAcknowledged(7),
    C_ArrowCalloutManager.IsCalloutAcknowledged(13),
    C_ArrowCalloutManager.IsCalloutAcknowledged(99),
}

C_ArrowCalloutManager.AcknowledgeCallout(7)

return acknowledgedBeforeRewrite[1],
       acknowledgedBeforeRewrite[2],
       acknowledgedBeforeRewrite[3],
       GetCVar("acknowledgedArrowCallouts")
"#;

#[test]
fn acknowledged_callouts_persist_through_cvar_across_fresh_load() {
    let persisted_cvar = {
        let env = load_arrow_callout_env(None);
        let (ack7, ack13, ack99, cvar): (bool, bool, bool, String) = env
            .eval(ACKNOWLEDGE_CALLOUTS_PROBE)
            .expect("acknowledging callouts should run cleanly");

        assert_acknowledged_triplet((ack7, ack13, ack99), "initial load");
        assert_eq!(
            cvar, "7,13,99",
            "acknowledgedArrowCallouts should serialize the acknowledged set in sorted order"
        );

        cvar
    };

    let env = load_arrow_callout_env(Some(&persisted_cvar));
    let (ack7, ack13, ack99, rewritten_cvar): (bool, bool, bool, String) = env
        .eval(RELOAD_ACKNOWLEDGED_CALLOUTS_PROBE)
        .expect("reloaded acknowledged callout probe should run cleanly");

    assert_acknowledged_triplet((ack7, ack13, ack99), "fresh load from cvar");
    assert_eq!(
        rewritten_cvar, persisted_cvar,
        "acknowledgedArrowCallouts should parse and serialize back to the same set"
    );
}

fn assert_acknowledged_triplet(values: (bool, bool, bool), context: &str) {
    assert!(values.0, "{context}: callout 7 should be acknowledged");
    assert!(values.1, "{context}: callout 13 should be acknowledged");
    assert!(values.2, "{context}: callout 99 should be acknowledged");
}

fn load_arrow_callout_env(acknowledged_cvar: Option<&str>) -> wow_ui_sim::lua_api::WowLuaEnv {
    let ui_dir = blizzard_ui_dir();
    let env = new_blizzard_addon_env(&ui_dir);

    if let Some(value) = acknowledged_cvar {
        env.state()
            .borrow_mut()
            .cvars
            .set("acknowledgedArrowCallouts", value);
    }

    load_panel_addons(&env);
    clear_recorded_lua_errors(&env);
    load_addon(&env.loader_env(), &arrow_callout_toc())
        .expect("Blizzard_ArrowCalloutFrame should load directly from its TOC");

    let errors = recorded_lua_errors(&env);
    assert!(
        errors.is_empty(),
        "{ROOT} should load without recorded Lua errors:\n  {}",
        errors.join("\n  ")
    );

    env
}

fn arrow_callout_toc() -> std::path::PathBuf {
    blizzard_ui_dir()
        .join(ROOT)
        .join("Blizzard_ArrowCalloutFrame.toc")
}
