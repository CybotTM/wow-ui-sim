#[path = "tooltip_full_env_helpers.rs"]
mod tooltip_full_env_helpers;

use tooltip_full_env_helpers::refresh_aura_frames;
use wow_ui_sim::lua_api::WowLuaEnv;

pub fn open_character_panel(env: &WowLuaEnv) {
    env.exec(
        r#"
        if CharacterFrame and CharacterFrame:IsShown() then
            CharacterFrame:Hide()
        end
        assert(not CharacterFrame:IsShown(), "CharacterFrame should start hidden")
        PanelTemplates_SetTab(CharacterFrame, PaperDollFrame:GetID())
        CharacterFrame:ShowSubFrame("PaperDollFrame")
        ShowUIPanel(CharacterFrame)
        CharacterFrame:RefreshDisplay()
        assert(CharacterFrame and CharacterFrame:IsShown(), "CharacterFrame should be shown")
        assert(CharacterFrame:GetNumPoints() > 0, "CharacterFrame should have a panel anchor")
        assert(CharacterHeadSlot ~= nil, "CharacterHeadSlot should exist")
        "#,
    )
    .expect("Failed to open character panel");
}

pub fn refresh_buff_frame(env: &WowLuaEnv) {
    refresh_aura_frames(env);
    wow_ui_sim::startup::seed_buff_durations(env);
}
