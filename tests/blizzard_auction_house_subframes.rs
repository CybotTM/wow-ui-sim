use crate::common;

use common::blizzard_addon_harness::with_blizzard_addon_smoke_shape;
use common::panel_fixtures::{clear_recorded_lua_errors, recorded_lua_errors};

const ROOT: &str = "Blizzard_AuctionHouseUI";
const SUBFRAME_SURFACE_PROBE_LUA: &str = r#"
local directKeys = {
    "BrowseResultsFrame",
    "CategoriesList",
    "SearchBar",
    "ItemBuyFrame",
    "CommoditiesBuyFrame",
    "ItemSellFrame",
    "CommoditiesSellFrame",
    "AuctionsFrame",
    "WoWTokenResults",
    "WoWTokenSellFrame",
    "BuyDialog",
    "BuyTab",
    "SellTab",
    "AuctionsTab",
}

local missing = {}
for _, key in ipairs(directKeys) do
    if AuctionHouseFrame[key] == nil then
        table.insert(missing, key)
    end
end

if AuctionHouseFrame.MoneyFrameBorder == nil or
        AuctionHouseFrame.MoneyFrameBorder.MoneyFrame == nil then
    table.insert(missing, "MoneyFrameBorder.MoneyFrame")
end

if type(AuctionHouseFrame.Tabs) ~= "table" or #AuctionHouseFrame.Tabs ~= 3 then
    table.insert(missing, "Tabs[1..3]")
end

return table.concat(missing, "\n")
"#;

#[test]
fn auction_house_child_frames_exist_after_load() {
    common::with_perf_lock(|| {
        common::with_timeout(240, || {
            with_blizzard_addon_smoke_shape(&[], &[], |env, _loaded| {
                clear_recorded_lua_errors(env);

                let (loaded, reason): (bool, Option<String>) = env
                    .eval(r#"return C_AddOns.LoadAddOn("Blizzard_AuctionHouseUI")"#)
                    .expect("C_AddOns.LoadAddOn should return");
                assert!(loaded, "`{ROOT}` should load: {reason:?}");

                let missing: String = env
                    .eval(SUBFRAME_SURFACE_PROBE_LUA)
                    .expect("AuctionHouse subframe surface probe should run");
                assert!(missing.is_empty(), "`{ROOT}` missing subframes:\n{missing}");

                let errors = recorded_lua_errors(env);
                assert!(
                    errors.is_empty(),
                    "`{ROOT}` subframe load emitted Lua errors:\n{}",
                    errors.join("\n")
                );
            });
        });
    });
}
