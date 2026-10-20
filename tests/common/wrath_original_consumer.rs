//! Load whole original consumers without editing the cache.

use wow_ui_sim::lua_api::WowLuaEnv;

pub fn load_original_files(env: &WowLuaEnv, names: &[&str]) {
    let cache = dirs::cache_dir()
        .expect("cache directory")
        .join("wow-ui-sim/blizzard-ui/wrath/FrameXML");
    for name in names {
        assert!(
            cache.join(name).is_file(),
            "SETUP: original consumer {name} absent"
        );
    }
    // Loading deduplicates by directory, so the entire original file sequence
    // must share one TOC rather than repeatedly loading the same directory.
    let files = names.join("\n");
    let toc = wow_ui_sim::toc::TocFile::parse(&cache, &format!("## Interface: 30300\n{files}\n"));
    let loaded = wow_ui_sim::loader::load_addon_from_toc(&env.loader_env(), &toc)
        .expect("SETUP: load unchanged original consumers");
    assert!(loaded.warnings.is_empty(), "SETUP: {:?}", loaded.warnings);
}
