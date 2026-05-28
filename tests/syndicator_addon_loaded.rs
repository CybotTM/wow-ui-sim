use wow_ui_sim::lua_api::{AddonInfo, WowLuaEnv};

#[test]
fn syndicator_owned_handlers_receive_their_addon_loaded_event() {
    let env = WowLuaEnv::new().expect("env should initialize");
    env.register_addon(AddonInfo {
        folder_name: "Syndicator".to_string(),
        title: "Syndicator".to_string(),
        enabled: true,
        loaded: false,
        ..Default::default()
    });

    {
        let mut state = env.state().borrow_mut();
        let index = state
            .addons
            .iter()
            .position(|addon| addon.folder_name == "Syndicator")
            .expect("Syndicator should be registered");
        state.loading_addon_index = Some(index as u16);
    }

    env.eval::<()>(
        r#"
        local frame = CreateFrame("Frame")
        frame:RegisterEvent("ADDON_LOADED")
        frame:SetScript("OnEvent", function(_, _, addonName)
            if addonName == "Syndicator" then
                _G.__syndicator_addon_loaded_seen = true
            end
        end)
        "#,
    )
    .expect("Syndicator event listener should install");

    {
        let mut state = env.state().borrow_mut();
        state.loading_addon_index = None;
        state.addons[0].loaded = true;
    }

    env.fire_event_with_args("ADDON_LOADED", &[env.lua_string("Syndicator")])
        .expect("ADDON_LOADED should dispatch");

    let seen = env
        .eval::<bool>("return __syndicator_addon_loaded_seen == true")
        .expect("flag should be readable");
    assert!(
        seen,
        "Syndicator initializes config from its own ADDON_LOADED callback"
    );
}
