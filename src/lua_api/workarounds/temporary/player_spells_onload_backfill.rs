//! Temporary PlayerSpells OnLoad backfill.
//!
//! PlayerSpells child frames can be used before their Blizzard OnLoad handlers
//! have initialized per-tab state. This keeps those frames usable until the
//! simulator models the PlayerSpells lifecycle ordering closely enough to drop
//! the backfill.

use crate::lua_api::LoaderEnv;
#[cfg(test)]
use crate::lua_api::WowLuaEnv;

const PLAYERSPELLS_ONLOAD_BACKFILL_LUA: &str = r#"
local function backfill_onload(frame, needs_init)
    if not frame or not needs_init then
        return
    end
    if type(frame.OnLoad) == "function" then
        frame:OnLoad()
        return
    end
    local handler = frame.GetScript and frame:GetScript("OnLoad")
    if type(handler) == "function" then
        handler(frame)
    end
end

local function backfill_playerspells_tab(frame_tab)
    if not PlayerSpellsFrame or not PlayerSpellsUtil or not PlayerSpellsUtil.FrameTabs then
        return
    end

    if frame_tab == PlayerSpellsUtil.FrameTabs.ClassSpecializations then
        backfill_onload(
            PlayerSpellsFrame.SpecFrame,
            PlayerSpellsFrame.SpecFrame and PlayerSpellsFrame.SpecFrame.SpecContentFramePool == nil
        )
    elseif frame_tab == PlayerSpellsUtil.FrameTabs.ClassTalents then
        backfill_onload(
            PlayerSpellsFrame.TalentsFrame,
            PlayerSpellsFrame.TalentsFrame and PlayerSpellsFrame.TalentsFrame.initialBasePanOffsetX == nil
        )
    elseif frame_tab == PlayerSpellsUtil.FrameTabs.SpellBook then
        backfill_onload(
            PlayerSpellsFrame.SpellBookFrame,
            PlayerSpellsFrame.SpellBookFrame and PlayerSpellsFrame.SpellBookFrame.internalTabTracker == nil
        )
    end
end

if PlayerSpellsFrame then
    backfill_onload(PlayerSpellsFrame, PlayerSpellsFrame.internalTabTracker == nil)
    if not __wow_uisim_playerspells_backfill_wrapped then
        __wow_uisim_playerspells_backfill_wrapped = true

        if type(PlayerSpellsFrame.TrySetTab) == "function" then
            local original_try_set_tab = PlayerSpellsFrame.TrySetTab
            PlayerSpellsFrame.TrySetTab = function(self, frame_tab)
                backfill_playerspells_tab(frame_tab)
                return original_try_set_tab(self, frame_tab)
            end
        end

        if type(PlayerSpellsFrame.SetInspecting) == "function" then
            local original_set_inspecting = PlayerSpellsFrame.SetInspecting
            PlayerSpellsFrame.SetInspecting = function(self, inspect_unit, inspect_string, inspect_string_level)
                if inspect_unit or inspect_string then
                    backfill_playerspells_tab(PlayerSpellsUtil.FrameTabs.ClassTalents)
                end
                return original_set_inspecting(self, inspect_unit, inspect_string, inspect_string_level)
            end
        end
    end
end
"#;

pub(crate) fn patch(env: &LoaderEnv<'_>) -> Result<(), crate::Error> {
    env.exec(PLAYERSPELLS_ONLOAD_BACKFILL_LUA)
}

#[cfg(test)]
fn patch_env(env: &WowLuaEnv) {
    env.exec(PLAYERSPELLS_ONLOAD_BACKFILL_LUA)
        .expect("PlayerSpells OnLoad backfill should install");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn backfills_player_spells_frame_on_install() {
        let env = WowLuaEnv::new().expect("lua env should initialize");
        install_playerspells_fixture(&env);

        patch_env(&env);

        let internal_tab_tracker: String = env
            .eval("return PlayerSpellsFrame.internalTabTracker")
            .expect("PlayerSpellsFrame OnLoad should run");

        assert_eq!(internal_tab_tracker, "loaded");
    }

    #[test]
    fn wraps_try_set_tab_to_backfill_spellbook_tab() {
        let env = WowLuaEnv::new().expect("lua env should initialize");
        install_playerspells_fixture(&env);

        patch_env(&env);

        let (spellbook_tracker, selected_tab): (String, String) = env
            .eval(
                r#"
                PlayerSpellsFrame:TrySetTab(PlayerSpellsUtil.FrameTabs.SpellBook)
                return PlayerSpellsFrame.SpellBookFrame.internalTabTracker,
                    PlayerSpellsFrame.selectedTab
                "#,
            )
            .expect("wrapped TrySetTab should run");

        assert_eq!(spellbook_tracker, "loaded");
        assert_eq!(selected_tab, "SpellBook");
    }

    #[test]
    fn wraps_set_inspecting_to_backfill_talents_tab() {
        let env = WowLuaEnv::new().expect("lua env should initialize");
        install_playerspells_fixture(&env);

        patch_env(&env);

        let (talents_pan_offset, inspecting_unit): (i64, String) = env
            .eval(
                r#"
                PlayerSpellsFrame:SetInspecting("target")
                return PlayerSpellsFrame.TalentsFrame.initialBasePanOffsetX,
                    PlayerSpellsFrame.inspectingUnit
                "#,
            )
            .expect("wrapped SetInspecting should run");

        assert_eq!(talents_pan_offset, 10);
        assert_eq!(inspecting_unit, "target");
    }

    #[test]
    fn ignores_missing_player_spells_frame() {
        let env = WowLuaEnv::new().expect("lua env should initialize");

        patch_env(&env);

        let player_spells_frame_type: String = env
            .eval("return type(PlayerSpellsFrame)")
            .expect("missing PlayerSpellsFrame should be untouched");

        assert_eq!(player_spells_frame_type, "nil");
    }

    fn install_playerspells_fixture(env: &WowLuaEnv) {
        install_playerspells_util_fixture(env);
        install_playerspells_frame_fixture(env);
    }

    fn install_playerspells_util_fixture(env: &WowLuaEnv) {
        env.exec(
            r#"
            PlayerSpellsUtil = {
                FrameTabs = {
                    ClassSpecializations = "ClassSpecializations",
                    ClassTalents = "ClassTalents",
                    SpellBook = "SpellBook",
                },
            }
            "#,
        )
        .expect("PlayerSpellsUtil fixture should install");
    }

    fn install_playerspells_frame_fixture(env: &WowLuaEnv) {
        env.exec(
            r#"
            PlayerSpellsFrame = {
                SpecFrame = {
                    OnLoad = function(self)
                        self.SpecContentFramePool = "loaded"
                    end,
                },
                TalentsFrame = {
                    OnLoad = function(self)
                        self.initialBasePanOffsetX = 10
                    end,
                },
                SpellBookFrame = {
                    OnLoad = function(self)
                        self.internalTabTracker = "loaded"
                    end,
                },
                OnLoad = function(self)
                    self.internalTabTracker = "loaded"
                end,
                TrySetTab = function(self, frameTab)
                    self.selectedTab = frameTab
                end,
                SetInspecting = function(self, inspectUnit)
                    self.inspectingUnit = inspectUnit
                end,
            }
            "#,
        )
        .expect("PlayerSpellsFrame fixture should install");
    }
}
