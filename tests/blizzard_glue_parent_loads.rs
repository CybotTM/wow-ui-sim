#![cfg(feature = "client-retail")]
use std::path::PathBuf;


use wow_ui_sim::loader::{discover_blizzard_addons_for_screen, find_toc_file, load_addon};
use wow_ui_sim::lua_api::WowLuaEnv;
use wow_ui_sim::paths::default_blizzard_ui_addons_path;
use wow_ui_sim::screen::ScreenKind;
use wow_ui_sim::startup::settle_headless_startup;
use wow_ui_sim::toc::TocFile;

fn blizzard_ui_dir() -> std::path::PathBuf {
    default_blizzard_ui_addons_path().expect("Blizzard UI cache should be synced")

}

fn glue_parent_dir() -> std::path::PathBuf {
    blizzard_ui_dir().join("Blizzard_GlueParent")
}

fn glue_parent_toc() -> std::path::PathBuf {
    glue_parent_dir().join("Blizzard_GlueParent_Mainline.toc")
}

fn load_character_select_screen() -> WowLuaEnv {
    let env = WowLuaEnv::new().expect("Failed to create Lua environment");
    env.set_screen_size(1024.0, 768.0);
    env.set_screen_mode(ScreenKind::CharacterSelect);

    {
        let mut state = env.state().borrow_mut();
        state.addon_base_paths = vec![blizzard_ui_dir()];
    }

    wow_ui_sim::xml::register_intrinsic_templates();

    let ui = blizzard_ui_dir();
    let addons = discover_blizzard_addons_for_screen(&ui, ScreenKind::CharacterSelect);
    for (name, toc_path) in &addons {
        load_addon(&env.loader_env(), toc_path)
            .unwrap_or_else(|err| panic!("[load {name}] FAILED: {err}"));
    }

    env.apply_post_load_workarounds();
    settle_headless_startup(&env);
    env
}

#[test]
fn blizzard_glue_parent_find_toc_resolves_mainline_variant() {
    let resolved =
        find_toc_file(&glue_parent_dir()).expect("Blizzard_GlueParent TOC should resolve");
    assert_eq!(
        resolved,
        glue_parent_toc(),
        "Blizzard_GlueParent ships only the `_Mainline.toc` variant — `find_toc_file` \
         (src/loader/mod.rs:65) finds it on the first pass via the `_Mainline.toc` suffix \
         lookup. There is no Classic-flavor TOC; Classic builds use a separate addon entirely"
    );
}

#[test]
fn blizzard_glue_parent_toc_declares_load_first_glue_with_glue_xml_base_and_static_popup_deps() {
    let toc = TocFile::from_file(&glue_parent_toc())
        .expect("Blizzard_GlueParent_Mainline TOC should parse");
    assert!(
        !toc.is_load_on_demand(),
        "Blizzard_GlueParent is non-LoadOnDemand — the GlueParent frame is the root of every \
         glue screen so it must auto-load before any other glue addon's XML tries to resolve \
         `parent=GlueParent`"
    );
    assert!(
        toc.is_load_first(),
        "Blizzard_GlueParent declares `## LoadFirst: 1` so the GlueParent root frame + the \
         UIParent alias (set in GlueParentMixin:OnLoad via `UIParent = self`) install before \
         the bulk of the glue-screen addons load — Blizzard_GlueXML / Blizzard_GlueMenuFrame / \
         Blizzard_CharacterSelectNavBar all reference `UIParent` or `GlueParent` directly in \
         their XML or OnLoad path"
    );
    assert!(
        !toc.is_secure_env(),
        "Blizzard_GlueParent does not declare UseSecureEnvironment — the addToSecureEnv hint \
         is on the inner ScopedModifier XML element, not a TOC-level secure flag"
    );
    let deps = toc.dependencies();
    assert_eq!(
        deps,
        vec![
            "Blizzard_GlueXMLBase".to_string(),
            "Blizzard_StaticPopup_Glue".to_string(),
        ],
        "Blizzard_GlueParent_Mainline declares exactly two deps: Blizzard_GlueXMLBase \
         (provides the `CallbackRegistrantTemplate` the GlueParent frame inherits from + the \
         `ScopedModifier` / `addToSecureEnv` XML elements + the `LE_AURORA_STATE_*` and \
         `LE_WOW_CONNECTION_STATE_*` constants used by GlueParent_IsScreenValid) and \
         Blizzard_StaticPopup_Glue (provides `StaticPopup_Show` consumed by the OPEN_STATUS_DIALOG \
         and SUBSCRIPTION_CHANGED_KICK_IMMINENT event branches in GlueParentMixin:OnEvent)"
    );
}

#[test]
fn blizzard_glue_parent_toc_declares_glue_screen_mainline_only_with_error_escalation() {
    let toc_text = std::fs::read_to_string(glue_parent_toc())
        .expect("Blizzard_GlueParent_Mainline TOC should read");
    assert!(
        toc_text.contains("## AllowLoad: Glue"),
        "Blizzard_GlueParent declares `## AllowLoad: Glue` (capital G — glue-screen-only). The \
         in-game UIParent surface is provided by the separate Blizzard_UIParent addon"
    );
    assert!(
        toc_text.contains("## AllowLoadGameType: mainline"),
        "Blizzard_GlueParent declares `## AllowLoadGameType: mainline` so the addon loads on \
         retail only"
    );
    assert!(
        toc_text.contains("## DefaultState: enabled"),
        "Blizzard_GlueParent declares `## DefaultState: enabled` — the glue-screen root must \
         be enabled by default, no opt-in toggle"
    );
    assert!(
        toc_text.contains("## EscalateErrorDuringLoad: 1"),
        "Blizzard_GlueParent declares `## EscalateErrorDuringLoad: 1` (raw substring — the \
         simulator's TOC parser doesn't model this attribute as a typed accessor; this is the \
         only Blizzard addon that escalates load errors because a broken GlueParent leaves the \
         player stuck on a black glue screen with no way to quit)"
    );
}

#[test]
fn blizzard_glue_parent_toc_lists_three_files_with_shared_util_first() {
    let toc_text = std::fs::read_to_string(glue_parent_toc())
        .expect("Blizzard_GlueParent_Mainline TOC should read");
    let body_lines: Vec<&str> = toc_text
        .lines()
        .map(str::trim)
        .filter(|line| !line.is_empty() && !line.starts_with('#'))
        .collect();

    assert_eq!(
        body_lines,
        vec![
            "Shared\\GlueParentUtil.lua",
            "Mainline\\GlueParent.lua",
            "Mainline\\GlueParent.xml",
        ],
        "Blizzard_GlueParent_Mainline TOC body lists exactly 3 files. The flavor-shared \
         Shared\\GlueParentUtil.lua MUST come first because it publishes the modal-frame stack \
         helpers (GlueParent_AddModalFrame / GlueParent_RemoveModalFrame) and the photosensitivity \
         check that Mainline\\GlueParent.lua's GlueParentMixin:OnEvent path immediately invokes \
         on the FRAMES_LOADED event"
    );
}

#[test]
fn blizzard_glue_parent_appears_in_all_three_glue_screens_and_absent_from_game() {
    for screen in [
        ScreenKind::Login,
        ScreenKind::CharacterSelect,
        ScreenKind::CharacterCreate,
    ] {
        let addons = discover_blizzard_addons_for_screen(&blizzard_ui_dir(), screen);
        let in_screen = addons.iter().any(|(name, _)| name == "Blizzard_GlueParent");
        assert!(
            in_screen,
            "Blizzard_GlueParent (## AllowLoad: Glue) must appear in {screen:?}-screen \
             auto-discovery — every glue screen needs the GlueParent root frame as its ancestor; \
             without it `parent=GlueParent` references in dependent addons (GlueMenuFrame, \
             AccountLogin, CharacterSelect, CharacterCreate) would resolve to nil"
        );
    }

    let game_addons = discover_blizzard_addons_for_screen(&blizzard_ui_dir(), ScreenKind::Game);
    let in_game = game_addons
        .iter()
        .any(|(name, _)| name == "Blizzard_GlueParent");
    assert!(
        !in_game,
        "Blizzard_GlueParent must NOT appear in Game-screen auto-discovery — the in-game \
         UIParent surface is owned by Blizzard_UIParent. Loading both would aliasing-clash on \
         the `UIParent = self` line in GlueParentMixin:OnLoad"
    );
}

#[test]
fn blizzard_glue_parent_loads_without_addon_specific_errors() {
    let env = load_character_select_screen();

    let parent_errors: Vec<String> = env
        .state()
        .borrow()
        .lua_errors
        .iter()
        .filter(|message| {
            (message.contains("GlueParent") || message.contains("GlueParentMixin"))
                && !message.contains("GlueParentScreenFrame")
        })
        .cloned()
        .collect();
    assert!(
        parent_errors.is_empty(),
        "Blizzard_GlueParent emitted Lua errors during CharacterSelect-screen load:\n  {}",
        parent_errors.join("\n  ")
    );
}

#[test]
fn blizzard_glue_parent_publishes_glue_parent_frame_with_blocking_and_screen_subframes() {
    let env = load_character_select_screen();

    let parent_present: bool = env
        .eval("return GlueParent ~= nil and type(GlueParent.IsShown) == 'function'")
        .expect("GlueParent frame query should succeed");
    assert!(
        parent_present,
        "Mainline/GlueParent.xml line 4 declares `<Frame name=\"GlueParent\">` so the frame \
         publishes as a global with all standard frame methods. The frame inherits \
         CallbackRegistrantTemplate (provided by Blizzard_GlueXMLBase) and is marked \
         setAllPoints=true so it covers the entire glue-screen viewport"
    );

    let screen_frame_attached: bool = env
        .eval(
            "return type(GlueParent.ScreenFrame) == 'table' \
             and GlueParent.ScreenFrame:GetName() == 'GlueParentScreenFrame'",
        )
        .expect("ScreenFrame parentKey query should succeed");
    assert!(
        screen_frame_attached,
        "Mainline/GlueParent.xml line 6 declares the `<Frame parentKey=\"ScreenFrame\" \
         name=\"GlueParentScreenFrame\" setAllPoints=\"true\">` child — the GlueParent root \
         exposes the ScreenFrame parentKey so callers like GlueParent_SetScreen can swap the \
         visible primary screen (AccountLogin / CharacterSelect / CharacterCreate / etc.) by \
         reparenting them under GlueParent.ScreenFrame"
    );

    let blocking_frame_attached: bool = env
        .eval(
            "return type(GlueParent.BlockingFrame) == 'table' \
             and type(GlueParent.BlockingFrame.IsShown) == 'function' \
             and not GlueParent.BlockingFrame:IsShown()",
        )
        .expect("BlockingFrame parentKey query should succeed");
    assert!(
        blocking_frame_attached,
        "Mainline/GlueParent.xml line 27 declares the `<Frame parentKey=\"BlockingFrame\" \
         setAllPoints=\"true\" frameStrata=\"HIGH\" frameLevel=\"10000\" hidden=\"true\" \
         enableMouse=\"true\">` overlay — the modal-stack helper GlueParent_AddModalFrame / \
         GlueParent_RemoveModalFrame in Shared/GlueParentUtil.lua toggles \
         GlueParent.BlockingFrame:Show()/:Hide() based on whether the modal-frame stack is \
         non-empty. It must start hidden because no modal frames are open before any addon \
         registers one"
    );
}

#[test]
fn blizzard_glue_parent_on_load_has_run_and_registered_ui_parent_alias_intent() {
    let env = load_character_select_screen();

    let on_load_ran: bool = env
        .eval(
            "return type(GlueParent) == 'table' \
             and type(GlueParent.BlockingFrame) == 'table' \
             and type(GlueParent.ScreenFrame) == 'table'",
        )
        .expect("GlueParent post-OnLoad shape query should succeed");
    assert!(
        on_load_ran,
        "After load the GlueParent frame must expose its BlockingFrame and ScreenFrame \
         parentKey children — proves the XML body resolved and GlueParentMixin:OnLoad ran. \
         The OnLoad path also performs `UIParent = self` (Mainline/GlueParent.lua line 74) \
         to alias UIParent on glue screens; the simulator separately keeps a pre-created \
         UIParent for in-game compatibility, so we don't assert pointer identity here, but \
         we do verify the OnLoad path completed without erroring (any error during OnLoad \
         would leave the parentKey children unattached and trip the `## EscalateErrorDuringLoad: \
         1` flag)"
    );

    let kiosk_check_executed: bool = env
        .eval("return type(Kiosk) == 'table' and type(Kiosk.IsEnabled) == 'function'")
        .expect("Kiosk surface query should succeed");
    assert!(
        kiosk_check_executed,
        "GlueParentMixin:OnLoad invokes `Kiosk.IsEnabled()` directly (Mainline/GlueParent.lua \
         line 102) — the Kiosk namespace must be defined before GlueParent.OnLoad runs (it is, \
         via the simulator's pre-bootstrap surface). If Kiosk were nil, OnLoad would error and \
         the parentKey children check above would fail too"
    );
}

#[test]
fn blizzard_glue_parent_publishes_mixin_with_five_lifecycle_handlers() {
    let env = load_character_select_screen();

    let mixin_present: bool = env
        .eval("return type(GlueParentMixin) == 'table'")
        .expect("GlueParentMixin query should succeed");
    assert!(
        mixin_present,
        "Mainline/GlueParent.lua line 70 publishes `GlueParentMixin = {{}}` — bound by the \
         GlueParent XML via `mixin=GlueParentMixin`"
    );

    for handler in [
        "OnLoad",
        "OnEvent",
        "OnSecondaryScreenClosed",
        "OnAddonListClosed",
        "OnStoreFrameClosed",
    ] {
        let has_handler: bool = env
            .eval(&format!(
                "return type(GlueParentMixin.{handler}) == 'function'"
            ))
            .expect("mixin-handler query should succeed");
        assert!(
            has_handler,
            "GlueParentMixin.{handler} should be a function after load — the mixin owns 5 \
             handlers total: OnLoad (aliases UIParent + registers 13 glue-relevant events + \
             wires AddStaticEventMethod for 3 EventRegistry channels), OnEvent (the 11-branch \
             event router for FRAMES_LOADED / LOGIN_STATE_CHANGED / OPEN_STATUS_DIALOG / \
             DISPLAY_SIZE_CHANGED / UI_SCALE_CHANGED / SUBSCRIPTION_CHANGED_KICK_IMMINENT / \
             ACTIVE_GAME_MODE_UPDATED / CONNECT_TO_EVENT_REALM_FAILED / GLOBAL_MOUSE_DOWN / \
             GLOBAL_MOUSE_UP / SCRIPTED_ANIMATIONS_UPDATE / KIOSK_ENABLED / \
             NOTCHED_DISPLAY_MODE_CHANGED), and the 3 EventRegistry callbacks \
             OnSecondaryScreenClosed / OnAddonListClosed / OnStoreFrameClosed (each routes back \
             to GlueMenuFrameUtil.ShowMenu when appropriate to re-raise the ESC menu)"
        );
    }
}

#[test]
fn blizzard_glue_parent_publishes_screen_dispatch_tables_with_six_primary_and_five_secondary() {
    let env = load_character_select_screen();

    let primary_keys: Vec<String> = env
        .eval(
            "local out = {}; for k in pairs(GLUE_SCREENS) do table.insert(out, k) end; \
             table.sort(out); return out",
        )
        .expect("GLUE_SCREENS keys query should succeed");
    assert_eq!(
        primary_keys,
        vec![
            "charcreate".to_string(),
            "charselect".to_string(),
            "kioskmodesplash".to_string(),
            "login".to_string(),
            "plunderstorm".to_string(),
            "realmlist".to_string(),
        ],
        "GLUE_SCREENS (Mainline/GlueParent.lua line 1) lists exactly 6 primary glue screens. \
         Each entry has a `frame` field naming the global to swap into GlueParent.ScreenFrame \
         (AccountLogin / RealmListUI / CharacterSelect / PlunderstormLobbyFrame / \
         CharacterCreateFrame / KioskModeSplash) plus playMusic / playAmbience flags. The \
         charselect / plunderstorm / charcreate entries each declare an `onAttemptShow` callback \
         that invokes InitializeCharacterScreenData(); plunderstorm additionally sets allowChat=true"
    );

    let secondary_keys: Vec<String> = env
        .eval(
            "local out = {}; for k in pairs(GLUE_SECONDARY_SCREENS) do \
             table.insert(out, k) end; table.sort(out); return out",
        )
        .expect("GLUE_SECONDARY_SCREENS keys query should succeed");
    assert_eq!(
        secondary_keys,
        vec![
            "cinematics".to_string(),
            "credits".to_string(),
            "movie".to_string(),
            "options".to_string(),
            "photosensitivity".to_string(),
        ],
        "GLUE_SECONDARY_SCREENS (Mainline/GlueParent.lua line 10) lists exactly 5 secondary \
         (overlay) screens. Each entry has frame / playMusic / playAmbience / fullScreen and \
         optional showSound + checkFit fields. The `movie` entry intentionally omits showSound \
         to work around bug 477070 (sound-engine crash race when MovieFrame shows while the \
         movie audio is starting)"
    );

    let charselect_frame: String = env
        .eval("return GLUE_SCREENS['charselect'].frame")
        .expect("charselect frame query should succeed");
    assert_eq!(
        charselect_frame, "CharacterSelect",
        "GLUE_SCREENS.charselect.frame names the `CharacterSelect` global frame — that's the \
         primary screen swapped into GlueParent.ScreenFrame when the player connects to a \
         realm; mismatched naming here would leave the character-select screen black"
    );
}

#[test]
fn blizzard_glue_parent_publishes_disconnect_error_code_constants() {
    let env = load_character_select_screen();

    let suspended: f64 = env
        .eval("return ACCOUNT_SUSPENDED_ERROR_CODE")
        .expect("ACCOUNT_SUSPENDED_ERROR_CODE query should succeed");
    assert_eq!(
        suspended, 53.0,
        "Mainline/GlueParent.lua line 20 sets `ACCOUNT_SUSPENDED_ERROR_CODE = 53` — consumed \
         by GlueParent_UpdateDialogs to detect the BNet suspension error path so the dialog \
         can format the remaining-suspension-time string via \
         C_Login.GetAccountSuspensionRemainingTime"
    );

    let disconnect: f64 = env
        .eval("return GENERIC_DISCONNECTED_ERROR_CODE")
        .expect("GENERIC_DISCONNECTED_ERROR_CODE query should succeed");
    assert_eq!(
        disconnect, 319.0,
        "Mainline/GlueParent.lua line 21 sets `GENERIC_DISCONNECTED_ERROR_CODE = 319` — used \
         by IsHigherPriorityError to suppress the generic-disconnect dialog when a more \
         specific error is already showing (avoids stacking the catch-all on top of an actual \
         diagnostic)"
    );
}

#[test]
fn blizzard_glue_parent_publishes_modal_frame_helpers_and_expansion_logo_helpers() {
    let env = load_character_select_screen();

    for fn_name in [
        "GlueParent_AddModalFrame",
        "GlueParent_RemoveModalFrame",
        "GlueParentBlockingFrame_OnKeyDown",
        "GlueParent_CheckPhotosensitivity",
        "GetLogoReleaseType",
        "GetDisplayedExpansionLogo",
        "SetExpansionLogo",
    ] {
        let has_fn: bool = env
            .eval(&format!("return type({fn_name}) == 'function'"))
            .expect("util-fn query should succeed");
        assert!(
            has_fn,
            "{fn_name} should be a global function after load — Shared/GlueParentUtil.lua \
             publishes the 7-function flavor-shared utility surface: 2 modal-frame stack \
             helpers (Add/Remove drive GlueParent.BlockingFrame visibility), 1 ESCAPE / \
             PRINTSCREEN keybinding handler bound by the BlockingFrame OnKeyDown script, \
             1 photosensitivity check that opens the warning screen the first time the \
             current expansion is reached, and 3 expansion-logo helpers that pick the right \
             logo texture path based on GetGameReleaseType / GetCNLogoReleaseType (CN client \
             returns its own per-region logo on China builds)"
        );
    }
}

#[test]
fn blizzard_glue_parent_publishes_screen_query_and_navigation_helpers() {
    let env = load_character_select_screen();

    for fn_name in [
        "GlueParent_GetCurrentScreen",
        "GlueParent_GetSecondaryScreen",
        "GlueParent_IsSecondaryScreenOpen",
        "GlueParent_SetScreen",
        "GlueParent_OpenSecondaryScreen",
        "GlueParent_CloseSecondaryScreen",
        "GlueParent_ShowOptionsScreen",
        "GlueParent_ShowCinematicsScreen",
        "GlueParent_ShowCreditsScreen",
        "GlueParent_GetCurrentScreenInfo",
        "GlueParent_EnsureValidScreen",
        "GlueParent_UpdateDialogs",
    ] {
        let has_fn: bool = env
            .eval(&format!("return type({fn_name}) == 'function'"))
            .expect("screen-helper query should succeed");
        assert!(
            has_fn,
            "{fn_name} should be a global function after load — these 12 GlueParent_* helpers \
             form the public navigation surface that other glue addons drive (e.g. \
             Blizzard_GlueMenuFrame's InitAccountLoginButtons / InitCharacterSelectButtons \
             call ShowOptionsScreen / ShowCreditsScreen / ShowCinematicsScreen via \
             GenerateFlatClosure to wire the ESC menu to the secondary-screen swap)"
        );
    }
}

#[test]
fn blizzard_glue_parent_dir_ships_only_mainline_toc_with_shared_and_mainline_subdirs() {
    let dir = glue_parent_dir();
    let mut entries: Vec<String> = std::fs::read_dir(&dir)
        .expect("Blizzard_GlueParent dir should read")
        .flatten()
        .filter_map(|entry| entry.file_name().into_string().ok())
        .collect();
    entries.sort();

    assert_eq!(
        entries,
        vec![
            "Blizzard_GlueParent_Mainline.toc".to_string(),
            "Mainline".to_string(),
            "Shared".to_string(),
        ],
        "Blizzard_GlueParent ships exactly: 1 Mainline TOC + Mainline/ subdir + Shared/ \
         subdir. There is NO Classic TOC variant — Classic builds use a separate addon. The \
         Shared/ directory holds the flavor-shared GlueParentUtil.lua + a few CN-specific \
         helpers and is symlinked into the Classic-flavor twin"
    );
}
