# Mists Panel Parity Baseline

Captured 2026-05-12 on `classic-profile-rollout` after the panel parity audit
and the clean installed-addon harness run. This baseline tracks the human-facing
panel state for Pandaria Classic separately from the machine-readable
`lua-errors` baseline.

Status meanings:

- `Pass`: focused panel coverage loads the Blizzard panel, exercises its main
  interaction path, and leaves Mists `lua-errors` clean.
- `Watch`: panel works but has an explicitly documented gap to revisit.
- `Fail`: panel has known broken rendering, loading, or interaction behavior.

Screenshots are not committed as binary baselines here. The artifact column
records the retained local runner outputs produced by
`scripts/mists-panel-parity.sh`: a filtered screenshot and the matching
frame-tree dump for each panel row.

| Panel | Status | Artifacts | Gap notes |
|---|---|---|---|
| Character panel: paperdoll, stats, titles, equipment manager | Pass | screenshot: `target/mists-panel-parity-with-saved-vars-after-castspell/character/screenshot.webp`; dump: `target/mists-panel-parity-with-saved-vars-after-castspell/character/dump-tree.txt` | No known current gaps. |
| Spellbook and professions | Pass | screenshot: `target/mists-panel-parity-with-saved-vars-after-castspell/spellbook-professions/screenshot.webp`; dump: `target/mists-panel-parity-with-saved-vars-after-castspell/spellbook-professions/dump-tree.txt` | No known current gaps. |
| Talents and glyphs | Pass | screenshot: `target/mists-panel-parity-with-saved-vars-after-castspell/talents-glyphs/screenshot.webp`; dump: `target/mists-panel-parity-with-saved-vars-after-castspell/talents-glyphs/dump-tree.txt` | Mists talent rows and glyph slots are intentionally distinct from retail trees. |
| Quest log and objective tracker | Pass | screenshot: `target/mists-panel-parity-with-saved-vars-after-castspell/quest-log/screenshot.webp`; dump: `target/mists-panel-parity-with-saved-vars-after-castspell/quest-log/dump-tree.txt` | No known current gaps. |
| World map | Pass | screenshot: `target/mists-panel-parity-with-saved-vars-after-castspell/world-map/screenshot.webp`; dump: `target/mists-panel-parity-with-saved-vars-after-castspell/world-map/dump-tree.txt` | Zone overlays, quest pins, and opacity path are covered. |
| Mail: inbox, send, attachments, COD | Pass | screenshot: `target/mists-panel-parity-with-saved-vars-after-castspell/mail/screenshot.webp`; dump: `target/mists-panel-parity-with-saved-vars-after-castspell/mail/dump-tree.txt` | No known current gaps. |
| Auction House: browse, bid, post, cancel | Pass | screenshot: `target/mists-panel-parity-with-saved-vars-after-castspell/auction-house/screenshot.webp`; dump: `target/mists-panel-parity-with-saved-vars-after-castspell/auction-house/dump-tree.txt` | No known current gaps. |
| AddOn list and UI management LoD panels | Pass | screenshot: `target/mists-panel-parity-with-saved-vars-after-castspell/addon-list/screenshot.webp`; dump: `target/mists-panel-parity-with-saved-vars-after-castspell/addon-list/dump-tree.txt` | Covers the LoD AddOnList management frame outside normal game-panel openers. |
| Bank, ReagentBank, Void Storage, Guild Bank | Pass | screenshot: `target/mists-panel-parity-with-saved-vars-after-castspell/bank-storage/screenshot.webp`; dump: `target/mists-panel-parity-with-saved-vars-after-castspell/bank-storage/dump-tree.txt` | No known current gaps. |
| Trade window and `TradePlayerInputMoneyFrame` | Pass | screenshot: `target/mists-panel-parity-with-saved-vars-after-castspell/trade/screenshot.webp`; dump: `target/mists-panel-parity-with-saved-vars-after-castspell/trade/dump-tree.txt` | Money input child-key root cause remains fixed. |
| Friends, Who, Guild, Communities, Club Finder | Pass | screenshot: `target/mists-panel-parity-with-saved-vars-after-castspell/social/screenshot.webp`; dump: `target/mists-panel-parity-with-saved-vars-after-castspell/social/dump-tree.txt` | No known current gaps. |
| Inspect and guild control LoD panels | Pass | screenshot: `target/mists-panel-parity-with-saved-vars-after-castspell/inspect-guild-control/screenshot.webp`; dump: `target/mists-panel-parity-with-saved-vars-after-castspell/inspect-guild-control/dump-tree.txt` | Covers inspect paperdoll and guild-control administration frames. |
| PvP UI: HonorFrame, BG queue, Conquest | Pass | screenshot: `target/mists-panel-parity-with-saved-vars-after-castspell/pvp/screenshot.webp`; dump: `target/mists-panel-parity-with-saved-vars-after-castspell/pvp/dump-tree.txt` | Honor/PvP backing API remains fixed. |
| LFG, LFR, Raid Browser | Pass | screenshot: `target/mists-panel-parity-with-saved-vars-after-castspell/lfg-lfr/screenshot.webp`; dump: `target/mists-panel-parity-with-saved-vars-after-castspell/lfg-lfr/dump-tree.txt` | Covers Mists-era pre-group-finder UI, not retail group finder. |
| Raid unit frames LoD panel | Pass | screenshot: `target/mists-panel-parity-with-saved-vars-after-castspell/raid-unit-frames/screenshot.webp`; dump: `target/mists-panel-parity-with-saved-vars-after-castspell/raid-unit-frames/dump-tree.txt` | Seeds an in-raid roster so legacy RaidParentFrame renders real RaidFrame group rows instead of a hidden-parent background-only capture. |
| Arena enemy unit frames LoD panel | Pass | screenshot: `target/mists-panel-parity-with-saved-vars-after-castspell/arena-unit-frames/screenshot.webp`; dump: `target/mists-panel-parity-with-saved-vars-after-castspell/arena-unit-frames/dump-tree.txt` | Seeds arena instance state so ArenaEnemyFrames passes the Blizzard visibility gate. |
| Battlefield map LoD panel | Pass | screenshot: `target/mists-panel-parity-with-saved-vars-after-castspell/battlefield-map/screenshot.webp`; dump: `target/mists-panel-parity-with-saved-vars-after-castspell/battlefield-map/dump-tree.txt` | Covers the legacy battlefield minimap LoD frame and its screenshot startup dispatch. |
| Collections: mounts, pets, toys, heirlooms, transmog | Pass | screenshot: `target/mists-panel-parity-with-saved-vars-after-castspell/collections/screenshot.webp`; dump: `target/mists-panel-parity-with-saved-vars-after-castspell/collections/dump-tree.txt` | MoP has no Wardrobe panel; no Wardrobe baseline expected. |
| Pet Journal and Battle Pet UI | Pass | screenshot: `target/mists-panel-parity-with-saved-vars-after-castspell/pet-journal/screenshot.webp`; dump: `target/mists-panel-parity-with-saved-vars-after-castspell/pet-journal/dump-tree.txt` | No known current gaps. |
| Achievements and Calendar | Pass | screenshot: `target/mists-panel-parity-with-saved-vars-after-castspell/achievements-calendar/screenshot.webp`; dump: `target/mists-panel-parity-with-saved-vars-after-castspell/achievements-calendar/dump-tree.txt` | No known current gaps. |
| Archaeology panel | Pass | screenshot: `target/mists-panel-parity-with-saved-vars-after-castspell/archaeology/screenshot.webp`; dump: `target/mists-panel-parity-with-saved-vars-after-castspell/archaeology/dump-tree.txt` | Covers the archaeology LoD skill journal. |
| Craft panel | Pass | screenshot: `target/mists-panel-parity-with-saved-vars-after-castspell/craft/screenshot.webp`; dump: `target/mists-panel-parity-with-saved-vars-after-castspell/craft/dump-tree.txt` | Covers the legacy CraftFrame LoD panel loaded by `CraftFrame_LoadUI()`. |
| TradeSkill panel | Pass | screenshot: `target/mists-panel-parity-with-saved-vars-after-castspell/trade-skill/screenshot.webp`; dump: `target/mists-panel-parity-with-saved-vars-after-castspell/trade-skill/dump-tree.txt` | Covers the legacy Mists TradeSkillFrame and its `GetTradeSkill*` backing data. |
| Encounter Journal | Pass | screenshot: `target/mists-panel-parity-with-saved-vars-after-castspell/encounter-journal/screenshot.webp`; dump: `target/mists-panel-parity-with-saved-vars-after-castspell/encounter-journal/dump-tree.txt` | No known current gaps. |
| Challenge mode LoD panel | Pass | screenshot: `target/mists-panel-parity-with-saved-vars-after-castspell/challenges/screenshot.webp`; dump: `target/mists-panel-parity-with-saved-vars-after-castspell/challenges/dump-tree.txt` | Covers the legacy Mists challenge-mode dungeon list. |
| Currency and Token UI | Pass | screenshot: `target/mists-panel-parity-with-saved-vars-after-castspell/currency-token/screenshot.webp`; dump: `target/mists-panel-parity-with-saved-vars-after-castspell/currency-token/dump-tree.txt` | Legacy `GetCurrencyListSize` wrapper remains verified. |
| Store, CatalogShop, WowToken, and SimpleCheckout | Pass | screenshot: `target/mists-panel-parity-with-saved-vars-after-castspell/store-commercial/screenshot.webp`; dump: `target/mists-panel-parity-with-saved-vars-after-castspell/store-commercial/dump-tree.txt` | Covers Mists commercial/store surfaces that correspond to retail AccountStore workflows. |
| Item socketing, reforging, and upgrade LoD panels | Pass | screenshot: `target/mists-panel-parity-with-saved-vars-after-castspell/item-services/screenshot.webp`; dump: `target/mists-panel-parity-with-saved-vars-after-castspell/item-services/dump-tree.txt` | Covers MoP item-service LoD panels loaded by NPC service events. |
| NPC service LoD panels: barber and black market | Pass | screenshot: `target/mists-panel-parity-with-saved-vars-after-castspell/npc-services/screenshot.webp`; dump: `target/mists-panel-parity-with-saved-vars-after-castspell/npc-services/dump-tree.txt` | Covers BarberShopFrame and BlackMarketFrame. |
| Class trainer LoD panel | Pass | screenshot: `target/mists-panel-parity-with-saved-vars-after-castspell/class-trainer/screenshot.webp`; dump: `target/mists-panel-parity-with-saved-vars-after-castspell/class-trainer/dump-tree.txt` | Covers Blizzard_TrainerUI legacy trainer-service globals and ClassTrainerFrame template wiring. |
| Quest choice LoD dialog | Pass | screenshot: `target/mists-panel-parity-with-saved-vars-after-castspell/quest-choice/screenshot.webp`; dump: `target/mists-panel-parity-with-saved-vars-after-castspell/quest-choice/dump-tree.txt` | Covers the reward-choice dialog loaded from `Blizzard_QuestChoice`. |
| Macro and key bindings | Pass | screenshot: `target/mists-panel-parity-with-saved-vars-after-castspell/macro-keybindings/screenshot.webp`; dump: `target/mists-panel-parity-with-saved-vars-after-castspell/macro-keybindings/dump-tree.txt` | No known current gaps. |
| Interface options | Pass | screenshot: `target/mists-panel-parity-with-saved-vars-after-castspell/interface-options/screenshot.webp`; dump: `target/mists-panel-parity-with-saved-vars-after-castspell/interface-options/dump-tree.txt` | Mists is pre-EditMode and uses `InterfaceOptionsFrame`. |
| Action bars, micro menu, bag bar, status bars | Pass | screenshot: `target/mists-panel-parity-with-saved-vars-after-castspell/action-bars/screenshot.webp`; dump: `target/mists-panel-parity-with-saved-vars-after-castspell/action-bars/dump-tree.txt` | Micro menu icon regressions are covered. |
| Time manager and move pad LoD utilities | Pass | screenshot: `target/mists-panel-parity-with-saved-vars-after-castspell/time-move-utilities/screenshot.webp`; dump: `target/mists-panel-parity-with-saved-vars-after-castspell/time-move-utilities/dump-tree.txt` | Covers TimeManagerFrame and MovePadFrame utility LoD surfaces. |
| Nameplates | Pass | screenshot: `target/mists-panel-parity-with-saved-vars-after-castspell/nameplates/screenshot.webp`; dump: `target/mists-panel-parity-with-saved-vars-after-castspell/nameplates/dump-tree.txt` | CVar defaults and rendering path are covered. |
| Loot, group loot, personal loot | Pass | screenshot: `target/mists-panel-parity-with-saved-vars-after-castspell/loot/screenshot.webp`; dump: `target/mists-panel-parity-with-saved-vars-after-castspell/loot/dump-tree.txt` | No known current gaps. |
| Game menu options | Pass | screenshot: `target/mists-panel-parity-with-saved-vars-after-castspell/game-menu-options/screenshot.webp`; dump: `target/mists-panel-parity-with-saved-vars-after-castspell/game-menu-options/dump-tree.txt` | Options panel breakage fixed in the current audit. |

## Verification

- `scripts/test-classic-addons.sh --profile mists`: `passed: 9`, `failed: 0`.
- Each `target/addon-harness/*-lua-errors.json` array length was `0`.
- Rechecked after the glyph/currency texture-directory fix:
  `scripts/mists-panel-parity.sh --with-saved-vars --out-dir target/mists-panel-parity-with-saved-vars-after-castspell --panel talents-glyphs --skip-build`
  and the same command for `--panel currency-token` both passed.
- `tests/mists_panel_artifact_logs.rs` verifies retained panel stderr artifacts
  do not decode `BlizzardInterfaceArt/` as an image directory.
- `PLAN.mists.md` local panel parity hardening tasks are checked through the
  texture-directory retained-artifact verification.
