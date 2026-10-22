# Wrath fixture batch F13

Bounded native TOC/discovery and test-owned loader specimens. The 27 original retained rows remain the acceptance matrix; additive specimens never replace a missing modern consumer.

## What it must do

- [ ] Preserve the original 27 tests, their assertions, and Retail results.
- [ ] Use original shipped Wrath FrameXML in committed TOC order for native consumers.
- [ ] Exercise generic empty-body registration, scope discovery, dependency accumulation, and file environment metadata with explicitly test-owned specimens.
- [ ] Keep modern-only consumers enabled and report their observed failures separately.

## How it works

- [Addon loading pipeline](../addon-loading-pipeline.md)

## Implementation inventory

- `tests/loader_specimens_f13.rs`: additive test-owned specimens; no Blizzard behavior claims.

## Tests asserting this spec

- `tests/loader_specimens_f13.rs`
- `tests/toc_parsing.rs`
- `tests/load_order.rs`

## Known gaps (current cycle)

- [ ] Exact before/after matrices and native fixture repairs await build capacity.

## Out of scope

Modern UI/mixin reimplementations, vendor/rilua changes, unrelated producer bugs, assertion relaxation, profile gates hiding failures, and native-client loader claims not established by shipped TOCs.
