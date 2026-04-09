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
    workarounds_editmode::patch_edit_mode_manager(env);
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


#[cfg(test)]
mod tests {
    use super::*;

    fn env() -> WowLuaEnv {
        WowLuaEnv::new().expect("Failed to create Lua environment")
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
}
