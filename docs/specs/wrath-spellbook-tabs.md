# Wrath single-tab spellbook views

Wrath's legacy spellbook globals expose full and compact views of existing `SimState.known_spells` membership. Implementation: `src/lua_api/globals/real/wrath_spellbook.rs`. The authenticated 12340 Q-SB1/Q-SB1b contract comes from independently verified laptop source reconstruction, not native capture. The single class-tab inventory is an approved simulator limitation, not an authenticated native inventory.

## What it must do

### Single-tab projection

- [ ] Keep the existing single class tab and expose exactly six GetSpellTabInfo values: name, icon, zero full offset, full count, zero compact offset, compact count.
- [ ] Derive the full list from known_spells, ordered by name, parsed subtext rank and subtext. ID breaks otherwise equal keys deterministically; no parallel membership or persistent compact store.
- [ ] Compact valid same-name nonzero-rank runs to their highest parsed rank, not their highest spell ID. Rank-zero entries remain separate boundaries.
- [ ] Parse the first nonzero ASCII decimal run, ignoring nondigits while the accumulator is zero. No prefix, locale or sign interpretation; e.g. Rank 2/4 gives 2, Rank 0/4 gives 4.
- [ ] Treat missing SpellInfo/subtext as rank zero (approved INFERRED simulator policy).
- [ ] Expose both views independently of ShowAllSpellRanks; original Lua selects the view.

### Legacy bindings

- [ ] GetSpellTabInfo accepts numbers/numeric strings, truncates ordinary finite fractions before indexing, and ignores extra arguments.
- [ ] Outside the sole tab, return exactly nil,nil,0,0,0,0. Reject missing/nil/boolean/table/nonnumeric-string arguments with Usage: GetSpellTabInfo(index).
- [ ] GetKnownSlotFromHighestRankSlot maps a one-based global compact slot to the first matching one-based full known slot, returning exactly one number or zero on failure, never nil.
- [ ] UpdateSpells ignores arguments, returns zero values and synchronously emits argless SPELLS_CHANGED unconditionally. Derived views require no persistent rebuild.
- [ ] Original Wrath SpellBookFrame show/select/hide produces numeric selectedSkillLineOffset and zero Lua errors.
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

- `tests/wrath_spellbook_tabinfo.rs` — unchanged original startup and SpellBook lifecycle; separate original DebugTools cursor case.
- `tests/wrath_spellbook_contract.rs` — focused tab binding, compact-slot mapping and synchronous UpdateSpells contracts. Ranked fixture Ebon Crown IDs 1248891/1248890/1248888 have ascending ranks 1/2/4 but descending IDs.
- `tests/spell_api.rs`, `tests/professions_spellbook.rs` — existing C_SpellBook/profile preservation.

## Known gaps (current cycle)

- [ ] Producer GREEN and preservation evidence pending. Original lifecycle RED: f71f7ce1b; focused API RED: 18089b84e against identical f71 production sources.
- [ ] Independent cursor gap OPEN, waiting on authoritative Q-CUR1 native ordering facts (`cursor-onshow-ordering-12340.md` in laptop handoff answers). Corrected fixture4d20f7625 and read-only hook observer both reached original UIPanelTemplates.lua:365 nil cursorOffset against the frozen f71 library. No prior cursor callback occurred before original DebugTools.lua:449 direct changed/update calls. Initial nil ScriptErrorsFrame failure was setup, not behavioral RED. Cursor producer is excluded from this slice.
- [ ] Stock Wrath startup [] and existing other-profile spellbook preservation after integration.

## Out of scope

- UNMODELED: multiple Wrath skill lines, cross-tab offsets, General-tab semantics, per-tab icon metadata and spell-level display-sort fallback. Real support needs supplied Wrath metadata inputs in a future slice.
- Native look-ahead/tail anomaly: explicitly excluded by Q-SB1; no emulation of an unexecuted static anomaly.
- Exceptional/overflow index conversions and uncertain compact-slot fractional rounding: not authenticated. Rank overflow is unmodeled; current committed metadata parses to ranks at most 4.
- Pet spellbook, spell learning, and full-list refresh/sort events beyond the approved derived projection.
