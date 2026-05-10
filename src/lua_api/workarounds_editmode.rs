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

    local function forceCastBarUnderPlayerFrame(layoutInfo)
        if not layoutInfo or not layoutInfo.layouts then return end
        local activeLayout = layoutInfo.layouts[layoutInfo.activeLayout]
        if not activeLayout or not activeLayout.systems then return end
        for _, systemInfo in ipairs(activeLayout.systems) do
            if systemInfo.system == Enum.EditModeSystem.CastBar then
                setSystemSetting(systemInfo, Enum.EditModeCastBarSetting.LockToPlayerFrame, 1)
            elseif systemInfo.system == Enum.EditModeSystem.UnitFrame
                and systemInfo.systemIndex == Enum.EditModeUnitFrameSystemIndices.Player then
                setSystemSetting(systemInfo, Enum.EditModeUnitFrameSetting.CastBarUnderneath, 1)
            end
        end
    end

    local function forceStandardPartyFrames(layoutInfo)
        if not layoutInfo or not layoutInfo.layouts then return end
        for _, preset in ipairs(layoutInfo.layouts) do
            if type(preset) == "table" and preset.systems then
                for _, systemInfo in ipairs(preset.systems) do
                    if systemInfo.system == Enum.EditModeSystem.UnitFrame
                        and systemInfo.systemIndex == Enum.EditModeUnitFrameSystemIndices.Party then
                        setSystemSetting(systemInfo, Enum.EditModeUnitFrameSetting.UseRaidStylePartyFrames, 0)
                    end
                end
            end
        end
    end

    if not EditModeManagerFrame then return end
    local emm = EditModeManagerFrame
    if not emm.layoutInfo then
        local layoutInfo = C_EditMode.GetLayouts()
        emm.layoutInfo = layoutInfo
        local savedLayouts = copyLayouts(emm.layoutInfo.layouts)
        emm.layoutInfo.layouts = copyLayouts(EditModePresetLayoutManager.presetLayoutInfo)
        tAppendAll(emm.layoutInfo.layouts, savedLayouts)
    end
    forceCastBarUnderPlayerFrame(emm.layoutInfo)
    forceStandardPartyFrames(emm.layoutInfo)
    if not emm.accountSettings then
        emm.accountSettings = C_EditMode.GetAccountSettings()
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

        local function seed_system_frame(systemFrame)
            local systemInfo = emm:GetActiveLayoutSystemInfo(systemFrame.system, systemFrame.systemIndex)
            if not systemInfo then
                return
            end

            systemFrame.savedSystemInfo = CopyTable(systemInfo)
            systemFrame.systemInfo = systemInfo
            systemFrame:SetHasActiveChanges(false)
            systemFrame:UpdateSettingMap(true)
        end

        local function refresh_action_bar_system(systemFrame)
            local systemInfo = systemFrame.systemInfo
            -- Replay the action-bar setting handlers without the full
            -- EditMode frame update path.
            for _, settingInfo in ipairs(systemInfo and systemInfo.settings or {}) do
                if systemFrame.UpdateSystemSetting then
                    pcall(
                        systemFrame.UpdateSystemSetting,
                        systemFrame,
                        settingInfo.setting,
                        true
                    )
                end
            end
            if systemFrame.RefreshGridLayout then
                systemFrame:RefreshGridLayout()
            end
            if systemFrame.RefreshDividers then
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
                -- default-position layout pass to run, and let the normal bar
                -- systems own their runtime layout afterward.
                seed_system_frame(systemFrame)
                refresh_action_bar_system(systemFrame)
            elseif skips_expensive_startup_system_update(systemFrame) then
                seed_system_frame(systemFrame)
                if systemFrame.ApplySystemAnchor then
                    pcall(systemFrame.ApplySystemAnchor, systemFrame)
                end
            else
                pcall(emm.UpdateSystem, emm, systemFrame)
            end
        end

        emm.layoutApplyInProgress = false
        pcall(emm.UpdateActionBarPositions, emm)
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
            if c then
                local cw, ch = c:GetSize()
                if cw and cw == cw and cw > 0 then
                    buttonWidth = cw
                end
                if ch and ch == ch and ch > 0 then
                    buttonHeight = ch
                end
            end
            if c and c:GetNumPoints() > 0 then
                local _, _, _, ox, _ = c:GetPoint(1)
                if ox and ox == ox then lastOx = ox end
            end
        end
        MainActionBar:SetSize(lastOx + buttonWidth, buttonHeight)
        if EditModeManagerFrame and type(EditModeManagerFrame.UpdateActionBarPositions) == "function" then
            pcall(EditModeManagerFrame.UpdateActionBarPositions,
                  EditModeManagerFrame)
        end
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
/// size directly from the button grid, then re-run
/// UpdateActionBarPositions to set the correct BOTTOMLEFT anchor.
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
mod tests {
    use super::{FIX_ACTION_BAR_NAN_SIZE_LUA, SETUP_LAYOUT_INFO_LUA, WowLuaEnv};

    #[test]
    fn setup_layout_info_clones_preset_layouts_without_copytable() {
        let env = WowLuaEnv::new().expect("Failed to create Lua environment");
        env.exec(
            r#"
            Enum = {
                EditModeSystem = {
                    CastBar = 1,
                    UnitFrame = 2,
                },
                EditModeCastBarSetting = {
                    LockToPlayerFrame = 101,
                },
                EditModeUnitFrameSetting = {
                    CastBarUnderneath = 201,
                },
                EditModeUnitFrameSystemIndices = {
                    Player = 301,
                },
            }

            C_EditMode = {
                GetLayouts = function()
                    return {
                        layouts = {
                            {
                                layoutIndex = 99,
                                layoutName = "Saved",
                                layoutType = 2,
                                systems = {
                                    {
                                        system = 77,
                                        systemIndex = 88,
                                        isInDefaultPosition = false,
                                        anchorInfo = { point = "BOTTOM" },
                                        settings = {
                                            { setting = 501, value = 601 },
                                        },
                                    },
                                },
                            },
                        },
                        activeLayout = 1,
                    }
                end,
                GetAccountSettings = function()
                    return {
                        { setting = 1, value = 0 },
                    }
                end,
            }

            EditModePresetLayoutManager = {
                presetLayoutInfo = {
                    {
                        layoutIndex = 1,
                        layoutName = "Preset",
                        layoutType = 0,
                        systems = {
                            {
                                system = Enum.EditModeSystem.CastBar,
                                systemIndex = 1,
                                isInDefaultPosition = true,
                                anchorInfo = { point = "TOP" },
                                settings = {
                                    { setting = Enum.EditModeCastBarSetting.LockToPlayerFrame, value = 0 },
                                },
                            },
                            {
                                system = Enum.EditModeSystem.UnitFrame,
                                systemIndex = Enum.EditModeUnitFrameSystemIndices.Player,
                                isInDefaultPosition = true,
                                anchorInfo = { point = "LEFT" },
                                settings = {
                                    { setting = Enum.EditModeUnitFrameSetting.CastBarUnderneath, value = 0 },
                                },
                            },
                        },
                    },
                },
            }

            function tAppendAll(tbl, addedArray)
                for i, element in ipairs(addedArray) do
                    table.insert(tbl, element)
                end
            end

            EditModeManagerFrame = {}
            "#,
        )
        .expect("install edit mode stubs");

        env.exec(SETUP_LAYOUT_INFO_LUA)
            .expect("run setup layout info");

        let (layout_count, lock_to_player, cast_bar_underneath, saved_layout_name): (
            i32,
            i32,
            i32,
            String,
        ) = env
            .eval(
                r#"
                local layouts = EditModeManagerFrame.layoutInfo.layouts
                local presetSystems = layouts[1].systems
                local savedLayout = layouts[2]
                return #layouts,
                    presetSystems[1].settings[1].value,
                    presetSystems[2].settings[1].value,
                    savedLayout.layoutName
                "#,
            )
            .expect("read cloned layout info");

        assert_eq!(
            layout_count, 2,
            "preset and saved layouts should both be present"
        );
        assert_eq!(
            lock_to_player, 1,
            "cast bar setting should be forced on cloned preset"
        );
        assert_eq!(
            cast_bar_underneath, 1,
            "player frame setting should be forced on cloned preset"
        );
        assert_eq!(
            saved_layout_name, "Saved",
            "saved layouts should be appended"
        );

        env.exec(
            r#"
            EditModePresetLayoutManager.presetLayoutInfo[1].systems[1].settings[1].value = 999
            EditModePresetLayoutManager.presetLayoutInfo[1].systems[1].anchorInfo.point = "BROKEN"
            "#,
        )
        .expect("mutate original preset");

        let (copied_value, copied_point): (i32, String) = env
            .eval(
                r#"
                local system = EditModeManagerFrame.layoutInfo.layouts[1].systems[1]
                return system.settings[1].value, system.anchorInfo.point
                "#,
            )
            .expect("read cloned preset after source mutation");

        assert_eq!(
            copied_value, 1,
            "cloned settings must not alias preset source"
        );
        assert_eq!(
            copied_point, "TOP",
            "cloned anchor info must not alias preset source"
        );
    }

    #[test]
    fn fix_action_bar_size_skips_missing_update_action_bar_positions() {
        let env = WowLuaEnv::new().expect("Failed to create Lua environment");
        env.exec(
            r#"
            MainActionBar = CreateFrame("Frame", "TestMainActionBar", UIParent)
            function MainActionBar:GetSize() return 0, 0 end
            function MainActionBar:SetSize(width, height)
                self.fixedHeight = height
            end
            for i = 1, 12 do
                local button = CreateFrame("Frame", "MainActionBarButtonContainer" .. i, MainActionBar)
                function button:GetSize() return 45, 45 end
                function button:GetNumPoints() return 1 end
                function button:GetPoint() return "LEFT", nil, "LEFT", (i - 1) * 45, 0 end
            end
            EditModeManagerFrame = {}
            "#,
        )
        .expect("install action bar stubs");

        let before = env.state().borrow().lua_errors.len();
        env.exec(FIX_ACTION_BAR_NAN_SIZE_LUA)
            .expect("fix action bar size should not call a missing updater");
        let after = env.state().borrow().lua_errors.len();

        assert_eq!(after, before);
    }
}

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
