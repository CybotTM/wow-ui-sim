//! Full cached Lua files, supplied widgets, and explicit error capture only.
use sha2::{Digest, Sha256};
use wow_ui_sim::lua_api::WowLuaEnv;

pub fn create_env() -> WowLuaEnv {
    let env = WowLuaEnv::new().expect("create isolated Lua environment");
    env.exec(
        "BoundaryErrors = {}; seterrorhandler(function(e) \
         BoundaryErrors[#BoundaryErrors + 1] = tostring(e) end)",
    )
    .expect("install boundary error capture");
    env
}

pub fn assert_no_errors(env: &WowLuaEnv) {
    env.exec("assert(#BoundaryErrors == 0, table.concat(BoundaryErrors, '\\n'))")
        .expect("no captured Lua errors");
    assert!(
        env.state().borrow().lua_errors.is_empty(),
        "{:?}",
        env.state().borrow().lua_errors
    );
}

pub fn exec(env: &WowLuaEnv, code: &str) {
    let result = env.exec(code);
    assert_no_errors(env);
    result.expect("original callback and concrete behavioral assertions");
}

pub fn load_cached_lua(env: &WowLuaEnv, relative_path: &str, expected_sha256: &str) {
    let path = dirs::cache_dir()
        .expect("SETUP FAILURE: cache directory required")
        .join("wow-ui-sim/blizzard-ui")
        .join(relative_path);
    let bytes = std::fs::read(&path)
        .unwrap_or_else(|error| panic!("SETUP FAILURE: {}: {error}", path.display()));
    assert_eq!(
        format!("{:x}", Sha256::digest(&bytes)),
        expected_sha256,
        "SETUP FAILURE: cached source drift: {}",
        path.display()
    );
    let source = std::str::from_utf8(&bytes).expect("SETUP FAILURE: cached Lua UTF-8");
    let result = env.exec_named(source, &format!("@{}", path.display()));
    assert_no_errors(env);
    result.unwrap_or_else(|error| panic!("SETUP FAILURE: {}: {error}", path.display()));
}
