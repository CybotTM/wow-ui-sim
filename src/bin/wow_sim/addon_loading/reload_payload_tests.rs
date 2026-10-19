//! Real-addon payloads reach the unchanged Retail Settings keybinding consumer.
use super::*;
use clap::Parser;

fn observe(env: &WowLuaEnv, target: &str, label: &str) {
    env.exec(&format!(
        r#"
        ReloadPayloadTarget = {target:?}
        ReloadPayloadCount = nil
        ReloadPayloadType = nil
        ReloadPayloadBindings = nil
        BINDING_HEADER_OTHER = {label:?}
        if not ReloadPayloadObserver then
            ReloadPayloadObserver = CreateFrame('Frame')
            ReloadPayloadObserver:RegisterEvent('ADDON_LOADED')
            ReloadPayloadObserver:SetScript('OnEvent', function(self, event, ...)
                local name, containsBindings = ...
                if name == ReloadPayloadTarget then
                    ReloadPayloadCount = select('#', ...)
                    ReloadPayloadType = type(containsBindings)
                    ReloadPayloadBindings = containsBindings
                end
            end)
        end
    "#
    ))
    .unwrap();
}

fn assert_payload_and_original_label(env: &WowLuaEnv, label: &str, contains_bindings: bool) {
    assert_eq!(env.eval::<i32>("return ReloadPayloadCount").unwrap(), 2);
    assert_eq!(
        env.eval::<String>("return ReloadPayloadType").unwrap(),
        "boolean"
    );
    assert_eq!(
        env.eval::<bool>("return ReloadPayloadBindings").unwrap(),
        contains_bindings
    );
    env.exec("SettingsPanel:Hide(); Settings.OpenToCategory(Settings.KEYBINDINGS_CATEGORY_ID)")
        .unwrap();
    let state = env.state().borrow();
    let has_label = state.widgets.iter_ids().any(|id| {
        state
            .widgets
            .get(id)
            .is_some_and(|frame| frame.text.as_deref() == Some(label))
    });
    assert_eq!(
        has_label, contains_bindings,
        "unchanged Settings consumer regenerates the displayed binding section only for binding-bearing addons"
    );
}

#[test]
fn reload_real_addon_payload_retains_original_settings_consumer() {
    let args = crate::Args::try_parse_from(["wow-sim", "--no-addons", "--no-saved-vars"]).unwrap();
    let (env, _fonts, manager) = crate::startup::init_and_load(&args, ScreenKind::Game).unwrap();
    assert!(manager.is_none());
    wow_ui_sim::startup::fire_startup_events(&env);
    env.exec("Settings.OpenToCategory(Settings.KEYBINDINGS_CATEGORY_ID)")
        .unwrap();

    observe(&env, "Blizzard_PingUI", "Reload startup binding section");
    fire_addon_loaded(&env, "Blizzard_PingUI");
    assert_payload_and_original_label(&env, "Reload startup binding section", true);

    observe(&env, "Blizzard_Colors", "Reload startup no-binding section");
    fire_addon_loaded(&env, "Blizzard_Colors");
    assert_payload_and_original_label(&env, "Reload startup no-binding section", false);

    let fixture = tempfile::tempdir().unwrap();
    for (name, with_bindings) in [("ReloadBindingsLoD", true), ("ReloadNoBindingsLoD", false)] {
        let dir = fixture.path().join(name);
        std::fs::create_dir(&dir).unwrap();
        std::fs::write(dir.join(format!("{name}.toc")), "## LoadOnDemand: 1\n").unwrap();
        if with_bindings {
            let root = wow_ui_sim::paths::default_blizzard_ui_addons_path().unwrap();
            std::fs::copy(
                root.join("Blizzard_PingUI/Bindings.xml"),
                dir.join("Bindings.xml"),
            )
            .unwrap();
        }
    }
    env.state()
        .borrow_mut()
        .addon_base_paths
        .push(fixture.path().to_path_buf());
    for (name, label, expected) in [
        ("ReloadBindingsLoD", "Reload LoD binding section", true),
        (
            "ReloadNoBindingsLoD",
            "Reload LoD no-binding section",
            false,
        ),
    ] {
        observe(&env, name, label);
        env.exec(&format!("assert(C_AddOns.LoadAddOn({name:?}))"))
            .unwrap();
        assert_payload_and_original_label(&env, label, expected);
    }
}
