//! Wrath abandonment must commit only through the original confirmation consumer.
#![cfg(feature = "client-wrath")]

use sha2::{Digest, Sha256};
use std::path::{Path, PathBuf};
use wow_ui_sim::c_api::quest_objectives::QuestObjectiveProgress;
use wow_ui_sim::lua_api::WowLuaEnv;
use wow_ui_sim::lua_api::state::GossipQuestRow;

fn original(file: &str, expected: &str) -> PathBuf {
    let path = dirs::cache_dir()
        .unwrap()
        .join("wow-ui-sim/blizzard-ui/wrath/FrameXML")
        .join(file);
    let bytes = std::fs::read(&path).unwrap();
    let hash = format!("{:x}", Sha256::digest(bytes));
    assert_eq!(hash, expected, "source drift: {}", path.display());
    eprintln!("original {} sha256={hash}", path.display());
    path
}

fn assert_errors(env: &WowLuaEnv) {
    let errors: String = env
        .eval("return table.concat(AbandonErrors, '\n')")
        .unwrap();
    let sim = env.state().borrow();
    eprintln!(
        "handler={errors:?}; lua_errors={:?}; counts={:?}",
        sim.lua_errors, sim.lua_error_counts
    );
    assert!(errors.is_empty(), "{errors}");
    assert!(sim.lua_errors.is_empty(), "{:?}", sim.lua_errors);
    assert!(
        sim.lua_error_counts.is_empty(),
        "{:?}",
        sim.lua_error_counts
    );
}

fn exec(env: &WowLuaEnv, source: &str) {
    let result = env.exec(source);
    eprintln!("direct eval {source:?}: {result:?}");
    assert_errors(env);
    result.expect("original consumer and behavioral assertions");
}

fn load_xml(env: &WowLuaEnv, path: &Path) {
    let dir = tempfile::tempdir().unwrap();
    let toc = dir.path().join("AbandonFixture.toc");
    std::fs::write(
        &toc,
        format!("## Title: AbandonFixture\n{}\n", path.display()),
    )
    .unwrap();
    let result = wow_ui_sim::loader::load_addon(&env.loader_env(), &toc).unwrap();
    eprintln!("XML result: {result:?}");
    assert_errors(env);
}

fn seed_env() -> WowLuaEnv {
    let env = WowLuaEnv::new().unwrap();
    env.exec("AbandonErrors = {}; seterrorhandler(function(e) AbandonErrors[#AbandonErrors+1] = tostring(e) end)").unwrap();
    {
        let mut sim = env.state().borrow_mut();
        sim.quest_log_entries.entries = [
            (990101, "Wrath abandon first"),
            (990202, "Wrath abandon second"),
        ]
        .into_iter()
        .map(|(quest_id, title)| {
            sim.quest_log_entries
                .snapshot_gossip_offer(&GossipQuestRow {
                    quest_id,
                    title: title.into(),
                    quest_level: 23,
                    ..Default::default()
                })
                .unwrap()
        })
        .collect();
        sim.quest_log = vec![990101, 990202];
        sim.pending_quest_offer = None;
        sim.selected_quest_log_id = None;
        sim.abandon_quest_id = None;
        for id in [990101, 990202] {
            sim.quest_log_entries
                .objective_progress
                .insert(id, QuestObjectiveProgress::default());
            sim.quest_log_entries
                .background_materials
                .insert(id, format!("material-{id}"));
            sim.quest_log_entries.readiness.auto_complete.insert(id);
            sim.quest_log_entries.readiness.notified.insert(id);
            sim.quest_log_entries
                .readiness
                .popups
                .push((id, "OFFER".into()));
        }
    }
    exec(
        &env,
        r#"
        QuestDifficultyColors = {}
        for _, key in ipairs({'impossible', 'verydifficult', 'difficult', 'standard', 'trivial', 'header'}) do QuestDifficultyColors[key] = {} end
        QuestLogFrame = CreateFrame('Frame', 'QuestLogFrame')
        QuestLogDetailFrame = CreateFrame('Frame', 'QuestLogDetailFrame'); QuestLogDetailFrame:Hide()
        QuestLogDetailScrollFrame = CreateFrame('ScrollFrame', 'QuestLogDetailScrollFrame'); QuestLogDetailScrollFrame:Hide()
        QuestLogFrameTrackButton = CreateFrame('Button', 'QuestLogFrameTrackButton')
        QuestLogFramePushQuestButton = CreateFrame('Button', 'QuestLogFramePushQuestButton')
        AbandonEvents = {}
        local observer = CreateFrame('Frame')
        for _, event in ipairs({'QUEST_LOG_UPDATE', 'QUEST_REMOVED', 'QUEST_ACCEPTED', 'QUEST_TURNED_IN'}) do observer:RegisterEvent(event) end
        observer:SetScript('OnEvent', function(_, event, ...)
            AbandonEvents[#AbandonEvents+1] = {event, GetNumQuestLogEntries(), GetQuestLogTitle(1), select('#', ...)}
        end)
    "#,
    );
    for (file, hash) in [
        (
            "QuestLogFrame.lua",
            "6ac6ca92b3036417082b88cc2f8b5c9fc8889f51f82aa726fe0bcd44990bc5e2",
        ),
        (
            "StaticPopup.lua",
            "1134b08e87d8577c0353895207eaec24e74e8f669775a49257a82431a4ac3042",
        ),
    ] {
        let path = original(file, hash);
        let result = env.exec_named(
            &std::fs::read_to_string(&path).unwrap(),
            &format!("@{}", path.display()),
        );
        eprintln!("load {file}: {result:?}");
        assert_errors(&env);
        result.unwrap();
    }
    let popup = original(
        "StaticPopup.xml",
        "a575f35cc8e9e3a585c6ccd5db40f650dbd4f153e62be6736f1101c3f33dde7b",
    );
    load_xml(&env, &popup);
    let xml = original(
        "QuestLogFrame.xml",
        "530c396e36af4b23a725086d37b6cb68abaf1966db77ba26c4e3fed3ce76f154",
    );
    let whole = std::fs::read_to_string(xml).unwrap();
    let fixture = include_str!("fixtures/wrath-abandon-button.xml");
    assert!(
        whole.contains(
            fixture
                .strip_prefix("<Ui>\n")
                .unwrap()
                .strip_suffix("\n</Ui>\n")
                .unwrap()
        )
    );
    load_xml(
        &env,
        &Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/wrath-abandon-button.xml"),
    );
    env
}

fn snapshot(env: &WowLuaEnv) -> String {
    let sim = env.state().borrow();
    format!(
        "{:?}|{:?}|{:?}|{:?}|{}|{}",
        sim.quest_log,
        sim.quest_log_entries,
        sim.pending_quest_offer,
        sim.events.pending(),
        sim.player.money,
        sim.player.xp
    )
}

#[test]
fn original_button_cancel_then_accept_marked_target() {
    let env = seed_env();
    exec(&env, "SelectQuestLogEntry(1)");
    let before = snapshot(&env);
    exec(&env, "QuestLogFrameAbandonButton:Click()");
    assert_eq!(snapshot(&env), before, "marking must not delete or notify");
    exec(
        &env,
        r#"
        local popup = StaticPopup_FindVisible('ABANDON_QUEST')
        assert(popup and popup:IsShown(), 'original popup shown')
        assert(string.find(popup.text:GetText(), 'Wrath abandon first', 1, true), 'marked supplied title')
        popup.button2:Click()
        assert(not popup:IsShown(), 'real cancel button dismisses')
    "#,
    );
    assert_eq!(snapshot(&env), before, "cancel preserves all state");
    exec(
        &env,
        "QuestLogFrameAbandonButton:Click(); SelectQuestLogEntry(2)",
    );
    let history = env
        .state()
        .borrow()
        .quest_log_entries
        .completed_quest_ids
        .clone();
    let economy = {
        let sim = env.state().borrow();
        (sim.player.money, sim.player.xp)
    };
    exec(
        &env,
        "assert(GetAbandonQuestName() == 'Wrath abandon first'); StaticPopup_FindVisible('ABANDON_QUEST').button1:Click()",
    );
    {
        let sim = env.state().borrow();
        assert_eq!(sim.quest_log, vec![990202]);
        assert_eq!(
            sim.quest_log_entries
                .entries
                .iter()
                .map(|row| row.quest_id)
                .collect::<Vec<_>>(),
            vec![990202]
        );
        assert_eq!(sim.quest_log_entries.completed_quest_ids, history);
        assert_eq!((sim.player.money, sim.player.xp), economy);
        assert!(
            !sim.quest_log_entries
                .objective_progress
                .contains_key(&990101)
        );
        assert!(
            !sim.quest_log_entries
                .background_materials
                .contains_key(&990101)
        );
        assert!(
            !sim.quest_log_entries
                .readiness
                .auto_complete
                .contains(&990101)
        );
        assert!(!sim.quest_log_entries.readiness.notified.contains(&990101));
        assert!(
            !sim.quest_log_entries
                .readiness
                .popups
                .iter()
                .any(|(id, _)| *id == 990101)
        );
        assert!(
            sim.quest_log_entries
                .objective_progress
                .contains_key(&990202)
        );
        assert!(
            sim.quest_log_entries
                .background_materials
                .contains_key(&990202)
        );
        assert!(
            sim.quest_log_entries
                .readiness
                .auto_complete
                .contains(&990202)
        );
    }
    exec(
        &env,
        r#"
        assert(#AbandonEvents == 1 and AbandonEvents[1][1] == 'QUEST_LOG_UPDATE', 'Wrath log notification, no modern removal event')
        assert(AbandonEvents[1][2] == 1 and AbandonEvents[1][3] == 'Wrath abandon second', 'listener sees committed log')
        assert(AbandonEvents[1][4] == 0, 'argumentless log update')
        StaticPopup1Button1:Click()
        AbandonQuest()
        assert(#AbandonEvents == 1, 'repeat does not notify')
    "#,
    );
    assert_eq!(env.state().borrow().quest_log, vec![990202]);
}

#[test]
fn original_selection_zero_is_nondestructive() {
    let env = seed_env();
    let before = snapshot(&env);
    exec(&env, "QuestLog_SetSelection(0)");
    assert_eq!(snapshot(&env), before);
    assert!(env.state().borrow().abandon_quest_id.is_none());
}

#[test]
fn original_nonzero_selection_is_nondestructive() {
    let env = seed_env();
    let before = snapshot(&env);
    exec(&env, "QuestLog_SetSelection(1)");
    assert_eq!(snapshot(&env), before);
}
