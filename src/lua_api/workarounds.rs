//! Post-load workarounds that are still required on the live rilua path.

mod character_lua;
mod map_canvas;
mod panel_lua;
mod permanent;
mod post_event_lua;
mod runtime_surfaces;
mod temporary;

pub(crate) use temporary::source_patches::patch_lua_source;

use character_lua::*;
use map_canvas::*;
use panel_lua::*;
use post_event_lua::*;
pub(crate) use runtime_surfaces::patch_account_store_set_storefront;
use runtime_surfaces::*;
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

pub fn close_startup_special_windows_before_first_frame(env: &crate::lua_api::WowLuaEnv) {
    temporary::startup_windows::close_before_first_frame(env);
}

pub(crate) fn sanitize_imported_wtf_addon_saved_variables(
    state: &mut rilua::vm::state::LuaState,
    addon_name: &str,
) {
    temporary::details_saved_variables::sanitize_imported_wtf_addon(state, addon_name);
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
    let _ = env.exec(REFRESH_ACTION_BUTTONS_LUA);
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
        patch_map_canvas_scroll_container(env);
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
