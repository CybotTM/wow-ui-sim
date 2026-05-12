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

Screenshots are not committed as binary baselines here; the screenshot column
records the retained visual evidence path. Rows marked `test-backed` are covered
by frame-tree, render, icon, or interaction assertions in the listed focused test.

| Panel | Status | Screenshot | Gap notes |
|---|---|---|---|
| Character panel: paperdoll, stats, titles, equipment manager | Pass | test-backed: `tests/mists_character_panel.rs`, `tests/mists_character_reputation_panels.rs` | No known current gaps. |
| Spellbook and professions | Pass | test-backed: `tests/mists_spellbook_panel.rs`, `tests/spellbook_layout.rs` | No known current gaps. |
| Talents and glyphs | Pass | test-backed: `tests/mists_talents_glyphs_panel.rs` | Mists talent rows and glyph slots are intentionally distinct from retail trees. |
| Quest log and objective tracker | Pass | test-backed: `tests/mists_quest_panel.rs` | No known current gaps. |
| World map | Pass | test-backed: `tests/mists_world_map_panel.rs`, `tests/mists_world_map_opacity.rs` | Zone overlays, quest pins, and opacity path are covered. |
| Mail: inbox, send, attachments, COD | Pass | test-backed: `tests/mists_mail_panel.rs`, `tests/test_mail.rs` | No known current gaps. |
| Auction House: browse, bid, post, cancel | Pass | test-backed: `tests/mists_auction_house_panel.rs`, `tests/test_showuipanel_auction_house.rs` | No known current gaps. |
| Bank, ReagentBank, Void Storage, Guild Bank | Pass | test-backed: `tests/mists_bank_storage_panel.rs` | No known current gaps. |
| Trade window and `TradePlayerInputMoneyFrame` | Pass | test-backed: `tests/mists_trade_panel.rs`, `tests/trade_info.rs` | Money input child-key root cause remains fixed. |
| Friends, Who, Guild, Communities, Club Finder | Pass | test-backed: `tests/mists_social_panels.rs`, `tests/friends_panel.rs`, `tests/test_guild_panel.rs` | No known current gaps. |
| PvP UI: HonorFrame, BG queue, Conquest | Pass | test-backed: `tests/mists_pvp_ui_panel.rs`, `tests/pvp_probes.rs` | Honor/PvP backing API remains fixed. |
| LFG, LFR, Raid Browser | Pass | test-backed: `tests/mists_lfg_lfr_panel.rs`, `tests/battlefield_lfg_probes.rs` | Covers Mists-era pre-group-finder UI, not retail group finder. |
| Collections: mounts, pets, toys, heirlooms, transmog | Pass | test-backed: `tests/mists_collections_panel.rs` | MoP has no Wardrobe panel; no Wardrobe baseline expected. |
| Pet Journal and Battle Pet UI | Pass | test-backed: `tests/mists_pet_battle_ui.rs`, `tests/blizzard_pet_battle_ui_loads.rs` | No known current gaps. |
| Achievements and Calendar | Pass | test-backed: `tests/mists_achievements_calendar_panel.rs` | No known current gaps. |
| Encounter Journal | Pass | test-backed: `tests/mists_encounter_journal_panel.rs`, `tests/encounter_journal_regressions.rs` | No known current gaps. |
| Currency and Token UI | Pass | test-backed: `tests/mists_currency_token_ui_panel.rs`, `tests/mists_currency_list.rs` | Legacy `GetCurrencyListSize` wrapper remains verified. |
| Macro and key bindings | Pass | test-backed: `tests/mists_macro_keybindings_panel.rs`, `tests/keybindings.rs` | No known current gaps. |
| Interface options | Pass | test-backed: `tests/mists_interface_options_panel.rs`, `tests/game_menu.rs` | Mists is pre-EditMode and uses `InterfaceOptionsFrame`. |
| Action bars, micro menu, bag bar, status bars | Pass | test-backed: `tests/mists_action_bars_panel.rs`, `tests/mists_action_micro_bag_status.rs`, `tests/mists_micro_menu_icons.rs` | Micro menu icon regressions are covered. |
| Nameplates | Pass | test-backed: `tests/mists_nameplates_panel.rs`, `tests/mists_nameplate_scale.rs` | CVar defaults and rendering path are covered. |
| Loot, group loot, personal loot | Pass | test-backed: `tests/mists_loot_panel.rs`, `tests/party_info_loot.rs` | No known current gaps. |
| Game menu options | Pass | test-backed: `tests/mists_interface_options_panel.rs`, `tests/game_menu.rs` | Options panel breakage fixed in the current audit. |

## Verification

- `scripts/test-classic-addons.sh --profile mists`: `passed: 9`, `failed: 0`.
- Each `target/addon-harness/*-lua-errors.json` array length was `0`.
- `PLAN.mists.md` panel parity audit is checked through `Game menu options breaks`.
