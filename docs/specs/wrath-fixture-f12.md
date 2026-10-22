# Wrath auction fixture F12

F12 separates modeled auction API probes from native AuctionUI consumer coverage.
Fixtures live in `tests/common/auction_fixtures.rs`; native cases live in
`tests/wrath_auction_fixture.rs`.

## What it must do

- [ ] Preserve all original model probe expectations while avoiding modern UI bootstrap on Wrath.
- [ ] Preserve the original generic show/hide probe verbatim with Wrath-only local `AuctionHouseFrame = AuctionFrame` identifier binding; no modern children or methods are adapted.
- [ ] Load the original cached `Blizzard_AuctionUI` after ordered native FrameXML and preserve the original LoD loaded-state/error assertions.
- [ ] Exercise native browse search, bid and owned tabs with seeded data, labels, quantities and prices (INFERRED from cached UI declarations, not authenticated client behavior).
- [ ] Preserve Retail fixture ordering and assertions, with no baseline-pass regressions.
- [ ] Keep modern-only mixin, commodity/token UI, dependency, persistence, AsyncRequest and grouped-provider requirements ungated and report failing rows rather than fabricate compatibility.

## How it works

- [Addon loading](../addon-loading-pipeline.md)
- [Client profiles](../wiki/systems/client-profiles.md)

## Implementation inventory

- `tests/common/auction_fixtures.rs`: bare Wrath model environment and native LoD fixture selection; other profiles retain their prior bootstrap.
- `tests/blizzard_auction_house_*.rs`: exact F12 model probes and unchanged modern surface oracles.
- `tests/wrath_auction_fixture.rs`: additive original cached AuctionUI consumer assertions.

## Tests asserting this spec

- `tests/blizzard_auction_house_load.rs`
- `tests/blizzard_auction_house_browse_query.rs`
- `tests/blizzard_auction_house_cancel_auction.rs`
- `tests/blizzard_auction_house_commodity_purchase.rs`
- `tests/blizzard_auction_house_commodity_search.rs`
- `tests/blizzard_auction_house_item_key_roundtrip.rs`
- `tests/blizzard_auction_house_item_search.rs`
- `tests/blizzard_auction_house_place_bid.rs`
- `tests/blizzard_auction_house_post_item.rs`
- `tests/blizzard_auction_house_throttle.rs`
- `tests/blizzard_auction_house_wow_token.rs`
- `tests/wrath_auction_fixture.rs`

## Known gaps (current cycle)

- [ ] Build-bound before/after matrix and native consumer RED/GREEN not yet recorded.
- [ ] Cached declarations establish fixture shape only; native 12340 row return contract awaits evidence.

## Out of scope

Modern AuctionHouse mixins, favorites/sorts SavedVariables, token modes and AsyncRequest
are not implemented as Wrath substitutes. New native tests are additive and do not
convert modern-only original rows into passes.
