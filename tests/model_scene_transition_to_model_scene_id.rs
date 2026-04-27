//! Integration tests for `ModelScene:TransitionToModelSceneID`.
//!
//! Drives `AlliedRacesFrameMixin:UpdateModel`
//! (`Blizzard_AlliedRacesFrameUI.lua:142-147`): clear scene → transition to
//! scene 727 → look up the per-race actor by tag. Mirrors
//! `ModelSceneMixin:TransitionToModelSceneID`
//! (`vendor/wow-ui-source/Interface/AddOns/Blizzard_SharedXML/ModelSceneMixin.lua:71`).
//!
//! The simulator does not render 3D content; the contract this test pins
//! down is the actor-pool round-trip: after a transition, the script tags
//! declared for that scene must resolve via `GetActorByTag`, and the chosen
//! `modelSceneID` must persist on the scene's table so a Blizzard-mixin
//! `:Reset()` (which re-reads `self.modelSceneID`) sees a stable value.

use wow_ui_sim::lua_api::WowLuaEnv;

const ALLIED_RACES_SCENE_ID: i64 = 727;

#[test]
fn transition_method_exists_on_model_scene() {
    let env = WowLuaEnv::new().expect("env");
    let kind: String = env
        .eval(
            r#"
            local scene = CreateFrame("ModelScene", "TestSceneTransitionExists")
            return type(scene.TransitionToModelSceneID)
            "#,
        )
        .unwrap();
    assert_eq!(kind, "function");
}

#[test]
fn transition_populates_actors_so_get_actor_by_tag_resolves_each_seeded_tag() {
    let env = WowLuaEnv::new().expect("env");
    env.exec(
        r#"
            local scene = CreateFrame("ModelScene", "TestSceneTransitionPopulates")
            scene:TransitionToModelSceneID(727, 1, 0, true)
            PLAYER_ACTOR = scene:GetActorByTag("player")
            VOIDELF_F_ACTOR = scene:GetActorByTag("voidelf-female")
            EARTHEN_F_ACTOR = scene:GetActorByTag("earthendwarf-female")
        "#,
    )
    .unwrap();
    let player_present: bool = env.eval("return PLAYER_ACTOR ~= nil").unwrap();
    let voidelf_present: bool = env.eval("return VOIDELF_F_ACTOR ~= nil").unwrap();
    let earthen_present: bool = env.eval("return EARTHEN_F_ACTOR ~= nil").unwrap();
    assert!(player_present, "the \"player\" fallback must be seeded");
    assert!(
        voidelf_present,
        "voidelf-female is in Actor_X_ModelID and must be seeded"
    );
    assert!(
        earthen_present,
        "earthendwarf-female is in Actor_X_ModelID and must be seeded"
    );
    let _ = ALLIED_RACES_SCENE_ID;
}

#[test]
fn transition_writes_model_scene_id_onto_self() {
    let env = WowLuaEnv::new().expect("env");
    let scene_id: f64 = env
        .eval(
            r#"
            local scene = CreateFrame("ModelScene", "TestSceneTransitionStoresID")
            scene:TransitionToModelSceneID(727, 1, 0, true)
            return scene.modelSceneID
            "#,
        )
        .unwrap();
    assert_eq!(
        scene_id as i64, 727,
        "TransitionToModelSceneID must persist self.modelSceneID for :Reset()"
    );
}

#[test]
fn transition_to_unknown_scene_id_is_a_noop() {
    let env = WowLuaEnv::new().expect("env");
    env.exec(
        r#"
            local scene = CreateFrame("ModelScene", "TestSceneTransitionUnknown")
            scene:TransitionToModelSceneID(999999, 1, 0, true)
            FETCHED = scene:GetActorByTag("player")
            STORED_ID = scene.modelSceneID
        "#,
    )
    .unwrap();
    let actor_nil: bool = env.eval("return FETCHED == nil").unwrap();
    let id_nil: bool = env.eval("return STORED_ID == nil").unwrap();
    assert!(
        actor_nil,
        "an unknown scene id must leave the actor pool empty"
    );
    assert!(
        id_nil,
        "an unknown scene id must not stamp self.modelSceneID, matching the early return at ModelSceneMixin.lua:73"
    );
}

#[test]
fn transition_to_same_scene_id_without_force_skips_rebuild() {
    let env = WowLuaEnv::new().expect("env");
    env.exec(
        r#"
            local scene = CreateFrame("ModelScene", "TestSceneTransitionSameNoForce")
            scene:TransitionToModelSceneID(727, 1, 0, true)
            local first = scene:GetActorByTag("voidelf-female")
            scene:TransitionToModelSceneID(727, 1, 0, false)
            local second = scene:GetActorByTag("voidelf-female")
            STABLE = (first == second) and (first ~= nil)
        "#,
    )
    .unwrap();
    let stable: bool = env.eval("return STABLE").unwrap();
    assert!(
        stable,
        "re-entering the same scene without force must not churn actor handles"
    );
}

#[test]
fn transition_with_force_replaces_actor_handles_even_for_same_scene_id() {
    let env = WowLuaEnv::new().expect("env");
    env.exec(
        r#"
            local scene = CreateFrame("ModelScene", "TestSceneTransitionForce")
            scene:TransitionToModelSceneID(727, 1, 0, true)
            local first = scene:GetActorByTag("voidelf-female")
            scene:TransitionToModelSceneID(727, 1, 0, true)
            local second = scene:GetActorByTag("voidelf-female")
            DIFFERENT = (first ~= second) and (first ~= nil) and (second ~= nil)
        "#,
    )
    .unwrap();
    let different: bool = env.eval("return DIFFERENT").unwrap();
    assert!(
        different,
        "forceEvenIfSame=true must rebuild the actor pool, matching the mixin contract"
    );
}

#[test]
fn allied_races_update_model_pattern_resolves_actor_for_any_race_tag() {
    // Mirrors AlliedRacesFrameMixin:UpdateModel: ClearScene, Transition,
    // GetActorByTag(<race-tag>) || GetActorByTag("player").
    let env = WowLuaEnv::new().expect("env");
    env.exec(
        r#"
            local scene = CreateFrame("ModelScene", "TestSceneTransitionAllied")
            scene:ClearScene()
            scene:TransitionToModelSceneID(727, 1, 0, true)
            local actor = scene:GetActorByTag("voidelf-female") or scene:GetActorByTag("player")
            FOUND = (actor ~= nil and actor:GetParent():GetName() == "TestSceneTransitionAllied")
        "#,
    )
    .unwrap();
    let found: bool = env.eval("return FOUND").unwrap();
    assert!(
        found,
        "AlliedRacesFrameMixin:UpdateModel relies on at least one race tag (or the player fallback) resolving after a transition"
    );
}

#[test]
fn transition_does_not_invoke_reset_callback_directly() {
    // Per-mixin contract: only `:Reset()` invokes `self.resetCallback`,
    // not `:TransitionToModelSceneID`. AlliedRaces' callback recurses
    // back through TransitionToModelSceneID, so firing the callback here
    // would loop forever.
    let env = WowLuaEnv::new().expect("env");
    env.exec(
        r#"
            CALLBACK_FIRED = 0
            local scene = CreateFrame("ModelScene", "TestSceneTransitionNoCallback")
            scene:SetResetCallback(function() CALLBACK_FIRED = CALLBACK_FIRED + 1 end)
            scene:TransitionToModelSceneID(727, 1, 0, true)
        "#,
    )
    .unwrap();
    let fired: f64 = env.eval("return CALLBACK_FIRED").unwrap();
    assert_eq!(
        fired as i64, 0,
        "TransitionToModelSceneID must not invoke resetCallback (Blizzard mixin only fires it from :Reset())"
    );
}
