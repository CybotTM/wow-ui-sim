//! Load whole original consumers without editing the cache.

use wow_ui_sim::lua_api::WowLuaEnv;

pub fn load_original(env: &WowLuaEnv, name: &str) {
    let cache = dirs::cache_dir()
        .expect("cache directory")
        .join("wow-ui-sim/blizzard-ui/wrath/FrameXML");
    assert!(
        cache.join(name).is_file(),
        "SETUP: original consumer {name} absent"
    );
    let mut toc =
        wow_ui_sim::toc::TocFile::parse(&cache, &format!("## Interface: 30300\n{name}\n"));
    toc.name = format!("Blizzard_LabelFixture_{name}");
    let loaded = wow_ui_sim::loader::load_addon_from_toc(&env.loader_env(), &toc)
        .expect("SETUP: load unchanged original consumer");
    assert!(loaded.warnings.is_empty(), "SETUP: {:?}", loaded.warnings);
}
