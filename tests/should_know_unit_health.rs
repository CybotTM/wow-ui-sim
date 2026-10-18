use wow_ui_sim::lua_api::WowLuaEnv;

#[cfg(any(feature = "client-era", feature = "client-anniversary"))]
#[test]
fn r20_health_knowledge_reads_supplied_per_unit_state() {
    let env = WowLuaEnv::new().unwrap();
    env.exec(
        "assert(ShouldKnowUnitHealth('player') == false); assert(ShouldKnowUnitHealth('unknown') == false)",
    )
    .unwrap();
    {
        let mut state = env.state().borrow_mut();
        state.unit_health_knowledge.insert("player".into(), true);
        state.unit_health_knowledge.insert("party1".into(), false);
        state.unit_health_knowledge.insert("target".into(), true);
    }
    env.exec(
        "assert(ShouldKnowUnitHealth('player') == true); assert(ShouldKnowUnitHealth('party1') == false); assert(ShouldKnowUnitHealth('target') == true)",
    )
    .unwrap();
    {
        let mut state = env.state().borrow_mut();
        state.unit_health_knowledge.insert("player".into(), false);
        state.unit_health_knowledge.insert("party1".into(), true);
        state.unit_health_knowledge.remove("target");
    }
    env.apply_post_load_workarounds();
    env.exec(
        "assert(ShouldKnowUnitHealth('player') == false); assert(ShouldKnowUnitHealth('party1') == true); assert(ShouldKnowUnitHealth('target') == false); assert(ShouldKnowUnitHealth('unknown') == false)",
    )
    .unwrap();
}

#[cfg(not(any(feature = "client-era", feature = "client-anniversary")))]
#[test]
fn r20_health_knowledge_does_not_expand_other_profile_surfaces() {
    let env = WowLuaEnv::new().unwrap();
    env.exec("assert(rawget(_G, 'ShouldKnowUnitHealth') == nil)")
        .unwrap();
}
