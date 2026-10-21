# Wrath legacy LoD APIs

Wrath-only APIs used by unchanged 3.3.5a LoD addons. Native static evidence: laptop clean-room report `wrath-legacy-lod-apis-12340-20261020.md` (handoff `laptop-labels-answers-20261018`), reconstructed 12340 revision `0e4e9fe0a`. No native runtime proof. [Consumer investigation](../wiki/investigations/wrath-addon-load-fixture.md) owns the earlier failure inventory.

## What it must do

- [x] Derive the auction dataset read-only from authenticated 12340 enUS WDBC inputs, checking exact hashes/header lengths and preserving physical subclass record order. No raw DBCs or alternate-locale fallback.
- [ ] `GetAuctionItemClasses` follows native selector order and skips missing class records; `GetAuctionItemSubClasses` validates numeric selectors, excludes displayFlags bit 2 and returns same-locale verbose/display names in DB order.
- [ ] `GetAuctionInvTypes` validates both numeric selectors, uses flags bit `0x200` for a found subclass and preserves native unfound-ordinal fall-through, returning 14 token/1-or-nil pairs in native order.
- [x] `GetActiveTalentGroup` returns one cached group plus one; inspect precedes pet, pet returns one. Arguments follow native StringToBOOL, not Lua truthiness. No usage error.
- [ ] `GetNumTalentGroups` returns one cached player/inspect count, including zero; inspect precedes pet. Pet returns 1 only when its cached talent-tab presence and tab count are both nonzero, otherwise 0, not the tab count. Same StringToBOOL arguments, no usage error, writes or events.
- [x] `CreateMiniWorldMapArrowFrame(parent)` validates table/slot-zero/frame type, emits exact native errors, returns zero values and lazily creates two named child Models. Subsequent calls retain their original parents. Model mesh rendering is unsupported.
- [x] `CalendarEventGetTypes` returns five loaded localized Calendar labels in native order; ignores arguments.
- [ ] `CalendarEventGetRepeatOptions` returns four localized strings in order: `CALENDAR_REPEAT_NEVER`, `CALENDAR_REPEAT_WEEKLY`, `CALENDAR_REPEAT_BIWEEKLY`, `CALENDAR_REPEAT_MONTHLY`; ignores arguments, no writes/events.
- [ ] `CalendarCanSendInvite` returns one boolean: pending action denies; otherwise deny only when throttle bypass is false, last invite is nonzero and elapsed milliseconds are below 2000. At 2000 ms it allows. Ignores args, no player gate, writes or events.
- [x] `CalendarDefaultGuildFilter` returns level cap twice (60/70/80 by account expansion) and the cached guild-rank count directly (`CGGuildInfo::m_numRanks`, laptop clarification); no guild-name membership gate. Absent player returns three zeros. Ignores arguments.
- [x] `ArenaTeamRoster` accepts Lua-numeric values/strings, otherwise errors `Usage: ArenaTeamRoster(team)`; no modeled team => zero returns, no roster, query or immediate event.
- [x] Trade-skill subclass/inventory setters accept number/string arg1; zero/negative/nonnumeric-string ordinals reset, return zero values and emit `TRADE_SKILL_UPDATE`. Subclass reset stores both subclass values as `0xFFFFFFFF` and inventory mask 1; inventory reset stores `0xFFFFFFFF`.
- [x] With no profession filter catalog, positive ordinals fail native `Bad sub class ...` / `Bad inv slot ...` validation before inspecting on/off. Invalid arg1 types use exact native usage messages.
- [ ] `GetTradeSkillSubClasses` and `GetTradeSkillInvSlots` return zero values (not nil) for the fresh empty legacy catalog, ignore args and emit no events. No window-open check; populated retained catalogs after close remain outside the modeled slice.
- [x] Wrath publishes `random = math.random` before Calendar captures its local; no `randomseed` alias is introduced.
- [ ] Preserve Retail publication and other profiles.

## How it works

- [Legacy surface](../wiki/systems/lua-api.md#wrath-selected-quest-legacy-surface).

## Implementation inventory

- `tools/gen_wrath_auction_catalog.py`: hash-pinned read-only WDBC decoding and minimal projection.
- `data/wrath-auction-catalog.json`, `data/wrath-auction-catalog.provenance.json`: 12 class records, native selectors and 112 relevant subclass records, with source/derived hashes and loader-layout provenance.
- `src/lua_api/globals/real/wrath_auction_catalog.rs`: DBC-backed selectors/names, numeric validation, native sparse inventory pairs and ordinal fall-through. The old numeric Mists adapter remains Mists-only, not a Wrath fallback.
- `src/lua_api/globals/real/wrath_talent_groups.rs`: cached group/count projections, pet predicate and StringToBOOL.
- `src/lua_api/globals/real/wrath_legacy_state.rs`: supplied zero-based player/inspect indices, independently mutable from fixtures.
- `src/wrath/compat_bootstrap.rs`: Wrath-only registration.
- `src/lua_api/globals/real/wrath_legacy.rs`: native math alias publication before original consumer chunks.
- `src/lua_api/workarounds/temporary/wrath_trade_skill_filters.rs`: determined resets/validation and native fresh-empty catalog reads, with observable cached filter masks and update event.
- `src/lua_api/workarounds/temporary/wrath_arena_roster.rs`: native no-team branch until membership/query lifecycle exists. Reuses existing Wrath numeric-string parser.
- `src/lua_api/globals/real/wrath_calendar.rs`: localized event/repeat label catalogs, cached context and read-only invite query using the existing GetTime monotonic clock.
- `src/lua_api/globals/real/wrath_minimap_arrows.rs`: parent validation and lazy child ownership.

## Tests asserting this spec

- `tools/tests/test_gen_wrath_auction_catalog.py`: digest rejection, raw record order, input immutability, string offsets, distinct flags and same-locale name projection.
- `tests/wrath_legacy_lod_auction.rs`: original AuctionUI line 380, literal enUS catalog/order, hidden rows, sparse inventory pairs and numeric/error/zero-return semantics.
- `tests/wrath_legacy_lod_apis.rs`: original TalentUI line 266, inspect/pet precedence, argument conversion and environment-local cache.
- `tests/wrath_legacy_talent_group_counts.rs`: original line 267, stored zero counts, inspect priority, pet 0/1 predicate, argument conversion and no events/writes.
- `tests/wrath_calendar_repeat_options.rs`: original repeat dropdown initializer at 3811, exact count/order and no events.
- `tests/wrath_calendar_invite.rs`: original line 4644, pending precedence, pure model 1999/2000-ms boundaries, public monotonic-clock query and no writes/events.
- `tests/wrath_trade_skill_catalog.rs`: unchanged dropdown callers at 470/515, fresh/reset empty catalog cardinality, ignored arguments and getter event silence.
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

## Authenticated auction data layout

User-supplied data-only client uses enUS and matching `wow-data`/`wow-enus` component manifests at build 12340. Latest containing archives are `Data/enUS/patch-enUS.MPQ` (ItemClass) and `Data/enUS/patch-enUS-3.MPQ` (ItemSubClass). Raw files stay outside Git; the derived projection is reproducible with `python3 tools/gen_wrath_auction_catalog.py /path/to/DBCs`.

| WDBC | Records × fields | Record bytes / string bytes | Used zero-based fields |
|---|---|---|---|
| ItemClass | 17 × 20 | 80 / 158 | classID 0; enUS className 3 |
| ItemSubClass | 119 × 44 | 176 / 1226 | classID 0; subclassID 1; flags 4; displayFlags 5; enUS displayName 10; enUS verboseName 27 |

Class 2 field 3 points at string-block offset 22, `Weapon`. Weapon subclass 0 field 10 points at 190 (`Axe`), while field 27 points at 194 (`One-Handed Axes`). The loader mappings are **established**, not inferred: laptop clean-room `ItemClassRec.cpp:21–40,66`, `ItemSubClassRec.cpp:25–68,118–119`; getters `AuctionHouse_Scripts.cpp:98,124–127,160–163`. Locale fields are base plus client locale; enUS index 0, enGB maps to enUS, with no alternate-locale fallback. Only the supplied enUS projection is shipped.

Inventory ordinal lookup is established to use the same visible displayFlags-bit-2-filtered stream (`AuctionHouse_Scripts.cpp:143–191`, laptop clean-room follow-up). Weapon ordinal 18 is unfound because only 17 rows are visible: it returns **28 values / 14 pairs**, all availability numbers 1. A found visible row without flags bit `0x200` returns zero values. No ordinal-sorting or out-of-range empty-result shortcut is permitted.

Input SHA256: ItemClass `f14b09254010765ab75c3c04ddf55a4b03e75163655c5015d188b04e31fa1b20`; ItemSubClass `ed037619b6277fb2b05c3cf05233b19687a4004427d5e63a12d038804bc8b0e5`. Full metadata and derived hash live in the provenance JSON. The decoder walks little-endian 32-bit fields using the authenticated record strides and bounded NUL-terminated UTF-8 string offsets. The projection retains hidden rows for auditable flags but omits unused columns/classes; verboseName/displayName selection occurs in the same locale. Consumable subclass IDs stay `0,5,1,2,3,7,6,4,8`, not sorted.

## Known gaps (current cycle)

- [ ] INFERRED defaults: player/inspect group indices zero and counts one (single-group character); pet tab absent/count zero. Native initialization unestablished; count defaults approved by parent. No count clamping or class-specialization substitution.
- [ ] Auction data gap is resolved by supplied, authenticated DBCs; native API producer GREEN remains pending. The earlier UI-only Gethe cache and modern name constants were not used as a substitute catalog.
- [ ] INFERRED arrow lifetime: retain handles in the environment; globals are not used as ownership. Native lifetime/forged identities remain unestablished. Model path uses the supplied `MinimapArrow.mdx` name only, not an inferred directory prefix.
- [ ] INFERRED Calendar default: active player and expansion 2; native initialization unproved. Reuses existing `world.guild_ranks`, not fabricated rank content. Loaded FrameXML GlobalStrings supplies labels; missing labels/invalid supplied expansion are explicit model errors, not claimed native malformed-state behavior.
- [ ] Profession subclass/inventory catalog absent. Existing Mists subclass/inventory getters return zero values; modern blacksmithing recipes lack native subclass flags/inventory masks. Positive selection, missing-on/off validation after valid selection, bit toggles/exclusivity, and recipe recomputation remain held, not approximated.
- [ ] INFERRED Calendar invite initialization: pending false, last timestamp zero and throttle-bypass false. Native CVar public name is unknown, so its nonzero/bypass state is an explicit cached flag, not a guessed CVar. Invite mutations/recording timestamps remain outside the modeled query; native wraparound-period parity is unestablished.
- [ ] INFERRED native guild cache lifecycle: presumably cleared on leaving a guild; unestablished. This wrapper does not repair inconsistent guild membership/rank state.
- [x] Initial slice: stock Wrath/Retail startup both `[]`, Retail direct plus five consumers 6/6 pass, six successful build/check phases emit zero warnings, and fmt passes. See handoff `initial-proof-complete.json`. These receipts predate the auction/follow-up implementation.

## Out of scope

- Fabricated item/profession/arena catalogs; vendor or rilua changes; native runtime claims.
