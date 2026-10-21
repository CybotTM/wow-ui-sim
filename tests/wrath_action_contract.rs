#![cfg(feature = "client-wrath")]
//! Laptop-authenticated 12340 query shapes exercised with native callback fixtures.
use crate::common::action_bar_fixture::wrath_env;
use wow_ui_sim::lua_api::state::PetActionSlot;

#[test]
fn shapeshift_info_uses_native_name_and_numeric_flags() {
    let env = wrath_env();
    env.exec(
        r#"
        ShapeshiftBar_UpdateState()
        assert(not ShapeshiftButton1:GetChecked())
        local texture, name, active, castable = GetShapeshiftFormInfo("1")
        assert(select('#', GetShapeshiftFormInfo(1)) == 4)
        assert(type(texture) == "string" and type(name) == "string")
        assert(active == nil and castable == 1)
        assert(select('#', GetShapeshiftFormInfo(999)) == 0)
        assert(select('#', GetShapeshiftFormInfo(-1)) == 0)
        assert(not pcall(GetShapeshiftFormInfo, 0))
        assert(not pcall(GetShapeshiftFormInfo, "not a number"))
    "#,
    )
    .unwrap();
    {
        let mut state = env.state().borrow_mut();
        state.shapeshift_forms[0].is_active = true;
        state.shapeshift_forms[0].is_castable = false;
    }
    env.exec(
        r#"
        ShapeshiftBar_UpdateState()
        assert(ShapeshiftButton1:GetChecked())
        local _, _, active, castable = GetShapeshiftFormInfo(1)
        assert(active == 1 and castable == 1)
    "#,
    )
    .unwrap();
}

#[test]
fn pet_info_uses_native_seven_value_spell_and_token_shapes() {
    let env = wrath_env();
    env.exec(
        r#"
        PetActionBar_Update(PetActionBarFrame)
        assert(not PetActionButton1Icon:IsShown())
        assert(select('#', GetPetActionInfo(1)) == 7)
        for i = 1, 7 do assert(select(i, GetPetActionInfo(1)) == nil) end
        assert(select('#', GetPetActionInfo(999)) == 7)
        assert(not pcall(GetPetActionInfo, "invalid"))
    "#,
    )
    .unwrap();
    env.state().borrow_mut().pet_actions[0] = PetActionSlot {
        has_action: true,
        name: Some("Flash of Light".into()),
        texture: Some("Interface/Icons/Spell_Holy_FlashHeal".into()),
        is_active: true,
        auto_cast_allowed: true,
        auto_cast_enabled: true,
        spell_id: Some(19750),
        ..Default::default()
    };
    env.fire_event("PET_BAR_UPDATE").unwrap();
    env.exec(r#"
        assert(PetActionButton1.tooltipName == "Flash of Light" and PetActionButton1Icon:IsShown())
        local name, subtext, texture, token, active, allowed, enabled = GetPetActionInfo("1")
        assert(name == "Flash of Light" and subtext == "" and texture == "Interface/Icons/Spell_Holy_FlashHeal")
        assert(token == nil and active == nil and allowed == 1 and enabled == 1)
    "#).unwrap();
    {
        let mut state = env.state().borrow_mut();
        let slot = &mut state.pet_actions[0];
        slot.name = Some("PET_ACTION_FOLLOW".into());
        slot.texture = Some("PET_FOLLOW_TEXTURE".into());
        slot.is_token = true;
    }
    env.fire_event("PET_BAR_UPDATE").unwrap();
    env.exec(
        r#"
        local name, subtext, texture, token, active, allowed, enabled = GetPetActionInfo(1)
        assert(name == "PET_ACTION_FOLLOW" and subtext == nil and texture == "PET_FOLLOW_TEXTURE")
        assert(token == 1 and active == 1 and allowed == nil and enabled == nil)
    "#,
    )
    .unwrap();
}

#[test]
fn action_range_uses_native_numeric_result_and_nil_unknowns() {
    let env = wrath_env();
    env.state().borrow_mut().action_bars.insert(1, 19750);
    env.exec("TargetUnit('player')").unwrap();
    let guid: String = env.eval("return UnitGUID('player')").unwrap();
    for (in_range, expected) in [(false, 0), (true, 1)] {
        env.state()
            .borrow_mut()
            .action_spell_ranges
            .entry(19750)
            .or_default()
            .in_range_by_guid
            .insert(guid.clone(), in_range);
        let result: i32 = env.eval("return IsActionInRange('1', 'player')").unwrap();
        assert_eq!(result, expected);
    }
    env.exec(
        r#"
        assert(select('#', IsActionInRange(1)) == 1 and IsActionInRange(1) == 1)
        for _, slot in ipairs({0, -1, 145, "invalid", false}) do
            assert(select('#', IsActionInRange(slot)) == 1 and IsActionInRange(slot) == nil)
        end
        assert(IsActionInRange() == nil and IsActionInRange(1, "missing") == nil)
    "#,
    )
    .unwrap();
    env.state().borrow_mut().action_bars.remove(&1);
    env.exec("assert(IsActionInRange(1, 'player') == nil)")
        .unwrap();
}
