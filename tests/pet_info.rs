use wow_ui_sim::lua_api::WowLuaEnv;

fn env() -> WowLuaEnv {
    WowLuaEnv::new().expect("Failed to create Lua environment")
}

#[test]
fn pet_tamers_for_map_are_state_backed_and_return_copies() {
    let env = env();
    let (
        has_two_tamers,
        first_name_ok,
        first_position_ok,
        second_call_isolated,
        unknown_map_empty,
    ): (bool, bool, bool, bool, bool) = env
        .eval(
            r#"
            C_PetInfo._state.petTamersByMapID = {
                [2025] = {
                    {
                        areaPoiID = 9001,
                        position = { x = 0.25, y = 0.75 },
                        name = "Aki the Chosen",
                        atlasName = "poi-battlepet",
                        textureIndex = 6,
                    },
                    {
                        areaPoiID = 9002,
                        position = { x = 0.52, y = 0.24 },
                        name = "Li the Master",
                    },
                },
            }

            local firstRead = C_PetInfo.GetPetTamersForMap(2025)
            local first = firstRead[1]
            local hasTwoTamers = #firstRead == 2
            local firstNameOk = first and first.name == "Aki the Chosen"
            local firstPositionOk = first and first.position and first.position.x == 0.25 and first.position.y == 0.75

            firstRead[1].name = "Mutated"
            firstRead[1].position.x = 0

            local secondRead = C_PetInfo.GetPetTamersForMap("2025")
            local second = secondRead[1]
            local secondCallIsolated = second and second.name == "Aki the Chosen" and second.position and second.position.x == 0.25
            local unknownMapEmpty = #C_PetInfo.GetPetTamersForMap(9999) == 0

            return hasTwoTamers, firstNameOk, firstPositionOk, secondCallIsolated, unknownMapEmpty
            "#,
        )
        .unwrap();

    assert!(has_two_tamers, "expected two map tamers for configured map");
    assert!(first_name_ok, "expected first tamer name from state");
    assert!(
        first_position_ok,
        "expected first tamer position to match configured state"
    );
    assert!(
        second_call_isolated,
        "GetPetTamersForMap should return fresh copies instead of shared tables"
    );
    assert!(
        unknown_map_empty,
        "unknown maps should return an empty list"
    );
}

#[test]
fn pet_action_spell_lookup_reads_state_mappings() {
    let env = env();
    let (
        direct_mapping_ok,
        string_mapping_ok,
        table_mapping_ok,
        unknown_returns_nil,
        invalid_returns_nil,
    ): (bool, bool, bool, bool, bool) = env
        .eval(
            r#"
            C_PetInfo._state.spellByPetActionID = {
                [1] = 17253,
                [2] = "49966",
            }
            C_PetInfo._state.petActionsByID = {
                [3] = {
                    spellID = 2649,
                    isPassive = true,
                },
            }

            return C_PetInfo.GetSpellForPetAction(1) == 17253,
                   C_PetInfo.GetSpellForPetAction("2") == 49966,
                   C_PetInfo.GetSpellForPetAction(3) == 2649,
                   C_PetInfo.GetSpellForPetAction(4) == nil,
                   C_PetInfo.GetSpellForPetAction("invalid") == nil
            "#,
        )
        .unwrap();

    assert!(
        direct_mapping_ok,
        "direct pet-action spell mapping should be used"
    );
    assert!(
        string_mapping_ok,
        "numeric strings should be accepted for action and spell IDs"
    );
    assert!(
        table_mapping_ok,
        "spellID should fall back to petActionsByID table when needed"
    );
    assert!(
        unknown_returns_nil,
        "unknown action IDs should return nil spell ID"
    );
    assert!(
        invalid_returns_nil,
        "invalid action IDs should return nil spell ID"
    );
}

#[test]
fn pet_action_passive_lookup_is_state_backed() {
    let env = env();
    let (from_set, from_action_info, explicit_false, unknown_false, invalid_false): (
        bool,
        bool,
        bool,
        bool,
        bool,
    ) = env
        .eval(
            r#"
            C_PetInfo._state.passivePetActionIDs = {
                [7] = true,
            }
            C_PetInfo._state.petActionsByID = {
                [8] = {
                    spellID = 1234,
                    isPassive = true,
                },
                [9] = {
                    spellID = 5678,
                    isPassive = false,
                },
            }

            return C_PetInfo.IsPetActionPassive(7),
                   C_PetInfo.IsPetActionPassive(8),
                   not C_PetInfo.IsPetActionPassive(9),
                   not C_PetInfo.IsPetActionPassive(10),
                   not C_PetInfo.IsPetActionPassive("bad-input")
            "#,
        )
        .unwrap();

    assert!(from_set, "passive set should mark pet action as passive");
    assert!(
        from_action_info,
        "petActionsByID metadata should mark pet action as passive"
    );
    assert!(
        explicit_false,
        "explicit non-passive action should return false"
    );
    assert!(unknown_false, "unknown actions should default to false");
    assert!(invalid_false, "invalid action IDs should return false");
}

#[test]
fn pet_journal_ability_list_table_has_level_entries_and_info() {
    let env = env();
    let (has_three_slots, first_slot_ok, second_slot_level_ok, ability_info_ok): (
        bool,
        bool,
        bool,
        bool,
    ) = env
        .eval(
            r#"
            local petID, speciesID = C_PetJournal.GetPetInfoByIndex(1)
            local abilities = C_PetJournal.GetPetAbilityListTable(speciesID)
            local firstAbilityID = abilities[1] and abilities[1].abilityID
            local name, icon, petType = C_PetJournal.GetPetAbilityInfo(firstAbilityID)

            return #abilities == 3,
                   type(firstAbilityID) == "number" and abilities[1].level == 1,
                   abilities[2] and abilities[2].level == 2,
                   type(name) == "string" and type(icon) == "number" and type(petType) == "number"
            "#,
        )
        .unwrap();

    assert!(
        has_three_slots,
        "pet ability list should expose three slots"
    );
    assert!(
        first_slot_ok,
        "first pet ability slot should have an ID and level"
    );
    assert!(
        second_slot_level_ok,
        "second pet ability slot should preserve its unlock level"
    );
    assert!(
        ability_info_ok,
        "journal ability info should resolve generated ability IDs"
    );
}
