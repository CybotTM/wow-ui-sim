# Mists Blizzard UI Test Coverage Index

Captured 2026-05-14 on `classic-profile-rollout`.

This index ties the Mists panel parity rows to the local `client-mists` tests
and to the comparable retail Blizzard UI coverage board in
`/syncthing/Sync/Projects/wow/wow-ui-sim/PLAN.tests.md`. It is intentionally a
coverage map, not a screenshot manifest; retained visual artifacts remain in
`docs/baselines/mists-panels.md` and `docs/baselines/mists-release-proof.md`.

Status meanings:

- `Comparable`: Mists has state-changing or data-backed evidence for the
  Pandaria-era equivalent of the retail workflow.
- `Mists-specific`: the retail workflow exists, but the Pandaria-era UI uses a
  materially different frame or interaction model.
- `Weaker`: Mists has load/render evidence, but the equivalent retail workflow
  still has stronger interaction evidence.

| Mists panel row | Local `client-mists` evidence | Comparable retail coverage | Status |
|---|---|---|---|
| Character panel: paperdoll, stats, titles, equipment manager | `tests/mists_character_panel.rs`; `tests/mists_character_reputation_panels.rs` | Character/paperdoll, reputation, titles, and equipment-manager module rows in `PLAN.tests.md` | Comparable |
| Spellbook and professions | `tests/mists_spellbook_panel.rs`; `tests/mists_craft_ui_panel.rs`; `tests/mists_trade_skill_api.rs`; `tests/professions_api.rs` | Retail SpellBook/PlayerSpells plus profession-surface rows in `PLAN.tests.md` | Comparable |
| Talents and glyphs | `tests/mists_talents_glyphs_panel.rs` | Retail TalentUI/PlayerSpells talent rows | Mists-specific |
| Quest log and objective tracker | `tests/mists_quest_panel.rs` | Quest log, quest watch, objective tracker rows | Comparable |
| World map | `tests/mists_world_map_panel.rs`; `tests/mists_world_map_opacity.rs` | World map, map navigation, POI/pin rows | Comparable |
| Mail: inbox, send, attachments, COD | `tests/mists_mail_panel.rs` | Mail frame inbox/send rows | Comparable |
| Auction House: browse, bid, post, cancel | `tests/mists_auction_house_panel.rs` | Auction House browse/bid/post/cancel rows | Comparable |
| AddOn list and UI management LoD panels | `tests/mists_addonlist_interactions.rs` | AddOnList and AddOnPerformance rows | Comparable |
| Bank, ReagentBank, Void Storage, Guild Bank | `tests/mists_bank_storage_panel.rs` | Bank, reagent bank, void storage, guild bank rows | Comparable |
| Trade window and `TradePlayerInputMoneyFrame` | `tests/mists_trade_panel.rs` | TradeFrame and money-input rows | Comparable |
| Friends, Who, Guild, Communities, Club Finder | `tests/mists_social_panels.rs` | FriendsFrame, WhoFrame, GuildFrame, Communities/Club Finder rows | Comparable |
| Inspect and guild control LoD panels | `tests/mists_inspect_guild_control_interactions.rs` | Inspect and guild-control rows | Comparable |
| PvP UI: HonorFrame, BG queue, Conquest | `tests/mists_pvp_ui_panel.rs` | PVP UI, honor, battleground queue, conquest rows | Comparable |
| LFG, LFR, Raid Browser | `tests/mists_lfg_lfr_panel.rs` | Group Finder / LFG / LFR rows | Mists-specific |
| Raid unit frames LoD panel | `tests/mists_raid_arena_unit_frames.rs` | Raid UI / compact unit frame rows | Comparable |
| Arena enemy unit frames LoD panel | `tests/mists_raid_arena_unit_frames.rs` | Arena enemy frame rows | Comparable |
| Battlefield map LoD panel | `tests/mists_battlefield_map_panel.rs` | Battlefield map rows | Comparable |
| Collections: mounts, pets, toys, heirlooms, transmog | `tests/mists_collections_panel.rs` | CollectionsJournal rows | Mists-specific |
| Pet Journal and Battle Pet UI | `tests/mists_pet_battle_ui.rs` | Pet Journal and Battle Pet UI rows | Comparable |
| Achievements and Calendar | `tests/mists_achievements_calendar_panel.rs` | Achievement UI and Calendar rows | Comparable |
| Archaeology panel | `tests/mists_legacy_service_panel_interactions.rs` | Archaeology rows | Comparable |
| Craft panel | `tests/mists_craft_ui_panel.rs`; `tests/mists_legacy_service_panel_interactions.rs` | Craft/profession rows | Comparable |
| TradeSkill panel | `tests/mists_trade_skill_api.rs`; `tests/mists_legacy_service_panel_interactions.rs` | TradeSkill/profession rows | Comparable |
| Encounter Journal | `tests/mists_encounter_journal_panel.rs` | Encounter Journal rows | Comparable |
| Challenge mode LoD panel | `tests/mists_utility_dialog_interactions.rs` | Challenge mode rows | Comparable |
| Currency and Token UI | `tests/mists_currency_token_ui_panel.rs`; `tests/mists_currency_list.rs` | Currency and Token UI rows | Comparable |
| Store, CatalogShop, WowToken, and SimpleCheckout | `tests/mists_store_commercial_panel.rs`; `tests/mists_product_choice_compat.rs` | Retail AccountStore and product-choice rows | Mists-specific |
| Item socketing, reforging, and upgrade LoD panels | `tests/mists_legacy_service_panel_interactions.rs` | Item socketing, reforging, and upgrade rows | Comparable |
| NPC service LoD panels: barber and black market | `tests/mists_legacy_service_panel_interactions.rs` | Barber shop and Black Market rows | Comparable |
| Class trainer LoD panel | `tests/mists_trainer_api.rs`; `tests/mists_legacy_service_panel_interactions.rs` | Class trainer rows | Comparable |
| Quest choice LoD dialog | `tests/mists_utility_dialog_interactions.rs` | Quest choice / reward choice rows | Comparable |
| Macro and key bindings | `tests/mists_macro_keybindings_panel.rs` | Macro and key binding rows | Comparable |
| Interface options | `tests/mists_interface_options_panel.rs` | Interface options / settings rows | Mists-specific |
| Action bars, micro menu, bag bar, status bars | `tests/mists_action_bars_panel.rs`; `tests/mists_action_micro_bag_status.rs`; `tests/mists_micro_menu_icons.rs` | ActionBar, MicroMenu, bag bar, and status bar rows | Comparable |
| Time manager and move pad LoD utilities | `tests/mists_utility_dialog_interactions.rs` | Time manager and move pad rows | Comparable |
| Nameplates | `tests/mists_nameplates_panel.rs`; `tests/mists_nameplate_scale.rs` | Nameplate rows | Comparable |
| Loot, group loot, personal loot | `tests/mists_loot_panel.rs` | Loot, group loot, and bonus/personal loot rows | Comparable |
| Game menu options | `tests/mists_interface_options_panel.rs` | Game menu and settings rows | Comparable |

## Closed Weaker Workflow

The first Mists-applicable workflow with weaker evidence was the professions
side of Spellbook. That gap is now covered by
`tests/mists_spellbook_panel.rs::mists_spellbook_populates_visible_spell_buttons`:
the test opens the profession tab, invokes a visible profession button through
its Blizzard `OnClick` handler, and asserts the selected trade skill line
changes to Mining.

Next audit pass: re-run the interaction/retail comparison audit after this
profession-action coverage and promote the first remaining weaker workflow, if
one still exists.
