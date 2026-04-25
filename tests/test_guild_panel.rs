mod common;

use std::path::PathBuf;
use wow_ui_sim::loader::load_addon;
use wow_ui_sim::lua_api::WowLuaEnv;

fn blizzard_ui_dir() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("Interface/BlizzardUI")
}

/// Blizzard addons needed for the guild/communities panel.
const GUILD_ADDONS: &[(&str, &str)] = &[
    ("Blizzard_SharedXMLBase", "Blizzard_SharedXMLBase.toc"),
    ("Blizzard_Colors", "Blizzard_Colors_Mainline.toc"),
    ("Blizzard_SharedXML", "Blizzard_SharedXML_Mainline.toc"),
    (
        "Blizzard_SharedXMLGame",
        "Blizzard_SharedXMLGame_Mainline.toc",
    ),
    (
        "Blizzard_UIPanelTemplates",
        "Blizzard_UIPanelTemplates_Mainline.toc",
    ),
    (
        "Blizzard_FrameXMLBase",
        "Blizzard_FrameXMLBase_Mainline.toc",
    ),
    ("Blizzard_FrameEffects", "Blizzard_FrameEffects.toc"),
    ("Blizzard_LoadLocale", "Blizzard_LoadLocale.toc"),
    ("Blizzard_Fonts_Shared", "Blizzard_Fonts_Shared.toc"),
    ("Blizzard_HelpPlate", "Blizzard_HelpPlate.toc"),
    (
        "Blizzard_AccessibilityTemplates",
        "Blizzard_AccessibilityTemplates.toc",
    ),
    ("Blizzard_ObjectAPI", "Blizzard_ObjectAPI_Mainline.toc"),
    ("Blizzard_UIParent", "Blizzard_UIParent_Mainline.toc"),
    ("Blizzard_TextStatusBar", "Blizzard_TextStatusBar.toc"),
    ("Blizzard_MoneyFrame", "Blizzard_MoneyFrame_Mainline.toc"),
    ("Blizzard_POIButton", "Blizzard_POIButton.toc"),
    ("Blizzard_Flyout", "Blizzard_Flyout.toc"),
    ("Blizzard_StoreUI", "Blizzard_StoreUI_Mainline.toc"),
    ("Blizzard_MicroMenu", "Blizzard_MicroMenu_Mainline.toc"),
    ("Blizzard_EditMode", "Blizzard_EditMode.toc"),
    ("Blizzard_GarrisonBase", "Blizzard_GarrisonBase.toc"),
    ("Blizzard_GameTooltip", "Blizzard_GameTooltip_Mainline.toc"),
    (
        "Blizzard_UIParentPanelManager",
        "Blizzard_UIParentPanelManager_Mainline.toc",
    ),
    (
        "Blizzard_Settings_Shared",
        "Blizzard_Settings_Shared_Mainline.toc",
    ),
    (
        "Blizzard_SettingsDefinitions_Shared",
        "Blizzard_SettingsDefinitions_Shared.toc",
    ),
    (
        "Blizzard_SettingsDefinitions_Frame",
        "Blizzard_SettingsDefinitions_Frame_Mainline.toc",
    ),
    (
        "Blizzard_FrameXMLUtil",
        "Blizzard_FrameXMLUtil_Mainline.toc",
    ),
    ("Blizzard_ItemButton", "Blizzard_ItemButton_Mainline.toc"),
    ("Blizzard_QuickKeybind", "Blizzard_QuickKeybind.toc"),
    ("Blizzard_FrameXML", "Blizzard_FrameXML_Mainline.toc"),
    (
        "Blizzard_UIPanels_Game",
        "Blizzard_UIPanels_Game_Mainline.toc",
    ),
    ("Blizzard_Communities", "Blizzard_Communities_Mainline.toc"),
];

fn setup_env() -> WowLuaEnv {
    let env = WowLuaEnv::new().expect("Failed to create Lua environment");
    env.set_screen_size(1024.0, 768.0);

    {
        let mut state = env.state().borrow_mut();
        state.addon_base_paths = vec![blizzard_ui_dir()];
    }

    let ui = blizzard_ui_dir();
    for (name, toc) in GUILD_ADDONS {
        let toc_path = ui.join(name).join(toc);
        if !toc_path.exists() {
            continue;
        }
        if let Err(e) = load_addon(&env.loader_env(), &toc_path) {
            eprintln!("[load {name}] FAILED: {e}");
        }
    }

    env.apply_post_load_workarounds();
    fire_startup_events(&env);
    env
}

fn load_guild_control_ui(env: &WowLuaEnv) {
    let toc_path = blizzard_ui_dir()
        .join("Blizzard_GuildControlUI")
        .join("Blizzard_GuildControlUI.toc");
    load_addon(&env.loader_env(), &toc_path).expect("Blizzard_GuildControlUI should load");
}

fn fire_startup_events(env: &WowLuaEnv) {
    common::fire_addon_loaded(env, "WoWUISim");
    for event in ["VARIABLES_LOADED", "PLAYER_LOGIN"] {
        let _ = env.fire_event(event);
    }
    common::fire_player_entering_world(env, true, false);
    for event in [
        "UPDATE_BINDINGS",
        "DISPLAY_SIZE_CHANGED",
        "UI_SCALE_CHANGED",
    ] {
        let _ = env.fire_event(event);
    }
}

#[test]
fn guild_panel_opens_without_unavailable_error() {
    test_timeout! {
        let env = setup_env();
        let result: String = env.eval(r#"
            -- Capture error messages
            local errors = {}
            local origAddMessage = UIErrorsFrame.AddMessage
            UIErrorsFrame.AddMessage = function(self, msg, ...)
                table.insert(errors, msg)
                if origAddMessage then pcall(origAddMessage, self, msg, ...) end
            end

            -- Try to open guild frame via ToggleGuildFrame
            local ok, err = pcall(ToggleGuildFrame)
            if not ok then return "error: " .. tostring(err) end

            -- Check for the unavailable message
            for _, msg in ipairs(errors) do
                if msg and msg:find("unavailable") then
                    return "unavailable_error: " .. msg
                end
            end
            return "ok"
        "#).unwrap();
        assert_eq!(result, "ok", "Guild panel should open without 'unavailable' error: {result}");
    }
}

#[test]
fn bn_connected_returns_true() {
    let env = WowLuaEnv::new().unwrap();
    let connected: bool = env.eval("return BNConnected()").unwrap();
    assert!(
        connected,
        "BNConnected should return true for Communities to work"
    );
}

#[test]
fn c_club_is_enabled() {
    let env = WowLuaEnv::new().unwrap();
    let enabled: bool = env.eval("return C_Club.IsEnabled()").unwrap();
    assert!(enabled, "C_Club.IsEnabled should return true");
}

#[test]
fn c_club_returns_guild() {
    let env = WowLuaEnv::new().unwrap();
    let result: String = env
        .eval(
            r#"
        local clubs = C_Club.GetSubscribedClubs()
        if #clubs == 0 then return "no_clubs" end
        local club = clubs[1]
        if club.clubType ~= 2 then return "type=" .. tostring(club.clubType) end
        if club.name ~= "Heroes of Azeroth" then return "name=" .. tostring(club.name) end
        return "ok"
    "#,
        )
        .unwrap();
    assert_eq!(result, "ok", "C_Club should return guild: {result}");
}

#[test]
fn guild_member_rank_dropdown_generates_rank_options() {
    test_timeout! {
        let env = setup_env();
        let result: String = env.eval(r#"
            A_Admin.SetGuildRanks({
                { name = "Guild Leader", flags = {} },
                { name = "Officer", flags = {} },
                { name = "Member", flags = {} },
            })

            function CanGuildPromote() return true end
            function CanGuildDemote() return true end
            C_GuildInfo.IsGuildRankAssignmentAllowed = function() return true end
            C_GuildInfo.SetGuildRankOrder = function() end

            local dropdown = CreateFrame("DropdownButton", "GuildRankDropdownProbe", UIParent)
            Mixin(dropdown, DropdownButtonMixin)

            local detail = {
                RankDropdown = dropdown,
                GetClubId = function() return "guild-0" end,
                GetMemberInfo = function()
                    return { guid = "member-2", guildRankOrder = 2 }
                end,
            }
            setmetatable(detail, { __index = CommunitiesGuildMemberDetailMixin })

            detail:SetupRankDropdown()
            local desc = dropdown:GenerateMenu()
            if type(desc) ~= "table" or type(desc.__wow_elements) ~= "table" then
                return "missing_elements"
            end
            local closedText = dropdown:GetText() or ""

            local labels = {}
            for _, element in ipairs(desc.__wow_elements) do
                table.insert(labels, element.text or "")
            end
            local descriptorLabels = table.concat(labels, ",")

            dropdown:OpenMenu()
            local first = GuildRankDropdownProbeMenuButton1
            local second = GuildRankDropdownProbeMenuButton2
            if first == nil or second == nil then
                return descriptorLabels .. "|missing_buttons"
            end
            return closedText .. "|" .. descriptorLabels .. "|" .. first:GetText() .. "," .. second:GetText()
        "#).unwrap();
        assert_eq!(result, "Officer|Officer,Member|Officer,Member", "rank dropdown should show selected rank and visible assignable guild ranks: {result}");
    }
}

#[test]
fn guild_control_rank_settings_dropdown_shows_rank_rows() {
    test_timeout! {
        let env = setup_env();
        load_guild_control_ui(&env);
        let result: String = env.eval(r#"
            A_Admin.SetGuildRanks({
                { name = "Guild Leader", flags = {} },
                { name = "Officer", flags = {} },
                { name = "Member", flags = {} },
            })

            local dropdown = GuildControlUIRankSettingsFrame
                and GuildControlUIRankSettingsFrame.dropdown
            if dropdown == nil then
                return "missing_dropdown"
            end

            dropdown:OpenMenu()
            local first = GuildControlUIRankSettingsFrameRankDropdownMenuButton1
            local second = GuildControlUIRankSettingsFrameRankDropdownMenuButton2
            if first == nil or second == nil then
                return "missing_buttons"
            end
            return (dropdown:GetText() or "") .. "|" .. first:GetText() .. "," .. second:GetText()
        "#).unwrap();
        assert_eq!(result, "Officer|Officer,Member", "guild control rank dropdown should show selected rank and visible rank rows: {result}");
    }
}
