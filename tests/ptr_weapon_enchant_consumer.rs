#![cfg(feature = "client-ptr")]

use wow_ui_sim::c_api::weapon_enchants::{TEMPORARY_ENCHANT_TYPE, WeaponEnchant};
use wow_ui_sim::lua_api::WowLuaEnv;

prefork_full_ui_case! {
fn ptr_weapon_enchant_event_updates_buff_frame(env: &WowLuaEnv) {
    env.exec("assert(BuffFrame:IsEventRegistered('WEAPON_ENCHANT_CHANGED'))")
        .unwrap();
    for (charges, time_left) in [(2, 60000.0), (5, 30000.0)] {
        env.state().borrow_mut().weapon_enchants[0] = vec![WeaponEnchant {
            enchant_type: TEMPORARY_ENCHANT_TYPE,
            time_left,
            charges,
            enchant_id: 42,
            icon_id: 135913,
        }];
        env.state().borrow_mut().lua_errors.clear();
        env.fire_event("WEAPON_ENCHANT_CHANGED").unwrap();
        let errors = env.state().borrow().lua_errors.clone();
        assert!(errors.is_empty(), "weapon enchant event errors: {errors:?}");
        let actual = env.eval::<u32>("for _, aura in ipairs(BuffFrame.auraInfo) do if aura.auraType == 'TempEnchant' and aura.ID == 16 then return aura.count end end; return 0").unwrap();
        assert_eq!(actual, charges);
        let legacy = env.eval::<u32>("local _, _, charges = GetWeaponEnchantInfo(); return charges").unwrap();
        assert_eq!(legacy, charges);
    }
}
}
