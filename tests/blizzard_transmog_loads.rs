use std::path::PathBuf;

use wow_ui_sim::loader::load_addon;
use wow_ui_sim::loader::{discover_blizzard_addons_for_screen, find_toc_file};
use wow_ui_sim::lua_api::WowLuaEnv;
use wow_ui_sim::screen::ScreenKind;
use wow_ui_sim::startup::fire_startup_events_for_screen;
use wow_ui_sim::toc::TocFile;

fn blizzard_ui_dir() -> PathBuf {
    wow_ui_sim::paths::default_blizzard_ui_addons_path().expect("Blizzard UI cache should be available")
}

fn transmog_dir() -> PathBuf {
    blizzard_ui_dir().join("Blizzard_Transmog")
}

fn transmog_toc() -> PathBuf {
    transmog_dir().join("Blizzard_Transmog.toc")
}

const ALL_FOUR_SCREENS: &[ScreenKind] = &[
    ScreenKind::Game,
    ScreenKind::Login,
    ScreenKind::CharacterSelect,
    ScreenKind::CharacterCreate,
];

const TOC_DEPENDENCIES: &[&str] = &[
    "Blizzard_SharedXML",
    "Blizzard_SharedXMLGame",
    "Blizzard_FrameXMLBase",
    "Blizzard_FrameXMLUtil",
    "Blizzard_UIPanelTemplates",
    "Blizzard_StaticPopup_Game",
    "Blizzard_TransmogShared",
    "Blizzard_PagedContent",
    "Blizzard_GameTooltip",
    "Blizzard_MoneyFrame",
    "Blizzard_Menu",
    "Blizzard_HelpPlate",
    "Blizzard_FrameEffects",
];

const MIXINS_FROM_MAIN_LUA: &[&str] = &[
    "TransmogFrameMixin",
    "TransmogOutfitCollectionMixin",
    "ShowEquippedGearSpellFrameMixin",
    "TransmogOutfitPopupMixin",
    "TransmogCharacterMixin",
    "TransmogWardrobeMixin",
    "TransmogWardrobeItemsMixin",
    "TransmogWardrobeSetsMixin",
    "TransmogWardrobeCustomSetsMixin",
    "TransmogWardrobeSituationsMixin",
];

const MIXINS_FROM_TEMPLATES_LUA: &[&str] = &[
    "TransmogOutfitEntryMixin",
    "TransmogSlotMixin",
    "TransmogAppearanceSlotMixin",
    "TransmogSlotFlyoutDropdownMixin",
    "TransmogIllusionSlotMixin",
    "TransmogWardrobeCollectionTabMixin",
    "TransmogSearchBoxMixin",
    "TransmogSearchBoxProgressMixin",
    "TransmogItemModelMixin",
    "TransmogSetBaseModelMixin",
    "TransmogSetModelMixin",
    "TransmogCustomSetModelMixin",
    "TransmogSituationMixin",
];

const MAINLINE_OVERRIDE_GLOBALS: &[&str] = &[
    "DressUpFrameLinkingSupported",
    "DisplayTypeUnassignedSupported",
    "HelpPlatesSupported",
];

fn fresh_game_env() -> WowLuaEnv {
    let env = WowLuaEnv::new().expect("Failed to create Lua environment");
    env.set_screen_size(1024.0, 768.0);
    env.set_screen_mode(ScreenKind::Game);
    {
        let mut state = env.state().borrow_mut();
        state.addon_base_paths = vec![blizzard_ui_dir()];
    }
    wow_ui_sim::xml::register_intrinsic_templates();
    env
}

fn load_full_game_ui() -> WowLuaEnv {
    let env = fresh_game_env();

    let addons = discover_blizzard_addons_for_screen(&blizzard_ui_dir(), ScreenKind::Game);
    for (name, toc_path) in &addons {
        load_addon(&env.loader_env(), toc_path)
            .unwrap_or_else(|err| panic!("[load {name}] FAILED: {err}"));
    }

    env.apply_post_load_workarounds();
    fire_startup_events_for_screen(&env, ScreenKind::Game);
    env
}

#[test]
fn find_toc_file_resolves_bare_toc() {
    let resolved = find_toc_file(&transmog_dir()).expect("Transmog TOC resolves");
    assert_eq!(
        resolved,
        transmog_toc(),
        "Bare TOC — no flavor suffix; the transmogrification panel \
         lives in a single TOC and uses [Family] body-token \
         substitution to swap Mainline vs Classic override files at \
         load time. Resolved via the bare-TOC path in find_toc_file \
         at src/loader/mod.rs:65-95"
    );
}

#[test]
fn toc_is_load_on_demand_with_thirteen_dependencies() {
    let toc = TocFile::from_file(&transmog_toc()).expect("TOC parses");

    assert!(
        toc.is_load_on_demand(),
        "`## LoadOnDemand: 1` — Transmog only loads when the player \
         visits a transmogrifier NPC. Blizzard_Transmog_Bootstrap.lua \
         publishes `Transmog_LoadUI()` and registers it with \
         `RegisterPlayerInteraction`"
    );

    let deps = toc.dependencies();
    assert_eq!(
        deps.len(),
        TOC_DEPENDENCIES.len(),
        "Must declare exactly {} hard deps. Got {}: {:?}",
        TOC_DEPENDENCIES.len(),
        deps.len(),
        deps
    );
    for expected in TOC_DEPENDENCIES {
        assert!(
            deps.iter().any(|d| d == expected),
            "TOC must declare `{expected}` as a hard dep — \
             Blizzard_TransmogShared (NOT this addon, a sibling) \
             provides cross-addon transmog primitives shared with \
             ChatFrameBase/Collections/FrameXML/ObjectiveTracker/\
             RecruitAFriend/UIPanels_Game; Blizzard_PagedContent \
             provides the wardrobe paging template; \
             Blizzard_HelpPlate/Blizzard_FrameEffects power the \
             tutorial pulses. Got: {deps:?}"
        );
    }

    assert!(toc.optional_deps().is_empty());
    assert!(toc.saved_variables().is_empty());
    assert!(toc.saved_variables_per_character().is_empty());
    assert!(!toc.is_load_first());
    assert!(!toc.is_secure_env());
    assert!(
        !toc.is_game_type_restricted(),
        "AllowLoadGameType absent → not restricted"
    );
    assert!(toc.default_enabled());
}

#[test]
fn allow_load_game_restricts_to_in_world_only() {
    let toc = TocFile::from_file(&transmog_toc()).expect("TOC parses");

    assert!(
        toc.allows_screen(ScreenKind::Game),
        "`## AllowLoad: Game` → toc.rs:308 returns true for Game. \
         Transmogrifier interaction only exists inside the world"
    );
    for screen in [
        ScreenKind::Login,
        ScreenKind::CharacterSelect,
        ScreenKind::CharacterCreate,
    ] {
        assert!(
            !toc.allows_screen(screen),
            "Glue screen {screen:?} must be excluded — `AllowLoad: \
             Game` explicitly disallows glue"
        );
    }
}

#[test]
fn toc_raw_bytes_pin_three_directives_and_family_token() {
    let raw = std::fs::read_to_string(transmog_toc()).expect("TOC reads utf-8");

    let expected_lines = [
        "## Title: Blizzard_Transmog",
        "## LoadOnDemand: 1",
        "## AllowLoad: Game",
        "## Dependencies: Blizzard_SharedXML",
        "[Family]\\Blizzard_TransmogOverrides.lua",
        "Blizzard_TransmogTemplates.xml",
        "Blizzard_Transmog.xml",
        "Blizzard_TransmogRegistration.lua",
    ];

    for line in expected_lines {
        assert!(
            raw.contains(line),
            "Raw TOC must pin `{line}` — note the `[Family]\\` token \
             which the toc parser at toc.rs:145 substitutes for \
             `Mainline\\` (resolved to `Mainline/...` after \
             backslash-to-slash normalization). The 2 listed XML \
             files transitively pull `Blizzard_Transmog.lua` (3165 \
             lines) and `Blizzard_TransmogTemplates.lua` (1878 lines) \
             via their `<Script file=...>` directives — neither .lua \
             is listed directly in the TOC body"
        );
    }

    assert!(!raw.contains("## Author"));
    assert!(!raw.contains("## Version"));
    assert!(!raw.contains("## DefaultState"));
    assert!(!raw.contains("## RequiredDep"));
    assert!(!raw.contains("## OptionalDep"));
    assert!(!raw.contains("## SavedVariables"));
    assert!(!raw.contains("## AllowLoadGameType"));
    assert!(!raw.contains("## UseSecureEnvironment"));
    assert!(!raw.contains("## LoadFirst"));

    assert!(
        !raw.contains("Blizzard_Transmog.lua"),
        "TOC body must NOT list the main lua file directly — it's \
         loaded only via Blizzard_Transmog.xml's <Script file=...> \
         directive"
    );
    assert!(
        !raw.contains("Blizzard_TransmogTemplates.lua"),
        "TOC body must NOT list the templates lua file directly"
    );
}

#[test]
fn family_token_substitutes_to_mainline_override_path() {
    let toc = TocFile::from_file(&transmog_toc()).expect("TOC parses");

    let body: Vec<String> = toc
        .files
        .iter()
        .map(|p| p.to_string_lossy().to_string())
        .collect();

    assert_eq!(
        body,
        vec![
            "Mainline/Blizzard_TransmogOverrides.lua".to_string(),
            "Blizzard_TransmogTemplates.xml".to_string(),
            "Blizzard_Transmog.xml".to_string(),
            "Blizzard_TransmogRegistration.lua".to_string(),
        ],
        "Body must be exactly 4 entries with `[Family]\\` substituted \
         to `Mainline/` (toc.rs:145 hardcodes the substitution since \
         this codebase only targets the Mainline flavor; toc.rs:147 \
         normalizes backslashes to forward slashes). Got: {body:?}"
    );
}

#[test]
fn classic_override_file_remains_on_disk_for_other_flavors() {
    let classic_override = transmog_dir()
        .join("Classic")
        .join("Blizzard_TransmogOverrides.lua");
    assert!(
        classic_override.is_file(),
        "Classic/Blizzard_TransmogOverrides.lua must exist on disk \
         even though this flavor never loads it — keeping the file \
         intact preserves Blizzard's family-substitution shape and \
         documents the flavor matrix. Mainline currently substitutes \
         the [Family] token to `Mainline\\` (toc.rs:145), so Classic's \
         override is dead-on-arrival here but must not be deleted"
    );

    let mainline_override = transmog_dir()
        .join("Mainline")
        .join("Blizzard_TransmogOverrides.lua");
    let raw =
        std::fs::read_to_string(&mainline_override).expect("Mainline override file reads utf-8");
    for fn_name in MAINLINE_OVERRIDE_GLOBALS {
        let needle = format!("function {fn_name}()");
        assert!(
            raw.contains(&needle),
            "Mainline/Blizzard_TransmogOverrides.lua must define \
             `{fn_name}()` — the override pattern lets the main \
             Blizzard_Transmog.lua call these flavor-gated predicates \
             without an inline if/else. Mainline returns true for \
             DressUpFrameLinkingSupported, DisplayTypeUnassignedSupported, \
             and HelpPlatesSupported"
        );
    }
}

#[test]
fn absent_from_every_screen_eager_discovery() {
    for screen in ALL_FOUR_SCREENS {
        let addons = discover_blizzard_addons_for_screen(&blizzard_ui_dir(), *screen);
        let found = addons.iter().any(|(name, _)| name == "Blizzard_Transmog");
        assert!(
            !found,
            "Blizzard_Transmog must be absent from {screen:?} eager \
             discovery — `## LoadOnDemand: 1` excludes LoD addons \
             from the eager sweep. Game restriction (AllowLoad: Game) \
             additionally rules out Login/CharacterSelect/CharacterCreate"
        );
    }
}

#[test]
fn no_addon_declares_transmog_as_dependency() {
    let entries = std::fs::read_dir(blizzard_ui_dir()).expect("BlizzardUI dir reads");
    let mut declarers: Vec<String> = Vec::new();

    for entry in entries.flatten() {
        let addon_dir = entry.path();
        if !addon_dir.is_dir() {
            continue;
        }
        let dir_name = addon_dir
            .file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("")
            .to_string();
        if dir_name == "Blizzard_Transmog" {
            continue;
        }
        let Some(toc_path) = find_toc_file(&addon_dir) else {
            continue;
        };
        let Ok(toc) = TocFile::from_file(&toc_path) else {
            continue;
        };
        let declared = toc.dependencies().iter().any(|d| d == "Blizzard_Transmog")
            || toc.optional_deps().iter().any(|d| d == "Blizzard_Transmog");
        if declared {
            declarers.push(dir_name);
        }
    }

    assert!(
        declarers.is_empty(),
        "No Blizzard addon may declare Blizzard_Transmog (the LoD \
         panel) as a hard or optional dep — siblings reference \
         Blizzard_TransmogShared instead (a separate always-loaded \
         addon containing cross-addon primitives). The LoD panel is \
         triggered ONLY via `Transmog_LoadUI()` at \
         UIParent.lua:475-477 when the player engages a transmog NPC. \
         Found declarers: {declarers:?}"
    );
}

#[test]
fn transmog_shared_dep_directory_exists_on_disk() {
    let shared_dir = blizzard_ui_dir().join("Blizzard_TransmogShared");
    assert!(
        shared_dir.is_dir(),
        "Hard-dep directory `Blizzard_TransmogShared` must exist on \
         disk — without it the dependency-resolution path can't find \
         a TOC and load_addon would fail at Transmog load time"
    );
    let toc = find_toc_file(&shared_dir);
    assert!(
        toc.is_some(),
        "Blizzard_TransmogShared must have a discoverable TOC"
    );
}

prefork_full_ui_case! {
fn explicit_load_publishes_main_lua_mixin_tables(env: &WowLuaEnv) {

    load_addon(&env.loader_env(), &transmog_toc())
        .expect("Blizzard_Transmog must load via Rust loader");

    for mixin in MIXINS_FROM_MAIN_LUA {
        let kind: String = env
            .eval(&format!("return type({mixin})"))
            .unwrap_or_else(|err| panic!("{mixin} probe failed: {err}"));
        assert_eq!(
            kind, "table",
            "{mixin} must be a table after LoD load — declared in \
             Blizzard_Transmog.lua (3165 lines) which is loaded \
             transitively via Blizzard_Transmog.xml's <Script file=\
             \"Blizzard_Transmog.lua\"/> directive at xml:3"
        );
    }
}
}

prefork_full_ui_case! {
fn explicit_load_publishes_templates_lua_mixin_tables(env: &WowLuaEnv) {

    load_addon(&env.loader_env(), &transmog_toc())
        .expect("Blizzard_Transmog must load via Rust loader");

    for mixin in MIXINS_FROM_TEMPLATES_LUA {
        let kind: String = env
            .eval(&format!("return type({mixin})"))
            .unwrap_or_else(|err| panic!("{mixin} probe failed: {err}"));
        assert_eq!(
            kind, "table",
            "{mixin} must be a table after LoD load — declared in \
             Blizzard_TransmogTemplates.lua (1878 lines) which is \
             loaded transitively via Blizzard_TransmogTemplates.xml's \
             <Script file=...> directive. Templates load BEFORE \
             Blizzard_Transmog.xml so virtual templates referenced by \
             the main XML are already registered"
        );
    }
}
}

prefork_full_ui_case! {
fn explicit_load_creates_transmog_frame_global(env: &WowLuaEnv) {

    load_addon(&env.loader_env(), &transmog_toc())
        .expect("Blizzard_Transmog must load via Rust loader");

    let exists: bool = env
        .eval("return TransmogFrame ~= nil")
        .expect("TransmogFrame probe");
    assert!(
        exists,
        "TransmogFrame must exist as a named global after LoD load — \
         declared at Blizzard_Transmog.xml:5 as `<Frame \
         name=\"TransmogFrame\" mixin=\"TransmogFrameMixin\" \
         inherits=\"PortraitFrameTemplate\" parent=\"UIParent\" \
         toplevel=\"true\" enableMouse=\"true\" hidden=\"true\">` \
         with collapsedWidth=1308, full size 1618x883. This is the \
         only non-virtual top-level frame in Blizzard_Transmog.xml — \
         everything else is a Mixin/Template"
    );
}
}

prefork_full_ui_case! {
fn refresh_weapon_dropdown_counts_sparse_weapon_categories(env: &WowLuaEnv) {

    load_addon(&env.loader_env(), &transmog_toc())
        .expect("Blizzard_Transmog must load via Rust loader");

    let result: String = env
        .eval(
            r#"
            local originalGetCollectionInfo = C_TransmogOutfitInfo.GetCollectionInfoForSlotAndOption
            local originalFirst = FIRST_TRANSMOG_COLLECTION_WEAPON_TYPE
            local originalLast = LAST_TRANSMOG_COLLECTION_WEAPON_TYPE

            FIRST_TRANSMOG_COLLECTION_WEAPON_TYPE = 10
            LAST_TRANSMOG_COLLECTION_WEAPON_TYPE = 12
            C_TransmogOutfitInfo.GetCollectionInfoForSlotAndOption = function(_slot, _option, categoryID)
                if categoryID == 10 or categoryID == 12 then
                    return { isWeapon = true, name = "weapon-" .. categoryID }
                end
                return nil
            end

            local state = { hidden = 0, shown = 0, radios = 0 }
            local rootDescription = {
                SetTag = function() end,
                CreateRadio = function()
                    state.radios = state.radios + 1
                end,
            }

            local dropdown = {
                Hide = function()
                    state.hidden = state.hidden + 1
                end,
                Show = function()
                    state.shown = state.shown + 1
                end,
                SetupMenu = function(_, callback)
                    callback(dropdown, rootDescription)
                end,
            }

            local transmogLocation = {
                IsIllusion = function() return false end,
                GetSlot = function() return 16 end,
            }

            local wardrobe = {
                activeCategoryID = 10,
                WeaponDropdown = dropdown,
                GetSelectedSlotCallback = function()
                    return {
                        transmogLocation = transmogLocation,
                        currentWeaponOptionInfo = { weaponOption = 1 },
                    }
                end,
                SetActiveCategory = function(_, categoryID)
                    state.selected = categoryID
                end,
                RefreshFilterButtons = function()
                    state.refreshed = true
                end,
            }

            TransmogWardrobeItemsMixin.RefreshWeaponDropdown(wardrobe)

            C_TransmogOutfitInfo.GetCollectionInfoForSlotAndOption = originalGetCollectionInfo
            FIRST_TRANSMOG_COLLECTION_WEAPON_TYPE = originalFirst
            LAST_TRANSMOG_COLLECTION_WEAPON_TYPE = originalLast

            return string.format("%d/%d/%d", state.hidden, state.shown, state.radios)
            "#,
        )
        .expect("RefreshWeaponDropdown probe");

    assert_eq!(
        result, "0/1/2",
        "RefreshWeaponDropdown must treat sparse weapon categories 10 and 12 \
         as two valid categories. This exercises Blizzard_Transmog.lua:1914 \
         (`table.count(validCategories) <= 1`) instead of only proving that \
         Blizzard_Transmog loads."
    );
}
}

prefork_full_ui_case! {
fn explicit_load_registers_ui_panel_via_registration_lua(env: &WowLuaEnv) {

    load_addon(&env.loader_env(), &transmog_toc())
        .expect("Blizzard_Transmog must load via Rust loader");

    let kind: String = env
        .eval("return type(UIPanelWindows['TransmogFrame'])")
        .expect("UIPanelWindows entry probe");
    assert_eq!(
        kind, "table",
        "Blizzard_TransmogRegistration.lua (12 lines, last in body) \
         calls `RegisterUIPanel(TransmogFrame, attributes)` with \
         area=center, pushable=0, checkFit=1, checkFitExtraWidth=200, \
         checkFitExtraHeight=140, allowOtherPanels=1. The dedicated \
         registration file decouples panel-manager wiring from the \
         huge main lua, and runs LAST so TransmogFrame already exists \
         from the XML load that ran before it"
    );

    let area: String = env
        .eval("return UIPanelWindows['TransmogFrame'].area")
        .expect("area probe");
    assert_eq!(
        area, "center",
        "TransmogFrame must dock center — extra-width=200 + \
         extra-height=140 means the panel manager will tolerate 200 \
         px of horizontal expansion and 140 px of vertical expansion \
         before considering it overflowing"
    );
}
}

prefork_full_ui_case! {
fn transmog_load_ui_published_at_boot(env: &WowLuaEnv) {

    let kind: String = env
        .eval("return type(Transmog_LoadUI)")
        .expect("Transmog_LoadUI probe");
    assert_eq!(
        kind, "function",
        "Transmog_LoadUI must exist at boot — published by \
         Blizzard_Transmog_Bootstrap.lua before the rest of the LoD \
         addon body loads, so the Transmogrifier interaction's loadFunc \
         resolves at module-init time. Without this boot-time wrapper, \
         the first transmog interaction would race the LoD load"
    );
}
}

prefork_full_ui_case! {
fn explicit_load_emits_no_addon_specific_errors(env: &WowLuaEnv) {

    {
        let mut state = env.state().borrow_mut();
        state.lua_errors.clear();
        state.lua_error_records.clear();
        state.lua_error_counts.clear();
    }

    load_addon(&env.loader_env(), &transmog_toc())
        .expect("Blizzard_Transmog must load via Rust loader");

    let errors: Vec<String> = env.state().borrow().lua_errors.clone();
    let addon_specific: Vec<&String> = errors
        .iter()
        .filter(|e| {
            e.contains("Blizzard_Transmog/")
                || e.contains("Blizzard_TransmogOverrides")
                || e.contains("Blizzard_TransmogTemplates")
                || e.contains("Blizzard_TransmogRegistration")
                || e.contains("TransmogFrame.lua")
                || e.contains("TransmogFrameMixin")
        })
        .collect();

    assert!(
        addon_specific.is_empty(),
        "Re-loading Transmog over a fully-loaded game env must emit \
         zero addon-specific errors. Found {}: {:#?}",
        addon_specific.len(),
        addon_specific
    );
}
}
