//! Original cached consumers; source-level staging, not native-client evidence.
#![cfg(any(feature = "client-era", feature = "client-anniversary"))]

use wow_ui_sim::lua_api::WowLuaEnv;

fn load_cached(env: &WowLuaEnv, relative: &str) {
    let root = wow_ui_sim::client_profile::blizzard_ui_addons_dir_under(std::path::Path::new(
        env!("CARGO_MANIFEST_DIR"),
    ));
    let source = std::fs::read_to_string(root.join(relative))
        .unwrap_or_else(|error| panic!("read {relative}: {error}"));
    env.exec(&source)
        .unwrap_or_else(|error| panic!("execute original {relative}: {error}"));
}

fn load_constants(env: &WowLuaEnv) {
    // Preserve the three Constants chunks' actual FrameXMLBase TOC ordering.
    for file in ["Classic", "Shared", "Vanilla"] {
        load_cached(env, &format!("Blizzard_FrameXMLBase/{file}/Constants.lua"));
    }
}

#[test]
fn classic_a_r1_original_castbar_fill_and_full_transitions() {
    let env = WowLuaEnv::new().unwrap();
    load_cached(&env, "Blizzard_UIPanels_Game/Shared/CastingBarFrame.lua");
    env.exec(
        r#"
        local bar = CreateFrame('StatusBar')
        Mixin(bar, CastingBarMixin)
        bar.classicStyleCastBar = true
        for _, kind in ipairs({'Standard', 'Channel', 'Interrupted', 'Uninterruptable'}) do
            bar.barType = CastingBarType[kind]
            for _, full in ipairs({false, true}) do
                bar:UpdateBarFillTexture(full)
                local info = CastingBarTypeInfo[bar.barType]
                local color = full and info.classicFullColor or info.classicFillColor
                local r, g, b, a = color:GetRGBA()
                local actualR, actualG, actualB = bar:GetStatusBarColor()
                assert(math.abs(actualR-r) < 0.00001)
                assert(math.abs(actualG-g) < 0.00001)
                assert(math.abs(actualB-b) < 0.00001)
                assert(a == 0) -- preserved existing Mists palette policy, not native parity
            end
        end
    "#,
    )
    .expect("CastingBarFrame.lua:622-630 applies producer colors");
}

#[test]
fn classic_a_r3_original_quality_bank_and_difficulty_cascades() {
    let env = WowLuaEnv::new().unwrap();
    load_constants(&env);
    load_cached(&env, "Blizzard_UIParent/Vanilla/UIParent.lua");
    load_cached(&env, "Blizzard_UnitPopup/Vanilla/UnitPopup.lua");
    load_cached(
        &env,
        "Blizzard_SharedXML/Classic/SharedUIPanelTemplates.lua",
    );
    load_cached(&env, "Blizzard_UIPanels_Game/Vanilla/BankFrame.lua");
    env.exec(
        r#"
        assert(NEW_ITEM_ATLAS_BY_QUALITY[0] == 'bags-glow-white')
        assert(NUM_LE_ITEM_QUALITYS == 9)
        for quality = 0, 8 do
            local color = assert(ITEM_QUALITY_COLORS[quality])
            local r, g, b = C_Item.GetItemQualityColor(quality)
            assert(color.r == r and color.g == g and color.b == b)
            assert(type(color.hex) == 'string' and color.color:GetRGB() == r)
        end
        assert(RAID_DIFFICULTY_MAP[DIFFICULTY_PRIMARYRAID_NORMAL][10] == DIFFICULTY_RAID10_NORMAL)
        local slots = CreateFrame('Frame')
        slots.Item1 = CreateFrame('Button', nil, slots)
        for i = 1, NUM_BANKBAGSLOTS do
            slots['Bag'..i] = CreateFrame('Button', nil, slots)
        end
        BankSlotsFrame_OnLoad(slots)
        assert(slots:GetID() == BANK_CONTAINER and BANK_CONTAINER == Enum.BagIndex.Bank)
        assert(slots['Item'..NUM_BANKGENERIC_SLOTS]:GetID() == NUM_BANKGENERIC_SLOTS)
        local bank = CreateFrame('Frame')
        bank:SetWidth(400)
        BankFrame_OnLoad(bank)
        assert(bank:GetID() == BANK_CONTAINER and bank.size == NUM_BANKGENERIC_SLOTS)
    "#,
    )
    .expect("quality publication lets original Constants/UIParent/UnitPopup/Bank consumers finish");
}

#[test]
fn classic_a_r4_original_clubfinder_uses_vanilla_catalog_and_colors() {
    let env = WowLuaEnv::new().unwrap();
    load_cached(&env, "Blizzard_SharedXML/Vanilla/ClassColors.lua");
    load_cached(&env, "Blizzard_SharedXML/ColorUtil.lua");
    load_cached(&env, "Blizzard_Communities/ClubFinder.lua");
    // Only menu presentation is captured. Class/spec queries and color helpers stay real.
    env.exec(
        r#"
        local labels = {}
        DropdownButtonMixin = { SetupMenu = function(self, initialize)
            local submenu = {
                CreateButton = function() end,
                CreateCheckbox = function(self, text) table.insert(labels, text) end,
            }
            initialize(self, {
                SetTag = function() end,
                CreateCheckbox = function() return submenu end,
            })
        end }
        local dropdown = {SetSelectionText = function() end, checkedList = {}}
        Mixin(dropdown, ClubLookingForDropdownMixin)
        dropdown:SetupMenu()
        assert(GetNumClasses() == 9)
        local seen = {}
        for index = 1, GetNumClasses() do
            local name, file, id = GetClassInfo(index)
            seen[file] = id
            assert(GetClassColorObj(file), name)
            local canonical = C_CreatureInfo.GetClassInfo(id)
            assert(canonical.classFile == file and canonical.classID == id)
        end
        assert(seen.DRUID == 11 and seen.DEATHKNIGHT == nil and seen.MONK == nil)
        assert(seen.DEMONHUNTER == nil and seen.EVOKER == nil)
        assert(C_CreatureInfo.GetClassInfo(6) == nil)
        assert(#labels > 0)
        for _, text in ipairs(labels) do
            assert(text:find('|c', 1, true) == 1 and text:sub(-2) == '|r')
            assert(not text:find('Evoker', 1, true))
        end
    "#,
    )
    .expect("ClubFinder.lua:551-566 builds colored role/spec entries from Vanilla identities");
}

#[test]
fn classic_a_r6_original_honor_load_and_event_project_state() {
    let env = WowLuaEnv::new().unwrap();
    {
        let mut state = env.state().borrow_mut();
        let honor = &mut state.pvp_honor;
        honor.classic_honor_system_enabled = true;
        honor.yesterday_honorable_kills = 17;
        honor.this_week_honorable_kills = 23;
        honor.this_week_contribution = 456;
        honor.last_week_honorable_kills = 31;
        honor.last_week_contribution = 789;
        honor.session_honorable_kills = 7;
        honor.session_dishonorable_kills = 2;
        honor.lifetime_honorable_kills = 101;
        honor.lifetime_dishonorable_kills = 3;
        honor.rank_progress = 0.4;
    }
    load_cached(&env, "Blizzard_UIPanels_Game/Classic/HonorFrame_Shared.lua");
    // Presentation fixture: no query/consumer function replacement.
    env.exec(
        r#"
        HonorFrame = CreateFrame('Frame', 'HonorFrame')
        for _, suffix in ipairs({'YesterdayHKValue', 'YesterdayContributionValue',
            'ThisWeekHKValue', 'ThisWeekContributionValue', 'LastWeekHKValue',
            'LastWeekContributionValue', 'CurrentHKValue', 'CurrentDKValue',
            'LifeTimeHKValue', 'LifeTimeDKValue', 'LifeTimeRankValue',
            'CurrentPVPTitle', 'CurrentPVPRank'}) do
            _G['HonorFrame'..suffix] = HonorFrame:CreateFontString()
        end
        HonorFramePvPIcon = HonorFrame:CreateTexture()
        HonorFrameProgressBar = CreateFrame('StatusBar')
        HonorFrameProgressBar:SetMinMaxValues(0, 1)
        HonorFrame_GetCurrencyFrame = function() return HonorFrame end
        HonorFrame_OnLoad(HonorFrame)
        assert(HonorFrame:IsShown())
        HonorFrame_OnEvent(HonorFrame, 'PLAYER_ENTERING_WORLD')
        assert(HonorFrameYesterdayHKValue:GetText() == '17')
        assert(HonorFrameYesterdayContributionValue:GetText() == '0')
        assert(HonorFrameThisWeekHKValue:GetText() == '23')
        assert(HonorFrameThisWeekContributionValue:GetText() == '456')
        assert(HonorFrameLastWeekHKValue:GetText() == '31')
        assert(HonorFrameLastWeekContributionValue:GetText() == '789')
        assert(HonorFrameCurrentHKValue:GetText() == '7')
        assert(HonorFrameCurrentDKValue:GetText() == '2')
        assert(HonorFrameLifeTimeHKValue:GetText() == '101')
        assert(HonorFrameLifeTimeDKValue:GetText() == '3')
        HonorFrame_OnEvent(HonorFrame, 'PLAYER_PVP_RANK_CHANGED')
        assert(math.abs(HonorFrameProgressBar:GetValue() - 0.4) < 0.00001)
    "#,
    )
    .expect("HonorFrame_Shared.lua:27-94 consumes modeled honor globals");
    env.state().borrow_mut().pvp_honor.yesterday_contribution = 987;
    env.exec("HonorFrame_OnEvent(HonorFrame, 'PLAYER_PVP_KILLS_CHANGED'); assert(HonorFrameYesterdayContributionValue:GetText() == '987')").unwrap();
    env.state()
        .borrow_mut()
        .pvp_honor
        .classic_honor_system_enabled = false;
    env.exec("HonorFrame_UpdateShown(); assert(not HonorFrame:IsShown())")
        .unwrap();
}

#[test]
fn classic_a_r12_original_marker_menu_uses_enabled_capability() {
    let env = WowLuaEnv::new().unwrap();
    load_cached(
        &env,
        "Blizzard_CompactRaidFrames/Classic/Blizzard_CompactRaidFrameManager.lua",
    );
    for enabled in [true, false] {
        env.state()
            .borrow_mut()
            .plain_global_inputs
            .raid_marker_system_enabled = enabled;
        env.exec(
            r#"
        local entries = {}
        local dropdown = CreateFrame('Frame')
        dropdown.SetupMenu = function(self, initialize)
            initialize(self, {
                CreateCheckbox = function(self, text, selected, toggle, index)
                    entries[#entries+1] = {index=index, selected=selected, toggle=toggle}
                end,
                CreateButton = function() end,
            })
        end
        local ready = CreateFrame('Button')
        ready:SetWidth(80)
        dropdown:SetWidth(24)
        CompactRaidFrameManager_SetupRaidMarkerDropdown({displayFrame={leaderOptions={
            raidMarkerDropdown=dropdown, readyCheckButton=ready,
        }}})
        if IsRaidMarkerSystemEnabled() then
            assert(#entries == NUM_WORLD_RAID_MARKERS and #entries > 0)
            assert(ready:GetWidth() == 80 and dropdown:IsShown())
        else
            assert(#entries == 0 and not dropdown:IsShown())
            assert(ready:GetWidth() == 104)
        end
    "#,
        )
        .expect(
            "CompactRaidFrameManager.lua:73-111 creates the real marker menu or widens Ready Check",
        );
    }
}

#[test]
fn classic_a_r13_original_watch_login_and_sixth_watch_boundary() {
    let env = WowLuaEnv::new().unwrap();
    load_cached(&env, "Blizzard_FrameXMLBase/Vanilla/Constants.lua");
    load_cached(&env, "Blizzard_UIPanels_Game/Vanilla/QuestLogFrame.lua");
    env.exec(
        r#"
        assert(MAX_WATCHABLE_QUESTS == 5)
        QuestWatch_OnLogin()
        assert(#QUEST_WATCH_LIST == 3)
        assert(QUEST_WATCH_LIST[1].id == 80000)
        assert(QUEST_WATCH_LIST[2].id == 80001)
        assert(QUEST_WATCH_LIST[3].id == 80002)
        -- Supplied persisted list: capacity test, not modeled membership proof.
        QUEST_WATCH_LIST = {}
        for id = 91001, 91005 do
            table.insert(QUEST_WATCH_LIST, {id=id, timer=QUEST_WATCH_NO_EXPIRE})
        end
        local questIndex = GetQuestLogIndexByID(80002)
        AutoQuestWatch_Insert(questIndex, QUEST_WATCH_NO_EXPIRE)
        assert(#QUEST_WATCH_LIST == 5 and QUEST_WATCH_LIST[1].id == 91001)
        QUEST_WATCH_LIST[2].timer = 1
        AutoQuestWatch_Insert(questIndex, 30)
        assert(#QUEST_WATCH_LIST == 5 and QUEST_WATCH_LIST[5].id == 80002)
        for _, watch in ipairs(QUEST_WATCH_LIST) do assert(watch.id ~= 91002) end
    "#,
    )
    .expect(
        "QuestLogFrame.lua:623-632,742-767 restores watches and replaces only expiring entries",
    );
}
