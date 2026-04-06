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

#[test]
fn mount_journal_get_displayed_mount_info() {
    let env = env();
    let result: String = env
        .eval(
            r#"
            local name, spellID, icon, isActive, isUsable, sourceType,
                  isFavorite, isFactionSpecific, faction, shouldHideOnChar,
                  isCollected, mountID = C_MountJournal.GetDisplayedMountInfo(2)
            if name ~= "Swift Palomino" then return "name=" .. tostring(name) end
            if mountID ~= 18 then return "mountID=" .. tostring(mountID) end
            if isCollected ~= true then return "collected=" .. tostring(isCollected) end
            if isUsable ~= true then return "usable=" .. tostring(isUsable) end
            return "ok"
            "#,
        )
        .unwrap();
    assert_eq!(result, "ok", "GetDisplayedMountInfo: {result}");
}

#[test]
fn mount_journal_displayed_info_uncollected() {
    let env = env();
    let result: String = env
        .eval(
            r#"
            -- Mount 10 is Mighty Caravan Brutosaur (not collected)
            local name, _, _, _, _, _, _, _, _, _, isCollected = C_MountJournal.GetDisplayedMountInfo(10)
            if name ~= "Mighty Caravan Brutosaur" then return "name=" .. tostring(name) end
            if isCollected ~= false then return "collected=" .. tostring(isCollected) end
            return "ok"
            "#,
        )
        .unwrap();
    assert_eq!(result, "ok", "Uncollected mount: {result}");
}

#[test]
fn mount_journal_displayed_info_invalid_index() {
    let env = env();
    let result: String = env
        .eval(
            r#"
            local r = C_MountJournal.GetDisplayedMountInfo(99)
            return r == nil and "nil" or "not_nil"
            "#,
        )
        .unwrap();
    assert_eq!(result, "nil", "Invalid index should return nil");
}

#[test]
fn mount_journal_get_mount_info_by_id() {
    let env = env();
    let result: String = env
        .eval(
            r#"
            local name, spellID, icon, _, isUsable, _, _, _, _, _, isCollected, mountID
                = C_MountJournal.GetMountInfoByID(107)
            if name ~= "Ashes of Al'ar" then return "name=" .. tostring(name) end
            if mountID ~= 107 then return "id=" .. tostring(mountID) end
            if isCollected ~= true then return "collected=" .. tostring(isCollected) end
            return "ok"
            "#,
        )
        .unwrap();
    assert_eq!(result, "ok", "GetMountInfoByID: {result}");
}

#[test]
fn mount_journal_get_mount_info_by_id_invalid() {
    let env = env();
    let result: String = env
        .eval(
            r#"
            local r = C_MountJournal.GetMountInfoByID(99999)
            return r == nil and "nil" or "not_nil"
            "#,
        )
        .unwrap();
    assert_eq!(result, "nil");
}

#[test]
fn mount_journal_get_mount_info_extra_by_id() {
    let env = env();
    let result: String = env
        .eval(
            r#"
            local _, _, _, _, mountTypeID = C_MountJournal.GetMountInfoExtraByID(107)
            -- Ashes of Al'ar is flying (mount_type 248)
            if mountTypeID ~= 248 then return "type=" .. tostring(mountTypeID) end
            return "ok"
            "#,
        )
        .unwrap();
    assert_eq!(result, "ok", "GetMountInfoExtraByID: {result}");
}
