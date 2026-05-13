use crate::common;

use common::blizzard_addon_harness::with_blizzard_addon_smoke_shape;
use common::panel_fixtures::{clear_recorded_lua_errors, recorded_lua_errors};

const ROOT: &str = "Blizzard_AuthChallengeUI";
const SHARED_XML: &str = "Blizzard_SharedXML";

#[test]
fn blizzard_auth_challenge_ui_loads_without_ingestion_errors() {
    common::with_perf_lock(|| {
        common::with_timeout(240, || {
            with_blizzard_addon_smoke_shape(&[], &[], |env, _loaded| {
                clear_recorded_lua_errors(env);

                let (loaded, reason): (bool, Option<String>) = env
                    .eval(r#"return C_AddOns.LoadAddOn("Blizzard_AuthChallengeUI")"#)
                    .expect("C_AddOns.LoadAddOn should return");
                assert!(loaded, "`{ROOT}` should load: {reason:?}");

                let shared_xml_loaded: bool = env
                    .eval(r#"return C_AddOns.IsAddOnLoaded("Blizzard_SharedXML")"#)
                    .expect("Blizzard_SharedXML load-state probe should return");
                assert!(
                    shared_xml_loaded,
                    "`{SHARED_XML}` should be in the loaded-addons set for `{ROOT}`"
                );

                let is_loaded: bool = env
                    .eval(r#"return C_AddOns.IsAddOnLoaded("Blizzard_AuthChallengeUI")"#)
                    .expect("C_AddOns.IsAddOnLoaded should return");
                assert!(
                    is_loaded,
                    "`{ROOT}` should be reported loaded under AllowLoad: Both"
                );

                let errors = recorded_lua_errors(env);
                assert!(
                    errors.is_empty(),
                    "`{ROOT}` emitted Lua errors during load:\n{}",
                    errors.join("\n")
                );
            });
        });
    });
}
