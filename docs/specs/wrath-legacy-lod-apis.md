# Wrath legacy LoD APIs

Wrath-only APIs used by unchanged 3.3.5a LoD addons. Native static evidence: laptop clean-room report `wrath-legacy-lod-apis-12340-20261020.md` (handoff `laptop-labels-answers-20261018`), reconstructed 12340 revision `0e4e9fe0a`. No native runtime proof. [Consumer investigation](../wiki/investigations/wrath-addon-load-fixture.md) owns the earlier failure inventory.

## What it must do

- [x] `GetActiveTalentGroup` returns one cached group plus one; inspect precedes pet, pet returns one. Arguments follow native StringToBOOL, not Lua truthiness. No usage error.
- [x] `CreateMiniWorldMapArrowFrame(parent)` validates table/slot-zero/frame type, emits exact native errors, returns zero values and lazily creates two named child Models. Subsequent calls retain their original parents. Model mesh rendering is unsupported.
- [x] `CalendarEventGetTypes` returns five loaded localized Calendar labels in native order; ignores arguments.
- [x] `CalendarDefaultGuildFilter` returns level cap twice (60/70/80 by account expansion) and the cached guild-rank count directly (`CGGuildInfo::m_numRanks`, laptop clarification); no guild-name membership gate. Absent player returns three zeros. Ignores arguments.
- [x] `ArenaTeamRoster` accepts Lua-numeric values/strings, otherwise errors `Usage: ArenaTeamRoster(team)`; no modeled team => zero returns, no roster, query or immediate event.
- [x] Trade-skill subclass/inventory setters accept number/string arg1; zero/negative/nonnumeric-string ordinals reset, return zero values and emit `TRADE_SKILL_UPDATE`. Subclass reset stores both subclass values as `0xFFFFFFFF` and inventory mask 1; inventory reset stores `0xFFFFFFFF`.
- [x] With no profession filter catalog, positive ordinals fail native `Bad sub class ...` / `Bad inv slot ...` validation before inspecting on/off. Invalid arg1 types use exact native usage messages.
- [x] Wrath publishes `random = math.random` before Calendar captures its local; no `randomseed` alias is introduced.
- [ ] Preserve Retail publication and other profiles.

## How it works

- [Legacy surface](../wiki/systems/lua-api.md#wrath-selected-quest-legacy-surface).

## Implementation inventory

- `src/lua_api/globals/real/wrath_talent_groups.rs`: cached group projection and StringToBOOL.
- `src/lua_api/globals/real/wrath_legacy_state.rs`: supplied zero-based player/inspect indices, independently mutable from fixtures.
- `src/wrath/compat_bootstrap.rs`: Wrath-only registration.
- `src/lua_api/globals/real/wrath_legacy.rs`: native math alias publication before original consumer chunks.
- `src/lua_api/workarounds/temporary/wrath_trade_skill_filters.rs`: determined reset and empty-catalog validation, with observable cached masks and update event.
- `src/lua_api/workarounds/temporary/wrath_arena_roster.rs`: native no-team branch until membership/query lifecycle exists. Reuses existing Wrath numeric-string parser.
- `src/lua_api/globals/real/wrath_calendar.rs`: localized label and cached context projection.
- `src/lua_api/globals/real/wrath_minimap_arrows.rs`: parent validation and lazy child ownership.

## Tests asserting this spec

- `tests/wrath_legacy_lod_apis.rs`: original TalentUI line 266, inspect/pet precedence, argument conversion and environment-local cache.
- Original retained LoD shards and force-load in `tests/addon_coverage{,_lod}.rs`.

### Bounded Wrath proof — 2026-10-20

At `4270e9876`, six exact integration cases cover eight published functions/aliases: all pass, one executed test each. Full-feature Wrath build uses `CARGO_INCREMENTAL=0`, emits zero compiler warnings and embeds 1,423 own-worktree test paths with no alien paths. Stock/profile-preservation gates remain pending. Hand-off `wrath-legacy-apis/proof-ledger.json` owns commands, revisions, hashes, private CVar snapshots and raw outputs.

Five original consumer cases were re-run against the unchanged error budgets; each executes one test and remains RED for held or newly reached gaps:

| Consumer | Before → after unique errors (affected addons) | Retained boundary |
|---|---|---|
| Force-load all | BattlefieldMinimap 2→0; Calendar 8→4; TalentUI 8→8; TradeSkillUI 4→4; AuctionUI 1→1 | Same held catalog plus downstream functions below; unrelated errors unchanged. |
| Shard 10 | AuctionUI 1→1 | Held auction catalog at line 380. |
| Shard 11 | BattlefieldMinimap 1→0; TalentUI 4→4 | `GetNumTalentGroups` at lines 267/971, after active-group call succeeds. |
| Shard 13 | Calendar 4→2; TradeSkillUI 2→2 | `CalendarEventGetRepeatOptions` at 3811, `CalendarCanSendInvite` at 4644; trade catalog getters at 470/515. |
| Prior-runtime | AuctionUI 1→1 | Stops at held shard-10 catalog before later shards. |

No consumer failure is hidden, ignored or re-budgeted. These newly reached APIs are outside this slice; no values are fabricated to bypass them.

## Known gaps (current cycle)

- [ ] INFERRED default: player/inspect group zero (single-group character); native cache initialization unestablished. Approved by parent.
- [ ] Auction catalog held until the user supplies 12340 ItemClass/ItemSubClass DBCs. Laptop confirms no static rows in the source: only selectors/flags, with names read from DBCs. Modern name constants omit native DB row order, display flags, verbose names and missing-record identity. Wrath UI cache is a pinned Gethe UI-only snapshot, not an MPQ client. Exact inventory: handoff `wrath-legacy-apis/auction-catalog-gap.json`.
- [ ] INFERRED arrow lifetime: retain handles in the environment; globals are not used as ownership. Native lifetime/forged identities remain unestablished. Model path uses the supplied `MinimapArrow.mdx` name only, not an inferred directory prefix.
- [ ] INFERRED Calendar default: active player and expansion 2; native initialization unproved. Reuses existing `world.guild_ranks`, not fabricated rank content. Loaded FrameXML GlobalStrings supplies labels; missing labels/invalid supplied expansion are explicit model errors, not claimed native malformed-state behavior.
- [ ] Profession subclass/inventory catalog absent. Existing Mists subclass/inventory getters return zero values; modern blacksmithing recipes lack native subclass flags/inventory masks. Positive selection, missing-on/off validation after valid selection, bit toggles/exclusivity, and recipe recomputation remain held, not approximated.
- [ ] INFERRED native guild cache lifecycle: presumably cleared on leaving a guild; unestablished. This wrapper does not repair inconsistent guild membership/rank state.
- [ ] Stock Wrath/Retail startup and Retail preservation gates pending.

## Out of scope

- Fabricated item/profession/arena catalogs; vendor or rilua changes; native runtime claims.
