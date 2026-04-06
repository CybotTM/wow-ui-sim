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
    attach_castbar_to_player_frame(env);
}

/// Attach PlayerCastingBarFrame to PlayerFrame (WoW's default position).
///
/// EditMode's `ApplySystemAnchor` normally handles this, but PlayerCastingBarFrame
/// has nil systemInfo (no CastBar entry in the preset layout), so the anchor is
/// never applied.  Call Blizzard's `PlayerFrame_AttachCastBar()` directly.
fn attach_castbar_to_player_frame(env: &WowLuaEnv) {
    let _ = env.exec(
        r#"
        if PlayerFrame_AttachCastBar and PlayerCastingBarFrame then
            pcall(PlayerFrame_AttachCastBar)
        end
        "#,
    );
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
    super::workarounds_tracker::init_objective_tracker(env);
    super::chat_init::show_chat_frame(env);
    workarounds_bags::init_bag_bar(env);
    workarounds_bags::init_bag_token_tracker(env);
    super::chat_init::init_chat_type_colors(env);
    workarounds_editmode::patch_edit_mode_manager(env);
    stub_glow_emitter_factory(env);
    init_raid_frame_divider_pools(env);
    guard_lfg_backfill_cover(env);
}

/// Guard `LFGBackfillCover_Update` against nil self.
///
/// `RaidFinderQueueFrame.PartyBackfill` and `ScenarioQueueFrame.PartyBackfill`
/// may be nil if the template child wasn't instantiated. The Blizzard code at
/// LFGFrame.lua:274 passes these as self without nil-checking.
fn guard_lfg_backfill_cover(env: &WowLuaEnv) {
    let _ = env.exec(
        r#"
        if LFGBackfillCover_Update then
            local orig = LFGBackfillCover_Update
            LFGBackfillCover_Update = function(self, ...)
                if self then return orig(self, ...) end
            end
        end
        "#,
    );
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

/// CompactRaidFrameManager's OnLoad fails before creating divider pools
/// (line 160), leaving `dividerVerticalPool`/`dividerHorizontalPool` nil.
/// UpdateOptionsFlowContainer (line 473) then crashes accessing them.
/// Initialize the pools if OnLoad didn't get far enough.
fn init_raid_frame_divider_pools(env: &WowLuaEnv) {
    let _ = env.exec(
        r#"
        local mgr = CompactRaidFrameManager
        if mgr and not mgr.dividerVerticalPool then
            mgr.dividerVerticalPool = CreateTexturePool(mgr, "ARTWORK", 0, "CRFManagerDividerVertical")
            mgr.dividerHorizontalPool = CreateTexturePool(mgr, "ARTWORK", 0, "CRFManagerDividerHorizontal")
        end
    "#,
    );
}

/// GlowEmitterFactory is a C++ object in WoW managing spell overlay glow effects.
/// Stub with no-ops until properly implemented (see docs/glow-plan.md).
fn stub_glow_emitter_factory(env: &WowLuaEnv) {
    let _ = env.exec(
        r#"
        if not GlowEmitterFactory then
            GlowEmitterFactory = {
                Show = function() end,
                Hide = function() end,
                SetShown = function() end,
            }
        end
    "#,
    );
}
