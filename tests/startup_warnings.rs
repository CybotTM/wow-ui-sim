//! Test that loading Blizzard addons and firing startup events produces no warnings.

mod common;

use std::path::PathBuf;
use wow_ui_sim::loader::{discover_all_blizzard_addons, discover_blizzard_addons, load_addon};
use wow_ui_sim::lua_api::WowLuaEnv;

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

/// Load all Blizzard addons and fire startup events, collecting all warnings.
fn load_and_startup() -> Vec<String> {
    let env = WowLuaEnv::new().expect("Failed to create Lua environment");
    env.set_screen_size(1024.0, 768.0);

    let ui = blizzard_ui_dir();
    let addons = discover_blizzard_addons(&ui);
    let mut warnings = Vec::new();

    // Load addons
    for (name, toc_path) in &addons {
        match load_addon(&env.loader_env(), toc_path) {
            Ok(r) => {
                for w in r.warnings {
                    warnings.push(format!("[load {name}] {w}"));
                }
            }
            Err(e) => {
                warnings.push(format!("[load {name}] FAILED: {e}"));
            }
        }
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

        if count < KNOWN_WARNING_COUNT {
            panic!(
                "Warning count improved from {KNOWN_WARNING_COUNT} to {count}! \
                 Update KNOWN_WARNING_COUNT to {count} to lock in the improvement."
            );
        }
    }
}

/// Load all Blizzard addons and apply workarounds (no startup events).
fn load_all_addons() -> WowLuaEnv {
    let env = WowLuaEnv::new().expect("Failed to create Lua environment");
    env.set_screen_size(1024.0, 768.0);

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

    let ui = blizzard_ui_dir();
    let addons = discover_blizzard_addons(&ui);
    let (_, toc_path) = addons
        .into_iter()
        .find(|(name, _)| name == addon_name)
        .unwrap_or_else(|| panic!("addon {addon_name} should exist"));
    let result = load_addon(&env.loader_env(), &toc_path)
        .unwrap_or_else(|error| panic!("{addon_name} should load: {error}"));
    (env, result.warnings)
}

fn load_blizzard_addon_by_folder(folder_name: &str) -> (WowLuaEnv, Vec<String>) {
    let env = WowLuaEnv::new().expect("Failed to create Lua environment");
    env.set_screen_size(1024.0, 768.0);

    let ui = blizzard_ui_dir();
    let addons = discover_all_blizzard_addons(&ui);
    let (_, toc_path) = addons
        .into_iter()
        .find(|(name, _)| name == folder_name)
        .unwrap_or_else(|| panic!("addon folder {folder_name} should exist"));
    let result = load_addon(&env.loader_env(), &toc_path)
        .unwrap_or_else(|error| panic!("{folder_name} should load: {error}"));
    (env, result.warnings)
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
            get_object_type_ty,
            set_point_ty,
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
