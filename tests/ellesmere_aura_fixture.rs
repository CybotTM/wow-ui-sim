//! Acceptance-owned filter regression through unchanged Blizzard/AuraKit consumers.
#![cfg(feature = "client-wowforever")]

use wow_ui_sim::lua_api::WowLuaEnv;

const FRAME_SECONDS: f64 = 0.016;
const SETTLE_FRAMES: usize = 8;

fn load_blizzard_env() -> WowLuaEnv {
    let ui = wow_ui_sim::blizzard_ui_sync::default_cache_addons_path().unwrap();
    let (env, _) = crate::common::blizzard_addon_harness::build_blizzard_addon_closure_env(
        &ui,
        &["Blizzard_AuraContainer"],
        &[],
    );
    env
}

#[cfg(feature = "ellesmere-acceptance")]
fn load_aura_kit_env() -> WowLuaEnv {
    let env = load_blizzard_env();
    let path = std::env::var("ELLESMERE_AURA_KIT_PATH")
        .expect("ELLESMERE_AURA_KIT_PATH must name the unchanged installed AuraKit Lua file");
    let source = std::fs::read_to_string(&path).expect("read installed AuraKit");
    // AuraKit owns this namespace. No addon functions or Blizzard APIs are mocked.
    env.exec("EllesmereUI = {}").unwrap();
    env.exec(&source).expect("load unchanged installed AuraKit");
    env.exec("FixtureCreateContainer = EllesmereUI.AuraKit.CreateContainer")
        .unwrap();
    env
}

fn settle(env: &WowLuaEnv) {
    for _ in 0..SETTLE_FRAMES {
        env.fire_on_update(FRAME_SECONDS).unwrap();
    }
}

fn assert_admin_aura_lifecycle(env: &WowLuaEnv) {
    env.exec(include_str!("fixtures/ellesmere_aura_fixture.lua"))
        .unwrap();
    settle(env);
    env.exec("FixtureAssertShown(FixtureArray, 0); FixtureAssertShown(FixtureKeyed, 0)")
        .unwrap();
    env.exec(
        r#"
        A_Admin.AddBuff(19750, 'CURRENT-PACKAGE helpful aura', 135907, 30, 3)
        A_Admin.AddBuff(19751, 'Excluded helpful aura', 135908, 30, 2)
        local aura = C_UnitAuras.GetPlayerAuraBySpellID(19750)
        assert(aura and aura.spellId == 19750 and aura.icon == 135907)
        assert(aura.applications == 3)
        "#,
    )
    .unwrap();
    settle(env);
    env.exec("FixtureAssertShown(FixtureArray, 0); FixtureAssertShown(FixtureKeyed, 1)")
        .unwrap();
    env.exec("A_Admin.RemoveBuff(19750)").unwrap();
    settle(env);
    env.exec(
        r#"
        FixtureAssertShown(FixtureArray, 0)
        FixtureAssertShown(FixtureKeyed, 0)
        assert(C_UnitAuras.GetPlayerAuraBySpellID(19750) == nil)
        assert(C_UnitAuras.GetPlayerAuraBySpellID(19751) ~= nil)
        "#,
    )
    .unwrap();
    assert!(
        env.state().borrow().lua_errors.is_empty(),
        "{:?}",
        env.state().borrow().lua_errors
    );
}

#[test]
fn blizzard_keyed_filter_accepts_admin_aura_and_array_rejects_it() {
    let env = load_blizzard_env();
    env.exec(include_str!("fixtures/ellesmere_blizzard_container.lua"))
        .unwrap();
    assert_admin_aura_lifecycle(&env);
}

#[cfg(feature = "ellesmere-acceptance")]
#[test]
fn ellesmere_keyed_filter_accepts_admin_aura_and_array_rejects_it() {
    assert_admin_aura_lifecycle(&load_aura_kit_env());
}
