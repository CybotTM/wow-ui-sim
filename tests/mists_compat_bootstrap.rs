#![cfg(feature = "client-mists")]

use wow_ui_sim::lua_api::WowLuaEnv;
use wow_ui_sim::screen::ScreenKind;
use wow_ui_sim::toc::TocFile;

const HONOR_FRAME_SHARED_LUA: &str = include_str!(
    "../Interface/BlizzardUI/Mists/AddOns/Blizzard_UIPanels_Game/Classic/HonorFrame_Shared.lua"
);

fn blizzard_ui_dir() -> std::path::PathBuf {
    wow_ui_sim::client_profile::blizzard_ui_addons_dir_under(std::path::Path::new(env!(
        "CARGO_MANIFEST_DIR"
    )))
}

#[test]
fn mists_bootstrap_reports_pandaria_as_the_current_classic_expansion() {
    let env = WowLuaEnv::new().expect("Lua environment should initialize");

    let result: (i32, bool, bool, bool, bool) = env
        .eval(
            r#"
            return GetExpansionLevel(),
                ClassicExpansionAtLeast(LE_EXPANSION_MISTS_OF_PANDARIA),
                ClassicExpansionAtMost(LE_EXPANSION_MISTS_OF_PANDARIA),
                ClassicExpansionAtLeast(5),
                ClassicExpansionAtMost(LE_EXPANSION_CATACLYSM)
            "#,
        )
        .expect("Mists expansion helpers should be callable");

    assert_eq!(
        result,
        (4, true, true, false, false),
        "Mists Classic should report MoP as the current classic expansion"
    );
}

#[test]
fn mists_toc_game_token_resolves_to_mists_subdirectory() {
    let toc = TocFile::parse(
        std::path::Path::new("Blizzard_CharacterFrame"),
        r#"
        ## Interface: 50503
        [Game]\PaperDollFrameUtil.lua [AllowLoadGameType cata, mists]
        "#,
    );

    assert_eq!(
        toc.files,
        vec![std::path::PathBuf::from("Mists/PaperDollFrameUtil.lua")],
        "Mists TOC [Game] token should select the Mists source variant"
    );
}

#[test]
fn mists_loads_money_frame_before_uipanels_game() {
    let addons = wow_ui_sim::loader::discover_blizzard_addons_for_screen(
        &blizzard_ui_dir(),
        ScreenKind::Game,
    );
    let money_frame_index = addons
        .iter()
        .position(|(name, _)| name == "Blizzard_MoneyFrame")
        .expect("Mists game startup should include Blizzard_MoneyFrame");
    let ui_panels_index = addons
        .iter()
        .position(|(name, _)| name == "Blizzard_UIPanels_Game")
        .expect("Mists game startup should include Blizzard_UIPanels_Game");

    assert!(
        money_frame_index < ui_panels_index,
        "Blizzard_MoneyFrame must load before Blizzard_UIPanels_Game so MoneyInputFrameTemplate exists before TradeFrame.xml instantiates TradePlayerInputMoneyFrame; indexes were MoneyFrame={money_frame_index}, UIPanels={ui_panels_index}"
    );
}

#[test]
fn mists_bootstrap_supplies_legacy_startup_api_shapes() {
    let env = WowLuaEnv::new().expect("Lua environment should initialize");

    let result: (
        i32,
        i32,
        String,
        i32,
        i32,
        bool,
        i32,
        i32,
        i32,
        i32,
        i32,
        String,
    ) = env
        .eval(
            r#"
            local skillName, _header, _isExpanded, skillRank, tempPoints, _modifier, skillMaxRank = GetSkillLineInfo(GetSelectedSkill())
            SetGuildRosterSelection(7)
            local hk, contribution = GetPVPThisWeekStats()
            return GetNumClasses(),
                GetSelectedSkill(),
                skillName,
                skillRank,
                tempPoints,
                HonorSystemEnabled(),
                hk,
                contribution,
                GetCurrencyListSize(),
                GetGuildRosterSelection(),
                skillMaxRank,
                type(C_ProductChoice.GetChoices())
            "#,
        )
        .expect("Mists legacy startup APIs should be callable");

    assert_eq!(
        result,
        (
            11,
            1,
            "Weapon Skills".to_string(),
            1,
            0,
            false,
            0,
            0,
            15,
            7,
            1,
            "table".to_string()
        ),
        "Mists startup helpers should return non-nil legacy API shapes"
    );
}

#[test]
fn mists_honor_frame_shared_reproduces_missing_honor_system_enabled() {
    let env = WowLuaEnv::new().expect("Lua environment should initialize");
    env.exec(
        r#"
        rawset(_G, "HonorSystemEnabled", nil)
        HonorFrame_GetCurrencyFrame = function()
            return {
                Hide = function() end,
                Show = function() end,
            }
        end
        "#,
    )
    .expect("install HonorFrame reproduction fixtures");
    env.exec(HONOR_FRAME_SHARED_LUA)
        .expect("HonorFrame_Shared.lua should define functions before OnLoad runs");

    let (ok, err): (bool, String) = env
        .eval(
            r#"
            local frame = { RegisterEvent = function() end }
            local ok, err = pcall(HonorFrame_OnLoad, frame)
            return ok, tostring(err)
            "#,
        )
        .expect("HonorFrame_OnLoad pcall should return a status");

    assert!(!ok, "HonorFrame_OnLoad should reproduce the nil global");
    assert!(
        err.contains("HonorSystemEnabled"),
        "expected HonorSystemEnabled nil failure, got: {err}"
    );
}

#[test]
fn mists_honor_system_enabled_matches_disabled_legacy_honor_surface() {
    let env = WowLuaEnv::new().expect("Lua environment should initialize");

    let (kind, enabled): (String, bool) = env
        .eval(
            r#"
            return type(HonorSystemEnabled()), HonorSystemEnabled()
            "#,
        )
        .expect("HonorSystemEnabled should be callable in Mists");

    assert_eq!(
        (kind, enabled),
        ("boolean".to_string(), false),
        "MoP Classic keeps HonorSystemEnabled as a global boolean gate; false hides the legacy HonorFrame honor currency surface"
    );
}

#[test]
fn mists_honor_pvp_api_contract_matches_classic_shapes() {
    let env = WowLuaEnv::new().expect("Lua environment should initialize");
    {
        let mut state = env.state().borrow_mut();
        state.player.honor_level = 5;
        state.pvp_honor.classic_honor_system_enabled = true;
        state.pvp_honor.yesterday_honorable_kills = 11;
        state.pvp_honor.yesterday_dishonorable_kills = 1;
        state.pvp_honor.this_week_honorable_kills = 22;
        state.pvp_honor.this_week_contribution = 330;
        state.pvp_honor.last_week_honorable_kills = 44;
        state.pvp_honor.last_week_dishonorable_kills = 2;
        state.pvp_honor.last_week_contribution = 550;
        state.pvp_honor.last_week_rank = 6;
        state.pvp_honor.lifetime_honorable_kills = 88;
        state.pvp_honor.lifetime_highest_rank = 9;
        state.pvp_honor.rank_progress = 0.25;
    }

    let shape: (String, bool, i32, i32, i32, i32) = env
        .eval(
            r##"
            return type(HonorSystemEnabled()),
                HonorSystemEnabled(),
                select("#", GetPVPYesterdayStats()),
                select("#", GetPVPThisWeekStats()),
                select("#", GetPVPLastWeekStats()),
                select("#", GetPVPLifetimeStats())
            "##,
        )
        .expect("Mists honor/PvP APIs should expose Classic call shapes");
    let values: (i32, i32, i32, i32, i32, i32, i32, i32, String, i32, f64) = env
        .eval(
            r##"
            local yesterdayHK, yesterdayDK = GetPVPYesterdayStats()
            local weekHK, weekContribution = GetPVPThisWeekStats()
            local lastWeekHK, lastWeekDK = GetPVPLastWeekStats()
            local lifetimeHK, _, highestRank = GetPVPLifetimeStats()
            local rankName, rankNumber = GetPVPRankInfo(UnitPVPRank("player"))
            return
                yesterdayHK,
                yesterdayDK,
                weekHK,
                weekContribution,
                lastWeekHK,
                lastWeekDK,
                lifetimeHK,
                highestRank,
                rankName,
                rankNumber,
                GetPVPRankProgress()
            "##,
        )
        .expect("Mists honor/PvP APIs should read simulator state");

    assert_eq!(
        shape,
        ("boolean".to_string(), true, 2, 2, 4, 3),
        "Mists honor/PvP startup APIs should keep the Classic return shapes HonorFrame_Shared.lua consumes"
    );
    assert_eq!(
        values,
        (11, 1, 22, 330, 44, 2, 88, 9, "Rank".to_string(), 5, 0.25),
        "Mists honor/PvP startup APIs should read Classic return shapes from simulator state"
    );
}

#[test]
fn mists_bootstrap_registers_startup_cvar_defaults() {
    let env = WowLuaEnv::new().expect("Lua environment should initialize");

    let result: (String, String, String, bool, bool) = env
        .eval(
            r#"
            return GetCVar("worldMapOpacity"),
                GetCVar("NamePlateHorizontalScale"),
                GetCVar("NamePlateVerticalScale"),
                GetCVarBool("ShowClassColorInFriendlyNameplate"),
                GetCVarBool("ColorNameplateNameBySelection")
            "#,
        )
        .expect("Mists startup CVars should be readable");

    assert_eq!(
        result,
        (
            "1".to_string(),
            "1".to_string(),
            "1".to_string(),
            true,
            false
        ),
        "Mists startup CVars should have concrete defaults"
    );
}

#[test]
fn mists_skill_line_zero_selection_still_returns_numeric_rank() {
    let env = WowLuaEnv::new().expect("Lua environment should initialize");

    let result: (i32, String, i32, i32, i32) = env
        .eval(
            r#"
            SetSelectedSkill(0)
            local skillName, _header, _isExpanded, skillRank, tempPoints, _modifier, skillMaxRank = GetSkillLineInfo(GetSelectedSkill())
            return GetSelectedSkill(), skillName, skillRank, tempPoints, skillMaxRank
            "#,
        )
        .expect("Mists skill selection zero should still have a startup row");

    assert_eq!(
        result,
        (0, "Weapon Skills".to_string(), 1, 0, 1),
        "Mists SkillFrame startup should not receive nil rank fields"
    );
}

#[test]
fn mists_selected_skill_api_matches_skill_frame_tuple_shape() {
    let env = WowLuaEnv::new().expect("Lua environment should initialize");

    let result: (i32, String, bool, bool, i32, i32, i32, i32, bool, bool) = env
        .eval(
            r#"
            SetSelectedSkill(1)
            local selected = GetSelectedSkill()
            local skillName, header, isExpanded, skillRank, tempPoints,
                skillModifier, skillMaxRank, isAbandonable = GetSkillLineInfo(selected)
            local missingSkill = GetSkillLineInfo(2) == nil
            return selected,
                skillName,
                header,
                isExpanded,
                skillRank,
                tempPoints,
                skillModifier,
                skillMaxRank,
                isAbandonable,
                missingSkill
            "#,
        )
        .expect("Mists selected skill API should return the tuple SkillFrame destructures");

    assert_eq!(
        result,
        (
            1,
            "Weapon Skills".to_string(),
            false,
            false,
            1,
            0,
            0,
            1,
            false,
            true
        ),
        "SkillFrame expects concrete rank numbers for selected rows and nil for out-of-range rows"
    );
}

#[test]
fn mists_bootstrap_supplies_pvp_currency_and_debugbar_shapes() {
    let env = WowLuaEnv::new().expect("Lua environment should initialize");

    let result: (String, i32, i32, i32, i32, i32) = env
        .eval(
            r#"
            local rankName, rankNumber = GetPVPRankInfo(0)
            local honor = C_CurrencyInfo.GetCurrencyInfo(Constants.CurrencyConsts.CLASSIC_HONOR_CURRENCY_ID)
            local lifetimeKills, lifetimeDishonorable, highestRank = GetPVPLifetimeStats()
            return rankName, rankNumber, honor.quantity, DebugBarManager:GetScaledInternalBarsHeight(),
                UnitPVPRank("player"), highestRank
            "#,
        )
        .expect("Mists PVP, currency, and debug bar startup APIs should be callable");

    assert_eq!(
        result,
        ("None".to_string(), 0, 0, 0, 0, 0),
        "Mists startup helpers should return concrete numeric fields"
    );
}

#[test]
fn mists_bootstrap_supplies_legacy_bank_and_aura_shapes() {
    let env = WowLuaEnv::new().expect("Lua environment should initialize");

    let result: (i32, i32, i32, i32, i32, i32) = env
        .eval(
            r#"
            local _name, _icon, _count, _debuffType, duration, expirationTime,
                _source, _stealable, _personal, _spellID, _unused1, _unused2, _unused3,
                _unused4, timeMod = UnitBuff("player", 1)
            return NUM_BANKGENERIC_SLOTS,
                NUM_BANKBAGSLOTS,
                Constants.InventoryConstants.NumGenericBankSlots,
                Constants.InventoryConstants.NumBankBagSlots,
                duration,
                timeMod
            "#,
        )
        .expect("Mists bank constants and legacy aura tuple should be callable");

    assert_eq!(
        result,
        (28, 7, 28, 7, 3600, 1),
        "Mists startup should see bank constants and the legacy UnitBuff tuple shape"
    );
}

#[test]
fn mists_bootstrap_supplies_settings_label_globals() {
    let env = WowLuaEnv::new().expect("Lua environment should initialize");

    let result: (String, String) = env
        .eval(
            r#"
            return SHOW_AGGRO_PERCENTAGES,
                SHOW_COMBAT_HEALING_TEXT
            "#,
        )
        .expect("Mists settings labels should be strings");

    assert_eq!(
        result,
        ("Show aggro percentages".to_string(), "Healing".to_string()),
        "Mists settings variables should not register nil display names"
    );
}

#[test]
fn mists_create_forbidden_frame_forwards_to_create_frame() {
    let env = WowLuaEnv::new().expect("Lua environment should initialize");

    let result: (String, String, bool) = env
        .eval(
            r#"
            local frame = CreateForbiddenFrame("Button", "MistsForbiddenProbe", UIParent, "UIPanelButtonTemplate")
            return frame:GetObjectType(), frame:GetName(), frame:IsForbidden()
            "#,
        )
        .expect("CreateForbiddenFrame should create a real forbidden frame");

    assert_eq!(
        result,
        (
            "Button".to_string(),
            "MistsForbiddenProbe".to_string(),
            true
        ),
        "CreateForbiddenFrame should preserve CreateFrame semantics and mark the frame forbidden"
    );
}
