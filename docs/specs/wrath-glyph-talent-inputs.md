# Wrath glyph sockets and unspent talent points

Wrath consumers read scenario-supplied glyph sockets and unspent talent points. The [F10 investigation](../wiki/investigations/wrath-spell-panels-f10.md) describes the consumer boundaries.

## What it must do

- [ ] Supply each socket's enabled state, glyph type, spell ID and icon per one-based talent group and socket 1–6.
- [ ] Return six sockets. A supplied enabled socket without a glyph returns enabled 1 and its type, with nil spell and icon. A locked record returns nil enabled, spell and icon, and its supplied type. An absent record or index outside 1–6 returns zero values. These rules follow the aso 3.3.5a comparison; unsupplied records are scenario state, not native initial values.
- [ ] Return supplied unspent points by group, or scenario-default zero when unsupplied. An omitted or non-numeric group selects the active group.
- [ ] Expose the input setters only on Wrath and do not fire events from them.

## How it works

- [Wrath F10 consumer investigation](../wiki/investigations/wrath-spell-panels-f10.md)

## Implementation inventory

- `src/lua_api/globals/admin_spec.rs`: scenario input setters.
- `src/lua_api/globals/real/wrath_talent_groups.rs`: legacy query producers.
- `src/lua_api/state/sim_state.rs`: supplied state.

## Tests asserting this spec

- `tests/common/wrath_spell_panels_f10.rs`: original GlyphUI and TalentUI consumers.

## Known gaps (current cycle)

- [ ] Confirm the two independent original-consumer RED/GREEN pairs and stock startup comparisons.

## Out of scope

- Talent catalogs and native initial allocations are not supplied by these queries.

## Evidence

Wrath `Blizzard_GlyphUI.lua:65` and `Blizzard_TalentUI.lua:499` are the selected consumer call sites. Historical signatures are in `data/patch-api/sources/3.0.2-wikitext-register.json` and `data/patch-api/sources/3.1.0-api-changes.txt:24,34`. Semantics are from the aso 3.3.5a decompilation comparison (`GlyphInfo_Scripts.cpp:31–75`, `TalentInfo_Scripts.cpp:150–155`, `TalentInfo.cpp:220–226,344–352`), not a native runtime capture. Other defaults are simulator scenario policy.
