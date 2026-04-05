//! Post-load Lua workarounds for Blizzard code that depends on
//! unimplemented engine features (AnimationGroups, EditMode, etc.).

use super::workarounds_bags;
use super::workarounds_editmode;
use super::{SimState, WowLuaEnv};
use std::cell::RefCell;
use std::rc::Rc;

/// Apply workarounds that must run after startup events.
///
/// Some workarounds (like BagsBar anchoring) get undone by event handlers
/// (e.g. EDIT_MODE_LAYOUTS_UPDATED repositions managed frames).
pub fn apply_post_event(env: &WowLuaEnv) {
    workarounds_bags::fix_bags_bar_anchor(env);
    workarounds_bags::fix_bag_item_context_overlay(env);
    workarounds_editmode::init_edit_mode_layout(env);
    super::workarounds_tracker::fire_quest_callbacks(env);
    hide_talent_loadout_dialogs(env);
    suppress_spellbook_tutorials(env);
}

/// Apply targeted cleanup after a load-on-demand addon finishes loading.
pub fn apply_post_runtime_addon_load(env: &WowLuaEnv, addon_name: &str) {
    if addon_name == "Blizzard_PlayerSpells" {
        hide_talent_loadout_dialogs(env);
        suppress_spellbook_tutorials(env);
    }
}

pub fn apply_post_runtime_addon_load_from_lua(
    lua: &mlua::Lua,
    state: Rc<RefCell<SimState>>,
    addon_name: &str,
) {
    let env = WowLuaEnv {
        lua: lua.clone(),
        state,
    };
    apply_post_runtime_addon_load(&env, addon_name);
}

/// Apply all post-load workarounds. Called after addon loading, before events.
pub fn apply(env: &WowLuaEnv) {
    let _ = env.exec("UpdateMicroButtons = function() end");
    // CompactUnitFrame helpers may not be defined if CompactUnitFrame.lua fails to load fully
    let _ = env.exec(
        "if not CompactUnitFrame_GetOptionDisplayOnlyDispellableDebuffs then \
         CompactUnitFrame_GetOptionDisplayOnlyDispellableDebuffs = function() return false end end",
    );
    patch_map_canvas_scroll(env);
    patch_gradual_animated_status_bar(env);
    patch_spell_alert_animations(env);
    patch_character_frame_subframes(env);
    super::workarounds_tracker::init_objective_tracker(env);
    super::chat_init::show_chat_frame(env);
    workarounds_bags::init_bag_bar(env);
    workarounds_bags::init_bag_token_tracker(env);
    hide_super_tracked_frame(env);
    super::chat_init::init_chat_type_colors(env);
    workarounds_editmode::patch_edit_mode_manager(env);
    patch_compact_raid_container_pools(env);
    stub_arena_globals(env);
    stub_glow_and_lfg(env);
    patch_lfg_backfill(env);
    init_console_saved_vars(env);
    init_lfg_events_in_background(env);
    patch_scrollbox_nil_dataprovider(env);
    init_settings_panel_previews(env);
    patch_character_create_arrays(env);
}

/// SuperTrackedFrame shows a quest navigation arrow positioned by the engine's
/// 3D-to-screen projection (C_SuperTrack). Without the 3D world, OnUpdate never
/// repositions it, so the icon renders at default (0,0) in the top-left corner.
fn hide_super_tracked_frame(env: &WowLuaEnv) {
    let _ = env.exec("if SuperTrackedFrame then SuperTrackedFrame:Hide() end");
}

/// Blizzard's class talent loadout dialogs are hidden XML popups that should
/// remain closed until their dropdown actions explicitly open them. Normalize
/// them after startup events in case intermediate handlers surfaced them.
fn hide_talent_loadout_dialogs(env: &WowLuaEnv) {
    let _ = env.exec(
        r#"
        for _, name in ipairs({
            "ClassTalentLoadoutCreateDialog",
            "ClassTalentLoadoutEditDialog",
            "ClassTalentLoadoutImportDialog",
        }) do
            local frame = _G[name]
            if frame then
                frame:Hide()
            end
        end
    "#,
    );
}

/// Spellbook helptips are transient overlays that currently produce unstable
/// cold-open rendering in the simulator. Suppress them so the spellbook's
/// first visible state matches subsequent opens without faking world-exit.
fn suppress_spellbook_tutorials(env: &WowLuaEnv) {
    let _ = env.exec(
        r#"
        if SpellBookFrameTutorialsMixin then
            SpellBookFrameTutorialsMixin.CheckShowHelpTips = function() end
        end
        if HelpTip then
            HelpTip:HideAllSystem("SpellBook Helptips")
        end
    "#,
    );
}

/// SettingsDefinitions_Shared registers preview handlers even on glue/login
/// screens, but the full game-only Settings panel XML is not loaded there.
/// Reattach minimal preview objects after addon loading so those registrants
/// can safely call into SettingsPanel preview hooks.
fn init_settings_panel_previews(env: &WowLuaEnv) {
    let _ = env.exec(
        r#"
        if SettingsPanel then
            local function CreatePreviewStub()
                local preview = {
                    TitleText = { SetFontHeight = function() end },
                    BodyText = { SetFontHeight = function() end },
                }

                function preview:RegisterWithSettingInitializer() end
                function preview:SetValueAccessor() end
                function preview:UpdatePreview() end
                function preview:Layout() end

                return preview
            end

            SettingsPanel.AccessibilityFontPreview = SettingsPanel.AccessibilityFontPreview or CreatePreviewStub()
            SettingsPanel.QuestTextPreview = SettingsPanel.QuestTextPreview or CreatePreviewStub()
        end
    "#,
    );
}

/// CharacterCreate uses XML `parentArray="BGTex"` for its vignette textures.
/// Rebuild that array defensively until glue-screen parentArray wiring is
/// consistently available during this screen transition.
fn patch_character_create_arrays(env: &WowLuaEnv) {
    let _ = env.exec(
        r#"
        if CharacterCreateFrame and not CharacterCreateFrame.BGTex then
            CharacterCreateFrame.BGTex = {
                CharacterCreateFrame.TopBackgroundOverlay,
                CharacterCreateFrame.LeftBackgroundOverlay,
                CharacterCreateFrame.RightBackgroundOverlay,
                CharacterCreateFrame.BottomBackgroundOverlay,
            }
        end
    "#,
    );
}

/// MapCanvasScrollControllerMixin:IsZoomingOut/In compare targetScale with
/// GetCanvasScale(), but OnUpdate fires before CalculateScaleExtents sets
/// targetScale. Initialize it on the WorldMapFrame scroll container.
fn patch_map_canvas_scroll(env: &WowLuaEnv) {
    let _ = env.exec(
        r#"
        if WorldMapFrame and WorldMapFrame.ScrollContainer then
            local sc = WorldMapFrame.ScrollContainer
            sc.targetScale = sc.targetScale or 1
            sc.currentScale = sc.currentScale or 1
            sc.zoomLevels = sc.zoomLevels or {{ scale = 1 }}
        end
    "#,
    );
}

/// GradualAnimatedStatusBarTemplate XML defines an AnimationGroup with
/// parentKey="LevelUpMaxAlphaAnimation", but the simulator doesn't create
/// AnimationGroups from templates. Patch existing instances and the mixin.
fn patch_gradual_animated_status_bar(env: &WowLuaEnv) {
    let _ = env.exec(
        r#"
        local stub = { IsPlaying = function() return false end }

        if StatusTrackingBarManager and StatusTrackingBarManager.barContainers then
            for _, container in ipairs(StatusTrackingBarManager.barContainers) do
                for _, bar in pairs(container.bars or {}) do
                    if bar.StatusBar then
                        if not bar.StatusBar.LevelUpMaxAlphaAnimation then
                            bar.StatusBar.LevelUpMaxAlphaAnimation = stub
                        end
                    end
                end
            end
        end

        if GradualAnimatedStatusBarMixin then
            function GradualAnimatedStatusBarMixin:IsAnimating()
                return self.targetValue and self:GetValue() < self.targetValue
                    or self.gainFinishedAnimation and self.gainFinishedAnimation:IsPlaying()
                    or self.LevelUpMaxAlphaAnimation and self.LevelUpMaxAlphaAnimation:IsPlaying()
                    or self.overrideLevelUpMaxAlphaAnimation and self.overrideLevelUpMaxAlphaAnimation:IsPlaying()
            end
        end
    "#,
    );
}

/// Stub AnimationGroup methods on ActionButtonSpellAlert frames.
///
/// ActionButtonSpellAlertManager uses local functions (ShowAlert/HideAlert)
/// that access alertFrame.ProcStartAnim, an AnimationGroup defined in XML
/// with parentKey. The simulator doesn't create AnimationGroups from templates,
/// so these are nil. Replace the manager methods with no-ops since spell alert
/// animations aren't needed in the simulator.
fn patch_spell_alert_animations(env: &WowLuaEnv) {
    let _ = env.exec(
        r#"
        if ActionButtonSpellAlertManager then
            function ActionButtonSpellAlertManager:ShowAlert() end
            function ActionButtonSpellAlertManager:HideAlert() end
        end
    "#,
    );
}

/// CHARACTERFRAME_SUBFRAMES lists PaperDollFrame, ReputationFrame, TokenFrame.
/// TokenFrame is in Blizzard_TokenUI (not always loaded). Create stub frames
/// for any missing subframes so ShowSubFrame doesn't crash.
fn patch_character_frame_subframes(env: &WowLuaEnv) {
    let _ = env.exec(
        r#"
        if CHARACTERFRAME_SUBFRAMES then
            for _, name in ipairs(CHARACTERFRAME_SUBFRAMES) do
                if not _G[name] then
                    _G[name] = CreateFrame("Frame", name, CharacterFrame)
                    _G[name]:Hide()
                end
            end
        end
    "#,
    );
}

/// CompactRaidFrameContainer.dividerVerticalPool/dividerHorizontalPool are
/// initialized in CompactRaidFrameManager_OnLoad, which may fail before
/// reaching the pool creation code. Create stub pools so event handlers
/// that call ReleaseAll() don't error.
fn patch_compact_raid_container_pools(env: &WowLuaEnv) {
    let _ = env.exec(
        r#"
        if CompactRaidFrameContainer then
            local c = CompactRaidFrameContainer
            local stubPool = { ReleaseAll = function() end, Acquire = function() end }
            if not c.dividerVerticalPool then
                c.dividerVerticalPool = stubPool
            end
            if not c.dividerHorizontalPool then
                c.dividerHorizontalPool = stubPool
            end
        end
    "#,
    );
}

/// GetArenaOpponentSpec is a C function in WoW that returns spec info for
/// arena opponents. Stub it so CompactArenaFrame OnLoad doesn't crash.
fn stub_arena_globals(env: &WowLuaEnv) {
    let _ = env.exec(
        r#"
        if not GetArenaOpponentSpec then
            GetArenaOpponentSpec = function() return 0 end
        end
    "#,
    );
}

/// Stub globals used by spellbook/talents but defined in load-order-dependent addons.
fn stub_glow_and_lfg(env: &WowLuaEnv) {
    let _ = env.exec(
        r#"
        if not GlowEmitterFactory then
            GlowEmitterFactory = {
                Show = function() end,
                Hide = function() end,
                SetShown = function() end,
            }
        end
        if not GetLFGStringFromEnum then
            GetLFGStringFromEnum = function() return "" end
        end
    "#,
    );
}

/// LFGBackfillCover_Update is called with LFDQueueFrame.PartyBackfill which
/// is nil when the template child isn't created. Wrap the function to
/// silently ignore nil self.
fn patch_lfg_backfill(env: &WowLuaEnv) {
    let _ = env.exec(
        r#"
        if LFGBackfillCover_Update then
            local orig = LFGBackfillCover_Update
            LFGBackfillCover_Update = function(self, ...)
                if not self then return end
                return orig(self, ...)
            end
        end
    "#,
    );
}

/// Blizzard_Console_SavedVars is normally loaded from WTF/SavedVariables.
/// Without it, DeveloperConsoleMixin:OnLoad sets self.savedVars = nil, and
/// ShouldEditBoxTakeFocus (called via OnUpdate) crashes accessing .isShown.
/// Initialize the global and patch the existing frame since OnLoad already ran.
fn init_console_saved_vars(env: &WowLuaEnv) {
    let _ = env.exec(
        r#"
        if not Blizzard_Console_SavedVars then
            Blizzard_Console_SavedVars = {
                isShown = false,
                commandHistory = {},
                messageHistory = {},
                height = 300,
                fontHeight = 14,
            }
        end
        if DeveloperConsole and not DeveloperConsole.savedVars then
            DeveloperConsole.savedVars = Blizzard_Console_SavedVars
        end
    "#,
    );
}

/// LFGListFrame_OnLoad initializes EventsInBackground after registering
/// events. If OnLoad fails partway through (e.g. missing API), events fire
/// with EventsInBackground still nil. Initialize it as an empty table.
fn init_lfg_events_in_background(env: &WowLuaEnv) {
    let _ = env.exec(
        r#"
        if LFGListFrame and not LFGListFrame.EventsInBackground then
            LFGListFrame.EventsInBackground = {}
        end
    "#,
    );
}

/// Guard ScrollBoxListViewMixin methods against nil DataProvider.
///
/// CommunitiesFrame opens before its ScrollBox has a DataProvider set,
/// causing nil index errors in FindElementDataIndexByPredicate etc.
fn patch_scrollbox_nil_dataprovider(env: &WowLuaEnv) {
    let _ = env.exec(
        r#"
        local cl = CommunitiesFrameCommunitiesList
        if cl then
            cl.ScrollToClub = function(self, clubId)
                if self.ScrollBox and self.ScrollBox.HasDataProvider
                   and self.ScrollBox:HasDataProvider() then
                    self.ScrollBox:ScrollToElementDataByPredicate(function(elementData)
                        return elementData.clubInfo and elementData.clubInfo.clubId == clubId
                    end, ScrollBoxConstants.AlignCenter)
                end
            end
        end
    "#,
    );
}
