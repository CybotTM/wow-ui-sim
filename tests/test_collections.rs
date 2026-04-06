use wow_ui_sim::lua_api::WowLuaEnv;

fn env() -> WowLuaEnv {
    WowLuaEnv::new().unwrap()
}

#[test]
fn mount_journal_num_mounts() {
    let env = env();
    let count: i32 = env.eval("return C_MountJournal.GetNumMounts()").unwrap();
    assert_eq!(count, 10, "Should have 10 default mounts");
}

#[test]
fn mount_journal_num_displayed_mounts() {
    let env = env();
    let count: i32 = env
        .eval("return C_MountJournal.GetNumDisplayedMounts()")
        .unwrap();
    assert_eq!(count, 10, "Displayed mounts should equal total mounts");
}
