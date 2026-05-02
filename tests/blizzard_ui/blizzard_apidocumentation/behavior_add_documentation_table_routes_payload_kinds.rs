//! Behavior probes for `APIDocumentation:AddDocumentationTable`.

use crate::common::blizzard_addon_harness::{
    load_blizzard_addon_closure_into_env, with_blizzard_addon_startup_shape,
};
use crate::common::panel_fixtures::{
    blizzard_ui_dir, clear_recorded_lua_errors, recorded_lua_errors,
};

const ROOT: &str = "Blizzard_APIDocumentation";

#[test]
fn add_documentation_table_routes_payload_kinds() {
    with_blizzard_addon_startup_shape(&[], &[], |env, _loaded| {
        load_api_documentation(env);

        let (
            table_count,
            table_name,
            table_type,
            function_count,
            function_name,
            function_type,
            event_count,
            event_name,
            event_type,
            system_count,
            system_name,
            system_type,
        ): (
            i64,
            String,
            String,
            i64,
            String,
            String,
            i64,
            String,
            String,
            i64,
            String,
            String,
        ) = env
            .eval(
                r#"
                APIDocumentation:AddDocumentationTable({
                    Tables = {
                        { Name = "X", Type = "Structure", Fields = {} },
                    },
                })

                APIDocumentation:AddDocumentationTable({
                    Name = "DemoSystem",
                    Type = "System",
                    Namespace = "Demo",
                    Tables = {},
                    Functions = {
                        { Name = "Y", Type = "Function", Arguments = {}, Returns = {} },
                    },
                    Events = {
                        { Name = "Z", LiteralName = "z", Type = "Event", Payload = {} },
                    },
                })

                return #APIDocumentation.tables,
                       APIDocumentation.tables[1]:GetName(),
                       APIDocumentation.tables[1]:GetType(),
                       #APIDocumentation.functions,
                       APIDocumentation.functions[1]:GetName(),
                       APIDocumentation.functions[1]:GetType(),
                       #APIDocumentation.events,
                       APIDocumentation.events[1]:GetName(),
                       APIDocumentation.events[1]:GetType(),
                       #APIDocumentation.systems,
                       APIDocumentation.systems[1]:GetName(),
                       APIDocumentation.systems[1]:GetType()
                "#,
            )
            .expect("AddDocumentationTable routing probe must run cleanly");

        assert_eq!(1, table_count, "one table payload must be routed");
        assert_eq!("X", table_name, "table payload name must be preserved");
        assert_eq!(
            "table", table_type,
            "table payload must be mixed with TablesAPIMixin"
        );

        assert_eq!(1, function_count, "system function payload must be routed");
        assert_eq!(
            "Y", function_name,
            "function payload name must be preserved"
        );
        assert_eq!(
            "function", function_type,
            "function payload must be mixed with FunctionsAPIMixin"
        );

        assert_eq!(1, event_count, "system event payload must be routed");
        assert_eq!("Z", event_name, "event payload name must be preserved");
        assert_eq!(
            "event", event_type,
            "event payload must be mixed with EventsAPIMixin"
        );

        assert_eq!(
            1, system_count,
            "system payload must be appended exactly once"
        );
        assert_eq!(
            "DemoSystem", system_name,
            "system payload name must be preserved"
        );
        assert_eq!(
            "system", system_type,
            "system payload must be mixed with SystemsAPIMixin"
        );
    });
}

fn load_api_documentation(env: &wow_ui_sim::lua_api::WowLuaEnv) {
    clear_recorded_lua_errors(env);
    let ui_dir = blizzard_ui_dir();
    let loaded = load_blizzard_addon_closure_into_env(env, &ui_dir, &[ROOT], &[]);

    assert!(
        loaded.iter().any(|addon| addon == ROOT),
        "{ROOT} must be included in the loaded addon closure; loaded={loaded:?}"
    );

    let errors = recorded_lua_errors(env);
    assert!(
        errors.is_empty(),
        "{ROOT} must settle without recorded Lua errors:\n  {}",
        errors.join("\n  ")
    );
}
