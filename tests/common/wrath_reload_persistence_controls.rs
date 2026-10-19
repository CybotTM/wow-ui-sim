//! Managed reload readback preserves scalar/large values without stale WTF reimport.
use wow_ui_sim::lua_api::WowLuaEnv;
use wow_ui_sim::saved_variables::{SavedVariablesManager, WtfConfig};

pub fn assert_reload_readback() {
    let fixture = tempfile::tempdir().unwrap();
    let addon = fixture.path().join("WrathPersistenceFixture");
    std::fs::create_dir(&addon).unwrap();
    let toc = addon.join("WrathPersistenceFixture.toc");
    std::fs::write(&toc, "## SavedVariables: ReloadScalar, ReloadLarge\n## SavedVariablesPerCharacter: ReloadCharacterScalar\nCode.lua\n").unwrap();
    std::fs::write(
        addon.join("Code.lua"),
        "ReloadScalar = 3; ReloadCharacterScalar = 4; ReloadLarge = { codeDefault = true }",
    )
    .unwrap();
    let config = WtfConfig::new(
        fixture.path().join("WTF"),
        "ReloadAccount",
        "ReloadRealm",
        "ReloadChar",
    );
    let account = config.account_saved_vars_file("WrathPersistenceFixture");
    let character = config.character_saved_vars_file("WrathPersistenceFixture");
    for (path, content) in [
        (
            &account,
            "ReloadScalar = -1; ReloadLarge = { stale = true }",
        ),
        (&character, "ReloadCharacterScalar = -2"),
    ] {
        std::fs::create_dir_all(path.parent().unwrap()).unwrap();
        std::fs::write(path, content).unwrap();
    }
    let before = [
        std::fs::read(&account).unwrap(),
        std::fs::read(&character).unwrap(),
    ];
    let mut manager = SavedVariablesManager::with_storage_dir(fixture.path().join("storage"));
    manager.set_wtf_config(config);
    manager.set_max_load_bytes(Some(128));
    let mut env = WowLuaEnv::new().unwrap();
    wow_ui_sim::loader::load_addon_with_saved_vars(&env.loader_env(), &toc, &mut manager).unwrap();
    env.set_reload_loader(move |env, manager| {
        wow_ui_sim::loader::load_addon_with_saved_vars(
            &env.loader_env(),
            &toc,
            manager.as_mut().unwrap(),
        )
        .map_err(|error| wow_ui_sim::Error::Other(error.to_string()))?;
        Ok(())
    });
    env.exec("ReloadScalar = 73; ReloadCharacterScalar = 92; ReloadLarge = {}; for i=1,256 do ReloadLarge[i] = i*2 end; OldPersistenceGlobal = {}; CreateFrame('Frame', 'OldPersistenceFrame'); C_Timer.After(0, function() OldPersistenceTimer = true end); ReloadUI()").unwrap();
    let mut manager = Some(manager);
    assert!(env.service_reload(&mut manager).unwrap());
    let local = fixture.path().join("storage/WrathPersistenceFixture.lua");
    assert!(
        std::fs::metadata(local).unwrap().len() > 128,
        "local readback exceeds the external import cap"
    );
    env.exec("assert(ReloadScalar == 73 and ReloadCharacterScalar == 92, 'scalar readback beats defaults and stale WTF'); assert(#ReloadLarge == 256 and ReloadLarge[256] == 512 and ReloadLarge.stale == nil, 'uncapped trusted local table readback'); assert(OldPersistenceGlobal == nil and OldPersistenceFrame == nil, 'fresh VM globals and frames')").unwrap();
    wow_ui_sim::startup::run_lua_error_update_ticks(&env);
    assert!(
        env.eval::<bool>("return OldPersistenceTimer == nil")
            .unwrap()
    );
    assert_eq!(
        std::fs::read(account).unwrap(),
        before[0],
        "account WTF is read-only"
    );
    assert_eq!(
        std::fs::read(character).unwrap(),
        before[1],
        "character WTF is read-only"
    );
}
