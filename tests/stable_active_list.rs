//! Supplied PetInfo observations through the selected original active-slot refresh.
#![cfg(any(feature = "client-retail", feature = "client-ptr"))]

use crate::common::blizzard_addon_harness::{
    blizzard_ui_dir, load_blizzard_addon_closure_into_env, new_blizzard_addon_env,
};
use wow_ui_sim::client_profile::{ACTIVE, ClientProfile};
use wow_ui_sim::loader::find_toc_file;
use wow_ui_sim::lua_api::WowLuaEnv;
use wow_ui_sim::screen::ScreenKind;
use wow_ui_sim::toc::TocFile;

// Retail StableInfoDocumentation.lua:15–22,193–214; PTR 70493:19–26,215–241.
// Neither declaration specifies native ordering, initial membership or an empty default.
fn selected_stable_source(ui: &std::path::Path) -> &'static str {
    let path = find_toc_file(&ui.join("Blizzard_StableUI")).unwrap();
    let toc = TocFile::from_file(&path).unwrap();
    assert!(toc.allows_screen(ScreenKind::Game));
    assert!(!toc.is_game_type_restricted());
    let suffix = match ACTIVE {
        ClientProfile::Retail => "Blizzard_StableUI/Blizzard_StableUI.lua",
        ClientProfile::Ptr => "Blizzard_StableUI/Standard/Blizzard_StableUI.lua",
        _ => unreachable!(),
    };
    assert!(
        toc.files
            .iter()
            .any(|file| file.ends_with(suffix.split_once('/').unwrap().1))
    );
    eprintln!("selected StableUI: {} {:?}", path.display(), toc.files);
    suffix
}

fn load_original_stable() -> WowLuaEnv {
    let ui = blizzard_ui_dir();
    let suffix = selected_stable_source(&ui);
    let env = new_blizzard_addon_env(&ui);
    wow_ui_sim::xml::register_intrinsic_templates();
    let loaded = load_blizzard_addon_closure_into_env(
        &env,
        &ui,
        &["Blizzard_FrameXML", "Blizzard_StableUI"],
        &[],
    );
    assert!(loaded.iter().any(|name| name == "Blizzard_StableUI"));
    eprintln!(
        "unfiltered setup diagnostics: {:?}",
        env.state().borrow().lua_errors
    );
    env.exec(&format!("expectedStableSource = {suffix:?}"))
        .unwrap();
    env.exec(ORIGINAL_SLOT_OBSERVER)
        .expect("original selected StableUI XML and Lua identity");
    env.exec("A_Admin.SetStablePetInfo(Constants.PetConsts_PostCata.EXTRA_PET_STABLE_SLOT + 1, nil)")
        .expect("explicit empty secondary slot observation");
    env
}

const ORIGINAL_SLOT_OBSERVER: &str = r#"
        assert(StableFrame and StableFrame.ActivePetList, 'original XML list')
        local list = StableFrame.ActivePetList
        assert(list.Refresh == StableActivePetListMixin.Refresh, 'original Refresh identity')
        assert(#list.PetButtons >= 3, 'original XML slots')
        for _, button in ipairs(list.PetButtons) do
            assert(button:GetParent() == list, 'original slot parent')
            assert(button.SetPet == StableActivePetButtonTemplateMixin.SetPet)
            assert(button.Icon and button.Lock and button.Border, 'original slot regions')
        end
        local info = debug.getinfo(list.Refresh, 'S')
        local source = info.source:gsub('\\', '/')
        assert(info.what == 'Lua' and source:sub(-#expectedStableSource) == expectedStableSource)
        print('original active list', source, info.linedefined, info.lastlinedefined)
        function suppliedPet(slot, name)
            return {
                slotID=slot, icon=132203+slot, name=name, level=60+slot,
                familyName='Supplied family '..slot, specialization='Supplied spec '..slot,
                type='Supplied type '..slot, petAbilities={101+slot, 201+slot},
                specAbilities={301+slot}, displayID=1000+slot,
                isFavorite=slot==3, isExotic=slot==1, uiModelSceneID=900+slot,
                petNumber=700+slot, creatureID=800+slot, specID=70+slot,
                loyaltyLevel=slot+1, loyaltyName='Supplied loyalty '..slot,
                happinessLevel=slot+2, experience=110+slot, experienceNeeded=220+slot,
            }
        end
        function observeActiveSlots(expected)
            local list = StableFrame.ActivePetList
            local reached = false
            local successorLine = expectedStableSource:find('/Standard/', 1, true) and 479 or 468
            debug.sethook(function(_, line)
                local info = debug.getinfo(2, 'S')
                local source = info.source:gsub('\\', '/')
                if line ~= successorLine or source:sub(-#expectedStableSource) ~= expectedStableSource then return end
                debug.sethook()
                assert(#list.pets == #expected, 'supplied membership count')
                for i, pet in ipairs(expected) do
                    local actual = list.pets[i]
                    for key, value in pairs(pet) do
                        if expectedStableSource:find('/Standard/', 1, true) or
                            (key ~= 'loyaltyLevel' and key ~= 'loyaltyName' and key ~= 'happinessLevel'
                             and key ~= 'experience' and key ~= 'experienceNeeded') then
                            if type(value) == 'table' then
                                assert(#actual[key] == #value, key..' length')
                                for j, item in ipairs(value) do assert(actual[key][j] == item, key) end
                            else
                                assert(actual[key] == value, key)
                            end
                        else
                            assert(actual[key] == nil, 'PTR-only field leaked: '..key)
                        end
                    end
                end
                for slot, button in ipairs(list.PetButtons) do
                    local wanted
                    for _, pet in ipairs(expected) do if pet.slotID == slot then wanted = pet end end
                    assert((button.petData == nil) == (wanted == nil), 'slot cleared/populated '..slot)
                    if wanted then
                        assert(button.petData.name == wanted.name and button.petData.petNumber == wanted.petNumber)
                        assert(button.petData == list.pets[1] or button.petData == list.pets[2])
                    end
                end
                reached = true
                print('original active slots consumed', #expected)
            end, 'l')
            local ok, reason = pcall(list.Refresh, list)
            debug.sethook()
            assert(reached, tostring(reason or 'active-slot observation not reached'))
            -- The secondary slot has an explicit empty observation in this fixture.
            assert(ok, tostring(reason))
        end
    "#;

#[test]
fn original_active_list_tracks_supplied_records_and_isolation() {
    let first = load_original_stable();
    first.exec(r#"
        local ok, reason = pcall(StableFrame.ActivePetList.Refresh, StableFrame.ActivePetList)
        assert(not ok and tostring(reason):find('native-unknown: C_StableInfo.GetActivePetList', 1, true), tostring(reason))
        alpha = suppliedPet(3, 'Alpha observation')
        A_Admin.SetActivePetList({alpha})
        observeActiveSlots({alpha})
        beta = suppliedPet(1, 'Beta observation')
        A_Admin.SetActivePet(2, beta)
        observeActiveSlots({alpha, beta})
        alpha.name = 'Updated observation'
        alpha.petAbilities = {431, 432, 433}
        A_Admin.SetActivePet(1, alpha)
        observeActiveSlots({alpha, beta})
        A_Admin.RemoveActivePet(2)
        observeActiveSlots({alpha})
    "#).expect("populated/add/update/remove through original Refresh");
    let second = load_original_stable();
    second.exec(r#"
        local ok, reason = pcall(C_StableInfo.GetActivePetList)
        assert(not ok and tostring(reason):find('native-unknown:', 1, true), 'no cross-environment default')
        other = suppliedPet(1, 'Independent observation')
        A_Admin.SetActivePetList({other})
        observeActiveSlots({other})
    "#).unwrap();
    first
        .exec(
            r#"
        observeActiveSlots({alpha})
        A_Admin.RemoveActivePet(1)
        observeActiveSlots({})
        A_Admin.SetActivePetList(nil)
        local ok, reason = pcall(StableFrame.ActivePetList.Refresh, StableFrame.ActivePetList)
        assert(not ok and tostring(reason):find('native-unknown:', 1, true), tostring(reason))
    "#,
        )
        .unwrap();
    second.exec("observeActiveSlots({other})").unwrap();
}

#[test]
fn supplied_active_list_is_a_snapshot_with_explicit_unknown_and_validation() {
    let env = load_original_stable();
    env.exec(
        r#"
        local ok, reason = pcall(StableFrame.ActivePetList.Refresh, StableFrame.ActivePetList)
        assert(not ok and tostring(reason):find('native-unknown: C_StableInfo.GetActivePetList', 1, true), tostring(reason))
        local pet = suppliedPet(3, 'Snapshot observation')
        A_Admin.SetActivePetList({pet})
        pet.name = 'Caller mutation'
        pet.petAbilities[1] = -1
        local observed = suppliedPet(3, 'Snapshot observation')
        observeActiveSlots({observed})
        local result = C_StableInfo.GetActivePetList()
        assert(select('#', C_StableInfo.GetActivePetList()) == 1)
        result[1].name = 'Result mutation'
        result[1].petAbilities[1] = -2
        result[2] = suppliedPet(1, 'Result append')
        observeActiveSlots({observed})
        for _, mutate in ipairs({
            function() A_Admin.SetActivePet(0, observed) end,
            function() A_Admin.SetActivePet(3, observed) end,
            function() A_Admin.RemoveActivePet(2) end,
            function() A_Admin.SetActivePetList({{slotID=1}}) end,
            function() A_Admin.SetActivePet(1, {slotID=1}) end,
        }) do
            assert(not pcall(mutate), 'invalid simulator input must fail explicitly')
            observeActiveSlots({observed})
        end
        A_Admin.SetActivePetList(nil)
        assert(not pcall(A_Admin.SetActivePet, 1, observed), 'unknown ordering cannot be appended')
        assert(not pcall(A_Admin.RemoveActivePet, 1), 'unknown membership cannot be removed')
        local ok, reason = pcall(C_StableInfo.GetActivePetList)
        assert(not ok and tostring(reason):find('native-unknown:', 1, true), tostring(reason))
        A_Admin.SetActivePetList({})
        observeActiveSlots({})
        A_Admin.SetActivePet(1, observed)
        observeActiveSlots({observed})
    "#,
    )
    .expect("supplied observation snapshots and transactional simulator input validation");
}
