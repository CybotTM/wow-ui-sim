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
