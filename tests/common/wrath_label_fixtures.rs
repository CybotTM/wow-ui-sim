//! Original Wrath consumers loaded in place through cache-rooted parsed TOCs.

use wow_ui_sim::lua_api::WowLuaEnv;

#[path = "wrath_original_consumer.rs"]
mod originals;
use originals::load_original_files;

pub fn tooltip_env() -> WowLuaEnv {
    let env = WowLuaEnv::new().unwrap();
    load_original_files(
        &env,
        &[
            "Fonts.xml",
            "FontStyles.xml",
            "GameTooltip.lua",
            "GameTooltipTemplate.xml",
            "LFRFrame.lua",
        ],
    );
    env.exec(
        r#"
        B6Tip=CreateFrame('GameTooltip','B6Tip',UIParent,'GameTooltipTemplate')
        B6Owner=CreateFrame('Button','B6Owner',UIParent)
        B6Tip:SetOwner(B6Owner,'ANCHOR_NONE')
    "#,
    )
    .unwrap();
    env
}

pub fn lfr_env() -> WowLuaEnv {
    let env = tooltip_env();
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
