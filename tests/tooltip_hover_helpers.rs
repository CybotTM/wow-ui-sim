use std::path::{Path, PathBuf};

use wow_ui_sim::loader::{find_toc_file, load_addon};
use wow_ui_sim::lua_api::WowLuaEnv;

const TOOLTIP_TEST_ADDONS: &[(&str, &str)] = &[
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
    ("Blizzard_Minimap", "Blizzard_Minimap_Mainline.toc"),
    ("Blizzard_BuffFrame", "Blizzard_BuffFrame.toc"),
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
    ("Blizzard_TokenUI", "Blizzard_TokenUI.toc"),
    ("Blizzard_ActionBar", "Blizzard_ActionBar_Mainline.toc"),
];

const HOVER_FIRST_VISIBLE_BUFF_ICON_LUA: &str = r#"
    local totalButtons = 0
    local shownButtons = 0
    local buttonsWithInfo = 0
    local buttonsWithIndex = 0
    for _, button in ipairs(BuffFrame.auraFrames) do
        totalButtons = totalButtons + 1
        if button.buttonInfo then
            buttonsWithInfo = buttonsWithInfo + 1
        end
        if button.buttonInfo and button.buttonInfo.index then
            buttonsWithIndex = buttonsWithIndex + 1
        end
        if button:IsShown() and button.buttonInfo and button.buttonInfo.index then
            button:OnEnter()
            return
        end
        if button:IsShown() then
            shownButtons = shownButtons + 1
        end
    end
    error(string.format(
        "No visible buff icon with tooltip data (BuffFrameShown=%s auraInfo=%s totalButtons=%d shownButtons=%d buttonsWithInfo=%d buttonsWithIndex=%d isExpanded=%s collapseEnabled=%s consolidateEnabled=%s)",
        tostring(BuffFrame:IsShown()),
        tostring(BuffFrame.auraInfo and #BuffFrame.auraInfo or nil),
        totalButtons,
        shownButtons,
        buttonsWithInfo,
        buttonsWithIndex,
        tostring(BuffFrame:IsExpanded()),
        tostring(BuffFrame.CollapseAndExpandButton and BuffFrame.CollapseAndExpandButton:IsEnabled()),
        tostring(BuffFrame.ConsolidatedBuffs and BuffFrame.ConsolidatedBuffs:IsEnabled())
    ))
"#;

pub fn setup_full_env() -> WowLuaEnv {
    let env = WowLuaEnv::new().unwrap();
    env.set_screen_size(1024.0, 768.0);

    let ui = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("Interface/BlizzardUI");
    env.state().borrow_mut().addon_base_paths = vec![ui.clone()];

    load_blizzard_addons(&env, &ui);
    env.apply_post_load_workarounds();
    fire_startup_events(&env);
    env
}

fn load_blizzard_addons(env: &WowLuaEnv, ui: &Path) {
    for (name, toc) in TOOLTIP_TEST_ADDONS {
        let addon_dir = ui.join(name);
        let requested_toc = addon_dir.join(toc);
        let toc_path = if requested_toc.exists() {
            requested_toc
        } else if let Some(discovered_toc) = find_toc_file(&addon_dir) {
            discovered_toc
        } else {
            continue;
        };
        if let Err(error) = load_addon(&env.loader_env(), &toc_path) {
            eprintln!("[load {name}] FAILED: {error}");
        }
    }
}

fn fire_startup_events(env: &WowLuaEnv) {
    ensure_player_frame_for_aura_tests(env);

    let lua = env.lua();
    let _ = env.fire_event_with_args(
        "ADDON_LOADED",
        &[mlua::Value::String(lua.create_string("WoWUISim").unwrap())],
    );
    for event in ["VARIABLES_LOADED", "PLAYER_LOGIN"] {
        let _ = env.fire_event(event);
    }
    let _ = env.fire_event_with_args(
        "PLAYER_ENTERING_WORLD",
        &[mlua::Value::Boolean(true), mlua::Value::Boolean(false)],
    );
    for event in [
        "UPDATE_BINDINGS",
        "DISPLAY_SIZE_CHANGED",
        "UI_SCALE_CHANGED",
    ] {
        let _ = env.fire_event(event);
    }

    refresh_aura_frames(env);
}

pub fn open_character_panel(env: &WowLuaEnv) {
    env.exec(
        r#"
        local btn = CharacterMicroButton
        assert(btn, "CharacterMicroButton should exist")
        local onclick = btn:GetScript("OnClick")
        assert(onclick, "CharacterMicroButton should have an OnClick handler")
        onclick(btn, "LeftButton", false)
        assert(CharacterFrame and CharacterFrame:IsShown(), "CharacterFrame should be shown")
        assert(CharacterHeadSlot ~= nil, "CharacterHeadSlot should exist")
        "#,
    )
    .expect("Failed to open character panel");
}

pub fn hover_first_visible_buff_icon(env: &WowLuaEnv) {
    env.exec(HOVER_FIRST_VISIBLE_BUFF_ICON_LUA)
        .expect("Failed to hover a visible buff icon");
}

pub fn refresh_buff_frame(env: &WowLuaEnv) {
    refresh_aura_frames(env);
    wow_ui_sim::startup::seed_buff_durations(env);
}

fn ensure_player_frame_for_aura_tests(env: &WowLuaEnv) {
    env.exec(
        r#"
        if not PlayerFrame then
            PlayerFrame = CreateFrame("Frame", "PlayerFrame", UIParent)
        end
        PlayerFrame.unit = "player"
        "#,
    )
    .expect("Failed to create PlayerFrame stub for aura tests");
}

fn refresh_aura_frames(env: &WowLuaEnv) {
    env.exec(
        r#"
        assert(BuffFrame, "BuffFrame should exist")
        assert(BuffFrame.UpdateAuras, "BuffFrame should expose UpdateAuras")
        local updateInfo = { isFullUpdate = true }
        if BuffFrame:IsEventRegistered("UNIT_AURA") and BuffFrame:GetScript("OnEvent") then
            BuffFrame:GetScript("OnEvent")(BuffFrame, "UNIT_AURA", "player", updateInfo)
        end
        BuffFrame:UpdateAuras()
        BuffFrame:Update()
        "#,
    )
    .expect("Failed to refresh BuffFrame after seeding buffs");
}
