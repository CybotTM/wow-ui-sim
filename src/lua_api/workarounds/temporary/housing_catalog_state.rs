//! Temporary housing/catalog seeded state surface.
//!
//! The housing service flag is Rust-backed, but catalog/decor/neighborhood
//! data is still a seeded UI fixture. Keep those compatibility namespaces out
//! of the generic runtime surface until housing has a real backing subsystem.

const HOUSING_CATALOG_STATE_LUA: &str = include_str!("housing_catalog_state.lua");

pub(crate) fn apply_bootstrap(lua: &mut rilua::Lua) -> crate::Result<()> {
    lua.exec(HOUSING_CATALOG_STATE_LUA)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::lua_api::WowLuaEnv;

    #[test]
    fn installs_seeded_housing_catalog_surface() {
        let env = WowLuaEnv::new().expect("lua env should initialize");

        let result: String = env
            .eval(
                r#"
                if C_CatalogShop.IsShop2Enabled() ~= false then
                    return "bad_shop_flag"
                end
                local productIDs = C_CatalogShop.GetProductIDsForCategory(
                    Constants.HousingCatalogConsts.HOUSING_CATALOG_ALL_CATEGORY_ID)
                if type(productIDs) ~= "table" or #productIDs == 0 then
                    return "bad_catalog_products"
                end
                local decorInfo = C_HousingDecor.GetSelectedDecorInfo()
                if type(decorInfo) ~= "table" or decorInfo.decorID == nil then
                    return "bad_decor"
                end
                local featured = C_HousingCatalog.GetFeaturedSmallProducts()
                if type(featured) ~= "table" or #featured == 0 then
                    return "bad_featured"
                end
                local searcher = C_HousingCatalog.CreateCatalogSearcher()
                if type(searcher) ~= "table" or searcher:GetSearchCount() == 0 then
                    return "bad_searcher"
                end
                return "ok"
                "#,
            )
            .expect("housing catalog probe should run");

        assert_eq!(result, "ok");
    }
}
