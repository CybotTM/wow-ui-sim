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

## Harness (Phase 8.2 — landed)

Implemented as `scripts/test-classic-addons.sh` driven by the declarative
manifest `tools/classic-addon-manifest.tsv`. Per addon, it:

1. Clones (filter:blob:none) into `vendor/addons/<name>/`, pins the
   manifest ref
2. Symlinks `Interface/AddOns/<name>` → `vendor/addons/<name>/<subpath>`
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
