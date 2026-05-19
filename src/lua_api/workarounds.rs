//! Post-load workarounds that are still required on the live rilua path.

mod character_lua;
mod early_lua;
mod map_canvas;
mod map_lua;
mod panel_lua;
mod post_event_lua;
mod runtime_surfaces;
mod temporary;

use character_lua::*;
use early_lua::*;
use map_canvas::*;
use map_lua::*;
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

pub fn close_startup_special_windows_before_first_frame(env: &crate::lua_api::WowLuaEnv) {
    temporary::startup_windows::close_before_first_frame(env);
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
    let _ = env.exec(SETTINGS_CANVAS_LAYOUT_HIDE_LUA);
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
    let _ = env.exec(POST_EVENT_FRAME_LAYOUT_WORKAROUND_LUA);
}

fn refresh_post_event_surfaces(env: &crate::lua_api::WowLuaEnv) {
    refresh_character_frame_surface(env);
    patch_chat_voice_button_surface(env);
    patch_objective_tracker_quest_header(env);
}

pub fn apply_for_runtime_addon_load(env: &crate::lua_api::LoaderEnv<'_>, addon_name: &str) {
    if addon_name == "Blizzard_PagedContent" {
        let _ = env.exec(PAGING_CONTROLS_PAGE_TEXT_WORKAROUND_LUA);
    }
    if matches!(
        addon_name,
        "Blizzard_SharedTalentUI" | "Blizzard_PlayerSpells"
    ) {
        let _ = env.exec(TALENT_EDGE_FRAME_LEVEL_SYNC_WORKAROUND_LUA);
    }
    patch_runtime_map_addon_surfaces(env, addon_name);
    if addon_name == "Blizzard_Collections" {
        patch_toggle_collections_journal_for_runtime_addon_load(env);
        patch_collections_journal_namespace(env);
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
        let _ = env.exec(CATALOG_SHOP_PRODUCT_CARD_DEFAULTS_WORKAROUND_LUA);
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
        patch_collections_journal_namespace(env);
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

#[cfg(test)]
mod tests {
    use super::SETTINGS_CANVAS_LAYOUT_HIDE_LUA;
    use crate::lua_api::WowLuaEnv;

    #[test]
    fn settings_canvas_registration_hides_frame_until_displayed() {
        let env = WowLuaEnv::new().expect("env should initialize");
        env.exec(
            r#"
            SettingsLayoutMixin = { LayoutType = { Canvas = "Canvas" } }

            local categories = {}
            local layouts = {}

            SettingsPanel = {
                shown = false,
                currentLayout = nil,
                currentCategory = nil,
                GetAllCategories = function()
                    return categories
                end,
                GetLayout = function(_, category)
                    return layouts[category]
                end,
                IsShown = function(self)
                    return self.shown
                end,
                GetCurrentLayout = function(self)
                    return self.currentLayout
                end,
                GetCurrentCategory = function(self)
                    return self.currentCategory
                end,
            }

            Settings = {
                RegisterCanvasLayoutCategory = function(frame, name)
                    local category = { name = name }
                    local layout = {
                        frame = frame,
                        GetFrame = function(self)
                            return self.frame
                        end,
                        GetLayoutType = function()
                            return SettingsLayoutMixin.LayoutType.Canvas
                        end,
                    }
                    table.insert(categories, category)
                    layouts[category] = layout
                    return category, layout
                end,
                OpenToCategory = function(category)
                    SettingsPanel.shown = true
                    SettingsPanel.currentCategory = category
                    SettingsPanel.currentLayout = layouts[category]
                    return category
                end,
            }
            "#,
        )
        .expect("fake settings surface should install");

        env.exec(SETTINGS_CANVAS_LAYOUT_HIDE_LUA)
            .expect("settings canvas workaround should apply");

        let hidden_after_register: bool = env
            .eval(
                r#"
                local frame = CreateFrame("Frame", "SettingsCanvasLeakProbe")
                frame:Show()
                local category, layout = Settings.RegisterCanvasLayoutCategory(frame, "Probe")
                return not frame:IsShown()
                "#,
            )
            .expect("registration probe should run");

        assert!(
            hidden_after_register,
            "settings canvas frame should be hidden after registration"
        );

        let opened_canvas_visible_others_hidden: bool = env
            .eval(
                r#"
                local first = SettingsCanvasLeakProbe
                local firstCategory = SettingsPanel:GetAllCategories()[1]
                local second = CreateFrame("Frame", "SettingsSecondCanvasLeakProbe")
                second:Show()
                local secondCategory = Settings.RegisterCanvasLayoutCategory(second, "Second")

                Settings.OpenToCategory(firstCategory)
                local firstOpened = first:IsShown() and not second:IsShown()

                Settings.OpenToCategory(secondCategory)
                return firstOpened and (not first:IsShown()) and second:IsShown()
                "#,
            )
            .expect("open category probe should run");

        assert!(
            opened_canvas_visible_others_hidden,
            "opening a settings category should show only that category's canvas"
        );
    }
}
