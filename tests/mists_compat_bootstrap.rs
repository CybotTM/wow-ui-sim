#![cfg(feature = "client-mists")]

use wow_ui_sim::lua_api::WowLuaEnv;
use wow_ui_sim::toc::TocFile;

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
fn mists_bootstrap_supplies_pvp_currency_and_debugbar_shapes() {
    let env = WowLuaEnv::new().expect("Lua environment should initialize");

    let result: (String, i32, i32, i32) = env
        .eval(
            r#"
            local rankName, rankNumber = GetPVPRankInfo(0)
            local honor = C_CurrencyInfo.GetCurrencyInfo(Constants.CurrencyConsts.CLASSIC_HONOR_CURRENCY_ID)
            return rankName, rankNumber, honor.quantity, DebugBarManager:GetScaledInternalBarsHeight()
            "#,
        )
        .expect("Mists PVP, currency, and debug bar startup APIs should be callable");

    assert_eq!(
        result,
        ("None".to_string(), 0, 0, 0),
        "Mists startup helpers should return concrete numeric fields"
    );
}
