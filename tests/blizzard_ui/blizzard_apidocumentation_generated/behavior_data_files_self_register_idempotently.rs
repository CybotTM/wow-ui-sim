//! Behavior probes for generated APIDocumentation redundant addon loads.

use crate::common::blizzard_addon_harness::load_blizzard_addon_closure_into_env;
use crate::common::blizzard_addon_harness::new_blizzard_addon_env;
use crate::common::panel_fixtures::{
    blizzard_ui_dir, clear_recorded_lua_errors, load_panel_addons, recorded_lua_errors,
};

const ROOT: &str = "Blizzard_APIDocumentationGenerated";

#[test]
fn generated_data_files_do_not_register_twice_after_redundant_load_addon() {
    let env = load_generated_api_documentation();

    let (
        load_ok,
        systems_before,
        systems_after,
        functions_before,
        functions_after,
        events_before,
        events_after,
        tables_before,
        tables_after,
    ): (bool, i64, i64, i64, i64, i64, i64, i64, i64) = env
        .eval(
            r#"
            local systemsBefore = #APIDocumentation.systems
            local functionsBefore = #APIDocumentation.functions
            local eventsBefore = #APIDocumentation.events
            local tablesBefore = #APIDocumentation.tables

            local loadOk = C_AddOns.LoadAddOn("Blizzard_APIDocumentationGenerated") == true

            return loadOk,
                   systemsBefore,
                   #APIDocumentation.systems,
                   functionsBefore,
                   #APIDocumentation.functions,
                   eventsBefore,
                   #APIDocumentation.events,
                   tablesBefore,
                   #APIDocumentation.tables
            "#,
        )
        .expect("generated APIDocumentation redundant LoadAddOn probe must run cleanly");

    assert!(
        load_ok,
        "redundant C_AddOns.LoadAddOn({ROOT:?}) must report success"
    );
    assert_eq!(
        systems_before, systems_after,
        "redundant LoadAddOn must not register generated systems twice"
    );
    assert_eq!(
        functions_before, functions_after,
        "redundant LoadAddOn must not register generated functions twice"
    );
    assert_eq!(
        events_before, events_after,
        "redundant LoadAddOn must not register generated events twice"
    );
    assert_eq!(
        tables_before, tables_after,
        "redundant LoadAddOn must not register generated tables twice"
    );
}

fn load_generated_api_documentation() -> wow_ui_sim::lua_api::WowLuaEnv {
    let ui_dir = blizzard_ui_dir();
    let env = new_blizzard_addon_env(&ui_dir);
    load_panel_addons(&env);
    clear_recorded_lua_errors(&env);

    let loaded = load_blizzard_addon_closure_into_env(&env, &ui_dir, &[ROOT], &[]);
    assert!(
        loaded.iter().any(|addon| addon == ROOT),
        "{ROOT} must be included in the loaded addon closure; loaded={loaded:?}"
    );

    let errors = recorded_lua_errors(&env);
    assert!(
        errors.is_empty(),
        "{ROOT} must load without recorded Lua errors:\n  {}",
        errors.join("\n  ")
    );

    env
}
