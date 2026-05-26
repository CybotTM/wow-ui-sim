//! Temporary `C_GossipInfo` POI lookup defaults.
//!
//! Gossip option/quest/text state is modeled in Rust, but per-map gossip POI
//! data is not modeled yet. Keep the nil lookup shape explicit here until a
//! backing POI state exists.

const GOSSIP_POI_DEFAULTS_LUA: &str = r#"
C_GossipInfo = C_GossipInfo or __wow_namespace()

if rawget(C_GossipInfo, "GetPoiForUiMapID") == nil then
    function C_GossipInfo.GetPoiForUiMapID(_uiMapID)
        return nil
    end
end
"#;

pub(crate) fn apply_bootstrap(lua: &mut rilua::Lua) -> crate::Result<()> {
    lua.exec(GOSSIP_POI_DEFAULTS_LUA)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::lua_api::WowLuaEnv;

    #[test]
    fn installs_gossip_poi_default() {
        let env = WowLuaEnv::new().expect("lua env should initialize");

        let (returns, poi): (i32, Option<String>) = env
            .eval(
                r##"
                return select("#", C_GossipInfo.GetPoiForUiMapID(84)),
                    C_GossipInfo.GetPoiForUiMapID(84)
                "##,
            )
            .expect("gossip POI default should be callable");

        assert_eq!(returns, 1);
        assert_eq!(poi, None);
    }

    #[test]
    fn preserves_existing_gossip_poi_provider() {
        let env = WowLuaEnv::new().expect("lua env should initialize");
        env.exec(
            r#"
            C_GossipInfo = C_GossipInfo or __wow_namespace()

            function C_GossipInfo.GetPoiForUiMapID(uiMapID)
                return "poi:" .. tostring(uiMapID)
            end
            "#,
        )
        .expect("fixture should install existing C_GossipInfo provider");

        super::apply_bootstrap(&mut env.rilua_mut()).expect("workaround should apply");

        let poi: String = env
            .eval("return C_GossipInfo.GetPoiForUiMapID(84)")
            .expect("existing C_GossipInfo provider should remain callable");

        assert_eq!(poi, "poi:84");
    }
}
