const AUCTION_HOUSE_AUCTION_CREATED: &str = "AUCTION_HOUSE_AUCTION_CREATED";
const AUCTION_CANCELED: &str = "AUCTION_CANCELED";
const OWNED_AUCTIONS_UPDATED: &str = "OWNED_AUCTIONS_UPDATED";

/// Vendor-sell percentage paid as deposit. Multiplied by quantity and
/// duration band to mirror the live client's deposit formula
/// (`AuctionHouseUtil.lua` builds it the same way).
const DEPOSIT_PERCENT: i64 = 15;
const SECONDS_24_HOURS: i64 = 24 * 60 * 60;

/// `Enum.AuctionStatus` values used by `OwnedAuction.status`. The sim
/// flips a canceled row to `Sold` so the Auctions tab repaints with the
/// "no longer active" treatment.
const AUCTION_STATUS_ACTIVE: i32 = 0;
const AUCTION_STATUS_SOLD: i32 = 1;

/// `Enum.AuctionHouseTimeLeftBand.VeryLong`. Fresh posts always start
/// here — 12h/24h/48h durations all sit inside the 12..48h window the
/// VeryLong band spans.
const TIME_LEFT_BAND_VERY_LONG: i32 = 4;

/// 15% of vendor-sell × quantity × duration band, rounded down to copper.
/// Matches the live client's posted-deposit formula closely enough for
/// the sell-frame to display consistent numbers.
fn deposit_for(item_id: u32, duration: i32, quantity: i32) -> i64 {
    let Some(item) = items::get_item(item_id) else {
        return 0;
    };
    let sell_price = item.sell_price as i64;
    let qty = quantity.max(0) as i64;
    let dur = duration.clamp(1, 3) as i64;
    (sell_price * qty * DEPOSIT_PERCENT * dur) / 100
}

/// Maps `Enum.AuctionHouseDuration` (1..3) to seconds. Out-of-range
/// values fall back to the longest duration so a tester passing 0
/// still gets a usable row.
fn time_left_seconds_for_duration(duration: i32) -> i64 {
    match duration {
        1 => SECONDS_12_HOURS,
        2 => SECONDS_24_HOURS,
        _ => SECONDS_48_HOURS,
    }
}

fn next_auction_id(state: &mut LuaState) -> LuaResult<i32> {
    let next = borrow_state(state)?
        .auction_owned
        .iter()
        .map(|row| row.auction_id)
        .max()
        .unwrap_or(0)
        + 1;
    Ok(next)
}

fn deduct_player_money(state: &mut LuaState, amount: i64) -> LuaResult<()> {
    borrow_state_mut(state)?.player.money -= amount;
    Ok(())
}

fn refund_player_money(state: &mut LuaState, amount: i64) -> LuaResult<()> {
    borrow_state_mut(state)?.player.money += amount;
    Ok(())
}

fn dispatch_owned_auctions_updated(state: &mut LuaState) -> LuaResult<()> {
    dispatch_event_now(state, OWNED_AUCTIONS_UPDATED, &[])
}

fn dispatch_auction_created(state: &mut LuaState, auction_id: i32) -> LuaResult<()> {
    dispatch_event_now(
        state,
        AUCTION_HOUSE_AUCTION_CREATED,
        &[Val::Num(auction_id as f64)],
    )
}

fn dispatch_auction_canceled(state: &mut LuaState, auction_id: i32) -> LuaResult<()> {
    dispatch_event_now(state, AUCTION_CANCELED, &[Val::Num(auction_id as f64)])
}

/// Read a nilable copper amount. `bid?`/`buyout?` slots default to 0
/// when the addon omits them (matches the live client treating
/// `nil`/`0` as "no bid set").
fn read_optional_money_arg(state: &mut LuaState, slot: i32) -> LuaResult<i64> {
    Ok(match Val::from_stack(state, slot)? {
        Val::Num(n) => n as i64,
        _ => 0,
    })
}

/// Captures the per-listing inputs `PostItem`/`PostCommodity` collect
/// from the Lua stack so the shared finalize path stays under the
/// param-overload threshold.
struct PostListingContext {
    item_id: u32,
    duration: i32,
    quantity: i32,
    bid_amount: i64,
    buyout_amount: i64,
    deposit: i64,
}

/// Build + append a fresh `OwnedAuction`, deduct the deposit, fire the
/// pair of post-listing events, and clear the in-flight quote. Shared
/// between `PostItem`/`PostCommodity` and their `Confirm*` siblings.
fn finalize_owned_auction_post(state: &mut LuaState, ctx: &PostListingContext) -> LuaResult<i32> {
    let auction_id = next_auction_id(state)?;
    let row = build_owned_auction_row(auction_id, ctx);
    let owner_name = borrow_state(state)?.player.name.clone();
    {
        let mut sim = borrow_state_mut(state)?;
        sim.auction_owned.push(row);
        sim.auction_index.insert(
            auction_id as i64,
            AuctionRowInfo {
                owner: owner_name,
                bid_amount: ctx.bid_amount,
                buyout_amount: ctx.buyout_amount,
                deposit: ctx.deposit,
                consortium_cut: 0,
            },
        );
        sim.auction_sell_quote = None;
    }
    deduct_player_money(state, ctx.deposit)?;
    dispatch_auction_created(state, auction_id)?;
    dispatch_owned_auctions_updated(state)?;
    Ok(auction_id)
}

fn build_owned_auction_row(auction_id: i32, ctx: &PostListingContext) -> OwnedAuction {
    let item_level = items::get_item(ctx.item_id)
        .map(|item| item.item_level as i32)
        .unwrap_or(0);
    OwnedAuction {
        auction_id,
        item_id: ctx.item_id as i32,
        item_level,
        quantity: ctx.quantity,
        bid_amount: ctx.bid_amount,
        buyout_amount: ctx.buyout_amount,
        status: AUCTION_STATUS_ACTIVE,
        time_left: TIME_LEFT_BAND_VERY_LONG,
        time_left_seconds: time_left_seconds_for_duration(ctx.duration),
    }
}

fn capture_sell_quote(state: &mut LuaState, quote: AuctionSellQuote) -> LuaResult<()> {
    borrow_state_mut(state)?.auction_sell_quote = Some(quote);
    Ok(())
}

/// Buyout for the owned-auction row: items post a flat buyout, commodities
/// post `unit_price * quantity`. Mirrors retail's `PostCommodity` math.
fn buyout_from_quote(quote: &AuctionSellQuote) -> i64 {
    match quote.kind {
        AuctionSellQuoteKind::Item => quote.unit_price,
        AuctionSellQuoteKind::Commodity => quote.unit_price * quote.quantity as i64,
    }
}

/// Capture the in-flight quote, then finalize the listing using the same
/// quote fields. Shared between `PostItem` and `PostCommodity` so each
/// entry point only does Lua-stack parsing.
fn post_listing(state: &mut LuaState, quote: AuctionSellQuote, bid: i64) -> LuaResult<()> {
    let ctx = PostListingContext {
        item_id: quote.item_id as u32,
        duration: quote.duration,
        quantity: quote.quantity,
        bid_amount: bid,
        buyout_amount: buyout_from_quote(&quote),
        deposit: quote.deposit,
    };
    capture_sell_quote(state, quote)?;
    finalize_owned_auction_post(state, &ctx)?;
    Ok(())
}

/// Flip an `Active` row to `Sold` and return its `(bid, buyout)` so the
/// caller can compute the cancel refund. `None` when the auction id is
/// unknown or already inactive — matches the live client treating
/// `CancelAuction` on a stale id as a silent no-op.
fn mark_owned_auction_canceled(
    state: &mut LuaState,
    auction_id: i32,
) -> LuaResult<Option<(i64, i64)>> {
    let mut sim = borrow_state_mut(state)?;
    let Some(row) = sim
        .auction_owned
        .iter_mut()
        .find(|r| r.auction_id == auction_id)
    else {
        return Ok(None);
    };
    if row.status != AUCTION_STATUS_ACTIVE {
        return Ok(None);
    }
    row.status = AUCTION_STATUS_SOLD;
    Ok(Some((row.bid_amount, row.buyout_amount)))
}

fn c_auction_house_calculate_item_deposit(state: &mut LuaState) -> LuaResult<u32> {
    let location = Val::from_stack(state, 1)?;
    let duration = i32::from_stack(state, 2)?;
    let quantity = i32::from_stack(state, 3)?;
    let Some(item_id) = extract_item_id_from_location(state, location) else {
        state.push(Val::Nil);
        return Ok(1);
    };
    let deposit = deposit_for(item_id, duration, quantity);
    state.push(Val::Num(deposit as f64));
    Ok(1)
}

fn c_auction_house_calculate_commodity_deposit(state: &mut LuaState) -> LuaResult<u32> {
    let item_id = i32::from_stack(state, 1)?;
    let duration = i32::from_stack(state, 2)?;
    let quantity = i32::from_stack(state, 3)?;
    let deposit = deposit_for(item_id as u32, duration, quantity);
    state.push(Val::Num(deposit as f64));
    Ok(1)
}

fn c_auction_house_post_item(state: &mut LuaState) -> LuaResult<u32> {
    let location = Val::from_stack(state, 1)?;
    let duration = i32::from_stack(state, 2)?;
    let quantity = i32::from_stack(state, 3)?;
    let bid = read_optional_money_arg(state, 4)?;
    let buyout = read_optional_money_arg(state, 5)?;
    let Some(item_id) = extract_item_id_from_location(state, location) else {
        state.push(Val::Bool(false));
        return Ok(1);
    };
    let quote = AuctionSellQuote {
        kind: AuctionSellQuoteKind::Item,
        item_id: item_id as i32,
        duration,
        quantity,
        unit_price: buyout,
        deposit: deposit_for(item_id, duration, quantity),
    };
    post_listing(state, quote, bid)?;
    state.push(Val::Bool(true));
    Ok(1)
}

fn c_auction_house_post_commodity(state: &mut LuaState) -> LuaResult<u32> {
    let location = Val::from_stack(state, 1)?;
    let duration = i32::from_stack(state, 2)?;
    let quantity = i32::from_stack(state, 3)?;
    let unit_price = read_optional_money_arg(state, 4)?;
    let Some(item_id) = extract_item_id_from_location(state, location) else {
        state.push(Val::Bool(false));
        return Ok(1);
    };
    let quote = AuctionSellQuote {
        kind: AuctionSellQuoteKind::Commodity,
        item_id: item_id as i32,
        duration,
        quantity,
        unit_price,
        deposit: deposit_for(item_id, duration, quantity),
    };
    post_listing(state, quote, 0)?;
    state.push(Val::Bool(true));
    Ok(1)
}

fn c_auction_house_confirm_post_item(state: &mut LuaState) -> LuaResult<u32> {
    c_auction_house_post_item(state)
}

fn c_auction_house_confirm_post_commodity(state: &mut LuaState) -> LuaResult<u32> {
    c_auction_house_post_commodity(state)
}

fn c_auction_house_cancel_sell(state: &mut LuaState) -> LuaResult<u32> {
    borrow_state_mut(state)?.auction_sell_quote = None;
    Ok(0)
}

fn c_auction_house_cancel_auction(state: &mut LuaState) -> LuaResult<u32> {
    let auction_id = i32::from_stack(state, 1)?;
    let Some((bid_amount, buyout_amount)) = mark_owned_auction_canceled(state, auction_id)? else {
        return Ok(0);
    };
    let cancel_cost = cancel_cost_for_buyout(buyout_amount);
    let refund = (bid_amount - cancel_cost).max(0);
    refund_player_money(state, refund)?;
    dispatch_auction_canceled(state, auction_id)?;
    dispatch_owned_auctions_updated(state)?;
    Ok(0)
}
