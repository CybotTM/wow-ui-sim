# Mists Panel Interaction Coverage

Captured 2026-05-12 after comparing the Mists panel parity rows against the
retail-style panel workflows that make sense for Pandaria Classic. Retail-only
systems such as EditMode trees, Dragonflight group finder, and Wardrobe are not
expected under `client-mists`; the Mists row lists the equivalent legacy flow.

`Status` meanings:

- `Covered`: the referenced Mists test exercises state-changing interaction or
  populated backing data beyond simply opening a root frame.
- `Mists-specific`: the row intentionally uses a Pandaria-era workflow instead
  of the retail workflow.
- `Missing`: a retail-supported workflow still needs a Mists parity test.

| Panel | Retail-supported workflow audited | Mists coverage | Status |
|---|---|---|---|
| Character panel: paperdoll, stats, titles, equipment manager | Gear slots, reputation rows, titles, equipment set selection | `tests/mists_character_panel.rs::mists_character_panel_populates_gear_and_reputation`; `tests/mists_character_panel.rs::mists_character_subpanels_drive_titles_and_equipment_sets` | Covered |
| Spellbook and professions | Spell tab population and professions tab buttons | `tests/mists_spellbook_panel.rs::mists_spellbook_populates_visible_spell_buttons` | Covered |
| Talents and glyphs | Talent selection mutation; glyph socket mutation | `tests/mists_talents_glyphs_panel.rs::mists_talents_and_glyphs_panel_populates_rows_and_sockets`; `tests/mists_talents_glyphs_panel.rs::mists_talents_and_glyphs_mutate_selected_state` | Mists-specific |
| Quest log and objective tracker | Quest selection, watch state, objective tracker refresh | `tests/mists_quest_panel.rs::mists_quest_log_and_objective_tracker_open_cleanly` | Covered |
| World map | Zone art, zoom/navigation, quest pin selection, opacity path | `tests/mists_world_map_panel.rs::mists_world_map_opens_with_zone_art_and_quest_pins` | Covered |
| Mail: inbox, send, attachments, COD | Inbox open, money take, attachment send, COD send | `tests/mists_mail_panel.rs::mists_mail_panel_supports_inbox_send_attachments_and_cod` | Covered |
| Auction House: browse, bid, post, cancel | Browse query, bid placement, auction posting, cancel | `tests/mists_auction_house_panel.rs::mists_auction_house_supports_browse_bid_post_and_cancel` | Covered |
| Bank, ReagentBank, Void Storage, Guild Bank | Bank bags, reagent bank, void storage, guild bank data | `tests/mists_bank_storage_panel.rs::mists_bank_and_guild_bank_support_storage_flow` | Covered |
| Trade window and `TradePlayerInputMoneyFrame` | Trade open, money input, accept/cancel paths | `tests/mists_trade_panel.rs::mists_trade_window_supports_money_input_flow` | Covered |
| Friends, Who, Guild, Communities, Club Finder | Friends, who query, guild roster, club finder row/mode changes | `tests/mists_social_panels.rs::mists_social_panels_support_friends_who_guild_and_communities` | Covered |
| PvP UI: HonorFrame, BG queue, Conquest | Honor, battleground queue, conquest/rated panel state | `tests/mists_pvp_ui_panel.rs::mists_pvp_ui_supports_honor_battleground_and_conquest_panels` | Covered |
| LFG, LFR, Raid Browser | Dungeon finder, raid finder, raid browser selections | `tests/mists_lfg_lfr_panel.rs::mists_lfg_lfr_and_raid_finder_panels_render_with_seeded_choices` | Mists-specific |
| Collections: mounts, pets, toys, heirlooms, transmog | Mount, toy, heirloom tab data and item actions | `tests/mists_collections_panel.rs::mists_collections_tabs_render_without_wardrobe`; `tests/mists_collections_panel.rs::mists_collections_rows_drive_mount_toy_and_heirloom_actions` | Mists-specific |
| Pet Journal and Battle Pet UI | Pet selection and battle action buttons | `tests/mists_pet_battle_ui.rs::mists_pet_journal_and_battle_pet_ui_render_and_interact` | Covered |
| Achievements and Calendar | Achievement tab switching and calendar month navigation | `tests/mists_achievements_calendar_panel.rs::mists_achievements_and_calendar_open_without_lua_errors` | Covered |
| Encounter Journal | Tier selection, instance display, loot/overview tabs | `tests/mists_encounter_journal_panel.rs::mists_encounter_journal_opens_and_displays_instances` | Covered |
| Currency and Token UI | Legacy currency list wrappers and watched token rendering | `tests/mists_currency_token_ui_panel.rs::mists_currency_and_token_ui_render_without_lua_errors` | Covered |
| Macro and key bindings | Macro selection/run and keybinding mutation round trips | `tests/mists_macro_keybindings_panel.rs::mists_macro_and_keybindings_panels_render_without_lua_errors` | Covered |
| Interface options | Game-menu options flow, interface category focus, no retail EditMode | `tests/mists_interface_options_panel.rs::mists_game_menu_options_opens_interface_settings_without_lua_errors` | Mists-specific |
| Action bars, micro menu, bag bar, status bars | Action button updates, micro menu textures, bag/status bar state | `tests/mists_action_bars_panel.rs::mists_action_bag_micro_and_status_bars_interact_without_lua_errors`; `tests/mists_action_micro_bag_status.rs::mists_action_micro_bag_and_status_bars_are_interactive` | Covered |
| Nameplates | Nameplate driver acquisition and renderable unit frame state | `tests/mists_nameplates_panel.rs::mists_nameplate_driver_acquires_renderable_unit_frame` | Covered |
| Loot, group loot, personal loot | Loot slot take, group roll choice, bonus roll state | `tests/mists_loot_panel.rs::mists_loot_group_and_bonus_roll_ui_render_without_lua_errors` | Covered |
| Game menu options | Options, help, addons, close/reopen behavior | `tests/mists_interface_options_panel.rs::mists_game_menu_options_opens_interface_settings_without_lua_errors` | Covered |

No currently audited retail-supported panel workflow is marked `Missing`.
