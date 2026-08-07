//! Focused 12.1 service-payload compatibility contracts.

use super::super::*;

#[cfg(feature = "retail-12-1-0")]
#[test]
fn test_patch_12_1_pet_and_lfg_payloads() {
    let env = WowLuaEnv::new().unwrap();
    let result: String = env
        .eval(
            r#"
            local pet = C_PetJournal.GetPetInfoTableBySpeciesID(39)
            if type(pet) ~= "table" then return "pet-type" end
            if pet.name ~= "Mechanical Squirrel" then return "pet-name" end
            if pet.icon ~= 132932 or pet.petType ~= 9 or pet.speciesID ~= 39 then return "pet-identity" end
            if pet.isWild ~= false or pet.canBattle ~= true then return "pet-battle" end
            if pet.isTradeable ~= false or pet.isUnique ~= false or pet.obtainable ~= true then return "pet-flags" end
            if pet.canAttachToDecor ~= false or pet.creatureModelScale ~= 1 then return "pet-12-1" end
            if C_PetJournal.GetPetInfoTableBySpeciesID(999999) ~= nil then return "pet-unknown" end

            local listing = C_LFGList.GetSearchResultInfo(7)
            if type(listing) ~= "table" then return "lfg-type" end
            if listing.searchResultID ~= 7 or listing.name ~= "RBG yolo" then return "lfg-identity" end
            if listing.activityID ~= 493 or listing.activityIDs[1] ~= 493 then return "lfg-activity" end
            if listing.numMembers ~= 7 or listing.maxMembers ~= 10 then return "lfg-size" end
            if listing.partyGUID ~= "Party-3-0000-1234-00000007" then return "lfg-guid" end
            if listing.censored ~= false then return "lfg-censored" end
            if C_LFGList.GetSearchResultInfo(999999) ~= nil then return "lfg-unknown" end
            return "ok"
            "#,
        )
        .unwrap();

    assert_eq!(result, "ok");
}

#[cfg(feature = "retail-12-1-0")]
#[test]
fn test_patch_12_1_spell_cooldown_payload() {
    let env = WowLuaEnv::new().unwrap();
    {
        let mut state = env.state().borrow_mut();
        let start = state.start_time.elapsed().as_secs_f64();
        state.spell_cooldowns.insert(
            12345,
            crate::lua_api::state::SpellCooldownState {
                start,
                duration: 12.5,
            },
        );
    }

    let result: String = env
        .eval(
            r#"
            local active = C_Spell.GetSpellCooldown(12345)
            if type(active) ~= "table" then return "active-type" end
            if active.startTime < 0 or active.duration ~= 12.5 then return "active-time" end
            if active.isEnabled ~= true or active.isActive ~= true or active.modRate ~= 1 then return "active-flags" end

            local inactive = C_Spell.GetSpellCooldown(999999)
            if type(inactive) ~= "table" then return "inactive-type" end
            if inactive.startTime ~= 0 or inactive.duration ~= 0 then return "inactive-time" end
            if inactive.isEnabled ~= true or inactive.isActive ~= false or inactive.modRate ~= 1 then return "inactive-flags" end
            return "ok"
            "#,
        )
        .unwrap();

    assert_eq!(result, "ok");
}
