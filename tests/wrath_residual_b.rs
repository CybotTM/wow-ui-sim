//! Source-only staged regressions. Original consumers, not native-client evidence.
#![cfg(feature = "client-wrath")]
use sha2::{Digest, Sha256};
use wow_ui_sim::lua_api::WowLuaEnv;
use wow_ui_sim::lua_api::globals::real::wrath_legacy_state::{Battlefield, Landmark, Skill};

fn original(file: &str) -> String {
    let toc = wow_ui_sim::client_profile::blizzard_ui_framexml_toc().expect("Wrath cache required");
    let path = toc.parent().unwrap().join(file);
    let bytes = std::fs::read(&path).unwrap();
    let pins = include_str!("wrath_residual_b_pins.tsv");
    let expected = pins
        .lines()
        .find_map(|line| line.split_once('\t').filter(|(name, _)| *name == file))
        .unwrap()
        .1;
    assert_eq!(
        format!("{:x}", Sha256::digest(&bytes)),
        expected,
        "changed {}",
        path.display()
    );
    String::from_utf8(bytes).unwrap()
}
fn env(files: &[&str]) -> WowLuaEnv {
    let env = WowLuaEnv::new().unwrap();
    env.exec("ResidualErrors = {}; seterrorhandler(function(e) ResidualErrors[#ResidualErrors+1] = tostring(e) end)").unwrap();
    let fonts = wow_ui_sim::render::font::WowFontSystem::new_without_casc();
    assert!(
        fonts
            .family_name(Some(wow_ui_sim::render::font::DEFAULT_WOW_FONT))
            .is_some()
    );
    env.set_font_system(std::rc::Rc::new(std::cell::RefCell::new(fonts)));
    for file in files {
        original(file);
        if file.ends_with(".xml") {
            let toc = wow_ui_sim::client_profile::blizzard_ui_framexml_toc().unwrap();
            let temp = tempfile::tempdir().unwrap();
            let path = temp.path().join("Residual.toc");
            let original = toc.parent().unwrap().join(file);
            let common = temp
                .path()
                .ancestors()
                .find(|base| original.starts_with(base))
                .unwrap();
            let mut relative = std::path::PathBuf::new();
            for _ in temp.path().strip_prefix(common).unwrap().components() {
                relative.push("..");
            }
            relative.push(original.strip_prefix(common).unwrap());
            std::fs::write(
                &path,
                format!("## Interface: 30300\n{}\n", relative.display()),
            )
            .unwrap();
            let loaded = wow_ui_sim::loader::load_addon(&env.loader_env(), &path).unwrap();
            assert!(loaded.warnings.is_empty(), "{:?}", loaded.warnings);
        } else {
            env.loader_env().exec(&original(file)).unwrap();
        }
    }
    env
}
fn run(env: &WowLuaEnv, lua: &str) {
    let result = env.exec(lua);
    env.exec("assert(#ResidualErrors == 0, table.concat(ResidualErrors, '\\n'))")
        .unwrap();
    assert!(
        env.state().borrow().lua_errors.is_empty(),
        "{:?}",
        env.state().borrow().lua_errors
    );
    result.unwrap();
}

#[test]
fn wrath_resid_b_target_mode_original_event() {
    let env = env(&[
        "Fonts.xml",
        "FontStyles.xml",
        "UIDropDownMenu.xml",
        "InterfaceOptionsPanels.lua",
    ]);
    // InterfaceOptionsFrame.lua:319 supplies the shipped UI default 5.
    // InterfaceOptionsPanels.lua:252-298 is executed unchanged.
    run(
        &env,
        r#"
        RAID='Raid'; PARTY='Party'; SOLO='Solo'; RAID_AND_PARTY='Raid and party'; ALWAYS='Always'
        local f = CreateFrame('Frame', 'ResidualTOT', UIParent, 'UIDropDownMenuTemplate')
        f.uvar = 'SHOW_TARGET_OF_TARGET_STATE'
        OPTION_TOOLTIP_TARGETOFTARGET5 = 'All targets'
        InterfaceOptionsCombatPanelTOTDropDown_OnEvent(f, 'PLAYER_ENTERING_WORLD')
        assert(f.defaultValue == '5' and f.value == '5')
        assert(f.tooltip == 'All targets' and SHOW_TARGET_OF_TARGET_STATE == '5')
        f:SetValue('2')
        assert(GetCVar('targetOfTargetMode') == '2' and f.value == '2')
        assert(GetCVar('ResidualUnknownCVar') == nil)
    "#,
    );
}

fn install_original_mip_getter(env: &WowLuaEnv) {
    // VideoOptionsPanels.xml:677-689; no guessed default. Execute its exact
    // original GetCurrentValue body against an explicit control input.
    let xml = original("VideoOptionsPanels.xml");
    let start = xml.find("self.cvar = \"BaseMip\"").unwrap();
    let tail = &xml[start..];
    let getter = tail.find("self.GetCurrentValue = function (self)").unwrap();
    let body_start = getter + "self.GetCurrentValue = function (self)".len();
    let body_end = tail[body_start..].find("end\n").unwrap() + body_start;
    run(
        env,
        "ResidualMip = CreateFrame('Slider'); ResidualMip.cvar = 'BaseMip'; ResidualMip:SetMinMaxValues(0, 1)",
    );
    run(
        &env,
        &format!(
            "ResidualMip.GetCurrentValue = function(self) {} end",
            &tail[body_start..body_end]
        ),
    );
}

#[test]
fn wrath_resid_b_basemip_original_control_explicit_input_only() {
    let env = env(&["OptionsPanelTemplates.lua"]);
    install_original_mip_getter(&env);
    run(
        &env,
        "RegisterCVar('BaseMip', '0'); assert(ResidualMip:GetCurrentValue() == 1); SetCVar('BaseMip', '1'); assert(ResidualMip:GetCurrentValue() == 0)",
    );
}

#[test]
fn wrath_resid_b_skill_original_detail() {
    let env = env(&["SkillFrame.lua"]);
    run(
        &env,
        r#"
        SkillDetailStatusBar = CreateFrame('StatusBar', 'SkillDetailStatusBar', UIParent)
        SkillDetailStatusBar:SetWidth(256)
        SkillDetailDescriptionText = SkillDetailStatusBar:CreateFontString('SkillDetailDescriptionText')
        SkillDetailCostText = SkillDetailStatusBar:CreateFontString('SkillDetailCostText')
        for _, name in ipairs({'Background','FillBar'}) do SkillDetailStatusBar:CreateTexture('SkillDetailStatusBar'..name) end
        for _, name in ipairs({'SkillRank','SkillName'}) do SkillDetailStatusBar:CreateFontString('SkillDetailStatusBar'..name) end
        for _, name in ipairs({'UnlearnButton','LeftArrow','RightArrow','LearnSkillButton'}) do CreateFrame('Button', 'SkillDetailStatusBar'..name, SkillDetailStatusBar) end
        NORMAL_FONT_COLOR = {r=1,g=1,b=1}; HIGHLIGHT_FONT_COLOR = NORMAL_FONT_COLOR
        SKILL_DESCRIPTION = '%s %s'; SKILL_INCREMENT_COST = '%s%d'; SKILL_INCREMENT_COST_SINGULAR = '%s%d'
        GREEN_FONT_COLOR_CODE = ''; RED_FONT_COLOR_CODE = ''
        SetSelectedSkill(0)
        SkillDetailFrame_SetStatusBar(GetSelectedSkill(), 100)
        assert(not SkillDetailStatusBar:IsShown() and not SkillDetailDescriptionText:IsShown())
    "#,
    );
    env.state().borrow_mut().wrath_legacy.skills.push(Skill {
        name: "Alchemy".into(),
        rank: 42,
        temporary_points: 3,
        modifier: 0,
        max_rank: 75,
        description: "Mix potions".into(),
        ..Default::default()
    });
    // SkillFrame.lua:192-214 and :382-391; real selection setter, real tuple.
    run(
        &env,
        "SetSelectedSkill(1); SkillDetailFrame_SetStatusBar(GetSelectedSkill(), 100); assert(GetSelectedSkill() == 1); assert(SkillDetailStatusBarSkillRank:GetText() == '45/75'); assert(SkillDetailStatusBar:GetValue() == 42); assert(SkillDetailStatusBar:IsShown())",
    );
}

#[test]
fn wrath_resid_b_who_original_xml_lifecycle_routes_results() {
    let env = env(&[]);
    let xml = original("FriendsFrame.xml");
    // FriendsFrame.xml:2635-2643: exact original script bodies. Fixture frame
    // isolates routing from the unrelated WhoFrame templates/list presentation.
    let anchor = xml.find("SetWhoToUI(0)").unwrap();
    let start = xml[..anchor].rfind("<Scripts>").unwrap();
    let end = xml[anchor..].find("</Scripts>").unwrap() + anchor;
    run(
        &env,
        "ResidualWho = CreateFrame('Frame'); ResidualWho:Hide()",
    );
    for kind in ["OnLoad", "OnShow", "OnHide"] {
        let tag = format!("<{kind}>");
        let body_start = xml[start..end].find(&tag).unwrap() + start + tag.len();
        let body_end = xml[body_start..end].find(&format!("</{kind}>")).unwrap() + body_start;
        run(
            &env,
            &format!(
                "ResidualWho:SetScript('{kind}', function(self) {} end)",
                &xml[body_start..body_end]
            ),
        );
    }
    run(&env, "ResidualWho:GetScript('OnLoad')(ResidualWho)");
    assert!(!env.state().borrow().who_routing.to_ui);
    env.state().borrow_mut().who_routing.pending = vec!["Alyth - Level 80".into()];
    run(&env, "ResidualWho:Show(); SendWho('Alyth')");
    assert_eq!(
        env.state().borrow().who_routing.ui_results,
        ["Alyth - Level 80"]
    );
    assert_eq!(
        env.state().borrow().who_routing.last_query.as_deref(),
        Some("Alyth")
    );
    assert!(
        env.state()
            .borrow()
            .events
            .pending()
            .iter()
            .any(|event| event.name == "WHO_LIST_UPDATE")
    );
    env.state().borrow_mut().events.drain();
    run(&env, "ResidualWho:Hide(); SendWho('Alyth')");
    assert!(
        !env.state()
            .borrow()
            .events
            .pending()
            .iter()
            .any(|event| event.name == "WHO_LIST_UPDATE")
    );
    assert!(!env.state().borrow().who_routing.to_ui);
    assert_eq!(
        env.state().borrow().system_chat_log.last().unwrap(),
        "Alyth - Level 80"
    );
    run(&env, "C_FriendList.SetWhoToUi(true)");
    assert!(env.state().borrow().who_routing.to_ui);
}

#[test]
fn wrath_resid_b_map_original_update_selected_snapshot() {
    let env = env(&["WorldMapFrame.lua"]);
    // Unrelated overlay/debug feeds are deliberately empty; landmark producers
    // are never replaced. Entire original WorldMapFrame_Update executes.
    run(&env, include_str!("wrath_residual_b_map_fixture.lua"));
    env.state().borrow_mut().wrath_legacy.landmarks.insert(
        (7, 0),
        vec![Landmark {
            name: "Harbor".into(),
            description: "Dock".into(),
            texture_index: 1,
            x: 0.25,
            y: 0.5,
            map_link_id: 19,
        }],
    );
    run(
        &env,
        "SetMapZoom(7); assert(GetCurrentMapContinent()==7 and GetCurrentMapZone()==0); WorldMapFrame_Update(); local p=WorldMapFramePOI1; assert(p.name=='Harbor' and p.description=='Dock' and p.mapLinkID==19 and p:IsShown()); local _,_,_,x,y=p:GetPoint(); assert(x==200 and y==-300); local a,b,c,d=WorldMap_GetPOITextureCoords(1); local u,v=WorldMapFramePOI1Texture:GetTexCoord(); assert(u==a and v==c)",
    );
    run(
        &env,
        "SetMapZoom(8); assert(GetCurrentMapContinent()==8 and GetCurrentMapZone()==0); WorldMapFrame_Update(); assert(not WorldMapFramePOI1:IsShown())",
    );
}

#[test]
fn wrath_resid_b_battlefield_original_reset_info() {
    let env = env(&["PVPBattlegroundFrame.lua"]);
    run(
        &env,
        r#"
        PVPBattlegroundFrame = CreateFrame('Frame')
        PVPBattlegroundFrame.selectedBG = 1
        PVPBattlegroundFrameBGTex = PVPBattlegroundFrame:CreateTexture()
        PVPBattlegroundFrameInfoScrollFrame = CreateFrame('ScrollFrame')
        PVPBattlegroundFrameInfoScrollFrameChildFrameDescription = PVPBattlegroundFrame:CreateFontString()
        PVPBattlegroundFrameInfoScrollFrameChildFrameRewardsInfo = CreateFrame('Frame')
    "#,
    );
    env.state().borrow_mut().wrath_legacy.battlefields = vec![
        Battlefield {
            name: "One".into(),
            description: "First field".into(),
            max_group: 5,
        },
        Battlefield {
            name: "Two".into(),
            description: "Second field".into(),
            max_group: 3,
        },
    ];
    // PVPBattlegroundFrame.lua:147-151 requests then updates; :97-101 resets
    // scroll only when the selected record's description changes.
    run(
        &env,
        "PVPBattleground_ResetInfo(); assert(PVPBattlegroundFrameInfoScrollFrameChildFrameDescription:GetText()=='First field'); PVPBattlegroundFrameInfoScrollFrame:SetVerticalScroll(17); PVPBattleground_ResetInfo(); assert(PVPBattlegroundFrameInfoScrollFrame:GetVerticalScroll()==17); PVPBattlegroundFrame.selectedBG=2; PVPBattleground_ResetInfo(); assert(PVPBattlegroundFrameInfoScrollFrameChildFrameDescription:GetText()=='Second field'); assert(PVPBattlegroundFrameInfoScrollFrame:GetVerticalScroll()==0)",
    );
}

#[test]
fn wrath_resid_b_companions_original_availability() {
    let env = env(&["PetPaperDollFrame.lua"]);
    for action in &mut env.state().borrow_mut().pet_actions {
        action.has_action = false;
    }
    run(
        &env,
        "PetPaperDollFrame=CreateFrame('Frame'); PetPaperDollFrame:Hide(); CharacterFrameTab2=CreateFrame('Button','CharacterFrameTab2'); CharacterFrameTab3=CreateFrame('Button'); assert(not HasPetUI())",
    );
    {
        let mut state = env.state().borrow_mut();
        let mut mount = state
            .world
            .mounts
            .first()
            .expect("seeded mount fixture")
            .clone();
        mount.mount_id = 202;
        mount.is_collected = false;
        state.world.mounts = vec![mount];
        let mut pet = state
            .world
            .pets
            .first()
            .expect("seeded pet fixture")
            .clone();
        pet.species_id = 101;
        pet.is_collected = false;
        state.world.pets = vec![pet];
        state.wrath_legacy.eligible_critters.insert(101);
        state.wrath_legacy.eligible_mounts.insert(202);
    }
    run(
        &env,
        "PetPaperDollFrame_UpdateIsAvailable(); assert(PetPaperDollFrame.hidden and not CharacterFrameTab2:IsShown())",
    );
    // Explicit eligibility is separate from journal ownership. Uncollected
    // catalog rows above did not make the tab available.
    env.state().borrow_mut().world.pets[0].is_collected = true;
    run(
        &env,
        "PetPaperDollFrame_UpdateIsAvailable(); assert(not PetPaperDollFrame.hidden and CharacterFrameTab2:IsShown())",
    );
    env.state().borrow_mut().world.pets[0].is_collected = false;
    env.state().borrow_mut().world.mounts[0].is_collected = true;
    run(
        &env,
        "PetPaperDollFrame_UpdateIsAvailable(); assert(CharacterFrameTab2:IsShown()); local _,_,p,x=CharacterFrameTab3:GetPoint(); assert(p=='RIGHT' and x==-16)",
    );
    env.state().borrow_mut().world.mounts[0].is_collected = false;
    run(
        &env,
        "PetPaperDollFrame_UpdateIsAvailable(); assert(not CharacterFrameTab2:IsShown()); local _,_,p,x=CharacterFrameTab3:GetPoint(); assert(p=='LEFT' and x==0)",
    );
}

#[test]
fn wrath_resid_b_lfd_original_random_renderer() {
    let env = env(&["LFDFrame.lua"]);
    run(&env, include_str!("wrath_residual_b_lfd_fixture.lua"));
    let id = {
        let mut state = env.state().borrow_mut();
        let row = state
            .lfd_dungeons
            .first_mut()
            .expect("existing dungeon fixture");
        row.difficulty = 1;
        row.texture_filename = "DistinctTexture".into();
        row.description = "Distinct description".into();
        row.is_holiday = false;
        row.dungeon_id
    };
    run(
        &env,
        &format!(
            "LFDQueueFrame.type={id}; LFDQueueFrameRandom_UpdateFrame(); assert(LFDQueueFrameBackground:GetTexture()=='Interface\\\\LFGFrame\\\\UI-LFG-BACKGROUND-HEROIC')"
        ),
    );
    {
        let mut state = env.state().borrow_mut();
        let row = state.lfd_dungeons.first_mut().unwrap();
        row.difficulty = 0;
        row.is_holiday = true;
    }
    run(
        &env,
        "LFDQueueFrameRandom_UpdateFrame(); assert(LFDQueueFrameBackground:GetTexture()=='Interface\\\\LFGFrame\\\\UI-LFG-HOLIDAY-BACKGROUND-DistinctTexture'); assert(LFDQueueFrameRandomScrollFrameChildFrame.description:GetText()=='Distinct description'); assert(not LFDQueueFrameRandomScrollFrameChildFrame.rewardsLabel:IsShown()); LFDQueueFrame.type=nil; LFDQueueFrameRandom_UpdateFrame(); assert(LFDQueueFrameRandomScrollFrameChildFrame.description:GetText()=='Distinct description')",
    );
}
