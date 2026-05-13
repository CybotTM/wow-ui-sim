//! Event-registration coverage for `Blizzard_AuctionHouseUI`.

use crate::common;

use common::panel_fixtures::{clear_recorded_lua_errors, recorded_lua_errors};

const ROOT: &str = "Blizzard_AuctionHouseUI";
const EVENT_REGISTRATIONS_PROBE_LUA: &str = r#"
local failures = {}

local function expect(condition, message)
    if not condition then
        table.insert(failures, message)
    end
end

local requiredEvents = {
    "AUCTION_HOUSE_BROWSE_RESULTS_UPDATED",
    "ITEM_SEARCH_RESULTS_UPDATED",
    "COMMODITY_SEARCH_RESULTS_UPDATED",
    "OWNED_AUCTIONS_UPDATED",
    "BIDS_UPDATED",
    "AUCTION_HOUSE_POST_WARNING",
    "AUCTION_HOUSE_POST_ERROR",
    "PLAYER_MONEY",
}

AuctionHouseFrame:Show()

for _, eventName in ipairs(requiredEvents) do
    expect(AuctionHouseFrame:IsEventRegistered(eventName),
           "AuctionHouseFrame must register " .. eventName .. " after Show")
end

return table.concat(failures, "\n")
"#;

#[test]
fn auction_house_frame_registers_runtime_events_after_show() {
    common::with_perf_lock(|| {
        common::with_timeout(240, || {
            let env = common::panel_fixtures::setup_env();
            load_auction_house_through_runtime(&env);
            load_show_prerequisites(&env);
            clear_recorded_lua_errors(&env);

            let failures: String = env
                .eval(EVENT_REGISTRATIONS_PROBE_LUA)
                .expect("AuctionHouseFrame event-registration probe should run");
            assert!(
                failures.is_empty(),
                "`{ROOT}` missing event registrations:\n{failures}"
            );

            let errors = recorded_lua_errors(&env);
            assert!(
                errors.is_empty(),
                "`{ROOT}` event-registration probe emitted Lua errors:\n{}",
                errors.join("\n")
            );
        });
    });
}

fn load_show_prerequisites(env: &wow_ui_sim::lua_api::WowLuaEnv) {
    env.exec(
        r#"
        local tokenLoaded, tokenReason = C_AddOns.LoadAddOn("Blizzard_TokenUI")
        assert(tokenLoaded, "Blizzard_TokenUI should load: " .. tostring(tokenReason))
        if ContainerFrameSettingsManager and not ContainerFrameSettingsManager.TokenTracker then
            ContainerFrameSettingsManager:OnAddonLoaded("Blizzard_TokenUI")
        end

        local wowTokenLoaded, wowTokenReason = C_AddOns.LoadAddOn("Blizzard_WowTokenUI")
        assert(wowTokenLoaded, "Blizzard_WowTokenUI should load: " .. tostring(wowTokenReason))

        if type(WowToken_IsWowTokenAuctionDialogShown) ~= "function" then
            function WowToken_IsWowTokenAuctionDialogShown()
                return false
            end
        end
        "#,
    )
    .expect("show prerequisite addons should load");
    env.apply_post_load_workarounds();
}

fn load_auction_house_through_runtime(env: &wow_ui_sim::lua_api::WowLuaEnv) {
    let (loaded, reason): (bool, Option<String>) = env
        .eval(r#"return C_AddOns.LoadAddOn("Blizzard_AuctionHouseUI")"#)
        .expect("C_AddOns.LoadAddOn should return");
    assert!(loaded, "`{ROOT}` should load: {reason:?}");
}
