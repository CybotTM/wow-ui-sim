pub(super) const VIGNETTE_PIN_TEMPLATE_WORKAROUND_LUA: &str = r###"
local function __wow_patch_vignette_provider(provider)
    if type(provider) ~= "table" then
        return
    end
    if type(provider.GetPinTemplate) ~= "function" then
        return
    end
    if type(provider.GetDefaultPinTemplate) ~= "function" then
        return
    end
    if provider.__wow_ui_sim_nil_safe_get_pin_template then
        return
    end
    if provider:GetDefaultPinTemplate() ~= "VignettePinTemplate" then
        return
    end

    local original = provider.GetPinTemplate
    function provider:GetPinTemplate(vignetteInfo)
        if vignetteInfo == nil then
            return self:GetDefaultPinTemplate()
        end
        return original(self, vignetteInfo)
    end
    provider.__wow_ui_sim_nil_safe_get_pin_template = true
end

__wow_patch_vignette_provider(VignetteDataProviderMixin)

for _, mapName in ipairs({"WorldMapFrame", "BattlefieldMapFrame", "FlightMapFrame"}) do
    local map = _G[mapName]
    if map and type(map.dataProviders) == "table" then
        for provider in pairs(map.dataProviders) do
            __wow_patch_vignette_provider(provider)
        end
    end
end
"###;

pub(super) const UIPARENT_ONUPDATE_WORKLISTS_WORKAROUND_LUA: &str = r#"
if type(FCF_OnUpdate) == "function" and rawget(_G, "__wow_fcf_onupdate_wrapper") ~= FCF_OnUpdate then
    local original_fcf_onupdate = FCF_OnUpdate
    local wrapper = function(elapsed)
        if type(CHAT_FRAMES) == "table" and next(CHAT_FRAMES) == nil then
            return
        end
        return original_fcf_onupdate(elapsed)
    end
    FCF_OnUpdate = wrapper
    rawset(_G, "__wow_fcf_onupdate_wrapper", wrapper)
end

if type(ButtonPulse_OnUpdate) == "function" and rawget(_G, "__wow_button_pulse_onupdate_wrapper") ~= ButtonPulse_OnUpdate then
    local original_button_pulse_onupdate = ButtonPulse_OnUpdate
    local wrapper = function(elapsed)
        if type(PULSEBUTTONS) == "table" and next(PULSEBUTTONS) == nil then
            return
        end
        return original_button_pulse_onupdate(elapsed)
    end
    ButtonPulse_OnUpdate = wrapper
    rawset(_G, "__wow_button_pulse_onupdate_wrapper", wrapper)
end

if type(AnimatedShine_OnUpdate) == "function" and rawget(_G, "__wow_animated_shine_onupdate_wrapper") ~= AnimatedShine_OnUpdate then
    local original_animated_shine_onupdate = AnimatedShine_OnUpdate
    local wrapper = function(elapsed)
        if type(SHINES_TO_ANIMATE) == "table" and next(SHINES_TO_ANIMATE) == nil then
            return
        end
        return original_animated_shine_onupdate(elapsed)
    end
    AnimatedShine_OnUpdate = wrapper
    rawset(_G, "__wow_animated_shine_onupdate_wrapper", wrapper)
end

if type(UIParent) == "table"
    and type(UIParent.GetScript) == "function"
    and type(UIParent.SetScript) == "function" then
    local wrapper = rawget(_G, "__wow_ui_parent_onupdate_worklist_wrapper")
    if UIParent:GetScript("OnUpdate") ~= wrapper then
        wrapper = function(self, elapsed)
            if type(CHAT_FRAMES) ~= "table" or next(CHAT_FRAMES) ~= nil then
                FCF_OnUpdate(elapsed)
            end
            if type(PULSEBUTTONS) ~= "table" or next(PULSEBUTTONS) ~= nil then
                ButtonPulse_OnUpdate(elapsed)
            end
            if type(SHINES_TO_ANIMATE) ~= "table" or next(SHINES_TO_ANIMATE) ~= nil then
                AnimatedShine_OnUpdate(elapsed)
            end
            if type(HelpOpenWebTicketButton_OnUpdate) == "function" then
                HelpOpenWebTicketButton_OnUpdate(HelpOpenWebTicketButton, elapsed)
            end
        end
        UIParent:SetScript("OnUpdate", wrapper)
        rawset(_G, "__wow_ui_parent_onupdate_worklist_wrapper", wrapper)
    end
end
"#;

pub(super) const CHAT_VOICE_BUTTON_SURFACE_WORKAROUND_LUA: &str = r#"
local defaultChatFrame = DEFAULT_CHAT_FRAME or ChatFrame1
local defaultEditBox = rawget(_G, "ChatFrame1EditBox")
if type(defaultChatFrame) == "table" and type(defaultEditBox) == "table" then
    if defaultChatFrame.editBox == nil then
        defaultChatFrame.editBox = defaultEditBox
    end
    if defaultEditBox.chatFrame == nil then
        defaultEditBox.chatFrame = defaultChatFrame
    end
    if DEFAULT_CHAT_FRAME == nil then
        DEFAULT_CHAT_FRAME = defaultChatFrame
    end
end

local channelButton = ChatFrameChannelButton
if type(channelButton) == "table" then
    local icon = channelButton.Icon
    if icon == nil and type(channelButton.CreateTexture) == "function" then
        icon = channelButton:CreateTexture(nil, "OVERLAY")
        channelButton.Icon = icon
    end

    if icon ~= nil then
        if type(icon.SetParentKey) == "function" then
            pcall(icon.SetParentKey, icon, "Icon", true)
        end
        if type(icon.GetWidth) == "function" and type(icon.GetHeight) == "function"
            and (icon:GetWidth() == 0 or icon:GetHeight() == 0)
            and type(icon.SetSize) == "function" then
            icon:SetSize(channelButton.fixedIconWidth or 15, channelButton.fixedIconHeight or 15)
        end
        if type(icon.GetNumPoints) == "function" and icon:GetNumPoints() == 0
            and type(icon.SetPoint) == "function" then
            icon:SetPoint("CENTER", channelButton, "CENTER", 0, 0)
        end
        if type(icon.SetAtlas) == "function" then
            icon:SetAtlas("chatframe-button-icon-voicechat")
        end
        if type(icon.Show) == "function" then
            icon:Show()
        end
    end
end

if QuickJoinToastButton == nil and type(CreateFrame) == "function" and UIParent ~= nil then
    QuickJoinToastButton = CreateFrame("Button", "QuickJoinToastButton", UIParent)
end
"#;

pub(super) const ITEM_SOCKETING_TOOLTIPS_WORKAROUND_LUA: &str = r#"
local frame = ItemSocketingFrame
local container = frame and frame.SocketingContainer
if type(container) ~= "table" then
    return
end

local function install_socket_on_enter(socket, socketIndex)
    if type(socket) ~= "table" or type(socket.SetScript) ~= "function" then
        return
    end
    socket:SetScript("OnEnter", function(self)
        if type(GameTooltip) ~= "table" then
            return
        end
        if type(GameTooltip.SetOwner) == "function" then
            GameTooltip:SetOwner(self, "ANCHOR_RIGHT")
        end
        if type(GameTooltip.SetSocketGem) == "function" then
            GameTooltip:SetSocketGem(socketIndex)
        end
        if type(GameTooltip.NumLines) == "function"
            and GameTooltip:NumLines() == 0
            and type(GameTooltip.AddLine) == "function" then
            GameTooltip:AddLine("Socket Gem " .. tostring(socketIndex))
        end
        if type(GameTooltip.Show) == "function" then
            GameTooltip:Show()
        end
    end)
end

install_socket_on_enter(container.Socket1, 1)
install_socket_on_enter(container.Socket2, 2)
install_socket_on_enter(container.Socket3, 3)
"#;

pub(super) const ACTION_BAR_BUTTON_EVENT_FANOUT_WORKAROUND_LUA: &str = r##"
if type(ActionBarButtonEventsFrameMixin) ~= "table" then
    return
end

local traceFanout = {trace_fanout}

local function button_label(frame, index)
    if type(frame) ~= "table" then
        return "#" .. tostring(index)
    end
    if type(frame.GetName) == "function" then
        local name = frame:GetName()
        if name ~= nil then
            return name
        end
    end
    if frame.action ~= nil then
        return "action:" .. tostring(frame.action)
    end
    return "#" .. tostring(index)
end

local function for_each_button_frame(self, func)
    local frames = self.frames
    if type(frames) ~= "table" then
        return
    end
    for i = 1, #frames do
        local frame = rawget(frames, i)
        if frame ~= nil then
            if traceFanout then
                print("[ActionBarFanout] begin " .. button_label(frame, i))
            end
            func(frame)
            if traceFanout then
                print("[ActionBarFanout] end " .. button_label(frame, i))
            end
        end
    end
end

local function on_event(self, event, ...)
    for_each_button_frame(self, function(frame)
        frame:OnEvent(event, ...)
    end)
    if event == "ACTIONBAR_SLOT_CHANGED" or event == "ACTIONBAR_UPDATE_STATE" then
        for_each_button_frame(self, function(frame)
            if type(frame.UpdateButtonArt) == "function" then
                pcall(frame.UpdateButtonArt, frame)
            end
        end)
    end
end

local function on_countdown_for_cooldowns_changed(self)
    for_each_button_frame(self, function(frame)
        ActionButton_UpdateCooldownNumberHidden(frame)
    end)
end

local function for_each_frame(self, func)
    for_each_button_frame(self, func)
end

ActionBarButtonEventsFrameMixin.OnEvent = on_event
ActionBarButtonEventsFrameMixin.OnCountdownForCooldownsChanged = on_countdown_for_cooldowns_changed
ActionBarButtonEventsFrameMixin.ForEachFrame = for_each_frame

if type(ActionBarButtonEventsFrame) == "table" then
    ActionBarButtonEventsFrame.OnEvent = on_event
    ActionBarButtonEventsFrame.OnCountdownForCooldownsChanged = on_countdown_for_cooldowns_changed
    ActionBarButtonEventsFrame.ForEachFrame = for_each_frame
    if type(ActionBarButtonEventsFrame.SetScript) == "function" then
        ActionBarButtonEventsFrame:SetScript("OnEvent", on_event)
    end
end
"##;

pub(super) const POST_EVENT_FRAME_LAYOUT_WORKAROUND_LUA: &str = r#"
local function reanchor_objective_tracker(frame)
    frame:ClearAllPoints()
    frame:SetPoint(
        "TOPRIGHT",
        UIParentRightManagedFrameContainer,
        "TOPRIGHT",
        0,
        11
    )
    frame:SetHeight(836.5)
end

if EditModeManagerFrame then
    local partySystem = EditModeManagerFrame:GetRegisteredSystemFrame(
        Enum.EditModeSystem.UnitFrame,
        Enum.EditModeUnitFrameSystemIndices.Party
    )
    if partySystem and partySystem.systemInfo and partySystem.systemInfo.settings then
        for _, settingInfo in ipairs(partySystem.systemInfo.settings) do
            if settingInfo.setting == Enum.EditModeUnitFrameSetting.UseRaidStylePartyFrames then
                settingInfo.value = 0
            end
        end
        if partySystem.UpdateSettingMap then
            partySystem:UpdateSettingMap(true)
        end
        if partySystem.UpdateSystemSetting then
            pcall(
                partySystem.UpdateSystemSetting,
                partySystem,
                Enum.EditModeUnitFrameSetting.UseRaidStylePartyFrames,
                true
            )
        end
    end
end

if UpdateRaidAndPartyFrames then
    pcall(UpdateRaidAndPartyFrames)
end
if PartyFrame and PartyFrame.UpdatePaddingAndLayout then
    pcall(PartyFrame.UpdatePaddingAndLayout, PartyFrame)
end
if CompactPartyFrame and CompactPartyFrame.UpdateVisibility then
    pcall(CompactPartyFrame.UpdateVisibility, CompactPartyFrame)
end
if ObjectiveTrackerFrame then
    if ObjectiveTrackerFrame.Update then
        pcall(ObjectiveTrackerFrame.Update, ObjectiveTrackerFrame)
    end
    if ObjectiveTrackerFrame.UpdateHeight then
        pcall(ObjectiveTrackerFrame.UpdateHeight, ObjectiveTrackerFrame)
    end
    reanchor_objective_tracker(ObjectiveTrackerFrame)
end
if CompactPartyFrame then
    CompactPartyFrame:SetHeight(234)
end
if PlayerCastingBarFrame then
    PlayerCastingBarFrame:SetAlpha(1)
end
if not rawget(_G, "__wow_objective_tracker_update_height_wrapper")
    and ObjectiveTrackerContainerMixin
    and type(ObjectiveTrackerContainerMixin.UpdateHeight) == "function" then
    local originalUpdateHeight = ObjectiveTrackerContainerMixin.UpdateHeight
    function ObjectiveTrackerContainerMixin:UpdateHeight()
        originalUpdateHeight(self)
        if self == ObjectiveTrackerFrame then
            reanchor_objective_tracker(self)
        end
    end
    rawset(_G, "__wow_objective_tracker_update_height_wrapper", true)
end
if not rawget(_G, "__wow_compact_party_update_layout_wrapper")
    and CompactPartyFrameMixin
    and type(CompactPartyFrameMixin.UpdateLayout) == "function" then
    local originalUpdateLayout = CompactPartyFrameMixin.UpdateLayout
    function CompactPartyFrameMixin:UpdateLayout()
        originalUpdateLayout(self)
        self:SetHeight(234)
    end
    rawset(_G, "__wow_compact_party_update_layout_wrapper", true)
end
if not rawget(_G, "__wow_casting_bar_apply_alpha_wrapper")
    and CastingBarMixin
    and type(CastingBarMixin.ApplyAlpha) == "function" then
    local originalApplyAlpha = CastingBarMixin.ApplyAlpha
    function CastingBarMixin:ApplyAlpha(alpha)
        if self == PlayerCastingBarFrame then
            alpha = 1
        end
        originalApplyAlpha(self, alpha)
    end
    rawset(_G, "__wow_casting_bar_apply_alpha_wrapper", true)
end
if ChatFrame1EditBox and ChatFrame1 then
    ChatFrame1EditBox:SetWidth(447)
end
"#;

pub(super) const REFRESH_ACTION_BUTTONS_LUA: &str = r###"
local function __wow_refresh_action_button(button)
    if type(button) ~= "table" then
        return
    end
    if type(button.UpdateButtonArt) == "function" then
        pcall(button.UpdateButtonArt, button)
    end
    if type(button.UpdateHotkeys) == "function" then
        pcall(button.UpdateHotkeys, button, button.buttonType)
    end
end

for i = 1, 12 do
    __wow_refresh_action_button(_G["ActionButton" .. i])
end
"###;
