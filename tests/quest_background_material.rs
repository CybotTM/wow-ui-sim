//! Supplied quest material and original nil-to-Parchment consumer behavior.
#![cfg(any(feature = "client-retail", feature = "client-wowforever"))]

use wow_ui_sim::lua_api::WowLuaEnv;

fn material_env() -> WowLuaEnv {
    let cache = wow_ui_sim::blizzard_ui_sync::default_cache_addons_path().unwrap();
    let (env, _) = crate::common::blizzard_addon_harness::build_blizzard_addon_closure_env(
        &cache,
        &["Blizzard_AccessibilityTemplates", "Blizzard_UIPanels_Game"],
        &[],
    );
    env.exec("C_CVar.SetCVar('questTextContrast', '0')")
        .unwrap();
    env
}

fn supplied_material(env: &WowLuaEnv) -> Option<String> {
    // Missing publication is tolerated only at the invocation boundary for RED.
    env.eval("local query = rawget(_G, 'GetQuestBackgroundMaterial'); if type(query) == 'function' then return query() end")
        .unwrap()
}

#[test]
fn absent_quest_material_uses_original_parchment_consumer_default() {
    let env = material_env();
    let material: Option<String> = env.eval("if type(rawget(_G, 'GetQuestBackgroundMaterial')) == 'function' then return QuestFrame_GetMaterial() end").unwrap();
    assert_eq!(
        material.as_deref(),
        Some("Parchment"),
        "original consumer must reach its documented nil-material default"
    );
    assert_eq!(supplied_material(&env), None);
    let default: bool = env
        .eval("return select(2, QuestFrame_GetMaterial())")
        .unwrap();
    assert!(default);
    let arity: i32 = env
        .eval("return select('#', GetQuestBackgroundMaterial())")
        .unwrap();
    assert_eq!(arity, 1);
}

#[test]
fn selected_supplied_quest_material_is_live_environment_local_and_not_invented() {
    let env = material_env();
    let other = material_env();
    {
        let mut sim = env.state().borrow_mut();
        sim.quest_log_entries
            .background_materials
            .insert(80000, "Stone".into());
        sim.selected_quest_log_id = Some(80000);
    }
    assert_eq!(supplied_material(&env).as_deref(), Some("Stone"));
    assert_eq!(supplied_material(&other), None);
    let consumer: (String, bool) = env.eval("return QuestFrame_GetMaterial()").unwrap();
    assert_eq!(consumer, ("Stone".into(), false));
    env.state().borrow_mut().selected_quest_log_id = Some(80001);
    assert_eq!(supplied_material(&env), None);
    env.state().borrow_mut().selected_quest_log_id = Some(80000);
    env.exec("SetAbandonQuest()").unwrap();
    env.state().borrow_mut().selected_quest_log_id = Some(80000);
    assert_eq!(
        supplied_material(&env),
        None,
        "removed quest must not retain stale supplied material"
    );
    {
        let mut sim = env.state().borrow_mut();
        sim.selected_quest_log_id = Some(80001);
        sim.quest_log_entries
            .background_materials
            .insert(80001, "Stone".into());
    }
    env.exec("assert(A_Admin.TurnInSelectedQuest())").unwrap();
    env.state().borrow_mut().selected_quest_log_id = Some(80001);
    assert_eq!(
        supplied_material(&env),
        None,
        "turn-in clears the consumed material snapshot"
    );
}
