//! CF-2 Forever original consumer regression. Authored only; RED awaits a slot.
#![cfg(any(feature = "client-wowforever",))]

use wow_ui_sim::lua_api::WowLuaEnv;

fn load_original_guild_control() -> WowLuaEnv {
    let ui = crate::common::blizzard_addon_harness::blizzard_ui_dir();
    let env = crate::common::blizzard_addon_harness::new_blizzard_addon_env(&ui);
    {
        let mut sim = env.state().borrow_mut();
        sim.world.guild_name = Some("CF2 Guild".into());
        sim.world
            .guild_members
            .first_mut()
            .expect("seeded roster contains the player")
            .rank_index = 1;
        assert!(sim.cvars.set("discordClientEnabled", "0"));
    }
    // GuildControlUI declares no dependencies. These are its original menu,
    // popup and frame-template prerequisites, not replacement consumer code.
    let roots = [
        "Blizzard_Menu",
        "Blizzard_StaticPopup",
        "Blizzard_UIPanels_Game",
        "Blizzard_GuildControlUI",
    ];
    let loaded = crate::common::blizzard_addon_harness::load_blizzard_addon_closure_into_env(
        &env,
        &ui,
        &roots,
        &[],
    );
    for root in roots {
        assert!(loaded.iter().any(|name| name == root), "missing {root}");
    }
    eprintln!("CF-2 Forever original cache: {}", ui.display());
    env
}

fn install_original_guild_menu_generator(env: &WowLuaEnv) {
    env.exec(
        r#"
        local info = debug.getinfo(GuildControlUI_SetupSelected, 'S')
        local suffix = 'Blizzard_GuildControlUI/Blizzard_GuildControlUI.lua'
        local source = info.source:gsub('\\', '/')
        assert(info.what == 'Lua' and source:sub(-#suffix) == suffix,
            'must execute original cached GuildControlUI setup')
        print('CF-2 consumer provenance', source, info.linedefined, info.lastlinedefined)
        assert(IsGuildLeader(), 'normal roster seed must select leader branch')
        -- Hidden dropdown defers secure menu generation. Invoke its unchanged
        -- generator directly below so framework error handling cannot hide RED.
        GuildControlUI.dropdown:Hide()
        GuildControlUI_SetupSelected(1)
        local generatorInfo = debug.getinfo(GuildControlUI.dropdown.menuGenerator, 'S')
        local generatorSource = generatorInfo.source:gsub('\\', '/')
        assert(generatorInfo.what == 'Lua' and generatorSource:sub(-#suffix) == suffix,
            'menu generator must be the original Blizzard closure')
        print('CF-2 generator provenance', generatorSource,
            generatorInfo.linedefined, generatorInfo.lastlinedefined)
        "#,
    )
    .expect("original guild-control setup and original generator provenance");
}

fn assert_original_discord_radio(env: &WowLuaEnv, enabled: bool) {
    let cvar = if enabled { "1" } else { "0" };
    assert!(env.state().borrow().cvars.set("discordClientEnabled", cvar));
    let errors_before = env.state().borrow().lua_errors.len();
    let (discord_count, rank_count, selected_tab): (i32, i32, i32) = env
        .eval(
            r#"
            local dropdown = GuildControlUI.dropdown
            local root = dropdown:CreateDefaultRootMenuDescription()
            dropdown.menuGenerator(dropdown, root)
            assert(root:GetTag() == 'MENU_GUILD_PERMISSIONS')
            local discordCount, rankCount = 0, 0
            for _, description in root:EnumerateElementDescriptions() do
                if description:GetData() == 4 then
                    assert(description:IsRadio(), 'Discord option must be selectable radio')
                    assert(not description:IsSelected(), 'rank tab remains selected')
                    discordCount = discordCount + 1
                elseif description:GetData() == 1 then
                    assert(description:IsRadio() and description:IsSelected(),
                        'guild-ranks option must remain selected')
                    rankCount = rankCount + 1
                end
            end
            return discordCount, rankCount, GuildControlUI.selectedTab
            "#,
        )
        .expect("original GuildControlUI.lua:70 must consume live Discord enablement");
    assert_eq!(discord_count, i32::from(enabled), "Discord radio presence");
    assert_eq!(
        rank_count, 1,
        "unrelated selected guild-ranks radio survives"
    );
    assert_eq!(
        selected_tab, 1,
        "menu generation must not change selected tab"
    );
    assert_eq!(env.state().borrow().lua_errors.len(), errors_before);
}

#[test]
fn original_guild_control_discord_radio_tracks_live_enablement() {
    let env = load_original_guild_control();
    install_original_guild_menu_generator(&env);
    assert_original_discord_radio(&env, false);
    assert_original_discord_radio(&env, true);
    assert_original_discord_radio(&env, false);
}
