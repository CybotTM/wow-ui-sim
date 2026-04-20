# PLAN.tests.md

## Test Sweep Triage

- [x] Full test-sweep triage: latest `cargo test --tests --no-fail-fast` sweep is not "one stuck tail test" territory — current count is **3187 passed / 1126 failed**. Bucket failures by subsystem/root cause from `/tmp/claude/cargo-test.out` first, then fix by cluster instead of chasing individual tests in execution order.
  - [x] Confirm sweep shape from `/tmp/claude/cargo-test.out` is clustered failure territory, not one late stuck tail test.
  - [x] Bucket legacy achievement failures: missing globals, category plumbing, criteria plumbing, and earn-achievement behavior.
  - [x] Bucket startup/UI boot failures: missing globals and helpers such as `RegisterCVar`, `GetCVarBitfield`, `FadingFrame_OnLoad`, `CooldownFrame_Set`, `MinimapCluster`, and `MapUtil`.
  - [x] Bucket addon-list failures: nil-group path in addon-list scroll/update handling.
  - [x] Bucket addon-coverage failures: poison-lock fallout after the first shard panic contaminates later shards.
  - [x] Record bucket summary directly in `PLAN.tests.md` so follow-up fixes can target subsystems instead of individual test execution order.

## Test Fix Buckets

- [x] Fix achievement globals bucket: restore legacy top-level achievement globals expected by tests and Blizzard startup code.
- [x] Fix achievement category bucket: restore category lookup/list/traversal behavior used by `achievements_api` tests.
- [x] Fix achievement criteria bucket: restore criteria query/count/progress behavior used by `achievements_api` and related tests.
- [x] Fix achievement earn-state bucket: make `A_Admin.EarnAchievement` update earned state consistently across achievement APIs.
- [x] Fix achievement event bucket: make earn-achievement fire the expected event(s) with the expected delivery timing/payload.
- [x] Re-run achievement-focused suites (`cargo test --test achievements_api`, targeted `cargo test --lib c_achievement`) and split any remaining achievement failures into new unchecked tasks.

- [x] Fix startup CVar registration bucket: implement `RegisterCVar` behavior needed during UI bootstrap.
- [x] Fix fading-frame startup bucket: implement `FadingFrame_OnLoad` and adjacent fading-frame helpers needed during boot.
- [x] Fix cooldown startup bucket: implement `CooldownFrame_Set` and any immediate helper/method dependencies it needs.
- [x] Fix startup CVar bitfield bucket: implement `GetCVarBitfield` behavior used by startup/UI code.
- [x] Fix minimap startup bucket: provide `MinimapCluster` global/frame expectations required by startup code.
- [x] Fix map-util startup bucket: provide `MapUtil` helpers referenced during startup/UI initialization.
- [x] Fix remaining startup blocker bucket: identify and resolve next startup-only missing global/helper after the named blockers stop dominating.
- [x] Re-run startup/UI-focused failing suites after each blocker cluster to keep the next boot blocker visible.
- [x] Fix startup EventUtil bucket: provide the `EventUtil` helpers now dominating the startup warning sweep.
- [x] Fix startup SetupLocalization bucket: provide localization bootstrap helpers still failing during startup/UI initialization.
- [x] Fix startup FrameUtil bucket: provide `FrameUtil` helpers still used by `Blizzard_FrameXML` startup code.

- [x] Fix addon-list nil-group bucket: make addon-list update paths tolerate nil/missing groups without panicking.
- [x] Fix addon-list scroll/update bucket: restore scroll-frame/update behavior once nil-group handling is correct.
- [x] Re-run addon-list-focused failing suites and split any remaining addon-list failures into new unchecked tasks.
- [x] Fix addon-list button texture bucket: restore `AddonList.EnableAllButton.Left` atlas wiring in `blizzard_shared`.

- [x] Fix addon-coverage shard-poison bucket: stop the first shard panic from poisoning shared state for later shards.
- [x] Fix addon-coverage isolation bucket: reset or isolate shared state/locks between coverage shards so one failure does not cascade.
- [x] Re-run addon-coverage-focused suites after poison-lock isolation is in place.
- [x] Fix addon-coverage runtime LoD shard-10 bucket: make `Blizzard_Contribution` load cleanly during post-startup shard coverage.
- [x] Fix addon-coverage runtime LoD shard-13 bucket: restore the missing runtime helpers hit by `Blizzard_GlueParent` and related shard-13 addons.
- [x] Fix addon-coverage panel-open runtime bucket: restore the Collections opener path so `panel_open_runtime_paths_stay_within_known_error_baseline` no longer dies on `Lua error: not a function`.
- [x] Fix addon-coverage shard-14 follow-up bucket: keep `shard_14_runtime_load_survives_prior_runtime_shards_in_process` aligned with the remaining shard-10 runtime `LoadAddOn` failure.

- [x] Re-run `cargo test --tests --no-fail-fast`, update bucket counts, and split any remaining high-volume failure groups into new unchecked subsystem tasks instead of collapsing them into one item.
  - [x] Fresh rerun completed on 2026-04-18 from `/tmp/claude/cargo-test-tests-rerun.out`.
  - [x] Current sweep shape is `180 failed targets` from Cargo's final summary, not a single dominant startup/addon-coverage cluster.
  - [x] Follow-up work is now split into the concrete subsystem buckets below.

## Current High-Volume Failure Buckets

- [x] Revisit the shims you added to implement
- [x] Why are you refusing to implement to replace the shims
- [x] Re-organize the directory structure into c_api that will contain all the c_api calls, includiding two sub folders with temporary-shims and permanent-shims, classify shims as temporary if possible
- [x] Replace temporary shims
- [x] Re-organize the directory structure for lua_api to include two sub folders one with temporary-shims and permanent-shims, outside of those it should always be real implementations
- [x] Fix compilation warnings
- [x] Fix compilation erros
- [x] Fix startup performance issue
- [x] Startup performance tests that verifies that full load of blizzard ui reaches `[Startup] Firing UPDATE_CHAT_WINDOWS` in less than 20s
- [x] Fix core loader/widget surface bucket (`19` failed targets): `lib`, `generated_data_refresh_coverage`, `global_frames`, `global_function_diff_coverage`, `intrinsic_types`, `method_diff_coverage`, `methods_anchor`, `methods_button`, `methods_button_textures`, `methods_texture`, `parent_sub`, `userdata_proxy`, `widget_methods`, `widget_methods_colorselect`, `widget_methods_model`, `widget_misc_methods`, `widget_slider`, `xml_parsing`, `xml_templates`.
- [x] Fix action bar and click-casting bucket (`14` failed targets): `action_bar`, `action_bar_drag`, `action_bar_tree`, `action_button_spell_alerts`, `click_all_frames`, `click_targeting`, `cooldown_widget`, `key_dispatch`, `keyboard`, `panel_harness_runtime`, `test_keybindings`, `test_keybindings_panels`, `test_keybindings_panels_detail`, `test_keybindings_targeting`.
- [x] Fix menus/glue/startup navigation bucket (`15` failed targets): `game_menu`, `glue_character_select`, `glue_login`, `micro_menu`, `prototype_dialog`, `screen_mode`, `startup_api_stubs`, `startup_perf`, `test_guild_panel`, `test_mail`, `test_premade_groups`, `test_showuipanel_lod`, `test_showuipanel_lod_player_spells`, `test_showuipanel_toggles`, `world_map_voice_button_order`.
- [x] Fix animation/dispatcher/onupdate bucket (`10` failed targets): `anim_target_visibility`, `animation_anim`, `animation_group`, `animation_group_state`, `arrow_callout_manager`, `dispatcher`, `event_dispatch_perf`, `event_scheduler`, `onupdate_handler_audit`, `uiparent_onupdate_worklists`.
- [x] Fix aura legacy surface bucket: restore `UnitBuff` / `UnitDebuff` / `UnitAura` / `GetPlayerAuraBySpellID` / `AuraUtil` behavior used by `aura_api` and `aura_table_shape` tests.
- [x] Fix aura and buff surface bucket (`8` failed targets): `admin_buff_api`, `aura_api`, `aura_table_shape`, `bags`, `battle_net_api`, `tooltip_world_cursor`, `unit_auras_private`, `workarounds_bags`.
- [x] Fix unit/spell/character-stats bucket (`6` failed targets): `admin_combat_api`, `character_stats`, `spell_api`, `spell_casting`, `spellbook`, `unit_api`.
- [x] Fix admin/state-backed services bucket (`31` failed targets): `account_services`, `admin_actionbar_api`, `admin_economy_api`, `admin_encounter_api`, `admin_event_api`, `admin_identity_api`, `admin_movement_api`, `admin_pvp_guild_api`, `admin_spec_talent_api`, `admin_spell_effects_api`, `admin_vault_api`, `campaign_info`, `commentator_api`, `covenant_sanctum_ui`, `friend_list_who`, `guild_info`, `level_link`, `neighborhood_initiative`, `party_info_loot`, `pet_battles`, `pet_battles_counts`, `pet_info`, `professions_api`, `report_system`, `reputation_api`, `store_api`, `store_tree`, `trade_info`, `transmog_outfit_info`, `world_quest_api`, `zone_ability`.
- [x] Fix C namespace and legacy-global coverage bucket (`18` failed targets): `addon_nil_symbol_report`, `c_collection_api`, `c_item_api`, `c_map_api`, `c_map_probes`, `c_namespace_noop_replacements`, `c_system_api`, `c_transmog_heirloom_api`, `globals_legacy`, `globals_legacy_quest_blobs`, `missing_apis`, `nil_symbol_access`, `pool_api`, `system_api`, `system_api_seeded`, `test_cvar_display_settings`, `utility_api`, `utility_pools`.
- [x] Fix seeded content/map/housing/hero bucket (`11` failed targets): `catalog_shop`, `hero_talents`, `hero_talents_render`, `hero_talents_render_visual`, `house_exterior`, `housing_catalog`, `housing_customize_mode`, `housing_decor`, `housing_neighborhood`, `map_canvas_pins`, `test_collections`.
- [x] Fix frame controls and text-widget bucket (`9` failed targets): `chat_frame`, `checkbox`, `debug_api`, `dropdown_api`, `editbox_stub_family`, `font_api`, `frame_creation`, `frame_creation_checkbutton`, `message_frame`.
- [x] Fix layout/render/visual regression bucket (`24` failed targets): `blizzard_ui_unit`, `frame_positions`, `game_time_frame_perf`, `layout_perf`, `nine_slice`, `objective_tracker_titles`, `render_order`, `render_order_world_map`, `rendering_pipeline`, `scroll_widgets`, `scroll_widgets_minimal`, `simple_html`, `store_anchor_regression`, `tooltip`, `tooltip_allow_empty`, `tooltip_anchoring`, `tooltip_basic`, `tooltip_hover`, `tooltip_item_spell`, `tooltip_shrink_to_fit_wrapped`, `tooltip_text`, `tooltip_text_layout`, `tooltip_word_wrap_min_width`, `widget_registry_perf`.
- [x] Fix systems/security/misc gameplay bucket (`15` failed targets): `encounter_events`, `equipment_api`, `protected_frame_enforcement`, `pvp_info`, `quest_async`, `quest_counts`, `reincarnation`, `rot_damage`, `scenario_info`, `secure_transfer`, `security_api`, `talent_change_events`, `test_addon`, `test_sim_commands`, `ui_frame_manager`.

- [x] Re-run `cargo test --tests --no-fail-fast`, update bucket counts, and split any remaining high-volume failure groups into new unchecked subsystem tasks instead of collapsing them into one item.
  - [x] Fresh rerun captured 12 lib failures before the harness stopped, plus a separate `action_bar` cluster with 7 failures.
  - [x] The remaining clusters below are split into subsystem-sized follow-up tasks instead of one umbrella item.
- [x] Fix spellbook with `s` keybind not showing

- [x] Fix loader/global-frame access bucket (`2` failed targets): `global_frame_access` child/template key wiring regressions.
- [x] Fix startup bootstrap namespace bucket (`3` failed targets): `wow_api_globals` startup namespace checks.
- [x] Fix XML inheritance bucket (`1` failed target): `xml_basics` inherited button text availability after load.
- [x] Fix state-render repair bucket (`3` failed targets): `state_render_repairs` and `state_render` bucket-order repair regressions.
- [x] Fix global-slots bucket (`2` failed targets): `global_slots` root-global refresh behavior.
- [x] Fix editmode workarounds bucket (`1` failed target): `workarounds_editmode` preset cloning behavior.
- [x] Fix action-bar startup bucket (`7` failed targets): `action_bar` startup/load behavior that still depends on `WowLuaEnv` initialization.
- [x] Fix probe bucket (`6` failed targets): `c_club_probes`, `c_map_api`, `c_pet_battles_probes`, `c_small_probes`.
- [x] Investigate addon-loading perf harness failure (`1` failed target): `addon_loading_perf`.
- [x] Re-run `cargo test --tests --no-fail-fast`, update bucket counts, and split any remaining high-volume failure groups into new unchecked subsystem tasks instead of collapsing them into one item.
  - [x] Fresh 2026-04-19 rerun warmed `CARGO_TARGET_DIR=/tmp/wow-ui-sim-full-sweep`; the direct `timeout 90 cargo test --tests --no-fail-fast` run stalled in `action_bar`, so the final sweep used per-executable `timeout 90` runs against all `377` built test targets.
  - [x] Current remaining count after the full sweep: `39` failed targets and `1` timeout (`test_keybindings_panels_detail`).
  - [x] Remaining work is split below by subsystem instead of keeping another umbrella rerun item.
- [x] Fix saved-variables serialization lib regression bucket (`1` failed target, `7` failed lib tests): `wow_ui_sim` `saved_variables::saved_variables_serialize::*` failures after the missing-saved-vars default change.
- [x] Fix loader inherited-layer lib regression bucket (`1` failed target, `1` failed lib test): `wow_ui_sim` `loader::tests::test_runtime_template_creates_inherited_layer_regions`.
- [x] Fix panel/glue/navigation startup bucket (`13` failed targets, `1` timeout): `game_boot`, `glue_character_select`, `glue_login`, `micro_menu`, `panel_harness_runtime`, `panel_toggle_verbs`, `startup_warnings`, `store_tree`, `test_keybindings_panels_detail`, `test_showuipanel_auction_house`, `test_showuipanel_lod`, `test_showuipanel_lod_fixtures`, `test_showuipanel_toggles`.
- [x] Fix click/targeting/protected interaction bucket (`5` failed targets): `action_bar_drag`, `click_all_frames`, `click_targeting`, `protected_attribute_enforcement`, `targeting_verbs`.
- [x] Fix spell/cast-state bucket (`3` failed targets): `combat_verbs`, `spell_casting`, `spell_state_probes`.
- [x] Fix pet-battle seeded/default-count bucket (`2` failed targets): `pet_battles`, `pet_battles_counts`.
- [x] Fix scroll/html/widget layout bucket (`4` failed targets): `scroll_widgets`, `scroll_widgets_minimal`, `simple_html`, `widget_registry_perf`.
- [x] Fix world-map/hero visual bucket (`2` failed targets): `hero_talents_render_visual`, `world_map_onupdate_inventory`.
- [x] Fix alpha-visibility expectation drift bucket (`1` failed target): `frame_creation` `IsVisible()` behavior under zero-alpha parents.
- [x] Refresh coverage/audit drift bucket (`3` failed targets): `addon_coverage`, `globals_legacy`, `method_diff_coverage`.
- [ ] Fix widget/pool/heirloom surface bucket (`3` failed targets): `heirloom_probes`, `pool_api`, `widget_slider`.
- [ ] Fix onupdate audit bucket (`1` failed target): `onupdate_handler_audit`.
- [ ] Fix admin split harness bucket (`1` failed target): `rilua_admin_split_smoke`.
