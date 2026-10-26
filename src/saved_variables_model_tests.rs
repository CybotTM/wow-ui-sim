use super::{SavedVariablesManager, get_global, set_global};
use rilua::{Lua, LuaApiMut, Val};

#[test]
fn private_model_round_trips_through_fresh_vm_and_manager() {
    let fixture = tempfile::tempdir().unwrap();
    let directory = fixture.path().join("models");
    let manager = SavedVariablesManager::with_storage_dir(directory.clone());
    let mut first = Lua::new().unwrap();
    assert!(matches!(
        manager
            .load_model(first.state_mut(), "TTSSettings")
            .unwrap(),
        Val::Nil
    ));
    first
        .exec(
            r#"
        fixture = {
            account = {rate = 4},
            character = {
                volume = 37, enabled = false,
                voices = {[0] = {name = 'fixture voice'}},
            },
            saved = true,
        }
    "#,
        )
        .unwrap();
    let value = get_global(first.state_mut(), "fixture");
    manager
        .save_model(first.state_mut(), "TTSSettings", value)
        .unwrap();
    drop(first);
    drop(manager);

    let manager = SavedVariablesManager::with_storage_dir(directory);
    let mut second = Lua::new().unwrap();
    let restored = manager
        .load_model(second.state_mut(), "TTSSettings")
        .unwrap();
    set_global(second.state_mut(), "restored", restored);
    second
        .exec(
            r#"
        assert(preferences == nil)
        assert(restored.account.rate == 4)
        assert(restored.character.volume == 37)
        assert(restored.character.enabled == false)
        assert(restored.character.voices[0].name == 'fixture voice')
        assert(restored.saved == true)
    "#,
        )
        .unwrap();
}

#[test]
fn private_model_rejects_executable_or_malformed_storage_without_publishing_globals() {
    let fixture = tempfile::tempdir().unwrap();
    let manager = SavedVariablesManager::with_storage_dir(fixture.path().to_path_buf());
    let mut lua = Lua::new().unwrap();
    for source in [
        "preferences = {saved = true}; injected = true",
        "preferences = {saved = true",
        "preferences = false",
    ] {
        std::fs::write(fixture.path().join("TTSSettings.lua"), source).unwrap();
        assert!(manager.load_model(lua.state_mut(), "TTSSettings").is_err());
        lua.exec("assert(preferences == nil and injected == nil)")
            .unwrap();
    }
}

#[test]
fn private_model_surfaces_read_and_write_failures() {
    let fixture = tempfile::tempdir().unwrap();
    let blocked = fixture.path().join("not-a-directory");
    std::fs::write(&blocked, "blocked").unwrap();
    let manager = SavedVariablesManager::with_storage_dir(blocked);
    let mut lua = Lua::new().unwrap();
    lua.exec("fixture = {saved = true}").unwrap();
    let value = get_global(lua.state_mut(), "fixture");
    assert!(
        manager
            .save_model(lua.state_mut(), "TTSSettings", value)
            .is_err()
    );
    assert!(manager.load_model(lua.state_mut(), "TTSSettings").is_err());
}
