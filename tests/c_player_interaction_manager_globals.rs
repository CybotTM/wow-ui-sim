//! Integration tests for the `C_PlayerInteractionManager` surface registered
//! in `src/c_api/c_player_interaction_manager.rs`. The surface drives
//! `AlliedRacesFrameMixin:OnHide` (`Blizzard_AlliedRacesFrameUI.lua:156`),
//! which calls `ClearInteraction(Enum.PlayerInteractionType.AlliedRaceDetailsGiver)`
//! and expects an `ALLIED_RACE_CLOSE` event for any open listener.

use wow_ui_sim::lua_api::WowLuaEnv;

const ALLIED_RACE_DETAILS_GIVER: i32 = 9;

#[test]
fn clear_interaction_namespace_and_method_exist() {
    let env = WowLuaEnv::new().expect("env");
    let kind: String = env
        .eval("return type(C_PlayerInteractionManager.ClearInteraction)")
        .unwrap();
    assert_eq!(kind, "function");
}

#[test]
fn allied_race_details_giver_enum_value_is_nine() {
    let env = WowLuaEnv::new().expect("env");
    let value: f64 = env
        .eval("return Enum.PlayerInteractionType.AlliedRaceDetailsGiver")
        .unwrap();
    assert_eq!(
        value, ALLIED_RACE_DETAILS_GIVER as f64,
        "AlliedRaceDetailsGiver must match the canonical PlayerInteractionType ordinal"
    );
}

#[test]
fn clear_interaction_removes_active_entry() {
    let env = WowLuaEnv::new().expect("env");
    env.state()
        .borrow_mut()
        .active_player_interactions
        .insert(ALLIED_RACE_DETAILS_GIVER);

    env.exec(&format!(
        "C_PlayerInteractionManager.ClearInteraction({ALLIED_RACE_DETAILS_GIVER})"
    ))
    .unwrap();

    let still_active = env
        .state()
        .borrow()
        .active_player_interactions
        .contains(&ALLIED_RACE_DETAILS_GIVER);
    assert!(!still_active, "ClearInteraction should remove the entry");
}

#[test]
fn clear_interaction_fires_allied_race_close_when_active() {
    let env = WowLuaEnv::new().expect("env");
    env.state()
        .borrow_mut()
        .active_player_interactions
        .insert(ALLIED_RACE_DETAILS_GIVER);

    env.exec(
        r#"
        ALLIED_RACE_CLOSE_FIRED = false
        local listener = CreateFrame("Frame", "AlliedRaceCloseListener")
        listener:RegisterEvent("ALLIED_RACE_CLOSE")
        listener:SetScript("OnEvent", function(self, event)
            if event == "ALLIED_RACE_CLOSE" then
                ALLIED_RACE_CLOSE_FIRED = true
            end
        end)
        "#,
    )
    .unwrap();

    env.exec(&format!(
        "C_PlayerInteractionManager.ClearInteraction({ALLIED_RACE_DETAILS_GIVER})"
    ))
    .unwrap();

    let fired: bool = env.eval("return ALLIED_RACE_CLOSE_FIRED").unwrap();
    assert!(
        fired,
        "ALLIED_RACE_CLOSE should fire when AlliedRaceDetailsGiver is cleared"
    );
}

#[test]
fn clear_interaction_does_not_fire_event_when_not_active() {
    let env = WowLuaEnv::new().expect("env");
    assert!(env.state().borrow().active_player_interactions.is_empty());

    env.exec(
        r#"
        ALLIED_RACE_CLOSE_FIRED = false
        local listener = CreateFrame("Frame", "AlliedRaceCloseListener")
        listener:RegisterEvent("ALLIED_RACE_CLOSE")
        listener:SetScript("OnEvent", function(self, event)
            if event == "ALLIED_RACE_CLOSE" then
                ALLIED_RACE_CLOSE_FIRED = true
            end
        end)
        "#,
    )
    .unwrap();

    env.exec(&format!(
        "C_PlayerInteractionManager.ClearInteraction({ALLIED_RACE_DETAILS_GIVER})"
    ))
    .unwrap();

    let fired: bool = env.eval("return ALLIED_RACE_CLOSE_FIRED").unwrap();
    assert!(
        !fired,
        "ALLIED_RACE_CLOSE should not fire if no interaction was active"
    );
}

#[test]
fn clear_interaction_with_unmapped_type_clears_silently() {
    let env = WowLuaEnv::new().expect("env");
    env.state()
        .borrow_mut()
        .active_player_interactions
        .insert(1);

    env.exec(
        r#"
        UNEXPECTED_EVENT_FIRED = false
        local listener = CreateFrame("Frame", "UnknownInteractionListener")
        listener:RegisterEvent("ALLIED_RACE_CLOSE")
        listener:SetScript("OnEvent", function() UNEXPECTED_EVENT_FIRED = true end)
        "#,
    )
    .unwrap();

    env.exec("C_PlayerInteractionManager.ClearInteraction(1)")
        .unwrap();

    let still_active = env.state().borrow().active_player_interactions.contains(&1);
    assert!(
        !still_active,
        "Unmapped types should still clear from the set"
    );

    let fired: bool = env.eval("return UNEXPECTED_EVENT_FIRED").unwrap();
    assert!(!fired, "Unmapped types should not fire ALLIED_RACE_CLOSE");
}

#[test]
fn clear_interaction_missing_arg_is_a_noop() {
    let env = WowLuaEnv::new().expect("env");
    env.state()
        .borrow_mut()
        .active_player_interactions
        .insert(ALLIED_RACE_DETAILS_GIVER);

    env.exec("C_PlayerInteractionManager.ClearInteraction()")
        .unwrap();

    let still_active = env
        .state()
        .borrow()
        .active_player_interactions
        .contains(&ALLIED_RACE_DETAILS_GIVER);
    assert!(
        still_active,
        "Missing arg must not silently clear arbitrary entries"
    );
}
