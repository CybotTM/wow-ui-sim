//! End-to-end tests for `C_TradeSkillUI.SetRecipeTracked`.
//!
//! Drives the Lua-visible API and checks both the SimState side-effect and
//! the `TRACKED_RECIPE_UPDATE` event payload.

use crate::common;

use std::path::PathBuf;
use tempfile::tempdir;
use wow_ui_sim::loader::load_addon_with_saved_vars;
use wow_ui_sim::lua_api::WowLuaEnv;
use wow_ui_sim::saved_variables::SavedVariablesManager;

fn admin_toc_path() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("Interface/AddOns/Admin/Admin.toc")
}

fn load_admin_with_saved_vars(env: &WowLuaEnv, mgr: &mut SavedVariablesManager) {
    load_addon_with_saved_vars(&env.loader_env(), &admin_toc_path(), mgr)
        .expect("Admin addon should load with SavedVariables");
}

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
fn set_recipe_tracked_fires_update_event_only_on_real_change() {
    let env = WowLuaEnv::new().expect("Lua env");

    env.exec(
        r#"
        __recipe_events = {}
        local f = CreateFrame("Frame")
        f:RegisterEvent("TRACKED_RECIPE_UPDATE")
        f:SetScript("OnEvent", function(_, _event, recipeID, tracked)
            __recipe_events[#__recipe_events + 1] = { id = recipeID, tracked = tracked }
        end)
        "#,
    )
    .expect("listener");

    env.exec("C_TradeSkillUI.SetRecipeTracked(7, true, false)")
        .expect("first set");
    // No-op: already tracked.
    env.exec("C_TradeSkillUI.SetRecipeTracked(7, true, false)")
        .expect("second set");
    env.exec("C_TradeSkillUI.SetRecipeTracked(7, false, false)")
        .expect("untrack");

    let count: i64 = env
        .eval("return #__recipe_events")
        .expect("read event count");
    assert_eq!(count, 2, "only real transitions fire the event");

    let last_id: i64 = env.eval("return __recipe_events[2].id").expect("event id");
    let last_tracked: bool = env
        .eval("return __recipe_events[2].tracked")
        .expect("event tracked flag");
    assert_eq!(last_id, 7, "event carries recipe id");
    assert!(!last_tracked, "second transition was untrack");
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

#[test]
fn tracked_recipes_roundtrip_through_admin_saved_variables() {
    let dir = tempdir().expect("tempdir");

    {
        let env = WowLuaEnv::new().expect("Lua env");
        let mut mgr = SavedVariablesManager::with_storage_dir(dir.path().to_path_buf());
        load_admin_with_saved_vars(&env, &mut mgr);

        env.exec("C_TradeSkillUI.SetRecipeTracked(101, true, false)")
            .expect("track normal recipe");
        env.exec("C_TradeSkillUI.SetRecipeTracked(202, true, true)")
            .expect("track recraft recipe");
        env.exec("C_TradeSkillUI.SetRecipeTracked(303, true, false)")
            .expect("track transient recipe");
        env.exec("C_TradeSkillUI.SetRecipeTracked(303, false, false)")
            .expect("untrack transient recipe");

        let (saved_normal, saved_recraft): (Vec<i64>, Vec<i64>) = env
            .eval("return WowSimTrackedRecipesDB.normal, WowSimTrackedRecipesDB.recrafting")
            .expect("saved variables table should be readable");
        assert_eq!(
            saved_normal,
            vec![101],
            "normal DB bucket should stay in sync"
        );
        assert_eq!(
            saved_recraft,
            vec![202],
            "recrafting DB bucket should stay in sync",
        );

        env.loader_env()
            .with_state(|state| mgr.save_addon(state, "Admin"))
            .expect("save Admin SavedVariables");
    }

    {
        let env = WowLuaEnv::new().expect("Lua env");
        let mut mgr = SavedVariablesManager::with_storage_dir(dir.path().to_path_buf());
        load_admin_with_saved_vars(&env, &mut mgr);

        let (loaded_normal, loaded_recraft): (Vec<i64>, Vec<i64>) = env
            .eval("return WowSimTrackedRecipesDB.normal, WowSimTrackedRecipesDB.recrafting")
            .expect("saved variables should reload before login");
        assert_eq!(loaded_normal, vec![101]);
        assert_eq!(loaded_recraft, vec![202]);

        let sim = env.state().borrow();
        assert!(
            sim.tracked_recipes.list(false).is_empty(),
            "SimState should stay empty until PLAYER_LOGIN replay",
        );
        assert!(
            sim.tracked_recipes.list(true).is_empty(),
            "recrafting state should stay empty until PLAYER_LOGIN replay",
        );
        drop(sim);

        env.exec(
            r#"
            __recipe_events = {}
            local f = CreateFrame("Frame")
            f:RegisterEvent("TRACKED_RECIPE_UPDATE")
            f:SetScript("OnEvent", function(_, _event, recipeID, tracked)
                __recipe_events[#__recipe_events + 1] = { id = recipeID, tracked = tracked }
            end)
            "#,
        )
        .expect("listener");
        env.fire_event("VARIABLES_LOADED")
            .expect("VARIABLES_LOADED should dispatch");
        env.fire_event("PLAYER_LOGIN")
            .expect("PLAYER_LOGIN should dispatch");

        let sim = env.state().borrow();
        assert_eq!(
            sim.tracked_recipes.list(false),
            &[101],
            "PLAYER_LOGIN should replay the saved normal recipe list",
        );
        assert_eq!(
            sim.tracked_recipes.list(true),
            &[202],
            "PLAYER_LOGIN should replay the saved recraft list",
        );
        drop(sim);

        let count: i64 = env
            .eval("return #__recipe_events")
            .expect("read event count");
        assert_eq!(
            count, 2,
            "replaying saved recipes should emit tracker updates for both buckets",
        );
    }
}
