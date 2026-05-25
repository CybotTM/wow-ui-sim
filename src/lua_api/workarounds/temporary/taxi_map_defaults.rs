//! Temporary `C_TaxiMap` defaults.
//!
//! Taxi-node map data is not modeled yet. These defaults keep flight-map callers
//! on empty lists while preserving the "taxi nodes can be shown" gate.

const TAXI_MAP_DEFAULTS_LUA: &str = r#"
C_TaxiMap = C_TaxiMap or __wow_namespace()

if rawget(C_TaxiMap, "GetAllTaxiNodes") == nil then
    function C_TaxiMap.GetAllTaxiNodes(_mapID)
        return {}
    end
end

if rawget(C_TaxiMap, "GetTaxiNodesForMap") == nil then
    function C_TaxiMap.GetTaxiNodesForMap(_mapID)
        return {}
    end
end

if rawget(C_TaxiMap, "ShouldMapShowTaxiNodes") == nil then
    function C_TaxiMap.ShouldMapShowTaxiNodes(_mapID)
        return true
    end
end
"#;

pub(crate) fn apply_bootstrap(lua: &mut rilua::Lua) -> crate::Result<()> {
    lua.exec(TAXI_MAP_DEFAULTS_LUA)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::lua_api::WowLuaEnv;

    #[test]
    fn installs_empty_taxi_map_defaults() {
        let env = WowLuaEnv::new().expect("lua env should initialize");

        let result: (i32, i32, bool) = env
            .eval(
                r#"
                return #C_TaxiMap.GetAllTaxiNodes(1),
                       #C_TaxiMap.GetTaxiNodesForMap(1),
                       C_TaxiMap.ShouldMapShowTaxiNodes(1)
                "#,
            )
            .expect("taxi map defaults should be callable");

        assert_eq!(result, (0, 0, true));
    }

    #[test]
    fn preserves_existing_taxi_map_provider() {
        let env = WowLuaEnv::new().expect("lua env should initialize");
        env.exec(
            r#"
            function C_TaxiMap.GetAllTaxiNodes()
                return { "existing" }
            end
            "#,
        )
        .expect("fixture should install existing taxi map provider");

        super::apply_bootstrap(&mut env.rilua_mut()).expect("workaround should apply");

        let first_node: String = env
            .eval("local nodes = C_TaxiMap.GetAllTaxiNodes(); return nodes[1]")
            .expect("existing taxi map provider should remain callable");

        assert_eq!(first_node, "existing");
    }
}
