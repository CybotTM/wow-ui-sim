//! Declared-but-unconsumed Forever and undeclared classic profiles remain unchanged.
#![cfg(not(any(feature = "client-retail", feature = "client-ptr")))]

use wow_ui_sim::lua_api::WowLuaEnv;

#[test]
fn pet_spec_member_and_admin_input_remain_absent() {
    let env = WowLuaEnv::new().unwrap();
    env.exec(
        r#"
        assert(rawget(C_StableInfo, 'GetAvailablePetSpecInfos') == nil)
        assert(rawget(A_Admin, 'SetAvailablePetSpecInfos') == nil)
        "#,
    )
    .unwrap();
}
