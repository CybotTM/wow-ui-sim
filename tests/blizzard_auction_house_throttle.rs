//! Throttled browse-query coverage for `Blizzard_AuctionHouseUI`.

use crate::common;

use common::blizzard_addon_harness::with_blizzard_addon_smoke_shape;
use common::panel_fixtures::{clear_recorded_lua_errors, recorded_lua_errors};

const ROOT: &str = "Blizzard_AuctionHouseUI";
const THROTTLE_PROBE_LUA: &str = r#"
local failures = {}

local function expect(condition, message)
    if not condition then
        table.insert(failures, message)
    end
end

A_Admin.ClearAuctionBrowseResults()
A_Admin.AddAuctionBrowseResult(210935, 70, 25000, 400, false)
A_Admin.AddAuctionBrowseResult(224072, 80, 9000000, 1, true)
A_Admin.SetAuctionThrottleReady(false)

local queuedCount = 0
local readyCount = 0
local resultCount = 0
local listener = CreateFrame("Frame")
listener:RegisterEvent("AUCTION_HOUSE_THROTTLED_MESSAGE_QUEUED")
listener:RegisterEvent("AUCTION_HOUSE_THROTTLED_SYSTEM_READY")
listener:RegisterEvent("AUCTION_HOUSE_BROWSE_RESULTS_UPDATED")
listener:SetScript("OnEvent", function(_, event)
    if event == "AUCTION_HOUSE_THROTTLED_MESSAGE_QUEUED" then
        queuedCount = queuedCount + 1
    elseif event == "AUCTION_HOUSE_THROTTLED_SYSTEM_READY" then
        readyCount = readyCount + 1
    elseif event == "AUCTION_HOUSE_BROWSE_RESULTS_UPDATED" then
        resultCount = resultCount + 1
    end
end)

C_AuctionHouse.SendBrowseQuery({
    searchString = "",
    sorts = {},
})

expect(queuedCount == 1, "queued event count " .. tostring(queuedCount))
expect(resultCount == 0, "result event before ready " .. tostring(resultCount))
expect(C_AuctionHouse.IsThrottledMessageSystemReady() == false,
       "throttle should report not ready")

A_Admin.SetAuctionThrottleReady(true)

expect(C_AuctionHouse.IsThrottledMessageSystemReady() == true,
       "throttle should report ready")
expect(readyCount == 1, "ready event count " .. tostring(readyCount))
expect(resultCount == 1, "result event after ready " .. tostring(resultCount))

local rows = C_AuctionHouse.GetBrowseResults()
expect(#rows == 2, "browse rows after replay " .. tostring(#rows))
if #rows >= 2 then
    expect(rows[1].itemKey.itemID == 210935, "row 1 itemID")
    expect(rows[2].itemKey.itemID == 224072, "row 2 itemID")
end

return table.concat(failures, "\n")
"#;

#[test]
fn auction_house_throttle_replays_queued_browse_query_when_ready() {
    common::with_perf_lock(|| {
        common::with_timeout(240, || {
            with_blizzard_addon_smoke_shape(&[], &[], |env, _loaded| {
                clear_recorded_lua_errors(env);

                let (loaded, reason): (bool, Option<String>) = env
                    .eval(r#"return C_AddOns.LoadAddOn("Blizzard_AuctionHouseUI")"#)
                    .expect("C_AddOns.LoadAddOn should return");
                assert!(loaded, "`{ROOT}` should load: {reason:?}");

                let failures: String = env
                    .eval(THROTTLE_PROBE_LUA)
                    .expect("AuctionHouse throttle probe should run");
                assert!(
                    failures.is_empty(),
                    "`{ROOT}` throttle failures:\n{failures}"
                );

                let errors = recorded_lua_errors(env);
                assert!(
                    errors.is_empty(),
                    "`{ROOT}` throttle probe emitted Lua errors:\n{}",
                    errors.join("\n")
                );
            });
        });
    });
}
