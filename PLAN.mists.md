# Goal

- [x] Back Mists CharacterFrame pet tab visibility with simulator pet UI state so seeded pets keep `CharacterFrameTab2` available while default no-pet state still hides it.
- [x] Record Mists CharacterFrame pet paper-doll tab interaction coverage in the panel parity docs and audit guard.
- [x] Guard Mists CharacterFrame default no-pet state so `CharacterFrameTab2` stays hidden when `HasPetUI()` is false.
- [x] Refresh focused Mists Character panel parity artifacts after the pet UI state guard, including normal SavedVariables.
- [x] Re-run focused Mists base `lua-errors` and installed-addon startup matrix after the pet UI state guard.
- [x] Fix Mists CharacterFrame real open path so the Pet tab hides immediately when `HasPetUI()` is false, without manually calling `PetPaperDollFrame_UpdateIsAvailable()`.
- [x] Re-run focused installed-addon Mists Character panel matrix with normal SavedVariables after the pet UI open-path fix.
- [x] Refresh focused Mists base `lua-errors` and Character panel parity artifacts after the startup `PET_UI_UPDATE` fix.
- [x] Guard the Mists release-proof docs so the deferred full installed-addon screenshot matrix is recorded as a validation scope limit, not silently treated as completed local proof.
- [ ] Bring the Mists of Pandaria client profile to feature parity with the retail `wow-ui-sim`: every Blizzard UI panel loads cleanly, renders correctly, and supports the same interactions the retail build supports — no stub-only frames, no silently-broken panels, and `lua-errors` stays empty across the matrix of installed Mists addons.

# Mists / Pandaria Classic Startup Plan

## Active TODO — Panel Parity

In-flight (git status shows uncommitted work):

- [x] Finish the spellbook panel under `client-mists` — currently editing `src/c_api/c_spell.rs`, `src/c_api/c_spell_book.rs`, and `tests/mists_spellbook_panel.rs`; land the test green and commit.
- [x] Land professions spellbook coverage under `client-mists` — `tests/professions_spellbook.rs` is modified; verify, run, commit.
- [x] Resolve `src/lua_api/globals/talent_spec_probes.rs` changes — confirm Mists talent specs match retail probes, then commit.

Parity hardening TODO:
- [x] Run a local non-release Mists completion audit covering base `lua-errors`, installed-addon startup, panel parity validation, installed-addon panel validation, and live-GUI smoke validation; promote the first failing or unproven lane to the next top-priority task instead of treating partial validation as completion.
- [x] Reconcile `docs/baselines/mists-panel-interactions.md` with the expanded live connected-GUI smoke coverage, and add follow-up tasks for any panel workflow still backed only by load/show or static frame-dump evidence.
- [x] Add AddOnList/UI-management interaction coverage: toggle addon rows, exercise enable/disable or reload-state controls, and assert the visible row state changes instead of only proving the AddOnList root renders.
- [x] Add Inspect and Guild Control interaction coverage: seed inspect/guild-control state, switch guild-control tabs/ranks or inspect equipment slots, and assert the visible detail state changes.
- [x] Add legacy service-panel interaction coverage for Archaeology, CraftFrame, TradeSkillFrame, barber shop, black market, item socketing, reforging, and item upgrade panels; drive a selection/action path in each grouped panel instead of only checking renderable roots.
- [x] Add utility/dialog interaction coverage for Challenge Mode, Quest Choice, Time Manager, and Move Pad LoD panels; assert a user-visible state change for each rather than relying on manifest-runner frame construction.
- [x] Re-capture local asset-backed HUD/action-bar/unit-frame evidence after the font and live-smoke fixes, then add a focused regression test for the first remaining confirmed visual/layout mismatch.
- [x] Run a local non-release Mists completion audit after the latest interaction and HUD-layout coverage, covering base `lua-errors`, panel parity, installed-addon startup/panel validation, live-GUI smoke, and interaction audit; update `docs/baselines/mists-release-proof.md` and promote the first failing or weak lane to a new concrete task.
- [x] Strengthen local installed-addon panel evidence without release-proof or CI texture work: run a bounded non-release installed-addon panel parity sample across at least one UI-mutating addon and the recent HUD/talent/action-bar panel rows, then document whether the full installed-addon screenshot matrix remains deferred.
- [x] Compare the Mists panel parity matrix against retail `PLAN.tests.md` Blizzard UI module coverage, identify any Mists-applicable user-facing Blizzard addon or workflow that still lacks a `client-mists` panel/interaction row, and promote the first real gap to a focused test/fix task.
- [x] Add Mists Store/CatalogShop/WowToken/SimpleCheckout parity coverage: create a retained panel row plus interaction tests that load `Blizzard_StoreUI`, show `StoreFrame`/`CatalogShopFrame`, exercise category or product selection and checkout/token affordances, and keep `lua-errors` empty.
- [x] Re-run the local non-release Mists completion audit after adding the Store/CatalogShop panel row, covering base `lua-errors`, full 38-row panel parity, installed-addon startup, live-GUI smoke, interaction audit, and a bounded installed-addon panel sample.
- [x] Re-run local 38-row Mists panel parity with normal SavedVariables enabled after the Store/CatalogShop row, and fix or promote the first saved-var-only panel regression.
- [x] Run a bounded local installed-addon panel sample with normal SavedVariables enabled for the Store/CatalogShop row, and fix or promote the first addon-plus-saved-vars regression.
- [x] Audit whether any Mists panel row still lacks retained local asset-backed visual evidence after the 38-row saved-vars and addon samples; promote the first missing artifact or weak visual row to a focused fix.
- [x] Expand bounded local installed-addon panel sampling to a second UI-mutating addon with normal SavedVariables enabled, covering Store/CatalogShop plus one non-store panel row, and fix or promote the first addon-specific regression.
- [x] Re-run base Mists `lua-errors` and the installed-addon startup matrix after the CatalogShop `SOUNDKIT` fix, and fix or promote the first startup regression.
- [x] Re-run the live connected-GUI Mists smoke runner after the CatalogShop `SOUNDKIT` fix, and fix or promote the first real-input regression.
- [x] Re-run the Mists panel interaction audit after the CatalogShop `SOUNDKIT` fix, and fix or promote the first interaction regression.
- [x] Re-run focused Store/CatalogShop panel parity for the base profile after the CatalogShop `SOUNDKIT` fix, and fix or promote the first retained-artifact regression.
- [x] Re-run local 38-row Mists panel parity after the CatalogShop `SOUNDKIT` fix, and fix or promote the first full-matrix regression.
- [x] Re-run local 38-row Mists panel parity with normal SavedVariables enabled after the CatalogShop `SOUNDKIT` fix, and fix or promote the first saved-vars full-matrix regression.
- [x] Create a dedicated Mists Blizzard UI test coverage index that maps each `docs/baselines/mists-panels.md` panel row to the relevant `tests/mists_*` files and the comparable retail `PLAN.tests.md` Blizzard UI module coverage, then promote the first Mists-applicable workflow whose evidence is weaker than retail.
- [x] Add a Mists Spellbook/professions interaction test that clicks or invokes a visible profession button and asserts the resulting profession action path, visible profession frame, or recorded cast/action state changes instead of only proving the profession tab renders.
- [x] Re-run the Mists panel interaction audit and coverage index comparison after the Spellbook/profession action coverage, then promote the first remaining weaker workflow or document that none are currently identified.
- [x] Re-run focused Mists Spellbook/professions panel parity after the legacy `CastSpell(slot, bookType)` fix, with and without normal SavedVariables, and fix or promote the first panel-render or `lua-errors` regression.
- [x] Re-run base Mists `lua-errors` and the installed-addon startup matrix after the legacy `CastSpell(slot, bookType)` fix, and fix or promote the first startup or addon-induced regression.
- [x] Re-run the live connected-GUI Mists smoke runner after the legacy `CastSpell(slot, bookType)` fix, and fix or promote the first real-input regression.
- [x] Run a bounded installed-addon Spellbook/professions panel sample after the legacy `CastSpell(slot, bookType)` fix, with normal SavedVariables enabled for one UI-mutating addon, and fix or promote the first addon-specific profession-action regression.
- [x] Re-run local 38-row Mists panel parity after the legacy `CastSpell(slot, bookType)` fix, and fix or promote the first full-matrix panel regression.
- [x] Re-run local 38-row Mists panel parity with normal SavedVariables enabled after the legacy `CastSpell(slot, bookType)` fix, and fix or promote the first saved-var-only panel regression.
- [x] Re-run a bounded local installed-addon panel sample across a second UI-mutating addon after the legacy `CastSpell(slot, bookType)` fix, covering Spellbook/professions and one adjacent profession panel with normal SavedVariables, and fix or promote the first addon-specific regression.
- [x] Re-run a bounded local installed-addon panel sample with a non-frame-mover UI addon after the legacy `CastSpell(slot, bookType)` fix, covering Spellbook/professions and TradeSkill with normal SavedVariables, and fix or promote the first addon-specific regression.
- [x] Run a bounded local installed-addon panel sample with a data-heavy Mists addon, covering a high-data panel row plus one profession-adjacent row with normal SavedVariables, and fix or promote the first addon-specific regression.
- [x] Run a bounded local installed-addon panel sample with an auction-focused Mists addon, covering Auction House and one adjacent commerce panel with normal SavedVariables, and fix or promote the first addon-specific regression.
- [x] Run a bounded local installed-addon panel sample with a nameplate/action-bar Mists addon, covering Nameplates and Action bars with normal SavedVariables, and fix or promote the first addon-specific regression.
- [x] Run a bounded local installed-addon panel sample with a map/navigation Mists addon, covering World Map and Quest log with normal SavedVariables, and fix or promote the first addon-specific regression.
- [x] Run a bounded local installed-addon panel sample with a dialogue replacement Mists addon, covering Quest choice and Quest log with normal SavedVariables, and fix or promote the first addon-specific regression.
- [x] Run a bounded local installed-addon panel sample with an item-level Mists addon, covering Character and Inspect/Guild Control panels with normal SavedVariables, and fix or promote the first addon-specific regression.
- [x] Audit bounded installed-addon panel samples against all installed Mists addon manifest rows, record which addons and panel categories have saved-vars panel evidence, and promote the first remaining coverage gap.
- [x] Add a local bounded-addon-sample coverage checker that fails when any installed Mists addon manifest row lacks at least one normal-SavedVariables panel sample artifact recorded in `docs/baselines/mists-release-proof.md`.
- [x] Add a local Mists interaction-coverage checker that compares `docs/baselines/mists-panel-interactions.md` to `docs/baselines/mists-panels.md` and fails when a passing panel row lacks interaction evidence or an explicit documented gap.
- [x] Add a local Mists panel evidence checker that fails when `docs/baselines/mists-panels.md` references stale or missing retained artifact paths for the latest local panel parity output tree.
- [x] Add a local Mists panel lua-error artifact checker that fails when any latest retained panel parity row is missing `lua-errors.json` or contains a non-empty error array.
- [x] Add a local Mists panel frame-dump checker that fails when any latest retained panel parity row has an empty dump or lacks a visible root frame marker.
- [x] Add a local Mists screenshot artifact checker that fails when any latest retained panel parity row is missing `screenshot.webp` or has a zero-byte screenshot.
- [x] Add a local Mists screenshot format checker that fails when any latest retained panel screenshot is not a WebP image header.
- [x] Add a local Mists panel artifact consistency checker that fails when the latest retained screenshot, frame-dump, and lua-error artifact slug sets diverge.
- [x] Add a local Mists interaction evidence checker that fails when `docs/baselines/mists-panel-interactions.md` references a missing `tests/mists_*.rs` file or missing test function.
- [x] Add a local Mists interaction-strength checker that flags interaction baseline rows whose cited test name or row notes still only prove load/show/no-lua-errors coverage.
- [x] Add a local Mists interaction-reference checker that requires every `Covered` or `Mists-specific` row to cite at least one `tests/mists_*.rs` reference unless the row documents a non-Mists shared test exception.
- [x] Add a local Mists interaction status checker that fails if any row is marked `Follow-up` or `Missing` without a matching unchecked PLAN.mists remediation task naming that panel.
- [x] Add a local Mists interaction baseline schema checker that fails if table rows have missing columns, unknown statuses, or duplicate panel names.
- [x] Add a local Mists panel baseline schema checker that fails if `docs/baselines/mists-panels.md` has malformed rows, unknown statuses, duplicate panel names, or missing screenshot/dump artifact references.
- [x] Add a local Mists panel status checker that fails if any `Watch` or `Fail` row lacks a matching unchecked PLAN.mists remediation task naming that panel.
- [x] Add a local Mists retained-artifact freshness checker that fails if `docs/baselines/mists-panels.md` points at an artifact root different from the latest local panel parity root constant.
- [x] Fix Mists glyph panel texture resolution so opening the glyph panel does not request the directory path `/syncthing/World of Warcraft/_retail_/BlizzardInterfaceArt/` as an image.
- [x] Add a local retained-artifact texture-error checker so Mists panel parity logs fail if any retained screenshot or `lua-errors` lane tries to decode `BlizzardInterfaceArt/` as an image directory.
- [x] Re-run local Mists addon startup and focused panel parity after the glyph/currency texture-directory fix, then record or promote the first remaining `lua-errors` or texture-log regression.
- [x] Reconcile stale Mists release-proof notes that still describe bounded saved-vars addon sample enforcement as a remaining local gap, now that `mists_panel_parity_runner` enforces it.
- [x] Implement Mists `GameTooltip:SetGlyph` so hovering glyph sockets in `Blizzard_GlyphUI` does not throw and displays socket/glyph tooltip text.
- [x] Re-audit local Mists idle HUD and core panel screenshots with real local CASC assets, focusing on the previously reported unit-frame/action-bar layout regressions, missing bottom-tab text, specialization Learn flow, and font fallback symptoms; fix the first root-cause regression found.
- [x] Add a non-CI Mists visual triage note that separates local asset-backed parity evidence from CI signal-only checks, so texture/font availability is not confused with panel parity completion.
- [x] Extend the live connected-GUI Mists smoke runner to include the no-panel HUD state plus the specialization Learn-to-talents path, then fail on `lua-errors` or missing/hidden target frames after real input dispatch.
- [x] Build a scripted Mists panel parity runner that opens every row in `docs/baselines/mists-panels.md`, records `lua-errors`, and fails on missing root frames, hidden stub-only frames, or empty render output.
- [x] Replace the `test-backed` placeholders in `docs/baselines/mists-panels.md` with retained screenshot or frame-dump artifact paths for every panel row.
- [x] Audit each Mists panel test for real interaction assertions, not just load/show coverage, and add follow-up tasks for any panel that only proves startup.
- [x] Add Communities/Club Finder interaction coverage: select finder rows, switch finder modes, and assert the visible finder detail state changes instead of only showing `CommunitiesFrame`.
- [x] Add World Map interaction coverage: exercise zoom or map navigation plus a quest-pin click/selection path, not only map load and seeded POI data.
- [x] Add Character subpanel interaction coverage for titles and equipment manager flows, since current Mists character tests focus on gear slots and reputation rows.
- [x] Add Talents/Glyphs mutation coverage: select a Mists talent row option and glyph socket path, then assert the selected/learned state changes.
- [x] Add Collections item-action coverage for mount, toy, and heirloom rows; current coverage switches tabs and verifies data, but does not prove row actions work.
- [x] Run the installed Mists addon matrix with normal saved variables enabled and record whether saved-var-backed startup still stays at zero addon-induced errors.
- [x] Add a CI-friendly guard that enforces the zero Mists `lua-errors` baseline and the scripted panel parity runner.
- [x] Expand the CI guard beyond base startup: run the installed Mists addon matrix in CI and fail on any addon-induced `lua-errors` regression.
- [x] Add visual artifact comparison for the Mists panel parity runner so CI catches blank or materially-regressed panel renders, not just missing frame roots.
- [x] Audit Mists panel interactions against retail interaction coverage and add parity tests for any retail-supported workflow still missing under `client-mists`.
- [x] Add a low-signal visual-artifact gate to `scripts/mists-panel-parity.sh`: fail rows whose screenshot is mostly background or whose root-frame bounding box is too small for the panel being audited.
- [x] Run panel parity with normal SavedVariables enabled and document/fix any panel that regresses only when real WTF state loads.
- [x] Expand the installed-addon matrix beyond startup: run `scripts/mists-panel-parity.sh` once per installed Mists addon enabled and fail on addon-induced panel `lua-errors` or visual regressions.
- [x] Add a live connected-GUI Mists smoke runner that clicks every micro-menu/panel opener through `wow-cli`, then asserts `lua-errors` remains empty after real input dispatch.
- [x] Audit every Mists Blizzard LoD addon not represented in `docs/baselines/mists-panels.md` and add panel/interaction rows for any user-facing frame still outside the parity matrix.
- [x] Fix Mists Blizzard_RaidUI and Blizzard_ArenaUI visual rendering so raid and arena unit frames can join the panel parity matrix without background-only screenshots.
- [x] Fix Mists Blizzard_BattlefieldMap screenshot startup dispatch so BattlefieldMapFrame can enter the panel parity matrix without `SetScale(0)` errors.
- [x] Fix Mists Blizzard_CraftUI LoD construction so `CraftFrame_LoadUI()` creates a renderable `CraftFrame` for the panel parity matrix.
- [x] Implement the legacy Mists `GetTradeSkill*` backing globals so Blizzard_TradeSkillUI can render and enter the panel parity matrix.
- [x] Fix Mists Blizzard_TrainerUI legacy trainer-service globals and template wiring so ClassTrainerFrame can enter the panel parity matrix.
- [x] Add a release-profile Mists CI proof command that runs zero `lua-errors`, installed-addon matrix, panel parity, interaction audit, and visual comparison with pipefail-safe logging.
- [x] Run the full release-profile Mists proof command locally with `--skip-clone`, document every failure, and fix the first failing lane instead of treating validation-only as sufficient.
- [x] Add the connected-GUI micro-menu smoke runner as a release-proof lane using release `wow-sim` and `wow-cli`, so real input dispatch is part of the parity gate.
- [x] Extend the release proof to run panel parity and installed-addon panel parity with normal SavedVariables enabled, then fix any saved-var-only panel or addon regression.
- [x] Fix Mists action/micro/bag bar visual regression where lower bar slots render black/empty with clipped or offset highlight/hand affordances.
- [x] Fix Mists unit frame/action area visual regression where PlayerFrame/target-frame art overlaps or clips, target names like Khadgar render in cramped frames, gray placeholder bars leak, and right-side action slots render as broken black boxes.
- [x] Fix Mists specialization/talent UI layout overlap, then implement the `Learn` specialization flow so it switches from spec selection into the actual talents UI.
- [x] Audit and fix Mists font loading so Blizzard panels use the expected font assets instead of fallback/missing-font rendering.
- [x] Add a retained release-proof artifact index under `docs/baselines/mists-release-proof.md` that records log paths, panel artifact directories, addon rows, and any remaining gaps from the latest full run.
- [x] Wire the release-proof command into CI with artifact upload for logs, lua-error JSON, panel screenshots, and frame dumps once the local full proof is green.
- [x] Write brief in docs of `GitHub Actions Mists release-proof`
- [x] bottom tabs e.g reputation panel have the text missing
- [x] layouts problem even when no panel is open
- [x] fritz font is not loading? does it need the fallback by md5?
- [x] Run the new GitHub Actions Mists release-proof job via `workflow_dispatch`, record the run/artifact links in `docs/baselines/mists-release-proof.md`, and fix the first failing CI-only lane instead of relying on the local proof.
- [x] Make the Mists release-proof CI lane self-contained for installed-addon coverage: replace `/syncthing/World of Warcraft/_classic_/Interface/AddOns/...` assumptions with CI-available pinned addon fixtures or an explicit prepared artifact/cache step.
- [x] Add release-proof artifact completeness validation that fails when `target/mists-release-proof/` is missing lane logs, the base `mists-release-lua-errors.json`, per-panel screenshots, or frame dumps before upload.
- [x] Promote the Mists release-proof job from opt-in to required PR/master validation after the CI environment has CASC data and installed-addon fixtures available.

Panel-by-panel parity audit (pass = panel renders + interacts + zero `lua-errors` under Mists):

- [x] Character panel (paperdoll, stats, titles, equipment manager)
- [x] Spellbook & professions: missing icons, layout problems
- [x] Talents & glyphs (Mists uses talent rows + glyph slots, not retail trees)
- [x] Quest log / objective tracker
- [x] World map (zone overlays, quest pins, opacity slider already verified)
- [x] Mail (inbox, send, attachments, COD)
- [x] Auction House (browse, bid, post, cancel)
- [x] Bank, ReagentBank, Void Storage, Guild Bank
- [x] Trade window & TradePlayerInputMoneyFrame (root cause fixed in triage)
- [x] Friends, Who, Guild, Communities/Club finder
- [x] PvP UI (HonorFrame, BG queue, Conquest — backing API fixed in triage)
- [x] LFG / LFR / Raid Browser (Mists-era pre-group-finder rewrite)
- [x] Collections: mounts, pets, toys, heirlooms, transmog (no Wardrobe in MoP)
- [x] Pet Journal & Battle Pet UI (MoP-introduced system)
- [x] Achievements & Calendar
- [x] Encounter Journal (Mists adventure guide)
- [x] Currency / Token UI (legacy `GetCurrencyListSize` wrapper verified)
- [x] Macro & key bindings
- [x] EditMode / interface options (Mists pre-EditMode — uses InterfaceOptionsFrame)
- [x] Action bars, micro menu, bag bar, status bars
- [x] Nameplates (CVar defaults fixed in triage; verify rendering)
- [x] Loot / group loot / personal loot UI
- [x] Game menu options breaks

Cross-cutting follow-ups:

- [x] Re-run `scripts/test-classic-addons.sh --profile mists` after each panel batch lands; keep addon-induced errors at zero.
- [x] Capture a panel parity baseline under `docs/baselines/mists-panels.md` (one row per panel: status, screenshot, gap notes) once the audit starts producing real signal.
- [x] Decide whether to refresh `docs/baselines/mists-lua-errors.json` after panel parity is reached, or keep it as the historical Phase 7.4 capture.

Latest addon harness verification: `scripts/test-classic-addons.sh --profile mists`
completed with `passed: 9` and `failed: 0`. Each
`target/addon-harness/*-lua-errors.json` output remained an empty array, so all
installed Mists addon targets stayed at `0` total Lua errors and `0`
addon-induced errors.

SavedVariables-backed addon harness verification:
`scripts/test-classic-addons.sh --profile mists --skip-clone --with-saved-vars`
completed with `passed: 9` and `failed: 0`. Each
`target/addon-harness/*-with-saved-vars-lua-errors.json` output remained an
empty array, so normal SavedVariables loading stayed at `0` total Lua errors
and `0` addon-induced errors across the installed Mists addon matrix.

CI-style addon harness verification:
`scripts/test-classic-addons.sh --profile mists --skip-clone --with-saved-vars --fail-on-addon-errors`
completed with `passed: 9` and `failed: 0`. This matches the CI guard path for
installed Mists addons: saved variables load normally, addon artifacts use the
`*-with-saved-vars-lua-errors.json` suffix, and any addon-induced regression now
fails the harness.

Release-profile proof verification: `scripts/ci-mists-release-proof.sh --skip-clone`
initially failed in the `interaction-audit` lane because
`docs/baselines/mists-panel-interactions.md` had `34` rows while the panel
baseline had `37` passing rows. Added the missing Craft, TradeSkill, and Class
trainer audit rows, then reran
`scripts/ci-mists-release-proof.sh --skip-build --skip-clone`; all lanes passed.
Artifacts were written under `target/mists-release-proof/`, including release
logs, base panel artifacts, and installed-addon panel artifacts for all 9 Mists
addon rows.

Mists `lua-errors` baseline decision: refreshed
`docs/baselines/mists-lua-errors.json` to the current clean `[]` capture after
panel parity reached zero startup errors. The old Phase 7.4 non-empty snapshot
is no longer the active regression baseline.

Scripted panel parity runner: `scripts/mists-panel-parity.sh` validates all
`docs/baselines/mists-panels.md` rows, writes per-panel artifacts under
`target/mists-panel-parity/<slug>/`, and fails on Lua/exec errors, missing or
hidden roots, dump trees without visible renderable descendants, and screenshots
with empty render batches. Current verification covered all 23 manifest rows,
a full sweep through the first 22 rows, and a focused pass for the corrected
`game-menu-options` row.

SavedVariables-backed panel parity verification:
`scripts/mists-panel-parity.sh --skip-build --with-saved-vars` completed with
all 23 panel rows passing. Artifacts were written under
`target/mists-panel-parity-with-saved-vars/<slug>/`, and no panel produced a
SavedVariables-only Lua, root-frame, visual-signal, or visual-baseline
regression.

Addon-backed panel parity verification:
`scripts/test-mists-addon-panels.sh --skip-build --with-saved-vars` completed
with `panel parity passed: 9` and `failed: 0`. The harness symlinked each
installed Mists addon from `tools/classic-addon-manifest.tsv` one at a time,
ran all 23 panel rows through `scripts/mists-panel-parity.sh --with-addons`,
and wrote artifacts under `target/mists-addon-panel-parity/<addon>/<slug>/`.
No installed addon produced panel `lua-errors`, root-frame failures,
low-signal renders, or visual-baseline regressions.

Panel artifact baseline: `docs/baselines/mists-panels.md` now references the
retained `target/mists-panel-parity/<slug>/screenshot.webp` and
`target/mists-panel-parity/<slug>/dump-tree.txt` outputs for every panel row.
The runner test rejects reintroducing `test-backed` placeholders.

Mists panel interaction audit: tests with strong interaction/state assertions
include quest/objective tracking, mail, auction house, bank/guild bank, trade,
PvP queues, LFG/LFR/Raid Finder, pet journal/battle pets, achievements/calendar,
encounter journal, currency/token, macro/keybindings, interface options, action
bars, loot, and game menu options. Follow-up checkboxes above track the panels
where coverage still proves mostly load/render/data rather than a user-visible
interaction state change.

## Current State

- [x] Rebased `classic-profile-rollout` onto latest `origin/master`.
- [x] Mists Blizzard UI source is Gethe `wow-ui-source` at `33d87412` (`5.5.3.67158`).
- [x] Local WoW install active classic product is Pandaria Classic (`wow_classic` build `5.5.3.67158`).
- [x] Classic addon harness can use installed local addon sources with `local:<absolute-path>`.
- [x] Mists addon manifest resolves Pandaria addon rows via `mists-addon:<name>`, using installed addons when present and committed CI fixtures otherwise.
- [x] Feature-gated Pandaria addon source tests exist under `client-mists`.
- [x] Fixed Mists expansion helper state:
  - `GetExpansionLevel()` returns `LE_EXPANSION_MISTS_OF_PANDARIA`.
  - `ClassicExpansionAtLeast()` / `ClassicExpansionAtMost()` match MoP.
- [x] Fixed TOC `[Game]` token under `client-mists` so it resolves to `Mists/`, not `Standard/`.
- [x] Mists startup is clean: `lua-errors` reports `0` distinct errors with saved variables and third-party addons disabled.

## Reproduction Commands

Build Mists:

```bash
cargo build --bin wow-sim --no-default-features --features "sound,gui,casc,client-mists"
```

Headless startup error capture:

```bash
WOW_SIM_NO_SAVED_VARS=1 WOW_SIM_NO_ADDONS=1 timeout 90 \
  target/debug/wow-sim lua-errors \
  > /tmp/mists-lua-errors.json \
  2>/tmp/mists-lua-errors.stderr

jq 'length' /tmp/mists-lua-errors.json
```

GUI startup smoke:

```bash
WOW_SIM_NO_SAVED_VARS=1 WOW_SIM_NO_ADDONS=1 timeout 30 target/debug/wow-sim
```

Focused tests:

```bash
cargo test --no-default-features --features "sound,gui,casc,client-mists" --test mists_compat_bootstrap
cargo test --no-default-features --features "sound,gui,casc,client-mists" --test pandaria_installed_addons
```

## Verification Gate

- [x] `jq 'length' /tmp/mists-lua-errors.json` returns `0` for `WOW_SIM_NO_SAVED_VARS=1 WOW_SIM_NO_ADDONS=1`.
- [x] GUI startup reaches first frame without printing Lua errors.
- [x] `mists_compat_bootstrap` passes.
- [x] `pandaria_installed_addons` passes.
- [x] `readability-audit` passes for changed Rust files.
- [x] `git diff --check` passes.

## Startup Error Triage

### 1. SkillFrame Skill Rank State

- [x] Reproduce `SkillFrame.lua:185: attempt to perform arithmetic on local 'skillRank' (a nil value)`.
- [x] Identify backing API/state feeding `SkillFrame_Update`.
- [x] Add a Mists-gated test that proves the selected skill API returns the shape Blizzard expects.
- [x] Fix the backing API/state model, not the `SkillFrame` output layer.
- [x] Verify the `SkillFrame` errors disappear from `lua-errors`.

Observed errors:

```text
Blizzard_UIPanels_Game/Classic/SkillFrame.lua:185:
attempt to perform arithmetic on local 'skillRank' (a nil value)
```

### 2. Honor Frame API Surface

- [x] Reproduce `HonorSystemEnabled` nil during `HonorFrame_Shared.lua` load.
- [x] Determine expected MoP Classic behavior for `HonorSystemEnabled()`.
- [x] Add a Mists-gated API contract test.
- [x] Implement or correct the backing honor/PvP API state.
- [x] Verify `HonorSystemEnabled` and `GetPVPThisWeekStats` errors disappear.

Observed errors:

```text
HonorFrame_Shared.lua:29: attempt to call global 'HonorSystemEnabled' (a nil value)
attempt to call global 'GetPVPThisWeekStats' (a nil value)
```

### 3. Money Frame Template Initialization

- [x] Reproduce `TradePlayerInputMoneyFrame` missing `copper`.
- [x] Inspect generated XML for the money input frame and its child-key wiring.
- [x] Determine whether the issue is XML template inheritance, parentKey sync, or MoneyFrame API state.
- [x] Add a focused XML/widget test for the missing `copper` child.
- [x] Fix the upstream template/widget construction path.
- [x] Verify money-frame errors disappear.

Observed errors:

```text
TradePlayerInputMoneyFrame:
attempt to index field 'copper' (a nil value)
```

### 4. Product Choice Data Model

- [x] Reproduce `ProductChoice.lua:61: attempt to get length of a nil value`.
- [x] Identify which product-choice table is nil.
- [x] Determine whether MoP Classic expects empty data or an unavailable feature path.
- [x] Add a focused Mists-gated test.
- [x] Fix the backing data/API state or load gating.
- [x] Verify ProductChoice errors disappear.

Observed errors:

```text
Blizzard_UIPanels_Game/Classic/ProductChoice.lua:61:
attempt to get length of a nil value
```

Root cause identified: `C_ProductChoice` exists and `C_ProductChoice.GetChoices`
is callable, but `C_ProductChoice.GetChoices()` returns nil where
`ProductChoiceFrame_ShowAlerts` expects a choices table.

Expected behavior: ProductChoice is an available Classic API/UI path in MoP
Classic. The Mists `Blizzard_UIPanels_Game_Mists.toc` loads
`Classic\ProductChoice.lua` and `.xml`, and the Classic API schema declares
`C_ProductChoice.GetChoices()` as returning a table. Therefore an account with
no product choices should expose empty data (`GetChoices()` returns `{}`), not a
nil/unavailable feature path. If a choice ID is present, `GetProducts(choiceID)`
must also return a table for the item list.

### 5. World Map Opacity State

- [x] Reproduce `WorldMapFrame_SetOpacity` nil `opacity`.
- [x] Find the CVar or saved setting that should seed map opacity.
- [x] Add a focused test for the default value path.
- [x] Fix the backing setting/CVar state.
- [x] Verify world-map opacity errors disappear.

Observed errors:

```text
Blizzard_WorldMap/Cata/Blizzard_WorldMap.lua:790:
attempt to perform arithmetic on local 'opacity' (a nil value)
```

Root cause identified: the minimized WorldMap path calls
`WorldMapFrame_SetOpacity(GetCVar("worldMapOpacity"))`, and the opacity slider
saves back to the same `worldMapOpacity` CVar. The Mists startup CVar default
should seed `worldMapOpacity` as a concrete string value (`"1"` in the current
test contract), so `tonumber`-compatible arithmetic never receives nil.

Backing state fix: `worldMapOpacity: '1'` is present in the shared
`src/cvars.yaml` CVar defaults, and the registered `GetCVar` global reads from
`SimState.cvars`. The fix is therefore the real backing CVar state, not a
Mists-only Lua workaround.

### 6. Nameplate Vertical Scale State

- [x] Reproduce `namePlateVerticalScale` nil in `Blizzard_NamePlates.lua:293`.
- [x] Find the CVar or nameplate option that should provide the value.
- [x] Add a focused Mists-gated test.
- [x] Fix the backing setting/CVar state.
- [x] Verify nameplate scale errors disappear.

Observed errors:

```text
Blizzard_NamePlates/TBC/Blizzard_NamePlates.lua:293:
attempt to perform arithmetic on local 'namePlateVerticalScale' (a nil value)
```

Root cause identified: `NamePlateDriverMixin:UpdateNamePlateOptions()` reads
`tonumber(GetCVar("NamePlateVerticalScale"))` and immediately subtracts `1.0`.
The backing setting is the `NamePlateVerticalScale` CVar, paired with
`NamePlateHorizontalScale` for nameplate width. The shared `src/cvars.yaml`
default should seed `NamePlateVerticalScale` as `"1"`.

Backing state fix: `NamePlateVerticalScale: '1'` and
`NamePlateHorizontalScale: '1'` are present in the shared `src/cvars.yaml`
defaults, and the registered `GetCVar` global reads from `SimState.cvars`.
The fix is the shared CVar backing state, not a Mists-only Lua workaround.

Verification: after rebuilding `wow-sim` with the Mists feature gate,
`WOW_SIM_NO_SAVED_VARS=1 WOW_SIM_NO_ADDONS=1 timeout 90 target/debug/wow-sim lua-errors`
reported `0` distinct errors, and no `NamePlateVerticalScale`,
`namePlateVerticalScale`, `Blizzard_NamePlates`, or
`UpdateNamePlateOptions` strings appeared in stdout or stderr.

### 7. Guild Roster Selection State

- [x] Reproduce `SetGuildRosterSelection` nil.
- [x] Do not add a no-op stub: prior note says that can hang guild roster retry logic.
- [x] Implement real selected-guild-roster-index state with matching getter/setter behavior.
- [x] Add a focused Mists/Wrath-compatible test.
- [x] Verify guild roster errors disappear and startup does not hang.

Observed errors:

```text
FriendsFrame.lua: attempt to call global 'SetGuildRosterSelection' (a nil value)
```

Reproduction: `classic_guild_roster_selection::friends_frame_onload_reproduces_missing_guild_roster_selection`
removes the bootstrap-provided global and exercises the FriendsFrame OnLoad
guild selection call shape, confirming the missing API fails with
`SetGuildRosterSelection` nil before the real selection-state fix is considered.

No-op guard: `classic_guild_roster_selection::guild_roster_selection_setter_is_not_a_noop`
asserts that `SetGuildRosterSelection(7)` is observable through
`GetGuildRosterSelection()` and that clearing back to `0` is also preserved.
The Mists bootstrap comment now explicitly warns that a no-op setter can leave
the guild roster UI retrying against unchanged selection state.

State implementation: `src/mists/compat_bootstrap.lua` owns a local
`selectedGuildRosterIndex`, initializes it to `0`, updates it through
`SetGuildRosterSelection(index)` using `tonumber(index) or 0`, and exposes the
current value through `GetGuildRosterSelection()`. This is real backing state
for the legacy FriendsFrame/GuildFrame selection contract.

Shared test: `classic_guild_roster_selection` runs under both `client-mists`
and `client-wrath`, reproduces the missing setter error, and verifies the
setter/getter state round-trip. Wrath now uses the same `selectedGuildRosterIndex`
contract so the compatibility test proves both classic profiles avoid no-op
guild roster selection behavior.

Verification: after rebuilding `wow-sim` with the Mists feature gate,
`WOW_SIM_NO_SAVED_VARS=1 WOW_SIM_NO_ADDONS=1 timeout 90 target/debug/wow-sim lua-errors`
exited successfully with `0` distinct errors. No `SetGuildRosterSelection`,
`GetGuildRosterSelection`, `FriendsFrame`, `GuildRoster`, `guild roster`, or
`selectedGuildRoster` strings appeared in stdout or stderr, so the old guild
roster startup error is gone and startup did not hang.

### 8. Currency List Compatibility

- [x] Reproduce `GetCurrencyListSize` nil.
- [x] Confirm whether Mists Blizzard Lua calls legacy `GetCurrencyListSize` while simulator only exposes `C_CurrencyInfo.GetCurrencyListSize`.
- [x] Add a focused compatibility test.
- [x] Implement the legacy global as a wrapper over the C API if semantics match.
- [x] Verify currency errors disappear.

Observed errors:

```text
attempt to call global 'GetCurrencyListSize' (a nil value)
```

Reproduction: `mists_currency_list::token_frame_update_reproduces_missing_currency_list_size`
removes the bootstrap-provided legacy global and calls the Mists Cata
`TokenFrame_Update()` path, confirming Blizzard Token UI fails with a
`GetCurrencyListSize` nil global before the compatibility wrapper is considered.

Confirmed: Mists Blizzard Lua calls the legacy global in
`Blizzard_CharacterFrame/Wrath/CharacterFrame.lua`,
`Blizzard_ActionBar/Classic/MainMenuBar.lua`, and both Cata and shared
`Blizzard_TokenUI` sources. The simulator's backing C API registers
`GetCurrencyListSize` under `C_CurrencyInfo` in
`src/c_api/item_spell/c_currency.rs`; the legacy global exists only through the
Mists compatibility wrapper in `src/mists/compat_bootstrap.lua`.

Compatibility test: `mists_currency_list::legacy_currency_list_size_wraps_c_currency_info`
asserts the legacy global returns the same size as
`C_CurrencyInfo.GetCurrencyListSize()` and that the Mists Cata
`TokenFrame_Update()` path runs through the wrapper without throwing.

Implementation: `src/mists/compat_bootstrap.lua` installs
`GetCurrencyListSize()` only when the legacy global is absent. It delegates to
`C_CurrencyInfo.GetCurrencyListSize()` when that namespaced C API is registered,
and returns `0` only as a defensive fallback if the namespace is unavailable.

Verification: after rebuilding `wow-sim` with the Mists feature gate,
`WOW_SIM_NO_SAVED_VARS=1 WOW_SIM_NO_ADDONS=1 timeout 90 target/debug/wow-sim lua-errors`
exited successfully with `0` distinct errors. No `GetCurrencyListSize`,
`C_CurrencyInfo`, `TokenFrame`, `CurrencyList`, `currency list`,
`Blizzard_TokenUI`, or `CharacterFrameTab4` strings appeared in stdout or
stderr, so the old currency startup error is gone.

### 9. Dialog and Popup Text Helpers

- [x] Reproduce `SetBasicMessageDialogText` nil.
- [x] Locate expected helper definition in Blizzard UI or legacy API surface.
- [x] Add a focused test for dialog text mutation.
- [x] Implement the helper against the real dialog frame state.
- [x] Verify dialog helper errors disappear.

Observed errors:

```text
attempt to call global 'SetBasicMessageDialogText' (a nil value)
```

Reproduction: `mists_dialog_helpers::money_frame_set_type_reproduces_missing_basic_message_dialog_helper`
removes the bootstrap-provided helper and calls the Mists Classic
`MoneyFrame_SetType()` invalid-money-type path, confirming Blizzard MoneyFrame
fails with `SetBasicMessageDialogText` nil before the helper implementation is
considered.

Expected helper: Mists Blizzard UI defines `SetBasicMessageDialogText(text, force)`
in `Blizzard_SharedXML/SharedBasicControls.lua`. It updates
`BasicMessageDialog.Text` and shows `BasicMessageDialog` when forced or when the
dialog is not already shown. The backing frame and `Text` parentKey are declared
in `Blizzard_SharedXML/SharedBasicControls.xml`.

Mutation test: `mists_dialog_helpers::basic_message_dialog_helper_updates_text`
installs a concrete `BasicMessageDialog.Text:SetText` fixture and asserts
`SetBasicMessageDialogText("Invalid money type: TEST")` writes that exact text
to the dialog text frame.

Implementation: the Mists compatibility helper now mirrors Blizzard's
`SharedBasicControls.lua` behavior against real `BasicMessageDialog` state. It
writes `BasicMessageDialog.Text`, calls `BasicMessageDialog:Show()`, skips
updates while the dialog is already shown, and honors the `force` argument to
replace shown dialog text.

Verification: after rebuilding `wow-sim` with the Mists feature gate,
`WOW_SIM_NO_SAVED_VARS=1 WOW_SIM_NO_ADDONS=1 timeout 90 target/debug/wow-sim lua-errors`
exited successfully with `0` distinct errors. No `SetBasicMessageDialogText`,
`BasicMessageDialog`, `SharedBasicControls`, `MoneyFrame_SetType`,
`Invalid money type`, or `dialog helper` strings appeared in stdout or stderr.

### 10. Class Color and Miscellaneous Nil Data

- [x] Reproduce `classColor` nil in `Blizzard_Communities/ClubFinder.lua`.
- [x] Identify which class token lacks a color.
- [x] Add a focused class-color/default data test.
- [x] Fix the backing class-color data.
- [x] Verify class-color errors disappear.

Observed errors:

```text
Blizzard_Communities/ClubFinder.lua:564:
attempt to index local 'classColor' (a nil value)
```

Reproduction: `mists_class_colors::club_finder_setup_menu_reproduces_missing_class_color`
loads Mists `Blizzard_Communities/ClubFinder.lua`, supplies a class token whose
`GetClassColorObj()` result is nil, and exercises
`ClubLookingForDropdownMixin:SetupMenu()`, confirming the nil `classColor`
failure before the backing color data fix is considered.

Missing token identified: `EVOKER`. The shared simulator class data includes
`EVOKER` as class 13 (`src/lua_api/game_data.rs`), but Mists
`Blizzard_SharedXML/Blizzard_SharedXML_Mists.toc` loads the TBC
`ClassColors.lua`, whose `RAID_CLASS_COLORS` table includes `MONK` and
`DEMONHUNTER` but no `EVOKER`. Any Mists path that iterates the uncapped shared
class list can therefore get nil from `GetClassColorObj("EVOKER")`.

Default data test: `mists_class_colors::mists_visible_classes_have_color_data`
asserts Mists exposes exactly 11 visible classes, includes `MONK`, excludes
`DEMONHUNTER` and `EVOKER`, and every visible `GetClassInfo()` class token has
a `GetClassColorObj()` result.

Backing data fix: `src/mists/compat_bootstrap.lua` caps `GetNumClasses()` at
`11` for Mists, matching the MoP class roster through `MONK`. This keeps
Mists-only Blizzard code from iterating shared retail-backed `DEMONHUNTER` and
`EVOKER` class tokens that are outside MoP's visible class set, so every visible
class token has matching `RAID_CLASS_COLORS` data.

Verification: after rebuilding `wow-sim` with the Mists feature gate,
`WOW_SIM_NO_SAVED_VARS=1 WOW_SIM_NO_ADDONS=1 timeout 90 target/debug/wow-sim lua-errors`
reported `0` distinct errors. No `classColor`, `ClubFinder`,
`RAID_CLASS_COLORS`, `DEMONHUNTER`, or `EVOKER` strings appeared in stdout or
stderr.

## Addon Harness Follow-Up

- [x] After base Mists startup is clean, run each installed Pandaria addon through `scripts/test-classic-addons.sh --profile mists`.
- [x] For each addon, record addon-induced errors separately from base startup.
- [x] Promote shared missing APIs to `src/mists/compat_bootstrap.lua` or Rust backing systems.
- [x] Keep per-addon quirks under `tools/classic-addon-compat/<addon>/`.
- [x] Do not update `docs/baselines/mists-lua-errors.json` to bless known startup errors; use it only after the base startup is clean or intentionally documented.

Installed Mists targets:

- [x] `AllTheThings`
- [x] `Auctionator`
- [x] `BlizzMove`
- [x] `DeModal`
- [x] `DialogueUI`
- [x] `Leatrix_Maps`
- [x] `Leatrix_Plus`
- [x] `Plater`
- [x] `SimpleItemLevel`

Harness run: `scripts/test-classic-addons.sh --profile mists` completed with
`passed: 9` and `failed: 0`. All nine addon runs booted successfully and wrote
empty `target/addon-harness/*-lua-errors.json` arrays, reporting `0` distinct
errors total and `0` addon-induced errors versus the Mists baseline.

Per-addon error split:

| Addon | Total startup errors | Addon-induced errors |
|---|---:|---:|
| `AllTheThings` | 0 | 0 |
| `Auctionator` | 0 | 0 |
| `BlizzMove` | 0 | 0 |
| `DeModal` | 0 | 0 |
| `DialogueUI` | 0 | 0 |
| `Leatrix_Maps` | 0 | 0 |
| `Leatrix_Plus` | 0 | 0 |
| `Plater` | 0 | 0 |
| `SimpleItemLevel` | 0 | 0 |

Promotion audit: no additional Mists shared API gaps were promotable after the
addon harness run. Every `target/addon-harness/*-lua-errors.json` file reported
`0` errors, and the only existing per-addon shim under `tools/classic-addon-compat`
is the empty Wrath `Bartender4` seed. No new Mists bootstrap or Rust backing
state was added for this pass.

Per-addon quirk audit: current installed Mists addons do not need per-addon
compat entries because each addon produced `0` addon-induced errors. Future
addon-specific fixes should follow the existing `tools/classic-addon-compat`
layout documented in `tools/classic-addon-compat/README.md`; shared gaps still
belong in the Mists bootstrap or Rust backing systems.

Baseline guardrail: `docs/baselines/mists-lua-errors.json` is now refreshed to
the clean panel-parity baseline because base Mists startup reports `0` distinct
errors. Do not rewrite it to bless future startup errors; only refresh it again
after another verified clean capture.

## Notes

- `PLAN.md` remains the repo dispatch board and currently contains an unrelated `src/paths.rs` readability refactor item.
- `PLAN.mists.md` tracks the Pandaria Classic startup/addon effort only.
- Prefer upstream simulator state/model fixes over downstream Blizzard Lua shims.
- Shims are acceptable only when they represent a real legacy API compatibility surface or an explicitly temporary stopgap with a retirement path.
