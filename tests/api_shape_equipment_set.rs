//! Restored equipment-set coverage: supplied locks, not native lock policy.
#![cfg(any(
    feature = "client-retail",
    feature = "client-ptr",
    feature = "client-wowforever",
    feature = "client-mists"
))]

use crate::common::blizzard_addon_harness::{
    blizzard_ui_dir, load_blizzard_addon_closure_into_env, new_blizzard_addon_env,
};
use wow_ui_sim::client_profile::{ACTIVE, ClientProfile};
use wow_ui_sim::loader::find_toc_file;
use wow_ui_sim::lua_api::WowLuaEnv;
use wow_ui_sim::screen::ScreenKind;
use wow_ui_sim::toc::TocFile;

// Pinned EquipmentManagerDocumentation.lua:56-69 (Retail/PTR/Forever),
// :53-65 (Mists): required numeric equipmentSetID -> one non-nil bool.
// PTR re-authentication: declaration remains :56-69, TOC moved to :105.
fn load_equipment_consumer() -> WowLuaEnv {
    let ui = blizzard_ui_dir();
    let selected = assert_selected_equipment_toc(&ui);
    let env = new_blizzard_addon_env(&ui);
    let loaded = load_blizzard_addon_closure_into_env(&env, &ui, &["Blizzard_FrameXML"], &[]);
    assert!(loaded.iter().any(|name| name == "Blizzard_FrameXML"));
    assert_original_equipment_consumer(&env, selected);
    seed_saved_equipment_fixture(&env);
    env
}

fn assert_selected_equipment_toc(ui: &std::path::Path) -> &'static str {
    let toc_path = find_toc_file(&ui.join("Blizzard_FrameXML")).unwrap();
    let toc = TocFile::from_file(&toc_path).unwrap();
    assert!(toc.allows_screen(ScreenKind::Game));
    assert!(!toc.is_game_type_restricted());
    let selected = if ACTIVE == ClientProfile::Mists {
        "Wrath/EquipmentManager.lua"
    } else {
        "Mainline/EquipmentManager.lua"
    };
    assert!(toc.files.iter().any(|file| file == std::path::Path::new(selected)));
    eprintln!("Selected equipment TOC: {} -> {selected}", toc_path.display());
    selected
}

fn assert_original_equipment_consumer(env: &WowLuaEnv, selected: &str) {
    env.exec(&format!(
        r#"
        local info = debug.getinfo(EquipmentManager_EquipSet, 'S')
        local suffix = 'Blizzard_FrameXML/{selected}'
        local source = info.source:gsub('\\', '/')
        assert(info.what == 'Lua' and source:sub(-#suffix) == suffix,
            'must execute the original selected EquipmentManager_EquipSet')
        print('equipment consumer provenance', source, info.linedefined, info.lastlinedefined)
        assert(UnitCastingInfo('player') == nil, 'fixture requires a non-casting player')
        assert(UIErrorsFrame and UIErrorsFrame.AddMessage, 'original rejection message sink')
        "#,
    ))
    .unwrap();
}

fn seed_saved_equipment_fixture(env: &WowLuaEnv) {
    env.state().borrow_mut().player.equipped_items.clear();
    env.exec(
        r#"
        A_Admin.EquipItem(1, 211995)
        A_Admin.EquipItem(2, 211996)
        C_EquipmentSet.CreateEquipmentSet('Lock fixture', '12345')
        lockSetID = C_EquipmentSet.GetEquipmentSetID('Lock fixture')
        assert(type(lockSetID) == 'number', 'saved numeric set ID')
        A_Admin.EquipItem(1, 211993)
        equipmentSwaps = 0
        local observer = CreateFrame('Frame')
        observer:RegisterEvent('EQUIPMENT_SWAP_PENDING')
        observer:SetScript('OnEvent', function() equipmentSwaps = equipmentSwaps + 1 end)
        "#,
    )
    .unwrap();
}

// The preliminary missing-admin RED is input plumbing, not consumer proof.
#[test]
fn equipment_lock_admin_inputs_do_not_equip_items() {
    let env = WowLuaEnv::new().unwrap();
    env.exec("A_Admin.EquipItem(1, 211993); A_Admin.SetEquipmentItemLock(211995, true)")
        .unwrap();
    assert_eq!(
        env.state().borrow().equipment_manager.item_lock_observations.get(&211995),
        Some(&true)
    );
    env.exec("A_Admin.SetEquipmentItemLock(211995, false); A_Admin.SetEquipmentItemLock(211996, true)")
        .unwrap();
    assert_eq!(
        env.state().borrow().equipment_manager.item_lock_observations.get(&211995),
        Some(&false)
    );
    env.exec(
        r#"
        A_Admin.ClearEquipmentItemLock(211995)
        assert(GetInventoryItemID('player', 1) == 211993, 'observations do not equip gear')
        "#,
    )
    .unwrap();
    let sim = env.state().borrow();
    assert!(!sim.equipment_manager.item_lock_observations.contains_key(&211995));
    assert_eq!(sim.equipment_manager.item_lock_observations.get(&211996), Some(&true));
    assert_eq!(sim.equipment_manager.last_used_set_id, None);
}

#[test]
fn equipment_lock_admin_requires_explicit_boolean() {
    let env = WowLuaEnv::new().unwrap();
    env.exec(
        r#"
        A_Admin.SetEquipmentItemLock(211995, true)
        local attempts = {
            function() A_Admin.SetEquipmentItemLock(211995) end,
            function() A_Admin.SetEquipmentItemLock(211995, nil) end,
            function() A_Admin.SetEquipmentItemLock(211995, 0) end,
            function() A_Admin.SetEquipmentItemLock(211995, 'false') end,
            function() A_Admin.SetEquipmentItemLock(211995, {}) end,
        }
        for _, attempt in ipairs(attempts) do
            local ok, message = pcall(attempt)
            assert(not ok, 'admin must not turn unsupplied or non-boolean data into a lock observation')
            assert(message:find('explicit boolean lock observation', 1, true), message)
        end
        "#,
    )
    .unwrap();
    assert_eq!(
        env.state().borrow().equipment_manager.item_lock_observations.get(&211995),
        Some(&true)
    );
}

#[test]
fn equipment_set_original_equip_set_contract() {
    let env = load_equipment_consumer();
    env.exec(
        r#"
        A_Admin.SetEquipmentItemLock(211995, true)
        A_Admin.SetEquipmentItemLock(211996, false)
        EquipmentManager_EquipSet(lockSetID)
        assert(GetInventoryItemID('player', 1) == 211993,
            'LOCKED: original consumer must not equip the saved item')
        assert(equipmentSwaps == 0, 'LOCKED: no swap may start')
        "#,
    )
    .expect("original consumer must reject a supplied locked saved item");
    assert_eq!(env.state().borrow().equipment_manager.last_used_set_id, None);
    env.exec(
        r#"
        A_Admin.SetEquipmentItemLock(211995, false)
        EquipmentManager_EquipSet(lockSetID)
        assert(GetInventoryItemID('player', 1) == 211995, 'UNLOCKED: saved item equipped')
        assert(GetInventoryItemID('player', 2) == 211996, 'UNLOCKED: second saved item equipped')
        assert(equipmentSwaps == 1, 'UNLOCKED: exactly one swap')
        assert(select('#', C_EquipmentSet.EquipmentSetContainsLockedItems(lockSetID)) == 1)
        assert(C_EquipmentSet.EquipmentSetContainsLockedItems(lockSetID) == false)
        A_Admin.EquipItem(1, 211993)
        A_Admin.SetEquipmentItemLock(211996, true)
        EquipmentManager_EquipSet(lockSetID)
        assert(GetInventoryItemID('player', 1) == 211993, 'RELOCKED: second saved item blocks swap')
        assert(equipmentSwaps == 1, 'RELOCKED: no additional swap')
        assert(C_EquipmentSet.EquipmentSetContainsLockedItems(lockSetID) == true)
        "#,
    )
    .expect("original consumer must follow unlocked and relocked observations");
    let set_id: i32 = env.eval("return lockSetID").unwrap();
    assert_eq!(env.state().borrow().equipment_manager.last_used_set_id, Some(set_id));
}

#[test]
fn equipment_set_locks_follow_live_saved_contents() {
    let env = load_equipment_consumer();
    env.exec(
        r#"
        A_Admin.SetEquipmentItemLock(211995, false)
        A_Admin.SetEquipmentItemLock(211996, false)
        A_Admin.SetEquipmentItemLock(211997, true)
        EquipmentManager_EquipSet(lockSetID)
        assert(GetInventoryItemID('player', 1) == 211995,
            'UNRELATED: a locked item outside the saved set must not block equip')
        assert(equipmentSwaps == 1)
        A_Admin.EquipItem(1, 211997)
        C_EquipmentSet.SaveEquipmentSet(lockSetID)
        A_Admin.EquipItem(1, 211993)
        EquipmentManager_EquipSet(lockSetID)
        assert(GetInventoryItemID('player', 1) == 211993,
            'RESAVED: newly saved locked item must block equip')
        assert(equipmentSwaps == 1)
        A_Admin.SetEquipmentItemLock(211997, false)
        EquipmentManager_EquipSet(lockSetID)
        assert(GetInventoryItemID('player', 1) == 211997, 'RESAVED: unlocked new saved contents equip')
        assert(equipmentSwaps == 2)
        "#,
    )
    .expect("original consumer joins observations to current saved-set items, not all inventory");
}

#[test]
fn equipment_set_unsupplied_locks_and_unknown_sets_are_explicit_errors() {
    let env = load_equipment_consumer();
    env.exec(
        r#"
        local function expectUnknown(id, reason)
            local ok, message = pcall(EquipmentManager_EquipSet, id)
            assert(not ok, 'missing native evidence must not become an unlocked default')
            assert(message:find('native-unknown: C_EquipmentSet.EquipmentSetContainsLockedItems', 1, true), message)
            assert(message:find(reason, 1, true), message)
            assert(GetInventoryItemID('player', 1) == 211993, 'unknown query must not equip')
            assert(equipmentSwaps == 0, 'unknown query must not start a swap')
        end
        expectUnknown(lockSetID, 'no supplied lock observation')
        A_Admin.SetEquipmentItemLock(211995, false)
        expectUnknown(lockSetID, 'no supplied lock observation')
        A_Admin.SetEquipmentItemLock(211996, false)
        A_Admin.ClearEquipmentItemLock(211995)
        expectUnknown(lockSetID, 'no supplied lock observation')
        expectUnknown(987654, 'no saved set')
        C_EquipmentSet.DeleteEquipmentSet(lockSetID)
        expectUnknown(lockSetID, 'no saved set')
        "#,
    )
    .expect("original consumer propagates explicit simulator native-unknown policy errors");
    let other = load_equipment_consumer();
    env.exec("A_Admin.SetEquipmentItemLock(211995, false); A_Admin.SetEquipmentItemLock(211996, false)")
        .unwrap();
    other.exec(
        r#"
        local ok, message = pcall(EquipmentManager_EquipSet, lockSetID)
        assert(not ok and message:find('no supplied lock observation', 1, true),
            'observations in another environment must not leak')
        assert(GetInventoryItemID('player', 1) == 211993)
        assert(equipmentSwaps == 0)
        "#,
    )
    .unwrap();
}
