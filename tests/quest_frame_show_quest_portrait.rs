//! Integration tests for `QuestFrame_ShowQuestPortrait` and
//! `QuestFrame_HideQuestPortrait` globals registered by
//! `globals/quest_verbs.rs`.

use wow_ui_sim::lua_api::WowLuaEnv;

fn env_with_model_scene_and_parent() -> WowLuaEnv {
    let env = WowLuaEnv::new().expect("WowLuaEnv init");
    env.exec(
        r#"
        QuestModelScene = CreateFrame("Frame", "QuestModelScene")
        ParentDialog = CreateFrame("Frame", "ParentDialog", UIParent)
        "#,
    )
    .unwrap();
    env
}

fn quest_model_scene_id(env: &WowLuaEnv) -> u64 {
    env.state()
        .borrow()
        .widgets
        .get_id_by_name("QuestModelScene")
        .expect("QuestModelScene frame")
}

fn parent_dialog_id(env: &WowLuaEnv) -> u64 {
    env.state()
        .borrow()
        .widgets
        .get_id_by_name("ParentDialog")
        .expect("ParentDialog frame")
}

#[test]
fn show_records_state_with_all_args() {
    let env = env_with_model_scene_and_parent();
    env.exec(
        r#"
        QuestFrame_ShowQuestPortrait(ParentDialog, 11, 22, 33,
            "Greetings, traveler.", "Innkeeper Allison", -7.5, 14.0, true)
        "#,
    )
    .unwrap();
    let st = env.state().borrow();
    let portrait = st.quest_portrait_state.as_ref().expect("state recorded");
    assert_eq!(portrait.portrait_display_id, 11);
    assert_eq!(portrait.mount_portrait_display_id, 22);
    assert_eq!(portrait.model_scene_id, 33);
    assert_eq!(portrait.text, "Greetings, traveler.");
    assert_eq!(portrait.name, "Innkeeper Allison");
    assert_eq!(portrait.x, -7.5);
    assert_eq!(portrait.y, 14.0);
    assert!(portrait.hide_model);
    assert_eq!(portrait.parent_frame_id, Some(parent_dialog_id(&env)));
}

#[test]
fn show_reparents_quest_model_scene_to_parent() {
    let env = env_with_model_scene_and_parent();
    let parent_id = parent_dialog_id(&env);
    env.exec(
        r#"
        QuestFrame_ShowQuestPortrait(ParentDialog, 0, 0, 0, "", "", 0, 0, false)
        "#,
    )
    .unwrap();
    let scene_id = quest_model_scene_id(&env);
    let scene_parent = env
        .state()
        .borrow()
        .widgets
        .get(scene_id)
        .and_then(|f| f.parent_id);
    assert_eq!(scene_parent, Some(parent_id));
}

#[test]
fn hide_clears_recorded_state() {
    let env = env_with_model_scene_and_parent();
    env.exec(
        r#"
        QuestFrame_ShowQuestPortrait(ParentDialog, 1, 2, 3, "t", "n", 0, 0, false)
        QuestFrame_HideQuestPortrait()
        "#,
    )
    .unwrap();
    assert!(env.state().borrow().quest_portrait_state.is_none());
}

#[test]
fn hide_detaches_quest_model_scene() {
    let env = env_with_model_scene_and_parent();
    env.exec(
        r#"
        QuestFrame_ShowQuestPortrait(ParentDialog, 0, 0, 0, "", "", 0, 0, false)
        QuestFrame_HideQuestPortrait()
        "#,
    )
    .unwrap();
    let scene_id = quest_model_scene_id(&env);
    let scene_parent = env
        .state()
        .borrow()
        .widgets
        .get(scene_id)
        .and_then(|f| f.parent_id);
    assert_eq!(scene_parent, None);
}

#[test]
fn hide_with_no_prior_show_is_a_noop() {
    let env = env_with_model_scene_and_parent();
    env.exec("QuestFrame_HideQuestPortrait()").unwrap();
    assert!(env.state().borrow().quest_portrait_state.is_none());
}

#[test]
fn show_records_state_even_when_quest_model_scene_missing() {
    let env = WowLuaEnv::new().expect("WowLuaEnv init");
    env.exec(
        r#"
        ParentDialog = CreateFrame("Frame", "ParentDialog", UIParent)
        QuestFrame_ShowQuestPortrait(ParentDialog, 5, 6, 7, "txt", "nm", 1, 2, true)
        "#,
    )
    .unwrap();
    let st = env.state().borrow();
    let portrait = st.quest_portrait_state.as_ref().expect("state recorded");
    assert_eq!(portrait.portrait_display_id, 5);
    assert_eq!(portrait.text, "txt");
    assert!(portrait.hide_model);
}

#[test]
fn show_followed_by_show_overwrites_state() {
    let env = env_with_model_scene_and_parent();
    env.exec(
        r#"
        QuestFrame_ShowQuestPortrait(ParentDialog, 1, 2, 3, "first", "alpha", 0, 0, false)
        QuestFrame_ShowQuestPortrait(ParentDialog, 9, 8, 7, "second", "beta", 4, 5, true)
        "#,
    )
    .unwrap();
    let st = env.state().borrow();
    let portrait = st.quest_portrait_state.as_ref().expect("state recorded");
    assert_eq!(portrait.portrait_display_id, 9);
    assert_eq!(portrait.text, "second");
    assert_eq!(portrait.name, "beta");
    assert!(portrait.hide_model);
}

#[test]
fn show_records_nil_parent_when_called_without_frame_arg() {
    let env = env_with_model_scene_and_parent();
    env.exec(
        r#"
        QuestFrame_ShowQuestPortrait(nil, 1, 2, 3, "t", "n", 0, 0, false)
        "#,
    )
    .unwrap();
    let st = env.state().borrow();
    let portrait = st.quest_portrait_state.as_ref().expect("state recorded");
    assert_eq!(portrait.parent_frame_id, None);
}
