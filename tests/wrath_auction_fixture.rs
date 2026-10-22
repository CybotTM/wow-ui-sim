//! Native cached AuctionUI consumers, not modern AuctionHouse mixin substitutes.
//! Label and row expectations are INFERRED from the committed Wrath UI corpus.
#![cfg(feature = "client-wrath")]

use crate::common;
use wow_ui_sim::lua_api::WowLuaEnv;

fn load_native_auction() -> WowLuaEnv {
    let env = common::panel_fixtures::setup_env();
    common::panel_fixtures::clear_recorded_lua_errors(&env);
    env.exec(
        r#"
        local loaded, reason = C_AddOns.LoadAddOn("Blizzard_AuctionUI")
        assert(loaded, tostring(reason))
        assert(AuctionFrame and AuctionFrameBrowse and AuctionFrameBid and AuctionFrameAuctions, "AuctionFrame and AuctionFrameBrowse and AuctionFrameBid and AuctionFrameAuctions")
        assert(AuctionFrameBrowse.page == 0, "AuctionFrameBrowse.page == 0")
        assert(AuctionFrameBid.page == 0, "AuctionFrameBid.page == 0")
        assert(AuctionFrameAuctions.page == 0, "AuctionFrameAuctions.page == 0")
    "#,
    )
    .unwrap();
    env
}

fn assert_no_errors(env: &WowLuaEnv) {
    let errors = common::panel_fixtures::recorded_lua_errors(env);
    assert!(
        errors.is_empty(),
        "native AuctionUI errors: {}",
        errors.join("\n")
    );
}

#[test]
fn native_browse_search_renders_seeded_row_and_prices() {
    common::with_timeout(90, || {
        let env = load_native_auction();
        env.exec(
            r#"
            A_Admin.ClearAuctionBrowseResults()
            A_Admin.AddAuctionBrowseResult(6948, 1, 2500, 4, false, 7001)
            AuctionFrame_Show()
            assert(AuctionFrame:IsShown(), "AuctionFrame:IsShown()")
            BrowseSearchButton:Click()
            assert(GetNumAuctionItems("list") == 1, "GetNumAuctionItems(\"list\") == 1")
            assert(BrowseButton1:IsShown(), "BrowseButton1:IsShown()")
            assert(not BrowseButton2:IsShown(), "not BrowseButton2:IsShown()")
            assert(BrowseButton1Name:GetText() == "Hearthstone", "BrowseButton1Name:GetText() == \"Hearthstone\"")
            assert(BrowseButton1.itemCount == 4, "BrowseButton1.itemCount == 4")
            assert(BrowseButton1.bidAmount == 2500, "BrowseButton1.bidAmount == 2500")
            assert(BrowseButton1.buyoutPrice == 2500, "BrowseButton1.buyoutPrice == 2500")
            assert(BrowseButton1HighBidder:GetText() == "Browse Seller", "BrowseButton1HighBidder:GetText() == \"Browse Seller\"")
            assert(not BrowseButton1YourBidText:IsShown(), "not BrowseButton1YourBidText:IsShown()")
            HideUIPanel(AuctionFrame)
            assert(not AuctionFrame:IsShown(), "not AuctionFrame:IsShown()")
        "#,
        )
        .unwrap();
        assert_no_errors(&env);
    });
}

#[test]
fn native_bid_tab_renders_seeded_row_and_high_bidder_status() {
    common::with_timeout(90, || {
        let env = load_native_auction();
        env.exec(r#"
            A_Admin.ClearAuctionBids()
            A_Admin.AddAuctionBid(201, 6948, 1, 2, 4100, 5000, 4, 86400, UnitGUID("player"))
            AuctionFrame_Show()
            AuctionFrameTab2:Click()
            assert(AuctionFrameBid:IsShown(), "AuctionFrameBid:IsShown()")
            assert(not AuctionFrameBrowse:IsShown(), "not AuctionFrameBrowse:IsShown()")
            assert(not AuctionFrameAuctions:IsShown(), "not AuctionFrameAuctions:IsShown()")
            assert(GetNumAuctionItems("bidder") == 1, "GetNumAuctionItems(\"bidder\") == 1")
            assert(BidButton1:IsShown(), "BidButton1:IsShown()")
            assert(not BidButton2:IsShown(), "not BidButton2:IsShown()")
            assert(BidButton1Name:GetText() == "Hearthstone", "BidButton1Name:GetText() == \"Hearthstone\"")
            assert(BidButton1.itemCount == 2, "BidButton1.itemCount == 2")
            assert(BidButton1.bidAmount == 4100, "BidButton1.bidAmount == 4100")
            assert(BidButton1.buyoutPrice == 5000, "BidButton1.buyoutPrice == 5000")
            assert(BidButton1BidStatus:GetText() == GREEN_FONT_COLOR_CODE..HIGH_BIDDER..FONT_COLOR_CODE_CLOSE, "BidButton1BidStatus:GetText() == GREEN_FONT_COLOR_CODE..HIGH_BIDDER..FONT_COLOR_CODE_CLOSE")
        "#).unwrap();
        assert_no_errors(&env);
    });
}

#[test]
fn native_owned_tab_renders_seeded_row_and_cancel_price() {
    common::with_timeout(90, || {
        let env = load_native_auction();
        env.exec(
            r#"
            A_Admin.ClearOwnedAuctions()
            A_Admin.AddOwnedAuction(101, 6948, 1, 3, 4000, 5000, 0, 4, 86400)
            AuctionFrame_Show()
            AuctionFrameTab3:Click()
            assert(AuctionFrameAuctions:IsShown(), "AuctionFrameAuctions:IsShown()")
            assert(not AuctionFrameBrowse:IsShown(), "not AuctionFrameBrowse:IsShown()")
            assert(not AuctionFrameBid:IsShown(), "not AuctionFrameBid:IsShown()")
            assert(GetNumAuctionItems("owner") == 1, "GetNumAuctionItems(\"owner\") == 1")
            assert(AuctionsButton1:IsShown(), "AuctionsButton1:IsShown()")
            assert(not AuctionsButton2:IsShown(), "not AuctionsButton2:IsShown()")
            assert(AuctionsButton1Name:GetText() == "Hearthstone", "AuctionsButton1Name:GetText() == \"Hearthstone\"")
            assert(AuctionsButton1.itemCount == 3, "AuctionsButton1.itemCount == 3")
            assert(AuctionsButton1.bidAmount == 4000, "AuctionsButton1.bidAmount == 4000")
            assert(AuctionsButton1.buyoutPrice == 5000, "AuctionsButton1.buyoutPrice == 5000")
            assert(AuctionsButton1.cancelPrice == 200, "AuctionsButton1.cancelPrice == 200")
        "#,
        )
        .unwrap();
        assert_no_errors(&env);
    });
}
