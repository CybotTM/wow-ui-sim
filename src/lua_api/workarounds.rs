//! Post-load Lua workarounds for Blizzard code that still depends on
//! simulator gaps or partial OnLoad recovery.
//!
//! If a shim stops being necessary, remove it rather than broadening it.

use super::workarounds_editmode;
use super::{SimState, WowLuaEnv};
use std::cell::RefCell;
use std::rc::Rc;

/// Apply workarounds that must run after startup events.
///
/// These post-event shims only correct state that Blizzard event handlers can
/// still leave inconsistent after startup.
pub fn apply_post_event(env: &WowLuaEnv) {
    workarounds_editmode::init_edit_mode_layout(env);
    suppress_spellbook_tutorials(env);
    init_world_map_frame(env);
    patch_fog_of_war_pin_canvas_resize(env);
}

/// Apply targeted cleanup after a load-on-demand addon finishes loading.
///
/// Blizzard_PlayerSpells creates these tutorial dialogs late. Keeping the
/// cleanup targeted to that addon avoids broad UI mutation on unrelated loads.
pub fn apply_post_runtime_addon_load(env: &WowLuaEnv, addon_name: &str) {
    if addon_name == "Blizzard_PlayerSpells" {
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
///
/// The remaining shims here fall into two groups:
/// - bootstrap recovery for Blizzard frames that partially initialize
/// - explicit stubs for WoW runtime objects we do not model yet
pub fn apply(env: &WowLuaEnv) {
    super::chat_init::show_chat_frame(env);
    super::chat_init::init_chat_type_colors(env);
    patch_uiparent_idle_onupdate_worklists(env);
    workarounds_editmode::patch_edit_mode_manager(env);
    patch_bag_openers(env);
    patch_character_toggle(env);
    patch_communities_toggle(env);
    patch_group_finder_toggle(env);
    patch_mail_toggle(env);
    patch_quest_map_current_map_lookup(env);
    patch_map_canvas_zoom(env);
    patch_fog_of_war_pin_canvas_resize(env);
    patch_poi_button_update_point(env);
    patch_combat_log_filters(env);
    patch_missing_frame_stubs(env);
}

/// Skip idle UIParent fan-out handlers when their backing worklists are empty.
///
/// Mainline UIParent unconditionally calls `FCF_OnUpdate`,
/// `ButtonPulse_OnUpdate`, and `AnimatedShine_OnUpdate` every frame. The
/// Blizzard implementations immediately iterate their global worklist tables,
/// so the empty-list case still pays for Lua dispatch + `pairs()` each tick.
///
/// Wrapping only the empty-table case keeps active work unchanged while
/// removing needless idle polling in reduced harnesses and fully-settled UI.
fn patch_uiparent_idle_onupdate_worklists(env: &WowLuaEnv) {
    let _ = env.exec(
        r#"
        if not __wow_ui_sim_uiparent_onupdate_worklists_patched then
            local function patchEmptyWorklist(globalName, worklistName)
                local original = _G[globalName]
                if type(original) ~= "function" then
                    return
                end

                _G[globalName] = function(...)
                    local worklist = _G[worklistName]
                    if type(worklist) == "table" and next(worklist) == nil then
                        return
                    end

                    return original(...)
                end
            end

            patchEmptyWorklist("FCF_OnUpdate", "CHAT_FRAMES")
            patchEmptyWorklist("ButtonPulse_OnUpdate", "PULSEBUTTONS")
            patchEmptyWorklist("AnimatedShine_OnUpdate", "SHINES_TO_ANIMATE")

            __wow_ui_sim_uiparent_onupdate_worklists_patched = true
        end
    "#,
    );
}

/// Suppress spellbook helptips via CVars instead of monkey-patching.
///
/// CheckShowHelpTips checks these bitfields before showing any tip.
/// Setting them to true tells Blizzard code the tutorials are already
/// dismissed, matching a real player who clicked them away.
fn suppress_spellbook_tutorials(env: &WowLuaEnv) {
    let _ = env.exec(
        r#"
        SetCVarBitfield("closedInfoFrames", LE_FRAME_TUTORIAL_BOOSTED_SPELL_BOOK, true)
        SetCVarBitfield("closedInfoFrames", LE_FRAME_TUTORIAL_PLAYER_SPELLS_MINIMIZE, true)
    "#,
    );
}

/// Load Blizzard_TokenUI on demand before Blizzard bag-opening entrypoints run.
///
/// The lighter panel harness loads ContainerFrame code without the separate
/// Blizzard_TokenUI addon, but bag setup still assumes
/// ContainerFrameSettingsManager.TokenTracker exists. Wrapping the public bag
/// openers keeps Blizzard container logic intact while ensuring the token
/// tracker is created before bag setup mutates ownership.
fn patch_bag_openers(env: &WowLuaEnv) {
    let _ = env.exec(PATCH_BAG_OPENERS_LUA);
}

const PATCH_BAG_OPENERS_LUA: &str = r#"
    if not __wow_ui_sim_bag_openers_patched then
        local loader = (C_AddOns and C_AddOns.LoadAddOn) or LoadAddOn

        local function ensureBagTokenTracker()
            if not ContainerFrameSettingsManager or ContainerFrameSettingsManager.TokenTracker then
                return
            end

            if loader then
                pcall(loader, "Blizzard_TokenUI")
            end

            if ContainerFrameSettingsManager and not ContainerFrameSettingsManager.TokenTracker and type(ContainerFrameSettingsManager.OnAddonLoaded) == "function" then
                ContainerFrameSettingsManager:OnAddonLoaded("Blizzard_TokenUI")
            end
        end

        local function patchBagOpener(name)
            local original = _G[name]
            if type(original) ~= "function" then
                return
            end

            _G[name] = function(...)
                ensureBagTokenTracker()
                return original(...)
            end
        end

        patchBagOpener("OpenAllBags")
        patchBagOpener("OpenBackpack")
        patchBagOpener("OpenBag")

        __wow_ui_sim_bag_openers_patched = true
    end
"#;

/// Load Blizzard_TokenUI on demand before character tabs that depend on TokenFrame.
///
/// CharacterFrame's Blizzard ShowSubFrame() path always hides all three
/// subframes, including TokenFrame. In the lighter panel harness TokenFrame
/// does not exist until Blizzard_TokenUI loads, so reputation toggles can fail
/// even though ReputationFrame itself is present.
fn patch_character_toggle(env: &WowLuaEnv) {
    let _ = env.exec(
        r#"
        if not __wow_ui_sim_toggle_character_patched and type(ToggleCharacter) == "function" then
            local originalToggleCharacter = ToggleCharacter
            ToggleCharacter = function(tab, onlyShow, ...)
                if (tab == "ReputationFrame" or tab == "TokenFrame") and not TokenFrame then
                    local loader = TokenFrame_LoadUI or ((C_AddOns and C_AddOns.LoadAddOn) or LoadAddOn)
                    if loader then
                        pcall(loader, "Blizzard_TokenUI")
                    end
                end

                return originalToggleCharacter(tab, onlyShow, ...)
            end

            __wow_ui_sim_toggle_character_patched = true
        end
    "#,
    );
}

/// Load Blizzard_Communities on demand before toggling the communities panel.
///
/// Mainline UIParent assumes CommunitiesFrame already exists when
/// ToggleCommunitiesFrame() runs, but the lighter panel harness intentionally
/// does not preload Blizzard_Communities. Wrapping the public toggle keeps
/// Blizzard's real ToggleGuildFrame() logic intact while making the panel
/// opener honest in that environment.
fn patch_communities_toggle(env: &WowLuaEnv) {
    let _ = env.exec(
        r#"
        if not __wow_ui_sim_toggle_communities_patched and type(ToggleCommunitiesFrame) == "function" then
            local originalToggleCommunitiesFrame = ToggleCommunitiesFrame
            ToggleCommunitiesFrame = function(...)
                if not CommunitiesFrame then
                    local loader = (C_AddOns and C_AddOns.LoadAddOn) or LoadAddOn
                    if loader then
                        pcall(loader, "Blizzard_Communities")
                    end
                end

                if not CommunitiesFrame then
                    return
                end

                return originalToggleCommunitiesFrame(...)
            end

            __wow_ui_sim_toggle_communities_patched = true
        end
    "#,
    );
}

/// Load Blizzard_GroupFinder on demand before toggling the group finder panel.
///
/// Mainline UIParent calls PVEFrame_ToggleFrame() directly from
/// ToggleLFDParentFrame(), but the lighter panel harness does not preload
/// Blizzard_GroupFinder. Wrapping the public toggle preserves Blizzard's
/// faction and eligibility guards while ensuring the toggle target exists.
fn patch_group_finder_toggle(env: &WowLuaEnv) {
    let _ = env.exec(
        r#"
        if not __wow_ui_sim_toggle_lfd_parent_patched and type(ToggleLFDParentFrame) == "function" then
            local originalToggleLFDParentFrame = ToggleLFDParentFrame
            ToggleLFDParentFrame = function(...)
                if not PVEFrame_ToggleFrame then
                    local loader = (C_AddOns and C_AddOns.LoadAddOn) or LoadAddOn
                    if loader then
                        pcall(loader, "Blizzard_GroupFinder")
                    end
                end

                if not PVEFrame_ToggleFrame then
                    return
                end

                return originalToggleLFDParentFrame(...)
            end

            __wow_ui_sim_toggle_lfd_parent_patched = true
        end
    "#,
    );
}

/// Install a legacy ToggleMailFrame() helper for the lighter panel harness.
///
/// Blizzard_MailFrame exposes MailFrame_Show()/MailFrame_Hide() after the
/// addon loads, but the older ToggleMailFrame global is absent in the current
/// runtime surface. The compat shim keeps the behavior narrow: define it only
/// when missing, lazy-load Blizzard_MailFrame, then delegate to the real mail
/// panel functions when available.
fn patch_mail_toggle(env: &WowLuaEnv) {
    let _ = env.exec(
        r#"
        if not __wow_ui_sim_toggle_mail_patched and type(ToggleMailFrame) ~= "function" then
            ToggleMailFrame = function()
                if not MailFrame then
                    local loader = (C_AddOns and C_AddOns.LoadAddOn) or LoadAddOn
                    if loader then
                        pcall(loader, "Blizzard_MailFrame")
                    end
                end

                if not MailFrame then
                    return
                end

                if MailFrame:IsShown() then
                    if type(MailFrame_Hide) == "function" then
                        return MailFrame_Hide()
                    end
                    return HideUIPanel(MailFrame)
                end

                if type(MailFrame_Show) == "function" then
                    return MailFrame_Show()
                end
                return ShowUIPanel(MailFrame)
            end

            __wow_ui_sim_toggle_mail_patched = true
        end
    "#,
    );
}

/// Guard QuestLogMixin:GetCurrentMapID when QuestMapFrame has no world map parent.
///
/// In partial panel harnesses and some startup traces, Blizzard_UIPanels_Game
/// loads QuestMapFrame before Blizzard_WorldMap attaches it to WorldMapFrame.
/// PLAYER_ENTERING_WORLD still calls QuestMapFrame:Refresh(), which reaches
/// GetCurrentMapID() and nil-dereferences self:GetParent():IsShown().
///
/// Falling back to C_Map.GetBestMapForUnit("player") matches Blizzard's own
/// non-world-map branch and keeps the quest log path usable until the parent
/// exists.
const QUEST_MAP_CURRENT_MAP_LOOKUP_PATCH: &str = r#"
    if not __wow_ui_sim_quest_map_parent_guard_patched then
        local function getQuestMapParent(frame)
            if not frame or type(frame.GetParent) ~= "function" then
                return nil
            end

            local parent = frame:GetParent()
            if not parent or type(parent.IsShown) ~= "function" then
                return nil
            end

            return parent
        end

        local function getCurrentMapID(self)
            local parent = getQuestMapParent(self)
            if parent and parent:IsShown() and type(parent.GetMapID) == "function" then
                return parent:GetMapID()
            end

            return C_Map.GetBestMapForUnit("player")
        end

        local function refresh(self)
            local parent = getQuestMapParent(self)
            if parent and QuestMapFrame.DetailsFrame.questMapID
                and self.DetailsFrame.questMapID ~= parent:GetMapID() then
                QuestMapFrame_CloseQuestDetails()
            end

            self:SyncQuestSystemWithCurrentMap()
            SortQuestSortTypes()
            SortQuests()
            local numPOIs = QuestMapUpdateAllQuests()
            QuestMapFrame_ResetFilters()
            QuestMapFrame_UpdateAll(numPOIs)

            self:ValidateTabs()
            self:CheckEventsTabTutorial()
        end

        local function updateAll(numPOIs)
            QuestMapFrame:UpdatePOIs()

            if not numPOIs then
                QuestMapUpdateAllQuests()
            end

            local parent = getQuestMapParent(QuestMapFrame)
            if parent and parent:IsShown() then
                local questDetailID = QuestMapFrame.DetailsFrame.questID
                if questDetailID then
                    QuestMapFrame_ShowQuestDetails(questDetailID)
                else
                    QuestLogQuests_Update()
                end

                if type(parent.OnQuestLogUpdate) == "function" then
                    parent:OnQuestLogUpdate()
                end
            end
        end

        if QuestLogMixin then
            QuestLogMixin.GetCurrentMapID = getCurrentMapID
            QuestLogMixin.Refresh = refresh
        end

        if QuestMapFrame then
            QuestMapFrame.GetCurrentMapID = getCurrentMapID
            QuestMapFrame.Refresh = refresh
        end

        if type(QuestMapFrame_UpdateAll) == "function" then
            QuestMapFrame_UpdateAll = updateAll
        end

        __wow_ui_sim_quest_map_parent_guard_patched = true
    end
"#;

fn patch_quest_map_current_map_lookup(env: &WowLuaEnv) {
    let _ = env.exec(QUEST_MAP_CURRENT_MAP_LOOKUP_PATCH);
}

/// Patch MapCanvasScrollControllerMixin to guard nil targetScale before compare.
///
/// `IsZoomingIn` and `IsZoomingOut` compare `self.targetScale` to a number but
/// `targetScale` is only set after the first zoom action. Before any zoom
/// the mixin leaves it nil, causing a "attempt to compare nil with number" error.
///
/// Patching the methods directly to default nil values avoids timing issues with
/// OnLoad having already fired for existing scroll container instances.
fn patch_map_canvas_zoom(env: &WowLuaEnv) {
    if let Err(e) = env.exec(
        r#"
        if MapCanvasScrollControllerMixin then
            function MapCanvasScrollControllerMixin:IsZoomingIn()
                if self.targetScale == nil then return false end
                return self:GetCanvasScale() < self.targetScale
            end
            function MapCanvasScrollControllerMixin:IsZoomingOut()
                if self.targetScale == nil then return false end
                return self.targetScale < self:GetCanvasScale()
            end
        end
    "#,
    ) {
        eprintln!("[workaround] patch_map_canvas_zoom failed: {e}");
    }
}

/// Keep FogOfWar pins sized to the canvas on first map open.
///
/// Blizzard's FogOfWarPinMixin only resizes itself in `OnCanvasScaleChanged()`,
/// but the simulator's first visible world-map frame can reach `OnCanvasSizeChanged()`
/// before any scale change callback. Other full-canvas map pins already handle
/// size changes directly, so patch fog to follow the same rule.
fn patch_fog_of_war_pin_canvas_resize(env: &WowLuaEnv) {
    if let Err(e) = env.exec(
        r#"
        local function resizeToCanvas(self)
            if not self or type(self.GetMap) ~= "function" then
                return
            end

            local map = self:GetMap()
            if not map then
                return
            end

            if type(map.DenormalizeHorizontalSize) ~= "function" or type(map.DenormalizeVerticalSize) ~= "function" then
                return
            end

            self:SetSize(
                map:DenormalizeHorizontalSize(1.0),
                map:DenormalizeVerticalSize(1.0)
            )
        end

        local function installFogCallbacks(pin)
            if not pin then
                return
            end

            pin.OnCanvasSizeChanged = resizeToCanvas
        end

        if FogOfWarPinMixin then
            local originalOnCanvasScaleChanged = FogOfWarPinMixin.OnCanvasScaleChanged
            function FogOfWarPinMixin:OnCanvasScaleChanged(...)
                resizeToCanvas(self)
                if type(originalOnCanvasScaleChanged) == "function" then
                    return originalOnCanvasScaleChanged(self, ...)
                end
            end

            FogOfWarPinMixin.OnCanvasSizeChanged = resizeToCanvas
        end

        if WorldMapFrame and type(WorldMapFrame.EnumeratePinsByTemplate) == "function" then
            for pin in WorldMapFrame:EnumeratePinsByTemplate("FogOfWarPinTemplate") do
                installFogCallbacks(pin)
            end
        end
    "#,
    ) {
        eprintln!("[workaround] patch_fog_of_war_pin_canvas_resize failed: {e}");
    }
}

/// Guard POIButtonDisplayLayerMixin:UpdatePoint against nil parent.
///
/// When map pins are created via template pools, the Display child's
/// GetParent() may return a frame whose IsEnabled method isn't accessible
/// (parent not yet fully initialized as a Button). Guard the call to
/// prevent "attempt to call method 'IsEnabled' (a nil value)" errors
/// that block world quest pin rendering.
fn patch_poi_button_update_point(env: &WowLuaEnv) {
    if let Err(e) = env.exec(PATCH_POI_BUTTON_UPDATE_POINT_LUA) {
        eprintln!("[workaround] patch_poi_button_update_point failed: {e}");
    }
}

const PATCH_POI_BUTTON_UPDATE_POINT_LUA: &str = r#"
    -- Guard UpdateButtonAlpha against nil NormalTexture/PushedTexture.
    -- These children should be created by POIButtonTemplate XML but
    -- template child creation during CreateFrame doesn't cover them yet.
    if POIButtonMixin then
        local orig = POIButtonMixin.UpdateButtonAlpha
        function POIButtonMixin:UpdateButtonAlpha()
            if self.NormalTexture and self.PushedTexture then
                orig(self)
            end
        end
    end
    if POIButtonDisplayLayerMixin then
        function POIButtonDisplayLayerMixin:UpdatePoint(isPushed)
            local parent = self:GetParent()
            if not parent or not parent.IsEnabled or not parent:IsEnabled() then
                return
            end
            local pushedX = isPushed and 1 or 0
            local pushedY = isPushed and -1 or 0
            local x = (self.offsetX or 0) + pushedX
            local y = (self.offsetY or 0) + pushedY
            if PixelUtil then
                PixelUtil.SetPoint(self, "CENTER", parent, "CENTER", x, y, x, y)
            else
                self:SetPoint("CENTER", parent, "CENTER", x, y)
            end
        end
    end
"#;

/// Add minimal stubs for global frames expected by Blizzard code but not
/// created by any loaded addon (e.g. the frame exists in XML but the parent
/// frame was nil, preventing creation).
///
/// These stubs only provide the specific methods/fields called at startup.
fn patch_missing_frame_stubs(env: &WowLuaEnv) {
    for (name, code) in MISSING_FRAME_STUBS {
        if let Err(e) = env.exec(code) {
            eprintln!("[workaround] patch_missing_frame_stubs({name}) failed: {e}");
        }
    }
}

/// Stub snippets for global frames expected by Blizzard code but not created
/// by any loaded addon. Each entry: (frame_name, lua_code).
const MISSING_FRAME_STUBS: &[(&str, &str)] = &[
    // FriendsFrameIcon: a Texture child of FriendsFrame, but FriendsFrame is
    // hidden at startup. Provides SetTexture stub to silence OnEvent errors.
    (
        "FriendsFrameIcon",
        r#"
        if not FriendsFrameIcon then
            rawset(_G, "FriendsFrameIcon", { SetTexture = function() end })
        end
    "#,
    ),
    // QueueStatusButton: created by Blizzard_QueueStatusFrame XML.
    // QueueStatusButtonMixin provides SetGlowLock; stub it if missing.
    (
        "QueueStatusButton",
        r#"
        if QueueStatusButton and not QueueStatusButton.SetGlowLock then
            QueueStatusButton.SetGlowLock = function() end
        end
        if not QueueStatusButton then
            rawset(_G, "QueueStatusButton", { SetGlowLock = function() end })
        end
    "#,
    ),
    // TextToSpeechDefaultButton: Button in ChatConfigFrame.xml.
    // If the parent frame failed to create (nil parent), the button is nil.
    // Provide a stub with Text field for UpdateDefaultButtons().
    (
        "TextToSpeechDefaultButton",
        r#"
        if not TextToSpeechDefaultButton then
            local t = { GetWidth = function() return 100 end }
            rawset(_G, "TextToSpeechDefaultButton", {
                Text = t,
                SetShown = function() end,
                SetWidth = function() end,
                SetPoint = function() end,
            })
        elseif not TextToSpeechDefaultButton.Text then
            TextToSpeechDefaultButton.Text = { GetWidth = function() return 100 end }
        end
    "#,
    ),
    (
        "TextToSpeechCharacterSpecificButton",
        r#"
        if not TextToSpeechCharacterSpecificButton then
            rawset(_G, "TextToSpeechCharacterSpecificButton", {
                SetShown = function() end,
                SetPoint = function() end,
            })
        end
    "#,
    ),
];

/// Initialize WorldMapFrame with the player's current zone map and refresh data
/// providers so that map pins (world quests, POIs, etc.) are populated.
///
/// In real WoW this happens when the player opens the map (OnShow). The
/// simulator never triggers OnShow for the map, so we call SetMapID and
/// RefreshAllDataProviders explicitly after startup events complete.
fn init_world_map_frame(env: &WowLuaEnv) {
    if let Err(e) = env.exec(
        r#"
        if WorldMapFrame and WorldMapFrame.SetMapID and WorldMapFrame.RefreshAllDataProviders then
            local mapID = C_Map.GetBestMapForUnit("player") or 2248
            pcall(WorldMapFrame.SetMapID, WorldMapFrame, mapID)
            pcall(WorldMapFrame.RefreshAllDataProviders, WorldMapFrame)
        end
    "#,
    ) {
        eprintln!("[workaround] init_world_map_frame failed: {e}");
    }
}

/// Initialize Blizzard_CombatLog_Filters with the structure ChatConfigFrame expects.
///
/// Blizzard_CombatLog is LoadOnDemand — it sets this global at line 411 but
/// ChatConfigFrame accesses `.filters` during startup OnShow events before
/// the combat log addon loads. Provide the minimal structure so those
/// accesses don't error.
fn patch_combat_log_filters(env: &WowLuaEnv) {
    if let Err(e) = env.exec(
        r#"
        if Blizzard_CombatLog_Filters == nil then
            rawset(_G, "Blizzard_CombatLog_Filters", { filters = {} })
        end
        if CHATCONFIG_SELECTED_FILTER == nil then
            rawset(_G, "CHATCONFIG_SELECTED_FILTER", nil)
        end
    "#,
    ) {
        eprintln!("[workaround] patch_combat_log_filters failed: {e}");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn env() -> WowLuaEnv {
        WowLuaEnv::new().expect("Failed to create Lua environment")
    }

    #[test]
    fn spellbook_tutorial_suppression_sets_cvars() {
        let env = env();
        suppress_spellbook_tutorials(&env);

        let (boosted, minimize): (bool, bool) = env
            .eval(
                r#"
                return GetCVarBitfield("closedInfoFrames", LE_FRAME_TUTORIAL_BOOSTED_SPELL_BOOK),
                    GetCVarBitfield("closedInfoFrames", LE_FRAME_TUTORIAL_PLAYER_SPELLS_MINIMIZE)
                "#,
            )
            .unwrap();

        assert!(
            boosted,
            "BOOSTED_SPELL_BOOK tutorial should be marked closed"
        );
        assert!(
            minimize,
            "PLAYER_SPELLS_MINIMIZE tutorial should be marked closed"
        );
    }

    #[test]
    fn quest_map_patch_falls_back_to_best_map_for_unit_without_parent() {
        let env = env();
        env.exec(
            r#"
            C_Map = {
                GetBestMapForUnit = function(unit)
                    assert(unit == "player")
                    return 4242
                end
            }
            QuestLogMixin = {}
            QuestMapFrame = {
                DetailsFrame = {},
                UpdatePOIs = function() end,
                ValidateTabs = function() end,
                CheckEventsTabTutorial = function() end,
                SyncQuestSystemWithCurrentMap = function() end,
            }
            function SortQuestSortTypes() end
            function SortQuests() end
            function QuestMapUpdateAllQuests() return 0 end
            function QuestMapFrame_ResetFilters() end
            function QuestMapFrame_UpdateAll() end
        "#,
        )
        .unwrap();

        patch_quest_map_current_map_lookup(&env);

        let map_id: i64 = env.eval("return QuestLogMixin:GetCurrentMapID()").unwrap();
        let patched: bool = env
            .eval("return __wow_ui_sim_quest_map_parent_guard_patched == true")
            .unwrap();

        assert_eq!(map_id, 4242);
        assert!(patched, "quest map guard patch should be marked installed");
    }
}
