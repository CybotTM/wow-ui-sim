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

#[test]
fn modeled_c_chat_bubbles_lives_in_c_api() {
    assert!(
        !std::path::Path::new("src/lua_api/globals/missing_surface/chat_bubbles.rs").exists(),
        "modeled C_ChatBubbles state belongs under src/c_api, not lua_api::globals::missing_surface"
    );
    assert!(
        std::path::Path::new("src/c_api/c_chat_bubbles.rs").exists(),
        "C_ChatBubbles should have an explicit C API owner"
    );
}

#[test]
fn modeled_c_party_info_lives_in_c_api() {
    assert!(
        !std::path::Path::new("src/lua_api/globals/missing_surface/party_info.rs").exists(),
        "modeled C_PartyInfo state belongs under src/c_api, not lua_api::globals::missing_surface"
    );
    assert!(
        std::path::Path::new("src/c_api/c_party_info.rs").exists(),
        "C_PartyInfo should have an explicit C API owner"
    );
}

#[test]
fn modeled_c_character_services_lives_in_c_api() {
    assert!(
        !std::path::Path::new("src/lua_api/globals/missing_surface/character_services.rs")
            .exists(),
        "modeled C_CharacterServices state belongs under src/c_api, not lua_api::globals::missing_surface"
    );
    assert!(
        std::path::Path::new("src/c_api/c_character_services.rs").exists(),
        "C_CharacterServices should have an explicit C API owner"
    );
}

#[test]
fn modeled_c_report_system_lives_in_c_api() {
    assert!(
        !std::path::Path::new("src/lua_api/globals/missing_surface/report_system.rs").exists(),
        "modeled C_ReportSystem state belongs under src/c_api, not lua_api::globals::missing_surface"
    );
    assert!(
        std::path::Path::new("src/c_api/c_report_system.rs").exists(),
        "C_ReportSystem should have an explicit C API owner"
    );
}

#[test]
fn modeled_c_summon_info_lives_in_c_api() {
    assert!(
        !std::path::Path::new("src/lua_api/globals/missing_surface/summon_info.rs").exists(),
        "modeled C_SummonInfo state belongs under src/c_api, not lua_api::globals::missing_surface"
    );
    assert!(
        std::path::Path::new("src/c_api/c_summon_info.rs").exists(),
        "C_SummonInfo should have an explicit C API owner"
    );
}

#[test]
fn modeled_c_stable_info_lives_in_c_api() {
    let small_namespaces =
        std::fs::read_to_string("src/lua_api/globals/missing_surface/small_namespaces.rs")
            .expect("small namespace surface should read");
    assert!(
        !small_namespaces.contains("C_StableInfo"),
        "modeled C_StableInfo state belongs under src/c_api, not lua_api::globals::missing_surface"
    );
    assert!(
        std::path::Path::new("src/c_api/c_stable_info.rs").exists(),
        "C_StableInfo should have an explicit C API owner"
    );
}
