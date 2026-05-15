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
| Base panel parity execution | `38` panel rows passed with per-panel `lua-errors`, dump-tree, screenshot, root-frame, and visual checks | `target/mists-local-completion-audit/panel-parity.log`; artifacts under `target/mists-local-completion-audit/panel-parity/` |
| Base panel parity with normal SavedVariables | `38` panel rows passed; all per-panel `lua-errors.json` files have length `0`; no saved-var-only panel regression found | `target/mists-local-saved-vars-panel-parity.log`; artifacts under `target/mists-local-saved-vars-panel-parity/` |
| Retained local asset-backed visual artifact audit | All `38` documented panel rows have retained local saved-vars screenshots, frame dumps, and per-panel `lua-errors.json`; no empty retained artifact found | `target/mists-local-saved-vars-panel-parity/` |
| Installed Mists addon panel manifest validation | `9` addon rows validated | `target/mists-local-completion-audit/addon-panel-validation.log` |
| Live connected-GUI smoke validation and execution | Passed idle HUD, specialization Learn-to-talents, and micro-menu probes | `target/mists-local-completion-audit/live-gui-smoke.log` |
| Post-CatalogShop `SOUNDKIT` live connected-GUI smoke | Passed idle HUD, specialization Learn-to-talents, Store micro-button, and other micro-menu probes after the sound constants fix | `target/mists-local-completion-audit/live-gui-smoke-after-soundkit.log` |
| Installed Mists addon startup matrix | `passed: 9`, `failed: 0`, all rows `0` addon-induced errors | `target/mists-local-completion-audit/installed-addon-startup.log` |
| Post-CatalogShop `SOUNDKIT` startup regression check | Base `lua-errors` length `0`; installed-addon matrix `passed: 9`, `failed: 0` after seeding the looping CatalogShop sound constants | `target/mists-local-completion-audit/base-lua-errors-after-soundkit.json`; `target/mists-local-completion-audit/installed-addon-startup-after-soundkit.log` |
| Mists panel interaction audit | `1` Rust test suite passed; every pass panel remains represented in the interaction audit | `target/mists-local-completion-audit/interaction-audit.log` |
| Post-CatalogShop `SOUNDKIT` interaction audit | `1` Rust test suite passed after the Store/CatalogShop soundkit coverage change | `cargo test --no-default-features --features sound,gui,casc,client-mists --test mists_panel_interaction_audit -- --nocapture` |
| Post-CatalogShop `SOUNDKIT` focused Store panel parity | Store/CatalogShop row passed; per-panel `lua-errors` length `0`, root dump and screenshot refreshed after the sound constants fix | `target/mists-local-completion-audit/store-commercial-after-soundkit.log`; artifacts under `target/mists-panel-parity/store-commercial/` |
| Post-CatalogShop `SOUNDKIT` full panel parity | All `38` panel rows passed; all per-panel `lua-errors.json` files have length `0`; screenshots and frame dumps refreshed | `target/mists-local-completion-audit/panel-parity-after-soundkit.log`; artifacts under `target/mists-panel-parity/` |
| Post-CatalogShop `SOUNDKIT` saved-vars full panel parity | All `38` panel rows have current SavedVariables-backed evidence with per-panel `lua-errors.json` length `0`; the resumed focused checks passed the final `nameplates`, `loot`, and `game-menu-options` rows | `target/mists-local-completion-audit/panel-parity-with-saved-vars-after-soundkit.log`; artifacts under `target/mists-local-saved-vars-panel-parity/` and focused final-row artifacts under `target/mists-local-saved-vars-panel-parity-focused/` |
| Post-Spellbook `CastSpell(slot, bookType)` focused panel parity | Spellbook/professions row passed with and without normal SavedVariables after the legacy spellbook cast fix | Artifacts under `target/mists-focused-spellbook-professions-after-castspell/base/` and `target/mists-focused-spellbook-professions-after-castspell/with-saved-vars/` |
| Post-Spellbook `CastSpell(slot, bookType)` startup regression check | Base Mists `lua-errors` length `0`; installed-addon startup matrix `passed: 9`, `failed: 0`, with all addon `lua-errors.json` files length `0` | `target/mists-local-completion-audit/base-lua-errors-after-castspell.json`; `target/mists-local-completion-audit/installed-addon-startup-after-castspell.log` |
| Post-Spellbook `CastSpell(slot, bookType)` live connected-GUI smoke | Passed idle HUD, specialization Learn-to-talents, Spellbook micro-button, Store micro-button, and the remaining visible micro-menu probes after the legacy spellbook cast fix | Artifacts under `target/mists-live-gui-smoke-after-castspell/` |
| Post-Spellbook `CastSpell(slot, bookType)` addon Spellbook sample | `BlizzMove` with normal SavedVariables passed the Spellbook/professions panel row; per-panel `lua-errors` length `0`, root dump and screenshot written | Artifacts under `target/mists-local-addon-panel-sample/blizzmove-spellbook-with-saved-vars-after-castspell/BlizzMove/spellbook-professions/` |
| Post-Spellbook `CastSpell(slot, bookType)` full panel parity | All `38` panel rows passed after the legacy spellbook cast fix; all per-panel `lua-errors.json` files have length `0`; screenshots and frame dumps refreshed | `target/mists-local-completion-audit/panel-parity-after-castspell.log`; artifacts under `target/mists-panel-parity-after-castspell/` |
| Post-Spellbook `CastSpell(slot, bookType)` saved-vars full panel parity | All `38` panel rows passed with normal SavedVariables after the legacy spellbook cast fix; all per-panel `lua-errors.json` files have length `0`; screenshots and frame dumps refreshed | `target/mists-local-completion-audit/panel-parity-with-saved-vars-after-castspell.log`; artifacts under `target/mists-panel-parity-with-saved-vars-after-castspell/` |
| Post-pet UI state guard focused Character panel parity | Character panel row passed with and without normal SavedVariables after backing `HasPetUI()` with simulator pet action state; both per-panel `lua-errors.json` files have length `0` | Artifacts under `target/mists-character-after-pet-ui-guard/base/` and `target/mists-character-after-pet-ui-guard/with-saved-vars/` |
| Post-pet UI state guard startup regression check | Base Mists `lua-errors` length `0`; installed-addon startup matrix `passed: 9`, `failed: 0`, with `0` addon-induced errors for every installed Mists addon | `target/mists-pet-ui-startup-regression/base-lua-errors.json`; `target/addon-harness/*-lua-errors.json` |
| Post-pet open-path focused addon Character panel matrix | All `9` installed Mists addons passed the Character panel row with normal SavedVariables after the real open-path `PET_UI_UPDATE` fix; every per-addon Character `lua-errors.json` file has length `0` | Artifacts under `target/mists-character-addon-panel-after-pet-ui-open-path-fix/` |
| Post-pet open-path focused Character panel parity | Base Mists `lua-errors` length `0`; Character panel row passed with and without normal SavedVariables after startup began firing `PET_UI_UPDATE`; both per-panel Character `lua-errors.json` files have length `0` | `target/mists-pet-open-path-refresh/base-lua-errors.json`; artifacts under `target/mists-character-after-pet-open-path-fix/base/` and `target/mists-character-after-pet-open-path-fix/with-saved-vars/` |

This audit is intentionally local and non-release: it uses debug `wow-sim` /
`wow-cli` binaries and does not run the release-proof wrapper. Its purpose is to
confirm the current working tree still satisfies the core Mists parity gates
before adding or promoting any narrower follow-up task.

The only intentionally bounded lane in this local audit is installed-addon
panel validation: it confirms all 9 installed Mists addon rows are present and
resolvable, while the expensive full installed-addon panel screenshot matrix
remains deliberately outside this local completion pass. The retained
non-release installed-addon panel parity sample below strengthens addon-panel
evidence without reintroducing release-proof or CI texture work.

## Local Installed-Addon Panel Sample

Latest bounded local sample: 2026-05-14 on `classic-profile-rollout`, using
debug `wow-sim` plus the installed `BlizzMove` and `DeModal` UI-mutating addons
under test.

| Addon | Panel row | Result | Artifact |
|---|---|---|---|
| `BlizzMove` | Talents and glyphs | Passed; per-panel `lua-errors` length `0`, root dump and screenshot written | `target/mists-local-addon-panel-sample/blizzmove-talents.log`; artifacts under `target/mists-local-addon-panel-sample/blizzmove-talents/BlizzMove/talents-glyphs/` |
| `BlizzMove` | Action bars, micro menu, bag bar, status bars | Passed; per-panel `lua-errors` length `0`, root dump and screenshot written | `target/mists-local-addon-panel-sample/blizzmove-action-bars.log`; artifacts under `target/mists-local-addon-panel-sample/blizzmove-action-bars/BlizzMove/action-bars/` |
| `BlizzMove` | Store, CatalogShop, WowToken, and SimpleCheckout | Passed; per-panel `lua-errors` length `0`, root dump and screenshot written | `target/mists-local-addon-panel-sample/blizzmove-store-commercial.log`; artifacts under `target/mists-local-addon-panel-sample/blizzmove-store-commercial/BlizzMove/store-commercial/` |
| `BlizzMove` + normal SavedVariables | Store, CatalogShop, WowToken, and SimpleCheckout | Passed; per-panel `lua-errors` length `0`, root dump and screenshot written | `target/mists-local-addon-panel-sample/blizzmove-store-commercial-with-saved-vars.log`; artifacts under `target/mists-local-addon-panel-sample/blizzmove-store-commercial-with-saved-vars/BlizzMove/store-commercial/` |
| `BlizzMove` + normal SavedVariables | Character panel: paperdoll, stats, titles, equipment manager | Passed after the pet-tab open-path fix; per-panel `lua-errors` length `0`, visible `CharacterFrame` root dump and screenshot written | `target/mists-local-addon-panel-sample/blizzmove-character-with-saved-vars-after-scope-guard.log`; artifacts under `target/mists-local-addon-panel-sample/blizzmove-character-with-saved-vars-after-scope-guard/BlizzMove/character/` |
| `BlizzMove` + normal SavedVariables | Spellbook and professions | Passed after the scope-limit guard; per-panel `lua-errors` length `0`, visible `SpellBookFrame` root dump and screenshot written | `target/mists-local-addon-panel-sample/blizzmove-spellbook-with-saved-vars-after-scope-guard.log`; artifacts under `target/mists-local-addon-panel-sample/blizzmove-spellbook-with-saved-vars-after-scope-guard/BlizzMove/spellbook-professions/` |
| `BlizzMove` + normal SavedVariables | Quest log and objective tracker | Passed after the scope-limit guard; per-panel `lua-errors` length `0`, visible `QuestLogFrame` root dump and screenshot written | `target/mists-local-addon-panel-sample/blizzmove-quest-log-with-saved-vars-after-scope-guard.log`; artifacts under `target/mists-local-addon-panel-sample/blizzmove-quest-log-with-saved-vars-after-scope-guard/BlizzMove/quest-log/` |
| `BlizzMove` + normal SavedVariables | World map | Passed after the scope-limit guard; per-panel `lua-errors` length `0`, visible `WorldMapFrame` root dump and screenshot written | `target/mists-local-addon-panel-sample/blizzmove-world-map-with-saved-vars-after-scope-guard.log`; artifacts under `target/mists-local-addon-panel-sample/blizzmove-world-map-with-saved-vars-after-scope-guard/BlizzMove/world-map/` |
| `BlizzMove` + normal SavedVariables | Auction House: browse, bid, post, cancel | Passed after the scope-limit guard; per-panel `lua-errors` length `0`, visible `AuctionFrame` root dump and screenshot written | `target/mists-local-addon-panel-sample/blizzmove-auction-house-with-saved-vars-after-scope-guard.log`; artifacts under `target/mists-local-addon-panel-sample/blizzmove-auction-house-with-saved-vars-after-scope-guard/BlizzMove/auction-house/` |
| `BlizzMove` + normal SavedVariables | Mail: inbox, send, attachments, COD | Passed after the scope-limit guard; per-panel `lua-errors` length `0`, visible Blizzard `MailFrame` root dump and screenshot written | `target/mists-local-addon-panel-sample/blizzmove-mail-with-saved-vars-after-scope-guard.log`; artifacts under `target/mists-local-addon-panel-sample/blizzmove-mail-with-saved-vars-after-scope-guard/BlizzMove/mail/` |
| `BlizzMove` + normal SavedVariables | Collections: mounts, pets, toys, heirlooms, transmog | Passed after the scope-limit guard; per-panel `lua-errors` length `0`, visible `CollectionsJournal` root dump and screenshot written | `target/mists-local-addon-panel-sample/blizzmove-collections-with-saved-vars-after-scope-guard.log`; artifacts under `target/mists-local-addon-panel-sample/blizzmove-collections-with-saved-vars-after-scope-guard/BlizzMove/collections/` |
| `BlizzMove` + normal SavedVariables | Pet Journal and Battle Pet UI | Passed after the scope-limit guard; per-panel `lua-errors` length `0`, visible Pet Journal `CollectionsJournal` root dump and screenshot written | `target/mists-local-addon-panel-sample/blizzmove-pet-journal-with-saved-vars-after-scope-guard.log`; artifacts under `target/mists-local-addon-panel-sample/blizzmove-pet-journal-with-saved-vars-after-scope-guard/BlizzMove/pet-journal/` |
| `BlizzMove` + normal SavedVariables | Bank, ReagentBank, Void Storage, Guild Bank | Passed after the scope-limit guard; per-panel `lua-errors` length `0`, visible `BankFrame` root dump and screenshot written | `target/mists-local-addon-panel-sample/blizzmove-bank-storage-with-saved-vars-after-scope-guard.log`; artifacts under `target/mists-local-addon-panel-sample/blizzmove-bank-storage-with-saved-vars-after-scope-guard/BlizzMove/bank-storage/` |
| `BlizzMove` + normal SavedVariables | Trade window and `TradePlayerInputMoneyFrame` | Passed after the scope-limit guard; per-panel `lua-errors` length `0`, visible `TradeFrame` root dump and screenshot written | `target/mists-local-addon-panel-sample/blizzmove-trade-with-saved-vars-after-scope-guard.log`; artifacts under `target/mists-local-addon-panel-sample/blizzmove-trade-with-saved-vars-after-scope-guard/BlizzMove/trade/` |
| `BlizzMove` + normal SavedVariables | TradeSkill panel | Passed after the same trade shard filter also matched TradeSkill; per-panel `lua-errors` length `0`, visible `TradeSkillFrame` root dump and screenshot written | `target/mists-local-addon-panel-sample/blizzmove-trade-with-saved-vars-after-scope-guard.log`; artifacts under `target/mists-local-addon-panel-sample/blizzmove-trade-with-saved-vars-after-scope-guard/BlizzMove/trade-skill/` |
| `BlizzMove` + normal SavedVariables | Inspect and guild control LoD panels | Passed after the scope-limit guard; per-panel `lua-errors` length `0`, opener asserted renderable `GuildControlUI`, and visible `InspectFrame` root dump and screenshot were written | `target/mists-local-addon-panel-sample/blizzmove-inspect-guild-control-with-saved-vars-after-scope-guard.log`; artifacts under `target/mists-local-addon-panel-sample/blizzmove-inspect-guild-control-with-saved-vars-after-scope-guard/BlizzMove/inspect-guild-control/` |
| `BlizzMove` + normal SavedVariables | AddOn list and UI management LoD panels | Passed after the scope-limit guard; per-panel `lua-errors` length `0`, visible `AddonList` root dump and screenshot written | `target/mists-local-addon-panel-sample/blizzmove-addon-list-with-saved-vars-after-scope-guard.log`; artifacts under `target/mists-local-addon-panel-sample/blizzmove-addon-list-with-saved-vars-after-scope-guard/BlizzMove/addon-list/` |
| `BlizzMove` + normal SavedVariables | Friends, Who, Guild, Communities, Club Finder | Passed after the scope-limit guard; per-panel `lua-errors` length `0`, visible Blizzard `FriendsFrame` root dump and screenshot written | `target/mists-local-addon-panel-sample/blizzmove-social-with-saved-vars-after-scope-guard.log`; artifacts under `target/mists-local-addon-panel-sample/blizzmove-social-with-saved-vars-after-scope-guard/BlizzMove/social/` |
| `BlizzMove` + normal SavedVariables | PvP UI: HonorFrame, BG queue, Conquest | Passed after the scope-limit guard; per-panel `lua-errors` length `0`, visible `PVPQueueFrame` root dump and screenshot written | `target/mists-local-addon-panel-sample/blizzmove-pvp-with-saved-vars-after-scope-guard.log`; artifacts under `target/mists-local-addon-panel-sample/blizzmove-pvp-with-saved-vars-after-scope-guard/BlizzMove/pvp/` |
| `BlizzMove` + normal SavedVariables | LFG, LFR, Raid Browser | Passed after the scope-limit guard; per-panel `lua-errors` length `0`, visible Blizzard `PVEFrame` root dump and screenshot written | `target/mists-local-addon-panel-sample/blizzmove-lfg-lfr-with-saved-vars-after-scope-guard.log`; artifacts under `target/mists-local-addon-panel-sample/blizzmove-lfg-lfr-with-saved-vars-after-scope-guard/BlizzMove/lfg-lfr/` |
| `DeModal` + normal SavedVariables | Store, CatalogShop, WowToken, and SimpleCheckout | Passed after seeding the CatalogShop looping `SOUNDKIT` constants; per-panel `lua-errors` length `0`, root dump and screenshot written | `target/mists-local-addon-panel-sample/demodal-store-commercial-with-saved-vars.log`; artifacts under `target/mists-local-addon-panel-sample/demodal-store-commercial-with-saved-vars/DeModal/store-commercial/` |
| `DeModal` + normal SavedVariables | Character panel: paperdoll, stats, titles, equipment manager | Passed; per-panel `lua-errors` length `0`, root dump and screenshot written | `target/mists-local-addon-panel-sample/demodal-character-with-saved-vars.log`; artifacts under `target/mists-local-addon-panel-sample/demodal-character-with-saved-vars/DeModal/character/` |
| `DeModal` + normal SavedVariables | Spellbook and professions | Passed after the legacy `CastSpell(slot, bookType)` fix; per-panel `lua-errors` length `0`, root dump and screenshot written | `target/mists-local-addon-panel-sample/demodal-spellbook-with-saved-vars-after-castspell.log`; artifacts under `target/mists-local-addon-panel-sample/demodal-spellbook-with-saved-vars-after-castspell/DeModal/spellbook-professions/` |
| `DeModal` + normal SavedVariables | TradeSkill panel | Passed after the legacy `CastSpell(slot, bookType)` fix; per-panel `lua-errors` length `0`, root dump and screenshot written | `target/mists-local-addon-panel-sample/demodal-trade-skill-with-saved-vars-after-castspell.log`; artifacts under `target/mists-local-addon-panel-sample/demodal-trade-skill-with-saved-vars-after-castspell/DeModal/trade-skill/` |
| `Leatrix_Plus` + normal SavedVariables | Spellbook and professions | Passed after the legacy `CastSpell(slot, bookType)` fix; per-panel `lua-errors` length `0`, root dump and screenshot written | `target/mists-local-addon-panel-sample/leatrix-plus-spellbook-with-saved-vars-after-castspell.log`; artifacts under `target/mists-local-addon-panel-sample/leatrix-plus-spellbook-with-saved-vars-after-castspell/Leatrix_Plus/spellbook-professions/` |
| `Leatrix_Plus` + normal SavedVariables | TradeSkill panel | Passed after the legacy `CastSpell(slot, bookType)` fix; per-panel `lua-errors` length `0`, root dump and screenshot written | `target/mists-local-addon-panel-sample/leatrix-plus-trade-skill-with-saved-vars-after-castspell.log`; artifacts under `target/mists-local-addon-panel-sample/leatrix-plus-trade-skill-with-saved-vars-after-castspell/Leatrix_Plus/trade-skill/` |
| `AllTheThings` + normal SavedVariables | Collections: mounts, pets, toys, heirlooms, transmog | Passed after the legacy `CastSpell(slot, bookType)` fix; per-panel `lua-errors` length `0`, root dump and screenshot written | `target/mists-local-addon-panel-sample/allthethings-collections-with-saved-vars-after-castspell.log`; artifacts under `target/mists-local-addon-panel-sample/allthethings-collections-with-saved-vars-after-castspell/AllTheThings/collections/` |
| `AllTheThings` + normal SavedVariables | TradeSkill panel | Passed after the legacy `CastSpell(slot, bookType)` fix; per-panel `lua-errors` length `0`, root dump and screenshot written | `target/mists-local-addon-panel-sample/allthethings-trade-skill-with-saved-vars-after-castspell.log`; artifacts under `target/mists-local-addon-panel-sample/allthethings-trade-skill-with-saved-vars-after-castspell/AllTheThings/trade-skill/` |
| `Auctionator` + normal SavedVariables | Auction House: browse, bid, post, cancel | Passed after the legacy `CastSpell(slot, bookType)` fix; per-panel `lua-errors` length `0`, root dump and screenshot written | `target/mists-local-addon-panel-sample/auctionator-auction-house-with-saved-vars-after-castspell.log`; artifacts under `target/mists-local-addon-panel-sample/auctionator-auction-house-with-saved-vars-after-castspell/Auctionator/auction-house/` |
| `Auctionator` + normal SavedVariables | Mail: inbox, send, attachments, COD | Passed after the legacy `CastSpell(slot, bookType)` fix; per-panel `lua-errors` length `0`, root dump and screenshot written | `target/mists-local-addon-panel-sample/auctionator-mail-with-saved-vars-after-castspell.log`; artifacts under `target/mists-local-addon-panel-sample/auctionator-mail-with-saved-vars-after-castspell/Auctionator/mail/` |
| `Plater` + normal SavedVariables | Nameplates | Passed after the legacy `CastSpell(slot, bookType)` fix; per-panel `lua-errors` length `0`, root dump and screenshot written | `target/mists-local-addon-panel-sample/plater-nameplates-with-saved-vars-after-castspell.log`; artifacts under `target/mists-local-addon-panel-sample/plater-nameplates-with-saved-vars-after-castspell/Plater/nameplates/` |
| `Plater` + normal SavedVariables | Action bars, micro menu, bag bar, status bars | Passed after the legacy `CastSpell(slot, bookType)` fix; per-panel `lua-errors` length `0`, root dump and screenshot written | `target/mists-local-addon-panel-sample/plater-action-bars-with-saved-vars-after-castspell.log`; artifacts under `target/mists-local-addon-panel-sample/plater-action-bars-with-saved-vars-after-castspell/Plater/action-bars/` |
| `Leatrix_Maps` + normal SavedVariables | World map | Passed after the legacy `CastSpell(slot, bookType)` fix; per-panel `lua-errors` length `0`, root dump and screenshot written | `target/mists-local-addon-panel-sample/leatrix-maps-world-map-with-saved-vars-after-castspell.log`; artifacts under `target/mists-local-addon-panel-sample/leatrix-maps-world-map-with-saved-vars-after-castspell/Leatrix_Maps/world-map/` |
| `Leatrix_Maps` + normal SavedVariables | Quest log and objective tracker | Passed after the legacy `CastSpell(slot, bookType)` fix; per-panel `lua-errors` length `0`, root dump and screenshot written | `target/mists-local-addon-panel-sample/leatrix-maps-quest-log-with-saved-vars-after-castspell.log`; artifacts under `target/mists-local-addon-panel-sample/leatrix-maps-quest-log-with-saved-vars-after-castspell/Leatrix_Maps/quest-log/` |
| `DialogueUI` + normal SavedVariables | Quest choice LoD dialog | Passed after the legacy `CastSpell(slot, bookType)` fix; per-panel `lua-errors` length `0`, root dump and screenshot written | `target/mists-local-addon-panel-sample/dialogueui-quest-choice-with-saved-vars-after-castspell.log`; artifacts under `target/mists-local-addon-panel-sample/dialogueui-quest-choice-with-saved-vars-after-castspell/DialogueUI/quest-choice/` |
| `DialogueUI` + normal SavedVariables | Quest log and objective tracker | Passed after the legacy `CastSpell(slot, bookType)` fix; per-panel `lua-errors` length `0`, root dump and screenshot written | `target/mists-local-addon-panel-sample/dialogueui-quest-log-with-saved-vars-after-castspell.log`; artifacts under `target/mists-local-addon-panel-sample/dialogueui-quest-log-with-saved-vars-after-castspell/DialogueUI/quest-log/` |
| `SimpleItemLevel` + normal SavedVariables | Character panel: paperdoll, stats, titles, equipment manager | Passed after the legacy `CastSpell(slot, bookType)` fix; per-panel `lua-errors` length `0`, root dump and screenshot written | `target/mists-local-addon-panel-sample/simpleitemlevel-character-with-saved-vars-after-castspell.log`; artifacts under `target/mists-local-addon-panel-sample/simpleitemlevel-character-with-saved-vars-after-castspell/SimpleItemLevel/character/` |
| `SimpleItemLevel` + normal SavedVariables | Inspect and guild control LoD panels | Passed after the legacy `CastSpell(slot, bookType)` fix; per-panel `lua-errors` length `0`, root dump and screenshot written | `target/mists-local-addon-panel-sample/simpleitemlevel-inspect-guild-control-with-saved-vars-after-castspell.log`; artifacts under `target/mists-local-addon-panel-sample/simpleitemlevel-inspect-guild-control-with-saved-vars-after-castspell/SimpleItemLevel/inspect-guild-control/` |

The full installed-addon screenshot matrix remains deferred locally because it
is the expensive 9-addon by 38-panel release-proof-style lane. This sample keeps
addon-panel evidence moving by covering the recently touched talent,
HUD/action-bar, Store/CatalogShop, CharacterFrame, Spellbook/professions,
Collections, TradeSkill, Auction House, Mail, Nameplates, World Map, Quest log,
Quest choice, Character, and Inspect surfaces with frame-moving, UI-option,
data-heavy, auction-focused, nameplate, map, dialogue, and item-level addons
enabled, including Store/CatalogShop rows with normal SavedVariables loaded,
without adding CI texture requirements or rerunning the release-proof wrapper.

## Validation Scope Limits

The full installed-addon screenshot matrix remains a validation scope limit for
local runs. The bounded installed-addon samples prove selected panels under each
installed addon with normal SavedVariables enabled; they are not proof that every
installed-addon panel screenshot has been exercised locally.

### Bounded Sample Coverage Audit

The installed Mists addon manifest currently contains nine rows. Each row now
has at least one bounded panel sample with normal SavedVariables enabled:

| Addon | SavedVariables panel evidence |
|---|---|
| `AllTheThings` | Collections, TradeSkill |
| `Auctionator` | Auction House, Mail |
| `BlizzMove` | Store/CatalogShop |
| `DeModal` | Store/CatalogShop, Character, Spellbook/professions, TradeSkill |
| `DialogueUI` | Quest choice, Quest log |
| `Leatrix_Maps` | World Map, Quest log |
| `Leatrix_Plus` | Spellbook/professions, TradeSkill |
| `Plater` | Nameplates, Action bars |
| `SimpleItemLevel` | Character, Inspect/Guild Control |

Local enforcement now exists in
`tests/mists_panel_parity_runner.rs::bounded_saved_vars_addon_samples_cover_installed_mists_addons`:
the checker compares the installed Mists addon manifest rows against the
SavedVariables sample rows recorded above and fails when a future addon lacks at
least one retained saved-vars panel sample.

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
