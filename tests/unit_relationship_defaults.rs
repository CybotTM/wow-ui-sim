use wow_ui_sim::lua_api::WowLuaEnv;

#[test]
fn unit_relationship_defaults_are_registered_by_unit_owner() {
    let env = WowLuaEnv::new().expect("Failed to create Lua environment");
    let result: String = env
        .eval(
            r#"
            if UnitIsPossessed("player") then return "possessed" end
            if UnitRealmRelationship("player") ~= LE_REALM_RELATION_SAME then return "realm" end
            if UnitInPartyIsAI("player") then return "ai_party" end
            if UnitIsPVPFreeForAll("player") then return "pvp_ffa" end
            if UnitPhaseReason("player") ~= nil then return "phase" end
            return "ok"
            "#,
        )
        .unwrap();
    assert_eq!(result, "ok");
}
