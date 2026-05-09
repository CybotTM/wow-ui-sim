# Classic-profile addon validation targets (Phase 8.1)

Popular community addons picked to validate the wow-ui-sim's classic-profile
coverage. Wrath (`Interface: 30300`) keeps the historical 3.3.5 lane alive,
while the Mists lane now focuses on installed Pandaria Classic addons from the
local WoW install (`/syncthing/World of Warcraft/_classic_/Interface/AddOns`).
One vanilla pick keeps era/anniversary in scope.

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
| 5 | **AllTheThings** (installed Pandaria Classic) | mists | 50503/50504 | Large achievement/collection database. Stresses big static data loads, tooltip hooks, item APIs, and saved option tables. |
| 6 | **Auctionator** (installed Pandaria Classic) | mists | 50503 | Auction UI workflow. Stresses `C_AuctionHouse`, tab setup, money formatting, and list/search controls. |
| 7 | **BlizzMove** (installed Pandaria Classic) | mists | 50503/50504 | Frame movement hooks. Stresses frame discovery, script hooks, saved placement state, and addon interaction with Blizzard panels. |
| 8 | **DeModal** (installed Pandaria Classic) | mists | 50503 | Panel behavior override. Stresses dialog frame state, frame strata/level changes, and visibility hooks. |
| 9 | **DialogueUI** (installed Pandaria Classic) | mists | 50503/50504 | Quest/dialog replacement. Stresses gossip/quest APIs, font strings, textures, animations, and event ordering. |
| 10 | **Leatrix Maps** (installed Pandaria Classic) | mists | 50503/50504 | Map replacement. Stresses `C_Map`, scroll/zoom controls, POI overlays, and map event handling. |
| 11 | **Leatrix Plus** (installed Pandaria Classic) | mists | 50503/50504 | Broad quality-of-life addon. Stresses many small global APIs, hooks, options UI, and saved-variable paths. |
| 12 | **Plater** (installed Pandaria Classic) | mists | 50503/50504 | Nameplate replacement. Stresses nameplate APIs, aura/event churn, frame pools, and OnUpdate-heavy rendering logic. |
| 13 | **Simple Item Level** (installed Pandaria Classic) | mists | 50503 | Character/inspection overlays. Stresses item location, tooltip, paper doll, and equipment APIs. |
| 14 | **pfQuest-Vanilla** | era | 11500 | Vanilla quest helper, smaller than Questie. Stresses minimap pin overlays, world-map pin overlays, `GetQuestLog*` family, and `C_Map.GetMapInfo` shims. |

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

Wrath/Era rows use upstream repositories the harness pins and clones. Mists
rows intentionally use installed Pandaria Classic addons from
`/syncthing/World of Warcraft/_classic_/Interface/AddOns` because that install
is currently active (`wow_classic` build 5.5.3).

- Bartender4 (Wrath): https://www.curseforge.com/wow/addons/bartender4 — Wrath/3.3.5 release
- DBM-WotLK: https://github.com/DeadlyBossMods/DBM-Wrath — `master` branch
- AtlasLoot Classic-WotLK: https://github.com/Hoizame/AtlasLootClassic — `wotlk` branch
- Skada (WotLK): https://github.com/bkader/Skada-WoTLK — `master` branch
- Mists installed addons: `AllTheThings`, `Auctionator`, `BlizzMove`, `DeModal`, `DialogueUI`, `Leatrix_Maps`, `Leatrix_Plus`, `Plater`, `SimpleItemLevel`
- pfQuest-Vanilla: https://github.com/shagu/pfQuest — `master` branch (vanilla support kept inline)

## Harness (Phase 8.2 — landed)

Implemented as `scripts/test-classic-addons.sh` driven by the declarative
manifest `tools/classic-addon-manifest.tsv`. Per addon, it:

1. Resolves each manifest source: Git rows clone (filter:blob:none) into
   `vendor/addons/<name>/` and pin the manifest ref; `local:<absolute-path>`
   rows use the existing installed addon directory
2. Symlinks `Interface/AddOns/<name>` → `source/<subpath>`
3. Builds `wow-sim` with `--features client-<profile>`
4. Runs `wow-sim lua-errors`, saves output to
   `target/addon-harness/<name>-lua-errors.json`
5. Diffs message-set against `docs/baselines/<profile>-lua-errors.json` to
   report **addon-induced** errors (= present after addon load, absent in
   profile baseline)
6. Removes the symlink (clean tree for the next run; pass `--keep-symlinks`
   to skip)

Pass criterion: wow-sim must exit 0. Addon-induced Lua errors are reported
but do NOT fail the harness — those become Phase 8.3 stub work.

Usage:

```
scripts/test-classic-addons.sh                    # every addon
scripts/test-classic-addons.sh Bartender4         # one addon by name
scripts/test-classic-addons.sh --profile wrath    # filter by profile
scripts/test-classic-addons.sh --skip-clone       # reuse vendor checkouts
```

CI lane (Phase 8.4) will invoke this script over a (profile, addon) matrix.

## Per-addon stub shims (Phase 8.3 — landed)

When the harness reports a nonzero `addon-induced errors` count, fix it by
adding rawget-guarded stubs to a per-addon companion addon under
`tools/classic-addon-compat/<manifest-name>/<shim-name>/`. The harness
auto-symlinks every immediate subdirectory there into `Interface/AddOns/`
when running the parent addon; the shim's TOC declares `## LoadFirst: 1`
so its globals exist before the third-party addon's first chunk runs. See
`tools/classic-addon-compat/README.md` for the full convention. Promote
shared gaps to a profile-level bootstrap (`src/wrath/`, `src/mists/`,
`src/era/`) instead of repeating them per-addon.
