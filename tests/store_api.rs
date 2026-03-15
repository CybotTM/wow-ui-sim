use wow_ui_sim::lua_api::WowLuaEnv;

fn env() -> WowLuaEnv {
    WowLuaEnv::new().expect("Failed to create Lua environment")
}

#[test]
fn store_apis_report_enabled_and_available() {
    let env = env();
    let (store_public_enabled, store_secure_available, has_purchase_list, has_product_list, has_distribution_list): (
        bool,
        bool,
        bool,
        bool,
        bool,
    ) = env
        .eval(
            "return C_StorePublic.IsEnabled(), C_StoreSecure.IsAvailable(), C_StoreSecure.HasPurchaseList(), C_StoreSecure.HasProductList(), C_StoreSecure.HasDistributionList()",
        )
        .expect("store API flags should be queryable");

    assert!(store_public_enabled, "C_StorePublic.IsEnabled() should be true");
    assert!(store_secure_available, "C_StoreSecure.IsAvailable() should be true");
    assert!(has_purchase_list, "C_StoreSecure.HasPurchaseList() should be true");
    assert!(has_product_list, "C_StoreSecure.HasProductList() should be true");
    assert!(
        has_distribution_list,
        "C_StoreSecure.HasDistributionList() should be true"
    );
}
