//! Retail original-consumer proof; no vehicle token substitution or vitals claim.
#![cfg(feature = "client-retail")]

use sha2::{Digest, Sha256};
use wow_ui_sim::lua_api::WowLuaEnv;

fn load_original_compact_unit_frame() -> WowLuaEnv {
    let ui = crate::common::blizzard_addon_harness::blizzard_ui_dir();
    let (env, _) = crate::common::blizzard_addon_harness::build_blizzard_addon_closure_env(
        &ui,
        &["Blizzard_FrameXMLUtil"],
        &[],
    );
    let path = wow_ui_sim::client_profile::blizzard_ui_addons_dir()
        .join("Blizzard_UnitFrame/Shared/CompactUnitFrame.lua");
    let source = std::fs::read_to_string(&path).expect("read original cached CompactUnitFrame");
    assert_eq!(
        format!("{:x}", Sha256::digest(source.as_bytes())),
        "1f7064c5d54466c26c53fded64684612d76891cb10d514fe3a806839079848fb",
        "cached original consumer changed: {}",
        path.display()
    );
    env.exec(&source)
        .expect("load complete unmodified CompactUnitFrame chunk");
    env
}

#[test]
fn compact_unit_frame_vehicle_visibility_tracks_enter_and_exit() {
    let env = load_original_compact_unit_frame();
    env.exec(
        r#"
        VehicleExistenceFrame = CreateFrame('Frame')
        VehicleExistenceFrame.unit = 'vehicle'
        VehicleExistenceFrame.displayedUnit = 'vehicle'
        PlayerControlFrame = CreateFrame('Frame')
        PlayerControlFrame.unit = 'player'
        PlayerControlFrame.displayedUnit = 'player'
        "#,
    )
    .unwrap();
    let stored_pet_icon = env.state().borrow().pet.icon.clone();
    assert!(!env.state().borrow().player.in_vehicle);

    for (occupied, phase) in [(false, "absent"), (true, "entered"), (false, "exited")] {
        env.state().borrow_mut().player.in_vehicle = occupied;
        env.exec("CompactUnitFrame_UpdateVisible(VehicleExistenceFrame)")
            .unwrap();
        let (shown, exists): (bool, bool) = env
            .eval("return VehicleExistenceFrame:IsShown(), VehicleExistenceFrame.unitExists")
            .unwrap();
        assert_eq!(shown, occupied, "Blizzard vehicle visibility: {phase}");
        assert_eq!(exists, occupied, "Blizzard vehicle unitExists: {phase}");
        env.exec(
            r#"
            assert(VehicleExistenceFrame.unit == 'vehicle')
            assert(VehicleExistenceFrame.displayedUnit == 'vehicle')
            CompactUnitFrame_UpdateVisible(PlayerControlFrame)
            assert(PlayerControlFrame:IsShown(), 'player control remains visible')
            assert(PlayerControlFrame.unitExists == true)
            assert(PlayerControlFrame.unit == 'player')
            assert(PlayerControlFrame.displayedUnit == 'player')
            "#,
        )
        .unwrap();
        assert_eq!(env.state().borrow().pet.icon, stored_pet_icon);
    }
}
