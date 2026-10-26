#![cfg(feature = "client-mists")]

use crate::common::blizzard_addon_harness::{
    blizzard_ui_dir, load_blizzard_addon_closure_into_env, new_blizzard_addon_env,
};
use wow_ui_sim::c_api::engraving_inputs::{EngravingData, EngravingInputs};
use wow_ui_sim::lua_api::WowLuaEnv;

fn supplied_rune() -> EngravingData {
    EngravingData {
        skill_line_ability_id: 41001,
        item_enchantment_id: 42002,
        name: "Supplied test rune".into(),
        icon_texture: 134400,
        equipment_slot: 5,
        level: 12,
        learned_ability_spell_ids: vec![43003, 44004],
    }
}

fn supplied_env(enabled: bool) -> WowLuaEnv {
    let ui = blizzard_ui_dir();
    let env = new_blizzard_addon_env(&ui);
    {
        let mut state = env.state().borrow_mut();
        state.engraving.enabled = Some(enabled);
        state.engraving.mode_enabled = Some(true);
        state.engraving.current_rune = Some(None);
        for slot in 1..=36 {
            state
                .engraving
                .inventory_eligibility
                .insert((0, slot), true);
            state.engraving.inventory_runes.insert((0, slot), None);
            state
                .engraving
                .current_cast_eligibility
                .insert((0, slot), false);
        }
        state
            .engraving
            .inventory_runes
            .insert((0, 1), Some(supplied_rune()));
    }
    load_blizzard_addon_closure_into_env(
        &env,
        &ui,
        &["Blizzard_UIParentPanelManager", "Blizzard_UIPanels_Game"],
        &[],
    );
    env.exec(r#"
        SetCVar("alwaysShowRuneIcons", "0")
        A_Admin.ClearBags()
        A_Admin.AddBagItem(0, 1, 6948, 1)
        OpenBag(0)
        assert(ContainerFrame1:GetID() == 0 and ContainerFrame1:IsShown())
        local source = debug.getinfo(ContainerFrame_RefreshRuneIcons, "S").source:gsub("\\", "/")
        assert(source:find("Blizzard_UIPanels_Game/Classic/ContainerFrame_Shared.lua", 1, true), source)
    "#).expect("load and open the original Mists bag with pre-startup supplied input");
    env
}

#[test]
fn original_bag_rune_icons_follow_supplied_mode_availability_and_slots() {
    let env = supplied_env(true);
    env.exec(r#"
        ContainerFrame_RefreshRuneIcons(ContainerFrame1, C_Engraving.GetEngravingModeEnabled())
        assert(ContainerFrame1Item1SubIconTexture:IsShown())
        assert(ContainerFrame1Item1SubIconTexture:GetTexture() == 134400)
        assert(ContainerFrame1Item2SubIconTexture:GetTexture() == "Interface\\PaperDoll\\UI-Backpack-EmptySlot")
    "#).unwrap();
    env.state().borrow_mut().engraving.mode_enabled = Some(false);
    env.exec(
        r#"
        ContainerFrame_RefreshRuneIcons(ContainerFrame1, C_Engraving.GetEngravingModeEnabled())
        assert(not ContainerFrame1Item1SubIconTexture:IsShown())
        SetCVar("alwaysShowRuneIcons", "1")
        ContainerFrame_RefreshRuneIcons(ContainerFrame1, C_Engraving.GetEngravingModeEnabled())
        assert(ContainerFrame1Item1SubIconTexture:IsShown())
        SetCVar("alwaysShowRuneIcons", "0")
    "#,
    )
    .unwrap();
    assert_eq!(env.state().borrow().engraving.refresh_requests, 1);
    {
        let mut state = env.state().borrow_mut();
        state.engraving.mode_enabled = Some(true);
        state.engraving.inventory_eligibility.insert((0, 1), false);
    }
    env.exec(
        r#"
        ContainerFrame_RefreshRuneIcons(ContainerFrame1, C_Engraving.GetEngravingModeEnabled())
        assert(not ContainerFrame1Item1SubIconTexture:IsShown())
        assert(ContainerFrame1Item2SubIconTexture:IsShown())
    "#,
    )
    .unwrap();
    env.state().borrow_mut().engraving.enabled = Some(false);
    env.exec(
        r#"
        ContainerFrame_RefreshRuneIcons(ContainerFrame1, C_Engraving.GetEngravingModeEnabled())
        assert(not ContainerFrame1Item2SubIconTexture:IsShown())
    "#,
    )
    .unwrap();
}

#[test]
fn original_bag_targeting_projects_supplied_current_cast_eligibility() {
    let env = supplied_env(true);
    env.exec(r#"
        ContainerFrame_EngravingTargetingModeChanged(ContainerFrame1, C_Engraving.GetCurrentRuneCast() ~= nil)
        assert(not ContainerFrame1Item1IconTexture:IsDesaturated())
    "#).unwrap();
    {
        let mut state = env.state().borrow_mut();
        state.engraving.current_rune = Some(Some(supplied_rune()));
        state
            .engraving
            .current_cast_eligibility
            .insert((0, 1), true);
    }
    env.exec(r#"
        ContainerFrame_EngravingTargetingModeChanged(ContainerFrame1, C_Engraving.GetCurrentRuneCast() ~= nil)
        assert(not ContainerFrame1Item1IconTexture:IsDesaturated())
        assert(ContainerFrame1Item2IconTexture:IsDesaturated())
    "#).unwrap();
    env.state().borrow_mut().engraving.current_rune = Some(None);
    env.exec(r#"
        ContainerFrame_EngravingTargetingModeChanged(ContainerFrame1, C_Engraving.GetCurrentRuneCast() ~= nil)
        assert(not ContainerFrame1Item2IconTexture:IsDesaturated())
    "#).unwrap();
}

#[test]
fn direct_engraving_queries_project_records_and_reject_unsupplied_inputs() {
    let env = WowLuaEnv::new().unwrap();
    env.exec(r#"
        local function missing(call, name)
            local ok, message = pcall(call)
            assert(not ok and message:find("C_Engraving." .. name, 1, true), tostring(message))
            assert(message:find("unsupplied", 1, true), message)
        end
        missing(function() C_Engraving.IsEngravingEnabled() end, "IsEngravingEnabled")
        missing(function() C_Engraving.GetEngravingModeEnabled() end, "GetEngravingModeEnabled")
        missing(function() C_Engraving.GetCurrentRuneCast() end, "GetCurrentRuneCast")
        missing(function() C_Engraving.IsInventorySlotEngravable(0, 1) end, "IsInventorySlotEngravable")
        missing(function() C_Engraving.GetRuneForInventorySlot(0, 1) end, "GetRuneForInventorySlot")
        missing(function() C_Engraving.IsInventorySlotEngravableByCurrentRuneCast(0, 1) end, "IsInventorySlotEngravableByCurrentRuneCast")
    "#).unwrap();
    {
        let mut state = env.state().borrow_mut();
        state.engraving.enabled = Some(false);
        state.engraving.mode_enabled = Some(true);
        state.engraving.current_rune = Some(Some(supplied_rune()));
        state
            .engraving
            .inventory_runes
            .insert((0, 1), Some(supplied_rune()));
        state.engraving.inventory_runes.insert((0, 2), None);
        state.engraving.inventory_eligibility.insert((0, 1), false);
        state
            .engraving
            .current_cast_eligibility
            .insert((0, 1), true);
    }
    env.exec(
        r#"
        assert(C_Engraving.IsEngravingEnabled() == false)
        assert(C_Engraving.GetEngravingModeEnabled() == true)
        assert(C_Engraving.IsInventorySlotEngravable(0, 1) == false)
        assert(C_Engraving.IsInventorySlotEngravableByCurrentRuneCast(0, 1) == true)
        local rune = C_Engraving.GetRuneForInventorySlot(0, 1)
        assert(rune.skillLineAbilityID == 41001 and rune.itemEnchantmentID == 42002)
        assert(rune.name == "Supplied test rune" and rune.iconTexture == 134400)
        assert(rune.equipmentSlot == 5 and rune.level == 12)
        assert(#rune.learnedAbilitySpellIDs == 2 and rune.learnedAbilitySpellIDs[2] == 44004)
        assert(C_Engraving.GetCurrentRuneCast().skillLineAbilityID == 41001)
        rune.learnedAbilitySpellIDs[2] = 0
        assert(C_Engraving.GetRuneForInventorySlot(0, 1).learnedAbilitySpellIDs[2] == 44004)
        assert(select('#', C_Engraving.GetRuneForInventorySlot(0, 2)) == 1)
        assert(C_Engraving.GetRuneForInventorySlot(0, 2) == nil)
        assert(select('#', C_Engraving.RefreshRunesList()) == 0)
        C_Engraving.RefreshRunesList()
    "#,
    )
    .unwrap();
    assert_eq!(env.state().borrow().engraving.refresh_requests, 2);
    let other = WowLuaEnv::new().unwrap();
    assert!(other.state().borrow().engraving.enabled.is_none());
    assert_eq!(other.state().borrow().engraving.refresh_requests, 0);
    env.state().borrow_mut().engraving = EngravingInputs::default();
    assert!(
        env.eval::<bool>("return pcall(C_Engraving.GetEngravingModeEnabled)")
            .is_ok_and(|ok| !ok)
    );
}

#[test]
fn original_settings_register_rune_checkbox_only_when_supplied_enabled() {
    for enabled in [false, true] {
        let ui = blizzard_ui_dir();
        let env = new_blizzard_addon_env(&ui);
        env.state().borrow_mut().engraving.enabled = Some(enabled);
        load_blizzard_addon_closure_into_env(
            &env,
            &ui,
            &["Blizzard_SettingsDefinitions_Frame"],
            &[],
        );
        env.exec(r#"
            local source = debug.getinfo(InterfaceOverrides.AdjustDisplaySettings, "S").source:gsub("\\", "/")
            assert(source:find("Blizzard_SettingsDefinitions_Frame/Classic/InterfaceOverrides.lua", 1, true), source)
            local category = Settings.RegisterVerticalLayoutCategory("Supplied engraving inputs")
            InterfaceOverrides.AdjustDisplaySettings(category)
        "#).unwrap();
        let registered: bool = env
            .eval("return Settings.GetSetting('alwaysShowRuneIcons') ~= nil")
            .unwrap();
        assert_eq!(
            registered, enabled,
            "original settings must follow supplied availability"
        );
    }
}
