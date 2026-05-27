//! Integration tests for `src/lua_api/globals/real/loot_method.rs`.

use wow_ui_sim::lua_api::WowLuaEnv;

fn env() -> WowLuaEnv {
    WowLuaEnv::new().expect("WowLuaEnv init")
}

// ── GetLootMethod ─────────────────────────────────────────────────────────────

#[test]
fn loot_method_globals_live_under_real_globals_boundary() {
    assert!(
        !std::path::Path::new("src/lua_api/globals/loot_method.rs").exists(),
        "loot-method globals are modeled through SimState and belong under globals::real",
    );
    assert!(
        std::path::Path::new("src/lua_api/globals/real/loot_method.rs").exists(),
        "loot-method globals should stay classified as real modeled Lua globals",
    );
}

#[test]
fn get_loot_method_defaults_to_personal_loot() {
    let env = env();
    let (method, party_idx, raid_idx): (String, i32, i32) =
        env.eval("return GetLootMethod()").unwrap();
    assert_eq!(method, "personalloot");
    assert_eq!(party_idx, 0);
    assert_eq!(raid_idx, 0);
}

#[test]
fn get_loot_method_reads_master_indices_from_state() {
    let env = env();
    {
        let mut st = env.state().borrow_mut();
        st.loot_method.method = "master".into();
        st.loot_method.party_master_index = 2;
        st.loot_method.raid_master_index = 5;
    }
    let (method, party_idx, raid_idx): (String, i32, i32) =
        env.eval("return GetLootMethod()").unwrap();
    assert_eq!(method, "master");
    assert_eq!(party_idx, 2);
    assert_eq!(raid_idx, 5);
}

// ── GetMasterLooterThreshold ──────────────────────────────────────────────────

#[test]
fn get_master_looter_threshold_defaults_to_uncommon() {
    let env = env();
    let q: i32 = env.eval("return GetMasterLooterThreshold()").unwrap();
    assert_eq!(q, 2, "retail default master-loot threshold is Uncommon");
}

#[test]
fn get_master_looter_threshold_reads_state() {
    let env = env();
    env.state().borrow_mut().loot_method.threshold = 4; // Epic
    let q: i32 = env.eval("return GetMasterLooterThreshold()").unwrap();
    assert_eq!(q, 4);
}

// ── RequestPartyLootMethod ────────────────────────────────────────────────────

#[test]
fn request_party_loot_method_fires_party_loot_method_changed() {
    let env = env();
    env.exec("RequestPartyLootMethod()").unwrap();
    let fired = env
        .state()
        .borrow()
        .events
        .pending()
        .iter()
        .any(|e| e.name == "PARTY_LOOT_METHOD_CHANGED");
    assert!(
        fired,
        "expected PARTY_LOOT_METHOD_CHANGED to land in the event queue"
    );
}
