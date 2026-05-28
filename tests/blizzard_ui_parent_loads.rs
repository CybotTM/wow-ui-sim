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

fn ui_parent_dir() -> PathBuf {
    blizzard_ui_dir().join("Blizzard_UIParent")
}

fn ui_parent_toc() -> PathBuf {
    ui_parent_dir().join("Blizzard_UIParent_Mainline.toc")
}

const GLUE_SCREENS: &[ScreenKind] = &[
    ScreenKind::Login,
    ScreenKind::CharacterSelect,
    ScreenKind::CharacterCreate,
];

const TOC_DEPENDENCIES: &[&str] = &[
    "Blizzard_FrameXMLBase",
    "Blizzard_ObjectAPI",
    "Blizzard_Colors",
];

const MIXINS: &[&str] = &[
    "UIParentManagedFrameMixin",
    "UIParentManagedFrameContainerMixin",
];

const MODULE_LOAD_TABLES: &[&str] = &[
    "PULSEBUTTONS",
    "SHINES_TO_ANIMATE",
    "UIChildWindows",
    "UISpecialFrames",
    "UIMenus",
];

const MODULE_LOAD_NUMBER_CONSTANTS: &[(&str, f64)] = &[
    ("TOOLTIP_UPDATE_TIME", 0.2),
    ("BOSS_FRAME_CASTBAR_HEIGHT", 16.0),
    ("MAX_ACCOUNT_MACROS", 120.0),
    ("MAX_CHARACTER_MACROS", 30.0),
    ("FRAMERATE_FREQUENCY", 0.25),
];

const FREE_FUNCTIONS: &[&str] = &[
    "UIParent_OnLoad",
    "UIParent_OnEvent",
    "UIParent_OnShow",
    "UIParent_OnHide",
    "UIParent_Shared_OnLoad",
    "UIParent_Shared_OnEvent",
    "UIParent_UpdateTopFramePositions",
    "UIParentLoadAddOn",
    "WorldFrame_OnLoad",
    "WorldFrame_OnUpdate",
    "UpdateUIElementsForClientScene",
    "ToggleAchievementFrame",
    "ToggleGuildFrame",
    "ToggleEncounterJournal",
    "ToggleCollectionsJournal",
    "OpenAchievementFrameToAchievement",
    "ToggleLFGFrame",
    "InClickBindingMode",
    "ReverseQuestObjective",
];

const VIRTUAL_TEMPLATES: &[&str] = &[
    "ChatBubbleTemplate",
    "UIParentManagedFrameTemplate",
    "UIParentBottomManagedFrameTemplate",
    "UIParentRightManagedFrameTemplate",
    "UIParentManagedFrameContainer",
];

const NAMED_NON_VIRTUAL_FRAMES: &[&str] = &[
    "UIParent",
    "WorldFrame",
    "UIParentBottomManagedFrameContainer",
    "UIParentRightManagedFrameContainer",
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
fn find_toc_file_resolves_mainline_variant() {
    let resolved = find_toc_file(&ui_parent_dir()).expect("UIParent TOC resolves");
    assert_eq!(
        resolved,
        ui_parent_toc(),
        "find_toc_file at src/loader/mod.rs:65-95 prefers \
         `<addon>_Mainline.toc` first. Blizzard_UIParent ships TWO TOCs: \
         `_Mainline.toc` and `_Mists.toc` (the latter carries \
         `## LoadFirst: 1` and a Classic body); the Mainline preference \
         wins on a mainline build"
    );
}

#[test]
fn toc_is_eager_with_three_dependencies() {
    let toc = TocFile::from_file(&ui_parent_toc()).expect("TOC parses");

    assert!(
        !toc.is_load_on_demand(),
        "No `## LoadOnDemand` directive → eagerly loaded on Game. The \
         UIParent singleton frame is the protected root parent for every \
         in-world UI surface; it MUST be created before any sibling \
         addon's `parent=\"UIParent\"` resolution"
    );

    let deps = toc.dependencies();
    assert_eq!(
        deps.len(),
        TOC_DEPENDENCIES.len(),
        "Mainline TOC must declare exactly {} hard deps. Got {}: {:?}",
        TOC_DEPENDENCIES.len(),
        deps.len(),
        deps
    );
    for expected in TOC_DEPENDENCIES {
        assert!(
            deps.iter().any(|d| d == expected),
            "TOC must declare `{expected}` — UIParent leans on \
             Blizzard_FrameXMLBase (Mixin/CreateFromMixins/EventRegistry \
             plumbing), Blizzard_ObjectAPI (the C_* shim layer including \
             C_GameRules.IsGameRuleActive used by \
             UpdateUIElementsForClientScene), and Blizzard_Colors (named \
             color globals consumed by the templates). Got: {deps:?}"
        );
    }

    assert!(toc.optional_deps().is_empty());
    assert!(toc.saved_variables().is_empty());
    assert!(toc.saved_variables_per_character().is_empty());
    assert!(
        !toc.is_load_first(),
        "Mainline does NOT carry `## LoadFirst: 1` — that is a Classic-only \
         directive in the `_Mists.toc` companion. On Mainline the dep \
         graph (FrameXMLBase + ObjectAPI + Colors → UIParent) handles \
         load order"
    );
    assert!(toc.default_enabled());
}

#[test]
fn allow_load_game_restricts_to_in_world() {
    let toc = TocFile::from_file(&ui_parent_toc()).expect("TOC parses");

    assert!(
        toc.allows_screen(ScreenKind::Game),
        "`## AllowLoad: game` (lowercase) hits the `eq_ignore_ascii_case` \
         branch at toc.rs:308 → Game-only. The UIParent root frame and \
         WorldFrame world-render container only exist in-world; glue \
         screens use GlueParent / TitleScreen instead"
    );
    for screen in GLUE_SCREENS {
        assert!(
            !toc.allows_screen(*screen),
            "Glue screen {screen:?} must be excluded — `AllowLoad: game` \
             matches only the Game variant via toc.rs:308"
        );
    }
}

#[test]
fn allow_load_game_type_mainline_is_not_restricted() {
    let toc = TocFile::from_file(&ui_parent_toc()).expect("TOC parses");

    assert!(
        !toc.is_game_type_restricted(),
        "`## AllowLoadGameType: mainline` is recognised as a non-restricting \
         flavor at toc.rs:294-302 (standard|mainline). The Mists companion \
         TOC carries `mists` which IS restricting on a mainline build"
    );
}

#[test]
fn toc_raw_bytes_pin_six_directives_and_seven_body_files() {
    let raw = std::fs::read_to_string(ui_parent_toc()).expect("TOC reads utf-8");

    let expected_lines = [
        "## Title: Blizzard_UIParent",
        "## Author: Blizzard Entertainment",
        "## DefaultState: enabled",
        "## Dependencies: Blizzard_FrameXMLBase, Blizzard_ObjectAPI, Blizzard_Colors",
        "## AllowLoad: game",
        "## AllowLoadGameType: mainline",
        "ChatBubbleTemplates.xml",
        "Mainline\\WorldFrame.lua",
        "Mainline\\WorldFrame.xml",
        "Shared\\UIParent.lua",
        "Mainline\\UIParent.lua",
        "Mainline\\UIParent.xml",
        "Shared\\Localization.lua",
    ];

    for line in expected_lines {
        assert!(
            raw.contains(line),
            "Raw TOC must pin `{line}` — body order matters: \
             ChatBubbleTemplates.xml first publishes ChatBubbleTemplate \
             before any consumer; then the WorldFrame lua/xml pair (so \
             WorldFrame_OnLoad/_OnUpdate exist when the singleton is \
             created); then Shared/UIParent.lua publishes the 2 mixins \
             and UIParent_Shared_OnLoad/OnEvent helpers BEFORE \
             Mainline/UIParent.lua adds the per-flavor handlers and \
             Mainline/UIParent.xml instantiates the singleton; finally \
             Shared/Localization.lua overlays localized strings AFTER \
             everything else has loaded"
        );
    }

    assert!(!raw.contains("## LoadOnDemand"));
    assert!(
        !raw.contains("## LoadFirst"),
        "Mainline TOC must NOT carry LoadFirst — that's the Mists/Classic-only \
         companion directive"
    );
    assert!(!raw.contains("## OptionalDeps"));
    assert!(!raw.contains("## SavedVariables"));
    assert!(!raw.contains("## RequiredDep"));
    assert!(!raw.contains("[Family]"));
}

#[test]
fn mists_companion_toc_carries_load_first_and_classic_gametype() {
    let mists_toc = ui_parent_dir().join("Blizzard_UIParent_Mists.toc");
    assert!(
        mists_toc.is_file(),
        "Mists companion TOC must exist on disk"
    );

    let raw = std::fs::read_to_string(&mists_toc).expect("Mists TOC reads utf-8");
    assert!(
        raw.contains("## LoadFirst: 1"),
        "Mists companion TOC carries `## LoadFirst: 1` because Classic \
         lacks the FrameXML graph that promotes UIParent eagerly via deps"
    );
    assert!(
        raw.contains("## AllowLoadGameType: mists"),
        "Mists companion is gametype-restricted to `mists` — gated out \
         on a mainline build by toc.rs:294-302"
    );
    assert!(
        raw.contains("## Dependencies: Blizzard_FrameXMLBase, Blizzard_ObjectAPI"),
        "Mists drops the Blizzard_Colors hard dep that Mainline carries"
    );
}

#[test]
fn appears_in_game_eager_discovery() {
    let addons = discover_blizzard_addons_for_screen(&blizzard_ui_dir(), ScreenKind::Game);
    let found = addons.iter().any(|(name, _)| name == "Blizzard_UIParent");
    assert!(
        found,
        "Blizzard_UIParent must appear in Game eager discovery — without \
         it no in-world panel can resolve `parent=\"UIParent\"` and the \
         entire UI tree fails to compose"
    );
}

#[test]
fn absent_from_glue_screens_eager_discovery() {
    for screen in GLUE_SCREENS {
        let addons = discover_blizzard_addons_for_screen(&blizzard_ui_dir(), *screen);
        let found = addons.iter().any(|(name, _)| name == "Blizzard_UIParent");
        assert!(
            !found,
            "Blizzard_UIParent must NOT appear on {screen:?} — \
             AllowLoad:game restricts to in-world via toc.rs:308, \
             checked at loader/mod.rs:527 BEFORE pool partitioning"
        );
    }
}

#[test]
fn dep_directories_exist_on_disk() {
    for dep in TOC_DEPENDENCIES {
        let dir = blizzard_ui_dir().join(dep);
        assert!(
            dir.is_dir(),
            "Hard-dep directory `{dep}` must exist on disk"
        );
        assert!(
            find_toc_file(&dir).is_some(),
            "{dep} must have a discoverable TOC"
        );
    }
}

#[test]
fn full_game_load_publishes_mixins() {
    let env = load_full_game_ui();

    for mixin in MIXINS {
        let kind: String = env
            .eval(&format!("return type({mixin})"))
            .unwrap_or_else(|err| panic!("{mixin} probe failed: {err}"));
        assert_eq!(
            kind, "table",
            "{mixin} must be a global table after load. Both mixins live \
             in Shared/UIParent.lua: UIParentManagedFrameMixin gives \
             OnShow/OnHide hooks that route into the parent container's \
             UpdateFrame; UIParentManagedFrameContainerMixin maintains \
             the managedFrames list with AddManagedFrame / \
             RemoveManagedFrame / UpdateManagedFrames / \
             AnimIn/AnimOutManagedFrames / UpdateManagedFramesAlphaState"
        );
    }
}

#[test]
fn full_game_load_publishes_module_load_tables() {
    let env = load_full_game_ui();

    for table in MODULE_LOAD_TABLES {
        let kind: String = env
            .eval(&format!("return type({table})"))
            .unwrap_or_else(|err| panic!("{table} probe failed: {err}"));
        assert_eq!(
            kind, "table",
            "{table} must be a global table. PULSEBUTTONS / \
             SHINES_TO_ANIMATE feed the OnUpdate-driven pulse/shine \
             animation drivers (ButtonPulse_OnUpdate, \
             AnimatedShine_OnUpdate). UIChildWindows is the list of \
             windows that must close when their parent does (OpenMail, \
             GuildMemberDetail, GuildBankPopup, GearManagerDialog). \
             UISpecialFrames is the ESC-closes list (ItemRefTooltip, \
             ColorPickerFrame, the floating tooltips). UIMenus is the \
             dropdown-list registry (DropDownList1..3)"
        );
    }
}

#[test]
fn full_game_load_publishes_module_load_number_constants() {
    let env = load_full_game_ui();

    for (name, expected) in MODULE_LOAD_NUMBER_CONSTANTS {
        let value: f64 = env
            .eval(&format!("return {name}"))
            .unwrap_or_else(|err| panic!("{name} probe failed: {err}"));
        assert!(
            (value - expected).abs() < 1e-9,
            "{name} must equal {expected} after load (got {value}). \
             TOOLTIP_UPDATE_TIME and BOSS_FRAME_CASTBAR_HEIGHT come from \
             Mainline/UIParent.lua:1-2; MAX_ACCOUNT_MACROS / \
             MAX_CHARACTER_MACROS come from lines 11-12; \
             FRAMERATE_FREQUENCY comes from Mainline/WorldFrame.lua:2 \
             and is the throttle for the framerate-meter sample window"
        );
    }
}

#[test]
fn full_game_load_publishes_free_functions() {
    let env = load_full_game_ui();

    for func in FREE_FUNCTIONS {
        let kind: String = env
            .eval(&format!("return type({func})"))
            .unwrap_or_else(|err| panic!("{func} probe failed: {err}"));
        assert_eq!(
            kind, "function",
            "{func} must be a global function. UIParent_OnLoad / OnEvent \
             / OnShow / OnHide are the singleton frame's script handlers \
             wired in UIParent.xml:18-32. The `_Shared_` variants are the \
             cross-flavor handlers in Shared/UIParent.lua invoked at the \
             top of the Mainline handler. UIParentLoadAddOn drives the \
             on-demand-addon-load helpers for every per-feature \
             `*_LoadUI()` shim. WorldFrame_OnLoad / OnUpdate are the \
             world-render container's tick driver (StaticPopup_UpdateAll, \
             MirrorTimerContainer:ForceUpdateTimers, the tutorial \
             polling). UpdateUIElementsForClientScene flips PlayerFrame \
             / TargetFrame visibility based on Enum.ClientSceneType. The \
             Toggle* / Open* family are the public API for opening \
             addon-side panels"
        );
    }
}

#[test]
fn full_game_load_registers_virtual_templates() {
    let _env = load_full_game_ui();

    for template in VIRTUAL_TEMPLATES {
        let entry = wow_ui_sim::xml::get_template(template);
        assert!(
            entry.is_some(),
            "{template} must be a registered virtual template. \
             ChatBubbleTemplate (ChatBubbleTemplates.xml) is the \
             nine-slice chat-bubble chassis with ARTWORK-layer FontString \
             and a tail texture. UIParentManagedFrameTemplate is the \
             base for all UIParent-managed frames (OnShow/OnHide route \
             to the mixin); the Bottom and Right variants add KeyValues \
             for layoutParent/align/hideWhenActionBarIsOverriden. \
             UIParentManagedFrameContainer is the VerticalLayoutFrame + \
             container-mixin chassis that owns the actual layout slots"
        );
    }
}

#[test]
fn full_game_load_publishes_named_non_virtual_frames() {
    let env = load_full_game_ui();

    for name in NAMED_NON_VIRTUAL_FRAMES {
        let exists: bool = env
            .eval(&format!("return _G[{name:?}] ~= nil"))
            .unwrap_or_else(|err| panic!("{name} probe failed: {err}"));
        assert!(
            exists,
            "{name} must exist as a global frame after load. UIParent \
             (UIParent.xml:4) is the protected `setAllPoints` MEDIUM-strata \
             root with `preventSecretValues=\"true\"` and a ScopedModifier \
             wrapping (`addToSecureEnv=\"true\"`). WorldFrame \
             (WorldFrame.xml:21) is the unique world-render container \
             with `clipChildren=\"true\"` and `propagateMouseInput=\"Both\"`. \
             UIParentBottomManagedFrameContainer / \
             UIParentRightManagedFrameContainer are concrete instances of \
             UIParentManagedFrameContainer at frameStrata=\"LOW\" — they \
             host the layout slots populated by managed-frame consumers"
        );
    }
}

#[test]
fn full_game_load_emits_no_addon_specific_errors() {
    let env = load_full_game_ui();

    let errors: Vec<String> = env.state().borrow().lua_errors.clone();
    let addon_specific: Vec<&String> = errors
        .iter()
        .filter(|e| e.contains("Blizzard_UIParent/"))
        .collect();

    assert!(
        addon_specific.is_empty(),
        "Full game-screen load with UIParent in dependency order must \
         emit zero UIParent-body errors. The 7 body files (3 lua + 3 xml \
         + 1 localization-overlay lua, ~4000 lines) include \
         Mainline/UIParent.lua at 2842 lines with ~131 free functions \
         and the UIParent_OnEvent megaswitch; all must execute cleanly. \
         Found: {addon_specific:?}"
    );
}
