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
| Character panel: paperdoll, stats, titles, equipment manager | Pass | screenshot: `target/mists-panel-parity/character/screenshot.webp`; dump: `target/mists-panel-parity/character/dump-tree.txt` | No known current gaps. |
| Spellbook and professions | Pass | screenshot: `target/mists-panel-parity/spellbook-professions/screenshot.webp`; dump: `target/mists-panel-parity/spellbook-professions/dump-tree.txt` | No known current gaps. |
| Talents and glyphs | Pass | screenshot: `target/mists-panel-parity/talents-glyphs/screenshot.webp`; dump: `target/mists-panel-parity/talents-glyphs/dump-tree.txt` | Mists talent rows and glyph slots are intentionally distinct from retail trees. |
| Quest log and objective tracker | Pass | screenshot: `target/mists-panel-parity/quest-log/screenshot.webp`; dump: `target/mists-panel-parity/quest-log/dump-tree.txt` | No known current gaps. |
| World map | Pass | screenshot: `target/mists-panel-parity/world-map/screenshot.webp`; dump: `target/mists-panel-parity/world-map/dump-tree.txt` | Zone overlays, quest pins, and opacity path are covered. |
| Mail: inbox, send, attachments, COD | Pass | screenshot: `target/mists-panel-parity/mail/screenshot.webp`; dump: `target/mists-panel-parity/mail/dump-tree.txt` | No known current gaps. |
| Auction House: browse, bid, post, cancel | Pass | screenshot: `target/mists-panel-parity/auction-house/screenshot.webp`; dump: `target/mists-panel-parity/auction-house/dump-tree.txt` | No known current gaps. |
| AddOn list and UI management LoD panels | Pass | screenshot: `target/mists-panel-parity/addon-list/screenshot.webp`; dump: `target/mists-panel-parity/addon-list/dump-tree.txt` | Covers the LoD AddOnList management frame outside normal game-panel openers. |
| Bank, ReagentBank, Void Storage, Guild Bank | Pass | screenshot: `target/mists-panel-parity/bank-storage/screenshot.webp`; dump: `target/mists-panel-parity/bank-storage/dump-tree.txt` | No known current gaps. |
| Trade window and `TradePlayerInputMoneyFrame` | Pass | screenshot: `target/mists-panel-parity/trade/screenshot.webp`; dump: `target/mists-panel-parity/trade/dump-tree.txt` | Money input child-key root cause remains fixed. |
| Friends, Who, Guild, Communities, Club Finder | Pass | screenshot: `target/mists-panel-parity/social/screenshot.webp`; dump: `target/mists-panel-parity/social/dump-tree.txt` | No known current gaps. |
| Inspect and guild control LoD panels | Pass | screenshot: `target/mists-panel-parity/inspect-guild-control/screenshot.webp`; dump: `target/mists-panel-parity/inspect-guild-control/dump-tree.txt` | Covers inspect paperdoll and guild-control administration frames. |
| PvP UI: HonorFrame, BG queue, Conquest | Pass | screenshot: `target/mists-panel-parity/pvp/screenshot.webp`; dump: `target/mists-panel-parity/pvp/dump-tree.txt` | Honor/PvP backing API remains fixed. |
| LFG, LFR, Raid Browser | Pass | screenshot: `target/mists-panel-parity/lfg-lfr/screenshot.webp`; dump: `target/mists-panel-parity/lfg-lfr/dump-tree.txt` | Covers Mists-era pre-group-finder UI, not retail group finder. |
| Collections: mounts, pets, toys, heirlooms, transmog | Pass | screenshot: `target/mists-panel-parity/collections/screenshot.webp`; dump: `target/mists-panel-parity/collections/dump-tree.txt` | MoP has no Wardrobe panel; no Wardrobe baseline expected. |
| Pet Journal and Battle Pet UI | Pass | screenshot: `target/mists-panel-parity/pet-journal/screenshot.webp`; dump: `target/mists-panel-parity/pet-journal/dump-tree.txt` | No known current gaps. |
| Achievements and Calendar | Pass | screenshot: `target/mists-panel-parity/achievements-calendar/screenshot.webp`; dump: `target/mists-panel-parity/achievements-calendar/dump-tree.txt` | No known current gaps. |
| Archaeology panel | Pass | screenshot: `target/mists-panel-parity/archaeology/screenshot.webp`; dump: `target/mists-panel-parity/archaeology/dump-tree.txt` | Covers the archaeology LoD skill journal. |
| Encounter Journal | Pass | screenshot: `target/mists-panel-parity/encounter-journal/screenshot.webp`; dump: `target/mists-panel-parity/encounter-journal/dump-tree.txt` | No known current gaps. |
| Challenge mode LoD panel | Pass | screenshot: `target/mists-panel-parity/challenges/screenshot.webp`; dump: `target/mists-panel-parity/challenges/dump-tree.txt` | Covers the legacy Mists challenge-mode dungeon list. BattlefieldMapFrame is tracked as a separate visual gap. |
| Currency and Token UI | Pass | screenshot: `target/mists-panel-parity/currency-token/screenshot.webp`; dump: `target/mists-panel-parity/currency-token/dump-tree.txt` | Legacy `GetCurrencyListSize` wrapper remains verified. |
| Item socketing, reforging, and upgrade LoD panels | Pass | screenshot: `target/mists-panel-parity/item-services/screenshot.webp`; dump: `target/mists-panel-parity/item-services/dump-tree.txt` | Covers MoP item-service LoD panels loaded by NPC service events. |
| NPC service LoD panels: barber and black market | Pass | screenshot: `target/mists-panel-parity/npc-services/screenshot.webp`; dump: `target/mists-panel-parity/npc-services/dump-tree.txt` | Covers BarberShopFrame and BlackMarketFrame. ClassTrainerFrame is tracked as a separate legacy trainer-service gap. |
| Quest choice LoD dialog | Pass | screenshot: `target/mists-panel-parity/quest-choice/screenshot.webp`; dump: `target/mists-panel-parity/quest-choice/dump-tree.txt` | Covers the reward-choice dialog loaded from `Blizzard_QuestChoice`. |
| Macro and key bindings | Pass | screenshot: `target/mists-panel-parity/macro-keybindings/screenshot.webp`; dump: `target/mists-panel-parity/macro-keybindings/dump-tree.txt` | No known current gaps. |
| Interface options | Pass | screenshot: `target/mists-panel-parity/interface-options/screenshot.webp`; dump: `target/mists-panel-parity/interface-options/dump-tree.txt` | Mists is pre-EditMode and uses `InterfaceOptionsFrame`. |
| Action bars, micro menu, bag bar, status bars | Pass | screenshot: `target/mists-panel-parity/action-bars/screenshot.webp`; dump: `target/mists-panel-parity/action-bars/dump-tree.txt` | Micro menu icon regressions are covered. |
| Time manager and move pad LoD utilities | Pass | screenshot: `target/mists-panel-parity/time-move-utilities/screenshot.webp`; dump: `target/mists-panel-parity/time-move-utilities/dump-tree.txt` | Covers TimeManagerFrame and MovePadFrame utility LoD surfaces. |
| Nameplates | Pass | screenshot: `target/mists-panel-parity/nameplates/screenshot.webp`; dump: `target/mists-panel-parity/nameplates/dump-tree.txt` | CVar defaults and rendering path are covered. |
| Loot, group loot, personal loot | Pass | screenshot: `target/mists-panel-parity/loot/screenshot.webp`; dump: `target/mists-panel-parity/loot/dump-tree.txt` | No known current gaps. |
| Game menu options | Pass | screenshot: `target/mists-panel-parity/game-menu-options/screenshot.webp`; dump: `target/mists-panel-parity/game-menu-options/dump-tree.txt` | Options panel breakage fixed in the current audit. |

## Verification

- `scripts/test-classic-addons.sh --profile mists`: `passed: 9`, `failed: 0`.
- Each `target/addon-harness/*-lua-errors.json` array length was `0`.
- `PLAN.mists.md` panel parity audit is checked through `Game menu options breaks`.
