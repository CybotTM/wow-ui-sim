//! Slot-observation publication and preservation outside the modern producer.
use wow_ui_sim::client_profile::{ACTIVE, ClientProfile};
use wow_ui_sim::lua_api::WowLuaEnv;

#[test]
fn stable_slot_members_follow_profile_declarations() {
    let env = WowLuaEnv::new().unwrap();
    let modern = matches!(ACTIVE, ClientProfile::Retail | ClientProfile::Ptr);
    let query_present = modern || ACTIVE == ClientProfile::WowForever;
    env.exec(&format!(
        r#"
        assert((rawget(C_StableInfo,'GetStablePetInfo') ~= nil) == {query_present})
        assert((rawget(A_Admin,'SetStablePetInfo') ~= nil) == {modern})
        assert((rawget(A_Admin,'ClearStablePetInfo') ~= nil) == {modern})
        "#
    ))
    .unwrap();
    if modern {
        env.exec(
            r#"
            local ok, reason = pcall(C_StableInfo.GetStablePetInfo, 9)
            assert(not ok and tostring(reason):find('native-unknown:',1,true),tostring(reason))
            A_Admin.SetActivePetList({})
            ok, reason = pcall(C_StableInfo.GetStablePetInfo, 9)
            assert(not ok and tostring(reason):find('slot 9',1,true),tostring(reason))
            A_Admin.SetStablePetInfo(9,nil)
            assert(select('#',C_StableInfo.GetStablePetInfo(9)) == 1)
            assert(C_StableInfo.GetStablePetInfo(9) == nil)
            A_Admin.ClearStablePetInfo(9)
            assert(not pcall(C_StableInfo.GetStablePetInfo, 9))
            "#,
        )
        .unwrap();
    } else if ACTIVE == ClientProfile::WowForever {
        env.exec(
            r#"
            assert(C_StableInfo.GetStablePetInfo(1) == nil)
            assert(select('#',C_StableInfo.GetStablePetInfo(1)) == 1)
            "#,
        )
        .unwrap();
    }
}
