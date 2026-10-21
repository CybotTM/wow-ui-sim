//! Active-cache original consumer and generated API documentation fixtures.
#![cfg(not(feature = "client-wrath"))]

use wow_ui_sim::lua_api::WowLuaEnv;

pub(crate) fn load_original(env: &WowLuaEnv, file: &str) {
    let path = wow_ui_sim::client_profile::blizzard_ui_addons_dir().join(file);
    let source = std::fs::read_to_string(path).expect("SETUP: active profile original source");
    env.loader_env().exec(&source).unwrap();
}

pub(crate) fn documented_env(file: &str, member: &str) -> WowLuaEnv {
    let env = WowLuaEnv::new().unwrap();
    env.exec("APIDocumentation = { AddDocumentationTable = function(_, t) FactoryDocumentation = t end }")
        .unwrap();
    load_original(&env, &format!("Blizzard_APIDocumentationGenerated/{file}"));
    env.exec(&format!(
        "local found = false; for _, f in ipairs(FactoryDocumentation.Functions) do \
         if f.Name == {member:?} then found = true; break end end; \
         assert(found, 'SETUP: documented method')"
    ))
    .unwrap();
    env
}
