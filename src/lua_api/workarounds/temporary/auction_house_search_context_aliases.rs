//! Temporary Auction House search-context alias repair.
//!
//! Retail code still references older alias names in a few startup paths. Keep
//! the alias seeding isolated until the Auction House enum surface is complete.

use crate::lua_api::{LoaderEnv, WowLuaEnv};

const AUCTION_HOUSE_SEARCH_CONTEXT_ALIASES_WORKAROUND_LUA: &str = r#"
if rawget(_G, "__wow_auction_house_search_context_aliases_patched") then
    return
end

if type(AuctionHouseSearchContext) ~= "table" then
    return
end

if AuctionHouseSearchContext.Auctions == nil then
    AuctionHouseSearchContext.Auctions = AuctionHouseSearchContext.AllAuctions
end

if AuctionHouseSearchContext.BrowseFavorites == nil then
    AuctionHouseSearchContext.BrowseFavorites = AuctionHouseSearchContext.AllFavorites
end

rawset(_G, "__wow_auction_house_search_context_aliases_patched", true)
"#;

pub(crate) fn patch(env: &LoaderEnv<'_>) {
    let _ = env.exec(AUCTION_HOUSE_SEARCH_CONTEXT_ALIASES_WORKAROUND_LUA);
}

pub(crate) fn patch_env(env: &WowLuaEnv) {
    let _ = env.exec(AUCTION_HOUSE_SEARCH_CONTEXT_ALIASES_WORKAROUND_LUA);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn seeds_missing_aliases_from_canonical_contexts() {
        let env = WowLuaEnv::new().expect("lua env should initialize");
        env.exec(
            r#"
            AuctionHouseSearchContext = {
                AllAuctions = 12,
                AllFavorites = 34,
            }
            "#,
        )
        .expect("auction search contexts should install");

        patch_env(&env);

        let (auctions, browse_favorites, patched): (i64, i64, bool) = env
            .eval(
                r#"
                return AuctionHouseSearchContext.Auctions,
                    AuctionHouseSearchContext.BrowseFavorites,
                    __wow_auction_house_search_context_aliases_patched == true
                "#,
            )
            .expect("auction search context aliases should be readable");

        assert_eq!(auctions, 12);
        assert_eq!(browse_favorites, 34);
        assert!(patched);
    }

    #[test]
    fn preserves_existing_aliases() {
        let env = WowLuaEnv::new().expect("lua env should initialize");
        env.exec(
            r#"
            AuctionHouseSearchContext = {
                AllAuctions = 12,
                AllFavorites = 34,
                Auctions = 56,
                BrowseFavorites = 78,
            }
            "#,
        )
        .expect("auction search contexts should install");

        patch_env(&env);

        let (auctions, browse_favorites): (i64, i64) = env
            .eval(
                r#"
                return AuctionHouseSearchContext.Auctions,
                    AuctionHouseSearchContext.BrowseFavorites
                "#,
            )
            .expect("preserved auction search context aliases should be readable");

        assert_eq!(auctions, 56);
        assert_eq!(browse_favorites, 78);
    }
}
