# Wrath native chat layout preservation

Wrath startup keeps the geometry and editbox state produced by original `FloatingChatFrame`/`ChatFrame` consumers. Simulator preparation lives in `src/lua_api/chat_init.rs`; [diagnosis](../wiki/investigations/wrath-chat-layout.md) records the boundary and source evidence.

## What it must do

- [x] Preparation before third-party addons preserves native chat dimensions, anchor, editbox dimensions and visibility.
- [x] Post-load and post-event preparation do not replace that native surface with simulator-authored modern geometry.
- [x] Headless normalization does not impose the modern editbox width on Wrath.
- [x] Retail chat preparation and layout remain unchanged in the bounded preservation cases.

## How it works

- [Native chat diagnosis](../wiki/investigations/wrath-chat-layout.md).

## Implementation inventory

- `src/lua_api/chat_init.rs` — native-layout policy and simulator-owned normalization.
- `src/lua_api/workarounds/temporary/post_event_frame_layout.rs` — post-event normalization caller.
- `src/startup.rs` — headless normalization caller.

## Tests asserting this spec

- `tests/chat_frame/native_layout.rs` — original native load followed by pre-addon/post-load/post-event preparation, comparing observable surface snapshots.
- `tests/chat_frame/layout_lock.rs`, `tests/chat_frame/scrollbar.rs` — original native chat/scroll-control geometry.
- `tests/chat_frame.rs` — chat input and Retail preservation cases.

Bounded proof (2026-10-20): the corrected pre-addon original-consumer regression fails at `60fcdf225` and passes at `9bb916545`; subsequent code delta through `70e2d01cb` changes only two independent test oracles. Stock Wrath headless assertions observe native chat `430x120`, editbox `440x32`, shown=true and an empty Lua-error array. All twelve Retail chat cases pass. The 88-case Retail preservation scope has 74 passes and fourteen failures whose first messages reproduce identically on exact base `53bc1e85d`; no new failing cases were introduced. Whole-project formatting and full-feature Wrath integration checking pass with zero compiler warnings. Receipts: `~/.local/state/wow-ui-sim/handoff/wrath-fixture-f02/proof-ledger.json`.

## Known gaps (current cycle)

No remaining gap in this bounded preservation surface. The retained Retail fixture failures are not repaired by this contract.

## Out of scope

Native client execution, saved-chat-layout persistence, other profiles' native geometry and unrelated chat protocol changes are not established by this bounded original-consumer regression.
