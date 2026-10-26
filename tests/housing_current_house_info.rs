//! Documented shape only; fixture values do not establish native housing policy.
#![cfg(any(feature = "client-retail", feature = "client-ptr"))]

use wow_ui_sim::c_api::c_housing::current_house::HouseInfo;
use wow_ui_sim::client_profile::{ACTIVE, ClientProfile};
use wow_ui_sim::lua_api::WowLuaEnv;

#[test]
fn supplied_snapshot_preserves_documented_nullable_shape() {
    let env = WowLuaEnv::new().unwrap();
    env.exec(
        "assert(select('#', C_Housing.GetCurrentHouseInfo()) == 1); \
         assert(C_Housing.GetCurrentHouseInfo() == nil)",
    )
    .expect("an unsupplied nullable snapshot is one nil, not zero returns");
    env.state().borrow_mut().housing.current_house_info = Some(HouseInfo {
        plot_id: 27.0,
        house_name: Some("Fixture House".into()),
        owner_name: Some("Fixture Owner".into()),
        plot_cost: Some(12345.0),
        neighborhood_name: Some("Fixture Neighborhood".into()),
        move_out_time: Some(1800000000),
        plot_reserved: Some(false),
        neighborhood_guid: Some("fixture-neighborhood".into()),
        house_guid: Some("fixture-house".into()),
    });
    env.exec(
        r#"
        assert(select('#', C_Housing.GetCurrentHouseInfo()) == 1)
        local info = C_Housing.GetCurrentHouseInfo()
        assert(type(info) == 'table' and info.plotID == 27)
        assert(info.houseName == 'Fixture House' and info.ownerName == 'Fixture Owner')
        assert(info.plotCost == 12345 and info.neighborhoodName == 'Fixture Neighborhood')
        assert(info.moveOutTime == 1800000000 and info.plotReserved == false)
        assert(info.neighborhoodGUID == 'fixture-neighborhood' and info.houseGUID == 'fixture-house')
        "#,
    )
    .expect("all supplied documented fields project without invented values");
    env.state().borrow_mut().housing.current_house_info = Some(HouseInfo {
        plot_id: 41.0,
        house_name: None,
        owner_name: None,
        plot_cost: None,
        neighborhood_name: None,
        move_out_time: None,
        plot_reserved: Some(true),
        neighborhood_guid: None,
        house_guid: None,
    });
    env.exec(
        r#"
        local info = C_Housing.GetCurrentHouseInfo()
        assert(info.plotID == 41 and info.plotReserved == true)
        for _, key in ipairs({'houseName', 'ownerName', 'plotCost', 'neighborhoodName',
            'moveOutTime', 'neighborhoodGUID', 'houseGUID'}) do
            assert(info[key] == nil, key)
        end
        "#,
    )
    .expect("nullable fields stay absent rather than receiving placeholder defaults");
    env.state().borrow_mut().housing.current_house_info = None;
    env.exec("assert(C_Housing.GetCurrentHouseInfo() == nil)")
        .expect("clearing the supplied snapshot clears the query");
}

#[test]
fn shard_transfer_registration_matches_active_profile() {
    let env = WowLuaEnv::new().unwrap();
    let observed: (bool, bool, bool) = env
        .eval(
            r#"
            local frame = CreateFrame('Frame')
            local ok = pcall(frame.RegisterEvent, frame, 'SHARD_TRANSFER')
            return ok, frame:IsEventRegistered('SHARD_TRANSFER'), C_EventUtils.IsEventValid('SHARD_TRANSFER')
            "#,
        )
        .unwrap();
    let declared = ACTIVE == ClientProfile::Ptr;
    assert_eq!(observed, (declared, declared, declared));
}
