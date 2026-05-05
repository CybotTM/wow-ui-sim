//! Test that loading Blizzard addons and firing startup events produces no warnings.

mod common;

use std::collections::{HashMap, HashSet};
use std::path::{Path, PathBuf};
use wow_ui_sim::loader::{discover_all_blizzard_addons, discover_blizzard_addons, load_addon};
use wow_ui_sim::lua_api::WowLuaEnv;
use wow_ui_sim::screen::ScreenKind;
use wow_ui_sim::toc::TocFile;

const STARTUP_WARNING_GAME_FOUNDATIONS: &[&str] = &[
    "Blizzard_SharedXMLBase",
    "Blizzard_Menu",
    "Blizzard_SharedXML",
    "Blizzard_SharedXMLGame",
    "Blizzard_FrameXMLBase",
    "Blizzard_UIPanelTemplates",
    "Blizzard_FrameXMLUtil",
    "Blizzard_FrameXML",
    "Blizzard_UIParent",
    "Blizzard_UIParentPanelManager",
];

const STARTUP_WARNING_GLUE_FOUNDATIONS: &[&str] = &[
    "Blizzard_GlueXMLBase",
    "Blizzard_GlueParent",
    "Blizzard_GlueMenuFrame",
    "Blizzard_GlueXML",
];

fn blizzard_ui_dir() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("Interface/BlizzardUI")
}

/// Install a Lua error handler that collects errors into `__test_errors`.
fn install_test_error_handler(env: &WowLuaEnv) {
    common::install_error_collector(env, "__test_errors");
}

/// Read collected errors from `__test_errors` and clear it.
fn drain_test_errors(env: &WowLuaEnv) -> Vec<String> {
    common::drain_string_table(env, "__test_errors")
}

/// Fire a single event, collecting handler errors via the Lua error handler.
fn fire(env: &WowLuaEnv, event: &str, args: &[rilua::Val]) -> Vec<String> {
    env.fire_event_with_args(event, args).ok();
    drain_test_errors(env)
}

fn collect_addon_load_warnings(env: &WowLuaEnv, name: &str, toc_path: &Path) -> Vec<String> {
    match load_addon(&env.loader_env(), toc_path) {
        Ok(result) => result
            .warnings
            .into_iter()
            .map(|warning| format!("[load {name}] {warning}"))
            .collect(),
        Err(error) => vec![format!("[load {name}] FAILED: {error}")],
    }
}

/// Load all Blizzard addons and fire startup events, collecting all warnings.
fn load_and_startup() -> Vec<String> {
    let env = WowLuaEnv::new().expect("Failed to create Lua environment");
    env.set_screen_size(1024.0, 768.0);
    env.set_screen_mode(ScreenKind::Login);

    let ui = blizzard_ui_dir();
    let addons = discover_blizzard_addons(&ui);
    let mut warnings = Vec::new();

    // Load addons
    for (name, toc_path) in &addons {
        warnings.extend(collect_addon_load_warnings(&env, name, toc_path));
    }

    // Apply workarounds (same as main.rs run_post_load_scripts)
    env.apply_post_load_workarounds();

    // Install error handler before firing events
    install_test_error_handler(&env);

    // Fire startup events (same sequence as main.rs)
    fire_startup_events(&env, &mut warnings);

    // Keep only the most recent 500 warnings
    if warnings.len() > 500 {
        warnings.drain(..warnings.len() - 500);
    }

    warnings
}

fn fire_startup_events(env: &WowLuaEnv, warnings: &mut Vec<String>) {
    warnings.extend(fire(env, "ADDON_LOADED", &[env.lua_string("WoWUISim")]));
    for event in ["VARIABLES_LOADED", "PLAYER_LOGIN"] {
        warnings.extend(fire(env, event, &[]));
    }

    env.fire_edit_mode_layouts_updated().ok();
    warnings.extend(drain_test_errors(env));

    common::call_global_if_present(env, "RequestTimePlayed");
    warnings.extend(fire(
        env,
        "PLAYER_ENTERING_WORLD",
        &[rilua::Val::Bool(true), rilua::Val::Bool(false)],
    ));
    for event in [
        "UPDATE_BINDINGS",
        "DISPLAY_SIZE_CHANGED",
        "UI_SCALE_CHANGED",
        "PLAYER_LEAVING_WORLD",
    ] {
        warnings.extend(fire(env, event, &[]));
    }

    // Fire one OnUpdate tick to catch handler errors
    env.fire_on_update(0.016).ok();
    warnings.extend(drain_test_errors(env));
}

/// Known warning count from unimplemented APIs. Update this when adding stubs.
/// Goal: drive this to zero over time by implementing missing APIs.
const KNOWN_WARNING_COUNT: usize = 0;

#[test]
fn test_no_warnings_on_startup() {
    test_timeout! {
        let warnings = load_and_startup();
        let count = warnings.len();
        let account_store_regressions: Vec<String> = warnings
            .iter()
            .filter(|warning| {
                (warning.contains("attempt to index global 'AccountStoreFrame'")
                    && warning.contains("UIParent.lua:352"))
                    || warning.contains("AccountStoreFrame:SetStoreFrontID")
            })
            .cloned()
            .collect();

        assert!(
            account_store_regressions.is_empty(),
            "Regression: AccountStore startup nil-path error reintroduced.\n\
             Matching warnings:\n  {}",
            account_store_regressions.join("\n  ")
        );

        if count > KNOWN_WARNING_COUNT {
            let mut msg = format!(
                "New warnings introduced! Expected at most {KNOWN_WARNING_COUNT}, got {count}.\n\
                 All warnings:\n"
            );
            for w in &warnings {
                msg.push_str(&format!("  {w}\n"));
            }
            panic!("{msg}");
        }
    }
}

#[test]
fn test_edit_mode_layout_update_ignores_preset_layouts_during_startup() {
    test_timeout! {
        let env = load_all_addons();
        install_test_error_handler(&env);

        env.fire_edit_mode_layouts_updated().ok();
        let warnings = drain_test_errors(&env);
        let edit_mode_layout_warnings: Vec<String> = warnings
            .iter()
            .filter(|warning| {
                warning.contains("Blizzard_EditMode/Shared/EditModeManager.lua:917")
                    || warning.contains("UpdateLayoutCounts")
                    || warning.contains("attempt to perform arithmetic on field '?'")
            })
            .cloned()
            .collect();

        assert!(
            edit_mode_layout_warnings.is_empty(),
            "Edit mode layout update should ignore preset layouts when counting saved layouts:\n  {}",
            edit_mode_layout_warnings.join("\n  ")
        );
    }
}

/// Load all Blizzard addons and apply workarounds (no startup events).
fn load_all_addons() -> WowLuaEnv {
    let env = WowLuaEnv::new().expect("Failed to create Lua environment");
    env.set_screen_size(1024.0, 768.0);
    env.set_screen_mode(ScreenKind::Login);

    let ui = blizzard_ui_dir();
    let addons = discover_blizzard_addons(&ui);
    for (name, toc_path) in &addons {
        if let Err(e) = load_addon(&env.loader_env(), toc_path) {
            eprintln!("[load {name}] FAILED: {e}");
        }
    }
    env.apply_post_load_workarounds();
    env
}

fn load_single_blizzard_addon(addon_name: &str) -> (WowLuaEnv, Vec<String>) {
    let env = WowLuaEnv::new().expect("Failed to create Lua environment");
    env.set_screen_size(1024.0, 768.0);
    env.set_screen_mode(ScreenKind::Login);

    let addons = addon_map_all();
    let warnings = load_blizzard_addon_with_foundations(&env, &addons, addon_name);
    (env, warnings)
}

fn load_blizzard_addon_by_folder(folder_name: &str) -> (WowLuaEnv, Vec<String>) {
    let env = WowLuaEnv::new().expect("Failed to create Lua environment");
    env.set_screen_size(1024.0, 768.0);
    env.set_screen_mode(ScreenKind::Login);

    let addons = addon_map_all();
    let warnings = load_blizzard_addon_with_foundations(&env, &addons, folder_name);
    (env, warnings)
}

fn addon_map_all() -> HashMap<String, PathBuf> {
    let ui = blizzard_ui_dir();
    discover_all_blizzard_addons(&ui).into_iter().collect()
}

fn load_blizzard_addon_with_foundations(
    env: &WowLuaEnv,
    addons: &HashMap<String, PathBuf>,
    addon_name: &str,
) -> Vec<String> {
    let mut loading = HashSet::new();
    let mut loaded = HashSet::new();
    let mut warnings = Vec::new();
    load_blizzard_addon_recursive(
        env,
        addons,
        addon_name,
        true,
        &mut loading,
        &mut loaded,
        &mut warnings,
    );
    warnings
}

fn load_blizzard_addon_recursive(
    env: &WowLuaEnv,
    addons: &HashMap<String, PathBuf>,
    addon_name: &str,
    required: bool,
    loading: &mut HashSet<String>,
    loaded: &mut HashSet<String>,
    warnings: &mut Vec<String>,
) {
    if loaded.contains(addon_name) || loading.contains(addon_name) {
        return;
    }
    loading.insert(addon_name.to_string());

    let Some(toc_path) = addons.get(addon_name) else {
        if required {
            panic!("addon {addon_name} should exist");
        }
        loading.remove(addon_name);
        return;
    };
    let toc = TocFile::from_file(toc_path)
        .unwrap_or_else(|error| panic!("{addon_name} TOC should parse: {error}"));

    for foundation in startup_warning_foundations(addon_name) {
        if foundation != addon_name && addons.contains_key(foundation) {
            load_blizzard_addon_recursive(
                env, addons, foundation, false, loading, loaded, warnings,
            );
        }
    }

    for dependency in toc_dependency_names(&toc, addons) {
        if dependency != addon_name {
            load_blizzard_addon_recursive(
                env,
                addons,
                &dependency,
                false,
                loading,
                loaded,
                warnings,
            );
        }
    }

    let result = load_addon(&env.loader_env(), toc_path)
        .unwrap_or_else(|error| panic!("{addon_name} should load: {error}"));
    warnings.extend(
        result
            .warnings
            .into_iter()
            .map(|warning| format!("[load {addon_name}] {warning}")),
    );
    loading.remove(addon_name);
    loaded.insert(addon_name.to_string());
}

fn startup_warning_foundations(addon_name: &str) -> Vec<&'static str> {
    let foundations = if addon_name.starts_with("Blizzard_Glue") {
        STARTUP_WARNING_GLUE_FOUNDATIONS
    } else {
        STARTUP_WARNING_GAME_FOUNDATIONS
    };
    let end = foundations
        .iter()
        .position(|candidate| *candidate == addon_name)
        .unwrap_or(foundations.len());
    foundations[..end].to_vec()
}

fn toc_dependency_names(toc: &TocFile, addons: &HashMap<String, PathBuf>) -> Vec<String> {
    let mut dependencies = toc.dependencies();
    let mut seen: HashSet<String> = dependencies.iter().cloned().collect();
    for dependency in toc.optional_deps() {
        if addons.contains_key(&dependency) && seen.insert(dependency.clone()) {
            dependencies.push(dependency);
        }
    }
    dependencies
}

/// Assert that a Lua expression evaluates to true.
fn assert_lua(env: &WowLuaEnv, code: &str, msg: &str) {
    assert!(env.eval::<bool>(code).unwrap_or(false), "{msg}");
}

/// Fire startup events and process timers.
fn fire_events_and_timers(env: &WowLuaEnv) {
    common::fire_addon_loaded(env, "WoWUISim");
    for event in ["VARIABLES_LOADED", "PLAYER_LOGIN"] {
        let _ = env.fire_event(event);
    }
    common::fire_player_entering_world(env, true, false);
    let _ = env.process_timers();
    let _ = env.fire_on_update(0.016);
    let _ = env.process_timers();
}

/// Audit: every `[LoadIntoEnvironment secure]` annotation in Blizzard TOCs
/// must match this explicit list.
///
/// Fails if a new annotation is introduced (we want to know about it
/// deliberately, not silently), or if an existing annotation disappears
/// (so we can track Blizzard's secure-env footprint across patches).
#[test]
fn test_secure_env_toc_annotations_are_exhaustive() {
    let ui = blizzard_ui_dir();
    let toc_paths = wow_ui_sim::loader::discover_all_blizzard_addons(&ui)
        .into_iter()
        .map(|(_, path)| path)
        .collect::<Vec<_>>();

    let mut secure_files: Vec<String> = Vec::new();
    for toc_path in &toc_paths {
        let Ok(toc) = wow_ui_sim::toc::TocFile::from_file(toc_path) else {
            continue;
        };
        for (index, file) in toc.files.iter().enumerate() {
            if toc.file_use_secure_env(index) == Some(true) {
                let addon = toc_path
                    .parent()
                    .and_then(|p| p.file_name())
                    .and_then(|n| n.to_str())
                    .unwrap_or("?");
                secure_files.push(format!("{addon}/{}", file.display()));
            }
        }
    }
    secure_files.sort();

    let expected = vec![
        "Blizzard_ChatFrameBase/Shared/ChatFrameFiltersSecure.lua".to_string(),
        "Blizzard_RestrictedAddOnEnvironment/RestrictedEnvironment.lua".to_string(),
    ];
    assert_eq!(
        secure_files, expected,
        "the set of [LoadIntoEnvironment secure] files drifted; update this test \
         only after confirming the new/removed entries actually run under secureenv"
    );
}

/// Every annotated secure file should load without warnings and its
/// downstream surface should be reachable — a smoke test that our per-file
/// TOC secureenv dispatch is wired end-to-end.
#[test]
fn test_secure_env_annotated_files_load_cleanly() {
    test_timeout! {
        let env = WowLuaEnv::new().expect("Failed to create Lua environment");
        env.set_screen_size(1024.0, 768.0);

        let ui = blizzard_ui_dir();
        let addons = discover_blizzard_addons(&ui);
        let mut warnings: Vec<String> = Vec::new();
        for (name, toc_path) in &addons {
            let result = load_addon(&env.loader_env(), toc_path)
                .unwrap_or_else(|e| panic!("{name} should load: {e}"));
            warnings.extend(result.warnings);
            // Stop once both secure-annotated addons have loaded.
            if name == "Blizzard_ChatFrameBase" || name == "Blizzard_RestrictedAddOnEnvironment" {
                continue;
            }
        }

        let noisy_secure = warnings
            .iter()
            .filter(|w| {
                w.contains("ChatFrameFiltersSecure.lua") || w.contains("RestrictedEnvironment.lua")
            })
            .cloned()
            .collect::<Vec<_>>();
        assert!(
            noisy_secure.is_empty(),
            "secure-annotated files should load cleanly, got:\n{}",
            noisy_secure.join("\n")
        );

        // Downstream surfaces exposed by secure files.
        let (restricted_scope_ty, secure_filters_delegate_ty): (String, String) = env
            .eval(
                r#"
                -- RESTRICTED_FUNCTIONS_SCOPE lands on the addon table, not _G,
                -- so we probe the scope via a known descendent global that
                -- Blizzard_RestrictedAddOnEnvironment registers once loaded.
                return type(CallRestrictedClosure),
                       type(SecureTypes and SecureTypes.CreateSecureArray)
                "#,
            )
            .expect("secure surface should be introspectable");
        assert_eq!(restricted_scope_ty, "function");
        assert_eq!(secure_filters_delegate_ty, "function");
    }
}

#[test]
fn test_restricted_addon_environment_exposes_execution_surface() {
    test_timeout! {
        let env = WowLuaEnv::new().expect("Failed to create Lua environment");
        env.set_screen_size(1024.0, 768.0);

        let ui = blizzard_ui_dir();
        let addons = discover_blizzard_addons(&ui);
        let mut restricted_result = None;
        for (name, toc_path) in addons {
            let result = load_addon(&env.loader_env(), &toc_path)
                .unwrap_or_else(|error| panic!("{name} should load before restricted env: {error}"));
            if name == "Blizzard_RestrictedAddOnEnvironment" {
                restricted_result = Some(result);
                break;
            }
        }
        let result = restricted_result.expect("Blizzard_RestrictedAddOnEnvironment should be in addon order");

        let warnings = result.warnings.join("\n");
        assert!(
            !warnings.contains("RestrictedExecution.lua"),
            "RestrictedExecution.lua should load cleanly:\n{warnings}"
        );
        assert!(
            !warnings.contains("SecureHoverDriver.lua"),
            "SecureHoverDriver.lua should load cleanly:\n{warnings}"
        );

        let (
            get_frame_mt_ty,
            get_frame_mt_index_ty,
            call_fn_ty,
            propagate_fn_ty,
        ): (String, String, String, String) = env
            .eval(
                r#"
                local frameMt = GetFrameMetatable()
                return type(frameMt),
                    type(frameMt and frameMt.__index),
                    type(CallRestrictedClosure),
                    type(PropagateForbiddenToReferencedFrames)
                "#,
            )
            .expect("restricted surface inspection should be callable");

        assert_eq!(get_frame_mt_ty, "table");
        assert_eq!(get_frame_mt_index_ty, "table");
        assert_eq!(call_fn_ty, "function");
        assert_eq!(propagate_fn_ty, "function");
    }
}

/// Verify that template mixin inheritance is properly applied.
///
/// ObjectiveTrackerUIWidgetContainer inherits UIWidgetContainerTemplate,
/// which inherits UIWidgetContainerNoResizeTemplate (mixin UIWidgetContainerMixin).
/// GetNumWidgetsShowing must be available on the frame.
#[test]
fn test_widget_container_mixin_applied() {
    test_timeout! {
        let env = load_all_addons();

        assert_lua(&env, "return type(UIWidgetContainerMixin) == 'table'",
            "UIWidgetContainerMixin should exist as a Lua table");
        assert_lua(&env, "return type(UIWidgetContainerMixin.GetNumWidgetsShowing) == 'function'",
            "UIWidgetContainerMixin should have GetNumWidgetsShowing");
        assert_lua(&env, "return ObjectiveTrackerUIWidgetContainer ~= nil",
            "ObjectiveTrackerUIWidgetContainer should exist");
        assert_lua(&env, "return type(ObjectiveTrackerUIWidgetContainer.GetNumWidgetsShowing) == 'function'",
            "ObjectiveTrackerUIWidgetContainer should have GetNumWidgetsShowing from UIWidgetContainerMixin");

        let result: i64 = env
            .eval("return ObjectiveTrackerUIWidgetContainer:GetNumWidgetsShowing()")
            .expect("GetNumWidgetsShowing() should not error");
        assert_eq!(result, 0, "No widgets should be showing initially");

        // Verify the method survives startup events and timer processing
        fire_events_and_timers(&env);
        assert_lua(&env, "return type(ObjectiveTrackerUIWidgetContainer.GetNumWidgetsShowing) == 'function'",
            "GetNumWidgetsShowing should still be available after startup events and timer processing");
    }
}

#[test]
fn test_uiparent_onshow_loads_account_store_without_nil_error() {
    test_timeout! {
        let env = load_all_addons();

        let (ok, loaded_after, account_store_exists, err): (bool, bool, bool, Option<String>) = env
            .eval(
                r#"
                local ok, err = pcall(function()
                    UIParent.firstTimeLoaded = nil
                    UIParent_OnShow(UIParent)
                end)
                return ok,
                    C_AddOns.IsAddOnLoaded("Blizzard_AccountStore"),
                    AccountStoreFrame ~= nil,
                    ok and nil or tostring(err)
                "#,
            )
            .expect("UIParent_OnShow should be callable");

        assert!(ok, "UIParent_OnShow should not error: {:?}", err);
        assert!(
            loaded_after,
            "UIParent_OnShow should load Blizzard_AccountStore via C_AddOns.LoadAddOn"
        );
        assert!(
            account_store_exists,
            "AccountStoreFrame should exist after UIParent_OnShow addon load"
        );
    }
}

#[test]
fn test_account_store_frame_exposes_mixin_methods_after_load() {
    test_timeout! {
        let (env, warnings) = load_blizzard_addon_by_folder("Blizzard_AccountStore");

        let (frame_ty, on_load_ty, set_storefront_ty, set_fullscreen_ty): (
            String,
            String,
            String,
            String,
        ) = env
            .eval(
                r#"
                return type(AccountStoreFrame),
                    type(AccountStoreFrame and AccountStoreFrame.OnLoad),
                    type(AccountStoreFrame and AccountStoreFrame.SetStoreFrontID),
                    type(AccountStoreFrame and AccountStoreFrame.SetFullscreenMode)
                "#,
            )
            .expect("AccountStoreFrame inspection should be callable");

        assert_eq!(frame_ty, "table", "AccountStoreFrame should exist after Blizzard_AccountStore load");
        assert_eq!(
            on_load_ty, "function",
            "AccountStoreFrame.OnLoad should come from AccountStoreMixin; warnings:\n  {}",
            warnings.join("\n  ")
        );
        assert_eq!(set_storefront_ty, "function");
        assert_eq!(set_fullscreen_ty, "function");
    }
}

#[test]
fn test_account_store_set_storefront_id_is_safe_after_load() {
    test_timeout! {
        let (env, warnings) = load_blizzard_addon_by_folder("Blizzard_AccountStore");

        let (ok, stored_id, err): (bool, i64, Option<String>) = env
            .eval(
                r#"
                local ok, err = pcall(function()
                    AccountStoreFrame:SetStoreFrontID(Constants.AccountStoreConsts.PlunderstormStoreFrontID)
                end)
                return ok, AccountStoreFrame.storeFrontID or 0, ok and nil or tostring(err)
                "#,
            )
            .expect("AccountStoreFrame:SetStoreFrontID should be callable");

        assert!(
            ok,
            "AccountStoreFrame:SetStoreFrontID should not error; warnings:\n  {}\nerror: {:?}",
            warnings.join("\n  "),
            err
        );
        assert_eq!(
            stored_id,
            1,
            "AccountStoreFrame:SetStoreFrontID should preserve the storefront id"
        );
    }
}

#[test]
fn test_c_addons_load_addon_preserves_account_store_mixin_methods() {
    test_timeout! {
        let env = load_all_addons();

        let (
            ok,
            loaded,
            frame_ty,
            mixin_ty,
            mixin_set_storefront_ty,
            on_load_ty,
            set_storefront_ty,
            set_fullscreen_ty,
            err,
        ): (
            bool,
            bool,
            String,
            String,
            String,
            String,
            String,
            String,
            Option<String>,
        ) = env
            .eval(
                r#"
                local ok, err = pcall(function()
                    C_AddOns.LoadAddOn("Blizzard_AccountStore")
                end)
                return ok,
                    C_AddOns.IsAddOnLoaded("Blizzard_AccountStore"),
                    type(AccountStoreFrame),
                    type(AccountStoreMixin),
                    type(AccountStoreMixin and AccountStoreMixin.SetStoreFrontID),
                    type(AccountStoreFrame and AccountStoreFrame.OnLoad),
                    type(AccountStoreFrame and AccountStoreFrame.SetStoreFrontID),
                    type(AccountStoreFrame and AccountStoreFrame.SetFullscreenMode),
                    ok and nil or tostring(err)
                "#,
            )
            .expect("C_AddOns.LoadAddOn inspection should be callable");

        assert!(ok, "C_AddOns.LoadAddOn should not error: {:?}", err);
        assert!(loaded, "Blizzard_AccountStore should be marked loaded");
        assert_eq!(
            frame_ty, "table",
            "AccountStoreFrame should exist after runtime load"
        );
        assert_eq!(
            mixin_ty, "table",
            "AccountStoreMixin should exist after runtime load"
        );
        assert_eq!(
            mixin_set_storefront_ty, "function",
            "AccountStoreMixin.SetStoreFrontID should exist after runtime load"
        );
        assert_eq!(
            on_load_ty, "function",
            "AccountStoreFrame.OnLoad should exist after runtime load"
        );
        assert_eq!(
            set_storefront_ty, "function",
            "AccountStoreFrame.SetStoreFrontID should exist after runtime load"
        );
        assert_eq!(
            set_fullscreen_ty, "function",
            "AccountStoreFrame.SetFullscreenMode should exist after runtime load"
        );
    }
}

#[test]
fn test_rust_load_addon_after_base_load_preserves_account_store_mixin_methods() {
    test_timeout! {
        let env = load_all_addons();

        let ui = blizzard_ui_dir();
        let addons = discover_all_blizzard_addons(&ui);
        let (_, toc_path) = addons
            .into_iter()
            .find(|(name, _)| name == "Blizzard_AccountStore")
            .expect("Blizzard_AccountStore should exist");
        let _result = load_addon(&env.loader_env(), &toc_path).expect("late Rust load should succeed");

        let (
            mixin_fn_ty,
            scratch_on_load_ty,
            frame_ty,
            mixin_ty,
            on_load_ty,
            set_storefront_ty,
            _get_object_type_ty,
            _set_point_ty,
        ): (
            String,
            String,
            String,
            String,
            String,
            String,
            String,
            String,
        ) =
            env.eval(
                r#"
                local scratch = {}
                if type(Mixin) == "function" and type(AccountStoreMixin) == "table" then
                    Mixin(scratch, AccountStoreMixin)
                end
                return type(Mixin),
                    type(scratch.OnLoad),
                    type(AccountStoreFrame),
                    type(AccountStoreMixin),
                    type(AccountStoreFrame and AccountStoreFrame.OnLoad),
                    type(AccountStoreFrame and AccountStoreFrame.SetStoreFrontID),
                    type(AccountStoreFrame and AccountStoreFrame.GetObjectType),
                    type(AccountStoreFrame and AccountStoreFrame.SetPoint)
                "#,
            )
            .expect("late Rust load inspection should be callable");
        assert_eq!(mixin_fn_ty, "function");
        assert_eq!(scratch_on_load_ty, "function");
        assert_eq!(frame_ty, "table");
        assert_eq!(mixin_ty, "table");
        assert_eq!(on_load_ty, "function");
        assert_eq!(set_storefront_ty, "function");
    }
}

#[test]
fn test_low_health_frame_animation_bound_after_load() {
    test_timeout! {
        let env = load_all_addons();

        let (frame_ty, group_ty, alpha_ty): (String, String, String) = env
            .eval(
                r#"
                return type(LowHealthFrame),
                    type(LowHealthFrame and LowHealthFrame.pulseAnim),
                    type(LowHealthFrame and LowHealthFrame.pulseAnim and LowHealthFrame.pulseAnim.AlphaAnim)
                "#,
            )
            .expect("LowHealthFrame inspection should be callable");

        assert_eq!(frame_ty, "table", "LowHealthFrame should exist after addon load");
        assert_eq!(group_ty, "table", "LowHealthFrame.pulseAnim should exist after addon load");
        assert_eq!(
            alpha_ty, "table",
            "LowHealthFrame.pulseAnim.AlphaAnim should exist after addon load"
        );
    }
}

#[test]
fn test_low_health_frame_animation_bound_after_blizzard_framexml_load() {
    test_timeout! {
        let (env, warnings) = load_single_blizzard_addon("Blizzard_FrameXML");

        let (frame_ty, group_ty, alpha_ty): (String, String, String) = env
            .eval(
                r#"
                return type(LowHealthFrame),
                    type(LowHealthFrame and LowHealthFrame.pulseAnim),
                    type(LowHealthFrame and LowHealthFrame.pulseAnim and LowHealthFrame.pulseAnim.AlphaAnim)
                "#,
            )
            .expect("LowHealthFrame inspection should be callable");

        assert_eq!(frame_ty, "table", "LowHealthFrame should exist after Blizzard_FrameXML load");
        assert_eq!(
            group_ty, "table",
            "LowHealthFrame.pulseAnim should exist after Blizzard_FrameXML load; warnings:\n  {}",
            warnings.join("\n  ")
        );
        assert_eq!(
            alpha_ty, "table",
            "LowHealthFrame.pulseAnim.AlphaAnim should exist after Blizzard_FrameXML load"
        );
    }
}

#[test]
fn test_blizzard_framexml_load_registers_boss_banner_cvar_without_warning() {
    test_timeout! {
        let (env, warnings) = load_single_blizzard_addon("Blizzard_FrameXML");

        let register_cvar_warnings: Vec<String> = warnings
            .iter()
            .filter(|warning| {
                warning.contains("RegisterCVar")
                    || warning.contains("CvarUtil.lua:2")
                    || warning.contains("PraiseTheSun")
            })
            .cloned()
            .collect();

        assert!(
            register_cvar_warnings.is_empty(),
            "Blizzard_FrameXML should not warn on BossBanner RegisterCVar:\n  {}",
            register_cvar_warnings.join("\n  ")
        );

        let (value, default_value): (String, String) = env
            .eval(r#"return GetCVar("PraiseTheSun"), GetCVarDefault("PraiseTheSun")"#)
            .expect("BossBanner cvar should be readable after Blizzard_FrameXML load");
        assert!(
            value == "0" || value == "1",
            "BossBanner cvar should be readable after registration, got {value:?}"
        );
        assert_eq!(default_value, "0");
    }
}

#[test]
fn test_blizzard_framexml_loads_role_poll_without_role_icon_warning() {
    test_timeout! {
        let env = WowLuaEnv::new().expect("Failed to create Lua environment");
        env.set_screen_size(1024.0, 768.0);

        let ui = blizzard_ui_dir();
        let addons = discover_blizzard_addons(&ui);
        let mut warnings = Vec::new();

        for (name, toc_path) in &addons {
            let result = load_addon(&env.loader_env(), toc_path)
                .unwrap_or_else(|error| panic!("{name} should load: {error}"));
            for warning in result.warnings {
                warnings.push(format!("[load {name}] {warning}"));
            }
            if name == "Blizzard_FrameXML" {
                break;
            }
        }

        let role_icon_warnings: Vec<String> = warnings
            .iter()
            .filter(|warning| {
                warning.contains("GetIconForRoleEnum")
                    || warning.contains("RolePollPopupRoleButton")
            })
            .cloned()
            .collect();

        assert!(
            role_icon_warnings.is_empty(),
            "Blizzard_FrameXML should not warn on RolePoll role icons:\n  {}",
            role_icon_warnings.join("\n  ")
        );

        let (tank_role, healer_role, damage_role): (i32, i32, i32) = env
            .eval(
                r#"
                return RolePollPopupRoleButtonTank.role or -1,
                       RolePollPopupRoleButtonHealer.role or -1,
                       RolePollPopupRoleButtonDPS.role or -1
                "#,
            )
            .expect("RolePoll role buttons should stay readable after FrameXML load");
        assert_eq!(tank_role, 0);
        assert_eq!(healer_role, 1);
        assert_eq!(damage_role, 2);
    }
}

#[test]
fn test_blizzard_framexml_loads_zone_text_without_fading_frame_warning() {
    test_timeout! {
        let (env, warnings) = load_single_blizzard_addon("Blizzard_FrameXML");

        let fading_warnings: Vec<String> = warnings
            .iter()
            .filter(|warning| {
                warning.contains("FadingFrame_OnLoad")
                    || warning.contains("FadingFrame_Show")
                    || warning.contains("ZoneText.lua:72")
                    || warning.contains("ZoneText.lua:124")
            })
            .cloned()
            .collect();

        assert!(
            fading_warnings.is_empty(),
            "Blizzard_FrameXML should not warn on fading-frame helpers:\n  {}",
            fading_warnings.join("\n  ")
        );

        let (zone_hidden, subzone_hidden, fade_in, hold, fade_out): (bool, bool, f64, f64, f64) = env
            .eval(
                r#"
                return not ZoneTextFrame:IsShown(),
                       not SubZoneTextFrame:IsShown(),
                       ZoneTextFrame.fadeInTime,
                       ZoneTextFrame.holdTime,
                       ZoneTextFrame.fadeOutTime
                "#,
            )
            .expect("zone text fading-frame state should be readable");
        assert!(zone_hidden);
        assert!(subzone_hidden);
        assert_eq!(fade_in, 0.5);
        assert_eq!(hold, 1.0);
        assert_eq!(fade_out, 2.0);
    }
}

#[test]
fn test_blizzard_framexml_loads_without_eventutil_warning() {
    test_timeout! {
        let (_env, warnings) = load_single_blizzard_addon("Blizzard_FrameXML");

        let eventutil_warnings: Vec<String> = warnings
            .iter()
            .filter(|warning| {
                warning.contains("EventUtil")
                    || warning.contains("MotionSickness.lua:23")
                    || warning.contains("AlertFrames.lua:281")
            })
            .cloned()
            .collect();

        assert!(
            eventutil_warnings.is_empty(),
            "Blizzard_FrameXML should not warn on EventUtil helpers:\n  {}",
            eventutil_warnings.join("\n  ")
        );
    }
}

#[test]
fn test_blizzard_framexml_loads_without_setup_localization_warning() {
    test_timeout! {
        let (_env, warnings) = load_single_blizzard_addon("Blizzard_FrameXML");

        let localization_warnings: Vec<String> = warnings
            .iter()
            .filter(|warning| {
                warning.contains("SetupLocalization")
                    || warning.contains("Shared/Localization.lua:55")
                    || warning.contains("Mainline/Localization.lua:48")
            })
            .cloned()
            .collect();

        assert!(
            localization_warnings.is_empty(),
            "Blizzard_FrameXML should not warn on SetupLocalization:\n  {}",
            localization_warnings.join("\n  ")
        );
    }
}

#[test]
fn test_blizzard_framexml_loads_without_frameutil_warning() {
    test_timeout! {
        let (_env, warnings) = load_single_blizzard_addon("Blizzard_FrameXML");

        let frameutil_warnings: Vec<String> = warnings
            .iter()
            .filter(|warning| {
                warning.contains("FrameUtil")
                    || warning.contains("UIErrorsFrame.lua:8")
                    || warning.contains("LootHistory.lua:307")
                    || warning.contains("QuestSession.lua:831")
            })
            .cloned()
            .collect();

        assert!(
            frameutil_warnings.is_empty(),
            "Blizzard_FrameXML should not warn on FrameUtil helpers:\n  {}",
            frameutil_warnings.join("\n  ")
        );
    }
}

#[test]
fn test_blizzard_commentator_loads_without_cooldown_frame_warning() {
    test_timeout! {
        let (_env, warnings) = load_blizzard_addon_by_folder("Blizzard_Commentator");

        let cooldown_warnings: Vec<String> = warnings
            .iter()
            .filter(|warning| {
                warning.contains("CooldownFrame_Set")
                    || warning.contains("CooldownFrame_Clear")
                    || warning.contains("Blizzard_CommentatorSpell.lua:74")
                    || warning.contains("Blizzard_CommentatorSpell.lua:87")
                    || warning.contains("Blizzard_CommentatorSpell.lua:88")
                    || warning.contains("Blizzard_CommentatorSpell.lua:105")
            })
            .cloned()
            .collect();

        assert!(
            cooldown_warnings.is_empty(),
            "Blizzard_Commentator should not warn on cooldown-frame helpers:\n  {}",
            cooldown_warnings.join("\n  ")
        );
    }
}

#[test]
fn test_ui_parent_panel_manager_loads_with_minimap_cluster_stub() {
    test_timeout! {
        let (env, warnings) = load_single_blizzard_addon("Blizzard_UIParentPanelManager");

        let minimap_cluster_warnings: Vec<String> = warnings
            .iter()
            .filter(|warning| {
                warning.contains("MinimapCluster")
                    || warning.contains("UIParentPanelManager.lua:784")
            })
            .cloned()
            .collect();

        assert!(
            minimap_cluster_warnings.is_empty(),
            "Blizzard_UIParentPanelManager should not warn on MinimapCluster:\n  {}",
            minimap_cluster_warnings.join("\n  ")
        );

        let (cluster_type, minimap_is_child, cluster_height): (String, bool, f64) = env
            .eval(
                r#"
                return type(MinimapCluster),
                       Minimap:GetParent() == MinimapCluster,
                       MinimapCluster:GetHeight()
                "#,
            )
            .expect("startup MinimapCluster stub should be queryable");

        assert_eq!(cluster_type, "table");
        assert!(minimap_is_child, "startup Minimap should hang off MinimapCluster");
        assert!(cluster_height > 0.0, "startup MinimapCluster should have a usable size");
    }
}

#[test]
fn test_housing_tutorials_load_without_cvar_bitfield_warning() {
    test_timeout! {
        let (env, warnings) = load_blizzard_addon_by_folder("Blizzard_HousingTutorials");

        let bitfield_warnings: Vec<String> = warnings
            .iter()
            .filter(|warning| {
                warning.contains("GetCVarBitfield")
                    || warning.contains("CvarUtil.lua:30")
            })
            .cloned()
            .collect();

        assert!(
            bitfield_warnings.is_empty(),
            "Blizzard_HousingTutorials should not warn on CVar bitfield helpers:\n  {}",
            bitfield_warnings.join("\n  ")
        );

        let tutorial_seen: bool = env
            .eval(
                r#"
                return C_CVar.GetCVarBitfield(
                    "closedInfoFramesAccountWide",
                    Enum.FrameTutorialAccount.HousingItemAcquisition
                )
                "#,
            )
            .expect("housing tutorial bitfield read should be callable after addon load");
        let _ = tutorial_seen;
    }
}

#[test]
fn test_new_player_experience_loads_without_minimap_cluster_warning() {
    test_timeout! {
        let (env, warnings) = load_blizzard_addon_by_folder("Blizzard_NewPlayerExperience");

        let minimap_cluster_warnings: Vec<String> = warnings
            .iter()
            .filter(|warning| {
                warning.contains("MinimapCluster")
                    || warning.contains("Blizzard_TutorialTutorials.lua:687")
                    || warning.contains("Blizzard_TutorialTutorials.lua:692")
                    || warning.contains("Blizzard_TutorialTutorials.lua:709")
            })
            .cloned()
            .collect();

        assert!(
            minimap_cluster_warnings.is_empty(),
            "Blizzard_NewPlayerExperience should not warn on MinimapCluster startup access:\n  {}",
            minimap_cluster_warnings.join("\n  ")
        );

        let (exists, parent_name): (bool, String) = env
            .eval(
                r#"
                return MinimapCluster ~= nil, MinimapCluster:GetParent():GetName()
                "#,
            )
            .expect("MinimapCluster should be available to startup addons");
        assert!(exists);
        assert_eq!(parent_name, "UIParent");
    }
}

#[test]
fn test_battlefield_map_startup_uses_maputil_displayable_map_helper() {
    test_timeout! {
        let (env, warnings) = load_blizzard_addon_by_folder("Blizzard_BattlefieldMap");

        let load_maputil_warnings: Vec<String> = warnings
            .iter()
            .filter(|warning| {
                warning.contains("MapUtil")
                    || warning.contains("GetDisplayableMapForPlayer")
            })
            .cloned()
            .collect();

        assert!(
            load_maputil_warnings.is_empty(),
            "Blizzard_BattlefieldMap should not warn on MapUtil during load:\n  {}",
            load_maputil_warnings.join("\n  ")
        );

        install_test_error_handler(&env);
        env.exec(
            r#"
            RegisterCVar("showBattlefieldMinimap", "1")
            SetCVar("showBattlefieldMinimap", "1")
            "#,
        )
        .expect("battlefield map cvar should be writable");

        let mut startup_warnings = Vec::new();
        startup_warnings.extend(fire(
            &env,
            "ADDON_LOADED",
            &[env.lua_string("Blizzard_BattlefieldMap")],
        ));
        startup_warnings.extend(fire(
            &env,
            "PLAYER_ENTERING_WORLD",
            &[rilua::Val::Bool(true), rilua::Val::Bool(false)],
        ));

        let maputil_warnings: Vec<String> = startup_warnings
            .iter()
            .filter(|warning| {
                warning.contains("MapUtil")
                    || warning.contains("GetDisplayableMapForPlayer")
                    || warning.contains("Blizzard_BattlefieldMap.lua:154")
                    || warning.contains("Blizzard_BattlefieldMap.lua:189")
            })
            .cloned()
            .collect();

        assert!(
            maputil_warnings.is_empty(),
            "Blizzard_BattlefieldMap startup should not warn on MapUtil:\n  {}",
            maputil_warnings.join("\n  ")
        );

        let (maputil_type, helper_type, displayable_map_id): (String, String, i32) = env
            .eval(
                r#"
                return type(MapUtil),
                       type(MapUtil.GetDisplayableMapForPlayer),
                       MapUtil.GetDisplayableMapForPlayer()
                "#,
            )
            .expect("battlefield map startup should leave MapUtil displayable-map helpers callable");
        assert_eq!(maputil_type, "table");
        assert_eq!(helper_type, "function");
        assert!(displayable_map_id > 0);
    }
}

#[test]
fn test_world_map_loads_without_maputil_warning() {
    test_timeout! {
        let (env, warnings) = load_blizzard_addon_by_folder("Blizzard_WorldMap");

        let maputil_warnings: Vec<String> = warnings
            .iter()
            .filter(|warning| {
                warning.contains("MapUtil")
                    || warning.contains("Blizzard_WorldMap.lua")
                    || warning.contains("Blizzard_WorldMapTemplates.lua")
            })
            .cloned()
            .collect();

        assert!(
            maputil_warnings.is_empty(),
            "Blizzard_WorldMap should not warn on MapUtil startup access:\n  {}",
            maputil_warnings.join("\n  ")
        );

        let (has_displayable_map, has_parent_info): (bool, bool) = env
            .eval(
                r#"
                local mapID = MapUtil.GetDisplayableMapForPlayer()
                return type(mapID) == "number",
                       pcall(function() return MapUtil.GetMapParentInfo(1, Enum.UIMapType.Zone) end)
                "#,
            )
            .expect("MapUtil startup helpers should be available after world map load");
        assert!(has_displayable_map);
        assert!(has_parent_info);
    }
}
