//! Display-mode switching coverage for `Blizzard_AuctionHouseUI`.

use crate::common;

use common::panel_fixtures::{clear_recorded_lua_errors, recorded_lua_errors};

const ROOT: &str = "Blizzard_AuctionHouseUI";
const TAB_SWITCH_PROBE_LUA: &str = r#"
local failures = {}

local function expect(condition, message)
    if not condition then
        table.insert(failures, message)
    end
end

local function expectShown(key, shown)
    local frame = AuctionHouseFrame[key]
    expect(frame ~= nil, key .. " exists")
    if frame then
        expect(frame:IsShown() == shown, key .. " shown=" .. tostring(frame:IsShown()))
    end
end

local function expectMode(displayMode, label)
    expect(AuctionHouseFrame:GetDisplayMode() == displayMode, label .. " display mode")
end

local function expectBuyMode()
    expectMode(AuctionHouseFrameDisplayMode.Buy, "buy tab")
    expectShown("CategoriesList", true)
    expectShown("SearchBar", true)
    expectShown("BrowseResultsFrame", true)
    expectShown("ItemSellFrame", false)
    expectShown("CommoditiesSellFrame", false)
    expectShown("AuctionsFrame", false)
    expectShown("WoWTokenResults", false)
    expectShown("WoWTokenSellFrame", false)
end

local function expectSellMode()
    expectMode(AuctionHouseFrameDisplayMode.ItemSell, "sell tab")
    expectShown("ItemSellFrame", true)
    expectShown("ItemSellList", true)
    expectShown("CategoriesList", false)
    expectShown("BrowseResultsFrame", false)
    expectShown("CommoditiesSellFrame", false)
    expectShown("AuctionsFrame", false)
    expectShown("WoWTokenSellFrame", false)
end

local function expectAuctionsMode()
    expectMode(AuctionHouseFrameDisplayMode.Auctions, "auctions tab")
    expectShown("AuctionsFrame", true)
    expectShown("CategoriesList", false)
    expectShown("BrowseResultsFrame", false)
    expectShown("ItemSellFrame", false)
    expectShown("CommoditiesSellFrame", false)
    expectShown("WoWTokenResults", false)
    expectShown("WoWTokenSellFrame", false)
end

local function expectWoWTokenBuyMode()
    expectMode(AuctionHouseFrameDisplayMode.WoWTokenBuy, "wow token buy mode")
    expectShown("CategoriesList", true)
    expectShown("SearchBar", true)
    expectShown("WoWTokenResults", true)
    expectShown("BrowseResultsFrame", false)
    expectShown("ItemSellFrame", false)
    expectShown("AuctionsFrame", false)
    expectShown("WoWTokenSellFrame", false)
end

local function expectWoWTokenSellMode()
    expectMode(AuctionHouseFrameDisplayMode.WoWTokenSell, "wow token sell mode")
    expectShown("WoWTokenSellFrame", true)
    expectShown("CategoriesList", false)
    expectShown("BrowseResultsFrame", false)
    expectShown("ItemSellFrame", false)
    expectShown("CommoditiesSellFrame", false)
    expectShown("AuctionsFrame", false)
    expectShown("WoWTokenResults", false)
end

ShowUIPanel(AuctionHouseFrame)
expect(AuctionHouseFrame:IsShown(), "AuctionHouseFrame shown")

AuctionHouseFrame.BuyTab:Click()
expectBuyMode()

AuctionHouseFrame.SellTab:Click()
expectSellMode()

AuctionHouseFrame.AuctionsTab:Click()
expectAuctionsMode()

AuctionHouseFrame:SetDisplayMode(AuctionHouseFrameDisplayMode.WoWTokenBuy)
expectWoWTokenBuyMode()

AuctionHouseFrame:SetDisplayMode(AuctionHouseFrameDisplayMode.WoWTokenSell)
expectWoWTokenSellMode()

HideUIPanel(AuctionHouseFrame)

return table.concat(failures, "\n")
"#;

#[test]
fn auction_house_tabs_and_token_modes_show_matching_subframes() {
    common::with_perf_lock(|| {
        common::with_timeout(240, || {
            let env = common::panel_fixtures::setup_env();
            load_auction_house_through_runtime(&env);
            load_show_hide_prerequisites(&env);
            clear_recorded_lua_errors(&env);

            let failures: String = env
                .eval(TAB_SWITCH_PROBE_LUA)
                .expect("AuctionHouseFrame tab-switch probe should run");
            assert!(
                failures.is_empty(),
                "`{ROOT}` tab-switch failures:\n{failures}"
            );

            let errors = recorded_lua_errors(&env);
            assert!(
                errors.is_empty(),
                "`{ROOT}` tab switching emitted Lua errors:\n{}",
                errors.join("\n")
            );
        });
    });
}

fn load_show_hide_prerequisites(env: &wow_ui_sim::lua_api::WowLuaEnv) {
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
    .expect("tab-switch prerequisite addons should load");
    env.apply_post_load_workarounds();
}

fn load_auction_house_through_runtime(env: &wow_ui_sim::lua_api::WowLuaEnv) {
    let (loaded, reason): (bool, Option<String>) = env
        .eval(r#"return C_AddOns.LoadAddOn("Blizzard_AuctionHouseUI")"#)
        .expect("C_AddOns.LoadAddOn should return");
    assert!(loaded, "`{ROOT}` should load: {reason:?}");
}
