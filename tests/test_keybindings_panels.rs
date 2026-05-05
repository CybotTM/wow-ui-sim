//! Integration tests for keybinding dispatch — panel interaction tests.
//!
//! Covers spellbook, talents, collections, world map, escape menu, and social panels.

mod common;

use std::path::PathBuf;
use wow_ui_sim::iced_app::{
    RegistryQuadBatchParams, build_quad_batch_for_registry, compute_frame_rect,
};
use wow_ui_sim::loader::load_addon;
use wow_ui_sim::lua_api::WowLuaEnv;
use wow_ui_sim::render::{QuadBatch, QuadVertex, TextureRequest};
use wow_ui_sim::widget::WidgetRegistry;

fn blizzard_ui_dir() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("Interface/BlizzardUI")
}

/// Blizzard addons in dependency order (same as micro_menu.rs).
const BLIZZARD_ADDONS: &[(&str, &str)] = &[
    ("Blizzard_SharedXMLBase", "Blizzard_SharedXMLBase.toc"),
    ("Blizzard_Colors", "Blizzard_Colors_Mainline.toc"),
    ("Blizzard_SharedXML", "Blizzard_SharedXML_Mainline.toc"),
    ("Blizzard_SharedXMLGame", "Blizzard_SharedXMLGame.toc"),
    (
        "Blizzard_UIPanelTemplates",
        "Blizzard_UIPanelTemplates_Mainline.toc",
    ),
    (
        "Blizzard_FrameXMLBase",
        "Blizzard_FrameXMLBase_Mainline.toc",
    ),
    ("Blizzard_FrameEffects", "Blizzard_FrameEffects.toc"),
    ("Blizzard_LoadLocale", "Blizzard_LoadLocale.toc"),
    ("Blizzard_Fonts_Shared", "Blizzard_Fonts_Shared.toc"),
    ("Blizzard_HelpPlate", "Blizzard_HelpPlate.toc"),
    (
        "Blizzard_AccessibilityTemplates",
        "Blizzard_AccessibilityTemplates.toc",
    ),
    ("Blizzard_ObjectAPI", "Blizzard_ObjectAPI_Mainline.toc"),
    ("Blizzard_UIParent", "Blizzard_UIParent_Mainline.toc"),
    ("Blizzard_TextStatusBar", "Blizzard_TextStatusBar.toc"),
    ("Blizzard_MoneyFrame", "Blizzard_MoneyFrame_Mainline.toc"),
    ("Blizzard_POIButton", "Blizzard_POIButton.toc"),
    ("Blizzard_Flyout", "Blizzard_Flyout.toc"),
    ("Blizzard_StoreUI", "Blizzard_StoreUI_Mainline.toc"),
    ("Blizzard_MicroMenu", "Blizzard_MicroMenu_Mainline.toc"),
    ("Blizzard_EditMode", "Blizzard_EditMode.toc"),
    ("Blizzard_GarrisonBase", "Blizzard_GarrisonBase.toc"),
    ("Blizzard_GameTooltip", "Blizzard_GameTooltip_Mainline.toc"),
    (
        "Blizzard_UIParentPanelManager",
        "Blizzard_UIParentPanelManager_Mainline.toc",
    ),
    (
        "Blizzard_Settings_Shared",
        "Blizzard_Settings_Shared_Mainline.toc",
    ),
    (
        "Blizzard_SettingsDefinitions_Shared",
        "Blizzard_SettingsDefinitions_Shared.toc",
    ),
    (
        "Blizzard_SettingsDefinitions_Frame",
        "Blizzard_SettingsDefinitions_Frame_Mainline.toc",
    ),
    ("Blizzard_FrameXMLUtil", "Blizzard_FrameXMLUtil.toc"),
    ("Blizzard_ItemButton", "Blizzard_ItemButton_Mainline.toc"),
    ("Blizzard_QuickKeybind", "Blizzard_QuickKeybind.toc"),
    ("Blizzard_FrameXML", "Blizzard_FrameXML_Mainline.toc"),
    (
        "Blizzard_UIPanels_Game",
        "Blizzard_UIPanels_Game_Mainline.toc",
    ),
    (
        "Blizzard_MapCanvasSecureUtil",
        "Blizzard_MapCanvasSecureUtil.toc",
    ),
    ("Blizzard_MapCanvas", "Blizzard_MapCanvas.toc"),
    (
        "Blizzard_SharedMapDataProviders",
        "Blizzard_SharedMapDataProviders_Mainline.toc",
    ),
    ("Blizzard_WorldMap", "Blizzard_WorldMap_Mainline.toc"),
    ("Blizzard_ActionBar", "Blizzard_ActionBar_Mainline.toc"),
    (
        "Blizzard_ActionBarController",
        "Blizzard_ActionBarController.toc",
    ),
    ("Blizzard_GameMenu", "Blizzard_GameMenu_Mainline.toc"),
    ("Blizzard_UIWidgets", "Blizzard_UIWidgets_Mainline.toc"),
    ("Blizzard_Minimap", "Blizzard_Minimap_Mainline.toc"),
    ("Blizzard_AddOnList", "Blizzard_AddOnList.toc"),
    ("Blizzard_TimerunningUtil", "Blizzard_TimerunningUtil.toc"),
    ("Blizzard_Communities", "Blizzard_Communities_Mainline.toc"),
];

fn setup_env() -> common::LockedEnv {
    common::lock_env(|| {
        let env = WowLuaEnv::new().expect("Failed to create Lua environment");
        env.set_screen_size(1024.0, 768.0);

        {
            let mut state = env.state().borrow_mut();
            state.addon_base_paths = vec![blizzard_ui_dir()];
        }

        let ui = blizzard_ui_dir();
        for (name, toc) in BLIZZARD_ADDONS {
            let toc_path = ui.join(name).join(toc);
            if !toc_path.exists() {
                continue;
            }
            if let Err(e) = load_addon(&env.loader_env(), &toc_path) {
                eprintln!("[load {name}] FAILED: {e}");
            } else {
                env.apply_runtime_addon_load_workarounds(name);
            }
        }

        load_token_ui(&env);
        env.apply_post_load_workarounds();
        fire_startup_events(&env);
        env
    })
}

fn fire_startup_events(env: &WowLuaEnv) {
    common::fire_addon_loaded(env, "WoWUISim");
    for event in ["VARIABLES_LOADED", "PLAYER_LOGIN"] {
        let _ = env.fire_event(event);
    }
    common::call_global_if_present(env, "RequestTimePlayed");
    common::fire_player_entering_world(env, true, false);
    for event in [
        "UPDATE_BINDINGS",
        "DISPLAY_SIZE_CHANGED",
        "UI_SCALE_CHANGED",
        "PLAYER_LEAVING_WORLD",
    ] {
        let _ = env.fire_event(event);
    }
}

fn load_token_ui(env: &WowLuaEnv) {
    env.exec(
        r#"
        local loaded, reason = LoadAddOn("Blizzard_TokenUI")
        assert(loaded, "LoadAddOn(Blizzard_TokenUI) failed: " .. tostring(reason))
        if ContainerFrameSettingsManager and not ContainerFrameSettingsManager.TokenTracker then
            ContainerFrameSettingsManager:OnAddonLoaded("Blizzard_TokenUI")
        end
        assert(BackpackTokenFrame, "BackpackTokenFrame should exist after loading Blizzard_TokenUI")
        assert(
            ContainerFrameSettingsManager and ContainerFrameSettingsManager.TokenTracker == BackpackTokenFrame,
            "ContainerFrameSettingsManager should own BackpackTokenFrame after loading Blizzard_TokenUI"
        )
        "#,
    )
    .expect("Failed to runtime-load Blizzard_TokenUI for keybinding bag tests");
}

fn frame_is_shown(env: &WowLuaEnv, frame_name: &str) -> bool {
    let code = format!("return {frame_name} ~= nil and {frame_name}:IsShown() == true");
    env.eval::<bool>(&code).unwrap_or(false)
}

fn frame_is_visible(env: &WowLuaEnv, frame_name: &str) -> bool {
    let code = format!("return {frame_name} ~= nil and {frame_name}:IsVisible() == true");
    env.eval::<bool>(&code).unwrap_or(false)
}

fn build_batch_for_root(env: &WowLuaEnv, root_name: &str) -> wow_ui_sim::render::QuadBatch {
    {
        let mut state = env.state().borrow_mut();
        state.ensure_layout_rects();
    }
    let buckets = {
        let mut state = env.state().borrow_mut();
        let _ = state.get_strata_buckets();
        state.strata_buckets.as_ref().unwrap().clone()
    };
    let state = env.state().borrow();
    build_quad_batch_for_registry(
        RegistryQuadBatchParams::new(&state.widgets, (1024.0, 768.0), &buckets)
            .root_name(Some(root_name)),
    )
}

fn quad_bounds_from_vertices(verts: &[QuadVertex]) -> (f32, f32, f32, f32) {
    let mut min_x = f32::INFINITY;
    let mut min_y = f32::INFINITY;
    let mut max_x = f32::NEG_INFINITY;
    let mut max_y = f32::NEG_INFINITY;
    for vert in verts {
        min_x = min_x.min(vert.position[0]);
        min_y = min_y.min(vert.position[1]);
        max_x = max_x.max(vert.position[0]);
        max_y = max_y.max(vert.position[1]);
    }
    (min_x, min_y, max_x, max_y)
}

fn quad_bounds(batch: &QuadBatch, request: &TextureRequest) -> (f32, f32, f32, f32) {
    let start = request.vertex_start as usize;
    let end = start + request.vertex_count as usize;
    quad_bounds_from_vertices(&batch.vertices[start..end])
}

fn bounds_match_rect(bounds: (f32, f32, f32, f32), rect: wow_ui_sim::LayoutRect) -> bool {
    let tolerance = 0.1;
    (bounds.0 - rect.x).abs() <= tolerance
        && (bounds.1 - rect.y).abs() <= tolerance
        && (bounds.2 - (rect.x + rect.width)).abs() <= tolerance
        && (bounds.3 - (rect.y + rect.height)).abs() <= tolerance
}

fn spellbook_paged_spells_frame_id(registry: &WidgetRegistry) -> Option<u64> {
    let player_spells_id = registry.get_id_by_name("PlayerSpellsFrame")?;
    let player_spells = registry.get(player_spells_id)?;
    let spellbook_id = *player_spells.children_keys.get("SpellBookFrame")?;
    let spellbook = registry.get(spellbook_id)?;
    spellbook.children_keys.get("PagedSpellsFrame").copied()
}

fn find_first_visible_spellbook_icon_id(registry: &WidgetRegistry) -> Option<u64> {
    let paged_id = spellbook_paged_spells_frame_id(registry)?;
    let paged = registry.get(paged_id)?;

    paged
        .children
        .iter()
        .copied()
        .find_map(|view_frame_id| find_visible_spellbook_icon_in_view(registry, view_frame_id))
}

fn find_visible_spellbook_icon_in_view(
    registry: &WidgetRegistry,
    view_frame_id: u64,
) -> Option<u64> {
    let view_frame = registry.get(view_frame_id)?;
    if !view_frame.visible {
        return None;
    }

    view_frame
        .children
        .iter()
        .copied()
        .find_map(|item_id| find_visible_spellbook_icon_in_item(registry, item_id))
}

fn find_visible_spellbook_icon_in_item(registry: &WidgetRegistry, item_id: u64) -> Option<u64> {
    let item = registry.get(item_id)?;
    if !item.visible || item.width <= 0.0 || item.height <= 0.0 {
        return None;
    }

    let button_id = *item.children_keys.get("Button")?;
    let button = registry.get(button_id)?;
    let icon_id = *button.children_keys.get("Icon")?;
    let icon = registry.get(icon_id)?;
    if !icon.visible || icon.width <= 0.0 || icon.height <= 0.0 {
        return None;
    }
    if icon.texture.is_some() || icon.texture_file_data_id.is_some() {
        return Some(icon_id);
    }

    None
}

// ── S → PlayerSpellsUtil.ToggleSpellBookFrame() ─────────────────────────

#[test]
fn keybind_s_opens_spellbook() {
    test_timeout! {
        let env = setup_env();
        env.send_key_press("S", None).expect("S keybind failed");
        assert!(
            frame_is_shown(&env, "PlayerSpellsFrame"),
            "PlayerSpellsFrame should be shown after pressing S"
        );
        // ShowUIPanel should scale-to-fit; PlayerSpellsFrame keeps its default strata.
        let scale: f64 = env
            .eval("return PlayerSpellsFrame:GetScale()")
            .expect("GetScale failed");
        assert!(
            scale < 1.0,
            "1618px-wide frame at 1024px screen should be scaled down, got {scale}"
        );
        let strata: String = env
            .eval("return PlayerSpellsFrame:GetFrameStrata()")
            .expect("GetFrameStrata failed");
        assert_eq!(
            strata, "MEDIUM",
            "PlayerSpellsFrame should keep its default MEDIUM strata"
        );
    }
}

#[test]
fn keybind_s_renders_spellbook_item_icon_quads_on_first_open() {
    test_timeout! {
        let env = setup_env();
        env.send_key_press("S", None).expect("S keybind failed");

        let batch = build_batch_for_root(&env, "PlayerSpellsFrame");
        let state = env.state().borrow();
        let registry = &state.widgets;
        let icon_id = find_first_visible_spellbook_icon_id(registry)
            .expect("Spellbook first-open path should expose a visible spell icon frame");
        let icon_rect = compute_frame_rect(registry, icon_id, 1024.0, 768.0);

        let icon_request = batch
            .texture_requests
            .iter()
            .find(|request| bounds_match_rect(quad_bounds(&batch, request), icon_rect))
            .unwrap_or_else(|| {
                panic!(
                    "Spellbook first-open path should emit a textured quad for the first visible spell icon at rect ({}, {}, {}x{}); batch had {} texture requests",
                    icon_rect.x,
                    icon_rect.y,
                    icon_rect.width,
                    icon_rect.height,
                    batch.texture_requests.len()
                )
            });

        assert!(
            icon_request.path.to_ascii_lowercase().contains("icons"),
            "First visible spellbook icon quad should come from an icon texture, got {}",
            icon_request.path
        );
    }
}

#[test]
fn keybind_s_keeps_spellbook_backgrounds_below_icons_after_initial_bucket_build() {
    test_timeout! {
        let env = setup_env();
        {
            let mut state = env.state().borrow_mut();
            let _ = state.get_strata_buckets();
        }

        env.send_key_press("S", None).expect("S keybind failed");

        let batch = build_batch_for_root(&env, "PlayerSpellsFrame");
        let state = env.state().borrow();
        let registry = &state.widgets;
        let icon_id = find_first_visible_spellbook_icon_id(registry)
            .expect("Spellbook first-open path should expose a visible spell icon frame");
        let icon_rect = compute_frame_rect(registry, icon_id, 1024.0, 768.0);

        let icon_request = batch
            .texture_requests
            .iter()
            .find(|request| bounds_match_rect(quad_bounds(&batch, request), icon_rect))
            .expect("Spellbook first-open path should emit a textured quad for the first visible spell icon");

        let last_background_request = batch
            .texture_requests
            .iter()
            .filter(|request| {
                request
                    .path
                    .to_ascii_lowercase()
                    .contains("spellbookbackgroundevergreen")
            })
            .max_by_key(|request| request.vertex_start)
            .expect("Spellbook should emit evergreen parchment background quads");

        assert!(
            last_background_request.vertex_start < icon_request.vertex_start,
            "Spellbook parchment backgrounds must render before spell icons after LoD open with prebuilt strata buckets; last background vertex_start={} icon vertex_start={}",
            last_background_request.vertex_start,
            icon_request.vertex_start
        );
    }
}

// ── N → PlayerSpellsUtil.ToggleClassTalentFrame() ───────────────────────

#[test]
fn keybind_n_opens_talents() {
    test_timeout! {
        let env = setup_env();
        env.send_key_press("N", None).expect("N keybind failed");
        assert!(
            frame_is_shown(&env, "PlayerSpellsFrame"),
            "PlayerSpellsFrame should be shown after pressing N (talents tab)"
        );
        assert!(
            !frame_is_shown(&env, "ClassTalentLoadoutImportDialog"),
            "ClassTalentLoadoutImportDialog should stay hidden until the Import action is clicked"
        );
        assert!(
            !frame_is_shown(&env, "ClassTalentLoadoutCreateDialog"),
            "ClassTalentLoadoutCreateDialog should stay hidden until the New Loadout action is clicked"
        );
        assert!(
            !frame_is_visible(&env, "ClassTalentLoadoutImportDialogImportControl"),
            "Import dialog content should not become visible when opening the talents tab"
        );
        assert!(
            !frame_is_visible(&env, "ClassTalentLoadoutImportDialogNameControl"),
            "Import dialog name control should not become visible when opening the talents tab"
        );
        assert!(
            !frame_is_visible(&env, "ClassTalentLoadoutCreateDialogNameControl"),
            "Create dialog content should not become visible when opening the talents tab"
        );
    }
}

#[test]
fn hidden_talent_dialogs_do_not_emit_quads_after_opening_talents() {
    test_timeout! {
        let env = setup_env();
        wow_ui_sim::startup::settle_headless_startup(&env);
        env.send_key_press("N", None).expect("N keybind failed");
        wow_ui_sim::startup::run_extra_update_ticks(&env, 3);

        let import_batch = build_batch_for_root(&env, "ClassTalentLoadoutImportDialog");
        assert_eq!(
            import_batch.quad_count(),
            12,
            "hidden import dialog should not emit quads"
        );
        assert_eq!(
            import_batch.texture_requests.len(),
            1,
            "hidden import dialog should only contribute the tiled background"
        );

        let hero_batch = build_batch_for_root(&env, "HeroTalentsSelectionDialog");
        assert_eq!(
            hero_batch.quad_count(),
            12,
            "hidden hero talents dialog should not emit quads"
        );
        assert_eq!(
            hero_batch.texture_requests.len(),
            1,
            "hidden hero talents dialog should only contribute the tiled background"
        );
    }
}

// ── A → ToggleAchievementFrame() ────────────────────────────────────────

#[test]
fn keybind_a_opens_achievements() {
    test_timeout! {
        let env = setup_env();
        env.send_key_press("A", None).expect("A keybind failed");
        assert!(
            frame_is_shown(&env, "AchievementFrame"),
            "AchievementFrame should be shown after pressing A"
        );
    }
}

// ── L → PVEFrame_ToggleFrame() ──────────────────────────────────────────

#[test]
fn keybind_l_opens_group_finder() {
    test_timeout! {
        let env = setup_env();
        env.exec(
            r#"
            assert(LoadAddOn("Blizzard_GroupFinder"))
            _G.__group_finder_triggered = false
            local original_toggle_group_finder = PVEFrame_ToggleFrame
            PVEFrame_ToggleFrame = function(...)
                _G.__group_finder_triggered = true
                return original_toggle_group_finder(...)
            end
            "#
        ).expect("Failed to wrap PVEFrame_ToggleFrame for L keybind test");
        env.send_key_press("L", None).expect("L keybind failed");
        let triggered: bool = env
            .eval("return _G.__group_finder_triggered == true")
            .expect("Failed to read L keybind trigger flag");
        assert!(
            triggered,
            "Pressing L should dispatch PVEFrame_ToggleFrame"
        );
        let premade_enabled: bool = env
            .eval(
                r#"
                return GroupFinderFrame ~= nil
                    and GroupFinderFrame.groupButton3 ~= nil
                    and GroupFinderFrame.groupButton3:IsEnabled() == true
                "#,
            )
            .expect("Failed to read Premade Groups button enabled state");
        assert!(
            premade_enabled,
            "Premade Groups should be enabled after opening Group Finder"
        );
    }
}

#[test]
fn raid_finder_group_button_path_handles_empty_rf_dungeon_list() {
    test_timeout! {
        let env = setup_env();
        env.exec(
            r#"
            assert(LoadAddOn("Blizzard_GroupFinder"))
            GroupFinderFrame_ShowGroupFrame(RaidFinderFrame)
            assert(RaidFinderFrame:IsShown(), "RaidFinderFrame should be shown")
            "#
        ).expect("Raid Finder group button path should not raise Lua errors");
    }
}

#[test]
fn premade_group_category_buttons_can_be_clicked() {
    test_timeout! {
        let env = setup_env();
        env.exec(
            r#"
            assert(LoadAddOn("Blizzard_GroupFinder"))
            LFGListPVEStub_OnShow(LFGListPVEStub)
            LFGListFrame_SetActivePanel(LFGListFrame, LFGListFrame.CategorySelection)
            assert(LFGListFrame.CategorySelection:IsShown(), "category selection should be shown")

            local clicked = 0
            for _, button in ipairs(LFGListFrame.CategorySelection.CategoryButtons) do
                if button:IsShown() then
                    button:GetScript("OnClick")(button)
                    assert(
                        LFGListFrame.CategorySelection.selectedCategory == button.categoryID,
                        "click should select category " .. tostring(button.categoryID)
                    )
                    clicked = clicked + 1
                end
            end

            assert(clicked > 0, "expected at least one visible premade category button")
            "#
        ).expect("Premade Group category buttons should click without Lua errors");
    }
}

#[test]
fn premade_group_categories_can_start_search() {
    test_timeout! {
        let env = setup_env();
        env.exec(
            r#"
            assert(LoadAddOn("Blizzard_GroupFinder"))
            LFGListPVEStub_OnShow(LFGListPVEStub)
            LFGListFrame_SetActivePanel(LFGListFrame, LFGListFrame.CategorySelection)

            for _, button in ipairs(LFGListFrame.CategorySelection.CategoryButtons) do
                if button:IsShown() then
                    LFGListFrame_SetActivePanel(LFGListFrame, LFGListFrame.CategorySelection)
                    button:GetScript("OnClick")(button)
                    LFGListFrame.CategorySelection.FindGroupButton:GetScript("OnClick")(
                        LFGListFrame.CategorySelection.FindGroupButton
                    )
                    assert(LFGListFrame.SearchPanel:IsShown(), "search panel should be shown")
                end
            end
            "#
        ).expect("Premade Group categories should start search without Lua errors");
    }
}

#[test]
fn premade_group_search_result_tooltips_do_not_error() {
    test_timeout! {
        let env = setup_env();
        env.exec(
            r#"
            assert(LoadAddOn("Blizzard_GroupFinder"))
            local _, results = C_LFGList.GetSearchResults()
            assert(#results > 0, "expected seeded premade search results")

            for _, resultID in ipairs(results) do
                LFGListUtil_SetSearchEntryTooltip(GameTooltip, resultID)
            end
            "#
        ).expect("Premade Group search result tooltips should not raise Lua errors");
    }
}

// ── O → ToggleFriendsFrame() ────────────────────────────────────────────

#[test]
fn keybind_o_opens_social() {
    test_timeout! {
        let env = setup_env();
        env.send_key_press("O", None).expect("O keybind failed");
        assert!(
            frame_is_shown(&env, "FriendsFrame"),
            "FriendsFrame should be shown after pressing O"
        );
    }
}

#[test]
fn keybind_o_dispatches_toggle_friends_frame() {
    test_timeout! {
        let env = setup_env();
        env.exec(
            r#"
            assert(LoadAddOn("Blizzard_FriendsFrame"))
            _G.__friends_keybind_triggered = false
            local original_toggle_friends_frame = ToggleFriendsFrame
            ToggleFriendsFrame = function(...)
                _G.__friends_keybind_triggered = true
                return original_toggle_friends_frame(...)
            end
            "#
        ).expect("Failed to wrap ToggleFriendsFrame for O keybind test");
        env.send_key_press("O", None).expect("O keybind failed");
        let triggered: bool = env
            .eval("return _G.__friends_keybind_triggered == true")
            .expect("Failed to read O keybind trigger flag");
        assert!(
            triggered,
            "Pressing O should dispatch ToggleFriendsFrame"
        );
    }
}

// ── J → ToggleGuildFrame() ──────────────────────────────────────────────

#[test]
fn keybind_j_opens_guild() {
    test_timeout! {
        let env = setup_env();
        env.exec(
            r#"
            _G.__guild_keybind_triggered = false
            local original_toggle_guild_frame = ToggleGuildFrame
            ToggleGuildFrame = function(...)
                _G.__guild_keybind_triggered = true
                return original_toggle_guild_frame(...)
            end
            "#
        ).expect("Failed to wrap ToggleGuildFrame for J keybind test");
        env.send_key_press("J", None).expect("J keybind failed");
        let triggered: bool = env
            .eval("return _G.__guild_keybind_triggered == true")
            .expect("Failed to read J keybind trigger flag");
        assert!(
            triggered,
            "Pressing J should dispatch ToggleGuildFrame"
        );
    }
}
