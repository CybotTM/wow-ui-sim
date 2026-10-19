//! Original Wrath consumers loaded in place through cache-rooted parsed TOCs.

use wow_ui_sim::lua_api::WowLuaEnv;

pub fn load_original(env: &WowLuaEnv, name: &str) {
    let cache = dirs::cache_dir().expect("cache directory")
        .join("wow-ui-sim/blizzard-ui/wrath/FrameXML");
    assert!(cache.join(name).is_file(), "SETUP: original consumer {name} absent");
    let mut toc = wow_ui_sim::toc::TocFile::parse(
        &cache,
        &format!("## Interface: 30300\n{name}\n"),
    );
    toc.name = format!("Blizzard_LabelFixture_{name}");
    let loaded = wow_ui_sim::loader::load_addon_from_toc(&env.loader_env(), &toc)
        .expect("SETUP: load unchanged original consumer");
    assert!(loaded.warnings.is_empty(), "SETUP: {:?}", loaded.warnings);
}

pub fn tooltip_env() -> WowLuaEnv {
    let env = WowLuaEnv::new().unwrap();
    for name in ["Fonts.xml", "FontStyles.xml", "GameTooltip.lua", "GameTooltipTemplate.xml"] {
        load_original(&env, name);
    }
    env.exec(r#"
        B6Tip=CreateFrame('GameTooltip','B6Tip',UIParent,'GameTooltipTemplate')
        B6Owner=CreateFrame('Button','B6Owner',UIParent)
        B6Tip:SetOwner(B6Owner,'ANCHOR_NONE')
    "#).unwrap();
    env
}

pub fn lfr_env() -> WowLuaEnv {
    let env = tooltip_env();
    load_original(&env, "LFRFrame.lua");
    env.exec(LFR_INPUTS).unwrap();
    env
}

// Supplied server/localization inputs only; no consumer or tooltip replacements.
const LFR_INPUTS: &str = r#"
    GameTooltip=B6Tip
    LOOKING_FOR_RAID='Raid'; LFM_NUM_RAID_MEMBER_TEMPLATE='%d members'
    IMPORTANT_PEOPLE_IN_GROUP='Relationships'; FRIEND='Friend'; IGNORED='Ignored'
    GREEN_FONT_COLOR={r=0,g=1,b=0}; RED_FONT_COLOR={r=1,g=0,b=0}
    function SearchLFGGetResults(index)
        assert(index==1)
        return 'Leader',80,'Icecrown','Paladin','',2,nil,'PALADIN',0,0,true,true,false,false
    end
    function SearchLFGGetPartyResults(index,member)
        assert(index==1)
        if member==1 then return 'Alice',80,'friend','Mage','Icecrown','' end
        return 'Bob',80,nil,'Warrior','Icecrown',''
    end
    B6Owner.index=1
    assert(type(LFRBrowseButton_OnEnter)=='function','SETUP: original LFR entry absent')
    LFRBrowseButton_OnEnter(B6Owner)
    assert(B6Tip:IsShown(),'SETUP: original LFR did not show tooltip')
"#;
