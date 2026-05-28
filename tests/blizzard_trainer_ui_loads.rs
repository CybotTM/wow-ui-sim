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

fn trainer_dir() -> PathBuf {
    blizzard_ui_dir().join("Blizzard_TrainerUI")
}

fn trainer_toc() -> PathBuf {
    trainer_dir().join("Blizzard_TrainerUI.toc")
}

const ALL_FOUR_SCREENS: &[ScreenKind] = &[
    ScreenKind::Game,
    ScreenKind::Login,
    ScreenKind::CharacterSelect,
    ScreenKind::CharacterCreate,
];

const PUBLISHED_GLOBAL_FUNCTIONS: &[&str] = &[
    "ClassTrainerFrame_Show",
    "ClassTrainerFrame_Hide",
    "ClassTrainerFrame_OnLoad",
    "ClassTrainerFrame_OnShow",
    "ClassTrainerFrame_OnHide",
    "ClassTrainerFrame_OnEvent",
    "ClassTrainerFrame_SetTrainButtonEnabled",
    "ClassTrainerFrame_Update",
    "ClassTrainerFrame_InitServiceButton",
    "ClassTrainer_SelectNearestLearnableSkill",
    "ClassTrainer_SetSelection",
    "ClassTrainerSkillButton_OnClick",
    "ClassTrainerTrainButton_OnClick",
];

const PUBLISHED_CONSTANTS: &[(&str, i64)] = &[
    ("CLASS_TRAINER_SKILLS_DISPLAYED", 7),
    ("CLASS_TRAINER_SCROLL_HEIGHT", 330),
    ("CLASS_TRAINER_SKILL_BUTTON_WIDTH", 318),
    ("CLASS_TRAINER_SKILL_BARBUTTON_WIDTH", 298),
    ("CLASS_TRAINER_SKILL_HEIGHT", 47),
    ("MAX_LEARNABLE_PROFESSIONS", 2),
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
    let resolved = find_toc_file(&trainer_dir()).expect("TrainerUI TOC resolves");
    assert_eq!(
        resolved,
        trainer_toc(),
        "Bare TOC — no flavor suffix; classic-era LoD addon resolved \
         via the bare-TOC path in find_toc_file at \
         src/loader/mod.rs:65-95"
    );
}

#[test]
fn toc_is_load_on_demand_with_no_dependencies() {
    let toc = TocFile::from_file(&trainer_toc()).expect("TOC parses");

    assert!(
        toc.is_load_on_demand(),
        "`## LoadOnDemand: 1` — only loads when the player engages an \
         NPC trainer; PlayerInteractionFrameManager.lua:29-35 maps \
         Enum.PlayerInteractionType.Trainer to loadFunc=\
         ClassTrainerFrame_LoadUI, which calls \
         UIParentLoadAddOn(\"Blizzard_TrainerUI\") at \
         UIParent.lua:265-267"
    );
    assert!(
        toc.dependencies().is_empty(),
        "No `## Dependencies:` directive — TrainerUI is a small classic-\
         era addon that relies only on the always-loaded Blizzard \
         FrameXML core (ButtonFrameTemplate, MagicButtonTemplate, \
         SmallMoneyFrameTemplate, WowScrollBoxList, MinimalScrollBar, \
         WowStyle1FilterDropdownTemplate, InsetFrameTemplate). Got: {:?}",
        toc.dependencies()
    );
    assert!(toc.optional_deps().is_empty());
    assert!(toc.saved_variables().is_empty());
    assert!(toc.saved_variables_per_character().is_empty());
    assert!(!toc.is_load_first());
    assert!(!toc.is_secure_env());
    assert!(
        !toc.is_game_type_restricted(),
        "AllowLoadGameType absent → not restricted (false). \
         Class trainers exist on every flavor (mainline, classic, mists, \
         etc.) so the addon stays unrestricted."
    );
    assert!(toc.default_enabled());
}

#[test]
fn allow_load_absent_defaults_to_game_only_screen() {
    let toc = TocFile::from_file(&trainer_toc()).expect("TOC parses");

    assert!(
        toc.allows_screen(ScreenKind::Game),
        "AllowLoad absent → toc.rs:305-313 None branch defaults to \
         Game-only — class-trainer dialogs only open via in-world NPC \
         interaction, so the panel is meaningless on glue screens"
    );
    for screen in [
        ScreenKind::Login,
        ScreenKind::CharacterSelect,
        ScreenKind::CharacterCreate,
    ] {
        assert!(
            !toc.allows_screen(screen),
            "Glue screen {screen:?} must be excluded — trainers are \
             world-NPC-driven and cannot exist before character entry"
        );
    }
}

#[test]
fn toc_raw_bytes_pin_minimal_two_directive_shape() {
    let raw = std::fs::read_to_string(trainer_toc()).expect("TOC reads utf-8");

    let expected_lines = [
        "## Title: Blizzard Trainer UI",
        "## LoadOnDemand: 1",
        "Blizzard_TrainerUI.xml",
        "Localization.lua",
    ];

    for line in expected_lines {
        assert!(
            raw.contains(line),
            "Raw TOC must pin `{line}` — minimal 2-directive shape \
             (Title + LoadOnDemand) plus 2 body files. Note: \
             `Blizzard_TrainerUI.lua` is NOT listed in the TOC body — \
             it's pulled in via the XML's `<Script \
             file=\"Blizzard_TrainerUI.lua\"/>` directive at xml:3 \
             (lua-via-XML-Script body shape, same pattern as \
             TorghastLevelPicker)"
        );
    }

    assert!(!raw.contains("## Author"));
    assert!(!raw.contains("## Version"));
    assert!(!raw.contains("## DefaultState"));
    assert!(!raw.contains("## Dependencies"));
    assert!(!raw.contains("## RequiredDep"));
    assert!(!raw.contains("## OptionalDep"));
    assert!(!raw.contains("## SavedVariables"));
    assert!(!raw.contains("## AllowLoad"));
    assert!(!raw.contains("## AllowLoadGameType"));
    assert!(!raw.contains("## UseSecureEnvironment"));
    assert!(!raw.contains("## LoadFirst"));
    assert!(
        !raw.contains("Blizzard_TrainerUI.lua\n") && !raw.ends_with("Blizzard_TrainerUI.lua"),
        "TOC body must NOT list Blizzard_TrainerUI.lua directly — the \
         lua is loaded only via the XML <Script file=...> directive"
    );
}

#[test]
fn body_resolves_to_xml_and_localization_lua() {
    let toc = TocFile::from_file(&trainer_toc()).expect("TOC parses");

    let body: Vec<String> = toc
        .files
        .iter()
        .map(|p| p.to_string_lossy().to_string())
        .collect();

    assert_eq!(
        body,
        vec![
            "Blizzard_TrainerUI.xml".to_string(),
            "Localization.lua".to_string(),
        ],
        "Body must be exactly 2 entries in this order — XML first \
         (which transitively pulls Blizzard_TrainerUI.lua via <Script \
         file=...>) then the empty Localization.lua trailer (just a \
         single comment line at Localization.lua:1). Got: {body:?}"
    );
}

#[test]
fn absent_from_every_screen_eager_discovery() {
    for screen in ALL_FOUR_SCREENS {
        let addons = discover_blizzard_addons_for_screen(&blizzard_ui_dir(), *screen);
        let found = addons.iter().any(|(name, _)| name == "Blizzard_TrainerUI");
        assert!(
            !found,
            "Blizzard_TrainerUI must be absent from {screen:?} eager \
             discovery — `## LoadOnDemand: 1` excludes LoD addons from \
             the eager sweep"
        );
    }
}

#[test]
fn no_addon_declares_trainer_ui_as_dependency() {
    let entries = std::fs::read_dir(blizzard_ui_dir()).expect("BlizzardUI dir reads");
    let mut declarers: Vec<String> = Vec::new();

    for entry in entries.flatten() {
        let addon_dir = entry.path();
        if !addon_dir.is_dir() {
            continue;
        }
        let Some(toc_path) = find_toc_file(&addon_dir) else {
            continue;
        };
        let Ok(toc) = TocFile::from_file(&toc_path) else {
            continue;
        };
        let declared = toc.dependencies().iter().any(|d| d == "Blizzard_TrainerUI")
            || toc
                .optional_deps()
                .iter()
                .any(|d| d == "Blizzard_TrainerUI");
        if declared {
            let name = addon_dir.file_name().unwrap().to_string_lossy().to_string();
            declarers.push(name);
        }
    }

    assert!(
        declarers.is_empty(),
        "No Blizzard addon may declare Blizzard_TrainerUI as a hard or \
         optional dep — strictly LoD, triggered ONLY by \
         PlayerInteractionFrameManager when the player engages a \
         trainer NPC. Found declarers: {declarers:?}"
    );
}

#[test]
fn explicit_load_publishes_constants() {
    let env = load_full_game_ui();

    load_addon(&env.loader_env(), &trainer_toc())
        .expect("Blizzard_TrainerUI must load via Rust loader");

    for (name, expected) in PUBLISHED_CONSTANTS {
        let actual: i64 = env
            .eval(&format!("return {name}"))
            .unwrap_or_else(|err| panic!("{name} probe failed: {err}"));
        assert_eq!(
            actual, *expected,
            "Global constant `{name}` must equal {expected} after LoD \
             load — declared at Blizzard_TrainerUI.lua lines 2-7. \
             These pin layout dimensions for the scroll list and the \
             profession-cap MAX_LEARNABLE_PROFESSIONS=2. Got {actual}"
        );
    }
}

#[test]
fn explicit_load_publishes_global_functions() {
    let env = load_full_game_ui();

    load_addon(&env.loader_env(), &trainer_toc())
        .expect("Blizzard_TrainerUI must load via Rust loader");

    for fn_name in PUBLISHED_GLOBAL_FUNCTIONS {
        let kind: String = env
            .eval(&format!("return type({fn_name})"))
            .unwrap_or_else(|err| panic!("{fn_name} probe failed: {err}"));
        assert_eq!(
            kind, "function",
            "Global function `{fn_name}` must be defined after LoD load. \
             These cover the 4 frame scripts (OnLoad/OnShow/OnHide/\
             OnEvent at lua:50-130), 2 panel-manager entry points \
             (ClassTrainerFrame_Show/Hide at lua:38-48), the train-\
             button enable wrapper (lua:132-151), the data-provider \
             rebuild (lua:153-207), per-row init (lua:209-331), \
             auto-selection (lua:333-364), selection state (lua:366-\
             402), and the 2 button click handlers (lua:404-416). Got \
             type={kind} for {fn_name}"
        );
    }
}

#[test]
fn explicit_load_creates_class_trainer_frame_global() {
    let env = load_full_game_ui();

    load_addon(&env.loader_env(), &trainer_toc())
        .expect("Blizzard_TrainerUI must load via Rust loader");

    let exists: bool = env
        .eval("return ClassTrainerFrame ~= nil")
        .expect("ClassTrainerFrame probe");
    assert!(
        exists,
        "ClassTrainerFrame must exist as a named global after LoD load \
         — declared at xml:110 as `<Frame name=\"ClassTrainerFrame\" \
         inherits=\"ButtonFrameTemplate\" toplevel=\"true\" \
         movable=\"true\" parent=\"UIParent\" enableMouse=\"true\" \
         hidden=\"true\">`. The frame is the lone non-virtual top-\
         level frame published by this addon — \
         ClassTrainerSkillButtonTemplate at xml:28 is virtual and \
         instantiated by the WowScrollBoxList view"
    );
}

#[test]
fn explicit_load_registers_ui_panel_windows_entry() {
    let env = load_full_game_ui();

    load_addon(&env.loader_env(), &trainer_toc())
        .expect("Blizzard_TrainerUI must load via Rust loader");

    let kind: String = env
        .eval("return type(UIPanelWindows['ClassTrainerFrame'])")
        .expect("UIPanelWindows entry probe");
    assert_eq!(
        kind, "table",
        "Blizzard_TrainerUI.lua:9 must register \
         `UIPanelWindows[\"ClassTrainerFrame\"]` with area=left, \
         pushable=0, allowOtherPanels=1 — UNLIKE many panels (e.g. \
         TorghastLevelPickerFrame) the entry is NOT pre-registered at \
         boot; it appears only after the LoD addon executes its body, \
         so any caller that wants to ShowUIPanel(ClassTrainerFrame) \
         must first call UIParentLoadAddOn(\"Blizzard_TrainerUI\")"
    );

    let area: String = env
        .eval("return UIPanelWindows['ClassTrainerFrame'].area")
        .expect("area field probe");
    assert_eq!(
        area, "left",
        "UIPanelWindows entry must have area=\"left\" so trainer dialogs \
         dock on the left edge of the screen alongside other primary \
         interaction windows (merchant, banker, mailbox)"
    );
}

#[test]
fn player_interaction_frame_manager_routes_trainer_via_load_addon() {
    let raw = std::fs::read_to_string(
        blizzard_ui_dir().join("Blizzard_UIPanels_Game/Shared/PlayerInteractionFrameManager.lua"),
    )
    .expect("PlayerInteractionFrameManager.lua reads utf-8");

    assert!(
        raw.contains("[Enum.PlayerInteractionType.Trainer]"),
        "PlayerInteractionFrameManager must key the trainer entry by \
         `Enum.PlayerInteractionType.Trainer` (line 29) — this is the \
         single dispatch point that the C side hits when the player \
         engages an NPC trainer"
    );
    assert!(
        raw.contains("frame = \"ClassTrainerFrame\"")
            && raw.contains("showFunc = \"ClassTrainerFrame_Show\"")
            && raw.contains("hideFunc = \"ClassTrainerFrame_Hide\"")
            && raw.contains("loadFunc = ClassTrainerFrame_LoadUI"),
        "Trainer interaction entry at \
         PlayerInteractionFrameManager.lua:29-35 must wire all 4 keys \
         — frame, showFunc, hideFunc (string lookups so the manager \
         tolerates the addon being unloaded) plus loadFunc (a direct \
         function reference resolved at boot from UIParent.lua:265-\
         267 BEFORE Blizzard_TrainerUI itself loads, so first \
         interaction can lazily load the addon)"
    );
}

#[test]
fn class_trainer_frame_load_ui_published_at_boot() {
    let env = load_full_game_ui();

    let kind: String = env
        .eval("return type(ClassTrainerFrame_LoadUI)")
        .expect("ClassTrainerFrame_LoadUI probe");
    assert_eq!(
        kind, "function",
        "ClassTrainerFrame_LoadUI must be defined at boot — \
         UIParent.lua:265-267 declares it BEFORE Blizzard_TrainerUI \
         loads, so PlayerInteractionFrameManager.lua:34 can capture \
         the reference via `loadFunc = ClassTrainerFrame_LoadUI` and \
         the trainer interaction can lazily LoadAddOn the panel on \
         first engagement (without this boot-time wrapper, the \
         loadFunc reference would be nil at the time the \
         playerInteractionToFrameInfo table is constructed)"
    );
}

#[test]
fn explicit_load_emits_no_addon_specific_errors() {
    let env = load_full_game_ui();

    {
        let mut state = env.state().borrow_mut();
        state.lua_errors.clear();
        state.lua_error_records.clear();
        state.lua_error_counts.clear();
    }

    load_addon(&env.loader_env(), &trainer_toc())
        .expect("Blizzard_TrainerUI must load via Rust loader");

    let errors: Vec<String> = env.state().borrow().lua_errors.clone();
    let addon_specific: Vec<&String> = errors
        .iter()
        .filter(|e| e.contains("Blizzard_TrainerUI") || e.contains("ClassTrainer"))
        .collect();

    assert!(
        addon_specific.is_empty(),
        "Re-loading TrainerUI over a fully-loaded game env must emit \
         zero addon-specific errors — load creates ClassTrainerFrame, \
         registers the StaticPopupDialogs[\"CONFIRM_PROFESSION\"] entry, \
         and sets UIPanelWindows[\"ClassTrainerFrame\"]; no event \
         handlers fire until the player actually engages a trainer. \
         Found {}: {:#?}",
        addon_specific.len(),
        addon_specific
    );
}
