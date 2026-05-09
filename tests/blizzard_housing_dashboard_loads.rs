#![cfg(feature = "client-retail")]
use std::path::PathBuf;

use wow_ui_sim::loader::{discover_blizzard_addons_for_screen, find_toc_file, load_addon};
use wow_ui_sim::lua_api::WowLuaEnv;
use wow_ui_sim::screen::ScreenKind;
use wow_ui_sim::startup::fire_startup_events_for_screen;
use wow_ui_sim::toc::TocFile;

fn blizzard_ui_dir() -> PathBuf {
    wow_ui_sim::client_profile::blizzard_ui_addons_dir_under(std::path::Path::new(env!(
        "CARGO_MANIFEST_DIR"
    )))
}

fn dashboard_dir() -> PathBuf {
    blizzard_ui_dir().join("Blizzard_HousingDashboard")
}

fn dashboard_toc() -> PathBuf {
    dashboard_dir().join("Blizzard_HousingDashboard.toc")
}

fn parse_dashboard_toc() -> TocFile {
    TocFile::from_file(&dashboard_toc()).expect("HousingDashboard TOC should parse")
}

fn load_full_game_ui_with_dashboard_lod() -> WowLuaEnv {
    let env = WowLuaEnv::new().expect("Failed to create Lua environment");
    env.set_screen_size(1024.0, 768.0);
    env.set_screen_mode(ScreenKind::Game);

    {
        let mut state = env.state().borrow_mut();
        state.addon_base_paths = vec![blizzard_ui_dir()];
    }

    wow_ui_sim::xml::register_intrinsic_templates();

    let ui = blizzard_ui_dir();
    let addons = discover_blizzard_addons_for_screen(&ui, ScreenKind::Game);
    for (name, toc_path) in &addons {
        load_addon(&env.loader_env(), toc_path)
            .unwrap_or_else(|err| panic!("[load {name}] FAILED: {err}"));
    }

    env.apply_post_load_workarounds();
    fire_startup_events_for_screen(&env, ScreenKind::Game);

    load_addon(&env.loader_env(), &dashboard_toc())
        .expect("Blizzard_HousingDashboard should load via explicit Rust loader call");

    env
}

fn assert_mixin_methods(env: &WowLuaEnv, mixin: &str, methods: &[&str], rationale: &str) {
    for method in methods {
        let exists: bool = env
            .eval(&format!("return type({mixin}['{method}']) == 'function'"))
            .unwrap_or_else(|err| panic!("{mixin}.{method} existence query failed: {err}"));
        assert!(exists, "{mixin} must expose `:{method}()` — {rationale}");
    }
}

#[test]
fn blizzard_housing_dashboard_find_toc_resolves_bare_variant() {
    let resolved =
        find_toc_file(&dashboard_dir()).expect("Blizzard_HousingDashboard TOC should resolve");
    assert_eq!(
        resolved,
        dashboard_toc(),
        "Blizzard_HousingDashboard ships exactly one bare TOC — retail-only addon resolves via \
         `find_toc_file` fallthrough"
    );
}

#[test]
fn blizzard_housing_dashboard_toc_declares_lod_with_two_dependencies() {
    let toc = parse_dashboard_toc();
    assert!(
        toc.is_load_on_demand(),
        "Blizzard_HousingDashboard declares `## LoadOnDemand: 1` — pulled via explicit LoadAddOn \
         from Blizzard_HousingEventHandler.lua:106 + Blizzard_UIPanels_Game/Mainline/\
         ItemRefHandlers.lua:298 + Blizzard_HousingInspectModeUI.lua:86"
    );
    assert!(!toc.is_load_first());
    assert!(!toc.is_secure_env());
    assert_eq!(
        toc.dependencies(),
        vec![
            "Blizzard_HousingTemplates".to_string(),
            "Blizzard_HousingModelPreview".to_string(),
        ],
        "Two `## Dependencies:` entries: HousingTemplates (atlases + housing utility surface) plus \
         HousingModelPreview (provides HousingModelPreviewTemplate inherited by the Catalog \
         PreviewFrame)"
    );
}

#[test]
fn blizzard_housing_dashboard_toc_is_retail_only_and_omits_allow_load() {
    let toc = parse_dashboard_toc();
    let toc_text = std::fs::read_to_string(dashboard_toc()).expect("TOC should read");
    assert!(
        toc_text.contains("## AllowLoadGameType: standard"),
        "Declares `## AllowLoadGameType: standard` — retail-only Midnight feature"
    );
    assert!(!toc.is_game_type_restricted());
    assert!(
        !toc_text.contains("## AllowLoad:"),
        "Omits `## AllowLoad:` — LoadOnDemand precludes auto-discovery gating"
    );
    assert!(!toc_text.contains("## DefaultState:"));
    assert!(
        toc.saved_variables().is_empty(),
        "No `## SavedVariables*` — house dashboard state is server-driven via house list events \
         and C_HousingNeighborhood / C_Housing API surface"
    );
}

#[test]
fn blizzard_housing_dashboard_toc_lists_nine_files_in_order() {
    let toc = parse_dashboard_toc();
    let files: Vec<String> = toc
        .files
        .iter()
        .map(|p| p.to_string_lossy().replace('\\', "/"))
        .collect();
    assert_eq!(
        files,
        vec![
            "Blizzard_HousingDashboardHouseUpgrade.lua".to_string(),
            "Blizzard_HousingDashboardHouseUpgrade.xml".to_string(),
            "Blizzard_HousingDashboardHouseInfoContent.lua".to_string(),
            "Blizzard_HousingDashboardHouseInfoContent.xml".to_string(),
            "Blizzard_HousingDashboardCatalog.lua".to_string(),
            "Blizzard_HousingDashboardCatalog.xml".to_string(),
            "Blizzard_HousingDashboard.lua".to_string(),
            "Blizzard_HousingDashboard.xml".to_string(),
            "Blizzard_HousingDashboardRegistration.lua".to_string(),
        ],
        "TOC body lists exactly 9 source files in this order — three pairs of feature .lua/.xml \
         (HouseUpgrade, HouseInfoContent, Catalog) declare the per-tab template mixins first, \
         then the umbrella .lua/.xml that wires the dashboard frame, then Registration.lua last"
    );
}

#[test]
fn blizzard_housing_dashboard_directory_holds_ten_entries() {
    let dir = dashboard_dir();
    let entries: Vec<String> = std::fs::read_dir(&dir)
        .expect("Blizzard_HousingDashboard directory should exist")
        .filter_map(|e| e.ok())
        .map(|e| e.file_name().to_string_lossy().into_owned())
        .collect();
    assert_eq!(
        entries.len(),
        10,
        "Directory ships exactly 10 entries (9 source files + 1 TOC). Got: {entries:?}"
    );
    assert!(entries.contains(&"Blizzard_HousingDashboard.toc".to_string()));
    assert!(entries.contains(&"Blizzard_HousingDashboardRegistration.lua".to_string()));
}

#[test]
fn blizzard_housing_dashboard_excluded_from_all_screen_auto_discovery_passes() {
    let ui = blizzard_ui_dir();
    for screen in [
        ScreenKind::Game,
        ScreenKind::Login,
        ScreenKind::CharacterSelect,
        ScreenKind::CharacterCreate,
    ] {
        let addons = discover_blizzard_addons_for_screen(&ui, screen);
        let discovered = addons
            .iter()
            .any(|(name, _)| name == "Blizzard_HousingDashboard");
        assert!(
            !discovered,
            "Must NOT appear in {screen:?} auto-discovery — `## LoadOnDemand: 1` keeps it out of \
             every screen pass; consumers pull via explicit LoadAddOn"
        );
    }
}

#[test]
fn blizzard_housing_dashboard_loads_without_addon_specific_lua_errors() {
    let env = load_full_game_ui_with_dashboard_lod();

    let lua_errors: Vec<String> = env.state().borrow().lua_errors.clone();
    let related: Vec<&String> = lua_errors
        .iter()
        .filter(|e| {
            e.contains("Blizzard_HousingDashboard/")
                || e.contains("Blizzard_HousingDashboard\\")
                || e.contains("HousingDashboardFrameMixin")
                || e.contains("HousingUpgradeFrameMixin")
                || e.contains("HouseUpgradeLevelFrameMixin")
                || e.contains("HousingTeleportToHouseMixin")
                || e.contains("HouseUpgradeRewardFrameMixin")
                || e.contains("HouseUpgradeCurrentLevelFrameMixin")
                || e.contains("HouseWatchFavorButtonMixin")
                || e.contains("HouseUpgradeProgressBarMixin")
                || e.contains("HouseLevelTrackFrameMixin")
                || e.contains("HousingDashboardHouseInfoMixin")
                || e.contains("HousingDashboardHouseInfoContentFrameMixin")
                || e.contains("HouseFinderButtonMixin")
                || e.contains("HouseXPCapIconMixin")
                || e.contains("InitiativesTabMixin")
                || e.contains("InitiativeTaskButtonMixin")
                || e.contains("ProgressThresholdMixin")
                || e.contains("InitiativeActiveNeighborhoodSwitcherMixin")
                || e.contains("HousingCatalogFrameMixin")
        })
        .collect();
    assert!(
        related.is_empty(),
        "Blizzard_HousingDashboard emitted addon-specific Lua errors during explicit LoD load:\n  \
         {}",
        related
            .iter()
            .map(|s| s.as_str())
            .collect::<Vec<_>>()
            .join("\n  ")
    );
}

#[test]
fn blizzard_housing_dashboard_is_addon_loaded_returns_true_after_explicit_lod_load() {
    let env = load_full_game_ui_with_dashboard_lod();

    let loaded: bool = env
        .eval("return C_AddOns.IsAddOnLoaded('Blizzard_HousingDashboard')")
        .expect("IsAddOnLoaded query should succeed");
    assert!(
        loaded,
        "After explicit LoD load, `IsAddOnLoaded('Blizzard_HousingDashboard')` must return true"
    );
}

#[test]
fn blizzard_housing_dashboard_template_dependency_loads_via_game_screen_pass() {
    let env = load_full_game_ui_with_dashboard_lod();

    let templates_loaded: bool = env
        .eval("return C_AddOns.IsAddOnLoaded('Blizzard_HousingTemplates')")
        .expect("IsAddOnLoaded query should succeed");
    assert!(
        templates_loaded,
        "Blizzard_HousingTemplates must be auto-loaded via Game-screen discovery — first \
         dependency on Dashboard's TOC, must already be present before the explicit LoD load \
         runs. The other declared dep Blizzard_HousingModelPreview is itself LoadOnDemand so it \
         is NOT transitively pulled by the explicit LoD load — consumers must LoadAddOn it \
         separately when they need the model preview frame"
    );
}

#[test]
fn blizzard_housing_dashboard_publishes_single_named_frame_globally() {
    let env = load_full_game_ui_with_dashboard_lod();

    let exists: bool = env
        .eval(
            "local f = _G['HousingDashboardFrame']; return type(f) == 'table' and type(f.GetName) == 'function'",
        )
        .expect("Named frame global lookup should succeed");
    assert!(
        exists,
        "After LoD load, `HousingDashboardFrame` should publish as the sole non-virtual `_G` \
         frame instance — Dashboard XML declares exactly one file-scope named frame; everything \
         else is `virtual=\"true\"` and only materializes through inheritance"
    );
}

#[test]
fn blizzard_housing_dashboard_does_not_publish_virtual_templates() {
    let env = load_full_game_ui_with_dashboard_lod();

    for template_name in [
        "HousingDashboardSideTabTemplate",
        "HousingDashboardHouseInfoTemplate",
        "HousingCatalogFrameTemplate",
        "HousingUpgradeFrameTemplate",
        "HouseUpgradeLevelFrameTemplate",
        "HouseUpgradeRewardFrameLargeTemplate",
        "HouseUpgradeRewardFrameSmallTemplate",
        "ProgressThresholdTemplate",
        "ProgressThresholdLargeTemplate",
        "HousingDashboard_HouseFinderButtonTemplate",
        "HousingDashboard_InitiativeTaskActivityEntryTemplate",
        "HousingDashboard_InitiativeTaskTemplate",
        "HousingDashboard_InitiativeSubtaskTemplate",
    ] {
        let published: bool = env
            .eval(&format!("return _G['{template_name}'] ~= nil"))
            .expect("Template global lookup should succeed");
        assert!(
            !published,
            "{template_name} is `virtual=\"true\"` — must not publish to `_G`; only inheriting \
             frames materialize"
        );
    }
}

#[test]
fn blizzard_housing_dashboard_umbrella_mixin_publishes_nine_methods() {
    let env = load_full_game_ui_with_dashboard_lod();
    assert_mixin_methods(
        &env,
        "HousingDashboardFrameMixin",
        &[
            "OnLoad",
            "OnShow",
            "OnHide",
            "OnTabButtonClicked",
            "OnOpenToCatalogFrame",
            "SetTab",
            "GetPanelExtraWidth",
            "OpenInitiativesFrameToTaskID",
            "UpdateSizeToContent",
        ],
        "umbrella dashboard frame mixin owns the 2-tab system (HouseInfoContent + \
         CatalogContent), the OpenToCatalogFrame EventRegistry callback, and the size-swap-on-\
         no-houses behaviour driven by UpdateSizeToContent",
    );
}

#[test]
fn blizzard_housing_dashboard_house_upgrade_mixin_publishes_fourteen_methods() {
    let env = load_full_game_ui_with_dashboard_lod();
    assert_mixin_methods(
        &env,
        "HousingUpgradeFrameMixin",
        &[
            "OnLoad",
            "OnEvent",
            "AllRewardsLoaded",
            "OnShow",
            "OnHide",
            "OnHouseSelected",
            "SelectHouseLevel",
            "OnHouseListUpdated",
            "SelectLevel",
            "CanUpgrade",
            "RefreshSelectedElement",
            "OnTrackUpdate",
            "SetRewards",
            "CancelLevelEffect",
        ],
        "owns the level-track / reward panel inside HouseInfoContent; reacts to \
         PLAYER_HOUSE_LIST_UPDATED + HOUSE_PROGRESSION_BAR_UPDATED + \
         HOUSE_LEVEL_REWARDS_UPDATED via OnEvent",
    );
}

#[test]
fn blizzard_housing_dashboard_house_upgrade_level_mixin_publishes_three_methods() {
    let env = load_full_game_ui_with_dashboard_lod();
    assert_mixin_methods(
        &env,
        "HouseUpgradeLevelFrameMixin",
        &["SetInfo", "GetLevel", "Refresh"],
        "level-plaque mixin used by the level-track scroll positions; SetInfo stores level \
         info, GetLevel returns the stored level, Refresh swaps Plaque/Pip/Checkmark atlases \
         based on completed-vs-incomplete-vs-selected",
    );
}

#[test]
fn blizzard_housing_dashboard_teleport_mixin_publishes_ten_methods() {
    let env = load_full_game_ui_with_dashboard_lod();
    assert_mixin_methods(
        &env,
        "HousingTeleportToHouseMixin",
        &[
            "OnLoad",
            "OnEvent",
            "OnClick",
            "OnMouseDown",
            "OnMouseUp",
            "SetHouseInfo",
            "UpdateCooldown",
            "UpdateState",
            "OnEnter",
            "OnLeave",
        ],
        "wraps C_Housing.TeleportToHouse + cooldown polling tied to SPELL_UPDATE_COOLDOWN \
         events; OnClick validates state then dispatches the teleport",
    );
}

#[test]
fn blizzard_housing_dashboard_house_upgrade_reward_mixin_publishes_two_methods() {
    let env = load_full_game_ui_with_dashboard_lod();
    assert_mixin_methods(
        &env,
        "HouseUpgradeRewardFrameMixin",
        &["OnEnter", "OnLeave"],
        "reward-tile tooltip-only shared base for both the Large + Small reward templates; \
         OnEnter wires the reward tooltip via GameTooltip:SetItemByID / SetCurrencyByID, \
         OnLeave hides the tooltip",
    );
}

#[test]
fn blizzard_housing_dashboard_house_upgrade_current_level_mixin_publishes_two_methods() {
    let env = load_full_game_ui_with_dashboard_lod();
    assert_mixin_methods(
        &env,
        "HouseUpgradeCurrentLevelFrameMixin",
        &["OnEnter", "OnLeave"],
        "tooltip-only mixin on the current-level chevron pointing at the active level row in \
         the upgrade track",
    );
}

#[test]
fn blizzard_housing_dashboard_house_watch_favor_button_mixin_publishes_four_methods() {
    let env = load_full_game_ui_with_dashboard_lod();
    assert_mixin_methods(
        &env,
        "HouseWatchFavorButtonMixin",
        &["OnShow", "OnClick", "SetHouse", "UpdateState"],
        "toggle that marks which house's favor bar the rest of the UI watches; SetHouse \
         stores the house GUID, OnClick flips the watch flag via C_HousingNeighborhood, \
         UpdateState refreshes the checked atlas, OnShow re-runs UpdateState",
    );
}

#[test]
fn blizzard_housing_dashboard_house_upgrade_progress_bar_mixin_publishes_eight_methods() {
    let env = load_full_game_ui_with_dashboard_lod();
    assert_mixin_methods(
        &env,
        "HouseUpgradeProgressBarMixin",
        &[
            "OnLoad",
            "SetFinishAnimCallback",
            "DoToEdges",
            "UpdateFill",
            "OnHide",
            "StopCurrentAnimation",
            "OnAnimationFinished",
            "SetHouseLevelFavor",
        ],
        "fillbar that animates between favor levels; DoToEdges fans method calls across the \
         bar's left/right edge children, UpdateFill recomputes the visible fill ratio, \
         SetHouseLevelFavor binds the current vs target favor for the next animation segment, \
         OnAnimationFinished + SetFinishAnimCallback drive the multi-segment level-up \
         animation chain",
    );
}

#[test]
fn blizzard_housing_dashboard_house_level_track_mixin_extends_reward_track() {
    let env = load_full_game_ui_with_dashboard_lod();

    let exists: bool = env
        .eval("return type(HouseLevelTrackFrameMixin) == 'table'")
        .expect("HouseLevelTrackFrameMixin lookup should succeed");
    assert!(
        exists,
        "HouseLevelTrackFrameMixin must publish as a global mixin table — declared via \
         `CreateFromMixins(RewardTrackFrameMixin)` so the level-track frame inherits the \
         shared reward-track scrolling behavior plus any HouseLevelTrackFrameMixin-specific \
         overrides"
    );
}

#[test]
fn blizzard_housing_dashboard_house_info_mixin_publishes_nine_methods() {
    let env = load_full_game_ui_with_dashboard_lod();
    assert_mixin_methods(
        &env,
        "HousingDashboardHouseInfoMixin",
        &[
            "OnLoad",
            "OnShow",
            "UpdateNoHousesDashboard",
            "LoadHouses",
            "OnEvent",
            "OnHouseListUpdated",
            "RefreshHouseDropdown",
            "OnHouseFinderButtonClicked",
            "OnTutorialButtonClicked",
        ],
        "the umbrella house-info tab; reacts to PLAYER_HOUSE_LIST_UPDATED via LoadHouses, swaps \
         in DashboardNoHousesFrame when the player owns zero houses (driving \
         Dashboard:UpdateSizeToContent), routes the HouseFinder + tutorial button clicks",
    );
}

#[test]
fn blizzard_housing_dashboard_house_info_content_mixin_publishes_five_methods() {
    let env = load_full_game_ui_with_dashboard_lod();
    assert_mixin_methods(
        &env,
        "HousingDashboardHouseInfoContentFrameMixin",
        &[
            "Initialize",
            "UpdateTabs",
            "SetToDefaultAvailableTab",
            "SetTab",
            "IsTabAvailable",
        ],
        "sub-tab system inside HouseInfoContent (House Upgrade vs Initiatives endeavors), \
         SetToDefaultAvailableTab walks IsTabAvailable to find the first eligible tab",
    );
}

#[test]
fn blizzard_housing_dashboard_house_finder_button_mixin_publishes_one_method() {
    let env = load_full_game_ui_with_dashboard_lod();
    assert_mixin_methods(
        &env,
        "HouseFinderButtonMixin",
        &["OnClick"],
        "single-method mixin used by the no-houses dashboard call-to-action that LoadAddOns \
         Blizzard_HousingHouseFinder then shows that frame",
    );
}

#[test]
fn blizzard_housing_dashboard_house_xp_cap_icon_mixin_publishes_three_methods() {
    let env = load_full_game_ui_with_dashboard_lod();
    assert_mixin_methods(
        &env,
        "HouseXPCapIconMixin",
        &["OnEnter", "OnLeave", "UpdateVisibility"],
        "small icon shown next to the progress bar when the house is XP-capped for the week; \
         UpdateVisibility queries C_HousingNeighborhood.IsHouseAtWeeklyXPCap",
    );
}

#[test]
fn blizzard_housing_dashboard_initiatives_tab_mixin_publishes_seventeen_methods() {
    let env = load_full_game_ui_with_dashboard_lod();
    assert_mixin_methods(
        &env,
        "InitiativesTabMixin",
        &[
            "OnLoad",
            "OnShow",
            "OnHide",
            "OnUpdate",
            "OnHouseListUpdated",
            "RefreshInitiativeTab",
            "RefreshTrackedTasks",
            "SetProgressBarThresholds",
            "RefreshActivityLog",
            "SetupActivityLog",
            "SetupTaskList",
            "RefreshTaskList",
            "SetCurrentPoints",
            "ScrollToInitiativeTaskID",
            "OnHouseSelected",
            "RefreshHouseDropdown",
            "UpdateBackground",
        ],
        "endeavors / weekly tasks tab; the biggest mixin (17 methods) drives the activity log \
         + task list scrollboxes plus the threshold reward animation tied to the progress bar",
    );
}

#[test]
fn blizzard_housing_dashboard_initiative_task_button_mixin_publishes_nine_methods() {
    let env = load_full_game_ui_with_dashboard_lod();
    assert_mixin_methods(
        &env,
        "InitiativeTaskButtonMixin",
        &[
            "OnEnter",
            "OnLeave",
            "ShowTooltip",
            "OnClick",
            "OnClick_Internal",
            "UpdateTracked",
            "Init",
            "SetCollapseState",
            "GetData",
        ],
        "task row button shared by both task and subtask templates (OnClick_Internal \
         centralises the click logic; SetCollapseState toggles the activity-log expand state; \
         GetData returns the bound task info for tooltip rendering)",
    );
}

#[test]
fn blizzard_housing_dashboard_progress_threshold_mixin_publishes_four_methods() {
    let env = load_full_game_ui_with_dashboard_lod();
    assert_mixin_methods(
        &env,
        "ProgressThresholdMixin",
        &["Setup", "ShowTooltip", "OnEnter", "SetCurrentPoints"],
        "milestone marker on the favor progress bar; Setup binds the threshold info, \
         ShowTooltip lists the rewards earned at that threshold, SetCurrentPoints redraws the \
         chevron pip when the parent bar's value advances past the threshold",
    );
}

#[test]
fn blizzard_housing_dashboard_initiative_neighborhood_switcher_mixin_publishes_one_method() {
    let env = load_full_game_ui_with_dashboard_lod();
    assert_mixin_methods(
        &env,
        "InitiativeActiveNeighborhoodSwitcherMixin",
        &["OnClick"],
        "single-method mixin that calls C_HousingNeighborhood.SetActiveNeighborhood when the \
         player has multiple eligible neighborhoods to earn favor in",
    );
}

#[test]
fn blizzard_housing_dashboard_catalog_frame_mixin_publishes_thirteen_methods() {
    let env = load_full_game_ui_with_dashboard_lod();
    assert_mixin_methods(
        &env,
        "HousingCatalogFrameMixin",
        &[
            "OnLoad",
            "OneTimeInit",
            "OnEvent",
            "OnShow",
            "OnHide",
            "OnEntryResultsUpdated",
            "UpdateCatalogData",
            "UpdateCategoryText",
            "OnOpenToDecorID",
            "OnCatalogEntryUpdated",
            "OnSearchTextUpdated",
            "OnCategoryFocusChanged",
            "ClearSearchText",
        ],
        "the catalog-tab umbrella; OneTimeInit lazily creates the \
         C_HousingCatalog.CreateCatalogSearcher and wires \
         Filters/Categories/SearchBox/OptionsContainer; OnOpenToDecorID supports deep-link \
         scroll-to-entry from chat link clicks",
    );
}

#[test]
fn blizzard_housing_dashboard_frame_publishes_tab_buttons_and_content_children() {
    let env = load_full_game_ui_with_dashboard_lod();

    for child in [
        "HouseInfoContent",
        "CatalogContent",
        "HouseInfoTabButton",
        "CatalogTabButton",
    ] {
        let exists: bool = env
            .eval(&format!(
                "local f = HousingDashboardFrame['{child}']; return type(f) == 'table' and type(f.GetName) == 'function'"
            ))
            .expect("HousingDashboardFrame parentKey lookup should succeed");
        assert!(
            exists,
            "HousingDashboardFrame.{child} must publish via parentKey — Dashboard XML wires four \
             child frames: HouseInfoContent (HousingDashboardHouseInfoTemplate), CatalogContent \
             (HousingCatalogFrameTemplate), and the two side-tab buttons that drive SetTab"
        );
    }
}

#[test]
fn blizzard_housing_dashboard_tab_buttons_array_holds_two_entries() {
    let env = load_full_game_ui_with_dashboard_lod();

    let count: i64 = env
        .eval("local arr = HousingDashboardFrame.TabButtons; return arr and #arr or -1")
        .expect("TabButtons parentArray lookup should succeed");
    assert_eq!(
        count, 2,
        "HousingDashboardFrame.TabButtons parentArray must collect exactly 2 entries — both side \
         tab buttons inherit `parentArray=\"TabButtons\"` from HousingDashboardSideTabTemplate so \
         they auto-collect into the array OnLoad iterates to wire SetCustomOnMouseUpHandler. Got \
         length {count}"
    );
}

#[test]
fn blizzard_housing_dashboard_registers_left_panel_with_extra_width_func() {
    let env = load_full_game_ui_with_dashboard_lod();

    let area: String = env
        .eval(
            "local entry = UIPanelWindows['HousingDashboardFrame']; return entry and entry.area or ''",
        )
        .expect("UIPanelWindows lookup should succeed");
    assert_eq!(
        area, "left",
        "Registration.lua calls RegisterUIPanel(HousingDashboardFrame, attributes) with \
         area=\"left\" — the dashboard docks to the left edge of the screen, contrasting with \
         HousingCornerstone's area=\"center\" for the central popup style"
    );

    let pushable: i64 = env
        .eval(
            "local entry = UIPanelWindows['HousingDashboardFrame']; return entry and entry.pushable or -1",
        )
        .expect("pushable lookup should succeed");
    assert_eq!(
        pushable, 0,
        "Registration uses pushable=0 — dashboard does not get pushed aside by other left-area \
         panels; it has the lowest displacement priority"
    );

    let extra_width_set: bool = env
        .eval(
            "local entry = UIPanelWindows['HousingDashboardFrame']; return entry and type(entry.extraWidthFunc) == 'function'",
        )
        .expect("extraWidthFunc lookup should succeed");
    assert!(
        extra_width_set,
        "Registration sets `extraWidthFunc = HousingDashboardFrameMixin.GetPanelExtraWidth` — \
         this lets UIParent reserve room for the side tab strip outside the dashboard's main \
         frame width when computing left-area layout slots"
    );
}
