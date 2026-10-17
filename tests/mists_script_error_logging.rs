//! Original ScriptErrors consumer must log errors through the modeled console sink.
#![cfg(feature = "client-mists")]

use sha2::{Digest, Sha256};
use wow_ui_sim::lua_api::WowLuaEnv;

#[test]
fn mists_original_script_error_handler_logs_and_delivers_message() {
    let env = WowLuaEnv::new().unwrap();
    let path = wow_ui_sim::client_profile::blizzard_ui_addons_dir()
        .join("Blizzard_ScriptErrors/Blizzard_ScriptErrors.lua");
    let source = std::fs::read_to_string(&path).expect("SETUP: cached original ScriptErrors");
    assert_eq!(
        format!("{:x}", Sha256::digest(source.as_bytes())),
        "77f2c81015e77038851bbb35191255504b48b2cb70fe8e3aa9707a1cba6132a2",
        "SETUP: original ScriptErrors changed"
    );
    env.loader_env().exec(&source).unwrap();
    env.exec(
        r#"
        ObservedErrorMessage = nil
        AddLuaErrorHandler(function(message) ObservedErrorMessage = message end)
        geterrorhandler()("mists-log-fixture")
        assert(ObservedErrorMessage == "mists-log-fixture")
        "#,
    )
    .unwrap();
    let state = env.state();
    let state = state.borrow();
    let logged: Vec<_> = state
        .console_output
        .iter()
        .filter(|message| message.starts_with("Lua Error: mists-log-fixture\n"))
        .collect();
    assert_eq!(
        logged.len(),
        1,
        "original error handler must log exactly once"
    );
    assert!(state.lua_errors.is_empty(), "{:?}", state.lua_errors);
}
