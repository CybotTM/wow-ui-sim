//! Shared original-consumer provenance and retained-error assertions.
#![cfg(any(feature = "client-retail", feature = "client-ptr"))]

use wow_ui_sim::lua_api::WowLuaEnv;

pub fn assert_original_source(env: &WowLuaEnv, function: &str, suffix: &str) {
    env.exec(&format!(
        r#"
        local info = debug.getinfo({function}, 'S')
        local source = info.source:gsub('\\', '/')
        local suffix = '{suffix}'
        assert(info.what == 'Lua' and source:sub(-#suffix) == suffix,
            'must execute the profile-selected original consumer')
        print('stock residual consumer', source, info.linedefined, info.lastlinedefined)
        "#,
    ))
    .expect("original selected consumer provenance");
}

pub fn assert_no_consumer_errors(env: &WowLuaEnv, phase: &str, markers: &[&str]) {
    let state = env.state().borrow();
    for error in &state.lua_errors {
        eprintln!("stock residual {phase}: {error}");
    }
    let consumer_errors: Vec<_> = state
        .lua_errors
        .iter()
        .filter(|error| markers.iter().any(|marker| error.contains(marker)))
        .collect();
    assert!(
        consumer_errors.is_empty(),
        "{phase}: original consumer errors were retained: {consumer_errors:?}"
    );
}
