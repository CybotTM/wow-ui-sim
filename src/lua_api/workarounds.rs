//! Post-load Lua workarounds for Blizzard code that still depends on
//! simulator gaps or partial OnLoad recovery.
//!
//! These helpers are intentionally narrow. They are kept only where the
//! underlying Blizzard code still expects either:
//! - a C++-backed object we do not implement yet (`GlowEmitterFactory`)
//! - a frame/template child that can be nil after partial addon load
//! - a startup ordering side effect that the simulator does not fully replay
//!
//! If a shim stops being necessary, remove it rather than broadening it.

use super::workarounds_bags;
use super::workarounds_editmode;
use super::{SimState, WowLuaEnv};
use std::cell::RefCell;
use std::rc::Rc;

/// Apply workarounds that must run after startup events.
///
/// These post-event shims only correct state that Blizzard event handlers can
/// still undo during startup (for example EditMode re-anchoring managed bars).
pub fn apply_post_event(env: &WowLuaEnv) {
    workarounds_bags::fix_bags_bar_anchor(env);
    workarounds_bags::fix_bag_item_context_overlay(env);
    workarounds_editmode::init_edit_mode_layout(env);
    hide_talent_loadout_dialogs(env);
    suppress_spellbook_tutorials(env);
    attach_castbar_to_player_frame(env);
}

/// Attach PlayerCastingBarFrame to PlayerFrame (WoW's default position).
///
/// EditMode's `ApplySystemAnchor` normally handles this, but PlayerCastingBarFrame
/// has nil systemInfo (no CastBar entry in the preset layout), so the anchor is
/// never applied. Retained until cast-bar placement comes from the normal
/// EditMode/system-anchor path.
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
///
/// Blizzard_PlayerSpells creates these tutorial dialogs late. Keeping the
/// cleanup targeted to that addon avoids broad UI mutation on unrelated loads.
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
///
/// The remaining shims here fall into two groups:
/// - bootstrap recovery for Blizzard frames that partially initialize
/// - explicit stubs for WoW runtime objects we do not model yet
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
/// LFGFrame.lua:274 passes these as self without nil-checking. Retained as a
/// narrow nil-guard until those template children are guaranteed to exist.
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
/// Retained because first-open PlayerSpells startup still leaks them visible.
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
/// Retained until spellbook startup produces the same first-open state without
/// tutorial suppression.
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
/// Initialize the pools if OnLoad didn't get far enough. Retained as partial
/// OnLoad recovery, not as a general raid-frame behavior override.
fn init_raid_frame_divider_pools(env: &WowLuaEnv) {
    let _ = env.exec(
        r#"
        local mgr = CompactRaidFrameManager
        if mgr and not mgr.dividerVerticalPool then
            mgr.dividerVerticalPool = CreateTexturePool(mgr, "ARTWORK", 0, "CRFManagerDividerVertical")
        end
        if mgr and not mgr.dividerHorizontalPool then
            mgr.dividerHorizontalPool = CreateTexturePool(mgr, "ARTWORK", 0, "CRFManagerDividerHorizontal")
        end
    "#,
    );
}

/// GlowEmitterFactory is a C++ object in WoW managing spell overlay glow effects.
/// Stub with no-ops until properly implemented (see docs/glow-plan.md).
/// The shim installs only when the real factory is absent so it does not
/// override a future Lua-side or engine-backed implementation.
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

#[cfg(test)]
mod tests {
    use super::*;

    fn env() -> WowLuaEnv {
        WowLuaEnv::new().expect("Failed to create Lua environment")
    }

    #[test]
    fn glow_emitter_stub_installs_only_when_missing() {
        let env = env();

        env.exec(
            r#"
            GlowEmitterFactory = {
                marker = 42,
                Show = function() end,
                Hide = function() end,
                SetShown = function() end,
            }
            "#,
        )
        .unwrap();
        stub_glow_emitter_factory(&env);

        let marker: i32 = env.eval("return GlowEmitterFactory.marker").unwrap();
        assert_eq!(
            marker, 42,
            "existing GlowEmitterFactory should be preserved"
        );

        let fresh_env = crate::lua_api::WowLuaEnv::new().expect("Failed to create Lua environment");
        stub_glow_emitter_factory(&fresh_env);
        let installed: bool = fresh_env
            .eval(
                r#"
                return type(GlowEmitterFactory) == "table"
                    and type(GlowEmitterFactory.Show) == "function"
                    and type(GlowEmitterFactory.Hide) == "function"
                    and type(GlowEmitterFactory.SetShown) == "function"
                "#,
            )
            .unwrap();
        assert!(
            installed,
            "missing GlowEmitterFactory should get a narrow stub"
        );
    }

    #[test]
    fn lfg_backfill_guard_ignores_nil_self_and_preserves_real_calls() {
        let env = env();
        env.exec(
            r#"
            backfill_calls = 0
            LFGBackfillCover_Update = function(self, forceUpdate)
                backfill_calls = backfill_calls + 1
                if not self then error("nil self") end
                _G.last_force = forceUpdate
                _G.last_self_seen = self == _G.expected_self
            end
            expected_self = {}
            "#,
        )
        .unwrap();

        guard_lfg_backfill_cover(&env);
        env.exec(
            r#"
            LFGBackfillCover_Update(nil, true)
            LFGBackfillCover_Update(expected_self, false)
            "#,
        )
        .unwrap();

        let (calls, last_force, same_self): (i32, bool, bool) = env
            .eval("return backfill_calls, last_force, last_self_seen")
            .unwrap();
        assert_eq!(
            calls, 1,
            "nil self should be ignored, real self should still call through"
        );
        assert!(!last_force);
        assert!(same_self);
    }

    #[test]
    fn raid_frame_pool_init_only_fills_missing_pools() {
        let env = env();
        env.exec(
            r#"
            created = {}
            local existing = { marker = "existing" }
            CompactRaidFrameManager = {
                dividerVerticalPool = existing,
                dividerHorizontalPool = nil,
            }
            CreateTexturePool = function(_, layer, sublevel, template)
                table.insert(created, template)
                return { layer = layer, sublevel = sublevel, template = template }
            end
            "#,
        )
        .unwrap();

        init_raid_frame_divider_pools(&env);
        let (vertical_marker, horizontal_template, created_count): (String, String, i32) = env
            .eval(
                r#"
                return CompactRaidFrameManager.dividerVerticalPool.marker,
                    CompactRaidFrameManager.dividerHorizontalPool.template,
                    #created
                "#,
            )
            .unwrap();
        assert_eq!(vertical_marker, "existing");
        assert_eq!(horizontal_template, "CRFManagerDividerHorizontal");
        assert_eq!(created_count, 1);
    }

    #[test]
    fn attach_castbar_calls_blizzard_helper_only_when_frames_exist() {
        let env = env();
        env.exec(
            r#"
            attach_calls = 0
            PlayerFrame_AttachCastBar = function()
                attach_calls = attach_calls + 1
            end
            "#,
        )
        .unwrap();

        attach_castbar_to_player_frame(&env);
        let calls_without_frame: i32 = env.eval("return attach_calls").unwrap();
        assert_eq!(calls_without_frame, 0);

        env.exec("PlayerCastingBarFrame = {}").unwrap();
        attach_castbar_to_player_frame(&env);
        let calls_with_frame: i32 = env.eval("return attach_calls").unwrap();
        assert_eq!(calls_with_frame, 1);
    }

    #[test]
    fn talent_dialog_cleanup_hides_only_target_dialogs() {
        let env = env();
        env.exec(
            r#"
            local function dialog()
                return {
                    hidden = false,
                    Hide = function(self) self.hidden = true end,
                }
            end
            ClassTalentLoadoutCreateDialog = dialog()
            ClassTalentLoadoutEditDialog = dialog()
            ClassTalentLoadoutImportDialog = dialog()
            UnrelatedDialog = dialog()
            "#,
        )
        .unwrap();

        hide_talent_loadout_dialogs(&env);
        let (create_hidden, edit_hidden, import_hidden, unrelated_hidden): (
            bool,
            bool,
            bool,
            bool,
        ) = env
            .eval(
                r#"
                return ClassTalentLoadoutCreateDialog.hidden,
                    ClassTalentLoadoutEditDialog.hidden,
                    ClassTalentLoadoutImportDialog.hidden,
                    UnrelatedDialog.hidden
                "#,
            )
            .unwrap();
        assert!(create_hidden && edit_hidden && import_hidden);
        assert!(
            !unrelated_hidden,
            "cleanup should stay scoped to talent loadout dialogs"
        );
    }

    #[test]
    fn spellbook_tutorial_suppression_only_overrides_tip_entrypoints() {
        let env = env();
        env.exec(
            r#"
            hide_calls = 0
            hidden_system = nil
            SpellBookFrameTutorialsMixin = {
                CheckShowHelpTips = function()
                    error("original should be replaced")
                end,
                KeepMe = "still-here",
            }
            HelpTip = {
                HideAllSystem = function(_, system)
                    hide_calls = hide_calls + 1
                    hidden_system = system
                end,
            }
            "#,
        )
        .unwrap();

        suppress_spellbook_tutorials(&env);
        let (keep_me, hide_calls, hidden_system): (String, i32, String) = env
            .eval(
                r#"
                SpellBookFrameTutorialsMixin.CheckShowHelpTips()
                return SpellBookFrameTutorialsMixin.KeepMe, hide_calls, hidden_system
                "#,
            )
            .unwrap();

        assert_eq!(keep_me, "still-here");
        assert_eq!(hide_calls, 1);
        assert_eq!(hidden_system, "SpellBook Helptips");
    }
}
