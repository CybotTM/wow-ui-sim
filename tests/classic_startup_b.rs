//! Original cached Classic consumers. Staged source only; no native parity claim.
#![cfg(any(feature = "client-era", feature = "client-anniversary"))]
use sha2::{Digest, Sha256};
use wow_ui_sim::loader::{discover_blizzard_startup_addons_for_screen, load_startup_addon};
use wow_ui_sim::lua_api::WowLuaEnv;
use wow_ui_sim::screen::ScreenKind;

fn ui() -> std::path::PathBuf {
    wow_ui_sim::paths::default_blizzard_ui_addons_path().expect("active Classic cache")
}
fn original(file: &str) -> String {
    let bytes = std::fs::read(ui().join(file)).expect(file);
    let expected = include_str!("classic_startup_b_pins.tsv")
        .lines()
        .find_map(|line| line.split_once('\t').filter(|(name, _)| *name == file))
        .expect("pinned consumer")
        .1;
    assert_eq!(
        format!("{:x}", Sha256::digest(&bytes)),
        expected,
        "changed {file}"
    );
    String::from_utf8(bytes).unwrap()
}
fn env(files: &[&str]) -> WowLuaEnv {
    let env = WowLuaEnv::new().unwrap();
    env.exec("ClassicBErrors = {}; seterrorhandler(function(e) ClassicBErrors[#ClassicBErrors+1] = tostring(e) end)").unwrap();
    for file in files {
        env.loader_env().exec(&original(file)).unwrap();
    }
    env
}
fn run(env: &WowLuaEnv, source: &str) {
    let result = env.exec(source);
    env.exec("assert(#ClassicBErrors == 0, table.concat(ClassicBErrors, '\\n'))")
        .unwrap();
    assert!(
        env.state().borrow().lua_errors.is_empty(),
        "{:?}",
        env.state().borrow().lua_errors
    );
    result.unwrap();
}
// These are byte-exact statements from pinned consumers, not rewritten algorithms.
// Isolate a publication boundary without replacing the API or downstream renderer.
fn statements(file: &str, start: &str, end: &str) -> String {
    let source = original(file);
    let begin = source.find(start).unwrap();
    let finish = source[begin..].find(end).unwrap() + begin;
    source[begin..finish].to_owned()
}

#[test]
fn classic_b_r2_original_quest_count_statements() {
    let env = env(&[]);
    run(
        &env,
        "QuestLogQuestCount = UIParent:CreateFontString(); QuestLogCountMiddle = UIParent:CreateTexture(); MAX_QUESTLOG_QUESTS = 20",
    );
    let body = statements(
        "Blizzard_UIPanels_Game/Vanilla/QuestLogFrame.lua",
        "QuestLogQuestCount:SetText(format(QUEST_LOG_COUNT_TEMPLATE",
        "-- ScrollFrame update",
    );
    run(&env, &format!("local numQuests = 3; {body}"));
    run(
        &env,
        "assert(QuestLogQuestCount:GetText() == 'Quests: |cffffffff3/20|r')",
    );
    // Restore-missing must use the same selected dataset, without overwriting vendor writes.
    env.exec("QUEST_LOG_COUNT_TEMPLATE = nil").unwrap();
    // Public post-load lifecycle invokes the real missing-string restoration.
    env.apply_post_load_workarounds();
    run(&env, &format!("local numQuests = 0; {body}"));
    run(
        &env,
        "assert(QuestLogQuestCount:GetText() == 'Quests: |cffffffff0/20|r'); QUEST_LOG_COUNT_TEMPLATE = 'owned %d/%d'",
    );
    env.apply_post_load_workarounds();
    run(&env, "assert(QUEST_LOG_COUNT_TEMPLATE == 'owned %d/%d')");
}

// Source-backed labels are supplied by the selected shipped Classic string dataset.
#[test]
fn classic_b_r2_original_microbutton_label_from_shipped_data() {
    let env = env(&[]);
    let definition = statements(
        "Blizzard_MicroMenu/Classic/MainMenuBarMicroButtons.lua",
        "function MicroButtonTooltipText(",
        "function MainMenuMicroButton_Init(",
    );
    run(&env, &definition);
    run(
        &env,
        "SetBinding('I', 'TOGGLELFGPARENT'); local text = MicroButtonTooltipText(LFG_BUTTON, 'TOGGLELFGPARENT'); assert(type(text) == 'string' and text:find(LFG_BUTTON, 1, true))",
    );
}
fn validate_original_setting_label(expression: &str, variable: &str, kind: &str, default: &str) {
    let env = env(&[]);
    // Appending the call keeps the original local validator in its own lexical
    // scope. No test replacement of Settings validation or the missing label.
    let source = original("Blizzard_Settings_Shared/Blizzard_Setting.lua");
    run(
        &env,
        &format!(
            "{source}\nErrorIfInvalidSettingArguments({expression}, '{variable}', '{kind}', {default}); assert(type({expression}) == 'string' and #{expression} > 0)"
        ),
    );
}
#[test]
fn classic_b_r2_original_hd_setting_label_from_shipped_data() {
    validate_original_setting_label(
        "OPTION_HD_TEXTURES",
        "PROXY_HIGH_RES_TEXTURES",
        "boolean",
        "true",
    );
}
#[test]
fn classic_b_r2_original_nameplate_setting_label_from_shipped_data() {
    validate_original_setting_label(
        "UNIT_NAMEPLATES_CLASS_COLOR",
        "UNIT_NAMEPLATES_CLASS_COLOR",
        "number",
        "0",
    );
}

#[test]
fn classic_b_r5_required_lod_dependency_fixture_remains_loadable() {
    let root = tempfile::tempdir().unwrap();
    for (name, metadata, script) in [
        (
            "Blizzard_Game",
            "## Dependencies: Blizzard_ClassicBRequired\n",
            "assert(classicBRequiredLoaded); classicBRootLoaded = true",
        ),
        (
            "Blizzard_ClassicBRequired",
            "## LoadOnDemand: 1\n",
            "classicBRequiredLoaded = true",
        ),
    ] {
        let dir = root.path().join(name);
        std::fs::create_dir(&dir).unwrap();
        std::fs::write(
            dir.join(format!("{name}.toc")),
            format!("## Interface: 11509\n{metadata}Test.lua\n"),
        )
        .unwrap();
        std::fs::write(dir.join("Test.lua"), script).unwrap();
    }
    let addons = discover_blizzard_startup_addons_for_screen(root.path(), ScreenKind::Game);
    let names: Vec<_> = addons.iter().map(|addon| addon.name.as_str()).collect();
    assert_eq!(names, ["Blizzard_ClassicBRequired", "Blizzard_Game"]);
    let env = env(&[]);
    env.state().borrow_mut().addon_base_paths = vec![root.path().to_owned()];
    for addon in addons {
        load_startup_addon(&env.loader_env(), &addon.toc_path, addon.kind, None).unwrap();
    }
    run(
        &env,
        "assert(classicBRequiredLoaded and classicBRootLoaded)",
    );
}

#[test]
fn classic_b_r5_original_startup_keeps_transmog_lod_and_hard_dependencies() {
    let addons = discover_blizzard_startup_addons_for_screen(&ui(), ScreenKind::Game);
    assert!(!addons.iter().any(|addon| addon.name == "Blizzard_Transmog"));
    let index = |name| {
        addons
            .iter()
            .position(|addon| addon.name == name)
            .expect(name)
    };
    assert!(index("Blizzard_MoneyFrame") < index("Blizzard_UIPanels_Game"));
    let env = env(&[]);
    env.set_screen_mode(ScreenKind::Game);
    env.state().borrow_mut().addon_base_paths = vec![ui()];
    wow_ui_sim::xml::register_intrinsic_templates();
    for addon in &addons {
        load_startup_addon(&env.loader_env(), &addon.toc_path, addon.kind, None).unwrap();
    }
    let errors = env.state().borrow().lua_errors.clone();
    assert!(
        !errors
            .iter()
            .any(|error| error.contains("Blizzard_Transmog")),
        "{errors:?}"
    );
    // Other startup roots remain eligible; their independent errors are not erased.
    run_targeted(
        &env,
        "assert(not C_AddOns.IsAddOnLoaded('Blizzard_Transmog')); assert(C_AddOns.IsAddOnLoaded('Blizzard_UIPanels_Game'))",
    );
}
fn run_targeted(env: &WowLuaEnv, source: &str) {
    env.exec(source).unwrap();
}
