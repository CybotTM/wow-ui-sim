use std::path::PathBuf;

use wow_ui_sim::loader::discover_blizzard_addons_for_screen;
use wow_ui_sim::lua_api::WowLuaEnv;
use wow_ui_sim::screen::ScreenKind;
use wow_ui_sim::startup::fire_startup_events_for_screen;

fn blizzard_ui_dir() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("Interface/BlizzardUI")
}

fn load_full_game_ui() -> WowLuaEnv {
    let env = WowLuaEnv::new().expect("Failed to create Lua environment");
    env.set_screen_size(1024.0, 768.0);
    env.set_screen_mode(ScreenKind::Game);

    {
        let mut state = env.state().borrow_mut();
        state.addon_base_paths = vec![blizzard_ui_dir()];
    }

    wow_ui_sim::xml::register_intrinsic_templates();

    let ui = blizzard_ui_dir();
    let addons = discover_blizzard_addons_for_screen(&ui, ScreenKind::Game);
    for (name, toc_path) in &addons {
        wow_ui_sim::loader::load_addon(&env.loader_env(), toc_path).unwrap_or_else(|err| {
            panic!("[load {name}] FAILED: {err}");
        });
    }

    env.apply_post_load_workarounds();
    fire_startup_events_for_screen(&env, ScreenKind::Game);
    env
}

#[test]
fn catalog_shop_loads_and_populates_navigation_and_products() {
    let env = load_full_game_ui();

    let (loaded, reason): (bool, Option<String>) = env
        .eval("return LoadAddOn('Blizzard_CatalogShop')")
        .expect("LoadAddOn('Blizzard_CatalogShop') should return");
    assert!(
        loaded,
        "Blizzard_CatalogShop should load successfully: reason={reason:?}"
    );

    let result: String = env
        .eval(
            r#"
            CatalogShopFrame:Show()

            local categoryIDs = C_CatalogShop.GetAvailableCategoryIDs()
            if not categoryIDs or #categoryIDs == 0 then
                return "no_categories"
            end

            if CatalogShopFrame.failedLoad then
                return "failed_load"
            end

            if CatalogShopFrame.CatalogShopUnavailableScreenFrame:IsShown() then
                return "unavailable_screen_shown"
            end

            local navProvider = CatalogShopFrame.HeaderFrame.CatalogShopNavBar.NavButtonScrollBox:GetDataProvider()
            if not navProvider then
                return "missing_nav_provider"
            end

            if navProvider:GetSize() == 0 then
                return "nav_provider_empty"
            end

            EventRegistry:TriggerEvent("CatalogShop.OnCategorySelected", categoryIDs[1])

            local productProvider = CatalogShopFrame.ProductContainerFrame.ProductsScrollBoxContainer.ScrollBox:GetDataProvider()
            if not productProvider then
                return "missing_product_provider"
            end

            if productProvider:GetSize() == 0 then
                return "product_provider_empty"
            end

            local firstProduct = productProvider:FindElementDataByPredicate(function(elementData)
                return elementData.elementType == CatalogShopConstants.ScrollViewElementType.Product
            end)
            if not firstProduct then
                return "missing_first_product"
            end

            return firstProduct.name or ""
            "#,
        )
        .expect("catalog shop UI should be queryable");

    assert_eq!(result, "Apprentice Rider Bundle");
}
