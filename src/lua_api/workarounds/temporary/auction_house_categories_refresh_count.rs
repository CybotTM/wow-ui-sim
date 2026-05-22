//! Temporary Auction House categories refresh-count repair.
//!
//! Some category refresh paths expect `GetNumElementsForRefresh` before the
//! simulated Auction House categories surface provides it natively.

use crate::lua_api::LoaderEnv;

const AUCTION_HOUSE_CATEGORIES_REFRESH_COUNT_WORKAROUND_LUA: &str = r#"
local function getNumElementsForRefresh()
    return type(AuctionCategories) == "table" and #AuctionCategories or 0
end

local categoriesList = AuctionHouseFrame and AuctionHouseFrame.CategoriesList or nil
if type(categoriesList) == "table"
    and type(categoriesList.GetNumElementsForRefresh) ~= "function" then
    categoriesList.GetNumElementsForRefresh = getNumElementsForRefresh
end

if type(AuctionHouseCategoriesListMixin) ~= "table" then
    return
end

if type(AuctionHouseCategoriesListMixin.GetNumElementsForRefresh) == "function" then
    return
end

AuctionHouseCategoriesListMixin.GetNumElementsForRefresh = getNumElementsForRefresh
"#;

pub(crate) fn patch(env: &LoaderEnv<'_>) {
    let _ = env.exec(AUCTION_HOUSE_CATEGORIES_REFRESH_COUNT_WORKAROUND_LUA);
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lua_api::WowLuaEnv;

    fn patch_env(env: &WowLuaEnv) {
        let _ = env.exec(AUCTION_HOUSE_CATEGORIES_REFRESH_COUNT_WORKAROUND_LUA);
    }

    #[test]
    fn installs_refresh_count_on_categories_list() {
        let env = WowLuaEnv::new().expect("lua env should initialize");
        env.exec(
            r#"
            AuctionCategories = { "armor", "weapon", "consumable" }
            AuctionHouseFrame = { CategoriesList = {} }
            AuctionHouseCategoriesListMixin = {}
            "#,
        )
        .expect("auction category globals should install");

        patch_env(&env);

        let (frame_count, mixin_count): (i64, i64) = env
            .eval(
                r#"
                return AuctionHouseFrame.CategoriesList:GetNumElementsForRefresh(),
                    AuctionHouseCategoriesListMixin:GetNumElementsForRefresh()
                "#,
            )
            .expect("auction category refresh counts should be callable");

        assert_eq!(frame_count, 3);
        assert_eq!(mixin_count, 3);
    }

    #[test]
    fn preserves_existing_refresh_count_methods() {
        let env = WowLuaEnv::new().expect("lua env should initialize");
        env.exec(
            r#"
            AuctionCategories = { "armor", "weapon" }
            AuctionHouseFrame = {
                CategoriesList = {
                    GetNumElementsForRefresh = function()
                        return 41
                    end,
                },
            }
            AuctionHouseCategoriesListMixin = {
                GetNumElementsForRefresh = function()
                    return 42
                end,
            }
            "#,
        )
        .expect("auction category globals should install");

        patch_env(&env);

        let (frame_count, mixin_count): (i64, i64) = env
            .eval(
                r#"
                return AuctionHouseFrame.CategoriesList:GetNumElementsForRefresh(),
                    AuctionHouseCategoriesListMixin:GetNumElementsForRefresh()
                "#,
            )
            .expect("existing auction category refresh counts should be callable");

        assert_eq!(frame_count, 41);
        assert_eq!(mixin_count, 42);
    }
}
