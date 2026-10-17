# Era/Anniversary startup producers — A

Finite data and existing modeled inputs required by the installed Vanilla startup consumers. Implementation lives in `src/c_api/classic_startup_data.rs`, `src/c_api/class_catalog.rs`, and existing color/PvP/plain-global producers. Source-derived simulator policies, not native-client parity.

## What it must do

- [ ] R1: publish the existing Mists Classic cast-bar palette before Era/Anniversary consumers construct their type table; preserve RGB and encoded zero alpha. Leave Wrath unchanged pending a consuming source.
- [ ] R3: publish Poor=0 and nine quality values (cached ItemQualitiesDocumentation.lua:6-21), supply finite C_Item RGB from existing quality data and Enum.BagIndex.Bank=-1 from cached BagIndexConstantsDocumentation.lua:29, and allow original Constants/UIParent/UnitPopup/Bank consumers to finish; do not inject downstream bank/difficulty aliases.
- [ ] R4: expose nine compact Vanilla enumeration entries with canonical IDs 1–5,7–9,11; C_CreatureInfo uses canonical IDs, not compact indexes. Preserve Druid, exclude later classes, and return nil for unsupported Vanilla IDs. Localized class lists use the same catalog. Non-Vanilla canonical lookup behavior remains unchanged; Mists retains eleven enumeration entries.
- [ ] R6: expose existing honor state to Era/Anniversary. Yesterday projects three values including a supplied contribution (default zero); Mists retains its two-value tuple. Honor shown state follows the supplied capability, default disabled as before.
- [ ] R12: register only the existing raid-marker capability accessor on Era/Anniversary, retaining the existing enabled simulator default and supplied true/false inputs. Do not register the other Retail-only globals. Mists keeps its existing disabled compatibility default.
- [ ] R13: publish ClassicPreWrath.MAX_QUEST_WATCHES=5 from cached QuestConstantsDocumentation.lua:154-158; let Vanilla Constants derive MAX_WATCHABLE_QUESTS and the original consumer enforce capacity/replacement.

## How it works

- [Client profiles](../wiki/systems/client-profiles.md)
- [Lua API](../lua-api.md)

## Implementation inventory

- `src/c_api/classic_startup_data.rs`: finite quality API and quality/quest constants, initialized before vendor loading.
- `src/c_api/class_catalog.rs`: selected enumeration and canonical ID lookup.
- `src/lua_api/globals/font_strings_collection/colors.rs`: shared Classic palette selection.
- `src/lua_api/globals/pvp_probes.rs`, `state_types/pvp.rs`: honor registration and contribution projection.
- `src/lua_api/globals/real/publication_12_0_0.rs`: shared capability accessor; unrelated registrations remain Retail-gated.
- `src/lua_api/globals/register.rs`, `real/mod.rs`, `state.rs`, `state/sim_state.rs`: shared capability storage and narrow registration.
- `src/era/compat_bootstrap.rs`, `compat_bootstrap.lua`: finite producer initialization and retired yesterday shim.
- `src/lua_api/globals/social_probes.rs`, `missing_surface/creature_info.rs`: global enumeration and canonical C API consumer of the same model.

## Tests asserting this spec

- `tests/classic_startup_a.rs`: six original-cache consumer cases, extended with supplied false capability and nonzero yesterday contribution after their Rust storage is available. Existing Mists cast/color/class/honor and Retail plain-global/social tests cover preservation; saved execution receipts follow below.

## Known gaps (current cycle)

- [x] Bounded runtime proof: Era and Anniversary each fail all six original consumers at test revision `fcd05ae2c`, then pass all six at production/test revision `9a454578d` (normal `sound,gui,casc,client-<profile>` libraries). This does not establish native or complete startup parity.
- [x] Preservation: existing Mists cast colors (2), cast fade (1), class colors (2), honor (3), Retail plain globals (8) and social probes (17) pass. Retail `cargo check --no-default-features --features sound,gui,casc,client-retail` and `cargo fmt --check` pass on `9a454578d`.
- [x] Separate rebased sealed startup runs: both profiles report 36 unique / 38 total errors versus committed 92 / 98 baselines, exit 1. Original persisted CVars unchanged; each runtime uses a fresh byte-exact child XDG copy. The historical 38/46 receipt below was rejected for two new CombatLog records; [empty-history repair](classic-empty-combatlog-history.md) removes them.
- [ ] Startup still fails. In particular CompactRaidFrameManager:84 reaches a different nil call than the bounded R12 fixture; attribution remains open.
- [ ] Consumer cases load original Lua chunks and drive handlers on real widgets or presentation fixtures, not the whole startup/XML dependency graph. Full startup-order coverage remains open.
- [ ] Watch login uses the existing three static legacy watches. Five/six capacity uses supplied UI-owned watch records, not six state-backed legacy quest memberships.
- [ ] Native Vanilla class-enumeration indexing and native palette RGBA are not established. Compact enumeration is an explicit simulator choice grounded in the shipped nine-class set.
- [ ] Existing item RGB values are reused, not newly acquired native measurements. Invalid-quality behavior is outside this startup contract.

## Rebased acceptance after CombatLog rejection

`classic-startup` was rebased onto master `275f9f09b`. The module conflict retains both `currency_list` and the A modules; state merges retain master's added fields and A's shared host storage. Production `0c6636255` repairs only empty history; later commits change tests/docs only. Populated replay remains blocked.

| Gate | Saved evidence under private classic-build root |
|---|---|
| New original-login RED | `classic-a-rebased-era-history-red-20261017T002354710646Z`, `classic-a-rebased-anniv-history-red-20261017T003746365651Z`: exact original :1705 → :610 → :597 → UIParent:381 chain |
| Anniversary consumers | `classic-a-rebased-anniv-final-20261017T004617362770Z/six-consumers`: 6/6; `classic-a-anniv-history-final-20261017T005410144190Z/empty-history`: 1/1 |
| Era consumers | `classic-a-rebased-era-final-20261017T010018428476Z`: six-consumers 6/6, empty-history 1/1 |
| Startup | `{era,anniversary}-a-rebased-startup-summary.json`: each 36/38, no string.sub records; only new records are the explicitly disclosed R12 :84 pair after normalizing StoreUI frame IDs |

Normal Anniversary binary/library revision `0c6636255`; Era revision `3c240bcc8` has identical production inputs. Final history test source `e2dab9508` removes only the parent-approved inaccessible progress-frame lookup; error, query, completion and OnUpdate assertions remain. Existing Mists/Retail class/palette/honor/plain-global preservation below remains attributed to its historical revision, not claimed as a fresh integrated-tree run.

## Historical saved proof ledger (before rejection)

Private evidence root: `/home/osso/.local/state/wow-ui-sim/handoff/classic-build/` (0700; report 0600). Builds use the assigned reusable `slot-3`, pressure-guarded launcher, emitted feature/artifact records and captured Cargo/rustc versions. No full test suite was run.

| Proof | Saved epoch / result |
|---|---|
| Era RED | `classic-a-era-red-consumers-20261016T225122002761Z/a-red`: 0/6, Rust test exit 101; normal library `fcd05ae2c` |
| Anniversary RED | `classic-a-anniversary-red-consumers-20261016T225702871989Z/a-red`: 0/6; same source |
| Era GREEN | `classic-a-era-final-20261016T233603903613Z/a-green`: 6/6; normal library/test source `9a454578d` |
| Anniversary GREEN | `classic-a-anniversary-final-20261016T232755151867Z/a-green`: 6/6; same source |
| Mists preservation | `classic-a-mists-preservation-20261016T233630295384Z`: cast-colors, cast-fade, class-colors, honor all exit 0; 8 logical tests (timeout child summaries are not additional tests) |
| Retail preservation | `classic-a-retail-preservation-20261016T234650693396Z`: plain-globals 8/8, social-probes 17/17 |
| Retail check / format | `classic-a-retail-check-20261017T000048380432Z`, `a-format.json`: exit 0 |
| Startup | `{era,anniversary}-a-startup-summary.json` link exact final normal binary epochs, each 38/46, exit 1 |

Initial producer compile failure (missing `LuaApiMut` import) and original-bank-handler failure are retained. The bank traceback reaches `SetID(BANK_CONTAINER)` because the shared Retail enum omitted `Bank`; publishing documented `Enum.BagIndex.Bank=-1` before the original Constants chunk fixes the producer rather than injecting a downstream alias. Admission-denied component epochs are not execution evidence; already compiled components were retained and run only after admission. No proof was rerun solely for a later milestone.

## Out of scope

Sibling B's roots and sites; Blizzard Lua modifications; native parity; baselines or asset manifests; manual unsupported LOD loads; expanding Wrath's class/honor surface.
