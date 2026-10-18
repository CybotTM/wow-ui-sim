//! Wrath abandonment through the unchanged full-startup selection/button/popup consumers.
#![cfg(feature = "client-wrath")]

#[path = "common/wrath_full_ui.rs"]
mod wrath_full_ui;

use sha2::{Digest, Sha256};
use std::ops::Deref;
use wow_ui_sim::c_api::quest_objectives::QuestObjectiveProgress;
use wow_ui_sim::lua_api::WowLuaEnv;
use wow_ui_sim::lua_api::state::GossipQuestRow;

struct ConsumerEnv {
    env: WowLuaEnv,
    startup_errors: String,
}

impl Deref for ConsumerEnv {
    type Target = WowLuaEnv;

    fn deref(&self) -> &Self::Target {
        &self.env
    }
}

fn error_snapshot(env: &WowLuaEnv) -> String {
    let sim = env.state().borrow();
    format!("{:?}|{:?}", sim.lua_errors, sim.lua_error_counts)
}

fn assert_errors(env: &ConsumerEnv) {
    let errors: String = env
        .eval(r"return table.concat(AbandonErrors, '\n')")
        .unwrap();
    assert!(errors.is_empty(), "{errors}");
    assert_eq!(
        error_snapshot(env),
        env.startup_errors,
        "new consumer errors"
    );
}

fn exec(env: &ConsumerEnv, source: &str) {
    let result = env.exec(source);
    eprintln!("direct eval {source:?}: {result:?}");
    assert_errors(env);
    result.expect("original consumer and behavioral assertions");
}

fn assert_originals() {
    let toc = wow_ui_sim::client_profile::blizzard_ui_framexml_toc().unwrap();
    for (file, expected) in [
        (
            "QuestLogFrame.lua",
            "6ac6ca92b3036417082b88cc2f8b5c9fc8889f51f82aa726fe0bcd44990bc5e2",
        ),
        (
            "StaticPopup.lua",
            "1134b08e87d8577c0353895207eaec24e74e8f669775a49257a82431a4ac3042",
        ),
        (
            "StaticPopup.xml",
            "a575f35cc8e9e3a585c6ccd5db40f650dbd4f153e62be6736f1101c3f33dde7b",
        ),
        (
            "QuestLogFrame.xml",
            "530c396e36af4b23a725086d37b6cb68abaf1966db77ba26c4e3fed3ce76f154",
        ),
    ] {
        let path = toc.parent().unwrap().join(file);
        let bytes = std::fs::read(&path).unwrap();
        let hash = format!("{:x}", Sha256::digest(bytes));
        assert_eq!(hash, expected, "source drift: {}", path.display());
        eprintln!("original {} sha256={hash}", path.display());
    }
}

fn seed_env() -> ConsumerEnv {
    assert_originals();
    let env = wrath_full_ui::load_original_wrath_startup();
    let startup_errors = error_snapshot(&env);
    eprintln!("retained full-startup errors: {startup_errors}");
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
    let env = ConsumerEnv {
        env,
        startup_errors,
    };
    exec(
        &env,
        r#"
        assert(QuestLogFrameAbandonButton and StaticPopup1, 'original startup widgets')
        AbandonEvents = {}
        local observer = CreateFrame('Frame')
        for _, event in ipairs({'QUEST_LOG_UPDATE', 'QUEST_REMOVED', 'QUEST_ACCEPTED', 'QUEST_TURNED_IN'}) do observer:RegisterEvent(event) end
        observer:SetScript('OnEvent', function(_, event, ...)
            AbandonEvents[#AbandonEvents+1] = {event, GetNumQuestLogEntries(), GetQuestLogTitle(1), select('#', ...)}
        end)
    "#,
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
    let clicked = env.exec("QuestLogFrameAbandonButton:Click()");
    eprintln!("original button result: {clicked:?}");
    assert_eq!(snapshot(&env), before, "marking must not delete or notify");
    assert_errors(&env);
    clicked.unwrap();
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
    let selected = env.exec("QuestLog_SetSelection(1)");
    eprintln!("original full selection result: {selected:?}");
    assert_eq!(snapshot(&env), before, "ordinary selection must not delete");
    assert_errors(&env);
    selected.unwrap();
}
