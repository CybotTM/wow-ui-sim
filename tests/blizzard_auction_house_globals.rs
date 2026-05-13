use crate::common;

use common::blizzard_addon_harness::with_blizzard_addon_smoke_shape;
use common::panel_fixtures::{clear_recorded_lua_errors, recorded_lua_errors};

const ROOT: &str = "Blizzard_AuctionHouseUI";
const GLOBAL_SURFACE_PROBE_LUA: &str = r#"
local failures = {}

local function expect(condition, message)
    if not condition then
        table.insert(failures, message)
    end
end

expect(AuctionHouseFrame ~= nil, "AuctionHouseFrame must exist")
expect(type(AuctionHouseFrameMixin) == "table", "AuctionHouseFrameMixin must be a table")
expect(type(AuctionHouseUtil) == "table", "AuctionHouseUtil must be a table")
expect(type(AuctionCategories) == "table", "AuctionCategories must be a table")
expect(type(AUCTION_HOUSE_FILTER_STRINGS) == "table",
       "AUCTION_HOUSE_FILTER_STRINGS must be a table")
expect(type(AuctionHouseSearchContext) == "table",
       "AuctionHouseSearchContext must be a table")

return table.concat(failures, "\n")
"#;

#[test]
fn auction_house_public_globals_exist_after_load() {
    common::with_perf_lock(|| {
        common::with_timeout(240, || {
            with_blizzard_addon_smoke_shape(&[], &[], |env, _loaded| {
                clear_recorded_lua_errors(env);

                let (loaded, reason): (bool, Option<String>) = env
                    .eval(r#"return C_AddOns.LoadAddOn("Blizzard_AuctionHouseUI")"#)
                    .expect("C_AddOns.LoadAddOn should return");
                assert!(loaded, "`{ROOT}` should load: {reason:?}");

                let failures: String = env
                    .eval(GLOBAL_SURFACE_PROBE_LUA)
                    .expect("AuctionHouse global surface probe should run");
                assert!(
                    failures.is_empty(),
                    "`{ROOT}` missing public globals:\n{failures}"
                );

                let errors = recorded_lua_errors(env);
                assert!(
                    errors.is_empty(),
                    "`{ROOT}` global-surface load emitted Lua errors:\n{}",
                    errors.join("\n")
                );
            });
        });
    });
}
