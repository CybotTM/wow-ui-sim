# Wrath legacy LoD APIs

Wrath-only APIs used by unchanged 3.3.5a LoD addons. Native static evidence: laptop clean-room report `wrath-legacy-lod-apis-12340-20261020.md` (handoff `laptop-labels-answers-20261018`), reconstructed 12340 revision `0e4e9fe0a`. No native runtime proof. [Consumer investigation](../wiki/investigations/wrath-addon-load-fixture.md) owns the earlier failure inventory.

## What it must do

- [ ] `GetActiveTalentGroup` returns one cached group plus one; inspect precedes pet, pet returns one. Arguments follow native StringToBOOL, not Lua truthiness. No usage error.
- [ ] `CreateMiniWorldMapArrowFrame(parent)` validates table/slot-zero/frame type, emits exact native errors, returns zero values and lazily creates two named child Models. Subsequent calls retain their original parents. Model mesh rendering is unsupported.
- [ ] `CalendarEventGetTypes` returns five loaded localized Calendar labels in native order; ignores arguments.
- [ ] `CalendarDefaultGuildFilter` returns level cap twice (60/70/80 by account expansion) and existing guild-rank count; absent player returns three zeros. Ignores arguments.
- [ ] Preserve Retail publication and other profiles.

## How it works

- [Legacy surface](../wiki/systems/lua-api.md#wrath-selected-quest-legacy-surface).

## Implementation inventory

- `src/lua_api/globals/real/wrath_talent_groups.rs`: cached group projection and StringToBOOL.
- `src/lua_api/globals/real/wrath_legacy_state.rs`: supplied zero-based player/inspect indices, independently mutable from fixtures.
- `src/wrath/compat_bootstrap.rs`: Wrath-only registration.
- `src/lua_api/globals/real/wrath_calendar.rs`: localized label and cached context projection.
- `src/lua_api/globals/real/wrath_minimap_arrows.rs`: parent validation and lazy child ownership.

## Tests asserting this spec

- `tests/wrath_legacy_lod_apis.rs`: original TalentUI line 266, inspect/pet precedence, argument conversion and environment-local cache.
- Original retained LoD shards and force-load in `tests/addon_coverage{,_lod}.rs`.

## Known gaps (current cycle)

- [ ] INFERRED default: player/inspect group zero (single-group character); native cache initialization unestablished. Approved by parent.
- [ ] Auction catalog held: no local 12340 ItemClass/ItemSubClass DB export. Modern name constants omit DB row order, display flags, verbose names and missing-record identity. Wrath UI cache is a pinned Gethe UI-only snapshot, not an MPQ client. Exact inventory: handoff `wrath-legacy-apis/auction-catalog-gap.json`.
- [ ] INFERRED arrow lifetime: retain handles in the environment; globals are not used as ownership. Native lifetime/forged identities remain unestablished. Model path uses the supplied `MinimapArrow.mdx` name only, not an inferred directory prefix.
- [ ] INFERRED Calendar default: active player and expansion 2; native initialization unproved. Reuses existing `world.guild_ranks`, not fabricated rank content. Loaded FrameXML GlobalStrings supplies labels; missing labels/invalid supplied expansion are explicit model errors, not claimed native malformed-state behavior.
- [ ] Current-head GREEN and preservation proof pending.

## Out of scope

- Fabricated item/profession/arena catalogs; vendor or rilua changes; native runtime claims.
