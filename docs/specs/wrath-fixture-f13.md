# Wrath fixture batch F13

Bounded native TOC/discovery and test-owned loader specimens. The 27 original retained rows remain the acceptance matrix; additive specimens never replace a missing modern consumer.

## What it must do

- [x] Preserve the original 27 tests, their assertions, and Retail results.
- [x] Use original shipped Wrath FrameXML in committed TOC order for native consumers.
- [x] Exercise original TimeManager/AchievementUI consumers on both profiles and original TradeSkillUI on Wrath after native core prerequisites. Shipped Wrath TOCs declare LoD and no dependencies; simulator execution is not native-client loader proof.
- [x] Exercise generic empty-body registration, scope discovery, dependency accumulation, and file environment metadata with explicitly test-owned specimens.
- [x] Keep modern-only consumers enabled and report their observed failures separately.

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

- [ ] Twenty-four modern-only original Wrath rows remain enabled and failing; no replacement or profile-gate credit.

## Verification

Before fixtures at `b0a7368e6`: Wrath0PASS/27FAIL; Retail26PASS/1FAIL. After master integration at `6f54e648f`: Wrath3PASS/24FAIL; Retail26PASS/1FAIL with identical per-row statuses. Retail splits22 integration cases and5 prefork cases. Archaeology retains the pre-existing missing Mainline TOC failure. Every exact invocation executes at least one test. Companion5PASS on each profile before/after, counted separately.

Stock Wrath `--no-addons --no-saved-vars lua-errors` returns byte-exact `[]` plus newline, exit0. Both profile checks (`--lib --test integration --bin wow-sim -j1`) and `cargo fmt --all -- --check` exit0. All build/check scopes emit zero compiler warnings. No producer changes; no producer RED/GREEN claimed.

## Out of scope

Modern UI/mixin reimplementations, vendor/rilua changes, unrelated producer bugs, assertion relaxation, profile gates hiding failures, and native-client loader claims not established by shipped TOCs.
