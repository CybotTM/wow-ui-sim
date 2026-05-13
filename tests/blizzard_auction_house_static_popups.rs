//! StaticPopup registration coverage for `Blizzard_AuctionHouseUI`.

use crate::common;

use common::blizzard_addon_harness::with_blizzard_addon_smoke_shape;
use common::panel_fixtures::{clear_recorded_lua_errors, recorded_lua_errors};

const ROOT: &str = "Blizzard_AuctionHouseUI";
const STATIC_POPUPS_PROBE_LUA: &str = r#"
local failures = {}

local function expect(condition, message)
    if not condition then
        table.insert(failures, message)
    end
end

local popupNames = {
    "BUYOUT_AUCTION",
    "BID_AUCTION",
    "PURCHASE_AUCTION_UNIQUE",
    "CANCEL_AUCTION",
    "AUCTION_HOUSE_POST_WARNING",
    "AUCTION_HOUSE_POST_ERROR",
    "TOKEN_NONE_FOR_SALE",
    "TOKEN_AUCTIONABLE_TOKEN_OWNED",
}

for _, popupName in ipairs(popupNames) do
    expect(type(StaticPopupDialogs[popupName]) == "table",
           "StaticPopupDialogs[" .. popupName .. "] must be registered")
end

return table.concat(failures, "\n")
"#;

#[test]
fn auction_house_static_popups_are_registered_after_load() {
    common::with_perf_lock(|| {
        common::with_timeout(240, || {
            with_blizzard_addon_smoke_shape(&[], &[], |env, _loaded| {
                clear_recorded_lua_errors(env);

                let (loaded, reason): (bool, Option<String>) = env
                    .eval(r#"return C_AddOns.LoadAddOn("Blizzard_AuctionHouseUI")"#)
                    .expect("C_AddOns.LoadAddOn should return");
                assert!(loaded, "`{ROOT}` should load: {reason:?}");

                let failures: String = env
                    .eval(STATIC_POPUPS_PROBE_LUA)
                    .expect("AuctionHouse static popup probe should run");
                assert!(
                    failures.is_empty(),
                    "`{ROOT}` missing static popups:\n{failures}"
                );

                let errors = recorded_lua_errors(env);
                assert!(
                    errors.is_empty(),
                    "`{ROOT}` static popup probe emitted Lua errors:\n{}",
                    errors.join("\n")
                );
            });
        });
    });
}
