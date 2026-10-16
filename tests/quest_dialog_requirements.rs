//! Typed-empty temporary domains, exercised through the unchanged QuestFrame.
#![cfg(any(feature = "client-retail", feature = "client-wowforever"))]
use wow_ui_sim::lua_api::WowLuaEnv;

fn dialog_env() -> WowLuaEnv {
    let cache = wow_ui_sim::blizzard_ui_sync::default_cache_addons_path().unwrap();
    let (env, _) = crate::common::blizzard_addon_harness::build_blizzard_addon_closure_env(
        &cache,
        &[
            "Blizzard_AccessibilityTemplates",
            "Blizzard_FrameXML",
            "Blizzard_UIPanels_Game",
            "Blizzard_ObjectiveTracker",
            "Blizzard_SplashFrame",
        ],
        &[],
    );
    env.state().borrow_mut().selected_quest_log_id = Some(80000);
    // Explicit first-dialog presentation state; no original functions are replaced.
    env.exec("C_CVar.SetCVar('questTextContrast', '0'); QuestInfoFrame.mapView = false")
        .unwrap();
    env
}

#[test]
fn original_progress_panel_displays_empty_unsupplied_dialogue_and_requirements() {
    let env = dialog_env();
    let before = env.state().borrow().lua_error_counts.clone();
    env.exec("QuestFrameProgressPanel_OnShow(QuestFrameProgressPanel)")
        .expect("original progress panel must not call absent globals");
    let text: String = env.eval("return QuestProgressText:GetText()").unwrap();
    assert_eq!(text, "");
    env.exec("assert(not QuestProgressRequiredMoneyFrame:IsShown()); assert(not QuestProgressRequiredItemsText:IsShown()); for i=1, MAX_REQUIRED_ITEMS do assert(not _G['QuestProgressItem'..i]:IsShown()) end").unwrap();
    assert_eq!(env.state().borrow().lua_error_counts, before);
}

#[test]
fn original_progress_items_refresh_has_zero_unsupplied_cost_and_rows() {
    let env = dialog_env();
    env.exec("QuestFrameProgressItems_Update()")
        .expect("original requirements refresh must not call absent globals");
    let values: (i32, i32, i32) = env
        .eval("return GetNumQuestItems(), GetNumQuestCurrencies(), GetQuestMoneyToGet()")
        .unwrap();
    assert_eq!(values, (0, 0, 0));
    env.exec("assert(not QuestProgressRequiredMoneyFrame:IsShown()); for i=1, MAX_REQUIRED_ITEMS do assert(not _G['QuestProgressItem'..i]:IsShown()) end").unwrap();
}

#[test]
fn original_reward_panel_has_no_invented_rewards_or_turnin_portrait() {
    let env = dialog_env();
    let before = env.state().borrow().lua_error_counts.clone();
    env.exec("QuestFrameRewardPanel_OnShow()")
        .expect("original reward panel must not call absent portrait getter");
    let empty: bool = env
        .eval("return not QuestInfoFrame.rewardsFrame:IsShown()")
        .unwrap();
    assert!(empty, "unsupplied reward panel must remain empty");
    let arity: i32 = env
        .eval("return select('#', GetQuestPortraitTurnIn())")
        .unwrap();
    assert_eq!(arity, 0);
    assert_eq!(env.state().borrow().lua_error_counts, before);
}
