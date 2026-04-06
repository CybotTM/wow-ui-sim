//! EditMode layout workarounds.
//!
//! Patches EditModeManagerFrame to apply preset layout anchors to all 43
//! registered system frames. The real UpdateLayoutInfo crashes partway through
//! due to cascading dependencies, so we manually set up layoutInfo and call
//! our custom InitSystemAnchors.

use super::WowLuaEnv;

/// Initialize EditMode layout info and apply system anchors.
///
/// EDIT_MODE_LAYOUTS_UPDATED fires during startup but UpdateLayoutInfo
/// crashes partway through (cascading dependencies). This leaves
/// layoutInfo nil. Manually set it up from C_EditMode.GetLayouts() +
/// preset layouts, then call our custom InitSystemAnchors. Also ensures
/// accountSettings is initialized so CanEnterEditMode() returns true.
pub fn init_edit_mode_layout(env: &WowLuaEnv) {
    setup_layout_info(env);
    apply_system_anchors(env);
    fix_action_bar_nan_size(env);
    fix_action_bar_scale(env);
    register_override_helpers(env);
    clear_edit_mode_overrides(env);
    reposition_managed_frames(env);
}

/// Register Lua helper functions used by clear_edit_mode_overrides.
fn register_override_helpers(env: &WowLuaEnv) {
    let _ = env.exec(CLEAR_FRAME_OVERRIDES_FN);
    let _ = env.exec(REAPPLY_PRESET_ANCHOR_FN);
}

/// Populate layoutInfo from C_EditMode.GetLayouts() + preset layouts.
fn setup_layout_info(env: &WowLuaEnv) {
    let _ = env.exec(
        r#"
        if not EditModeManagerFrame then return end
        local emm = EditModeManagerFrame
        if emm.layoutInfo then return end
        local layoutInfo = C_EditMode.GetLayouts()
        emm.layoutInfo = layoutInfo
        local savedLayouts = emm.layoutInfo.layouts
        emm.layoutInfo.layouts = EditModePresetLayoutManager:GetCopyOfPresetLayouts()
        tAppendAll(emm.layoutInfo.layouts, savedLayouts)
        if not emm.accountSettings then
            emm.accountSettings = C_EditMode.GetAccountSettings()
        end
    "#,
    );
}

/// Apply preset layout anchors and settings to all EditMode system frames.
fn apply_system_anchors(env: &WowLuaEnv) {
    let _ = env.exec(
        r#"
        if not EditModeManagerFrame then return end
        local emm = EditModeManagerFrame
        if not emm.layoutInfo then return end
        emm.layoutApplyInProgress = true
        emm:InitSystemAnchors()
        pcall(emm.UpdateSystems, emm)
        emm.layoutApplyInProgress = false
        pcall(emm.UpdateActionBarPositions, emm)
    "#,
    );
}

/// Fix MainActionBar NaN size after UpdateSystems.
///
/// Layout() produces NaN because the bar has no size yet when children try
/// to resolve anchors relative to it (chicken-and-egg). Compute the bar
/// size directly from children's grid positions, then re-run
/// UpdateActionBarPositions to set the correct BOTTOMLEFT anchor.
fn fix_action_bar_nan_size(env: &WowLuaEnv) {
    let _ = env.exec(
        r#"
        if not MainActionBar then return end
        local w = MainActionBar:GetWidth()
        if w == w then return end  -- not NaN, nothing to fix
        -- Compute width from button containers only (12 slots, 45px each
        -- with 2px gap = 47px stride). Last container offset + width.
        local lastOx, lastW = 0, 45
        for i = 1, 12 do
            local c = _G["MainActionBarButtonContainer" .. i]
            if c and c:GetNumPoints() > 0 then
                local _, _, _, ox, _ = c:GetPoint(1)
                if ox and ox == ox then lastOx = ox end
            end
        end
        MainActionBar:SetSize(lastOx + lastW, lastW)
        pcall(EditModeManagerFrame.UpdateActionBarPositions,
              EditModeManagerFrame)
    "#,
    );
}

/// Force MainActionBar scale=1 after EditMode initialization.
fn fix_action_bar_scale(env: &WowLuaEnv) {
    let _ = env.exec(
        r#"
        if MainActionBar then MainActionBar:SetScale(1) end
    "#,
    );
}

/// Clear EditMode method overrides and re-apply preset anchors for non-managed frames.
///
/// `OnSystemLoad` (line 10-17 of EditModeSystemTemplates.lua) replaces SetScale,
/// SetPoint, and ClearAllPoints with Override versions that adjust offsets and
/// track snapped frames. Since `__index` checks Lua fields before Rust methods
/// (commit 73e6032), these overrides intercept all calls:
///
/// - `SetScaleOverride` adjusts anchor offsets via `offset * oldScale / newScale`,
///   shifting FocusFrame x by +130px.
/// - `SetPointOverride` expects 5 explicit args; when VerticalLayoutMixin calls
///   the 3-arg `SetPoint("TOPRIGHT", x, y)`, the numbers are misinterpreted as
///   relativeTo/relativePoint and offsets default to 0.
///
/// Fix: clear all overrides from fenv, then re-apply preset anchors via the
/// Rust methods for non-managed frames. Managed frames get their position from
/// the container layout (Layout → SetPoint), which requires the Rust method.
fn clear_edit_mode_overrides(env: &WowLuaEnv) {
    let _ = env.exec(CLEAR_OVERRIDES_LUA);
}

const CLEAR_OVERRIDES_LUA: &str = r#"
    if not EditModeManagerFrame then return end
    local emm = EditModeManagerFrame
    local activeLayout = emm:GetActiveLayoutInfo()
    if not activeLayout or not activeLayout.systems then return end
    local lookup = {}
    for _, sysInfo in ipairs(activeLayout.systems) do
        local key = tostring(sysInfo.system) .. ":" .. tostring(sysInfo.systemIndex or 0)
        lookup[key] = sysInfo
    end
    for _, frame in ipairs(emm.registeredSystemFrames) do
        clear_frame_overrides(frame)
        if not frame.isManagedFrame then
            reapply_preset_anchor(frame, lookup)
        end
    end
"#;

/// Lua helper: clear SetPoint/SetScale/ClearAllPoints overrides from a frame's fenv.
/// `rawset(userdata, ...)` fails, so we access the per-frame fields via
/// `debug.getfenv(frame)[1]` which is the table checked by `__index`.
const CLEAR_FRAME_OVERRIDES_FN: &str = r#"
    function clear_frame_overrides(frame)
        local env = debug.getfenv(frame)
        if not env or not env[1] then return end
        local t = env[1]
        rawset(t, "SetPoint", nil)
        rawset(t, "SetScale", nil)
        rawset(t, "ClearAllPoints", nil)
    end
"#;

/// Lua helper: re-apply a frame's preset anchor from the active layout.
const REAPPLY_PRESET_ANCHOR_FN: &str = r#"
    function reapply_preset_anchor(frame, lookup)
        local key = tostring(frame.system) .. ":" .. tostring(frame.systemIndex or 0)
        local sysInfo = lookup[key]
        if not sysInfo or not sysInfo.anchorInfo then return end
        local a = sysInfo.anchorInfo
        local rel = a.relativeTo
        if type(rel) == "string" then rel = _G[rel] or rel end
        frame:ClearAllPoints()
        frame:SetPoint(a.point, rel, a.relativePoint, a.offsetX, a.offsetY)
    end
"#;

/// Re-run managed frame container layouts after edit mode initialization.
///
/// The VerticalLayoutMixin positions children via `SetPoint("TOPRIGHT", x, y)`,
/// but the 3-arg SetPoint form silently drops offsets for some frames (likely
/// related to frame re-creation). Using the explicit 5-arg `SetPoint` with the
/// container as relativeTo works correctly. Re-run UpdateManagedFrames which
/// triggers Layout() to reposition all managed children.
fn reposition_managed_frames(env: &WowLuaEnv) {
    position_right_managed_container(env);
    position_bottom_managed_container(env);
}

/// Position UIParentRightManagedFrameContainer using EditMode default offsets.
fn position_right_managed_container(env: &WowLuaEnv) {
    let _ = env.exec(
        r#"
        local f = UIParentRightManagedFrameContainer
        if not f then return end
        f:ClearAllPoints()
        f:SetPoint("TOPRIGHT", UIParent, "TOPRIGHT", -5, -260)
        local minimapHeight = 0
        if MinimapCluster and MinimapCluster.GetHeight then
            minimapHeight = MinimapCluster:GetHeight()
        end
        f.fixedHeight = UIParent:GetHeight() - minimapHeight - 100
        f:Layout()
        if f.BottomManagedLayoutContainer then
            f.BottomManagedLayoutContainer:Layout()
        end
        f:UpdateManagedFrames()
    "#,
    );
}

/// Position UIParentBottomManagedFrameContainer at screen bottom center.
fn position_bottom_managed_container(env: &WowLuaEnv) {
    let _ = env.exec(
        r#"
        local f = UIParentBottomManagedFrameContainer
        if not f then return end
        f.fixedWidth = 573
        f:ClearAllPoints()
        f:SetPoint("BOTTOM", UIParent, "BOTTOM", 0, 90)
        f:Layout()
        if f.BottomManagedLayoutContainer then
            f.BottomManagedLayoutContainer:Layout()
        end
        f:UpdateManagedFrames()
    "#,
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
    patch_get_active_layout(env);
    patch_get_setting_value(env);
    patch_apply_system_anchor_nil_guard(env);
    patch_init_anchors(env);
    patch_update_systems(env);
    patch_update_layout_info(env);
    patch_default_anchor(env);
    patch_enter_exit_edit_mode(env);
}

/// Guard GetActiveLayoutInfo against nil layoutInfo (pre-event calls).
fn patch_get_active_layout(env: &WowLuaEnv) {
    let _ = env.exec(
        r#"
        if not EditModeManagerFrame then return end
        local emm = EditModeManagerFrame
        local origGetActiveLayoutInfo = emm.GetActiveLayoutInfo
        function emm:GetActiveLayoutInfo()
            if not self.layoutInfo then
                return { layoutType = 0, layoutIndex = 1, systems = {} }
            end
            return origGetActiveLayoutInfo(self)
        end
    "#,
    );
}

/// Guard GetSettingValue against nil settingMap.
///
/// During startup events (VARIABLES_LOADED, PLAYER_ENTERING_WORLD), frames
/// may have systemInfo set (IsInitialized() → true) but settingMap not yet
/// populated (init_edit_mode_layout runs post-events). Return 0 when
/// settingMap is nil or missing the requested setting, matching the
/// IsInitialized() fallback behavior.
fn patch_get_setting_value(env: &WowLuaEnv) {
    let _ = env.exec(
        r#"
        if not EditModeSystemMixin then return end
        function EditModeSystemMixin:GetSettingValue(setting, useRawValue)
            if not self:IsInitialized() then
                return 0
            end
            local compositeNumberValue = self:GetCompositeNumberSettingValue(setting, useRawValue)
            if compositeNumberValue ~= nil then
                return compositeNumberValue
            end
            if not self.settingMap or not self.settingMap[setting] then
                return 0
            end
            if useRawValue then
                return self.settingMap[setting].value
            else
                return self.settingMap[setting].displayValue or self.settingMap[setting].value
            end
        end
    "#,
    );
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

/// Custom InitSystemAnchors: apply preset layout anchors to non-managed frames.
///
/// Skips managed frames (OTF, etc.) whose position comes from the container
/// layout system. UpdateSystems is called after this to apply settings.
fn patch_init_anchors(env: &WowLuaEnv) {
    let _ = env.exec(INIT_SYSTEM_ANCHORS_LUA);
}

const INIT_SYSTEM_ANCHORS_LUA: &str = r#"
    if not EditModeManagerFrame then return end
    local emm = EditModeManagerFrame
    function emm:InitSystemAnchors()
        local activeLayout = self:GetActiveLayoutInfo()
        if not activeLayout or not activeLayout.systems then return end
        local lookup = {}
        for _, sysInfo in ipairs(activeLayout.systems) do
            local idx = sysInfo.systemIndex or 0
            lookup[tostring(sysInfo.system) .. ":" .. tostring(idx)] = sysInfo
        end
        for _, frame in ipairs(self.registeredSystemFrames) do
            if not frame.isManagedFrame then
                local idx = frame.systemIndex or 0
                local sysInfo = lookup[tostring(frame.system) .. ":" .. tostring(idx)]
                if sysInfo and sysInfo.anchorInfo then
                    frame:ClearAllPoints()
                    local a = sysInfo.anchorInfo
                    local rel = a.relativeTo
                    if type(rel) == "string" then rel = _G[rel] or rel end
                    frame:SetPoint(a.point, rel, a.relativePoint, a.offsetX, a.offsetY)
                end
            end
        end
    end
"#;

/// Implement UpdateSystems to call UpdateSystem on each registered frame.
///
/// The real WoW UpdateSystems uses secureexecuterange which swallows
/// per-frame errors. We replicate that: look up each frame's systemInfo
/// from the active layout and call frame:UpdateSystem(sysInfo). Frames
/// that crash (missing subsystems, etc.) are caught individually.
fn patch_update_systems(env: &WowLuaEnv) {
    let _ = env.exec(
        r#"
        if not EditModeManagerFrame then return end
        function EditModeManagerFrame:UpdateSystems()
            local function callUpdateSystem(index, systemFrame)
                if systemFrame.isManagedFrame then return end
                local sysInfo = self:GetActiveLayoutSystemInfo(
                    systemFrame.system, systemFrame.systemIndex
                )
                if sysInfo then
                    systemFrame:UpdateSystem(sysInfo)
                end
            end
            secureexecuterange(self.registeredSystemFrames, callUpdateSystem)
        end
    "#,
    );
}

/// Override UpdateLayoutInfo to prevent double initialization.
///
/// The EDIT_MODE_LAYOUTS_UPDATED event handler in EditModeManager.lua:178
/// calls UpdateLayoutInfo after init_edit_mode_layout already ran the full
/// InitSystemAnchors + UpdateSystems + UpdateActionBarPositions flow. The
/// second pass produces NaN coordinates because Layout() → GetExtents()
/// fails on children that haven't been sized yet in the live GUI context.
/// Replace with a no-op since init_edit_mode_layout handles everything.
fn patch_update_layout_info(env: &WowLuaEnv) {
    let _ = env.exec(
        r#"
        if not EditModeManagerFrame then return end
        function EditModeManagerFrame:UpdateLayoutInfo() end
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
