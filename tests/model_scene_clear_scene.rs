//! Integration tests for `ModelScene:ClearScene()`.
//!
//! Surfaced by `AlliedRacesFrameMixin:UpdateModel`
//! (`Blizzard_AlliedRacesFrameUI.lua:142`), which calls
//! `self.ModelScene:ClearScene()` before re-creating actors for the
//! newly-selected race. Mirrors `ModelSceneMixin:ReleaseAllActors`
//! (`Blizzard_SharedXML/ModelSceneMixin.lua:217`) — every actor must be
//! detached from the scene so subsequent CreateActor calls start clean.

use wow_ui_sim::lua_api::WowLuaEnv;

#[test]
fn clear_scene_method_exists_on_model_scene() {
    let env = WowLuaEnv::new().expect("env");
    let kind: String = env
        .eval(
            r#"
            local scene = CreateFrame("ModelScene", "TestSceneClearMethodExists")
            return type(scene.ClearScene)
            "#,
        )
        .unwrap();
    assert_eq!(kind, "function");
}

#[test]
fn clear_scene_drops_every_actor_from_the_pool() {
    let env = WowLuaEnv::new().expect("env");
    env.exec(
        r#"
            local scene = CreateFrame("ModelScene", "TestSceneClearDropsActors")
            scene:CreateActor("alpha")
            scene:CreateActor("beta")
            scene:CreateActor("gamma")
            BEFORE = scene:GetNumActors()
            scene:ClearScene()
            AFTER = scene:GetNumActors()
            POST_INDEX = scene:GetActorAtIndex(1)
        "#,
    )
    .unwrap();
    let before: f64 = env.eval("return BEFORE").unwrap();
    let after: f64 = env.eval("return AFTER").unwrap();
    let post_index_is_nil: bool = env.eval("return POST_INDEX == nil").unwrap();
    assert_eq!(
        before, 3.0,
        "three actors should be active before ClearScene"
    );
    assert_eq!(after, 0.0, "ClearScene must leave the pool empty");
    assert!(
        post_index_is_nil,
        "GetActorAtIndex(1) must return nil after ClearScene drains the pool"
    );
}

#[test]
fn clear_scene_on_empty_pool_is_a_noop() {
    let env = WowLuaEnv::new().expect("env");
    env.exec(
        r#"
            local scene = CreateFrame("ModelScene", "TestSceneClearEmpty")
            scene:ClearScene()
            COUNT = scene:GetNumActors()
        "#,
    )
    .unwrap();
    let count: f64 = env.eval("return COUNT").unwrap();
    assert_eq!(count, 0.0, "ClearScene on an empty pool must not error");
}

#[test]
fn clear_scene_lets_subsequent_create_actor_repopulate() {
    // Mirrors AlliedRacesFrameMixin:UpdateModel — ClearScene followed
    // by TransitionToModelSceneID (which itself runs CreateActor under
    // the hood). The replacement actors must show up at fresh indices
    // starting from 1 with no ghosts from the prior call.
    let env = WowLuaEnv::new().expect("env");
    env.exec(
        r#"
            local scene = CreateFrame("ModelScene", "TestSceneClearRepopulate")
            scene:CreateActor("old1")
            scene:CreateActor("old2")
            scene:ClearScene()
            local fresh = scene:CreateActor("fresh")
            COUNT = scene:GetNumActors()
            FIRST_IS_FRESH = (scene:GetActorAtIndex(1) == fresh)
        "#,
    )
    .unwrap();
    let count: f64 = env.eval("return COUNT").unwrap();
    let first_is_fresh: bool = env.eval("return FIRST_IS_FRESH").unwrap();
    assert_eq!(
        count, 1.0,
        "after ClearScene + one CreateActor, exactly one actor should be active"
    );
    assert!(
        first_is_fresh,
        "the first slot after ClearScene must hold the newly-created actor, not a stale entry"
    );
}

#[test]
fn clear_scene_is_per_scene() {
    let env = WowLuaEnv::new().expect("env");
    env.exec(
        r#"
            local a = CreateFrame("ModelScene", "TestSceneClearA")
            local b = CreateFrame("ModelScene", "TestSceneClearB")
            a:CreateActor("a1")
            a:CreateActor("a2")
            b:CreateActor("b1")
            a:ClearScene()
            A_COUNT = a:GetNumActors()
            B_COUNT = b:GetNumActors()
        "#,
    )
    .unwrap();
    let a_count: f64 = env.eval("return A_COUNT").unwrap();
    let b_count: f64 = env.eval("return B_COUNT").unwrap();
    assert_eq!(
        a_count, 0.0,
        "ClearScene on scene A must drain only A's pool"
    );
    assert_eq!(
        b_count, 1.0,
        "ClearScene on scene A must leave scene B's actors untouched"
    );
}

#[test]
fn clear_scene_detaches_actors_from_scene_parent() {
    // ReleaseAllActors hides and resets the actor; the closest analog
    // in the simulator is reparenting it away from the scene. This
    // matches what scene_take_actor does for a single actor and
    // prevents the orphaned actor from continuing to render under the
    // scene's transform.
    let env = WowLuaEnv::new().expect("env");
    env.exec(
        r#"
            local scene = CreateFrame("ModelScene", "TestSceneClearReparent")
            local actor = scene:CreateActor("solo")
            PARENT_BEFORE = actor:GetParent()
            scene:ClearScene()
            PARENT_AFTER = actor:GetParent()
        "#,
    )
    .unwrap();
    let scene_parent_match: bool = env
        .eval("return PARENT_BEFORE and PARENT_BEFORE:GetName() == 'TestSceneClearReparent'")
        .unwrap();
    let detached_after: bool = env.eval("return PARENT_AFTER == nil").unwrap();
    assert!(
        scene_parent_match,
        "before ClearScene the actor should still be parented to the scene"
    );
    assert!(
        detached_after,
        "after ClearScene the actor must be detached from the scene"
    );
}
