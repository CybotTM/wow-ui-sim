# Mists Release-Proof Artifact Index

Latest full proof: `scripts/ci-mists-release-proof.sh --skip-clone` followed by
`scripts/ci-mists-release-proof.sh --skip-build --skip-clone` after the
interaction-audit row count fix. The rerun passed all lanes and wrote artifacts
under `target/mists-release-proof/`.

`target/` artifacts are retained local outputs, not committed binary baselines.
If the directory is missing after a clean build, regenerate it with the command
above. GitHub Actions uploads the same tree from the `Mists release proof` job
as the `mists-release-proof` artifact.

## Visual Evidence Scope

Treat local asset-backed captures and CI signal-only captures as different
evidence classes. Local runs on a machine with WoW CASC data are the authority
for visual parity questions such as Blizzard textures, font faces, action bar
art, unit frame art, and bottom-tab text rendering. Recent local triage outputs
live under `target/mists-local-visual-audit/`, including the idle HUD screenshot
and CharacterFrame reputation dump used to verify Mists font and tab text
rendering after the known-font CASC fallback fix.

GitHub-hosted release-proof runs currently exercise panel roots, Lua errors,
render-batch signal, screenshot non-blankness, frame dumps, interaction audit
coverage, and artifact completeness. They do not prove texture/font visual
parity while they run without a WoW CASC install and with signal-only panel
visual checks. A green CI signal-only run is therefore evidence that panels load,
produce render output, and stay free of `lua-errors`; it is not evidence that
the rendered art matches local asset-backed Mists captures.

Latest local asset-backed HUD recapture: 2026-05-14 on
`classic-profile-rollout`, retained under `target/mists-local-visual-audit/`.
`idle-hud-recapture.webp` and `target-hud-recapture.webp` show the player and
target unit frames using Mists art, populated action buttons, bag slots, micro
buttons, minimap art, quest tracker text, and no gray placeholder bars or broken
black action-slot boxes. Companion frame dumps
`player-frame-recapture.dump`, `target-frame-visible-recapture.dump`, and
`main-menu-bar-recapture.dump` verify the frame geometry and resolved texture
paths behind those captures. The focused regression test is
`tests/mists_hud_layout_regression.rs::mists_hud_keeps_unit_frames_and_bottom_bar_textured`.

## Local Non-Release Completion Audit

Latest local debug-profile audit: 2026-05-14 on `classic-profile-rollout`,
with artifacts retained under `target/mists-local-completion-audit/`.

| Lane | Result | Artifact |
|---|---|---|
| Base Mists `lua-errors` with addons and SavedVariables disabled | `0` distinct errors | `target/mists-local-completion-audit/base-lua-errors.json` |
| Base panel parity execution | `37` panel rows passed with per-panel `lua-errors`, dump-tree, screenshot, root-frame, and visual checks | `target/mists-local-completion-audit/panel-parity.log`; artifacts under `target/mists-local-completion-audit/panel-parity/` |
| Installed Mists addon panel manifest validation | `9` addon rows validated | `target/mists-local-completion-audit/addon-panel-validation.log` |
| Live connected-GUI smoke validation and execution | Passed idle HUD, specialization Learn-to-talents, and micro-menu probes | `target/mists-local-completion-audit/live-gui-smoke.log` |
| Installed Mists addon startup matrix | `passed: 9`, `failed: 0`, all rows `0` addon-induced errors | `target/mists-local-completion-audit/installed-addon-startup.log` |
| Mists panel interaction audit | `1` Rust test suite passed; every pass panel remains represented in the interaction audit | `target/mists-local-completion-audit/interaction-audit.log` |

This audit is intentionally local and non-release: it uses debug `wow-sim` /
`wow-cli` binaries and does not run the release-proof wrapper. Its purpose is to
confirm the current working tree still satisfies the core Mists parity gates
before adding or promoting any narrower follow-up task.

The only weak lane in this local audit is installed-addon panel validation: it
confirms all 9 installed Mists addon rows are present and resolvable, while the
expensive full installed-addon panel screenshot matrix remains deliberately
outside this local completion pass. The promoted follow-up is a bounded,
non-release installed-addon panel parity sample that strengthens addon-panel
evidence without reintroducing release-proof or CI texture work.

## Local Installed-Addon Panel Sample

Latest bounded local sample: 2026-05-14 on `classic-profile-rollout`, using
debug `wow-sim` and the installed `BlizzMove` addon as the UI-mutating addon
under test.

| Addon | Panel row | Result | Artifact |
|---|---|---|---|
| `BlizzMove` | Talents and glyphs | Passed; per-panel `lua-errors` length `0`, root dump and screenshot written | `target/mists-local-addon-panel-sample/blizzmove-talents.log`; artifacts under `target/mists-local-addon-panel-sample/blizzmove-talents/BlizzMove/talents-glyphs/` |
| `BlizzMove` | Action bars, micro menu, bag bar, status bars | Passed; per-panel `lua-errors` length `0`, root dump and screenshot written | `target/mists-local-addon-panel-sample/blizzmove-action-bars.log`; artifacts under `target/mists-local-addon-panel-sample/blizzmove-action-bars/BlizzMove/action-bars/` |

The full installed-addon screenshot matrix remains deferred locally because it
is the expensive 9-addon by 37-panel release-proof-style lane. This sample keeps
addon-panel evidence moving by covering the recently touched talent and
HUD/action-bar surfaces with a frame-mutating addon enabled, without adding CI
texture requirements or rerunning the release-proof wrapper.

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
| Artifact completeness | `target/mists-release-proof/logs/artifact-completeness.log` |

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
| `AllTheThings` | `mists-addon:AllTheThings` |
| `Auctionator` | `mists-addon:Auctionator` |
| `BlizzMove` | `mists-addon:BlizzMove` |
| `DeModal` | `mists-addon:DeModal` |
| `DialogueUI` | `mists-addon:DialogueUI` |
| `Leatrix_Maps` | `mists-addon:Leatrix_Maps` |
| `Leatrix_Plus` | `mists-addon:Leatrix_Plus` |
| `Plater` | `mists-addon:Plater` |
| `SimpleItemLevel` | `mists-addon:SimpleItemLevel` |

`mists-addon:<name>` resolves to the installed addon under `MISTS_ADDON_ROOT`
or `WOW_MISTS_ADDON_ROOT` when available. CI falls back to the committed
fixtures under `tools/classic-addon-fixtures/mists/<name>/`, so the release
proof no longer depends on a runner-local WoW install.

Latest full-run result: `passed: 9`, `failed: 0` for startup, installed-addon
panel parity, and installed-addon panel parity with SavedVariables.

## CI Upload

The `Test` workflow includes `Mists release proof` as a normal PR/master
validation job. The job uses `scripts/ci-mists-release-proof.sh --skip-clone`
and uploads `target/mists-release-proof/`, including logs, base `lua-errors`
JSON, panel screenshots, and frame dumps.

## GitHub Actions Brief

The GitHub Actions `Mists release proof` job is the CI version of the local
release-profile proof command. It builds release `wow-sim`, `wow-cli`, and
`panel-visual-metrics` for `client-mists`, then runs the zero `lua-errors`
baseline, installed-addon startup matrix, panel parity with visual comparison,
SavedVariables variants, connected-GUI micro-menu smoke, and interaction audit.

It runs on pull requests, pushes to `master`, and manual `workflow_dispatch`
runs alongside the normal retail and client-profile jobs. This makes the full
Mists parity proof part of the required CI surface instead of a separate
opt-in lane.

The current GitHub-hosted proof job sets `MISTS_PANEL_SIGNAL_ONLY=1` because
the runner does not have a WoW CASC install for texture/font extraction. In
that mode the panel runner still rejects missing roots, Lua errors, empty
render batches, background-only screenshots, and too-small foreground bounding
boxes, but it skips comparison against the asset-rich local
`docs/baselines/mists-panel-visuals.tsv` hashes. Remove that env var once CI has
CASC data available.

If the job fails, inspect the uploaded `mists-release-proof` artifact first.
Start with `logs/<lane>.log`, then open the matching panel or addon directory
for `lua-errors.json`, `dump-tree.txt`, and `screenshot.webp`. Treat the first
failing lane as the next fix target; do not refresh baselines just to hide a
new CI-only `lua-errors`, missing root frame, blank render, or visual-regression
failure.

CI proof dispatch attempts:
`https://github.com/Osso/wow-ui-sim/actions/runs/25824999285`. The target
`Mists release proof` job reached the base panel parity lane, uploaded
`mists-release-proof`, and failed on the first screenshot because the GitHub
runner lacked a headless GPU adapter. Follow-up run
`https://github.com/Osso/wow-ui-sim/actions/runs/25827656815` still failed in
the same screenshot path with `active_backends: 0` under generic Xvfb. The
workflow now installs the Mesa GL/OSMesa runtime packages, enables software
rendering, runs the release-proof script under `xvfb-run`, and forces wgpu to
the GL backend for screenshot capture.

Follow-up dispatch:
`https://github.com/Osso/wow-ui-sim/actions/runs/25832259479`. It used commit
`1dde4bb4` and reached `panel-parity-and-visual-comparison`, then failed on the
first `character` screenshot with `luminance contrast fell from 19063 to 7030`.
The uploaded artifact showed no Lua errors, but the runner had no CASC install
and logged missing character frame textures/icons, so the asset-rich local
visual baseline was not comparable in CI. The workflow now runs the release
proof with signal-only panel visuals until CASC data is available in CI.

Follow-up dispatch:
`https://github.com/Osso/wow-ui-sim/actions/runs/25832774071`. It used commit
`c801aebe`, cleared the signal-only panel visual gate, and was canceled after
the installed-addon panel matrix kept running past the previous failure point.
The uploaded artifact showed the matrix was still progressing through addon
panel rows, so the release proof now adds explicit deadlines to the later
connected-GUI smoke and interaction-audit lanes instead of allowing those lanes
to hang without a lane log.

Passing dispatch:
`https://github.com/Osso/wow-ui-sim/actions/runs/25834559363`. It used commit
`5de67dca`, passed the `Mists release proof` job, and uploaded
`mists-release-proof` at
`https://github.com/Osso/wow-ui-sim/actions/runs/25834559363/artifacts/6985920313`.
The proof ran from `2026-05-14T00:35:20Z` to `2026-05-14T02:29:12Z`.

## Remaining Gaps
- The latest audited Mists panel workflows have no `Missing` rows in
  `docs/baselines/mists-panel-interactions.md`.
- Mists-specific differences remain expected rather than gaps: Pandaria-era
  talents/glyphs, pre-EditMode interface options, and legacy LoD service frames
  intentionally do not match retail-only workflows one-for-one.
