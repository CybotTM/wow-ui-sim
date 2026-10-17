//! Original cached Era/Anniversary consumers; no native-parity claim.
#![cfg(any(feature = "client-era", feature = "client-anniversary"))]
use sha2::{Digest, Sha256};
use wow_ui_sim::lua_api::WowLuaEnv;

fn load_consumer(files: &[&str]) -> WowLuaEnv {
    let env = WowLuaEnv::new().unwrap();
    env.exec("ConsumerErrors = {}; seterrorhandler(function(e) ConsumerErrors[#ConsumerErrors+1] = tostring(e) end)").unwrap();
    for file in files {
        execute_pinned_consumer(&env, file);
    }
    env
}

fn execute_pinned_consumer(env: &WowLuaEnv, file: &str) {
    let root = wow_ui_sim::paths::default_blizzard_ui_addons_path().unwrap();
    let bytes = std::fs::read(root.join(file)).unwrap();
    let expected = include_str!("classic_r10r14_pins.tsv")
        .lines()
        .find_map(|line| line.split_once('\t').filter(|(name, _)| *name == file))
        .unwrap()
        .1;
    assert_eq!(format!("{:x}", Sha256::digest(&bytes)), expected);
    env.loader_env()
        .exec(&String::from_utf8(bytes).unwrap())
        .unwrap();
}

fn run_consumer(env: &WowLuaEnv, source: &str) {
    let result = env.exec(source);
    assert!(
        env.state().borrow().lua_errors.is_empty(),
        "{:?}",
        env.state().borrow().lua_errors
    );
    env.exec("assert(#ConsumerErrors == 0, table.concat(ConsumerErrors, '\n'))")
        .unwrap();
    result.unwrap();
}

#[test]
fn r10_original_skill_detail_hides_no_selection() {
    let env = load_consumer(&["Blizzard_UIPanels_Game/Classic/SkillFrame.lua"]);
    run_consumer(
        &env,
        r#"
        SkillDetailStatusBar = CreateFrame('StatusBar', 'SkillDetailStatusBar', UIParent)
        SkillDetailDescriptionText = SkillDetailStatusBar:CreateFontString()
        assert(GetNumSkillLines() == 0 and GetSelectedSkill() == 0)
        SkillDetailFrame_SetStatusBar(GetSelectedSkill())
        assert(not SkillDetailStatusBar:IsShown())
        assert(not SkillDetailDescriptionText:IsShown())
    "#,
    );
}

#[test]
fn r14_original_quest_timer_events_hide_empty_domain() {
    let env = load_consumer(&[
        "Blizzard_SharedXML/TimeUtil.lua",
        "Blizzard_QuestTimer/Blizzard_QuestTimer.lua",
    ]);
    run_consumer(
        &env,
        r#"
        local timer = CreateFrame('Frame', 'ResidualQuestTimer', UIParent)
        Mixin(timer, QuestTimerMixin)
        for i = 1, MAX_QUESTS do
            local row = CreateFrame('Button', 'QuestTimer'..i, timer)
            row:CreateFontString('QuestTimer'..i..'Text')
            row:Show()
        end
        timer:OnLoad()
        assert(timer:IsEventRegistered('QUEST_LOG_UPDATE'))
        assert(timer:IsEventRegistered('PLAYER_ENTERING_WORLD'))
        for _, event in ipairs({'PLAYER_ENTERING_WORLD', 'QUEST_LOG_UPDATE'}) do
            timer:Show()
            for i = 1, MAX_QUESTS do _G['QuestTimer'..i]:Show() end
            timer:OnEvent(event)
            assert(timer.numTimers == 0 and timer.updating == nil)
            assert(not timer:IsShown())
            for i = 1, MAX_QUESTS do assert(not _G['QuestTimer'..i]:IsShown()) end
        end
        assert(select('#', GetQuestTimers()) == 0)
    "#,
    );
}
