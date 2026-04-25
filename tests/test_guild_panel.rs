mod common;

use std::path::PathBuf;
use wow_ui_sim::loader::load_addon;
use wow_ui_sim::lua_api::WowLuaEnv;
use wow_ui_sim::lua_api::state::GuildMember;

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
            local closedText = (dropdown:GetText() or "") .. "/" .. (dropdown.Text and dropdown.Text:GetText() or "")

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
        assert_eq!(result, "Officer/Officer|Officer,Member|Officer,Member", "rank dropdown should show selected rank and visible assignable guild ranks: {result}");
    }
}

#[test]
fn communities_member_detail_rank_dropdown_shows_rank_rows() {
    test_timeout! {
        let env = setup_env();
        env.state().borrow_mut().world.guild_members = vec![
            GuildMember {
                name: "Uther".to_string(),
                rank_index: 1,
                online: true,
            },
            GuildMember {
                name: "Jaina".to_string(),
                rank_index: 2,
                online: true,
            },
        ];
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

            local frame = CommunitiesFrame and CommunitiesFrame.GuildMemberDetailFrame
            if frame == nil then
                return "missing_detail_frame"
            end

            local memberInfo = C_Club.GetMemberInfo("guild-0", 2)
            frame:DisplayMember("guild-0", memberInfo)
            frame:SetupRankDropdown()

            local dropdown = frame.RankDropdown
            if dropdown == nil then
                return "missing_rank_dropdown"
            end
            if not dropdown:IsShown() then
                return "rank_dropdown_hidden"
            end

            local desc = dropdown:GenerateMenu()
            local descLabels = {}
            if type(desc) == "table" and type(desc.__wow_elements) == "table" then
                for _, element in ipairs(desc.__wow_elements) do
                    if element.text ~= nil and element.text ~= "" then
                        table.insert(descLabels, element.text)
                    end
                end
            end

            dropdown:OpenMenu()
            local labels = {}
            for _, button in ipairs(dropdown.__wow_menu_buttons or {}) do
                if button ~= nil and button:IsVisible() then
                    local text = button:GetText()
                    if text ~= nil and text ~= "" then
                        table.insert(labels, text)
                    end
                end
            end

            if #labels == 0 then
                return "empty_rank_frames:desc=" .. table.concat(descLabels, ",")
            end

            local closedText = dropdown.Text and dropdown.Text:GetText() or dropdown:GetText()
            if closedText == nil or closedText == "" then
                return "empty_closed_text:frames=" .. table.concat(labels, ",")
            end
            return "ok:" .. closedText .. ":" .. table.concat(labels, ",")
        "#).unwrap();
        assert_eq!(
            result,
            "ok:Officer:Officer,Member",
            "member detail rank dropdown should show rank rows, not the guild selector: {result}"
        );
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
            local textWidth = dropdown.Text and dropdown.Text:GetWidth() or 0
            if textWidth <= 0 then
                return "zero_text_width:" .. tostring(textWidth)
            end
            return (dropdown:GetText() or "") .. "/" .. (dropdown.Text and dropdown.Text:GetText() or "") .. "/" .. tostring(textWidth) .. "|" .. first:GetText() .. "," .. second:GetText()
        "#).unwrap();
        assert!(result.starts_with("Officer/Officer/"), "guild control rank dropdown should show selected rank and visible rank rows: {result}");
        assert!(result.ends_with("|Officer,Member"), "guild control rank dropdown should materialize visible rank rows: {result}");
    }
}

#[test]
fn guild_control_tab_dropdown_shows_initial_selection() {
    test_timeout! {
        let env = setup_env();
        load_guild_control_ui(&env);
        let result: String = env.eval(r#"
            local dropdown = GuildControlUI and GuildControlUI.dropdown
            if dropdown == nil then
                return "missing_dropdown"
            end
            return (dropdown:GetText() or "") .. "/" .. (dropdown.Text and dropdown.Text:GetText() or "")
        "#).unwrap();
        assert_eq!(result, "Guild Ranks/Guild Ranks", "guild control tab dropdown should show its initial selected tab: {result}");
    }
}

#[test]
fn guild_control_rank_dropdown_falls_back_to_selected_rank_name() {
    test_timeout! {
        let env = setup_env();
        load_guild_control_ui(&env);
        let result: String = env.eval(r#"
            local dropdown = GuildControlUIRankSettingsFrame
                and GuildControlUIRankSettingsFrame.dropdown
            if dropdown == nil then
                return "missing_dropdown"
            end
            GuildControlSetRank(1)
            GuildControlUI.currentRank = 1
            dropdown:GenerateMenu()
            return (dropdown:GetText() or "") .. "/" .. (dropdown.Text and dropdown.Text:GetText() or "")
        "#).unwrap();
        assert_eq!(result, "Guild Leader/Guild Leader", "rank dropdown should show selected rank even when rank is not in assignable menu rows: {result}");
    }
}

#[test]
fn guild_dropdown_value_lists_are_not_empty() {
    test_timeout! {
        let env = setup_env();
        load_guild_control_ui(&env);
        let result: String = env.eval(r#"
            local function values(dropdown)
                if dropdown == nil then
                    return nil
                end
                local desc = dropdown:GenerateMenu()
                if type(desc) ~= "table" or type(desc.__wow_elements) ~= "table" then
                    return {}
                end
                local labels = {}
                for _, element in ipairs(desc.__wow_elements) do
                    if type(element) == "table" and element.text and element.text ~= "" then
                        table.insert(labels, element.text)
                    end
                end
                return labels
            end

            local checks = {
                { "guild_control_tabs", GuildControlUI and GuildControlUI.dropdown },
                { "rank_settings", GuildControlUIRankSettingsFrame and GuildControlUIRankSettingsFrame.dropdown },
                { "rank_bank", GuildControlUIRankBankFrame and GuildControlUIRankBankFrame.dropdown },
            }

            local failures = {}
            local summaries = {}
            for _, check in ipairs(checks) do
                local name, dropdown = check[1], check[2]
                local labels = values(dropdown)
                if labels == nil then
                    table.insert(failures, name .. ":missing_dropdown")
                elseif #labels == 0 then
                    table.insert(failures, name .. ":empty")
                else
                    table.insert(summaries, name .. "=" .. table.concat(labels, ","))
                end
            end

            if #failures > 0 then
                return "fail:" .. table.concat(failures, ";") .. "|values:" .. table.concat(summaries, ";")
            end
            return "ok:" .. table.concat(summaries, ";")
        "#).unwrap();
        assert!(
            result.starts_with("ok:"),
            "guild dropdown value lists must not be empty: {result}"
        );
    }
}

#[test]
fn guild_dropdown_materialized_frame_values_are_not_empty() {
    test_timeout! {
        let env = setup_env();
        load_guild_control_ui(&env);
        let result: String = env.eval(r#"
            local function frameValues(dropdown)
                if dropdown == nil or type(dropdown.GetName) ~= "function" then
                    return nil
                end
                local name = dropdown:GetName()
                if name == nil or name == "" then
                    return nil
                end
                dropdown:OpenMenu()
                local labels = {}
                for index = 1, 10 do
                    local button = _G[name .. "MenuButton" .. index]
                    if button ~= nil and button:IsVisible() then
                        local text = button:GetText()
                        if text ~= nil and text ~= "" then
                            table.insert(labels, text)
                        end
                    end
                end
                return labels
            end

            local checks = {
                { "guild_control_tabs", GuildControlUI and GuildControlUI.dropdown },
                { "rank_settings", GuildControlUIRankSettingsFrame and GuildControlUIRankSettingsFrame.dropdown },
                { "rank_bank", GuildControlUIRankBankFrame and GuildControlUIRankBankFrame.dropdown },
            }

            local failures = {}
            local summaries = {}
            for _, check in ipairs(checks) do
                local name, dropdown = check[1], check[2]
                local labels = frameValues(dropdown)
                if labels == nil then
                    table.insert(failures, name .. ":missing_dropdown")
                elseif #labels == 0 then
                    table.insert(failures, name .. ":empty_frames")
                else
                    table.insert(summaries, name .. "=" .. table.concat(labels, ","))
                end
            end

            if #failures > 0 then
                return "fail:" .. table.concat(failures, ";") .. "|frames:" .. table.concat(summaries, ";")
            end
            return "ok:" .. table.concat(summaries, ";")
        "#).unwrap();
        assert!(
            result.starts_with("ok:"),
            "guild dropdown materialized frame values must not be empty: {result}"
        );
    }
}

#[test]
fn communities_list_dropdown_frame_values_are_not_empty() {
    test_timeout! {
        let env = setup_env();
        let result: String = env.eval(r#"
            local dropdown = CommunitiesFrame and CommunitiesFrame.CommunitiesListDropdown
            if dropdown == nil then
                return "missing_dropdown"
            end

            local function frameValues()
                dropdown:OpenMenu()
                local labels = {}
                for _, button in ipairs(dropdown.__wow_menu_buttons or {}) do
                    if button ~= nil and button:IsVisible() then
                        local text = button:GetText()
                        if text ~= nil and text ~= "" then
                            table.insert(labels, text)
                        end
                    end
                end
                return labels
            end

            local labels = frameValues()
            if #labels == 0 then
                local clubCount = 0
                local clubs = C_Club.GetSubscribedClubs()
                if type(clubs) == "table" then
                    clubCount = #clubs
                end
                return "empty_frames:clubs=" .. tostring(clubCount)
            end

            local closedText = dropdown.Text and dropdown.Text:GetText() or dropdown:GetText()
            if closedText == nil or closedText == "" then
                return "empty_closed_text:frames=" .. table.concat(labels, ",")
            end
            return "ok:" .. closedText .. ":" .. table.concat(labels, ",")
        "#).unwrap();
        assert!(
            result.starts_with("ok:"),
            "CommunitiesFrame.CommunitiesListDropdown frame values must not be empty: {result}"
        );
    }
}
