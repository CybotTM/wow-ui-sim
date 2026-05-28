# Mists Addon Validation Targets

Installed Pandaria Classic addons selected to validate the `client-mists`
surface. The harness prefers installed addons from `MISTS_ADDON_ROOT` /
`WOW_MISTS_ADDON_ROOT` and falls back to committed fixtures under
`tools/classic-addon-fixtures/mists/`.

## Picks

| # | Addon | Interface | Why this addon |
|--:|-------|----------:|----------------|
| 1 | **AllTheThings** | 50503/50504 | Large achievement/collection database. Stresses static data loads, tooltip hooks, item APIs, and saved option tables. |
| 2 | **Auctionator** | 50503 | Auction UI workflow. Stresses auction APIs, tab setup, money formatting, and list/search controls. |
| 3 | **BlizzMove** | 50503/50504 | Frame movement hooks. Stresses frame discovery, script hooks, saved placement state, and Blizzard panel interaction. |
| 4 | **DeModal** | 50503/50504 | Panel modal behavior overrides. Stresses show/hide hooks and frame strata interactions. |
| 5 | **DialogueUI** | 50503/50504 | Dialog and quest presentation. Stresses gossip, quest text, and tooltip-adjacent UI flows. |
| 6 | **Leatrix_Maps** | 50503/50504 | World-map addon. Stresses map canvas, pins, scroll/zoom, and saved map state. |
| 7 | **Leatrix_Plus** | 50503/50504 | Broad UI convenience addon. Stresses many small globals and frame hooks. |
| 8 | **Plater** | 50503/50504 | Nameplate addon. Stresses nameplate globals, combat-ish events, and saved configuration tables. |
| 9 | **SimpleItemLevel** | 50503/50504 | Lightweight character/item overlay. Stresses inspect/item APIs and tooltip hooks. |

## Harness

`scripts/test-classic-addons.sh` reads `tools/classic-addon-manifest.tsv` and:

1. Resolves each `mists-addon:<addon>` source from an installed addon root or a committed fixture.
2. Symlinks `Interface/AddOns/<name>` to the resolved source.
3. Builds `wow-sim` with `--features client-mists`.
4. Runs `wow-sim lua-errors` and writes `target/addon-harness/<name>-lua-errors.json`.
5. Diffs against `docs/baselines/mists-lua-errors.json` to report addon-induced errors.
6. Removes the symlink unless `--keep-symlinks` is passed.

Usage:

```bash
scripts/test-classic-addons.sh
scripts/test-classic-addons.sh AllTheThings
scripts/test-classic-addons.sh --profile mists
scripts/test-classic-addons.sh --skip-clone
```

Per-addon shims belong under `tools/classic-addon-compat/<manifest-name>/`.
Shared gaps belong in `src/mists/compat_bootstrap.lua` or a Rust backing model.
