//! PTR's selected, unmodified UnitFrame name consumer and supplied policies.

#[cfg(feature = "client-ptr")]
mod original_consumer {
    use sha2::{Digest, Sha256};
    use wow_ui_sim::lua_api::{WowLuaEnv, state::TargetInfo};

    fn load_unit_frame() -> WowLuaEnv {
        let root = crate::common::blizzard_addon_harness::blizzard_ui_dir();
        let bytes = std::fs::read(root.join("Blizzard_UnitFrame/Mainline/UnitFrame.lua")).unwrap();
        assert_eq!(
            format!("{:x}", Sha256::digest(&bytes)),
            "c5e69e8f28e5d8bd62e5916048ee5d6df97d4ec76d0915fe87113134499458a8"
        );
        let (env, loaded) = crate::common::blizzard_addon_harness::build_blizzard_addon_closure_env(
            &root,
            &["Blizzard_UnitFrame"],
            &[],
        );
        assert!(loaded.iter().any(|name| name == "Blizzard_UnitFrame"));
        env.exec(r#"
            local info = debug.getinfo(UnitFrame_Update, 'S')
            assert(info.what == 'Lua')
            assert(info.source:gsub('\\', '/'):match('Blizzard_UnitFrame/Mainline/UnitFrame.lua$'))
            local naming = debug.getinfo(NameUtil.GetUnitFirstName, 'S')
            assert(naming.source:gsub('\\', '/'):match('Blizzard_FrameXMLUtil/NameUtil.lua$'))
            RegionalNameFrame = CreateFrame('Frame')
            RegionalNameFrame.unit = 'player'
            RegionalNameFrame.name = RegionalNameFrame:CreateFontString()
        "#).unwrap();
        env
    }

    fn supply_names(env: &WowLuaEnv) {
        let mut sim = env.state().borrow_mut();
        sim.player.name = "Ada-Lovelace".into();
        sim.player.should_display_surname = Some(false);
        sim.current_target = Some(TargetInfo {
            unit_id: "target".into(),
            name: "Mira-Stone".into(),
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
            creature_type: "Humanoid".into(),
            reaction: 1,
            has_vehicle_ui: false,
            interaction: Default::default(),
        });
    }

    fn assert_display(env: &WowLuaEnv, unit: &str, expected: &str) {
        env.exec(&format!(
            "RegionalNameFrame.unit = '{unit}'; UnitFrame_Update(RegionalNameFrame); \
             assert(RegionalNameFrame.name:GetText() == '{expected}', \
             tostring(RegionalNameFrame.name:GetText()))"
        ))
        .unwrap();
    }

    #[test]
    fn unit_frame_tracks_regional_and_surname_policy_transitions() {
        let env = load_unit_frame();
        supply_names(&env);
        assert_display(&env, "player", "Ada-Lovelace");
        env.state().borrow_mut().player.regional_unique_names_enabled = true;
        assert_display(&env, "player", "Ada");
        assert_display(&env, "target", "Mira-Stone");
        env.state().borrow_mut().player.should_display_surname = Some(true);
        assert_display(&env, "player", "Ada-Lovelace");
        assert_display(&env, "target", "Mira-Stone");
        env.state().borrow_mut().player.should_display_surname = Some(false);
        assert_display(&env, "player", "Ada");
        env.state().borrow_mut().player.regional_unique_names_enabled = false;
        assert_display(&env, "player", "Ada-Lovelace");
        assert_display(&env, "target", "Mira-Stone");
        env.exec("assert(select('#', RegionalUniqueNamesEnabled()) == 1); assert(select('#', C_PlayerInfo.ShouldDisplaySurname()) == 1)").unwrap();
        let other = WowLuaEnv::new().unwrap();
        other.exec("assert(RegionalUniqueNamesEnabled() == false); assert(not pcall(C_PlayerInfo.ShouldDisplaySurname))").unwrap();
    }
}
