#![cfg(feature = "client-ptr")]

use crate::common::blizzard_addon_harness::{
    blizzard_ui_dir, load_blizzard_addon_closure_into_env, new_blizzard_addon_env,
};

fn assert_consumer_loaded(env: &wow_ui_sim::lua_api::WowLuaEnv, source: &str) {
    let state = env.state().borrow();
    let errors: Vec<_> = state
        .lua_errors
        .iter()
        .filter(|error| error.contains(source))
        .collect();
    assert!(errors.is_empty(), "original consumer failed: {errors:#?}");
}

#[test]
fn ptr_currency_original_consumer_loads_and_distinguishes_codes() {
    let ui = blizzard_ui_dir();
    let env = new_blizzard_addon_env(&ui);
    load_blizzard_addon_closure_into_env(&env, &ui, &["Blizzard_CatalogShopSharedUtil"], &[]);
    assert_consumer_loaded(&env, "Blizzard_CatalogShop_VCUtil.lua");
    env.exec(
        r#"
        assert(Constants.CatalogShopVirtualCurrencyConstants.TALEBOUND_VC_CURRENCY_CODE == 'XZC')
        assert(CatalogShopUtil.IsValidVC('XZC'))
        assert(CatalogShopUtil.IsValidVC('XVV'))
        assert(not CatalogShopUtil.IsValidVC('XWP'))
        assert(not CatalogShopUtil.IsValidVC('unknown'))
        local info = debug.getinfo(CatalogShopUtil.IsValidVC, 'S')
        assert(info.source:gsub('\\', '/'):find('Blizzard_CatalogShop_VCUtil.lua', 1, true))
    "#,
    )
    .expect("original currency consumer must initialize and distinguish currencies");
}

#[test]
fn ptr_vc_init_original_consumer_initializes_reset_state() {
    let ui = blizzard_ui_dir();
    let env = new_blizzard_addon_env(&ui);
    // The separate currency producer is a prerequisite to the catalog closure.
    env.exec("Constants.CatalogShopVirtualCurrencyConstants.TALEBOUND_VC_CURRENCY_CODE = 'XZC'")
        .unwrap();
    load_blizzard_addon_closure_into_env(&env, &ui, &["Blizzard_CatalogShop"], &[]);
    assert_consumer_loaded(&env, "Blizzard_CatalogShop.lua");
    env.exec(
        r#"
        assert(Enum.VcInitState.VcIsReset == 0)
        assert(Enum.VcInitState.VcIsInitializing == 1)
        assert(Enum.VcInitState.VcIsInitialized == 2)
        local mixin = assert(rawget(__secureenv, 'CatalogShopVCFrameMixin'))
        assert(mixin.vcInitState == Enum.VcInitState.VcIsReset)
    "#,
    )
    .expect("original catalog consumer must initialize reset state");
}
