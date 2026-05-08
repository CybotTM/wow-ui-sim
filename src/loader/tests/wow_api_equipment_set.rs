//! C_EquipmentSet coverage extracted from `wow_api.rs`.

use super::*;

#[test]
fn test_c_equipment_set_api_surface() {
    let env = WowLuaEnv::new().unwrap();
    let ids_ty: String = env
        .eval("return type(C_EquipmentSet.GetEquipmentSetIDs())")
        .unwrap();
    assert_eq!(ids_ty, "table");

    for fn_name in [
        "CreateEquipmentSet",
        "ModifyEquipmentSet",
        "DeleteEquipmentSet",
        "SaveEquipmentSet",
        "UseEquipmentSet",
        "GetEquipmentSetID",
        "GetEquipmentSetInfo",
        "GetEquipmentSetAssignedSpec",
        "GetEquipmentSetForSpec",
        "AssignSpecToEquipmentSet",
        "UnassignEquipmentSetSpec",
        "GetIgnoredSlots",
        "GetItemIDs",
        "GetItemLocations",
        "IgnoreSlotForSave",
        "UnignoreSlotForSave",
        "IsSlotIgnoredForSave",
        "ClearIgnoredSlotsForSave",
        "EquipmentSetContainsLockedItems",
        "CanUseEquipmentSets",
        "PickupEquipmentSet",
    ] {
        let expr = format!("return type(C_EquipmentSet.{fn_name})");
        let ty: String = env.eval(&expr).unwrap();
        assert_eq!(ty, "function", "C_EquipmentSet.{fn_name} should exist");
    }
}

#[test]
fn test_c_equipment_set_create_appears_in_list() {
    let env = WowLuaEnv::new().unwrap();
    let n: i32 = env
        .eval("return C_EquipmentSet.GetNumEquipmentSets()")
        .unwrap();
    assert_eq!(n, 0, "starts empty");

    let id: i32 = env
        .eval(
            r#"
            C_EquipmentSet.CreateEquipmentSet("Tank", "Interface\\Icons\\INV_Misc_QuestionMark")
            return C_EquipmentSet.GetEquipmentSetID("Tank")
            "#,
        )
        .unwrap();
    assert!(id > 0, "new set must get a positive id");

    let count: i32 = env
        .eval("return C_EquipmentSet.GetNumEquipmentSets()")
        .unwrap();
    assert_eq!(count, 1);

    let len: i32 = env
        .eval("return #C_EquipmentSet.GetEquipmentSetIDs()")
        .unwrap();
    assert_eq!(len, 1);

    let name: String = env
        .eval(
            "return (C_EquipmentSet.GetEquipmentSetInfo(C_EquipmentSet.GetEquipmentSetID('Tank')))",
        )
        .unwrap();
    assert_eq!(name, "Tank");
}

#[test]
fn test_c_equipment_set_modify_and_delete() {
    let env = WowLuaEnv::new().unwrap();
    let _: i32 = env
        .eval(
            r#"
        C_EquipmentSet.CreateEquipmentSet("Heal", "Interface\\Icons\\Spell_Holy_Heal")
        local id = C_EquipmentSet.GetEquipmentSetID("Heal")
        C_EquipmentSet.ModifyEquipmentSet(id, "Holy", "Interface\\Icons\\Spell_Holy_HolyBolt")
        return 1
        "#,
        )
        .unwrap();
    let renamed: bool = env
        .eval("return C_EquipmentSet.GetEquipmentSetID('Heal') == nil and C_EquipmentSet.GetEquipmentSetID('Holy') ~= nil")
        .unwrap();
    assert!(renamed, "rename must replace lookup");

    let _: i32 = env
        .eval(
            r#"
        local id = C_EquipmentSet.GetEquipmentSetID("Holy")
        C_EquipmentSet.DeleteEquipmentSet(id)
        return 1
        "#,
        )
        .unwrap();
    let count: i32 = env
        .eval("return C_EquipmentSet.GetNumEquipmentSets()")
        .unwrap();
    assert_eq!(count, 0, "delete must remove from list");
}

#[test]
fn test_c_equipment_set_spec_assignment() {
    let env = WowLuaEnv::new().unwrap();
    let assigned: i32 = env
        .eval(
            r#"
            C_EquipmentSet.CreateEquipmentSet("DPS", nil)
            local id = C_EquipmentSet.GetEquipmentSetID("DPS")
            C_EquipmentSet.AssignSpecToEquipmentSet(id, 2)
            return C_EquipmentSet.GetEquipmentSetAssignedSpec(id) or -1
            "#,
        )
        .unwrap();
    assert_eq!(assigned, 2);

    let by_spec: i32 = env
        .eval("return C_EquipmentSet.GetEquipmentSetForSpec(2) or -1")
        .unwrap();
    assert!(by_spec > 0, "GetEquipmentSetForSpec must round-trip");

    let cleared: i32 = env
        .eval(
            r#"
            local id = C_EquipmentSet.GetEquipmentSetID("DPS")
            C_EquipmentSet.UnassignEquipmentSetSpec(id)
            return C_EquipmentSet.GetEquipmentSetAssignedSpec(id) or -1
            "#,
        )
        .unwrap();
    assert_eq!(cleared, -1, "unassign must drop the spec link");
}

#[test]
fn test_c_equipment_set_ignored_slots() {
    let env = WowLuaEnv::new().unwrap();
    let ignored: bool = env
        .eval(
            r#"
            C_EquipmentSet.ClearIgnoredSlotsForSave()
            C_EquipmentSet.IgnoreSlotForSave(5)
            return C_EquipmentSet.IsSlotIgnoredForSave(5)
            "#,
        )
        .unwrap();
    assert!(ignored);

    let after_unignore: bool = env
        .eval(
            r#"
            C_EquipmentSet.UnignoreSlotForSave(5)
            return C_EquipmentSet.IsSlotIgnoredForSave(5)
            "#,
        )
        .unwrap();
    assert!(!after_unignore);

    let mask_present: bool = env
        .eval(
            r#"
            C_EquipmentSet.CreateEquipmentSet("Mixed", nil)
            local id = C_EquipmentSet.GetEquipmentSetID("Mixed")
            C_EquipmentSet.IgnoreSlotForSave(7)
            C_EquipmentSet.SaveEquipmentSet(id)
            local t = C_EquipmentSet.GetIgnoredSlots(id)
            return t and t[7] == true
            "#,
        )
        .unwrap();
    assert!(
        mask_present,
        "SaveEquipmentSet must persist the pending ignore mask"
    );
}

#[test]
fn test_c_equipment_set_use_marks_equipped() {
    let env = WowLuaEnv::new().unwrap();
    let was_equipped: bool = env
        .eval(
            r#"
            C_EquipmentSet.CreateEquipmentSet("Active", nil)
            local id = C_EquipmentSet.GetEquipmentSetID("Active")
            C_EquipmentSet.UseEquipmentSet(id)
            local _, _, _, isEquipped = C_EquipmentSet.GetEquipmentSetInfo(id)
            return isEquipped
            "#,
        )
        .unwrap();
    assert!(was_equipped);
}

#[test]
fn test_c_equipment_set_fires_event_on_create() {
    let env = WowLuaEnv::new().unwrap();
    let fired: i32 = env
        .eval(
            r#"
            local f = CreateFrame("Frame")
            f.count = 0
            f:RegisterEvent("EQUIPMENT_SETS_CHANGED")
            f:SetScript("OnEvent", function(self) self.count = self.count + 1 end)
            C_EquipmentSet.CreateEquipmentSet("EventTest", nil)
            return f.count
            "#,
        )
        .unwrap();
    assert!(fired >= 1, "EQUIPMENT_SETS_CHANGED must fire on create");
}
