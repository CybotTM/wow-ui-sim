# Wrath single-tab spellbook views

Wrath's legacy spellbook globals expose full and compact views of existing `SimState.known_spells` membership. Implementation: `src/lua_api/globals/real/wrath_spellbook.rs`. The authenticated 12340 Q-SB1/Q-SB1b contract comes from independently verified laptop source reconstruction, not native capture. The single class-tab inventory is an approved simulator limitation, not an authenticated native inventory.

## What it must do

### Single-tab projection

- [ ] Keep the existing single class tab and expose exactly six GetSpellTabInfo values: name, icon, zero full offset, full count, zero compact offset, compact count.
- [ ] Derive the full list from known_spells, ordered by name, parsed subtext rank and subtext. ID breaks otherwise equal keys deterministically; no parallel membership or persistent compact store.
- [ ] Compact valid same-name nonzero-rank runs to their highest parsed rank, not their highest spell ID. Rank-zero entries remain separate boundaries.
- [ ] Parse the first nonzero ASCII decimal run, ignoring nondigits while the accumulator is zero. No prefix, locale or sign interpretation; e.g. Rank 2/4 gives 2, Rank 0/4 gives 4.
- [ ] Treat missing SpellInfo/subtext as rank zero (approved INFERRED simulator policy).
- [x] Expose both views independently of ShowAllSpellRanks; original Lua selects the view.

### Legacy bindings

- [x] GetSpellTabInfo accepts numbers/numeric strings, truncates ordinary finite fractions before indexing, and ignores extra arguments.
- [x] Outside the sole tab, return exactly nil,nil,0,0,0,0. Reject missing/nil/boolean/table/nonnumeric-string arguments with Usage: GetSpellTabInfo(index).
- [x] GetKnownSlotFromHighestRankSlot maps a one-based global compact slot to the first matching one-based full known slot, returning exactly one number or zero on failure, never nil.
- [x] UpdateSpells ignores arguments, returns zero values and synchronously emits argless SPELLS_CHANGED unconditionally. Derived views require no persistent rebuild.
- [x] Original Wrath SpellBookFrame show/select/hide produces numeric selectedSkillLineOffset and zero Lua errors.
- [ ] Retail and every other profile preserve existing spellbook behavior.

## How it works

- [Lua API architecture](../lua-api.md)
- [Event dispatch](../event-system.md)

## Implementation inventory

- `src/lua_api/globals/real/wrath_spellbook.rs` — Wrath-only ordered/compact projection, tab/mapping bindings and synchronous UpdateSpells; reuses existing libc numeric conversion for Lua-style numeric strings.
- `src/lua_api/globals/real/spell_tabs.rs` — Wrath registration selects its own producer; other-profile function bodies remain unchanged.
- `src/lua_api/globals/real/mod.rs` — Wrath-only module gate.
- `src/lua_api/state/sim_state.rs` — existing known_spells membership, unchanged.

## Tests asserting this spec

- `tests/wrath_spellbook_tabinfo.rs` — unchanged original startup and SpellBook lifecycle.
- `tests/wrath_spellbook_boundaries.rs` — approved rank-zero/missing-record policy, one-tab consistency and on-demand membership changes; execution pending.
- `tests/wrath_spellbook_contract.rs` — focused tab binding, compact-slot mapping and synchronous UpdateSpells contracts. Ranked fixture Ebon Crown IDs 1248891/1248890/1248888 have ascending ranks 1/2/4 but descending IDs.
- `tests/spell_api.rs`, `tests/professions_spellbook.rs` — existing C_SpellBook/profile preservation.

## Bounded producer proof — 2026-10-19

Original lifecycle RED ran at exact f71f7ce1b; API3 RED used fixture18089b84e against identical f71 production sources. After separate producer commits f7a564083/cfd036547, source9a4b109bc passed API3/3 plus unchanged original lifecycle1/1. Original startup and lifecycle both had zero collected Lua errors, without filtering, and no cursor :365 failure. The lifecycle uses the observed default empty known-spell membership; populated original-consumer rendering is not claimed. Build warnings0; original CVar full stat/hash unchanged. Private receipts: handoff/wrath-spellbook-tabinfo/green-{api,original}-receipt.json under the user state root. Later documentation and boundary-test additions do not change producer source.

## Known gaps (current cycle)

- [ ] Boundary coverage b4ff67e19 and seven-profile preservation/stock Wrath CLI execution pending; producer4/4 GREEN does not establish those gates.
- [ ] Independent cursor follow-up OPEN and excluded from this slice. Final authenticated Q-CUR1 (`cursor-onshow-ordering-12340.md` in laptop handoff answers) supports only conditional deferred dirty flushing, not synchronous SetText/OnShow cursor callbacks. Historical fixture4d20f7625 and observer called DebugTools updates back to back with NO intervening ticks; both reached original :365 nil cursorOffset, but do not prove a native violation. That unsupported synchronous assertion was removed from the shippable suite without an ignore; exact source/trace remain in handoff. Follow-up needs realistic intervening ticks and original-consumer deferred-flush RED before any producer fix.
- [ ] Stock Wrath startup [] and existing other-profile spellbook preservation after integration.

## Out of scope

- UNMODELED: multiple Wrath skill lines, cross-tab offsets, General-tab semantics, per-tab icon metadata and spell-level display-sort fallback. Real support needs supplied Wrath metadata inputs in a future slice.
- Native look-ahead/tail anomaly: explicitly excluded by Q-SB1; no emulation of an unexecuted static anomaly.
- Exceptional/overflow index conversions and uncertain compact-slot fractional rounding: not authenticated. Rank overflow is unmodeled; current committed metadata parses to ranks at most 4.
- Pet spellbook, spell learning, and full-list refresh/sort events beyond the approved derived projection.
