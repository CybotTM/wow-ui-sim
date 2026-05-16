use wow_ui_sim::lua_api::{AddonInfo, WowLuaEnv};

fn env_with_loading_addon(addon_name: &str) -> WowLuaEnv {
    let env = WowLuaEnv::new().expect("Failed to create Lua environment");
    let mut state = env.state().borrow_mut();
    state.addons.push(AddonInfo {
        folder_name: addon_name.to_string(),
        title: addon_name.to_string(),
        enabled: true,
        loaded: true,
        ..Default::default()
    });
    let addon_index = state
        .addons
        .iter()
        .position(|addon| addon.folder_name == addon_name)
        .expect("loading addon should exist");
    state.loading_addon_index = Some(addon_index as u16);
    state.nil_symbol_accesses.clear();
    drop(state);
    env
}

#[test]
fn missing_global_lookup_is_logged_with_addon_name() {
    let env = env_with_loading_addon("TestAddon");

    env.exec("local _ = MissingGlobalSymbol")
        .expect("missing global lookup should not error");

    let records = env.state().borrow().nil_symbol_accesses.clone();
    assert_eq!(records.len(), 1, "expected exactly one nil global access");
    assert_eq!(records[0].addon_name.as_deref(), Some("TestAddon"));
    assert_eq!(records[0].container, "_G");
    assert_eq!(records[0].key, "MissingGlobalSymbol");
}

#[test]
fn repeated_missing_global_lookup_is_logged_once() {
    let env = env_with_loading_addon("TestAddon");

    env.exec(
        r#"
        local _ = MissingGlobalSymbol
        local _ = MissingGlobalSymbol
        "#,
    )
    .expect("missing global lookup should not error");

    let records = env.state().borrow().nil_symbol_accesses.clone();
    assert_eq!(records.len(), 1, "expected one nil global access record");
    assert_eq!(records[0].container, "_G");
    assert_eq!(records[0].key, "MissingGlobalSymbol");
}

#[test]
fn missing_c_namespace_lookup_is_logged_with_addon_name() {
    let env = env_with_loading_addon("TestAddon");

    env.exec("local _ = C_Container.MissingMethod")
        .expect("missing namespace lookup should not error");

    let records = env.state().borrow().nil_symbol_accesses.clone();
    assert_eq!(
        records.len(),
        1,
        "expected exactly one nil namespace access"
    );
    assert_eq!(records[0].addon_name.as_deref(), Some("TestAddon"));
    assert_eq!(records[0].container, "C_Container");
    assert_eq!(records[0].key, "MissingMethod");
}
