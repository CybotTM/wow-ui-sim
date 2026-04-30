# Classic-profile addon validation targets (Phase 8.1)

Eight popular community addons picked to validate the wow-ui-sim's
classic-profile coverage. The picks are biased toward Wrath (`Interface:
30300`) and Mists (`Interface: 50500`) per the Phase 8 plan, with one
vanilla pick to keep era/anniversary in scope.

Each entry is intended to surface a different chunk of the simulator's
classic API surface; together they should produce a representative gap
report in Phase 8.3's per-addon stub list.

## Picks

| # | Addon | Profile | Interface | Why this addon |
|--:|-------|---------|----------:|----------------|
| 1 | **Bartender4** (Wrath fork) | wrath | 30300 | Replaces the entire action bar UI. Stresses `ActionButton_*`, `GetActionInfo`, `GetActionTexture`, secure templates, keybinding registration, and `LibActionButton-1.0`. Most-installed action-bar mod across all classic flavors. |
| 2 | **Deadly Boss Mods (DBM-WotLK)** | wrath | 30300 | Combat-event-driven boss timers. Stresses `COMBAT_LOG_EVENT_UNFILTERED` parsing, `RegisterEvent`, `RaidNotice_AddMessage`, sound playback, and `SendChatMessage`. Heaviest event subscriber on a typical raid. |
| 3 | **AtlasLoot Classic-WotLK** | wrath | 30300 | Instance-map and loot DB. Stresses `Item:CreateFromItemID` substitutes, tooltip rendering, dropdown menus, and large static-data tables (~2-5 MB compiled). |
| 4 | **Skada (WotLK fork)** | wrath | 30300 | Modular combat-log meter. Stresses combat-log parsing under load + `LibSharedMedia-3.0`. Lighter than Recount's per-event hooks; better coverage of `LibStub` consumers. |
| 5 | **WeakAuras 2 (MoP Classic)** | mists | 50500 | Generic visual aura engine. Stresses dynamic frame creation, animation API, OnUpdate ticker density, custom-trigger sandboxing, and `LibCompress`/`LibSerialize`. Often the heaviest single addon by frame count. |
| 6 | **ElvUI-Classic-Mists** | mists | 50500 | Full UI replacement (action bars, unit frames, chat, nameplates, datatext). Stresses essentially the entire frame API surface end-to-end; if any other addon works, this one usually surfaces something they don't. |
| 7 | **Details! Damage Meter (MoP Classic)** | mists | 50500 | Combat parser with internal frame pool. Stresses `CombatLogGetCurrentEventInfo`, segment switching, plugin discovery, and ScrollFrame heavy use. |
| 8 | **pfQuest-Vanilla** | era | 11500 | Vanilla quest helper, smaller than Questie. Stresses minimap pin overlays, world-map pin overlays, `GetQuestLog*` family, and `C_Map.GetMapInfo` shims. |

## Pass criterion

For Phase 8.2's harness, each addon passes if `wow-sim --no-saved-vars
run-tests` (or just startup events under `--exec-lua`) completes without a
**fatal** Lua error against the matching profile. Non-fatal `lua-errors`
output is acceptable noise — track it as Phase 8.3 stub work.

## Out of scope (deliberately not in the v1 list)

- **Questie** — too large; its DB-loaders alone would dominate the harness
  runtime. pfQuest covers the same surface more compactly.
- **BigWigs-Classic** — the Wrath fork is unmaintained; DBM-WotLK is the
  active equivalent.
- **Recount** — Skada is its strict superset for our purposes.
- **ElvUI-Classic** (era) — has a long, fragile bootstrap chain; revisit
  after the Mists fork passes.

## Source URLs (to fetch in Phase 8.2)

These are the upstream repositories the harness will pin and clone. Pin a
specific tag/SHA before running, mirroring the pattern in
`scripts/setup-blizzard-ui.sh`.

- Bartender4 (Wrath): https://www.curseforge.com/wow/addons/bartender4 — Wrath/3.3.5 release
- DBM-WotLK: https://github.com/DeadlyBossMods/DBM-Wrath — `master` branch
- AtlasLoot Classic-WotLK: https://github.com/Hoizame/AtlasLootClassic — `wotlk` branch
- Skada (WotLK): https://github.com/bkader/Skada-WoTLK — `master` branch
- WeakAuras 2 (MoP Classic): https://github.com/WeakAuras/WeakAuras2 — `mists` branch
- ElvUI-Classic-Mists: https://github.com/tukui-org/ElvUI-Classic — `mists` branch
- Details! (MoP Classic): https://github.com/Tercioo/Details-Damage-Meter — `mists` branch
- pfQuest-Vanilla: https://github.com/shagu/pfQuest — `master` branch (vanilla support kept inline)

## Phase 8.2 hand-off

The harness needs to do, per addon:
1. Clone/checkout into `vendor/addons-test/<name>/` (sparse-OK if the repo is large)
2. Symlink/copy into `Interface/AddOns/<name>/` for the matching profile
3. Build the simulator with `--features client-<profile>`
4. Run `wow-sim --no-saved-vars run-tests <name>` (if the addon ships a TestFramework-compatible test) OR `wow-sim --exec-lua` to fire startup events
5. Capture `lua-errors` output; diff against the profile baseline in
   `docs/baselines/<profile>-lua-errors.json` to isolate addon-induced new
   errors
6. CI lane: matrix over (profile, addon)
