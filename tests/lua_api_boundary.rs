#[test]
fn lua_api_globals_temporary_shims_module_is_removed() {
    assert!(
        !std::path::Path::new("src/lua_api/globals/temporary_shims/mod.rs").exists(),
        "temporary Lua compatibility defaults should register directly from missing_surface or lua_api::workarounds, not through globals::temporary_shims"
    );
}

#[test]
fn modeled_c_lfg_info_lives_in_c_api() {
    assert!(
        !std::path::Path::new("src/lua_api/globals/lfg_info.rs").exists(),
        "modeled C_LFGInfo state belongs under src/c_api, not lua_api::globals"
    );
    assert!(
        !std::path::Path::new("src/lua_api/globals/missing_surface/lfg_info.rs").exists(),
        "modeled C_LFGInfo state belongs under src/c_api, not lua_api::globals::missing_surface"
    );
    assert!(
        std::path::Path::new("src/c_api/c_lfg_info.rs").exists(),
        "C_LFGInfo should have an explicit C API owner"
    );
}

#[test]
fn modeled_c_death_recap_lives_in_c_api() {
    assert!(
        !std::path::Path::new("src/lua_api/globals/missing_surface/death_recap.rs").exists(),
        "modeled C_DeathRecap state belongs under src/c_api, not lua_api::globals::missing_surface"
    );
    assert!(
        std::path::Path::new("src/c_api/c_death_recap.rs").exists(),
        "C_DeathRecap should have an explicit C API owner"
    );
}

#[test]
fn modeled_c_social_lives_in_c_api() {
    assert!(
        !std::path::Path::new("src/lua_api/globals/missing_surface/social.rs").exists(),
        "modeled C_Social state belongs under src/c_api, not lua_api::globals::missing_surface"
    );
    assert!(
        std::path::Path::new("src/c_api/c_social.rs").exists(),
        "C_Social should have an explicit C API owner"
    );
}
