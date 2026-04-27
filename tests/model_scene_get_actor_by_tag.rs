//! Integration tests for `ModelScene:GetActorByTag(tag)`.
//!
//! Surfaced by `AlliedRacesFrameMixin:UpdateModel`
//! (`Blizzard_AlliedRacesFrameUI.lua:144`), which hands the per-race
//! tag (e.g. `"voidelf-female"`) and falls back to `"player"`. Mirrors
//! `ModelSceneMixin:GetActorByTag`
//! (`Blizzard_SharedXML/ModelSceneMixin.lua:136`), which simply reads
//! `self.tagToActor[tag]`.
//!
//! The simulator tags actors at `CreateActor(tag)` time — passing the
//! script tag as the actor's first argument registers it for tag
//! lookup. The 3D rendering path is intentionally stubbed, so this
//! test only verifies handle identity.

use wow_ui_sim::lua_api::WowLuaEnv;

#[test]
fn get_actor_by_tag_method_exists_on_model_scene() {
    let env = WowLuaEnv::new().expect("env");
    let kind: String = env
        .eval(
            r#"
            local scene = CreateFrame("ModelScene", "TestSceneTagMethodExists")
            return type(scene.GetActorByTag)
            "#,
        )
        .unwrap();
    assert_eq!(kind, "function");
}

#[test]
fn get_actor_by_tag_returns_actor_registered_with_that_tag() {
    let env = WowLuaEnv::new().expect("env");
    env.exec(
        r#"
            local scene = CreateFrame("ModelScene", "TestSceneTagLookup")
            local registered = scene:CreateActor("voidelf-female")
            local fetched = scene:GetActorByTag("voidelf-female")
            MATCH = (fetched == registered)
        "#,
    )
    .unwrap();
    let matched: bool = env.eval("return MATCH").unwrap();
    assert!(
        matched,
        "GetActorByTag must return the same actor handle that CreateActor produced"
    );
}

#[test]
fn get_actor_by_tag_returns_nil_for_unknown_tag() {
    let env = WowLuaEnv::new().expect("env");
    env.exec(
        r#"
            local scene = CreateFrame("ModelScene", "TestSceneTagMiss")
            scene:CreateActor("voidelf")
            UNKNOWN = scene:GetActorByTag("nightborne")
        "#,
    )
    .unwrap();
    let unknown_is_nil: bool = env.eval("return UNKNOWN == nil").unwrap();
    assert!(
        unknown_is_nil,
        "an unregistered tag should resolve to nil, mirroring tagToActor[tag]"
    );
}

#[test]
fn get_actor_by_tag_returns_nil_when_tag_omitted() {
    let env = WowLuaEnv::new().expect("env");
    env.exec(
        r#"
            local scene = CreateFrame("ModelScene", "TestSceneTagOmitted")
            scene:CreateActor("alpha")
            NIL_RESULT = scene:GetActorByTag()
        "#,
    )
    .unwrap();
    let nil_result: bool = env.eval("return NIL_RESULT == nil").unwrap();
    assert!(
        nil_result,
        "calling GetActorByTag with no argument must yield nil, not error"
    );
}

#[test]
fn clear_scene_drops_tag_lookups() {
    // AlliedRaces flow: ClearScene -> TransitionToModelSceneID ->
    // GetActorByTag. Stale tag entries must not survive ClearScene or
    // a fresh transition could resolve to a discarded actor.
    let env = WowLuaEnv::new().expect("env");
    env.exec(
        r#"
            local scene = CreateFrame("ModelScene", "TestSceneTagCleared")
            scene:CreateActor("voidelf")
            scene:ClearScene()
            POST_CLEAR = scene:GetActorByTag("voidelf")
        "#,
    )
    .unwrap();
    let post_clear_nil: bool = env.eval("return POST_CLEAR == nil").unwrap();
    assert!(
        post_clear_nil,
        "ClearScene must drop the tag→actor mapping, not just the actor list"
    );
}

#[test]
fn re_registering_a_tag_replaces_the_previous_actor() {
    let env = WowLuaEnv::new().expect("env");
    env.exec(
        r#"
            local scene = CreateFrame("ModelScene", "TestSceneTagReplace")
            local first = scene:CreateActor("player")
            local second = scene:CreateActor("player")
            FETCHED = scene:GetActorByTag("player")
            FIRST_MATCH = (FETCHED == first)
            SECOND_MATCH = (FETCHED == second)
        "#,
    )
    .unwrap();
    let first_match: bool = env.eval("return FIRST_MATCH").unwrap();
    let second_match: bool = env.eval("return SECOND_MATCH").unwrap();
    assert!(
        !first_match,
        "the older actor must no longer be reachable through the reused tag"
    );
    assert!(
        second_match,
        "the most recent CreateActor with a given tag wins the lookup, matching tagToActor overwrite semantics"
    );
}

#[test]
fn tag_lookups_are_per_scene() {
    let env = WowLuaEnv::new().expect("env");
    env.exec(
        r#"
            local a = CreateFrame("ModelScene", "TestSceneTagA")
            local b = CreateFrame("ModelScene", "TestSceneTagB")
            local actor_a = a:CreateActor("shared")
            local actor_b = b:CreateActor("shared")
            FROM_A = a:GetActorByTag("shared")
            FROM_B = b:GetActorByTag("shared")
            A_DISTINCT = (FROM_A == actor_a) and (FROM_A ~= actor_b)
            B_DISTINCT = (FROM_B == actor_b) and (FROM_B ~= actor_a)
        "#,
    )
    .unwrap();
    let a_ok: bool = env.eval("return A_DISTINCT").unwrap();
    let b_ok: bool = env.eval("return B_DISTINCT").unwrap();
    assert!(a_ok, "scene A's tag table must yield A's actor, not B's");
    assert!(b_ok, "scene B's tag table must yield B's actor, not A's");
}

#[test]
fn allied_races_update_model_pattern_finds_player_fallback() {
    // Mirrors AlliedRacesFrameMixin:UpdateModel: ClearScene, then
    // (transition-style) re-create actors with race tags, then look
    // one up. Falls back to "player" if the race tag is missing —
    // this test exercises the fallback path the addon relies on.
    let env = WowLuaEnv::new().expect("env");
    env.exec(
        r#"
            local scene = CreateFrame("ModelScene", "TestSceneTagFallback")
            scene:ClearScene()
            scene:CreateActor("player")
            local actor = scene:GetActorByTag("nonexistent-race") or scene:GetActorByTag("player")
            FOUND_FALLBACK = (actor ~= nil and actor:GetParent():GetName() == "TestSceneTagFallback")
        "#,
    )
    .unwrap();
    let found: bool = env.eval("return FOUND_FALLBACK").unwrap();
    assert!(
        found,
        "the player-tag fallback used by AlliedRacesFrameMixin:UpdateModel must resolve to the scene's player actor"
    );
}
