//! Bare Wrath auction model probes; other profiles retain their UI fixture.
use wow_ui_sim::lua_api::WowLuaEnv;

pub fn with_model(assertions: impl FnOnce(&WowLuaEnv)) {
    if cfg!(feature = "client-wrath") {
        let env = WowLuaEnv::new().expect("auction model environment");
        assertions(&env);
    } else {
        super::blizzard_addon_harness::with_blizzard_addon_smoke_shape(&[], &[], |env, _loaded| {
            assertions(env)
        });
    }
}

pub fn load_ui_for_model(env: &WowLuaEnv) {
    if cfg!(feature = "client-wrath") {
        return;
    }
    let (loaded, reason): (bool, Option<String>) = env
        .eval(r#"return C_AddOns.LoadAddOn("Blizzard_AuctionHouseUI")"#)
        .expect("C_AddOns.LoadAddOn should return");
    assert!(loaded, "`Blizzard_AuctionHouseUI` should load: {reason:?}");
}

pub fn with_auction_ui(assertions: impl FnOnce(&WowLuaEnv)) {
    if cfg!(feature = "client-wrath") {
        let env = super::panel_fixtures::setup_env();
        assertions(&env);
    } else {
        super::blizzard_addon_harness::with_blizzard_addon_smoke_shape(&[], &[], |env, _loaded| {
            assertions(env)
        });
    }
}
