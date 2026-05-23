//! Temporary Catalog Shop product-card layout guard.
//!
//! Catalog shop cards can arrive without a product ID in the simulated startup
//! data path. Keep the nil-safe layout guard isolated until that data is seeded.

use crate::lua_api::{LoaderEnv, WowLuaEnv};

const CATALOG_SHOP_SOUNDKIT_DEFAULTS_LUA: &str = r#"
SOUNDKIT = SOUNDKIT or {}
if SOUNDKIT.CATALOG_SHOP_SELECT_NAV_MENU == nil then
    SOUNDKIT.CATALOG_SHOP_SELECT_NAV_MENU = 303824
end
if SOUNDKIT.CATALOG_SHOP_SELECT_GENERIC_UI_BUTTON == nil then
    SOUNDKIT.CATALOG_SHOP_SELECT_GENERIC_UI_BUTTON = 303826
end
"#;

const CATALOG_SHOP_PRODUCT_CARD_DEFAULTS_WORKAROUND_LUA: &str = r#"
if rawget(_G, "__wow_catalog_shop_product_card_defaults_wrapped") then
    return
end

if type(CatalogShopDefaultProductCardMixin) ~= "table"
    or type(CatalogShopDefaultProductCardMixin.Layout) ~= "function" then
    return
end

local original_layout = CatalogShopDefaultProductCardMixin.Layout

local function resolve_product_id(card)
    if type(card.productInfo) == "table"
        and type(card.productInfo.catalogShopProductID) == "number" then
        return card.productInfo.catalogShopProductID
    end

    if type(card.GetElementData) == "function" then
        local elementData = card:GetElementData()
        if type(elementData) == "table" then
            local productID = elementData.catalogShopProductID or elementData.productID
            if type(productID) == "number" then
                if type(card.productInfo) == "table" then
                    card.productInfo.catalogShopProductID = productID
                end
                return productID
            end
        end
    end

    return nil
end

CatalogShopDefaultProductCardMixin.Layout = function(self, ...)
    if resolve_product_id(self) == nil then
        return
    end
    return original_layout(self, ...)
end

rawset(_G, "__wow_catalog_shop_product_card_defaults_wrapped", true)
"#;

pub(crate) fn apply_bootstrap(lua: &mut rilua::Lua) -> crate::Result<()> {
    lua.exec(CATALOG_SHOP_SOUNDKIT_DEFAULTS_LUA)?;
    Ok(())
}

pub(crate) fn patch(env: &WowLuaEnv) {
    let _ = env.exec(CATALOG_SHOP_PRODUCT_CARD_DEFAULTS_WORKAROUND_LUA);
}

pub(crate) fn patch_for_runtime_addon_load(env: &LoaderEnv<'_>) {
    let _ = env.exec(CATALOG_SHOP_PRODUCT_CARD_DEFAULTS_WORKAROUND_LUA);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn installs_catalog_shop_soundkit_defaults() {
        let env = WowLuaEnv::new().expect("lua env should initialize");
        {
            let mut lua = env.lua.borrow_mut();
            super::apply_bootstrap(&mut lua).expect("catalog shop soundkit defaults should apply");
        }

        let (nav, button): (i64, i64) = env
            .eval(
                r#"
                return SOUNDKIT.CATALOG_SHOP_SELECT_NAV_MENU,
                    SOUNDKIT.CATALOG_SHOP_SELECT_GENERIC_UI_BUTTON
                "#,
            )
            .expect("catalog shop soundkit defaults should be readable");

        assert_eq!(nav, 303824);
        assert_eq!(button, 303826);
    }

    fn install_layout_fixture(env: &WowLuaEnv) {
        env.exec(
            r#"
            layout_calls = 0
            layout_arg = nil
            CatalogShopDefaultProductCardMixin = {
                Layout = function(self, value)
                    layout_calls = layout_calls + 1
                    layout_arg = value
                    return "laid out"
                end,
            }
            "#,
        )
        .expect("catalog product card fixture should install");
    }

    #[test]
    fn skips_layout_without_product_id() {
        let env = WowLuaEnv::new().expect("lua env should initialize");
        install_layout_fixture(&env);
        patch(&env);

        let (result, layout_calls): (Option<String>, i64) = env
            .eval(
                r#"
                local card = {}
                return CatalogShopDefaultProductCardMixin.Layout(card, "ignored"), layout_calls
                "#,
            )
            .expect("catalog card layout should run");

        assert_eq!(result, None);
        assert_eq!(layout_calls, 0);
    }

    #[test]
    fn uses_product_info_id_to_call_original_layout() {
        let env = WowLuaEnv::new().expect("lua env should initialize");
        install_layout_fixture(&env);
        patch(&env);

        let (result, layout_calls, layout_arg): (String, i64, String) = env
            .eval(
                r#"
                local card = { productInfo = { catalogShopProductID = 123 } }
                return CatalogShopDefaultProductCardMixin.Layout(card, "from-product-info"),
                    layout_calls,
                    layout_arg
                "#,
            )
            .expect("catalog card layout should use productInfo ID");

        assert_eq!(result, "laid out");
        assert_eq!(layout_calls, 1);
        assert_eq!(layout_arg, "from-product-info");
    }

    #[test]
    fn copies_element_data_product_id_before_layout() {
        let env = WowLuaEnv::new().expect("lua env should initialize");
        install_layout_fixture(&env);
        patch(&env);

        let (layout_calls, product_id): (i64, i64) = env
            .eval(
                r#"
                local card = {
                    productInfo = {},
                    GetElementData = function()
                        return { productID = 456 }
                    end,
                }
                CatalogShopDefaultProductCardMixin.Layout(card, "from-element-data")
                return layout_calls, card.productInfo.catalogShopProductID
                "#,
            )
            .expect("catalog card layout should use element data ID");

        assert_eq!(layout_calls, 1);
        assert_eq!(product_id, 456);
    }
}
