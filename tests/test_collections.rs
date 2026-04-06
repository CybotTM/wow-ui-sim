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
fn pet_journal_num_pets() {
    let env = env();
    let count: i32 = env.eval("return C_PetJournal.GetNumPets()").unwrap();
    assert_eq!(count, 10, "Should have 10 default pets");
}

#[test]
fn pet_journal_num_collected_info() {
    let env = env();
    let result: String = env
        .eval(
            r#"
            local collected, total = C_PetJournal.GetNumCollectedInfo(0)
            return collected .. "," .. total
            "#,
        )
        .unwrap();
    assert_eq!(result, "9,10", "9 collected out of 10 total");
}

#[test]
fn pet_journal_get_pet_info_by_index() {
    let env = env();
    let result: String = env
        .eval(
            r#"
            local petID, speciesID, owned, _, level, _, _, name, icon, petType
                = C_PetJournal.GetPetInfoByIndex(1)
            if name ~= "Mechanical Squirrel" then return "name=" .. tostring(name) end
            if speciesID ~= 39 then return "species=" .. tostring(speciesID) end
            if owned ~= true then return "owned=" .. tostring(owned) end
            if level ~= 25 then return "level=" .. tostring(level) end
            return "ok"
            "#,
        )
        .unwrap();
    assert_eq!(result, "ok", "GetPetInfoByIndex: {result}");
}

#[test]
fn pet_journal_get_pet_info_by_species_id() {
    let env = env();
    let result: String = env
        .eval(
            r#"
            local petID, speciesID, owned, _, _, _, _, name
                = C_PetJournal.GetPetInfoBySpeciesID(254)
            if name ~= "Lil' Ragnaros" then return "name=" .. tostring(name) end
            return "ok"
            "#,
        )
        .unwrap();
    assert_eq!(result, "ok", "GetPetInfoBySpeciesID: {result}");
}

#[test]
fn pet_journal_get_pet_info_invalid() {
    let env = env();
    let result: String = env
        .eval(
            r#"
            local r = C_PetJournal.GetPetInfoByIndex(99)
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

#[test]
fn toy_box_num_total_displayed() {
    let env = env();
    let count: i32 = env.eval("return C_ToyBox.GetNumTotalDisplayedToys()").unwrap();
    assert_eq!(count, 10);
}

#[test]
fn toy_box_num_learned_displayed() {
    let env = env();
    let count: i32 = env.eval("return C_ToyBox.GetNumLearnedDisplayedToys()").unwrap();
    assert_eq!(count, 9, "9 collected out of 10");
}

#[test]
fn toy_box_get_toy_from_index() {
    let env = env();
    let id: i32 = env.eval("return C_ToyBox.GetToyFromIndex(1)").unwrap();
    assert_eq!(id, 166779, "First toy should be Hearthstone Game Table");
}

#[test]
fn toy_box_get_toy_info() {
    let env = env();
    let result: String = env
        .eval(
            r#"
            local itemID, name, icon = C_ToyBox.GetToyInfo(13379)
            if name ~= "Piccolo of the Flaming Fire" then return "name=" .. tostring(name) end
            if itemID ~= 13379 then return "id=" .. tostring(itemID) end
            return "ok"
            "#,
        )
        .unwrap();
    assert_eq!(result, "ok", "GetToyInfo: {result}");
}
