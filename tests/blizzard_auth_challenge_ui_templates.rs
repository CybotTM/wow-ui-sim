use crate::common;

use common::blizzard_addon_harness::with_blizzard_addon_smoke_shape;
use common::panel_fixtures::{clear_recorded_lua_errors, recorded_lua_errors};

const ROOT: &str = "Blizzard_AuthChallengeUI";

#[test]
fn auth_challenge_ui_registers_local_virtual_templates() {
    common::with_perf_lock(|| {
        common::with_timeout(240, || {
            with_blizzard_addon_smoke_shape(&[], &[], |env, _loaded| {
                clear_recorded_lua_errors(env);

                let (loaded, reason): (bool, Option<String>) = env
                    .eval(r#"return C_AddOns.LoadAddOn("Blizzard_AuthChallengeUI")"#)
                    .expect("C_AddOns.LoadAddOn should return");
                assert!(loaded, "`{ROOT}` should load: {reason:?}");

                for template_name in [
                    "AuthChallengeEditBoxTemplate",
                    "AuthChallengeButtonTemplate",
                ] {
                    assert!(
                        wow_ui_sim::xml::get_template(template_name).is_some(),
                        "`{ROOT}` should register local virtual template `{template_name}`"
                    );
                }

                let errors = recorded_lua_errors(env);
                assert!(
                    errors.is_empty(),
                    "`{ROOT}` template load emitted Lua errors:\n{}",
                    errors.join("\n")
                );
            });
        });
    });
}
