//! Tests for `A_Admin` Auction House seed APIs.

use wow_ui_sim::lua_api::WowLuaEnv;

fn env() -> WowLuaEnv {
    WowLuaEnv::new().expect("Failed to create Lua environment")
}

#[test]
fn admin_can_seed_and_clear_auction_browse_results() {
    let env = env();
    let result: String = env
        .eval(
            r#"
            A_Admin.ClearAuctionBrowseResults()
            A_Admin.AddAuctionBrowseResult(210935, 70, 25000, 400, false)
            A_Admin.AddAuctionBrowseResult(224072, 80, 9900000, 1, true)

            local rows = C_AuctionHouse.GetBrowseResults()
            if #rows ~= 2 then
                return "row_count=" .. tostring(#rows)
            end
            if rows[1].itemKey.itemID ~= 210935 then
                return "first_item=" .. tostring(rows[1].itemKey.itemID)
            end
            if rows[2].minPrice ~= 9900000 then
                return "second_price=" .. tostring(rows[2].minPrice)
            end
            if rows[2].containsOwnerItem ~= true then
                return "second_owner=" .. tostring(rows[2].containsOwnerItem)
            end

            A_Admin.ClearAuctionBrowseResults()
            if #C_AuctionHouse.GetBrowseResults() ~= 0 then
                return "clear_failed"
            end

            return "ok"
            "#,
        )
        .unwrap();

    assert_eq!(result, "ok", "admin browse-result seeding should round-trip: {result}");
}

#[test]
fn admin_can_seed_and_clear_auction_replicate_items() {
    let env = env();
    let result: String = env
        .eval(
            r#"
            A_Admin.ClearAuctionReplicateItems()
            A_Admin.AddAuctionReplicateItem("Temporal Alloy", 134400, 17, 3, true, 80, "Item Level")

            local name, texture, count, quality, usable, level, levelType =
                C_AuctionHouse.GetReplicateItemInfo(0)
            if name ~= "Temporal Alloy" then
                return "name=" .. tostring(name)
            end
            if texture ~= 134400 then
                return "texture=" .. tostring(texture)
            end
            if count ~= 17 then
                return "count=" .. tostring(count)
            end
            if quality ~= 3 then
                return "quality=" .. tostring(quality)
            end
            if usable ~= true then
                return "usable=" .. tostring(usable)
            end
            if level ~= 80 then
                return "level=" .. tostring(level)
            end
            if levelType ~= "Item Level" then
                return "levelType=" .. tostring(levelType)
            end

            A_Admin.ClearAuctionReplicateItems()
            if select('#', C_AuctionHouse.GetReplicateItemInfo(0)) ~= 0 then
                return "clear_failed"
            end

            return "ok"
            "#,
        )
        .unwrap();

    assert_eq!(
        result, "ok",
        "admin replicate-item seeding should round-trip: {result}"
    );
}

#[test]
fn admin_can_seed_and_clear_owned_auctions() {
    let env = env();
    let result: String = env
        .eval(
            r#"
            A_Admin.ClearOwnedAuctions()
            -- (auction_id, item_id, item_level, quantity, bid, buyout, status, time_left, time_left_seconds)
            A_Admin.AddOwnedAuction(101, 210935, 70, 200, 0, 50000, 0, 4, 86400)
            A_Admin.AddOwnedAuction(102, 224072, 80, 1, 9000000, 9900000, 0, 3, 7200)

            if C_AuctionHouse.GetNumOwnedAuctions() ~= 2 then
                return "count=" .. tostring(C_AuctionHouse.GetNumOwnedAuctions())
            end
            if C_AuctionHouse.GetNumOwnedAuctionTypes() ~= 2 then
                return "type_count=" .. tostring(C_AuctionHouse.GetNumOwnedAuctionTypes())
            end
            if not C_AuctionHouse.HasFullOwnedAuctionResults() then
                return "not_full"
            end

            local first = C_AuctionHouse.GetOwnedAuctionInfo(1)
            if first.auctionID ~= 101 then
                return "first_id=" .. tostring(first.auctionID)
            end
            if first.itemKey.itemID ~= 210935 then
                return "first_item=" .. tostring(first.itemKey.itemID)
            end
            if first.quantity ~= 200 then
                return "first_qty=" .. tostring(first.quantity)
            end
            if first.buyoutAmount ~= 50000 then
                return "first_buyout=" .. tostring(first.buyoutAmount)
            end
            if first.timeLeft ~= 4 then
                return "first_time_left_band=" .. tostring(first.timeLeft)
            end
            if first.timeLeftSeconds ~= 86400 then
                return "first_time_left_secs=" .. tostring(first.timeLeftSeconds)
            end

            local second = C_AuctionHouse.GetOwnedAuctionInfo(2)
            if second.bidAmount ~= 9000000 then
                return "second_bid=" .. tostring(second.bidAmount)
            end

            local typeRow = C_AuctionHouse.GetOwnedAuctionType(1)
            if typeRow.itemKey.itemID ~= 210935 then
                return "type_first_item=" .. tostring(typeRow.itemKey.itemID)
            end

            -- Out-of-range index returns nothing, not nil padding.
            if C_AuctionHouse.GetOwnedAuctionInfo(99) ~= nil then
                return "out_of_range_not_nil"
            end

            A_Admin.ClearOwnedAuctions()
            if C_AuctionHouse.GetNumOwnedAuctions() ~= 0 then
                return "clear_failed"
            end

            return "ok"
            "#,
        )
        .unwrap();

    assert_eq!(
        result, "ok",
        "admin owned-auction seeding should round-trip: {result}"
    );
}
