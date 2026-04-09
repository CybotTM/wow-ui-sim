use crate::tooltip_full_env_helpers::refresh_aura_frames;
use wow_ui_sim::lua_api::WowLuaEnv;

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
