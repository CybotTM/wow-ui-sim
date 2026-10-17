# Era/Anniversary startup producers — B

Audited startup roots beyond [A](classic-startup-a.md). [Shared shipped strings](classic-global-strings.md) and [marker active-state](classic-marker-state.md) have separate contracts. Simulator empty-state/selection policies were explicitly approved; they are not native defaults.

## What it must do

- [ ] R5: preserve actual required-LoD dependency loading and ordering, but do not force the Vanilla Transmog LoD addon into startup. Manual Transmog correctness is not claimed.
- [ ] R7/R8/R9: temporary empty product choices return a fresh collection; empty loot history returns numeric zero and original consumers recycle frames; unmodeled tracking returns nil and original consumer hides the icon. No populated behavior is implied.
- [ ] R11: project one-based roster selection over the existing guild member vector; zero means none, invalid/stale selection projects zero. This reset policy is approved simulator policy, not native evidence.
- [ ] R15: Classic CombatLogBase consumes public FrameXML masks without being replayed into the isolated secure environment; original color/filter values exist, public/secure writes remain isolated.

- [ ] R17: Publish only the existing real quest-background getter on Era/Anniversary. Original QuestFrame chooses Parchment for absent material and receives an explicit supplied map value unchanged; do not enable the rest of LIVE_QUEST_GLOBALS.

- [ ] R18: Exclude Retail watched-faction fixture seeding from Era/Anniversary. New state has zero watched ID and empty factions; existing supplied state survives unchanged. Retail/PTR retain all original fixture values. Original ReputationBar receives empty watched data, without inventing REPUTATION_STATUS_BAR_LABEL_ACCOUNT_WIDE.

- [ ] R19: Scope Classic expansion level to the selected per-profile cache/runtime, so original Vanilla quest rewards do not enter the TBC-only named-child path. Preserve every other profile's current expansion defaults; no missing XML child is fabricated.

## How it works

- [Addon pipeline](../addon-loading-pipeline.md)
- [Client profiles](../wiki/systems/client-profiles.md)

## Implementation inventory

- R5: `src/loader/mod.rs` — selected-profile implicit startup roots.
- R15: `src/loader/addon.rs` — Mists/Era/Anniversary CombatLogBase remains public-only.
- R7/R8/R9: `src/lua_api/workarounds/temporary/classic_startup_defaults.rs` — R7 fresh empty choices, R8 numeric zero loot, R9 nil tracking texture.
- R11: `src/lua_api/globals/real/legacy_guild_selection.rs` — existing guild-vector selection, zero/invalid/stale→none policy.

- R18: `src/lua_api/workarounds/temporary/reputation_state.rs` — selected-profile population only for newly created fixture state.
- R17: `src/lua_api/globals/quest_surface/register.rs` — narrow publication of existing selected-quest map getter.

- R19: `src/lua_api/workarounds/temporary/client_info_defaults.rs` — selected Vanilla expansion0, other profiles unchanged.

## Tests asserting this spec

`tests/classic_startup_b.rs` contains separate original consumer cases with `classic_startup_b_pins.tsv`. Both-profile RED observed: R5 startup discovers forced Transmog; R7 nil collection length; R8 nil numeric bound; R9 missing global; R11 missing selection API; R15 CombatLogBase replay error. Independent required-LoD preservation passes before producer. Tests/commits remain root-specific; groups share one sealed normal library without repeating logical tests.

## R19 selected-profile evidence

This concerns the selected runtime, not Anniversary realms generally or future TBC progression.

| Profile | Its own cache provenance | Actual QuestLog route / Interface evidence |
|---|---|---|
| Era | `AddOns/.wow-ui-sim-blizzard-ui-provenance`: `product=wow_classic_era`, `version=1.15.9.70003`, `build_key=2417084b8b041ee120d02d4adf31b0bc` | Manifest ships Vanilla/TBC/Wrath/Cata variants; selected `Blizzard_UIPanels_Game_Classic.toc:50` uses `[Game]\\QuestLogFrame.xml`; active loader maps Era to Vanilla. That selected XML lacks QuestLogHonorFrame/QuestLogPlayerTitleFrame. TOC declares no Interface; incidental other TOC values0/30001 are not build evidence. Simulator interface11507 is Vanilla-era, not a measured11509 declaration. |
| Anniversary | Its separate provenance file independently records the same product/version/key | Its separate manifest and TOC match Era and the current loader maps Anniversary to Vanilla; same selected XML absence. No claim about a 2.5.x/TBC Anniversary client. Same absent TOC Interface, same simulator11507. |

Both manifest hashes are `6647aded693f013c7ed4e1e1e613450558a781da1bf8a0494d8887e1f7810978`. Both caches' generated `ExpansionInfoDocumentation.lua` declares the existing Classic expansion queries; `ExpansionLevelConstantsDocumentation.lua:13-14,23` declares None0/BurningCrusade1/WarWithin10. Before fix the shared constant10 made `QuestFrame.lua:497,545-549` take TBC branches against Vanilla XML. Only TBC/QuestLogFrame.xml:1007,1016 declares those legacy named frames. The original startup/detail test reproduces the nil honorFrame on both profiles before projection changes. Re-evaluate this profile selection when a selected product/manifest/loader route actually changes; do not infer live realm progression from the profile name.

## Known gaps (current cycle)

- [ ] GREEN, preservation and final startup capture pending.
- [ ] R10 shared skills and R14 shared zero-result timers await wrath-fixes landing; no duplicate model/producer introduced.
- [ ] Replacing temporary domains with real reward/roll/tracking/timer state is required before populated claims. Retire each endpoint when its backing system exists.
- [ ] R17 quest-material records were separately attributed and authorized; GREEN pending. R18 reputation-fixture scoping is implemented, GREEN/preservation pending. Historical unmasking disclosure remains; R16 Store cursor attribution stays unstaged.

## Out of scope

Blizzard Lua changes, native parity, populated product/loot/tracking/timer domains, complete guild/skill XML lifecycle, speculative Store notification policy, and manual unsupported Transmog loading.
