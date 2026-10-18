//! Unit liveness and aura queries share the supplied simulator state.
use wow_ui_sim::lua_api::{WowLuaEnv, state::TargetInfo};

fn env() -> WowLuaEnv {
    WowLuaEnv::new().expect("unit existence environment")
}

fn target() -> TargetInfo {
    TargetInfo {
        unit_id: "target".into(),
        name: "Armor Training Dummy".into(),
        class_index: 1,
        level: 80,
        health: 100,
        health_max: 100,
        power: 0,
        power_max: 0,
        power_type: 1,
        power_type_name: "RAGE".into(),
        is_player: false,
        is_enemy: true,
        guid: "Creature-0-0-0-0-31146-1".into(),
        classification: "normal".into(),
        creature_type: "Mechanical".into(),
        reaction: 1,
        has_vehicle_ui: false,
        interaction: Default::default(),
    }
}

#[test]
fn pet_existence_tracks_summon_and_dismiss_from_rust_state() {
    let env = env();
    env.exec("assert(UnitExists('player')); assert(not UnitExists('pet'), 'no summoned pet')")
        .unwrap();
    env.state().borrow_mut().pet.icon = Some("Interface\\Icons\\Ability_Hunter_Pet_Wolf".into());
    env.exec("assert(UnitExists('pet'), 'summoned wolf exists')")
        .unwrap();
    env.state().borrow_mut().pet.icon = None;
    env.exec("assert(not UnitExists('pet'), 'dismissed wolf is absent'); assert(UnitExists('player'))")
        .unwrap();
}

#[test]
fn vehicle_existence_tracks_player_occupancy_from_rust_state() {
    let env = env();
    env.exec("assert(not UnitExists('vehicle'), 'no occupied vehicle')")
        .unwrap();
    env.state().borrow_mut().player.in_vehicle = true;
    env.exec("assert(UnitExists('vehicle'), 'occupied vehicle exists')")
        .unwrap();
    env.state().borrow_mut().player.in_vehicle = false;
    env.exec("assert(not UnitExists('vehicle'), 'exited vehicle is absent'); assert(UnitExists('player'))")
        .unwrap();
}

#[test]
fn target_aura_queries_follow_selection_and_clear() {
    let env = env();
    let assert_empty = || {
        env.exec(
            r#"
            assert(not UnitExists('target'))
            assert(UnitAura('target', 1, 'HARMFUL') == nil, 'absent target legacy aura')
            assert(UnitDebuff('target', 1) == nil)
            assert(C_UnitAuras.GetAuraDataByIndex('target', 1, 'HARMFUL') == nil,
                'absent target indexed aura')
            assert(C_UnitAuras.GetAuraDataByAuraInstanceID('target', 1) == nil)
            assert(select('#', C_UnitAuras.GetAuraSlots('target', 'HARMFUL')) == 1)
            "#,
        )
        .unwrap();
    };
    assert_empty();
    env.state().borrow_mut().current_target = Some(target());
    env.exec(
        r#"
        assert(UnitExists('target'))
        assert(UnitAura('target', 1, 'HARMFUL') == 'Weakened Armor')
        assert(UnitDebuff('target', 2) == 'Expose Weakness')
        local first = C_UnitAuras.GetAuraDataByIndex('target', 1, 'HARMFUL')
        assert(first.name == 'Weakened Armor' and first.spellId == 113746)
        local second = C_UnitAuras.GetAuraDataByAuraInstanceID('target', 2)
        assert(second.name == 'Expose Weakness' and second.spellId == 53338)
        local continuation, one, two = C_UnitAuras.GetAuraSlots('target', 'HARMFUL')
        assert(continuation == nil and one == 1 and two == 2)
        assert(UnitAura('target', 3, 'HARMFUL') == nil)
        "#,
    )
    .unwrap();
    env.state().borrow_mut().current_target = None;
    assert_empty();
}

#[test]
fn party_aura_queries_require_active_membership_without_deleting_stored_auras() {
    let env = env();
    let name = {
        let sim = env.state().borrow();
        assert!(!sim.party_group_active);
        sim.party_members[0].buffs[0].name.clone()
    };
    let assert_empty = || {
        env.exec(
            r#"
            assert(not UnitExists('party1'))
            assert(UnitAura('party1', 1, 'HELPFUL') == nil, 'inactive party legacy aura')
            assert(C_UnitAuras.GetAuraDataByIndex('party1', 1, 'HELPFUL') == nil,
                'inactive party indexed aura')
            "#,
        )
        .unwrap();
    };
    assert_empty();
    env.state().borrow_mut().party_group_active = true;
    let (legacy, packed): (String, String) = env
        .eval("return UnitAura('party1', 1, 'HELPFUL'), C_UnitAuras.GetAuraDataByIndex('party1', 1, 'HELPFUL').name")
        .unwrap();
    assert_eq!(legacy, name);
    assert_eq!(packed, name);
    env.state().borrow_mut().party_group_active = false;
    assert_empty();
    assert_eq!(env.state().borrow().party_members[0].buffs[0].name, name);
}

#[cfg(feature = "client-retail")]
fn consumer_env(relative: &str) -> WowLuaEnv {
    let ui = crate::common::blizzard_addon_harness::blizzard_ui_dir();
    let (env, _) = crate::common::blizzard_addon_harness::build_blizzard_addon_closure_env(
        &ui,
        &["Blizzard_FrameXMLUtil"],
        &[],
    );
    let addons = wow_ui_sim::client_profile::blizzard_ui_addons_dir_under(
        std::path::Path::new(env!("CARGO_MANIFEST_DIR")),
    );
    let source = std::fs::read_to_string(addons.join(relative)).expect("cached Blizzard consumer");
    env.exec(&source).expect("unmodified Blizzard consumer loads");
    env
}

#[cfg(feature = "client-retail")]
#[test]
fn compact_unit_frame_pet_visibility_tracks_summon_and_dismiss() {
    let env = consumer_env("Blizzard_UnitFrame/Shared/CompactUnitFrame.lua");
    env.exec(
        r#"
        PetExistenceFrame = CreateFrame('Frame')
        PetExistenceFrame.unit = 'pet'
        PetExistenceFrame.displayedUnit = 'pet'
        CompactUnitFrame_UpdateVisible(PetExistenceFrame)
        assert(not PetExistenceFrame:IsShown(), 'Blizzard hides absent pet')
        assert(PetExistenceFrame.unitExists == false)
        "#,
    )
    .unwrap();
    env.state().borrow_mut().pet.icon = Some("Interface\\Icons\\Ability_Hunter_Pet_Wolf".into());
    env.exec(
        r#"
        CompactUnitFrame_UpdateVisible(PetExistenceFrame)
        assert(PetExistenceFrame:IsShown(), 'Blizzard shows summoned pet')
        assert(PetExistenceFrame.unitExists == true)
        "#,
    )
    .unwrap();
    env.state().borrow_mut().pet.icon = None;
    env.exec(
        r#"
        CompactUnitFrame_UpdateVisible(PetExistenceFrame)
        assert(not PetExistenceFrame:IsShown(), 'Blizzard hides dismissed pet')
        "#,
    )
    .unwrap();
}

#[cfg(feature = "client-retail")]
#[test]
fn blizzard_aura_util_visits_only_selected_target_debuffs() {
    let env = consumer_env("Blizzard_FrameXMLUtil/AuraUtil.lua");
    env.exec(
        r#"
        function TargetDebuffNames()
            local names = {}
            AuraUtil.ForEachAura('target', 'HARMFUL', nil, function(aura)
                names[#names + 1] = aura.name
            end, true)
            return table.concat(names, ',')
        end
        assert(TargetDebuffNames() == '', 'Blizzard consumer sees no absent-target debuffs')
        "#,
    )
    .unwrap();
    env.state().borrow_mut().current_target = Some(target());
    let names: String = env.eval("return TargetDebuffNames()").unwrap();
    assert_eq!(names, "Weakened Armor,Expose Weakness");
    env.state().borrow_mut().current_target = None;
    let names: String = env.eval("return TargetDebuffNames()").unwrap();
    assert_eq!(names, "");
}
