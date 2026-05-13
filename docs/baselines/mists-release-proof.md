# Mists Release-Proof Artifact Index

Latest full proof: `scripts/ci-mists-release-proof.sh --skip-clone` followed by
`scripts/ci-mists-release-proof.sh --skip-build --skip-clone` after the
interaction-audit row count fix. The rerun passed all lanes and wrote artifacts
under `target/mists-release-proof/`.

`target/` artifacts are retained local outputs, not committed binary baselines.
If the directory is missing after a clean build, regenerate it with the command
above. GitHub Actions uploads the same tree from the `Mists release proof` job
as the `mists-release-proof` artifact.

## Lane Logs

All lane logs live under `target/mists-release-proof/logs/`:

| Lane | Log path |
|---|---|
| Release build | `target/mists-release-proof/logs/build-release.log` |
| Zero base `lua-errors` | `target/mists-release-proof/logs/zero-lua-errors.log` |
| Installed addon startup matrix | `target/mists-release-proof/logs/installed-addon-matrix.log` |
| Base panel parity and visual comparison | `target/mists-release-proof/logs/panel-parity-and-visual-comparison.log` |
| Installed addon panel matrix | `target/mists-release-proof/logs/installed-addon-panel-matrix.log` |
| Base panel parity with SavedVariables | `target/mists-release-proof/logs/panel-parity-with-saved-vars.log` |
| Installed addon panel matrix with SavedVariables | `target/mists-release-proof/logs/installed-addon-panel-matrix-with-saved-vars.log` |
| Connected GUI micro-menu smoke | `target/mists-release-proof/logs/live-gui-smoke.log` |
| Interaction audit | `target/mists-release-proof/logs/interaction-audit.log` |

The base `lua-errors` JSON snapshot for the proof run is
`target/mists-release-proof/mists-release-lua-errors.json`; the passing value is
an empty array matched against `docs/baselines/mists-lua-errors.json`.

## Panel Artifact Directories

Every panel row in `docs/baselines/mists-panels.md` has four parallel artifact
trees in the full proof:

| Matrix | Directory pattern | Contents per panel slug |
|---|---|---|
| Base panel parity | `target/mists-release-proof/panel-parity/<slug>/` | `open.lua`, `lua-errors.json`, `lua-errors.stderr`, `dump-tree.txt`, `dump-tree.stderr`, `screenshot.webp`, `screenshot.stdout`, `screenshot.stderr` |
| Base panel parity with SavedVariables | `target/mists-release-proof/panel-parity-with-saved-vars/<slug>/` | Same per-panel files as base panel parity, with normal SavedVariables loading enabled. |
| Installed addon panel parity | `target/mists-release-proof/addon-panel-parity/<addon>/<slug>/` | Same per-panel files with one installed Mists addon enabled. |
| Installed addon panel parity with SavedVariables | `target/mists-release-proof/addon-panel-parity-with-saved-vars/<addon>/<slug>/` | Same per-panel files with the addon and normal SavedVariables loading enabled. |

Panel slugs are the artifact slugs already recorded in
`docs/baselines/mists-panels.md`; the latest full proof covered all 37 `Pass`
rows.

## Installed Mists Addon Rows

The full proof uses the `mists` rows from `tools/classic-addon-manifest.tsv`:

| Addon | Source |
|---|---|
| `AllTheThings` | `local:/syncthing/World of Warcraft/_classic_/Interface/AddOns/AllTheThings` |
| `Auctionator` | `local:/syncthing/World of Warcraft/_classic_/Interface/AddOns/Auctionator` |
| `BlizzMove` | `local:/syncthing/World of Warcraft/_classic_/Interface/AddOns/BlizzMove` |
| `DeModal` | `local:/syncthing/World of Warcraft/_classic_/Interface/AddOns/DeModal` |
| `DialogueUI` | `local:/syncthing/World of Warcraft/_classic_/Interface/AddOns/DialogueUI` |
| `Leatrix_Maps` | `local:/syncthing/World of Warcraft/_classic_/Interface/AddOns/Leatrix_Maps` |
| `Leatrix_Plus` | `local:/syncthing/World of Warcraft/_classic_/Interface/AddOns/Leatrix_Plus` |
| `Plater` | `local:/syncthing/World of Warcraft/_classic_/Interface/AddOns/Plater` |
| `SimpleItemLevel` | `local:/syncthing/World of Warcraft/_classic_/Interface/AddOns/SimpleItemLevel` |

Latest full-run result: `passed: 9`, `failed: 0` for startup, installed-addon
panel parity, and installed-addon panel parity with SavedVariables.

## CI Upload

The `Test` workflow includes an opt-in `Mists release proof` job. It runs when
`RUN_MISTS_RELEASE_PROOF` is set to `1` as a repository variable, or when a
manual `workflow_dispatch` run sets `run_mists_release_proof` to true. The job
uses `scripts/ci-mists-release-proof.sh --skip-clone` and uploads
`target/mists-release-proof/`, including logs, base `lua-errors` JSON, panel
screenshots, and frame dumps.

First CI proof dispatch attempts:
`https://github.com/Osso/wow-ui-sim/actions/runs/25824999285`. The target
`Mists release proof` job reached the base panel parity lane, uploaded
`mists-release-proof`, and failed on the first screenshot because the GitHub
runner lacked a headless GPU adapter. Follow-up run
`https://github.com/Osso/wow-ui-sim/actions/runs/25827656815` still failed in
the same screenshot path with `active_backends: 0` under generic Xvfb. The
workflow now installs the Mesa GL/OSMesa runtime packages, enables software
rendering, runs the release-proof script under `xvfb-run`, and forces wgpu to
the GL backend for screenshot capture.

## Remaining Gaps
- The latest audited Mists panel workflows have no `Missing` rows in
  `docs/baselines/mists-panel-interactions.md`.
- Mists-specific differences remain expected rather than gaps: Pandaria-era
  talents/glyphs, pre-EditMode interface options, and legacy LoD service frames
  intentionally do not match retail-only workflows one-for-one.
