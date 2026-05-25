//! Post-load workarounds that are still required on the live rilua path.

mod permanent;
mod runtime_surfaces;
mod temporary;

pub(crate) use temporary::environment_cleanup_restore::restore_post_cleanup_globals;
pub(crate) use temporary::source_patches::patch_lua_source;

pub use runtime_surfaces::patch_uiparent_managed_frame_mixin;
use runtime_surfaces::*;
pub(crate) use runtime_surfaces::{
    patch_account_store_set_storefront, patch_glueparent_uiparent_attributes,
    patch_map_canvas_scroll_container, patch_playerspells_onload_backfill, patch_quest_log_mixin,
    patch_shared_xml_anim_mixins, patch_unit_position_frame_mixin,
};
use std::time::Instant;

struct WorkaroundStep {
    label: &'static str,
    apply: fn(&crate::lua_api::WowLuaEnv),
}

const POST_LOAD_WORKAROUNDS: &[WorkaroundStep] = &[
    WorkaroundStep {
        label: "patch_edit_mode_manager",
        apply: patch_edit_mode_manager,
    },
    WorkaroundStep {
        label: "init_edit_mode_layout",
        apply: init_edit_mode_layout,
    },
    WorkaroundStep {
        label: "patch_ui_parent_panel_toggles",
        apply: patch_ui_parent_panel_toggles,
    },
    WorkaroundStep {
        label: "patch_uiparent_onupdate_worklists",
        apply: patch_uiparent_onupdate_worklists,
    },
    WorkaroundStep {
        label: "init_chat_type_colors",
        apply: init_chat_type_colors,
    },
    WorkaroundStep {
        label: "patch_chat_voice_button_surface",
        apply: patch_chat_voice_button_surface,
    },
    WorkaroundStep {
        label: "patch_item_socketing_tooltips",
        apply: patch_item_socketing_tooltips,
    },
    WorkaroundStep {
        label: "patch_character_select_selected_name",
        apply: patch_character_select_selected_name,
    },
    WorkaroundStep {
        label: "patch_character_create_defaults",
        apply: patch_character_create_defaults,
    },
    WorkaroundStep {
        label: "patch_character_frame_title_refresh",
        apply: patch_character_frame_title_refresh,
    },
    WorkaroundStep {
        label: "patch_vignette_pin_template",
        apply: patch_vignette_pin_template,
    },
    WorkaroundStep {
        label: "patch_fog_of_war_pin_mixin",
        apply: patch_fog_of_war_pin_mixin,
    },
    WorkaroundStep {
        label: "patch_map_exploration_pin_mixin",
        apply: patch_map_exploration_pin_mixin,
    },
    WorkaroundStep {
        label: "patch_map_canvas_data_provider_attachment",
        apply: patch_map_canvas_data_provider_attachment,
    },
    WorkaroundStep {
        label: "ensure_adventure_map_frame_surface",
        apply: ensure_adventure_map_frame_surface,
    },
    WorkaroundStep {
        label: "patch_action_bar_button_event_fanout",
        apply: patch_action_bar_button_event_fanout,
    },
    WorkaroundStep {
        label: "patch_paging_controls_page_text",
        apply: patch_paging_controls_page_text,
    },
    WorkaroundStep {
        label: "patch_talent_edge_frame_level_sync",
        apply: patch_talent_edge_frame_level_sync,
    },
    WorkaroundStep {
        label: "patch_catalog_shop_product_card_defaults",
        apply: patch_catalog_shop_product_card_defaults,
    },
    WorkaroundStep {
        label: "patch_game_time_defaults",
        apply: patch_game_time_defaults,
    },
    WorkaroundStep {
        label: "patch_tooltip_nineslice_surface",
        apply: patch_tooltip_nineslice_surface,
    },
    WorkaroundStep {
        label: "patch_container_frame_token_tracker",
        apply: patch_container_frame_token_tracker,
    },
    WorkaroundStep {
        label: "patch_achievement_display_set_achievements",
        apply: patch_achievement_display_set_achievements,
    },
    WorkaroundStep {
        label: "patch_housing_dashboard_preload",
        apply: patch_housing_dashboard_preload_from_env,
    },
    WorkaroundStep {
        label: "patch_lfg_lock_list",
        apply: patch_lfg_lock_list,
    },
    WorkaroundStep {
        label: "patch_auction_house_browse_results_event",
        apply: patch_auction_house_browse_results_event_from_env,
    },
    WorkaroundStep {
        label: "patch_auction_house_search_context_aliases",
        apply: patch_auction_house_search_context_aliases_from_env,
    },
    WorkaroundStep {
        label: "patch_auth_challenge_frame_parent",
        apply: patch_auth_challenge_frame_parent_from_env,
    },
    WorkaroundStep {
        label: "patch_settings_canvas_layout_visibility",
        apply: patch_settings_canvas_layout_visibility,
    },
];

pub fn apply(env: &crate::lua_api::WowLuaEnv) {
    for step in POST_LOAD_WORKAROUNDS {
        log_step(env, step.label, || (step.apply)(env));
    }
}

pub(crate) fn apply_permanent_bootstrap(lua: &mut rilua::Lua) -> crate::Result<()> {
    permanent::apply_bootstrap(lua)
}

pub(crate) fn apply_temporary_bootstrap(lua: &mut rilua::Lua) -> crate::Result<()> {
    apply_temporary_state_bootstrap(lua)?;
    apply_temporary_namespace_bootstrap(lua)
}

fn apply_temporary_state_bootstrap(lua: &mut rilua::Lua) -> crate::Result<()> {
    apply_runtime_state_bootstrap(lua)?;
    apply_account_and_social_state_bootstrap(lua)?;
    apply_player_state_bootstrap(lua)?;
    apply_secure_and_store_state_bootstrap(lua)?;
    apply_unit_state_bootstrap(lua)?;
    temporary::uiparent_onupdate_worklists::apply_bootstrap(lua)?;
    temporary::video_options_state::apply_bootstrap(lua)
}

fn apply_runtime_state_bootstrap(lua: &mut rilua::Lua) -> crate::Result<()> {
    temporary::event_scheduler_state::apply_bootstrap(lua)?;
    temporary::combat_log_state::apply_bootstrap(lua)?;
    temporary::damage_meter_state::apply_bootstrap(lua)?;
    temporary::encounter_state::apply_bootstrap(lua)?;
    temporary::housing_catalog_state::apply_bootstrap(lua)?;
    temporary::map_runtime_state::apply_bootstrap(lua)?;
    temporary::perks_activities_state::apply_bootstrap(lua)?;
    temporary::private_aura_state::apply_bootstrap(lua)?;
    temporary::reputation_state::apply_bootstrap(lua)
}

fn apply_account_and_social_state_bootstrap(lua: &mut rilua::Lua) -> crate::Result<()> {
    temporary::battle_net_account_defaults::apply_bootstrap(lua)?;
    temporary::merchant_filter_state::apply_bootstrap(lua)
}

fn apply_player_state_bootstrap(lua: &mut rilua::Lua) -> crate::Result<()> {
    temporary::player_spells_onload_backfill::apply_bootstrap(lua)?;
    temporary::possess_info_defaults::apply_bootstrap(lua)?;
    temporary::totem_defaults::apply_bootstrap(lua)?;
    Ok(())
}

fn apply_secure_and_store_state_bootstrap(lua: &mut rilua::Lua) -> crate::Result<()> {
    temporary::secure_reference_defaults::apply_bootstrap(lua)?;
    temporary::secure_types_defaults::apply_bootstrap(lua)?;
    temporary::secure_transfer_state::apply_bootstrap(lua)?;
    temporary::store_glue_state::apply_bootstrap(lua)
}

fn apply_unit_state_bootstrap(lua: &mut rilua::Lua) -> crate::Result<()> {
    temporary::unit_auras_state::apply_bootstrap(lua)?;
    temporary::unit_stagger_defaults::apply_bootstrap(lua)?;
    temporary::unit_threat_defaults::apply_bootstrap(lua)
}

fn apply_temporary_namespace_bootstrap(lua: &mut rilua::Lua) -> crate::Result<()> {
    apply_core_temporary_namespace_bootstrap(lua)?;
    apply_feature_temporary_namespace_bootstrap(lua)
}

fn apply_core_temporary_namespace_bootstrap(lua: &mut rilua::Lua) -> crate::Result<()> {
    apply_core_foundation_defaults(lua)?;
    apply_core_legacy_defaults(lua)?;
    Ok(())
}

fn apply_core_foundation_defaults(lua: &mut rilua::Lua) -> crate::Result<()> {
    apply_core_foundation_frame_defaults(lua)?;
    apply_core_foundation_state_defaults(lua)?;
    apply_core_dispatcher_and_format_defaults(lua)
}

fn apply_core_foundation_frame_defaults(lua: &mut rilua::Lua) -> crate::Result<()> {
    apply_core_foundation_addon_defaults(lua)?;
    apply_core_foundation_journal_defaults(lua)?;
    temporary::camera_tutorial_defaults::apply_bootstrap(lua)?;
    temporary::catalog_shop_inbound_globals::apply_bootstrap(lua)?;
    temporary::catalog_shop_product_card_defaults::apply_bootstrap(lua)?;
    temporary::character_create_defaults::apply_bootstrap(lua)?;
    temporary::chat_voice_button_surface::apply_bootstrap(lua)?;
    temporary::chat_window_defaults::apply_bootstrap(lua)?;
    temporary::client_info_defaults::apply_bootstrap(lua)?;
    temporary::color_defaults::apply_bootstrap(lua)?;
    temporary::content_tracking_defaults::apply_bootstrap(lua)?;
    temporary::container_portrait_texture::apply_bootstrap(lua)?;
    temporary::debug_environment_defaults::apply_bootstrap(lua)?;
    temporary::difficulty_pvp_util_defaults::apply_bootstrap(lua)?;
    temporary::edit_mode_cache_defaults::apply_bootstrap(lua)?;
    temporary::global_frame_defaults::apply_bootstrap(lua)?;
    Ok(())
}

fn apply_core_foundation_addon_defaults(lua: &mut rilua::Lua) -> crate::Result<()> {
    temporary::addon_compartment_defaults::apply_bootstrap(lua)?;
    temporary::addons_beta_policy_defaults::apply_bootstrap(lua)?;
    temporary::auto_complete_defaults::apply_bootstrap(lua)?;
    temporary::behavioral_messaging_defaults::apply_bootstrap(lua)
}

fn apply_core_foundation_journal_defaults(lua: &mut rilua::Lua) -> crate::Result<()> {
    temporary::achievement_ui_access_defaults::apply_bootstrap(lua)?;
    temporary::achievement_search_preview::apply_bootstrap(lua)?;
    temporary::alert_frame_defaults::apply_bootstrap(lua)?;
    temporary::adventure_journal_fallbacks::apply_bootstrap(lua)
}

fn apply_core_foundation_state_defaults(lua: &mut rilua::Lua) -> crate::Result<()> {
    temporary::pet_battle_runtime_state::apply_bootstrap(lua)?;
    temporary::secure_execute_range::apply_bootstrap(lua)?;
    temporary::settings_surface_defaults::apply_bootstrap(lua)?;
    temporary::tooltip_data_processor_defaults::apply_bootstrap(lua)?;
    temporary::ui_widget_manager_defaults::apply_bootstrap(lua)?;
    Ok(())
}

fn apply_core_dispatcher_and_format_defaults(lua: &mut rilua::Lua) -> crate::Result<()> {
    temporary::dispatcher_callback_defaults::apply_bootstrap(lua)?;
    temporary::dispatcher_surface::apply_bootstrap(lua)?;
    temporary::display_scale_defaults::apply_bootstrap(lua)?;
    temporary::dropdown_list_defaults::apply_bootstrap(lua)?;
    temporary::formatting_utility_defaults::apply_bootstrap(lua)?;
    temporary::game_time_calendar_invites::apply_bootstrap(lua)?;
    temporary::gamepad_cursor_control_defaults::apply_bootstrap(lua)?;
    temporary::game_rules_namespace_fallback::apply_bootstrap(lua)?;
    temporary::glue_character_select_defaults::apply_bootstrap(lua)?;
    temporary::guild_info_namespace_fallback::apply_bootstrap(lua)?;
    temporary::inert_global_defaults::apply_bootstrap(lua)?;
    temporary::inventory_query_defaults::apply_bootstrap(lua)?;
    temporary::item_button_helper_defaults::apply_bootstrap(lua)?;
    temporary::weapon_enchant_defaults::apply_bootstrap(lua)?;
    Ok(())
}

fn apply_core_legacy_defaults(lua: &mut rilua::Lua) -> crate::Result<()> {
    temporary::kiosk_namespace_defaults::apply_bootstrap(lua)?;
    temporary::lfg_legacy_defaults::apply_bootstrap(lua)?;
    temporary::legacy_action_bar_globals::apply_bootstrap(lua)?;
    temporary::legacy_container_globals::apply_bootstrap(lua)?;
    temporary::legacy_spell_globals::apply_bootstrap(lua)?;
    temporary::modified_click_defaults::apply_bootstrap(lua)?;
    temporary::performance_metric_defaults::apply_bootstrap(lua)?;
    temporary::pool_constructor_defaults::apply_bootstrap(lua)?;
    temporary::misc_global_frame_defaults::apply_bootstrap(lua)?;
    Ok(())
}

fn apply_feature_temporary_namespace_bootstrap(lua: &mut rilua::Lua) -> crate::Result<()> {
    temporary::assisted_combat_manager_defaults::apply_bootstrap(lua)?;
    temporary::base_nine_slice_dialog_defaults::apply_bootstrap(lua)?;
    temporary::callback_registry_defaults::apply_bootstrap(lua)?;
    temporary::macro_defaults::apply_bootstrap(lua)?;
    temporary::object_api_request_load_callbacks::apply_bootstrap(lua)?;
    temporary::player_location_defaults::apply_bootstrap(lua)?;
    temporary::profession_specs_defaults::apply_bootstrap(lua)?;
    temporary::proxy_object_factories::apply_bootstrap(lua)?;
    temporary::quest_objective_defaults::apply_bootstrap(lua)?;
    temporary::seconds_formatter_defaults::apply_bootstrap(lua)?;
    temporary::shared_xml_utility_defaults::apply_bootstrap(lua)?;
    temporary::sound_driver_defaults::apply_bootstrap(lua)?;
    temporary::static_model_info_defaults::apply_bootstrap(lua)?;
    temporary::static_popup_defaults::apply_bootstrap(lua)?;
    temporary::top_level_parent_defaults::apply_bootstrap(lua)?;
    temporary::trade_skill_ui_fallbacks::apply_bootstrap(lua)?;
    temporary::transmog_util_defaults::apply_bootstrap(lua)?;
    temporary::ui_parent_panel_toggles::apply_bootstrap(lua)?;
    temporary::ui_frame_manager_defaults::apply_bootstrap(lua)
}

pub fn close_startup_special_windows_before_first_frame(env: &crate::lua_api::WowLuaEnv) {
    temporary::startup_windows::close_before_first_frame(env);
}

pub(crate) fn sanitize_imported_wtf_addon_saved_variables(
    state: &mut rilua::vm::state::LuaState,
    addon_name: &str,
) {
    temporary::details_saved_variables::sanitize_imported_wtf_addon(state, addon_name);
}

pub(crate) fn apply_cpp_mixin_stubs_after_lua_file(env: &crate::lua_api::LoaderEnv<'_>) {
    let _ = temporary::cpp_mixin_stubs::patch_after_lua_file(env);
}

fn patch_edit_mode_manager(env: &crate::lua_api::WowLuaEnv) {
    crate::lua_api::workarounds_editmode::patch_edit_mode_manager(env);
}

fn init_edit_mode_layout(env: &crate::lua_api::WowLuaEnv) {
    crate::lua_api::workarounds_editmode::init_edit_mode_layout(env);
}

fn init_chat_type_colors(env: &crate::lua_api::WowLuaEnv) {
    crate::lua_api::chat_init::init_chat_type_colors(env);
}

fn patch_settings_canvas_layout_visibility(env: &crate::lua_api::WowLuaEnv) {
    temporary::settings_canvas_visibility::patch(env);
}

fn patch_housing_dashboard_preload_from_env(env: &crate::lua_api::WowLuaEnv) {
    patch_housing_dashboard_preload(&env.loader_env());
}

pub fn apply_post_event(env: &crate::lua_api::WowLuaEnv) {
    apply_post_event_bootstrap(env);
    patch_post_event_frame_layout(env);
    refresh_post_event_surfaces(env);
}

fn apply_post_event_bootstrap(env: &crate::lua_api::WowLuaEnv) {
    temporary::post_event_action_button_refresh::patch(env);
    crate::lua_api::workarounds_editmode::init_edit_mode_layout(env);
    crate::lua_api::workarounds_editmode::reapply_player_frame_anchor(env);
    crate::lua_api::chat_init::init_chat_type_colors(env);
    crate::lua_api::chat_init::show_chat_frame(env);
}

fn patch_post_event_frame_layout(env: &crate::lua_api::WowLuaEnv) {
    temporary::post_event_frame_layout::patch(env);
}

fn refresh_post_event_surfaces(env: &crate::lua_api::WowLuaEnv) {
    refresh_character_frame_surface(env);
    patch_chat_voice_button_surface(env);
    patch_objective_tracker_quest_header(env);
}

pub fn apply_for_runtime_addon_load(env: &crate::lua_api::LoaderEnv<'_>, addon_name: &str) {
    patch_runtime_core_addon_surfaces(env, addon_name);
    patch_runtime_journal_addon_surfaces(env, addon_name);
    patch_runtime_feature_addon_surfaces(env, addon_name);
}

fn patch_runtime_core_addon_surfaces(env: &crate::lua_api::LoaderEnv<'_>, addon_name: &str) {
    if matches!(
        addon_name,
        "Blizzard_ChatFrame"
            | "Blizzard_QuickJoin"
            | "Blizzard_Channels"
            | "Blizzard_VoiceToggleButton"
    ) {
        temporary::chat_voice_button_surface::patch_loader(env);
    }
    if addon_name == "Blizzard_PagedContent" {
        temporary::paging_controls_page_text::patch_loader(env);
    }
    if matches!(
        addon_name,
        "Blizzard_SharedTalentUI" | "Blizzard_PlayerSpells"
    ) {
        temporary::talent_edge_frame_level_sync::patch_loader(env);
    }
    patch_runtime_map_addon_surfaces(env, addon_name);
}

fn patch_runtime_journal_addon_surfaces(env: &crate::lua_api::LoaderEnv<'_>, addon_name: &str) {
    if addon_name == "Blizzard_Collections" {
        patch_toggle_collections_journal_for_runtime_addon_load(env);
        temporary::collections_journal_namespace::patch(env);
    }
    if addon_name == "Blizzard_EncounterJournal" {
        patch_toggle_encounter_journal_for_runtime_addon_load(env);
    }
    if addon_name == "Blizzard_AdventureMap" {
        ensure_adventure_map_frame_surface_for_runtime_addon_load(env);
    }
}

fn patch_runtime_feature_addon_surfaces(env: &crate::lua_api::LoaderEnv<'_>, addon_name: &str) {
    if matches!(addon_name, "Blizzard_ArtifactUI" | "Blizzard_Colors") {
        patch_item_quality_color_data_methods(env);
    }
    if addon_name == "Blizzard_ArtifactUI" {
        patch_artifact_ui_show_panel_guard(env);
    }
    if addon_name == "Blizzard_AuctionHouseUI" {
        patch_auction_house_runtime_surface(env);
    }
    if addon_name == "Blizzard_AuthChallengeUI" {
        patch_auth_challenge_frame_parent(env);
    }
    if addon_name == "Blizzard_AccountStore" {
        let _ = patch_account_store_set_storefront(env);
    }
    if addon_name == "Blizzard_CatalogShop" {
        temporary::catalog_shop_product_card_defaults::patch_for_runtime_addon_load(env);
    }
    if addon_name == "Blizzard_DamageMeter" {
        patch_damage_meter_initial_scrollbox_extent(env);
    }
}

fn patch_auction_house_runtime_surface(env: &crate::lua_api::LoaderEnv<'_>) {
    patch_auction_house_categories_refresh_count(env);
    patch_auction_house_browse_results_event(env);
    patch_auction_house_search_context_aliases(env);
}

fn patch_runtime_map_addon_surfaces(env: &crate::lua_api::LoaderEnv<'_>, addon_name: &str) {
    if addon_name == "Blizzard_MapCanvas" {
        let _ = temporary::map_canvas_scroll_container::patch(env);
    }
    if matches!(
        addon_name,
        "Blizzard_MapCanvas"
            | "Blizzard_SharedMapDataProviders"
            | "Blizzard_WorldMap"
            | "Blizzard_BattlefieldMap"
    ) {
        patch_fog_of_war_pin_mixin_for_runtime_addon_load(env);
        patch_map_exploration_pin_mixin_for_runtime_addon_load(env);
        patch_map_canvas_data_provider_attachment_for_runtime_addon_load(env);
    }
}

pub fn apply_for_runtime_addon_preload(env: &crate::lua_api::LoaderEnv<'_>, addon_name: &str) {
    if addon_name == "Blizzard_Collections" {
        temporary::collections_journal_namespace::patch(env);
    }
    if matches!(
        addon_name,
        "Blizzard_HousingDashboard" | "Blizzard_HousingHouseFinder"
    ) {
        patch_housing_dashboard_preload(env);
    }
}

fn log_with_timestamp(env: &crate::lua_api::WowLuaEnv, message: &str) {
    let start_time = env.state().borrow().start_time;
    eprintln!("{} {}", crate::logging::elapsed_prefix(start_time), message);
}

fn log_step(env: &crate::lua_api::WowLuaEnv, label: &str, apply_step: impl FnOnce()) {
    log_with_timestamp(env, &format!("[Workarounds] starting {label}"));
    let started = Instant::now();
    apply_step();
    log_with_timestamp(
        env,
        &format!(
            "[Workarounds] finished {label} in {:.2?}",
            started.elapsed()
        ),
    );
}
