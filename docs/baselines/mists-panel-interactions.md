# Mists Panel Interaction Coverage

Captured 2026-05-12 after comparing the Mists panel parity rows against the
retail-style panel workflows that make sense for Pandaria Classic. Retail-only
systems such as EditMode trees, Dragonflight group finder, and Wardrobe are not
expected under `client-mists`; the Mists row lists the equivalent legacy flow.
Reconciled 2026-05-14 after adding live connected-GUI smoke coverage for idle
HUD state and the Mists specialization Learn-to-talents flow.

`Status` meanings:

- `Covered`: the referenced Mists test exercises state-changing interaction or
  populated backing data beyond simply opening a root frame.
- `Mists-specific`: the row intentionally uses a Pandaria-era workflow instead
  of the retail workflow.
- `Follow-up`: the panel loads or renders, but the cited coverage is still
  mostly load/show/static frame evidence and needs a state-changing interaction
  assertion.
- `Missing`: a retail-supported workflow still needs a Mists parity test.

| Panel | Retail-supported workflow audited | Mists coverage | Status |
|---|---|---|---|
| Character panel: paperdoll, stats, titles, equipment manager | Gear slots, reputation rows, titles, equipment set selection | `tests/mists_character_panel.rs::mists_character_panel_populates_gear_and_reputation`; `tests/mists_character_panel.rs::mists_character_subpanels_drive_titles_and_equipment_sets` | Covered |
| Spellbook and professions | Spell tab population and professions tab buttons | `tests/mists_spellbook_panel.rs::mists_spellbook_populates_visible_spell_buttons` | Covered |
| Talents and glyphs | Talent selection mutation; glyph socket mutation | `tests/mists_talents_glyphs_panel.rs::mists_talents_and_glyphs_panel_populates_rows_and_sockets`; `tests/mists_talents_glyphs_panel.rs::mists_talents_and_glyphs_mutate_selected_state` | Mists-specific |
| Quest log and objective tracker | Quest selection, watch state, objective tracker refresh | `tests/mists_quest_panel.rs::mists_quest_log_selects_entries_and_refreshes_tracker` | Covered |
| World map | Zone art, zoom/navigation, quest pin selection, opacity path | `tests/mists_world_map_panel.rs::mists_world_map_opens_with_zone_art_and_quest_pins` | Covered |
| Mail: inbox, send, attachments, COD | Inbox open, money take, attachment send, COD send | `tests/mists_mail_panel.rs::mists_mail_panel_supports_inbox_send_attachments_and_cod` | Covered |
| Auction House: browse, bid, post, cancel | Browse query, bid placement, auction posting, cancel | `tests/mists_auction_house_panel.rs::mists_auction_house_supports_browse_bid_post_and_cancel` | Covered |
| AddOn list and UI management LoD panels | AddOnList load, row toggle, reload-state, and enable-all visible state changes | `tests/mists_addonlist_interactions.rs::mists_addonlist_row_toggle_updates_visible_reload_state` | Mists-specific |
| Bank, ReagentBank, Void Storage, Guild Bank | Bank bags, reagent bank, void storage, guild bank data | `tests/mists_bank_storage_panel.rs::mists_bank_and_guild_bank_support_storage_flow` | Covered |
| Trade window and `TradePlayerInputMoneyFrame` | Trade open, money input, accept/cancel paths | `tests/mists_trade_panel.rs::mists_trade_window_supports_money_input_flow` | Covered |
| Friends, Who, Guild, Communities, Club Finder | Friends, who query, guild roster, club finder row/mode changes | `tests/mists_social_panels.rs::mists_social_panels_support_friends_who_guild_and_communities` | Covered |
| Inspect and guild control LoD panels | Inspect tab switching to guild details plus GuildControl tab/rank/permission interactions | `tests/mists_inspect_guild_control_interactions.rs::mists_inspect_and_guild_control_panels_support_interaction_flows` | Mists-specific |
| PvP UI: HonorFrame, BG queue, Conquest | Honor, battleground queue, conquest/rated panel state | `tests/mists_pvp_ui_panel.rs::mists_pvp_ui_supports_honor_battleground_and_conquest_panels` | Covered |
| LFG, LFR, Raid Browser | Dungeon finder, raid finder, raid browser selections | `tests/mists_lfg_lfr_panel.rs::mists_lfg_lfr_and_raid_finder_panels_render_with_seeded_choices` | Mists-specific |
| Raid unit frames LoD panel | Raid roster seeding and legacy raid group frame rendering | `tests/mists_raid_arena_unit_frames.rs::mists_raid_unit_frames_emit_filtered_textures` | Mists-specific |
| Arena enemy unit frames LoD panel | Arena instance seeding and enemy unit frame rendering | `tests/mists_raid_arena_unit_frames.rs::mists_arena_enemy_frames_emit_filtered_textures` | Mists-specific |
| Battlefield map LoD panel | Battlefield minimap LoD load, map canvas sizing, and screenshot dispatch | `tests/mists_battlefield_map_panel.rs::mists_battlefield_map_screenshot_dispatch_avoids_zero_canvas_scale` | Mists-specific |
| Collections: mounts, pets, toys, heirlooms, transmog | Mount, toy, heirloom tab data and item actions | `tests/mists_collections_panel.rs::mists_collections_tabs_render_without_wardrobe`; `tests/mists_collections_panel.rs::mists_collections_rows_drive_mount_toy_and_heirloom_actions` | Mists-specific |
| Pet Journal and Battle Pet UI | Pet selection and battle action buttons | `tests/mists_pet_battle_ui.rs::mists_pet_journal_and_battle_pet_ui_render_and_interact` | Covered |
| Achievements and Calendar | Achievement tab switching and calendar month navigation | `tests/mists_achievements_calendar_panel.rs::mists_achievements_tabs_and_calendar_navigation_round_trip` | Covered |
| Archaeology panel | Archaeology help/summary tab state and renderable panel load | `tests/mists_legacy_service_panel_interactions.rs::mists_legacy_service_panels_support_state_changing_interactions` | Covered |
| Craft panel | Legacy CraftFrame row selection and selected recipe detail state | `tests/mists_legacy_service_panel_interactions.rs::mists_legacy_service_panels_support_state_changing_interactions` | Covered |
| TradeSkill panel | Legacy TradeSkillFrame row selection plus repeat-count controls | `tests/mists_legacy_service_panel_interactions.rs::mists_legacy_service_panels_support_state_changing_interactions` | Covered |
| Encounter Journal | Tier selection, instance display, loot/overview tabs | `tests/mists_encounter_journal_panel.rs::mists_encounter_journal_opens_and_displays_instances` | Covered |
| Challenge mode LoD panel | Challenge dungeon row click, selected-map state, selected texture, and details refresh | `tests/mists_utility_dialog_interactions.rs::mists_utility_and_dialog_panels_support_state_changing_interactions` | Covered |
| Currency and Token UI | Legacy currency list wrappers and watched token rendering | `tests/mists_currency_token_ui_panel.rs::mists_currency_token_ui_populates_rows_and_watched_tokens` | Covered |
| Store, CatalogShop, WowToken, and SimpleCheckout | Store micro-button dispatch, CatalogShop category/product selection, checkout sizing/show path, WoW Token API probes | `tests/mists_store_commercial_panel.rs::mists_store_catalog_checkout_and_token_surfaces_are_interactive` | Covered |
| Item socketing, reforging, and upgrade LoD panels | Socket proposal/apply path, reforging service action controls, and item-upgrade selection/clear state | `tests/mists_legacy_service_panel_interactions.rs::mists_legacy_service_panels_support_state_changing_interactions` | Mists-specific |
| NPC service LoD panels: barber and black market | Barber altered-form state and black-market row selection state | `tests/mists_legacy_service_panel_interactions.rs::mists_legacy_service_panels_support_state_changing_interactions` | Mists-specific |
| Class trainer LoD panel | ClassTrainerFrame LoD load, legacy trainer services, and named texture wiring | `tests/mists_craft_ui_panel.rs::mists_class_trainer_frame_loads_with_named_textures`; `tests/mists_trainer_api.rs::legacy_trainer_services_expose_selectable_recipe_rows` | Mists-specific |
| Quest choice LoD dialog | Seeded option population and option-button hide/response path | `tests/mists_utility_dialog_interactions.rs::mists_utility_and_dialog_panels_support_state_changing_interactions`; `tests/quest_verbs.rs::c_quest_choice_populates_seeded_options_and_records_response` | Covered |
| Macro and key bindings | Macro selection/run and keybinding mutation round trips | `tests/mists_macro_keybindings_panel.rs::mists_macro_selection_and_keybindings_mutate_state` | Covered |
| Interface options | Game-menu options flow, interface category focus, no retail EditMode | `tests/mists_interface_options_panel.rs::mists_game_menu_options_drives_settings_help_and_addons` | Mists-specific |
| Action bars, micro menu, bag bar, status bars | Action button updates, micro menu textures, bag/status bar state | `tests/mists_action_bars_panel.rs::mists_action_bag_micro_and_status_bars_update_slots_and_hover_scripts`; `tests/mists_action_micro_bag_status.rs::mists_action_micro_bag_and_status_bars_are_interactive` | Covered |
| Time manager and move pad LoD utilities | Stopwatch toggle, alarm message/time-format state, and MovePad opposing-button movement state | `tests/mists_utility_dialog_interactions.rs::mists_utility_and_dialog_panels_support_state_changing_interactions` | Covered |
| Nameplates | Nameplate driver acquisition and renderable unit frame state | `tests/mists_nameplates_panel.rs::mists_nameplate_driver_acquires_renderable_unit_frame` | Covered |
| Loot, group loot, personal loot | Loot slot take, group roll choice, bonus roll state | `tests/mists_loot_panel.rs::mists_loot_group_and_bonus_roll_actions_record_state` | Covered |
| Game menu options | Options, help, addons, close/reopen behavior | `tests/mists_interface_options_panel.rs::mists_game_menu_options_drives_settings_help_and_addons` | Covered |

No currently audited retail-supported panel workflow is marked `Missing` or
`Follow-up`. The post-Spellbook profession-action audit passed on 2026-05-14
with no remaining weak interaction rows identified.

## Retail Module Comparison Audit

Rechecked 2026-05-14 against the retail Blizzard UI module coverage plan at
`/syncthing/Sync/Projects/wow/wow-ui-sim/PLAN.tests.md`. The first
Mists-applicable user-facing gap found in that pass was the commercial/store
path:

- Retail coverage has a full `Blizzard_AccountStore` module suite for
  storefront state, category/item selection, purchase/refund actions, and
  fullscreen escape behavior.
- Mists does not ship `Blizzard_AccountStore`; the equivalent user-facing
  surfaces are `Blizzard_StoreUI`, `Blizzard_CatalogShop`,
  `Blizzard_WowTokenUI`, and `Blizzard_SimpleCheckout`.
- Those Mists addons exist under
  `Interface/BlizzardUI/Mists/AddOns/`, and a local load probe confirmed
  `UIParentLoadAddOn("Blizzard_StoreUI")` succeeds with `StoreFrame` and
  `CatalogShopFrame` defined and zero captured `lua-errors`.
- Current Mists evidence only includes the live GUI
  `StoreMicroButton -> StoreFrame_IsShown` probe. There is no retained
  `docs/baselines/mists-panels.md` row, panel screenshot/dump artifact, or
  interaction-audit row for Store/CatalogShop/WowToken/SimpleCheckout yet.

That follow-up is now covered by `tests/mists_store_commercial_panel.rs` and the
`Store, CatalogShop, WowToken, and SimpleCheckout` row in
`docs/baselines/mists-panels.md`.

The current coverage index is `docs/baselines/mists-test-coverage.md`. Its first
weaker Mists-applicable workflow was Spellbook/professions, and that is now
covered by invoking a visible profession button through its Blizzard `OnClick`
handler and asserting the selected trade skill line changes.
