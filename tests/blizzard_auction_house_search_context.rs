//! Search-context enum coverage for `Blizzard_AuctionHouseUI`.

use crate::common;

use common::blizzard_addon_harness::with_blizzard_addon_smoke_shape;
use common::panel_fixtures::{clear_recorded_lua_errors, recorded_lua_errors};

const ROOT: &str = "Blizzard_AuctionHouseUI";
const SEARCH_CONTEXT_PROBE_LUA: &str = r#"
local failures = {}

local function expect(condition, message)
    if not condition then
        table.insert(failures, message)
    end
end

local requiredContexts = {
    "BuyItems",
    "BuyCommodities",
    "Auctions",
    "BrowseAll",
    "BrowseFavorites",
}

expect(type(AuctionHouseSearchContext) == "table",
       "AuctionHouseSearchContext must be a table")

if type(AuctionHouseSearchContext) == "table" then
    local seenValues = {}
    for _, contextName in ipairs(requiredContexts) do
        local contextValue = AuctionHouseSearchContext[contextName]
        expect(type(contextValue) == "number",
               "AuctionHouseSearchContext." .. contextName .. " must be numeric")

        if type(contextValue) == "number" then
            expect(contextValue > 0,
                   "AuctionHouseSearchContext." .. contextName .. " must be positive")
            expect(seenValues[contextValue] == nil,
                   "AuctionHouseSearchContext." .. contextName .. " must be unique")
            seenValues[contextValue] = contextName
        end
    end
end

return table.concat(failures, "\n")
"#;

#[test]
fn auction_house_search_context_exposes_saved_variable_keys() {
    common::with_perf_lock(|| {
        common::with_timeout(240, || {
            with_blizzard_addon_smoke_shape(&[], &[], |env, _loaded| {
                clear_recorded_lua_errors(env);

                let (loaded, reason): (bool, Option<String>) = env
                    .eval(r#"return C_AddOns.LoadAddOn("Blizzard_AuctionHouseUI")"#)
                    .expect("C_AddOns.LoadAddOn should return");
                assert!(loaded, "`{ROOT}` should load: {reason:?}");

                let failures: String = env
                    .eval(SEARCH_CONTEXT_PROBE_LUA)
                    .expect("AuctionHouseSearchContext probe should run");
                assert!(
                    failures.is_empty(),
                    "`{ROOT}` search-context enum mismatch:\n{failures}"
                );

                let errors = recorded_lua_errors(env);
                assert!(
                    errors.is_empty(),
                    "`{ROOT}` search-context probe emitted Lua errors:\n{}",
                    errors.join("\n")
                );
            });
        });
    });
}
