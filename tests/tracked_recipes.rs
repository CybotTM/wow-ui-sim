//! End-to-end tests for `C_TradeSkillUI.SetRecipeTracked`.
//!
//! Drives the Lua-visible API and checks both the SimState side-effect and
//! the `TRACKED_RECIPE_UPDATE` event payload.

mod common;

use wow_ui_sim::lua_api::WowLuaEnv;

#[test]
fn set_recipe_tracked_mutates_sim_state() {
    let env = WowLuaEnv::new().expect("Lua env");

    env.exec("C_TradeSkillUI.SetRecipeTracked(12345, true, false)")
        .expect("SetRecipeTracked");

    let sim = env.state().borrow();
    assert_eq!(
        sim.tracked_recipes.list(false),
        &[12345],
        "normal bucket must contain the tracked recipe",
    );
    assert!(
        sim.tracked_recipes.list(true).is_empty(),
        "recrafting bucket must stay untouched",
    );
}

#[test]
fn set_recipe_tracked_queues_update_event_only_on_real_change() {
    let env = WowLuaEnv::new().expect("Lua env");
    // Drop any startup-pushed events so the assertion only sees recipe-tracker activity.
    env.state().borrow_mut().events.drain();

    env.exec("C_TradeSkillUI.SetRecipeTracked(7, true, false)")
        .expect("first set");
    // No-op: already tracked.
    env.exec("C_TradeSkillUI.SetRecipeTracked(7, true, false)")
        .expect("second set");
    env.exec("C_TradeSkillUI.SetRecipeTracked(7, false, false)")
        .expect("untrack");

    let drained = env.state().borrow_mut().events.drain();
    let updates: Vec<_> = drained
        .iter()
        .filter(|e| e.name == "TRACKED_RECIPE_UPDATE")
        .collect();
    assert_eq!(updates.len(), 2, "only real transitions queue the event");

    let last = updates.last().expect("two events");
    assert_eq!(last.args.len(), 2, "event carries recipe id + tracked flag");
}

#[test]
fn is_recipe_tracked_reflects_set_recipe_tracked_state() {
    let env = WowLuaEnv::new().expect("Lua env");

    // Untouched state: nothing tracked.
    let absent: bool = env
        .eval("return C_TradeSkillUI.IsRecipeTracked(101, false)")
        .expect("query absent recipe");
    assert!(!absent);

    env.exec("C_TradeSkillUI.SetRecipeTracked(101, true, false)")
        .expect("track normal");

    let normal_present: bool = env
        .eval("return C_TradeSkillUI.IsRecipeTracked(101, false)")
        .expect("query normal");
    let recraft_absent: bool = env
        .eval("return C_TradeSkillUI.IsRecipeTracked(101, true)")
        .expect("query recraft");
    assert!(normal_present, "tracked normal recipe must report tracked");
    assert!(!recraft_absent, "recrafting bucket must stay independent");

    env.exec("C_TradeSkillUI.SetRecipeTracked(101, false, false)")
        .expect("untrack");
    let after_untrack: bool = env
        .eval("return C_TradeSkillUI.IsRecipeTracked(101, false)")
        .expect("re-query");
    assert!(!after_untrack, "untracked recipe must report not tracked");
}

#[test]
fn set_recipe_tracked_keeps_normal_and_recrafting_independent() {
    let env = WowLuaEnv::new().expect("Lua env");

    env.exec("C_TradeSkillUI.SetRecipeTracked(99, true, false)")
        .expect("normal");
    env.exec("C_TradeSkillUI.SetRecipeTracked(99, true, true)")
        .expect("recraft");
    env.exec("C_TradeSkillUI.SetRecipeTracked(99, false, false)")
        .expect("untrack normal");

    let sim = env.state().borrow();
    assert!(
        !sim.tracked_recipes.contains(99, false),
        "untracking normal should not touch recrafting",
    );
    assert!(
        sim.tracked_recipes.contains(99, true),
        "recrafting bucket survives normal-bucket untrack",
    );
}
