//! Tests for hero talent spec resolution.

use wow_ui_sim::lua_api::WowLuaEnv;

fn env() -> WowLuaEnv {
    WowLuaEnv::new().expect("Failed to create Lua environment")
}

#[test]
fn test_protection_hero_subtrees() {
    let env = env();
    // Protection (specID=66) should get Templar (48) and Lightsmith (49)
    let result: String = env
        .eval(
            r#"
            local ids, level = C_ClassTalents.GetHeroTalentSpecsForClassSpec(1, 66)
            assert(ids, "subtree IDs should not be nil")
            assert(level == 71, "unlock level should be 71")
            table.sort(ids)
            return table.concat(ids, ",")
            "#,
        )
        .unwrap();
    assert_eq!(
        result, "48,49",
        "Protection should have Templar(48) + Lightsmith(49)"
    );
}

#[test]
fn test_retribution_hero_subtrees() {
    let env = env();
    // Retribution (specID=70) should get Templar (48) and Herald of the Sun (50)
    let result: String = env
        .eval(
            r#"
            local ids, level = C_ClassTalents.GetHeroTalentSpecsForClassSpec(1, 70)
            assert(ids, "subtree IDs should not be nil")
            table.sort(ids)
            return table.concat(ids, ",")
            "#,
        )
        .unwrap();
    assert_eq!(
        result, "48,50",
        "Retribution should have Templar(48) + Herald of the Sun(50)"
    );
}

#[test]
fn test_holy_hero_subtrees() {
    let env = env();
    // Holy (specID=65) should get Lightsmith (49) and Herald of the Sun (50)
    let result: String = env
        .eval(
            r#"
            local ids, level = C_ClassTalents.GetHeroTalentSpecsForClassSpec(1, 65)
            assert(ids, "subtree IDs should not be nil")
            table.sort(ids)
            return table.concat(ids, ",")
            "#,
        )
        .unwrap();
    assert_eq!(
        result, "49,50",
        "Holy should have Lightsmith(49) + Herald of the Sun(50)"
    );
}

#[test]
fn test_subtree_info_has_selection_node_ids() {
    let env = env();
    // Templar (subtree 48) should have subTreeSelectionNodeIDs
    let result: String = env
        .eval(
            r#"
            local info = C_Traits.GetSubTreeInfo(1, 48)
            assert(info, "subtree info should not be nil")
            assert(info.name == "Templar", "name should be Templar, got: " .. tostring(info.name))
            assert(info.subTreeSelectionNodeIDs, "should have subTreeSelectionNodeIDs")
            assert(#info.subTreeSelectionNodeIDs > 0, "should have at least one selection node")
            return tostring(#info.subTreeSelectionNodeIDs)
            "#,
        )
        .unwrap();
    assert!(
        result.parse::<i32>().unwrap() > 0,
        "Templar should have selection nodes"
    );
}

#[test]
fn test_activate_hero_spec_lightsmith() {
    let env = env();
    // Activate Lightsmith by selecting entry 123361 on node 99838 (Protection's selection node)
    // Then verify: nodeInfo has the selection, and GetActiveHeroTalentSpec returns subtree 49
    let result: String = env
        .eval(
            r#"
            -- Clear auto-selected hero spec so we start from a clean state
            C_Traits.SetSelection(1, 99838, nil)
            assert(C_ClassTalents.GetActiveHeroTalentSpec() == nil, "should be nil after clearing")

            -- Selection node 99838 should be visible (Protection spec)
            local nodeInfo = C_Traits.GetNodeInfo(1, 99838)
            assert(nodeInfo.isVisible, "selection node should be visible for Protection")
            assert(nodeInfo.activeEntry.entryID == 0, "no entry selected yet")

            -- Activate Lightsmith: select entry 123361 (subtree 49) on node 99838
            local ok = C_Traits.SetSelection(1, 99838, 123361)
            assert(ok, "SetSelection should succeed")

            -- Verify nodeInfo now reflects the selection
            local updated = C_Traits.GetNodeInfo(1, 99838)
            assert(updated.activeEntry.entryID == 123361, "entry should be 123361, got: " .. tostring(updated.activeEntry.entryID))
            assert(updated.ranksPurchased == 1, "should have rank 1 after selection")

            -- Verify GetActiveHeroTalentSpec returns Lightsmith's subtree
            local active = C_ClassTalents.GetActiveHeroTalentSpec()
            assert(active == 49, "active hero spec should be 49 (Lightsmith), got: " .. tostring(active))

            return "ok"
            "#,
        )
        .unwrap();
    assert_eq!(result, "ok");
}

#[test]
fn test_tcontains_nil_safe() {
    let env = env();
    // tContains should return false when passed nil table
    let result: bool = env.eval("return tContains(nil, 42)").unwrap();
    assert!(!result, "tContains(nil, x) should return false");
}

#[test]
fn test_set_atlas_numeric_element_id() {
    let env = env();
    // SetAtlas should accept numeric element IDs (e.g. iconElementID from subtree info)
    // Element 26680 = "talents-heroclass-paladin-lightsmith"
    let result: String = env
        .eval(
            r#"
            local t = UIParent:CreateTexture(nil, "ARTWORK")
            t:SetAtlas(26680)
            return t:GetAtlas() or "nil"
            "#,
        )
        .unwrap();
    assert_eq!(
        result, "talents-heroclass-paladin-lightsmith",
        "SetAtlas should resolve numeric element ID to atlas name"
    );
}

#[test]
fn test_class_talents_switch_methods_update_seeded_spec_and_loadout_state() {
    let env = env();
    let result: String = env
        .eval(
            r#"
            local protectionConfigs = C_ClassTalents.GetConfigIDsBySpecID(66)
            assert(#protectionConfigs == 2, "Protection should expose two seeded loadouts")
            assert(C_ClassTalents.GetActiveConfigID() == protectionConfigs[1], "Protection default loadout should start active")

            C_ClassTalents.SwitchToLoadoutByName("Protection Mythic+")
            local protectionMythic = C_ClassTalents.GetActiveConfigID()
            assert(protectionMythic == protectionConfigs[2], "SwitchToLoadoutByName should pick the named Protection loadout")
            assert(C_ClassTalents.GetLastSelectedSavedConfigID(66) == protectionMythic, "last selected Protection config should update")
            assert(C_Traits.GetConfigInfo(protectionMythic).name == "Protection Mythic+", "config info should expose the active loadout name")

            C_ClassTalents.SwitchToSpecializationByName("Holy")
            assert(GetSpecialization() == 1, "Holy should become the active specialization")
            local holyConfigs = C_ClassTalents.GetConfigIDsBySpecID(65)
            assert(#holyConfigs == 2, "Holy should expose two seeded loadouts")
            assert(C_ClassTalents.GetActiveConfigID() == holyConfigs[1], "SwitchToSpecializationByName should activate Holy's default loadout")
            assert(C_ClassTalents.GetLastSelectedSavedConfigID(65) == holyConfigs[1], "Holy's last selected config should track the active default")
            local holyHeroSpecs = C_ClassTalents.GetHeroTalentSpecsForClassSpec(1, 65)
            assert(tContains(holyHeroSpecs, C_ClassTalents.GetActiveHeroTalentSpec()), "Holy reset should auto-select one of Holy's hero subtrees")

            C_ClassTalents.SwitchToLoadoutByIndex(2)
            assert(C_ClassTalents.GetActiveConfigID() == holyConfigs[2], "SwitchToLoadoutByIndex should use the current specialization's loadout order")
            assert(C_Traits.GetConfigInfo(holyConfigs[2]).name == "Holy Raid", "Holy loadout names should be queryable")

            C_ClassTalents.SwitchToSpecializationByIndex(3)
            assert(GetSpecialization() == 3, "Retribution should become the active specialization")
            local retributionConfigs = C_ClassTalents.GetConfigIDsBySpecID(70)
            assert(C_ClassTalents.GetActiveConfigID() == retributionConfigs[1], "Retribution default loadout should become active")
            assert(C_ClassTalents.GetLastSelectedSavedConfigID(70) == retributionConfigs[1], "Retribution's last selected config should update")
            local retributionHeroSpecs = C_ClassTalents.GetHeroTalentSpecsForClassSpec(1, 70)
            assert(tContains(retributionHeroSpecs, C_ClassTalents.GetActiveHeroTalentSpec()), "Retribution reset should auto-select one of Retribution's hero subtrees")

            return table.concat({
                tostring(protectionMythic),
                tostring(holyConfigs[2]),
                tostring(retributionConfigs[1]),
            }, ",")
            "#,
        )
        .unwrap();

    assert_eq!(result, "202,102,301");
}
