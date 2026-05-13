use crate::common;

use common::blizzard_addon_harness::with_blizzard_addon_smoke_shape;
use common::panel_fixtures::{clear_recorded_lua_errors, recorded_lua_errors};

const ROOT: &str = "Blizzard_AuctionHouseUI";
const MIXIN_METHOD_SURFACE_PROBE_LUA: &str = r#"
local expectedMethods = {
    "OnLoad",
    "OnShow",
    "OnEvent",
    "OnHide",
    "CloseStaticPopups",
    "ClearMaxWidthCaches",
    "UpdateMoneyFrame",
    "SetDisplayMode",
    "GetDisplayMode",
    "IsListingAuctions",
    "SetPostItem",
    "ClearPostItem",
    "UpdateTitle",
    "GetCategoriesList",
    "GetBrowseResultsFrame",
    "GetItemSellList",
    "GetCommoditiesSellListFrames",
    "GetBrowseSearchContext",
    "GetCategorySearchContext",
    "SelectBrowseResult",
    "GetSortOrderState",
    "SetSortOrder",
    "SetBrowseSortOrder",
    "GetBrowseSortOrderState",
    "GetSortsForContext",
    "QueryItem",
    "QueryAll",
    "SendBrowseQuery",
    "SendBrowseQueryInternal",
    "RefreshSearchResults",
    "StartCommoditiesPurchase",
    "StartItemBid",
    "StartItemBuyout",
    "StartItemPurchase",
    "SetSearchText",
    "GetMaxPriceWidth",
    "GetBidStatus",
    "SetDialogOverlayShown",
    "ShowPostConfirmationDialog",
}

local missing = {}
for _, methodName in ipairs(expectedMethods) do
    if type(AuctionHouseFrameMixin[methodName]) ~= "function" then
        table.insert(missing, methodName)
    end
end

return table.concat(missing, "\n")
"#;

#[test]
fn auction_house_frame_mixin_exposes_documented_methods() {
    common::with_perf_lock(|| {
        common::with_timeout(240, || {
            with_blizzard_addon_smoke_shape(&[], &[], |env, _loaded| {
                clear_recorded_lua_errors(env);

                let (loaded, reason): (bool, Option<String>) = env
                    .eval(r#"return C_AddOns.LoadAddOn("Blizzard_AuctionHouseUI")"#)
                    .expect("C_AddOns.LoadAddOn should return");
                assert!(loaded, "`{ROOT}` should load: {reason:?}");

                let missing: String = env
                    .eval(MIXIN_METHOD_SURFACE_PROBE_LUA)
                    .expect("AuctionHouseFrameMixin method probe should run");
                assert!(
                    missing.is_empty(),
                    "`{ROOT}` missing AuctionHouseFrameMixin methods:\n{missing}"
                );

                let errors = recorded_lua_errors(env);
                assert!(
                    errors.is_empty(),
                    "`{ROOT}` mixin-method load emitted Lua errors:\n{}",
                    errors.join("\n")
                );
            });
        });
    });
}
