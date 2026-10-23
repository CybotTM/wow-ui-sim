# Wrath auction fixture F12

F12 separates modeled auction API probes from native AuctionUI consumer coverage.
Fixtures live in `tests/common/auction_fixtures.rs`; native cases live in
`tests/wrath_auction_fixture.rs`.

## What it must do

- [x] Preserve all original model probe expectations while avoiding modern UI bootstrap on Wrath; nine pure API probes pass, while the PostItem probe retains its missing ItemLocation failure.
- [x] Preserve the original generic show/hide probe verbatim with Wrath-only local `AuctionHouseFrame = AuctionFrame` identifier binding; no modern children or methods are adapted.
- [x] Load the original cached `Blizzard_AuctionUI` after ordered native FrameXML and preserve the original LoD loaded-state/error assertions.
- [x] Exercise native browse search, bid and owned tabs with seeded data, labels, quantities and prices (INFERRED from cached UI declarations, not authenticated client behavior).
- [x] Preserve Retail fixture ordering and assertions, with no baseline-pass regressions.
- [x] Keep modern-only mixin, commodity/token UI, dependency, persistence, AsyncRequest and grouped-provider requirements ungated and report failing rows rather than fabricate compatibility.

## How it works

- [Addon loading](../addon-loading-pipeline.md)
- [Client profiles](../wiki/systems/client-profiles.md)

## No-active close boundary

- [x] Original native `AuctionFrame` OnHide must call raw `CloseAuctionHouse` without a modern namespace wrapper failure.
- [ ] With no modeled active auctioneer, closing returns zero values with no events or cleanup (source-inspected; native OnHide proves call/error behavior, not return arity or event absence). Reconstructed 12340 source supports only this tested branch here; active/matching-target double-CLOSED, multisell and conditional NEW_AUCTION_UPDATE remain gaps.

## Producer scope

Reconstructed 12340 evidence establishes thirteen auction row values; cached native
browse/bid/owned consumers expose the displaced price columns in the previous
Mists-shaped row. Wrath removes the Mists level-header/full-name columns and uses
numeric-one/nil flags. Empty browse counts must describe the same modeled result
vector as row lookup. Mists packing and counts remain unchanged; Retail does not
register this legacy module. Invalid-row tuples, argument coercion, uncached item
metadata and owner bidder-name resolution remain unmodified gaps, not parity claims.

## Implementation inventory

- `src/lua_api/globals/auction_verbs.rs`: profile-specific valid-row packing and coherent Wrath browse counts.
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

## Bounded proof

Code `67f33ffa8`; source-equivalent documentation revision `72626a664` supplies fresh
builds, exact cases, checks and formatter proof. Baseline `be7ee3449` has Wrath0/26
and Retail22/26 passing. Final Wrath11/26 and additive native3/3 pass; Retail22/26
retains exactly the same four baseline failures. Every exact invocation executes
one test (nested timeout summaries are not additional cases). These results are not
full-suite or all-native compatibility claims.

Native price-column RED at `adb5f65da` precedes `6dddff5f4`/`16e45077e`.
Fixture-isolated native-close RED at `86d15d096` precedes `67f33ffa8`.
No test-body edits follow the final producer. Fresh stock Wrath `lua-errors` emits
exactly `[]` and exits0. Both full-feature profile checks (`--lib --test integration
--bin wow-sim -j1`) and `cargo fmt --all -- --check` exit0; build/check warnings0.
Mists' eighteen-column output sequence is source-preserved; no Mists runtime gate
was run in this batch. Reconstructed 12340 evidence is not native execution.

## Known gaps (current cycle)

- [ ] Fifteen original Wrath rows remain failing and ungated: modern UI children/mixins/categories/token modes, modern dependency/persistence oracles, AsyncRequest, grouped-provider panels and PostItem's missing ItemLocation fixture.
- [ ] Four Retail baseline failures remain unchanged: dependency, saved-variable and both panel tests hardcode an unavailable Mainline TOC filename. No baseline PASS becomes FAIL.
- [ ] Default modern auction sample rows leak into native Wrath browse; native fixtures isolate those samples, with no producer seed change.
- [ ] Invalid-row thirteen-value tuple, coercion/type validation, level/texture semantics and bidder/owner identity remain unmodeled or unverified; cached widget-price success does not establish those contracts.
- [ ] Active-auctioneer close, matching-target reentrant double-CLOSED, multisell failure and conditional NEW_AUCTION_UPDATE ordering remain unmodeled; only the no-active branch was authorized and exercised.

## Out of scope

Modern AuctionHouse mixins, favorites/sorts SavedVariables, token modes and AsyncRequest
are not implemented as Wrath substitutes. New native tests are additive and do not
convert modern-only original rows into passes.
