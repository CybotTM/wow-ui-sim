//! Supplied slot observations through selected original StableUI methods.
#![cfg(any(feature = "client-retail", feature = "client-ptr"))]

use crate::common::blizzard_addon_harness::{
    blizzard_ui_dir, load_blizzard_addon_closure_into_env, new_blizzard_addon_env,
};
use wow_ui_sim::client_profile::{ACTIVE, ClientProfile};
use wow_ui_sim::loader::find_toc_file;
use wow_ui_sim::lua_api::WowLuaEnv;
use wow_ui_sim::screen::ScreenKind;
use wow_ui_sim::toc::TocFile;

fn load_original_stable() -> WowLuaEnv {
    let ui = blizzard_ui_dir();
    let toc_path = find_toc_file(&ui.join("Blizzard_StableUI")).unwrap();
    let toc = TocFile::from_file(&toc_path).unwrap();
    assert!(toc.allows_screen(ScreenKind::Game));
    assert!(!toc.is_game_type_restricted());
    let relative = if ACTIVE == ClientProfile::Ptr {
        "Standard/Blizzard_StableUI.lua"
    } else {
        "Blizzard_StableUI.lua"
    };
    assert!(toc.files.iter().any(|file| file.ends_with(relative)));
    eprintln!("selected TOC {} {:?}", toc_path.display(), toc.files);
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
    env.exec(&format!(
        "expectedStableSource = {:?}",
        format!("Blizzard_StableUI/{relative}")
    ))
    .unwrap();
    env.exec(ORIGINAL_OBSERVER).unwrap();
    env
}

const ORIGINAL_OBSERVER: &str = r#"
    assert(StableFrame and StableFrame.ActivePetList, 'original installed XML')
    assert(StableFrame.RefreshSelectedPetData == StableFrameMixin.RefreshSelectedPetData)
    assert(StableFrame.ActivePetList.Refresh == StableActivePetListMixin.Refresh)
    for _, method in ipairs({StableFrame.RefreshSelectedPetData, StableFrame.ActivePetList.Refresh}) do
        local info = debug.getinfo(method, 'S')
        local source = info.source:gsub('\\', '/')
        assert(info.what == 'Lua' and source:sub(-#expectedStableSource) == expectedStableSource)
        print('original slot consumer', source, info.linedefined, info.lastlinedefined)
    end
    function suppliedSlotPet(slot, name)
        return {
            slotID=slot, icon=132203+slot, name=name, level=60+slot,
            familyName='Observed family '..slot, specialization='Observed spec '..slot,
            type='Observed type '..slot, petAbilities={101+slot,201+slot},
            specAbilities={301+slot}, displayID=1000+slot,
            isFavorite=slot==3, isExotic=slot==1, uiModelSceneID=900+slot,
            petNumber=700+slot, creatureID=800+slot, specID=70+slot,
            loyaltyLevel=slot+1, loyaltyName='Observed loyalty '..slot,
            happinessLevel=slot+2, experience=110+slot, experienceNeeded=220+slot,
        }
    end
    function assertSlotRecord(actual, wanted)
        assert(actual and wanted, 'populated slot record')
        for key, value in pairs(wanted) do
            local ptrOnly = key=='loyaltyLevel' or key=='loyaltyName' or key=='happinessLevel'
                or key=='experience' or key=='experienceNeeded'
            if ptrOnly and not expectedStableSource:find('/Standard/',1,true) then
                assert(actual[key]==nil, 'PTR-only field leaked: '..key)
            elseif type(value)=='table' then
                assert(#actual[key]==#value, key..' length')
                for i, item in ipairs(value) do assert(actual[key][i]==item, key) end
            else
                assert(actual[key]==value, key)
            end
        end
    end
    function refreshSelected(slot, newSlot)
        StableFrame.selectedPet={slotID=slot, name='old selection'}
        StableFrame.selectedPetNewSlot=newSlot
        StableFrame:RefreshSelectedPetData()
        return StableFrame.selectedPet
    end
    function assertUnknownSelection(slot)
        local ok, reason=pcall(refreshSelected, slot)
        assert(not ok and tostring(reason):find(
            'native-unknown: C_StableInfo.GetStablePetInfo',1,true), tostring(reason))
        assert(tostring(reason):find('slot '..slot,1,true), tostring(reason))
    end
    function observeSecondary(wanted)
        local list=StableFrame.ActivePetList
        local successor=expectedStableSource:find('/Standard/',1,true) and 480 or 469
        local reached=false
        debug.sethook(function(_,line)
            local source=debug.getinfo(2,'S').source:gsub('\\','/')
            if line~=successor or source:sub(-#expectedStableSource)~=expectedStableSource then return end
            debug.sethook()
            local actual=list.BeastMasterSecondaryPetButton.petData
            if wanted then assertSlotRecord(actual,wanted) else assert(actual==nil,'empty secondary') end
            reached=true
        end,'l')
        local ok, reason=pcall(list.Refresh,list)
        debug.sethook()
        assert(reached,tostring(reason or 'secondary SetPet successor not reached'))
        print('bounded original secondary SetPet',ok,tostring(reason))
    end
"#;

#[test]
fn original_selected_pet_refresh_tracks_slots_and_empty() {
    let env = load_original_stable();
    env.exec(
        r#"
        assertUnknownSelection(2)
        local first=suppliedSlotPet(2,'First observation')
        local second=suppliedSlotPet(7,'Second observation')
        A_Admin.SetStablePetInfo(2,first)
        A_Admin.SetStablePetInfo(7,second)
        assertSlotRecord(refreshSelected(2),first)
        assertSlotRecord(refreshSelected(2,7),second)
        second.name='Replacement observation'
        second.petAbilities={401,402,403}
        A_Admin.SetStablePetInfo(7,second)
        assertSlotRecord(refreshSelected(7),second)
        A_Admin.SetStablePetInfo(7,nil)
        assert(refreshSelected(2,7)==nil,'empty moved-to slot must clear selection')
        assert(select('#',C_StableInfo.GetStablePetInfo(7))==1,'one nullable result')
        assertSlotRecord(refreshSelected(2),first)
        A_Admin.SetStablePetInfo(7,second)
        assertSlotRecord(refreshSelected(2,7),second)
    "#,
    )
    .unwrap();
}

#[test]
fn original_selected_pet_refresh_reports_unknown_after_clear() {
    let first = load_original_stable();
    let second = load_original_stable();
    first
        .exec(
            r#"
        assertUnknownSelection(3)
        own=suppliedSlotPet(3,'First environment')
        A_Admin.SetStablePetInfo(3,own)
        assertSlotRecord(refreshSelected(3),own)
        A_Admin.ClearStablePetInfo(3)
        assertUnknownSelection(3)
        own.name='Recovered observation'
        A_Admin.SetStablePetInfo(3,own)
        assertSlotRecord(refreshSelected(3),own)
    "#,
        )
        .unwrap();
    second
        .exec(
            r#"
        assertUnknownSelection(3)
        own=suppliedSlotPet(3,'Second environment')
        A_Admin.SetStablePetInfo(3,own)
        assertSlotRecord(refreshSelected(3),own)
        A_Admin.SetStablePetInfo(3,nil)
        assert(refreshSelected(3)==nil)
        A_Admin.SetStablePetInfo(3,own)
        assertSlotRecord(refreshSelected(3),own)
    "#,
        )
        .unwrap();
    first
        .exec("assertSlotRecord(refreshSelected(3),own)")
        .unwrap();
}

#[test]
fn original_active_refresh_consumes_secondary_slot_observation() {
    let env = load_original_stable();
    env.exec(
        r#"
        A_Admin.SetActivePetList({})
        -- Establish target reachability before any new admin input exists.
        local ok, reason=pcall(StableFrame.ActivePetList.Refresh,StableFrame.ActivePetList)
        assert(not ok and tostring(reason):find(
            'native-unknown: C_StableInfo.GetStablePetInfo',1,true),tostring(reason))
        local slot=Constants.PetConsts_PostCata.EXTRA_PET_STABLE_SLOT+1
        local pet=suppliedSlotPet(slot,'Secondary observation')
        A_Admin.SetStablePetInfo(slot,pet)
        observeSecondary(pet)
        pet.name='Changed secondary'
        pet.displayID=2345
        A_Admin.SetStablePetInfo(slot,pet)
        observeSecondary(pet)
        A_Admin.SetStablePetInfo(slot,nil)
        observeSecondary(nil)
        A_Admin.SetStablePetInfo(slot,pet)
        observeSecondary(pet)
    "#,
    )
    .unwrap();
}

#[test]
fn stable_slot_snapshots_validate_copy_and_profile_shape() {
    let env = load_original_stable();
    env.exec(
        r#"
        assertUnknownSelection(2)
        local pet=suppliedSlotPet(2,'Snapshot observation')
        A_Admin.SetStablePetInfo(2,pet)
        pet.name='Caller mutation'
        pet.petAbilities[1]=-1
        local wanted=suppliedSlotPet(2,'Snapshot observation')
        assertSlotRecord(refreshSelected(2),wanted)
        local result=C_StableInfo.GetStablePetInfo(2)
        result.name='Result mutation'
        result.petAbilities[1]=-2
        result.specAbilities[1]=-3
        assertSlotRecord(refreshSelected(2),wanted)
        local invalid=suppliedSlotPet(2,'Invalid')
        invalid.icon=0/0
        for _, mutate in ipairs({
            function() A_Admin.SetStablePetInfo(0,wanted) end,
            function() A_Admin.SetStablePetInfo(1.5,wanted) end,
            function() A_Admin.SetStablePetInfo(1/0,wanted) end,
            function() A_Admin.SetStablePetInfo(2,{slotID=2}) end,
            function() A_Admin.SetStablePetInfo(7,wanted) end,
            function() A_Admin.SetStablePetInfo(2,invalid) end,
            function() A_Admin.ClearStablePetInfo(0) end,
        }) do
            assert(not pcall(mutate),'invalid simulator input')
            assertSlotRecord(refreshSelected(2),wanted)
        end
        assert(not pcall(C_StableInfo.GetStablePetInfo))
        assert(not pcall(C_StableInfo.GetStablePetInfo,{}))
        A_Admin.SetStablePetInfo(2,nil)
        assert(refreshSelected(2)==nil)
        assert(select('#',C_StableInfo.GetStablePetInfo(2))==1)
        A_Admin.ClearStablePetInfo(2)
        assertUnknownSelection(2)
    "#,
    )
    .unwrap();
}
