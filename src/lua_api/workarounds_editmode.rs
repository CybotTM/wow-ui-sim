//! EditMode layout workarounds.
//!
//! Patches EditModeManagerFrame to apply preset layout anchors to all 43
//! registered system frames. The real UpdateLayoutInfo crashes partway through
//! due to cascading dependencies, so we manually set up layoutInfo and call
//! our custom InitSystemAnchors.

use super::WowLuaEnv;
use std::time::Instant;

const SETUP_LAYOUT_INFO_LUA: &str = r#"
    local function setSystemSetting(systemInfo, setting, value)
        if not systemInfo or not systemInfo.settings then return end
        for _, settingInfo in ipairs(systemInfo.settings) do
            if settingInfo.setting == setting then
                settingInfo.value = value
                return
            end
        end
        table.insert(systemInfo.settings, { setting = setting, value = value })
    end

    local function hasSystemSetting(systemInfo, setting)
        if not systemInfo or not systemInfo.settings then return false end
        for _, settingInfo in ipairs(systemInfo.settings) do
            if settingInfo.setting == setting then
                return true
            end
        end
        return false
    end

    local function copyAnchorInfo(anchorInfo)
        if type(anchorInfo) ~= "table" then
            return anchorInfo
        end
        local copy = {}
        for key, value in pairs(anchorInfo) do
            copy[key] = value
        end
        return copy
    end

    local function copySettings(settings)
        local copy = {}
        if type(settings) ~= "table" then
            return copy
        end
        for i, settingInfo in ipairs(settings) do
            copy[i] = {
                setting = settingInfo.setting,
                value = settingInfo.value,
            }
        end
        return copy
    end

    local function copySystems(systems)
        local copy = {}
        if type(systems) ~= "table" then
            return copy
        end
        for i, systemInfo in ipairs(systems) do
            copy[i] = {
                system = systemInfo.system,
                systemIndex = systemInfo.systemIndex,
                isInDefaultPosition = systemInfo.isInDefaultPosition,
                anchorInfo = copyAnchorInfo(systemInfo.anchorInfo),
                settings = copySettings(systemInfo.settings),
            }
        end
        return copy
    end

    local function copyLayouts(layouts)
        local copy = {}
        if type(layouts) ~= "table" then
            return copy
        end
        for i, layoutInfo in ipairs(layouts) do
            copy[i] = {
                layoutIndex = layoutInfo.layoutIndex,
                layoutName = layoutInfo.layoutName,
                layoutType = layoutInfo.layoutType,
                systems = copySystems(layoutInfo.systems),
            }
        end
        return copy
    end

    local function defaultSettingsFromModernMap(systemInfo)
        if not EditModePresetLayoutManager
            or not EditModePresetLayoutManager.GetModernSystemMap then
            return nil
        end

        local modernMap = EditModePresetLayoutManager:GetModernSystemMap()
        local systemDefaults = modernMap and modernMap[systemInfo.system]
        if type(systemDefaults) ~= "table" then
            return nil
        end

        if systemInfo.systemIndex == nil or systemInfo.systemIndex == -1 then
            if type(systemDefaults.settings) == "table" then
                return systemDefaults.settings
            end
        end

        local indexedDefaults = systemDefaults[systemInfo.systemIndex]
        if type(indexedDefaults) == "table" then
            return indexedDefaults.settings
        end
        return nil
    end

    local function defaultSettingsFromManager(systemInfo)
        if not EditModePresetLayoutManager
            or not EditModePresetLayoutManager.GetAllDefaultSettingsForSystem then
            return nil
        end

        local ok, defaults = pcall(
            EditModePresetLayoutManager.GetAllDefaultSettingsForSystem,
            EditModePresetLayoutManager,
            systemInfo.system,
            systemInfo.systemIndex
        )
        if ok then
            return defaults
        end
        return nil
    end

    local function mergeDefaultSystemSettings(systemInfo)
        if not systemInfo
            or not EditModePresetLayoutManager then
            return
        end

        local defaults = defaultSettingsFromModernMap(systemInfo)
        if not defaults and not EditModePresetLayoutManager.GetModernSystemMap then
            defaults = defaultSettingsFromManager(systemInfo)
        end
        if type(defaults) ~= "table" then
            return
        end

        for setting, value in pairs(defaults) do
            if not hasSystemSetting(systemInfo, setting) then
                setSystemSetting(systemInfo, setting, value)
            end
        end
    end

    local function mergeDefaultSettings(layoutInfo)
        if not layoutInfo or not layoutInfo.layouts then return end
        for _, layout in ipairs(layoutInfo.layouts) do
            if layout.layoutType ~= Enum.EditModeLayoutType.Preset and layout.systems then
                for _, systemInfo in ipairs(layout.systems) do
                    mergeDefaultSystemSettings(systemInfo)
                end
            end
        end
    end

    local function forceStandardPartyFrames(layoutInfo)
        if not layoutInfo or not layoutInfo.layouts then return end
        for _, preset in ipairs(layoutInfo.layouts) do
            if type(preset) == "table"
                and preset.layoutType == Enum.EditModeLayoutType.Preset
                and preset.systems then
                for _, systemInfo in ipairs(preset.systems) do
                    if systemInfo.system == Enum.EditModeSystem.UnitFrame
                        and systemInfo.systemIndex == Enum.EditModeUnitFrameSystemIndices.Party then
                        setSystemSetting(systemInfo, Enum.EditModeUnitFrameSetting.UseRaidStylePartyFrames, 0)
                    end
                end
            end
        end
    end

    local function remapActiveLayoutAfterPresetPrepend(layoutInfo, savedLayouts, presetCount)
        if not layoutInfo or type(layoutInfo.activeLayout) ~= "number" then
            return 1
        end
        if type((savedLayouts or {})[layoutInfo.activeLayout]) == "table" then
            return presetCount + layoutInfo.activeLayout
        end
        for savedIndex, savedLayout in ipairs(savedLayouts or {}) do
            if type(savedLayout) == "table" and savedLayout.layoutIndex == layoutInfo.activeLayout then
                return presetCount + savedIndex
            end
        end
        if layoutInfo.activeLayout >= 1 and layoutInfo.activeLayout <= #(layoutInfo.layouts or {}) then
            return layoutInfo.activeLayout
        end
        return 1
    end

    if not EditModeManagerFrame then return end
    local emm = EditModeManagerFrame
    if not emm.layoutInfo then
        local layoutInfo = C_EditMode.GetLayouts()
        emm.layoutInfo = layoutInfo
        local savedLayouts = copyLayouts(emm.layoutInfo.layouts)
        emm.layoutInfo.layouts = copyLayouts(EditModePresetLayoutManager.presetLayoutInfo)
        local presetCount = #emm.layoutInfo.layouts
        tAppendAll(emm.layoutInfo.layouts, savedLayouts)
        emm.layoutInfo.activeLayout = remapActiveLayoutAfterPresetPrepend(emm.layoutInfo, savedLayouts, presetCount)
    end
    mergeDefaultSettings(emm.layoutInfo)
    forceStandardPartyFrames(emm.layoutInfo)
    local function applyAccountSettingOverrides()
        local accountSettings = emm.AccountSettings
        local accountEnum = Enum and Enum.EditModeAccountSetting
        if not accountSettings or not accountEnum then
            return
        end

        local function getAccountSettingValue(setting)
            local settingValue = nil
            if emm.GetAccountSettingValue then
                settingValue = emm:GetAccountSettingValue(setting)
            else
                for _, settingInfo in ipairs(emm.accountSettings or {}) do
                    if settingInfo.setting == setting then
                        settingValue = settingInfo.value
                        break
                    end
                end
            end
            return settingValue
        end

        local function getAccountSettingBool(setting)
            local settingValue = getAccountSettingValue(setting)
            if settingValue == nil then
                return nil
            end
            return settingValue == 1 or settingValue == true
        end

        local function applyFrameSetting(setting, frame, setter, isBool)
            if setting == nil or type(frame) ~= "table" or type(frame[setter]) ~= "function" then
                return
            end
            local settingValue
            if isBool then
                settingValue = getAccountSettingBool(setting)
            else
                settingValue = getAccountSettingValue(setting)
            end
            if settingValue ~= nil then
                pcall(frame[setter], frame, settingValue)
            end
        end

        local managerSettings = {
            { setting = accountEnum.ShowGrid, setter = "SetGridShown", isBool = true },
            { setting = accountEnum.GridSpacing, setter = "SetGridSpacing" },
            { setting = accountEnum.EnableSnap, setter = "SetEnableSnap", isBool = true },
            { setting = accountEnum.EnableAdvancedOptions, setter = "SetEnableAdvancedOptions", isBool = true },
        }
        for _, settingInfo in ipairs(managerSettings) do
            applyFrameSetting(settingInfo.setting, emm, settingInfo.setter, settingInfo.isBool)
        end

        local accountSettingSetters = {
            { setting = accountEnum.SettingsExpanded, setter = "SetExpandedState" },
            { setting = accountEnum.ShowTargetAndFocus, setter = "SetTargetAndFocusShown" },
            { setting = accountEnum.ShowPartyFrames, setter = "SetPartyFramesShown" },
            { setting = accountEnum.ShowRaidFrames, setter = "SetRaidFramesShown" },
            { setting = accountEnum.ShowStanceBar, setter = "SetStanceBarShown" },
            { setting = accountEnum.ShowPetActionBar, setter = "SetPetActionBarShown" },
            { setting = accountEnum.ShowPossessActionBar, setter = "SetPossessActionBarShown" },
            { setting = accountEnum.ShowCastBar, setter = "SetCastBarShown" },
            { setting = accountEnum.ShowEncounterBar, setter = "SetEncounterBarShown" },
            { setting = accountEnum.ShowExtraAbilities, setter = "SetExtraAbilitiesShown" },
            { setting = accountEnum.ShowBuffsAndDebuffs, setter = "SetBuffsAndDebuffsShown" },
            { setting = accountEnum.ShowExternalDefensives, setter = "SetExternalDefensivesShown" },
            { setting = accountEnum.ShowTalkingHeadFrame, setter = "SetTalkingHeadFrameShown" },
            { setting = accountEnum.ShowVehicleLeaveButton, setter = "SetVehicleLeaveButtonShown" },
            { setting = accountEnum.ShowBossFrames, setter = "SetBossFramesShown" },
            { setting = accountEnum.ShowArenaFrames, setter = "SetArenaFramesShown" },
            { setting = accountEnum.ShowLootFrame, setter = "SetLootFrameShown" },
            { setting = accountEnum.ShowHudTooltip, setter = "SetHudTooltipShown" },
            { setting = accountEnum.ShowStatusTrackingBar2, setter = "SetStatusTrackingBar2Shown" },
            { setting = accountEnum.ShowDurabilityFrame, setter = "SetDurabilityFrameShown" },
            { setting = accountEnum.ShowPetFrame, setter = "SetPetFrameShown" },
            { setting = accountEnum.ShowTimerBars, setter = "SetTimerBarsShown" },
            { setting = accountEnum.ShowVehicleSeatIndicator, setter = "SetVehicleSeatIndicatorShown" },
            { setting = accountEnum.ShowArchaeologyBar, setter = "SetArchaeologyBarShown" },
            { setting = accountEnum.ShowCooldownViewer, setter = "SetCooldownViewerShown" },
            { setting = accountEnum.ShowPersonalResourceDisplay, setter = "SetPersonalResourceDisplayShown" },
            { setting = accountEnum.ShowEncounterEvents, setter = "SetEncounterEventsShown" },
            { setting = accountEnum.ShowDamageMeter, setter = "SetDamageMeterShown" },
            { setting = accountEnum.ShowTotemActionBar, setter = "SetTotemActionBarShown" },
        }
        for _, settingInfo in ipairs(accountSettingSetters) do
            applyFrameSetting(settingInfo.setting, accountSettings, settingInfo.setter, true)
        end
    end
    if emm.InitializeAccountSettings then
        emm:InitializeAccountSettings()
        applyAccountSettingOverrides()
    else
        if not emm.accountSettings then
            emm.accountSettings = C_EditMode.GetAccountSettings()
        end
        if emm.UpdateAccountSettingMap then
            pcall(emm.UpdateAccountSettingMap, emm)
        end
        applyAccountSettingOverrides()
    end
"#;

const APPLY_SYSTEM_ANCHORS_LUA: &str = r#"
        if not EditModeManagerFrame then return end
        local emm = EditModeManagerFrame
        if not emm.layoutInfo then return end
        emm.layoutApplyInProgress = true
        emm:InitSystemAnchors()

        local function system_frame_name(systemFrame)
            if not systemFrame then
                return "nil"
            end
            if type(systemFrame.GetName) == "function" then
                local name = systemFrame:GetName()
                if name ~= nil then
                    return name
                end
            end
            return tostring(systemFrame.system) .. ":" .. tostring(systemFrame.systemIndex)
        end

        local function is_bootstrap_action_bar(systemFrame)
            local frameName = system_frame_name(systemFrame)
            if string.sub(frameName, 1, 8) == "MultiBar" then
                return true
            end
            if not systemFrame or not EditModeUtil then
                return false
            end
            if Enum and Enum.EditModeSystem and systemFrame.system == Enum.EditModeSystem.ActionBar then
                return true
            end
            return EditModeUtil:IsBottomAnchoredActionBar(systemFrame)
                or EditModeUtil:IsRightAnchoredActionBar(systemFrame)
        end

        local function skips_expensive_startup_system_update(systemFrame)
            local frameName = system_frame_name(systemFrame)
            -- These frames run full roster/unit layout work during UpdateSystem.
            -- Their default startup positions only need systemInfo + anchors.
            return frameName == "PartyFrame"
                or frameName == "CompactArenaFrame"
                or frameName == "CompactRaidFrameContainer"
        end

        local function skips_full_startup_scale_update(systemFrame)
            local frameName = system_frame_name(systemFrame)
            -- Saved frame-size values are raw EditMode slider values. Running
            -- full size updates during startup can transiently feed those raw
            -- defaults into SetScale before the frame is fully ready.
            return frameName == "PlayerFrame"
                or frameName == "TargetFrame"
                or frameName == "FocusFrame"
                or frameName == "PlayerCastingBarFrame"
        end

        local function seed_system_frame(systemFrame)
            local systemIndex = systemFrame.systemIndex
            if systemIndex == nil then
                systemIndex = -1
            end
            local systemInfo = emm:GetActiveLayoutSystemInfo(systemFrame.system, systemIndex)
            if not systemInfo then
                return
            end

            systemFrame.savedSystemInfo = CopyTable(systemInfo)
            systemFrame.systemInfo = systemInfo
            systemFrame:SetHasActiveChanges(false)
            systemFrame:UpdateSettingMap(true)
            return true
        end

        local function anchor_targets_system_frame(systemFrame, anchorInfo)
            if not systemFrame or type(anchorInfo) ~= "table" then
                return false
            end

            local relativeTo = anchorInfo.relativeTo
            if relativeTo == systemFrame then
                return true
            end

            if type(relativeTo) == "string" and type(systemFrame.GetName) == "function" then
                return relativeTo == systemFrame:GetName()
            end

            return false
        end

        local function apply_system_anchor_if_safe(systemFrame)
            if not systemFrame or not systemFrame.ApplySystemAnchor then
                return
            end

            local frameName = system_frame_name(systemFrame)
            if frameName == "PlayerCastingBarFrame" then
                return
            end

            local systemInfo = systemFrame.systemInfo
            if anchor_targets_system_frame(systemFrame, systemInfo and systemInfo.anchorInfo) then
                return
            end

            if frameName == "PlayerFrame"
                and EditModeSystemMixin
                and EditModeSystemMixin.ApplySystemAnchor then
                pcall(EditModeSystemMixin.ApplySystemAnchor, systemFrame)
                return
            end

            pcall(systemFrame.ApplySystemAnchor, systemFrame)
        end

        local function replay_system_settings(systemFrame)
            local systemInfo = systemFrame and systemFrame.systemInfo
            if not systemInfo or not systemFrame.UpdateSystemSetting then
                return
            end

            for _, settingInfo in ipairs(systemInfo.settings or {}) do
                local setting = settingInfo.setting
                local unitFrameSettings = Enum and Enum.EditModeUnitFrameSetting
                local needsAuraUpdate = unitFrameSettings
                    and setting == unitFrameSettings.BuffsOnTop
                    and not systemFrame.UpdateAuras
                if not needsAuraUpdate then
                    pcall(systemFrame.UpdateSystemSetting, systemFrame, setting, true)
                end
            end
        end

        local function refresh_system_layout_after_setting_replay(systemFrame)
            if systemFrame and systemFrame.UpdateGridLayout then
                pcall(systemFrame.UpdateGridLayout, systemFrame)
            end
        end

        local function refresh_action_bar_system(systemFrame)
            local systemInfo = systemFrame.systemInfo
            local actionButtons = systemFrame.actionButtons
            local hasActionButtons = actionButtons
                and actionButtons[1] ~= nil
                and actionButtons.GetObjectType == nil

            local function mark_action_bar_layout_dirty()
                if systemFrame.MarkGridLayoutDirty then
                    systemFrame:MarkGridLayoutDirty()
                end
                if systemFrame.MarkDividersDirty then
                    systemFrame:MarkDividersDirty()
                end
                if systemFrame.MarkBarArtDirty then
                    systemFrame:MarkBarArtDirty()
                end
            end

            local function apply_action_bar_setting(settingInfo)
                local actionBarSettings = Enum and Enum.EditModeActionBarSetting
                if not actionBarSettings then
                    return
                end

                local setting = settingInfo.setting
                local function get_setting_value(useRawValue)
                    if systemFrame.GetSettingValue then
                        local ok, convertedValue = pcall(
                            systemFrame.GetSettingValue,
                            systemFrame,
                            setting,
                            useRawValue
                        )
                        if ok and convertedValue ~= nil then
                            return convertedValue
                        end
                    end
                    return settingInfo.value
                end

                local rawValue = get_setting_value(true)
                local value = get_setting_value(false)
                local valueBool = value == 1 or value == true

                if setting == actionBarSettings.Orientation then
                    systemFrame.isHorizontal = rawValue == Enum.ActionBarOrientation.Horizontal
                    if systemFrame.Selection and systemFrame.Selection.SetVerticalState then
                        pcall(systemFrame.Selection.SetVerticalState, systemFrame.Selection, not systemFrame.isHorizontal)
                    end
                    systemFrame.addButtonsToRight = true
                    systemFrame.addButtonsToTop = systemFrame.isHorizontal
                    mark_action_bar_layout_dirty()
                elseif setting == actionBarSettings.NumRows then
                    systemFrame.numRows = value
                    mark_action_bar_layout_dirty()
                elseif setting == actionBarSettings.NumIcons then
                    -- Deferred: live Palaky/Ultrawide shows twelve main-bar
                    -- buttons even though the cache stores NumIcons=8.
                    -- Keep the existing visible-button state while the other
                    -- profile settings replay.
                elseif setting == actionBarSettings.IconSize then
                    systemFrame.iconSize = value
                    local iconScale = value / 100
                    if systemFrame.EditModeSetScale then
                        pcall(systemFrame.EditModeSetScale, systemFrame, iconScale)
                    end
                    if hasActionButtons then
                        for _, actionButton in pairs(actionButtons) do
                            local container = actionButton and actionButton.container
                            if container and container.SetScale then
                                pcall(container.SetScale, container, iconScale)
                            end
                        end
                    end
                    if systemFrame.Layout then
                        pcall(systemFrame.Layout, systemFrame)
                    end
                    if emm.UpdateActionBarLayout then
                        pcall(emm.UpdateActionBarLayout, emm, systemFrame)
                    end
                    mark_action_bar_layout_dirty()
                elseif setting == actionBarSettings.IconPadding then
                    systemFrame.buttonPadding = value
                    mark_action_bar_layout_dirty()
                elseif setting == actionBarSettings.HideBarArt then
                    systemFrame.hideBarArt = valueBool
                    if systemFrame.UpdateEndCaps then
                        pcall(systemFrame.UpdateEndCaps, systemFrame, systemFrame.hideBarArt)
                    end
                    if systemFrame.BorderArt then
                        systemFrame.BorderArt:SetShown(not systemFrame.hideBarArt)
                    end
                    if hasActionButtons then
                        for _, actionButton in pairs(actionButtons) do
                            if actionButton and actionButton.UpdateButtonArt then
                                pcall(actionButton.UpdateButtonArt, actionButton)
                            end
                        end
                    end
                    mark_action_bar_layout_dirty()
                elseif setting == actionBarSettings.HideBarScrolling then
                    if systemFrame.ActionBarPageNumber then
                        systemFrame.ActionBarPageNumber:SetShown(not valueBool)
                    end
                    if systemFrame.MarkBarArtDirty then
                        systemFrame:MarkBarArtDirty()
                    end
                elseif setting == actionBarSettings.VisibleSetting
                    and Enum.ActionBarVisibleSetting then
                    if rawValue == Enum.ActionBarVisibleSetting.InCombat then
                        systemFrame.visibility = "InCombat"
                    elseif rawValue == Enum.ActionBarVisibleSetting.OutOfCombat then
                        systemFrame.visibility = "OutOfCombat"
                    elseif rawValue == Enum.ActionBarVisibleSetting.Hidden then
                        systemFrame.visibility = "Hidden"
                    else
                        systemFrame.visibility = "Always"
                    end
                    if systemFrame.UpdateVisibility then
                        pcall(systemFrame.UpdateVisibility, systemFrame)
                    end
                elseif setting == actionBarSettings.AlwaysShowButtons then
                    systemFrame.alwaysShowButtons = valueBool
                    if hasActionButtons
                        and systemFrame.SetShowGrid
                        and ACTION_BUTTON_SHOW_GRID_REASON_CVAR then
                        pcall(
                            systemFrame.SetShowGrid,
                            systemFrame,
                            valueBool,
                            ACTION_BUTTON_SHOW_GRID_REASON_CVAR
                        )
                    end
                    if systemFrame.MarkDividersDirty then
                        systemFrame:MarkDividersDirty()
                    end
                end
            end

            -- Replay the action-bar setting handlers without the full
            -- EditMode frame update path. Some bars expose FrameRef userdata
            -- through actionButtons during bootstrap; Blizzard's handlers call
            -- pairs(self.actionButtons), which is not safe until the real Lua
            -- array exists.
            for _, settingInfo in ipairs(systemInfo and systemInfo.settings or {}) do
                apply_action_bar_setting(settingInfo)
            end
            if hasActionButtons and systemFrame.RefreshGridLayout then
                systemFrame:RefreshGridLayout()
            end
            if hasActionButtons and systemFrame.RefreshDividers then
                systemFrame:RefreshDividers()
            end
            if systemFrame.RefreshBarArt then
                systemFrame:RefreshBarArt()
            end
        end

        for _, systemFrame in ipairs(emm.registeredSystemFrames or {}) do
            if is_bootstrap_action_bar(systemFrame) then
                -- Full EditMode action-bar updates are expensive on the live
                -- path and can stall startup. Seed just enough state for the
                -- layout pass to run, then apply the saved anchor directly.
                if seed_system_frame(systemFrame) then
                    apply_system_anchor_if_safe(systemFrame)
                    refresh_action_bar_system(systemFrame)
                end
            elseif skips_expensive_startup_system_update(systemFrame) then
                if seed_system_frame(systemFrame) then
                    apply_system_anchor_if_safe(systemFrame)
                    replay_system_settings(systemFrame)
                end
            elseif skips_full_startup_scale_update(systemFrame) then
                if seed_system_frame(systemFrame) then
                    apply_system_anchor_if_safe(systemFrame)
                    replay_system_settings(systemFrame)
                end
            elseif seed_system_frame(systemFrame)
                and anchor_targets_system_frame(systemFrame, systemFrame.systemInfo and systemFrame.systemInfo.anchorInfo) then
                -- Saved layouts can contain self-relative anchors for dependent
                -- systems such as BuffFrame. Seed their layout state, but do
                -- not hand that impossible anchor to SetPoint during startup.
                replay_system_settings(systemFrame)
                refresh_system_layout_after_setting_replay(systemFrame)
            else
                pcall(emm.UpdateSystem, emm, systemFrame)
            end
        end

        emm.layoutApplyInProgress = false
    "#;

const FIX_ACTION_BAR_NAN_SIZE_LUA: &str = r#"
        if not MainActionBar then return end
        local w, h = MainActionBar:GetSize()
        if w == 562 and h == 45 then return end
        -- Compute the bar bounds from the actual button grid. Border art and
        -- end caps are anchored outside the frame; baking them into the frame
        -- size shifts the whole bar off-center.
        local lastOx = 0
        local buttonWidth = 45
        local buttonHeight = 45
        for i = 1, 12 do
            local c = _G["MainActionBarButtonContainer" .. i]
            local isShown = not c or not c.IsShown or c:IsShown()
            if c and isShown then
                local cw, ch = c:GetSize()
                if cw and cw == cw and cw > 0 then
                    buttonWidth = cw
                end
                if ch and ch == ch and ch > 0 then
                    buttonHeight = ch
                end
            end
            if c and isShown and c:GetNumPoints() > 0 then
                local point, _, _, ox, _ = c:GetPoint(1)
                if point == "BOTTOMLEFT" and ox and ox == ox and ox > lastOx then
                    lastOx = ox
                end
            end
        end
        MainActionBar:SetSize(lastOx + buttonWidth, buttonHeight)
    "#;

/// Initialize EditMode layout info and apply system anchors.
///
/// EDIT_MODE_LAYOUTS_UPDATED fires during startup but UpdateLayoutInfo
/// crashes partway through (cascading dependencies). This leaves
/// layoutInfo nil. Manually set it up from C_EditMode.GetLayouts() +
/// preset layouts, then call our custom InitSystemAnchors. Also ensures
/// accountSettings is initialized so CanEnterEditMode() returns true.
pub fn init_edit_mode_layout(env: &WowLuaEnv) {
    log_step(env, "setup_layout_info", || {
        setup_layout_info(env);
    });
    log_step(env, "apply_system_anchors", || {
        apply_system_anchors(env);
    });
    log_step(env, "fix_action_bar_nan_size", || {
        fix_action_bar_nan_size(env);
    });
    log_step(env, "fix_action_bar_scale", || {
        fix_action_bar_scale(env);
    });
}

/// Re-run the normal player-frame anchor path after startup events settle.
///
/// The player frame's edit-mode anchor path re-applies the cast bar anchor as
/// a side effect. Post-event startup leaves that path unapplied in some
/// headless/test flows, so explicitly replay it here instead of directly
/// attaching the cast bar.
pub fn reapply_player_frame_anchor(env: &WowLuaEnv) {
    log_step(env, "reapply_player_frame_anchor", || {
        let _ = env.exec(
            r#"
            if PlayerFrame and type(PlayerFrame.ApplySystemAnchor) == "function" then
                pcall(PlayerFrame.ApplySystemAnchor, PlayerFrame)
            end
        "#,
        );
        refresh_player_frame_state(env);
    });
}

/// Re-run the player frame refresh after edit-mode anchoring settles.
///
/// `ApplySystemAnchor` can leave the player frame's health bar bound to the
/// wrong unit. Replaying the normal art update and then re-seeding the bar
/// restores the live health binding without forcing a full art swap.
fn refresh_player_frame_state(env: &WowLuaEnv) {
    let _ = env.exec(
        r#"
        if PlayerFrame and type(PlayerFrame_UpdateArt) == "function" then
            pcall(PlayerFrame_UpdateArt, PlayerFrame)
        end
        local healthBar = PlayerFrame and PlayerFrame_GetHealthBar and PlayerFrame_GetHealthBar()
        if healthBar and type(UnitFrameHealthBar_SetUnit) == "function" then
            pcall(UnitFrameHealthBar_SetUnit, healthBar, "player")
        end
        if PlayerFrame and type(UnitFrame_Update) == "function" then
            pcall(UnitFrame_Update, PlayerFrame)
        end
        if healthBar and type(UnitFrameHealthBar_Update) == "function" then
            pcall(UnitFrameHealthBar_Update, healthBar, "player")
        end
    "#,
    );
}

/// Populate layoutInfo from C_EditMode.GetLayouts() + preset layouts.
fn setup_layout_info(env: &WowLuaEnv) {
    let _ = env.exec(SETUP_LAYOUT_INFO_LUA);
}

/// Apply preset layout anchors and settings to all EditMode system frames.
fn apply_system_anchors(env: &WowLuaEnv) {
    let _ = env.exec(APPLY_SYSTEM_ANCHORS_LUA);
}

/// Fix MainActionBar NaN size after UpdateSystems.
///
/// Layout() produces NaN because the bar has no size yet when children try
/// to resolve anchors relative to it (chicken-and-egg). Compute the bar
/// size directly from the button grid; saved EditMode anchors are applied
/// separately and should not be repacked by Blizzard's automatic bar layout.
fn fix_action_bar_nan_size(env: &WowLuaEnv) {
    let _ = env.exec(FIX_ACTION_BAR_NAN_SIZE_LUA);
}

/// Force MainActionBar scale=1 after EditMode initialization.
fn fix_action_bar_scale(env: &WowLuaEnv) {
    let _ = env.exec(
        r#"
        if MainActionBar then MainActionBar:SetScale(1) end
    "#,
    );
}

fn log_with_timestamp(env: &WowLuaEnv, message: &str) {
    let start_time = env.state().borrow().start_time;
    eprintln!("{} {}", crate::logging::elapsed_prefix(start_time), message);
}

fn log_step(env: &WowLuaEnv, label: &str, apply_step: impl FnOnce()) {
    log_with_timestamp(env, &format!("[EditMode] starting {label}"));
    let started = Instant::now();
    apply_step();
    log_with_timestamp(
        env,
        &format!("[EditMode] finished {label} in {:.2?}", started.elapsed()),
    );
}

/// Patch EditModeManagerFrame after addon loading.
///
/// Before EDIT_MODE_LAYOUTS_UPDATED fires, layoutInfo is nil. Guard
/// GetActiveLayoutInfo with a fallback. Replace InitSystemAnchors with a
/// custom implementation that reads the active preset layout, applies
/// anchorInfo to all 43 registered system frames, then calls UpdateSystems
/// to apply settings (orientation, num rows, etc.) through the normal
/// Blizzard code path. Per-frame errors are caught by secureexecuterange.
///
/// Also wraps EnterEditMode/ExitEditMode with pcall protection so edit
/// mode can activate even when subsystems crash.
pub fn patch_edit_mode_manager(env: &WowLuaEnv) {
    patch_apply_system_anchor_nil_guard(env);
    fix_set_point_override_3arg(env);
    guard_action_bar_limits(env);
    patch_default_anchor(env);
    patch_enter_exit_edit_mode(env);
}

#[cfg(test)]
#[path = "workarounds_editmode_tests.rs"]
mod tests;

/// Guard ApplySystemAnchor against nil systemInfo.
///
/// `EditModePlayerFrameSystemMixin:ApplySystemAnchor` calls
/// `PlayerCastingBarFrame:ApplySystemAnchor()` as a side effect, but
/// PlayerCastingBarFrame (and 13 other frames) have nil systemInfo because
/// the preset layout doesn't include entries for all system types.
/// The original Blizzard code at EditModeSystemTemplates.lua:375 accesses
/// `self.systemInfo.anchorInfo` without a nil check.
fn patch_apply_system_anchor_nil_guard(env: &WowLuaEnv) {
    let _ = env.exec(
        r#"
        if not EditModeSystemMixin then return end
        local orig = EditModeSystemMixin.ApplySystemAnchor
        function EditModeSystemMixin:ApplySystemAnchor()
            if not self.systemInfo then return end
            return orig(self)
        end
    "#,
    );
}

/// Fix SetPointOverride to handle the 3-arg SetPoint form.
///
/// Blizzard's `SetPointOverride(point, relativeTo, relativePoint, offsetX, offsetY)`
/// always forwards all 5 args to `SetPointBase`. But `VerticalLayoutMixin` and other
/// code calls the 3-arg form: `SetPoint("TOPRIGHT", x, y)`, where x,y are offsets
/// relative to the parent. In that case `relativeTo` receives a number (the x offset)
/// and `relativePoint` receives a number (the y offset), which is wrong.
///
/// OnSystemLoad already ran during addon loading, copying the original
/// SetPointOverride into each frame's fields table. We replace it on each
/// registered frame by writing a fixed version directly on the frame table.
fn fix_set_point_override_3arg(env: &WowLuaEnv) {
    let _ = env.exec(
        r#"
        if not EditModeManagerFrame then return end
        local emm = EditModeManagerFrame
        if not emm.registeredSystemFrames then return end
        for _, frame in ipairs(emm.registeredSystemFrames) do
            if rawget(frame, "SetPoint") then
                local base = rawget(frame, "SetPointBase") or frame.SetPointBase
                if base then
                    rawset(frame, "SetPoint", function(self, point, relativeTo, relativePoint, offsetX, offsetY)
                        if type(relativeTo) == "number" then
                            offsetX = relativeTo
                            offsetY = relativePoint
                            relativeTo = nil
                            relativePoint = nil
                        end
                        base(self, point, relativeTo, relativePoint, offsetX, offsetY)
                        if relativeTo then
                            pcall(self.SetSnappedToFrame, self, relativeTo)
                        end
                        pcall(EditModeManagerFrame.OnEditModeSystemAnchorChanged, EditModeManagerFrame)
                    end)
                end
            end
        end
    "#,
    );
}

/// Guard action bar positioning methods against nil frame positions.
///
/// `GetRightActionBarTopLimit` calls `MinimapCluster:GetBottom() - 10`, which
/// returns nil when MinimapCluster hasn't been laid out yet. Similarly,
/// `GetRightActionBarBottomLimit` calls `MicroButtonAndBagsBar:GetTop() + 24`.
/// During the real UpdateLayoutInfo (EDIT_MODE_LAYOUTS_UPDATED event), layout
/// hasn't run yet, so these frame positions are nil. Fall back to UIParent
/// boundaries when the frame position isn't available.
fn guard_action_bar_limits(env: &WowLuaEnv) {
    let _ = env.exec(
        r#"
        if not EditModeManagerFrameMixin then return end
        function EditModeManagerFrameMixin:GetRightActionBarTopLimit()
            if MinimapCluster and MinimapCluster.IsInDefaultPosition
                    and MinimapCluster:IsInDefaultPosition() then
                local bottom = MinimapCluster:GetBottom()
                if bottom then return bottom - 10 end
            end
            return UIParent:GetTop()
        end
        function EditModeManagerFrameMixin:GetRightActionBarBottomLimit()
            if MicroButtonAndBagsBar then
                local top = MicroButtonAndBagsBar:GetTop()
                if top then return top + 24 end
            end
            return 0
        end
    "#,
    );
}

/// Provide a fallback GetDefaultAnchor for frames that query it.
fn patch_default_anchor(env: &WowLuaEnv) {
    let _ = env.exec(
        r#"
        if not EditModeManagerFrame then return end
        function EditModeManagerFrame:GetDefaultAnchor(frame)
            return {
                point = "TOPRIGHT",
                relativeTo = UIParent,
                relativePoint = "TOPRIGHT",
                offsetX = -205,
                offsetY = -13,
            }
        end
    "#,
    );
}

const PATCH_ENTER_EXIT_EDIT_MODE_LUA: &str = r#"
    if not EditModeManagerFrame then return end
    local emm = EditModeManagerFrame

    function emm:EnterEditMode()
        self.editModeActive = true
        pcall(self.ClearActiveChangesFlags, self)
        pcall(self.UpdateDropdownOptions, self)
        pcall(self.ShowSystemSelections, self)
        if self.AccountSettings
            and self.AccountSettings.OnEditModeEnter then
            pcall(
                self.AccountSettings.OnEditModeEnter,
                self.AccountSettings
            )
        end
        pcall(EventRegistry.TriggerEvent,
            EventRegistry, "EditMode.Enter")
    end

    function emm:ExitEditMode()
        self.editModeActive = false
        pcall(self.ClearSelectedSystem, self)
        pcall(function()
            secureexecuterange(
                self.registeredSystemFrames,
                function(_, f)
                    if f.OnEditModeExit then
                        pcall(f.OnEditModeExit, f)
                    end
                end
            )
        end)
        if self.AccountSettings
            and self.AccountSettings.OnEditModeExit then
            pcall(
                self.AccountSettings.OnEditModeExit,
                self.AccountSettings
            )
        end
        pcall(EventRegistry.TriggerEvent,
            EventRegistry, "EditMode.Exit")
    end
"#;

/// Wrap EnterEditMode/ExitEditMode with pcall protection.
///
/// EnterEditMode calls crash-prone functions: ShowSystemSelections
/// iterates 43 frames, AccountSettings does 30+ Setup/Refresh calls.
/// Wrapping each step with pcall lets edit mode activate even when
/// individual subsystems fail.
fn patch_enter_exit_edit_mode(env: &WowLuaEnv) {
    let _ = env.exec(PATCH_ENTER_EXIT_EDIT_MODE_LUA);
}
