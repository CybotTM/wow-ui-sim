use std::fs;
use wow_ui_sim::lua_api::WowLuaEnv;

const LIBSTUB_PATH: &str = "/home/osso/Projects/wow/Rarity/Libs/LibStub/LibStub.lua";
const CALLBACK_PATH: &str =
    "/home/osso/Projects/wow/Rarity/Libs/CallbackHandler-1.0/CallbackHandler-1.0.lua";

fn load_libstub(env: &WowLuaEnv) {
    let code = fs::read_to_string(LIBSTUB_PATH)
        .expect("LibStub.lua not found — skipping (local-only test)");
    env.exec(&code).unwrap();
}

/// Return true if the required Lua files exist on disk (local dev only).
fn has_test_files() -> bool {
    std::path::Path::new(LIBSTUB_PATH).exists()
}

#[test]
fn test_libstub_loads() {
    if !has_test_files() {
        return;
    }
    let env = WowLuaEnv::new().unwrap();
    load_libstub(&env);

    let exists: bool = env.eval("return LibStub ~= nil").unwrap();
    assert!(exists, "LibStub should exist");
}

#[test]
fn test_libstub_new_library() {
    if !has_test_files() {
        return;
    }
    let env = WowLuaEnv::new().unwrap();
    load_libstub(&env);

    env.exec(
        r#"
        local lib = LibStub:NewLibrary("TestLib", 1)
        lib.version = 1
    "#,
    )
    .expect("NewLibrary should work");

    let version: i32 = env
        .eval(
            r#"
        local lib = LibStub:GetLibrary("TestLib")
        return lib.version
    "#,
        )
        .unwrap();

    assert_eq!(version, 1);
}

#[test]
fn test_callbackhandler_loads() {
    if !has_test_files() {
        return;
    }
    let env = WowLuaEnv::new().unwrap();
    load_libstub(&env);

    let code = fs::read_to_string(CALLBACK_PATH).unwrap();
    env.exec(&code).expect("CallbackHandler should load");

    let exists: bool = env
        .eval(r#"return LibStub:GetLibrary("CallbackHandler-1.0") ~= nil"#)
        .unwrap();
    assert!(exists, "CallbackHandler should be registered");
}

#[test]
fn test_callbackhandler_works() {
    if !has_test_files() {
        return;
    }
    let env = WowLuaEnv::new().unwrap();
    load_libstub(&env);

    let code = fs::read_to_string(CALLBACK_PATH).unwrap();
    env.exec(&code).unwrap();

    // Create an object with callbacks
    env.exec(
        r#"
        local CBH = LibStub("CallbackHandler-1.0")
        TestObject = {}
        TestObject.callbacks = CBH:New(TestObject)

        -- Track if callback was called
        _G.callbackCalled = false

        -- Register a callback
        TestObject:RegisterCallback("OnTest", function()
            _G.callbackCalled = true
        end)

        -- Fire the callback
        TestObject.callbacks:Fire("OnTest")
    "#,
    )
    .expect("CallbackHandler usage should work");

    let called: bool = env.eval("return _G.callbackCalled").unwrap();
    assert!(called, "Callback should have been called");
}
