use std::path::PathBuf;

use wow_ui_sim::loader::discover_blizzard_addons_for_screen;
use wow_ui_sim::loader::load_addon;
use wow_ui_sim::lua_api::WowLuaEnv;
use wow_ui_sim::screen::ScreenKind;
use wow_ui_sim::startup::fire_startup_events_for_screen;
use wow_ui_sim::toc::TocFile;

fn blizzard_ui_dir() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("Interface/BlizzardUI")
}

fn environment_cleanup_mainline_toc() -> PathBuf {
    blizzard_ui_dir().join("Blizzard_EnvironmentCleanup/Blizzard_EnvironmentCleanup_Mainline.toc")
}

fn environment_cleanup_classic_toc() -> PathBuf {
    blizzard_ui_dir().join("Blizzard_EnvironmentCleanup/Blizzard_EnvironmentCleanup_Classic.toc")
}

fn load_full_game_ui() -> WowLuaEnv {
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
    env
}

#[test]
fn blizzard_environment_cleanup_mainline_toc_declares_load_first_with_five_deps() {
    let toc = TocFile::from_file(&environment_cleanup_mainline_toc())
        .expect("Blizzard_EnvironmentCleanup_Mainline TOC should parse");

    assert!(
        !toc.is_load_on_demand(),
        "Blizzard_EnvironmentCleanup omits `## LoadOnDemand:` — must auto-load on Game-screen \
         bring-up so the secure-environment globals are nil-ed before any insecure addon Lua \
         executes"
    );
    assert!(
        !toc.is_secure_env(),
        "Blizzard_EnvironmentCleanup does not declare `## UseSecureEnvironment` — runs in the \
         standard taint environment (the whole point is to remove secure-only references from \
         the standard environment so insecure addons can't reach them)"
    );

    let deps = toc.dependencies();
    assert_eq!(
        deps,
        &[
            "Blizzard_FrameXML".to_string(),
            "Blizzard_ActionBar".to_string(),
            "Blizzard_UnitFrame".to_string(),
            "Blizzard_UIPanels_Game".to_string(),
            "Blizzard_ChatFrame".to_string(),
        ],
        "Mainline TOC declares five hard dependencies in this order: Blizzard_FrameXML, \
         Blizzard_ActionBar, Blizzard_UnitFrame, Blizzard_UIPanels_Game, Blizzard_ChatFrame — \
         topological sort must place this addon AFTER all five so the cleanup pass nils out \
         globals that those addons may have published into the standard environment"
    );

    assert!(
        !toc.is_game_type_restricted(),
        "Blizzard_EnvironmentCleanup_Mainline declares `## AllowLoadGameType: mainline` — \
         `mainline` is the standard mainline-retail token (src/toc.rs:298-299), so the addon \
         is NOT considered game-type-restricted on retail"
    );

    let toc_text = std::fs::read_to_string(environment_cleanup_mainline_toc())
        .expect("Mainline TOC should read");
    assert!(
        toc_text.contains("## LoadFirst: 1"),
        "Mainline TOC declares `## LoadFirst: 1` — within its dependency tier the loader \
         should give this addon priority so the cleanup runs as early as possible"
    );
    assert!(
        toc_text.contains("## OptionalDeps: Blizzard_RestrictedAddOnEnvironment"),
        "Mainline TOC declares `## OptionalDeps: Blizzard_RestrictedAddOnEnvironment` — when \
         RestrictedAddOnEnvironment is also loaded it should load first, but its absence \
         must not prevent EnvironmentCleanup from running"
    );
    assert!(
        toc_text.contains("## AllowLoad: Game"),
        "Mainline TOC declares `## AllowLoad: Game` — confines the cleanup pass to the \
         in-game screen (the secure-only globals exist on game-side, not glue/login)"
    );
    assert!(
        toc_text.contains("## DefaultState: enabled"),
        "Mainline TOC declares `## DefaultState: enabled` — security cleanup runs by default, \
         no user opt-in"
    );
}

#[test]
fn blizzard_environment_cleanup_classic_toc_declares_three_deps_and_classic_game_type() {
    let toc = TocFile::from_file(&environment_cleanup_classic_toc())
        .expect("Blizzard_EnvironmentCleanup_Classic TOC should parse");

    let deps = toc.dependencies();
    assert_eq!(
        deps,
        &[
            "Blizzard_FrameXML".to_string(),
            "Blizzard_ActionBar".to_string(),
            "Blizzard_UIPanels_Game".to_string(),
        ],
        "Classic TOC declares only THREE deps (Blizzard_FrameXML, Blizzard_ActionBar, \
         Blizzard_UIPanels_Game) — Classic-era retail did not ship Blizzard_UnitFrame or \
         Blizzard_ChatFrame as separate addons, so those drop out of the dep list"
    );

    assert!(
        toc.is_game_type_restricted(),
        "Classic TOC declares `## AllowLoadGameType: classic` — NOT a mainline token, so the \
         addon IS game-type-restricted on standard retail and discover_blizzard_addons_for_screen \
         filters it out at src/loader/mod.rs:527"
    );
}

#[test]
fn blizzard_environment_cleanup_picks_mainline_toc_via_find_toc_file() {
    let game_addons = discover_blizzard_addons_for_screen(&blizzard_ui_dir(), ScreenKind::Game);
    let entry = game_addons
        .iter()
        .find(|(name, _)| name == "Blizzard_EnvironmentCleanup")
        .expect(
            "Blizzard_EnvironmentCleanup must be present in Game-screen discovery — the \
             Mainline TOC has `mainline` game type and `AllowLoad: Game`",
        );

    let toc_filename = entry
        .1
        .file_name()
        .expect("TOC path must have a filename component")
        .to_str()
        .expect("TOC filename must be valid UTF-8");
    assert_eq!(
        toc_filename, "Blizzard_EnvironmentCleanup_Mainline.toc",
        "find_toc_file (src/loader/mod.rs:65-95) must pick `Blizzard_EnvironmentCleanup_Mainline.toc` \
         over `Blizzard_EnvironmentCleanup_Classic.toc` because it tries the `_Mainline.toc` \
         variant first by addon-name pattern"
    );
}

#[test]
fn blizzard_environment_cleanup_appears_in_game_discovery_only() {
    let game_addons = discover_blizzard_addons_for_screen(&blizzard_ui_dir(), ScreenKind::Game);
    let in_game = game_addons
        .iter()
        .any(|(name, _)| name == "Blizzard_EnvironmentCleanup");
    assert!(
        in_game,
        "Blizzard_EnvironmentCleanup (Mainline TOC, `AllowLoad: Game`, mainline game type) \
         must auto-discover on the Game screen"
    );

    let login_addons = discover_blizzard_addons_for_screen(&blizzard_ui_dir(), ScreenKind::Login);
    let in_login = login_addons
        .iter()
        .any(|(name, _)| name == "Blizzard_EnvironmentCleanup");
    assert!(
        !in_login,
        "Blizzard_EnvironmentCleanup must NOT appear on Login / glue screens — `AllowLoad: \
         Game` confines the cleanup to the in-game environment"
    );
}

#[test]
fn blizzard_environment_cleanup_loads_after_all_five_mainline_dependencies() {
    let game_addons = discover_blizzard_addons_for_screen(&blizzard_ui_dir(), ScreenKind::Game);
    let cleanup_index = game_addons
        .iter()
        .position(|(name, _)| name == "Blizzard_EnvironmentCleanup")
        .expect("Blizzard_EnvironmentCleanup must be present in Game-screen discovery");

    let dep_names = [
        "Blizzard_FrameXML",
        "Blizzard_ActionBar",
        "Blizzard_UnitFrame",
        "Blizzard_UIPanels_Game",
        "Blizzard_ChatFrame",
    ];
    for dep_name in dep_names {
        let dep_index = game_addons
            .iter()
            .position(|(name, _)| name == dep_name)
            .unwrap_or_else(|| panic!("{dep_name} must be present in Game-screen discovery"));
        assert!(
            dep_index < cleanup_index,
            "topological_sort_addons must place {dep_name} (index {dep_index}) BEFORE \
             Blizzard_EnvironmentCleanup (index {cleanup_index}) — the cleanup nils out \
             secure-only globals that the dep addons may have published"
        );
    }
}

#[test]
fn blizzard_environment_cleanup_loads_without_errors() {
    let env = load_full_game_ui();

    let addon_errors: Vec<String> = env
        .state()
        .borrow()
        .lua_errors
        .iter()
        .filter(|message| message.contains("EnvironmentCleanup"))
        .cloned()
        .collect();
    assert!(
        addon_errors.is_empty(),
        "Blizzard_EnvironmentCleanup emitted Lua errors during Game-screen load:\n  {}",
        addon_errors.join("\n  ")
    );
}

#[test]
fn blizzard_environment_cleanup_is_addon_loaded_returns_true() {
    let env = load_full_game_ui();

    let loaded: bool = env
        .eval("return C_AddOns.IsAddOnLoaded('Blizzard_EnvironmentCleanup') and true or false")
        .expect("C_AddOns.IsAddOnLoaded probe should succeed");
    assert!(
        loaded,
        "C_AddOns.IsAddOnLoaded('Blizzard_EnvironmentCleanup') must return true after the \
         addon auto-loads on the Game screen"
    );
}

#[test]
fn blizzard_environment_cleanup_secure_namespaces_get_restored_post_load() {
    // EnvironmentCleanup.lua nils `C_StoreSecure`/`C_AuthChallenge`/`C_SecureTransfer`/
    // `C_WowTokenSecure` (lines 3, 5, 6, 224), but the simulator's per-addon post-load hook
    // `patch_environment_cleanup` (src/loader/addon.rs:169) calls `restore_post_cleanup_globals`
    // immediately after this addon loads — it re-runs `init_runtime_surface_bootstrap` /
    // `register_globals` so the simulator's own stubs and downstream Blizzard addons can still
    // invoke these C_* APIs. Capturing this contract here so the restoration is not silently
    // dropped: if any of the four falls back to nil, the per-addon hook regressed.
    let env = load_full_game_ui();

    let restored: (bool, bool, bool, bool) = env
        .eval(
            "return C_StoreSecure ~= nil, \
                    C_AuthChallenge ~= nil, \
                    C_SecureTransfer ~= nil, \
                    C_WowTokenSecure ~= nil",
        )
        .expect("secure-only namespace probe should succeed");
    assert_eq!(
        restored,
        (true, true, true, true),
        "After Blizzard_EnvironmentCleanup loads, the simulator's `patch_environment_cleanup` \
         hook (src/loader/addon.rs:169) must restore C_StoreSecure / C_AuthChallenge / \
         C_SecureTransfer / C_WowTokenSecure so the rest of the runtime keeps working. If any \
         entry is nil, the restoration regressed and downstream addons calling \
         `C_StoreSecure.GetProductInfo` (etc.) will throw"
    );
}

#[test]
fn blizzard_environment_cleanup_create_secure_delegate_gets_restored_post_load() {
    // EnvironmentCleanup.lua nils a list of secure-factory helpers (CreateForbiddenFrame line
    // 4, SecureMixin / CreateFromSecureMixins / CreateSecureDelegate / CreateSecureMixinCopy
    // lines 275-278, loadstring_untainted line 279, secretunwrap line 280). Of those,
    // `CreateSecureDelegate` is provided by the simulator's temporary debug/environment
    // workaround and `loadstring_untainted` is provided by the taint bootstrap. Capture the
    // current restored surface here so EnvironmentCleanup does not silently remove simulator
    // glue that downstream secure/restricted code depends on.
    let env = load_full_game_ui();

    let create_secure_delegate_restored: bool = env
        .eval("return CreateSecureDelegate ~= nil")
        .expect("CreateSecureDelegate probe should succeed");
    assert!(
        create_secure_delegate_restored,
        "After Blizzard_EnvironmentCleanup loads, `patch_environment_cleanup` (src/loader/\
         addon.rs:169) must restore CreateSecureDelegate via the temporary debug/environment workaround — \
         simulator-side glue that wraps insecure functions as secure delegates depends on it"
    );

    let loadstring_untainted_restored: bool = env
        .eval("return type(loadstring_untainted) == 'function'")
        .expect("loadstring_untainted probe should succeed");
    assert!(
        loadstring_untainted_restored,
        "After Blizzard_EnvironmentCleanup loads, loadstring_untainted must still expose the \
         original untainted compiler function for restricted execution helpers"
    );

    let unseeded_helpers_remain_nil: (bool, bool, bool, bool, bool) = env
        .eval(
            "return CreateForbiddenFrame == nil, \
                    SecureMixin == nil, \
                    CreateFromSecureMixins == nil, \
                    CreateSecureMixinCopy == nil, \
                    secretunwrap == nil",
        )
        .expect("unseeded-helper probe should succeed");
    assert_eq!(
        unseeded_helpers_remain_nil,
        (true, true, true, true, true),
        "Simulator does not seed CreateForbiddenFrame / SecureMixin / CreateFromSecureMixins / \
         CreateSecureMixinCopy / secretunwrap (no entries in \
         runtime_surface_bootstrap.lua / shared_bootstrap.lua / temporary workarounds), so EnvironmentCleanup.lua's \
         nil assignments are effectively a no-op and the post-load restoration leaves them \
         nil. If any becomes non-nil, somebody added a simulator stub and this test should \
         flip to verify the restoration path covers it"
    );
}

#[test]
fn blizzard_environment_cleanup_nils_out_blizzard_store_localization_strings() {
    let env = load_full_game_ui();

    let store_strings_nil: (bool, bool, bool, bool, bool) = env
        .eval(
            "return BLIZZARD_STORE_BUY == nil, \
                    BLIZZARD_STORE_CONFIRMATION_TITLE == nil, \
                    BLIZZARD_STORE_PAYMENT_METHOD == nil, \
                    BLIZZARD_STORE_VAS_ERROR_OTHER == nil, \
                    BLIZZARD_STORE_BUNDLE_DISCOUNT_BANNER == nil",
        )
        .expect("BLIZZARD_STORE_* probe should succeed");
    assert_eq!(
        store_strings_nil,
        (true, true, true, true, true),
        "EnvironmentCleanup.lua nils out 100+ `BLIZZARD_STORE_*` localization strings (lines \
         7-223). Spot-check five spread across the file (BUY @ line 9, CONFIRMATION_TITLE @ \
         16, PAYMENT_METHOD @ 19, VAS_ERROR_OTHER @ 198, BUNDLE_DISCOUNT_BANNER @ 218) — all \
         must read as nil after load"
    );
}

#[test]
fn blizzard_environment_cleanup_nils_out_vas_and_token_localization_strings() {
    let env = load_full_game_ui();

    let nils: (bool, bool, bool, bool, bool) = env
        .eval(
            "return VAS_SELECT_CHARACTER == nil, \
                    VAS_NAME_CHANGE_CONFIRMATION == nil, \
                    TOKEN_REDEEM_LABEL == nil, \
                    TOKEN_CONFIRMATION_TITLE == nil, \
                    BLIZZARD_CHALLENGE_SUBMIT == nil",
        )
        .expect("VAS/TOKEN/CHALLENGE string probe should succeed");
    assert_eq!(
        nils,
        (true, true, true, true, true),
        "EnvironmentCleanup.lua also nils out the VAS_* / TOKEN_* / BLIZZARD_CHALLENGE_* \
         localization strings (VAS_SELECT_CHARACTER line 125, VAS_NAME_CHANGE_CONFIRMATION \
         line 146, TOKEN_REDEEM_LABEL line 225, TOKEN_CONFIRMATION_TITLE line 231, \
         BLIZZARD_CHALLENGE_SUBMIT line 260)"
    );
}

#[test]
fn blizzard_environment_cleanup_nils_out_secure_only_enum_namespaces() {
    let env = load_full_game_ui();

    let nils: (bool, bool, bool, bool, bool, bool, bool, bool, bool, bool) = env
        .eval(
            "return Enum.StoreError == nil, \
                    Enum.VasTransactionPurchaseResult == nil, \
                    Enum.BattlepayBoostProduct == nil, \
                    Enum.BattlepayDisplayFlags == nil, \
                    Enum.PurchaseEligibility == nil, \
                    Enum.BattlepayProductDecorator == nil, \
                    Enum.VasServiceType == nil, \
                    Enum.VasPurchaseState == nil, \
                    Enum.BattlepayProductGroupFlags == nil, \
                    Enum.BattlepayGroupDisplayType == nil",
        )
        .expect("secure-only enum probe should succeed");
    assert_eq!(
        nils,
        (true, true, true, true, true, true, true, true, true, true),
        "EnvironmentCleanup.lua's final 12 lines (281-292) nil out the secure-only `Enum.*` \
         sub-tables. Spot-check 10 of them: Enum.StoreError, Enum.VasTransactionPurchaseResult, \
         Enum.BattlepayBoostProduct, Enum.BattlepayDisplayFlags, Enum.PurchaseEligibility, \
         Enum.BattlepayProductDecorator, Enum.VasServiceType, Enum.VasPurchaseState, \
         Enum.BattlepayProductGroupFlags, Enum.BattlepayGroupDisplayType — all must read as \
         nil after the cleanup pass"
    );

    let trailing_nils: (bool, bool) = env
        .eval(
            "return Enum.BattlepayCardType == nil, \
                    Enum.BattlepayBannerType == nil",
        )
        .expect("trailing battlepay enum probe should succeed");
    assert_eq!(
        trailing_nils,
        (true, true),
        "Final two enum nils (lines 291-292): Enum.BattlepayCardType + Enum.BattlepayBannerType \
         must also read as nil after the cleanup pass"
    );
}

#[test]
fn blizzard_environment_cleanup_dir_lists_classic_subdirectory_with_its_own_lua() {
    let dir = blizzard_ui_dir().join("Blizzard_EnvironmentCleanup");
    let entries: Vec<String> = std::fs::read_dir(&dir)
        .expect("Blizzard_EnvironmentCleanup dir should read")
        .flatten()
        .filter_map(|e| e.file_name().into_string().ok())
        .collect();

    assert!(
        entries.iter().any(|n| n == "Classic"),
        "Blizzard_EnvironmentCleanup must ship a `Classic/` subdirectory with the \
         Classic-era cleanup script. Got entries: {entries:?}"
    );

    let classic_lua = dir.join("Classic/EnvironmentCleanup.lua");
    assert!(
        classic_lua.exists(),
        "Classic/EnvironmentCleanup.lua must exist — referenced by the Classic TOC's file \
         list"
    );

    let mainline_lua = dir.join("EnvironmentCleanup.lua");
    assert!(
        mainline_lua.exists(),
        "Top-level EnvironmentCleanup.lua must exist — referenced by the Mainline TOC's file \
         list"
    );
}
