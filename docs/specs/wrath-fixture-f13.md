# Wrath fixture batch F13

Bounded native TOC/discovery and test-owned loader specimens. The 27 original retained rows remain the acceptance matrix; additive specimens never replace a missing modern consumer.

## What it must do

- [ ] Preserve the original 27 tests, their assertions, and Retail results.
- [ ] Use original shipped Wrath FrameXML in committed TOC order for native consumers.
- [ ] Exercise original TimeManager/AchievementUI consumers on both profiles and original TradeSkillUI on Wrath after native core prerequisites. Shipped Wrath TOCs declare LoD and no dependencies; simulator execution is not native-client loader proof.
- [ ] Exercise generic empty-body registration, scope discovery, dependency accumulation, and file environment metadata with explicitly test-owned specimens.
- [ ] Keep modern-only consumers enabled and report their observed failures separately.

## How it works

- [Addon loading pipeline](../addon-loading-pipeline.md)
- [F13 fixture diagnosis](../wiki/investigations/wrath-fixture-f13.md)

## Implementation inventory

- `tests/loader_specimens_f13.rs`: additive test-owned specimens and original shipped profile consumers; not replacement credit.
- `tests/toc_parsing.rs`: generic absolute/existing paths and Lua/XML counts select native Wrath FrameXML TOC; assertions unchanged.
- `tests/load_order.rs`: load native profile FrameXML before the original PaperDoll symbol assertion; Retail helper is a no-op.

## Tests asserting this spec

- `tests/loader_specimens_f13.rs`
- `tests/toc_parsing.rs`
- `tests/load_order.rs`

## Known gaps (current cycle)

- [ ] Final after matrices, stock startup and warning-free checks remain pending. Fresh before: Wrath0/27PASS; Retail26/27PASS across integration22 and prefork5. Archaeology retains its pre-existing missing Mainline TOC failure. Companion5PASS under each profile, not replacement credit.

## Out of scope

Modern UI/mixin reimplementations, vendor/rilua changes, unrelated producer bugs, assertion relaxation, profile gates hiding failures, and native-client loader claims not established by shipped TOCs.
