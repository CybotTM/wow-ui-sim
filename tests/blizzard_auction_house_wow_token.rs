//! WoW Token surface coverage through `Blizzard_AuctionHouseUI`.

use crate::common;

use common::blizzard_addon_harness::with_blizzard_addon_smoke_shape;
use common::panel_fixtures::{clear_recorded_lua_errors, recorded_lua_errors};

const ROOT: &str = "Blizzard_AuctionHouseUI";
const TOKEN_PRICE_COPPER: i64 = 200_000 * 10_000;
const GUARANTEED_PRICE_COPPER: i64 = 199_500 * 10_000;
const WOW_TOKEN_PROBE_LUA: &str = r#"
local failures = {}

local function expect(condition, message)
    if not condition then
        table.insert(failures, message)
    end
end

local commerceEnabled, pollSeconds, balanceEnabled = C_WowTokenPublic.GetCommerceSystemStatus()
local currentPrice, cachedPrice = C_WowTokenPublic.GetCurrentMarketPrice()
local guaranteedPrice = C_WowTokenPublic.GetGuaranteedPrice()

expect(commerceEnabled == true, "commerce enabled " .. tostring(commerceEnabled))
expect(pollSeconds == 45, "poll seconds " .. tostring(pollSeconds))
expect(balanceEnabled == true, "balance enabled " .. tostring(balanceEnabled))
expect(currentPrice == 2000000000, "current price " .. tostring(currentPrice))
expect(cachedPrice == 2000000000, "cached price " .. tostring(cachedPrice))
expect(guaranteedPrice == 1995000000, "guaranteed price " .. tostring(guaranteedPrice))

return table.concat(failures, "\n")
"#;

#[test]
fn auction_house_wow_token_public_reads_seeded_state() {
    common::with_perf_lock(|| {
        common::with_timeout(240, || {
            with_blizzard_addon_smoke_shape(&[], &[], |env, _loaded| {
                clear_recorded_lua_errors(env);
                seed_wow_token_state(env);

                let (loaded, reason): (bool, Option<String>) = env
                    .eval(r#"return C_AddOns.LoadAddOn("Blizzard_AuctionHouseUI")"#)
                    .expect("C_AddOns.LoadAddOn should return");
                assert!(loaded, "`{ROOT}` should load: {reason:?}");

                let failures: String = env
                    .eval(WOW_TOKEN_PROBE_LUA)
                    .expect("AuctionHouse WoW Token probe should run");
                assert!(
                    failures.is_empty(),
                    "`{ROOT}` WoW Token failures:\n{failures}"
                );

                let errors = recorded_lua_errors(env);
                assert!(
                    errors.is_empty(),
                    "`{ROOT}` WoW Token probe emitted Lua errors:\n{}",
                    errors.join("\n")
                );
            });
        });
    });
}

fn seed_wow_token_state(env: &wow_ui_sim::lua_api::WowLuaEnv) {
    let mut state = env.state().borrow_mut();
    state.wow_token.commerce_enabled = true;
    state.wow_token.poll_seconds = 45;
    state.wow_token.balance_enabled = true;
    state.wow_token.current_market_price = TOKEN_PRICE_COPPER;
    state.wow_token.guaranteed_price = GUARANTEED_PRICE_COPPER;
}
