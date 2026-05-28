# Mists Panel Parity Baseline

Captured 2026-05-12 on the Mists rollout branch after the panel parity audit
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
| Character panel: paperdoll, stats, titles, equipment manager | Pass | screenshot: `target/mists-final-local-audit/panel-parity-with-saved-vars/character/screenshot.webp`; dump: `target/mists-final-local-audit/panel-parity-with-saved-vars/character/dump-tree.txt` | Rechecked after the startup `PET_UI_UPDATE` fix; default no-pet open path hides the pet tab and seeded pet UI shows it. |
| Spellbook and professions | Pass | screenshot: `target/mists-final-local-audit/panel-parity-with-saved-vars/spellbook-professions/screenshot.webp`; dump: `target/mists-final-local-audit/panel-parity-with-saved-vars/spellbook-professions/dump-tree.txt` | No known current gaps. |
| Talents and glyphs | Pass | screenshot: `target/mists-final-local-audit/panel-parity-with-saved-vars/talents-glyphs/screenshot.webp`; dump: `target/mists-final-local-audit/panel-parity-with-saved-vars/talents-glyphs/dump-tree.txt` | Mists talent rows and glyph slots are intentionally distinct from retail trees. |
| Quest log and objective tracker | Pass | screenshot: `target/mists-final-local-audit/panel-parity-with-saved-vars/quest-log/screenshot.webp`; dump: `target/mists-final-local-audit/panel-parity-with-saved-vars/quest-log/dump-tree.txt` | No known current gaps. |
| World map | Pass | screenshot: `target/mists-final-local-audit/panel-parity-with-saved-vars/world-map/screenshot.webp`; dump: `target/mists-final-local-audit/panel-parity-with-saved-vars/world-map/dump-tree.txt` | Zone overlays, quest pins, and opacity path are covered. |
| Mail: inbox, send, attachments, COD | Pass | screenshot: `target/mists-final-local-audit/panel-parity-with-saved-vars/mail/screenshot.webp`; dump: `target/mists-final-local-audit/panel-parity-with-saved-vars/mail/dump-tree.txt` | No known current gaps. |
| Auction House: browse, bid, post, cancel | Pass | screenshot: `target/mists-final-local-audit/panel-parity-with-saved-vars/auction-house/screenshot.webp`; dump: `target/mists-final-local-audit/panel-parity-with-saved-vars/auction-house/dump-tree.txt` | No known current gaps. |
| AddOn list and UI management LoD panels | Pass | screenshot: `target/mists-final-local-audit/panel-parity-with-saved-vars/addon-list/screenshot.webp`; dump: `target/mists-final-local-audit/panel-parity-with-saved-vars/addon-list/dump-tree.txt` | Covers the LoD AddOnList management frame outside normal game-panel openers. |
| Bank, ReagentBank, Void Storage, Guild Bank | Pass | screenshot: `target/mists-final-local-audit/panel-parity-with-saved-vars/bank-storage/screenshot.webp`; dump: `target/mists-final-local-audit/panel-parity-with-saved-vars/bank-storage/dump-tree.txt` | No known current gaps. |
| Trade window and `TradePlayerInputMoneyFrame` | Pass | screenshot: `target/mists-final-local-audit/panel-parity-with-saved-vars/trade/screenshot.webp`; dump: `target/mists-final-local-audit/panel-parity-with-saved-vars/trade/dump-tree.txt` | Money input child-key root cause remains fixed. |
| Friends, Who, Guild, Communities, Club Finder | Pass | screenshot: `target/mists-final-local-audit/panel-parity-with-saved-vars/social/screenshot.webp`; dump: `target/mists-final-local-audit/panel-parity-with-saved-vars/social/dump-tree.txt` | No known current gaps. |
| Inspect and guild control LoD panels | Pass | screenshot: `target/mists-final-local-audit/panel-parity-with-saved-vars/inspect-guild-control/screenshot.webp`; dump: `target/mists-final-local-audit/panel-parity-with-saved-vars/inspect-guild-control/dump-tree.txt` | Covers inspect paperdoll and guild-control administration frames. |
| PvP UI: HonorFrame, BG queue, Conquest | Pass | screenshot: `target/mists-final-local-audit/panel-parity-with-saved-vars/pvp/screenshot.webp`; dump: `target/mists-final-local-audit/panel-parity-with-saved-vars/pvp/dump-tree.txt` | Honor/PvP backing API remains fixed. |
| LFG, LFR, Raid Browser | Pass | screenshot: `target/mists-final-local-audit/panel-parity-with-saved-vars/lfg-lfr/screenshot.webp`; dump: `target/mists-final-local-audit/panel-parity-with-saved-vars/lfg-lfr/dump-tree.txt` | Covers Mists-era pre-group-finder UI, not retail group finder. |
| Raid unit frames LoD panel | Pass | screenshot: `target/mists-final-local-audit/panel-parity-with-saved-vars/raid-unit-frames/screenshot.webp`; dump: `target/mists-final-local-audit/panel-parity-with-saved-vars/raid-unit-frames/dump-tree.txt` | Seeds an in-raid roster so legacy RaidParentFrame renders real RaidFrame group rows instead of a hidden-parent background-only capture. |
| Arena enemy unit frames LoD panel | Pass | screenshot: `target/mists-final-local-audit/panel-parity-with-saved-vars/arena-unit-frames/screenshot.webp`; dump: `target/mists-final-local-audit/panel-parity-with-saved-vars/arena-unit-frames/dump-tree.txt` | Seeds arena instance state so ArenaEnemyFrames passes the Blizzard visibility gate. |
| Battlefield map LoD panel | Pass | screenshot: `target/mists-final-local-audit/panel-parity-with-saved-vars/battlefield-map/screenshot.webp`; dump: `target/mists-final-local-audit/panel-parity-with-saved-vars/battlefield-map/dump-tree.txt` | Covers the legacy battlefield minimap LoD frame and its screenshot startup dispatch. |
| Collections: mounts, pets, toys, heirlooms, transmog | Pass | screenshot: `target/mists-final-local-audit/panel-parity-with-saved-vars/collections/screenshot.webp`; dump: `target/mists-final-local-audit/panel-parity-with-saved-vars/collections/dump-tree.txt` | MoP has no Wardrobe panel; no Wardrobe baseline expected. |
| Pet Journal and Battle Pet UI | Pass | screenshot: `target/mists-final-local-audit/panel-parity-with-saved-vars/pet-journal/screenshot.webp`; dump: `target/mists-final-local-audit/panel-parity-with-saved-vars/pet-journal/dump-tree.txt` | No known current gaps. |
| Achievements and Calendar | Pass | screenshot: `target/mists-final-local-audit/panel-parity-with-saved-vars/achievements-calendar/screenshot.webp`; dump: `target/mists-final-local-audit/panel-parity-with-saved-vars/achievements-calendar/dump-tree.txt` | No known current gaps. |
| Archaeology panel | Pass | screenshot: `target/mists-final-local-audit/panel-parity-with-saved-vars/archaeology/screenshot.webp`; dump: `target/mists-final-local-audit/panel-parity-with-saved-vars/archaeology/dump-tree.txt` | Covers the archaeology LoD skill journal. |
| Craft panel | Pass | screenshot: `target/mists-final-local-audit/panel-parity-with-saved-vars/craft/screenshot.webp`; dump: `target/mists-final-local-audit/panel-parity-with-saved-vars/craft/dump-tree.txt` | Covers the legacy CraftFrame LoD panel loaded by `CraftFrame_LoadUI()`. |
| TradeSkill panel | Pass | screenshot: `target/mists-final-local-audit/panel-parity-with-saved-vars/trade-skill/screenshot.webp`; dump: `target/mists-final-local-audit/panel-parity-with-saved-vars/trade-skill/dump-tree.txt` | Covers the legacy Mists TradeSkillFrame and its `GetTradeSkill*` backing data. |
| Encounter Journal | Pass | screenshot: `target/mists-final-local-audit/panel-parity-with-saved-vars/encounter-journal/screenshot.webp`; dump: `target/mists-final-local-audit/panel-parity-with-saved-vars/encounter-journal/dump-tree.txt` | No known current gaps. |
| Challenge mode LoD panel | Pass | screenshot: `target/mists-final-local-audit/panel-parity-with-saved-vars/challenges/screenshot.webp`; dump: `target/mists-final-local-audit/panel-parity-with-saved-vars/challenges/dump-tree.txt` | Covers the legacy Mists challenge-mode dungeon list. |
| Currency and Token UI | Pass | screenshot: `target/mists-final-local-audit/panel-parity-with-saved-vars/currency-token/screenshot.webp`; dump: `target/mists-final-local-audit/panel-parity-with-saved-vars/currency-token/dump-tree.txt` | Legacy `GetCurrencyListSize` wrapper remains verified. |
| Store, CatalogShop, WowToken, and SimpleCheckout | Pass | screenshot: `target/mists-final-local-audit/panel-parity-with-saved-vars/store-commercial/screenshot.webp`; dump: `target/mists-final-local-audit/panel-parity-with-saved-vars/store-commercial/dump-tree.txt` | Covers Mists commercial/store surfaces that correspond to retail AccountStore workflows. |
| Item socketing, reforging, and upgrade LoD panels | Pass | screenshot: `target/mists-final-local-audit/panel-parity-with-saved-vars/item-services/screenshot.webp`; dump: `target/mists-final-local-audit/panel-parity-with-saved-vars/item-services/dump-tree.txt` | Covers MoP item-service LoD panels loaded by NPC service events. |
| NPC service LoD panels: barber and black market | Pass | screenshot: `target/mists-final-local-audit/panel-parity-with-saved-vars/npc-services/screenshot.webp`; dump: `target/mists-final-local-audit/panel-parity-with-saved-vars/npc-services/dump-tree.txt` | Covers BarberShopFrame and BlackMarketFrame. |
| Class trainer LoD panel | Pass | screenshot: `target/mists-final-local-audit/panel-parity-with-saved-vars/class-trainer/screenshot.webp`; dump: `target/mists-final-local-audit/panel-parity-with-saved-vars/class-trainer/dump-tree.txt` | Covers Blizzard_TrainerUI legacy trainer-service globals and ClassTrainerFrame template wiring. |
| Quest choice LoD dialog | Pass | screenshot: `target/mists-final-local-audit/panel-parity-with-saved-vars/quest-choice/screenshot.webp`; dump: `target/mists-final-local-audit/panel-parity-with-saved-vars/quest-choice/dump-tree.txt` | Covers the reward-choice dialog loaded from `Blizzard_QuestChoice`. |
| Macro and key bindings | Pass | screenshot: `target/mists-final-local-audit/panel-parity-with-saved-vars/macro-keybindings/screenshot.webp`; dump: `target/mists-final-local-audit/panel-parity-with-saved-vars/macro-keybindings/dump-tree.txt` | No known current gaps. |
| Interface options | Pass | screenshot: `target/mists-final-local-audit/panel-parity-with-saved-vars/interface-options/screenshot.webp`; dump: `target/mists-final-local-audit/panel-parity-with-saved-vars/interface-options/dump-tree.txt` | Mists is pre-EditMode and uses `InterfaceOptionsFrame`. |
| Action bars, micro menu, bag bar, status bars | Pass | screenshot: `target/mists-final-local-audit/panel-parity-with-saved-vars/action-bars/screenshot.webp`; dump: `target/mists-final-local-audit/panel-parity-with-saved-vars/action-bars/dump-tree.txt` | Micro menu icon regressions are covered. |
| Time manager and move pad LoD utilities | Pass | screenshot: `target/mists-final-local-audit/panel-parity-with-saved-vars/time-move-utilities/screenshot.webp`; dump: `target/mists-final-local-audit/panel-parity-with-saved-vars/time-move-utilities/dump-tree.txt` | Covers TimeManagerFrame and MovePadFrame utility LoD surfaces. |
| Nameplates | Pass | screenshot: `target/mists-final-local-audit/panel-parity-with-saved-vars/nameplates/screenshot.webp`; dump: `target/mists-final-local-audit/panel-parity-with-saved-vars/nameplates/dump-tree.txt` | CVar defaults and rendering path are covered. |
| Loot, group loot, personal loot | Pass | screenshot: `target/mists-final-local-audit/panel-parity-with-saved-vars/loot/screenshot.webp`; dump: `target/mists-final-local-audit/panel-parity-with-saved-vars/loot/dump-tree.txt` | No known current gaps. |
| Game menu options | Pass | screenshot: `target/mists-final-local-audit/panel-parity-with-saved-vars/game-menu-options/screenshot.webp`; dump: `target/mists-final-local-audit/panel-parity-with-saved-vars/game-menu-options/dump-tree.txt` | Options panel breakage fixed in the current audit. |

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
