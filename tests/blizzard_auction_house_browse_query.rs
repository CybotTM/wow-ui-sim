//! Browse-query coverage for `Blizzard_AuctionHouseUI`.

use crate::common;

use common::blizzard_addon_harness::with_blizzard_addon_smoke_shape;
use common::panel_fixtures::{clear_recorded_lua_errors, recorded_lua_errors};

const ROOT: &str = "Blizzard_AuctionHouseUI";
const BROWSE_QUERY_PROBE_LUA: &str = r#"
local failures = {}

local function expect(condition, message)
    if not condition then
        table.insert(failures, message)
    end
end

A_Admin.ClearAuctionBrowseResults()
A_Admin.AddAuctionBrowseResult(210935, 70, 25000, 400, false)
A_Admin.AddAuctionBrowseResult(224072, 80, 9000000, 1, true)
A_Admin.AddAuctionBrowseResult(228231, 78, 125000, 12, false)

local eventCount = 0
local listener = CreateFrame("Frame")
listener:RegisterEvent("AUCTION_HOUSE_BROWSE_RESULTS_UPDATED")
listener:SetScript("OnEvent", function()
    eventCount = eventCount + 1
end)

C_AuctionHouse.SendBrowseQuery({
    searchString = "",
    sorts = {},
})

expect(eventCount == 1, "browse results event count " .. tostring(eventCount))

local rows = C_AuctionHouse.GetBrowseResults()
expect(type(rows) == "table", "GetBrowseResults must return a table")
expect(#rows == 3, "browse row count " .. tostring(#rows))

if #rows >= 3 then
    expect(rows[1].itemKey.itemID == 210935, "row 1 itemID")
    expect(rows[1].itemKey.itemLevel == 70, "row 1 itemLevel")
    expect(rows[1].minPrice == 25000, "row 1 minPrice")
    expect(rows[1].totalQuantity == 400, "row 1 quantity")
    expect(rows[1].containsOwnerItem == false, "row 1 owner flag")

    expect(rows[2].itemKey.itemID == 224072, "row 2 itemID")
    expect(rows[2].itemKey.itemLevel == 80, "row 2 itemLevel")
    expect(rows[2].minPrice == 9000000, "row 2 minPrice")
    expect(rows[2].totalQuantity == 1, "row 2 quantity")
    expect(rows[2].containsOwnerItem == true, "row 2 owner flag")

    expect(rows[3].itemKey.itemID == 228231, "row 3 itemID")
    expect(rows[3].itemKey.itemLevel == 78, "row 3 itemLevel")
    expect(rows[3].minPrice == 125000, "row 3 minPrice")
    expect(rows[3].totalQuantity == 12, "row 3 quantity")
    expect(rows[3].containsOwnerItem == false, "row 3 owner flag")
end

return table.concat(failures, "\n")
"#;

#[test]
fn auction_house_browse_query_returns_seeded_rows() {
    common::with_perf_lock(|| {
        common::with_timeout(240, || {
            with_blizzard_addon_smoke_shape(&[], &[], |env, _loaded| {
                clear_recorded_lua_errors(env);

                let (loaded, reason): (bool, Option<String>) = env
                    .eval(r#"return C_AddOns.LoadAddOn("Blizzard_AuctionHouseUI")"#)
                    .expect("C_AddOns.LoadAddOn should return");
                assert!(loaded, "`{ROOT}` should load: {reason:?}");

                let failures: String = env
                    .eval(BROWSE_QUERY_PROBE_LUA)
                    .expect("AuctionHouse browse-query probe should run");
                assert!(
                    failures.is_empty(),
                    "`{ROOT}` browse-query failures:\n{failures}"
                );

                let errors = recorded_lua_errors(env);
                assert!(
                    errors.is_empty(),
                    "`{ROOT}` browse-query probe emitted Lua errors:\n{}",
                    errors.join("\n")
                );
            });
        });
    });
}
