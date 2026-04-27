//! Integration tests for `Actor:SetModelByCreatureDisplayID`.
//!
//! Drives `AlliedRacesFrameMixin:UpdateModel`
//! (`Blizzard_AlliedRacesFrameUI.lua:146`):
//!
//! ```lua
//! local actor = self.ModelScene:GetActorByTag(actorTag);
//! if actor then
//!     actor:SetModelByCreatureDisplayID(modelID, true);
//! end
//! ```
//!
//! Real WoW renders the creature; the simulator only needs to swallow the
//! call without erroring and round-trip the display ID through
//! `GetDisplayInfo` so addons that read the value back observe what they
//! wrote.

use wow_ui_sim::lua_api::WowLuaEnv;

#[test]
fn set_model_by_creature_display_id_method_exists_on_actor() {
    let env = WowLuaEnv::new().expect("env");
    let kind: String = env
        .eval(
            r#"
            local scene = CreateFrame("ModelScene", "TestActorMethodExists")
            scene:TransitionToModelSceneID(727, 1, 0, true)
            local actor = scene:GetActorByTag("player")
            return type(actor.SetModelByCreatureDisplayID)
            "#,
        )
        .unwrap();
    assert_eq!(kind, "function");
}

#[test]
fn set_model_by_creature_display_id_records_display_id_on_actor() {
    let env = WowLuaEnv::new().expect("env");
    let display_id: f64 = env
        .eval(
            r#"
            local scene = CreateFrame("ModelScene", "TestActorRecordsDisplayId")
            scene:TransitionToModelSceneID(727, 1, 0, true)
            local actor = scene:GetActorByTag("voidelf-female")
            actor:SetModelByCreatureDisplayID(82735, true)
            return actor:GetDisplayInfo()
            "#,
        )
        .unwrap();
    assert_eq!(
        display_id as i32, 82735,
        "GetDisplayInfo must round-trip the value passed to SetModelByCreatureDisplayID"
    );
}

#[test]
fn set_model_by_creature_display_id_accepts_omitted_use_cached_flag() {
    let env = WowLuaEnv::new().expect("env");
    let display_id: f64 = env
        .eval(
            r#"
            local scene = CreateFrame("ModelScene", "TestActorOmittedFlag")
            scene:TransitionToModelSceneID(727, 1, 0, true)
            local actor = scene:GetActorByTag("player")
            actor:SetModelByCreatureDisplayID(12345)
            return actor:GetDisplayInfo()
            "#,
        )
        .unwrap();
    assert_eq!(
        display_id as i32, 12345,
        "the useCachedModelIfAvailable arg is optional in real WoW; calling without it must not error"
    );
}

#[test]
fn allied_races_update_model_chain_runs_without_error() {
    // Mirrors the full AlliedRacesFrameMixin:UpdateModel flow:
    // ClearScene → TransitionToModelSceneID → GetActorByTag →
    // SetModelByCreatureDisplayID(modelID, true).
    let env = WowLuaEnv::new().expect("env");
    let display_id: f64 = env
        .eval(
            r#"
            local scene = CreateFrame("ModelScene", "TestActorAlliedRacesChain")
            local modelID = 82735  -- voidelf-female live model id
            scene:ClearScene()
            scene:TransitionToModelSceneID(727, 1, 0, true)
            local actor = scene:GetActorByTag("voidelf-female") or scene:GetActorByTag("player")
            actor:SetModelByCreatureDisplayID(modelID, true)
            return actor:GetDisplayInfo()
            "#,
        )
        .unwrap();
    assert_eq!(
        display_id as i32, 82735,
        "the AlliedRaces UpdateModel chain must complete and the addon must see the model id it passed in"
    );
}

#[test]
fn set_model_by_creature_display_id_overwrites_previous_display_id() {
    let env = WowLuaEnv::new().expect("env");
    let display_id: f64 = env
        .eval(
            r#"
            local scene = CreateFrame("ModelScene", "TestActorOverwrite")
            scene:TransitionToModelSceneID(727, 1, 0, true)
            local actor = scene:GetActorByTag("player")
            actor:SetModelByCreatureDisplayID(11111, true)
            actor:SetModelByCreatureDisplayID(22222, false)
            return actor:GetDisplayInfo()
            "#,
        )
        .unwrap();
    assert_eq!(
        display_id as i32, 22222,
        "consecutive calls must overwrite — actors swap appearance every UpdateModel"
    );
}
