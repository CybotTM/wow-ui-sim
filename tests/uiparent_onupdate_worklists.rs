use crate::common;

use std::path::PathBuf;

use wow_ui_sim::loader::{discover_blizzard_addons_for_screen, load_addon};
use wow_ui_sim::lua_api::WowLuaEnv;
use wow_ui_sim::screen::ScreenKind;
use wow_ui_sim::startup::settle_headless_startup;

fn blizzard_ui_dir() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("Interface/BlizzardUI")
}

fn load_settled_game_ui() -> WowLuaEnv {
    let env = WowLuaEnv::new().expect("Failed to create Lua environment");
    env.set_screen_size(1024.0, 768.0);
    env.set_screen_mode(ScreenKind::Game);
    env.state().borrow_mut().addon_base_paths = vec![blizzard_ui_dir()];

    let addons = discover_blizzard_addons_for_screen(&blizzard_ui_dir(), ScreenKind::Game);
    for (name, toc_path) in &addons {
        load_addon(&env.loader_env(), toc_path)
            .unwrap_or_else(|err| panic!("Failed to load Blizzard addon {name}: {err}"));
    }

    env.apply_post_load_workarounds();
    settle_headless_startup(&env);
    env
}

#[test]
fn uiparent_onupdate_skips_empty_worklists() {
    test_timeout! {
        let env = load_settled_game_ui();

        let (chat_pairs, pulse_pairs, shine_pairs): (i32, i32, i32) = env
            .eval(
                r#"
                local counts = { chat = 0, pulse = 0, shine = 0 }
                local emptyChatFrames = {}
                local emptyPulseButtons = {}
                local emptyShines = {}

                local originalPairs = pairs
                pairs = function(t)
                    if t == emptyChatFrames then
                        counts.chat = counts.chat + 1
                    elseif t == emptyPulseButtons then
                        counts.pulse = counts.pulse + 1
                    elseif t == emptyShines then
                        counts.shine = counts.shine + 1
                    end
                    return originalPairs(t)
                end

                local originalChatFrames = CHAT_FRAMES
                local originalPulseButtons = PULSEBUTTONS
                local originalShines = SHINES_TO_ANIMATE

                CHAT_FRAMES = emptyChatFrames
                PULSEBUTTONS = emptyPulseButtons
                SHINES_TO_ANIMATE = emptyShines

                local script = assert(UIParent:GetScript("OnUpdate"), "missing UIParent OnUpdate")
                script(UIParent, 0.016)

                CHAT_FRAMES = originalChatFrames
                PULSEBUTTONS = originalPulseButtons
                SHINES_TO_ANIMATE = originalShines
                pairs = originalPairs

                return counts.chat, counts.pulse, counts.shine
                "#,
            )
            .unwrap();

        assert_eq!(
            chat_pairs, 0,
            "UIParent OnUpdate should skip FCF_OnUpdate when CHAT_FRAMES is empty"
        );
        assert_eq!(
            pulse_pairs, 0,
            "UIParent OnUpdate should skip ButtonPulse_OnUpdate when PULSEBUTTONS is empty"
        );
        assert_eq!(
            shine_pairs, 0,
            "UIParent OnUpdate should skip AnimatedShine_OnUpdate when SHINES_TO_ANIMATE is empty"
        );
    }
}

#[test]
fn button_pulse_onupdate_still_updates_active_buttons() {
    test_timeout! {
        let env = load_settled_game_ui();

        let (lock_calls, unlock_calls, pulse_on, pulse_time_left): (i32, i32, i32, f64) = env
            .eval(
                r#"
                local button = {
                    pulseDuration = -0.01,
                    pulseTimeLeft = 0.5,
                    pulseRate = 0.2,
                    pulseOn = 0,
                    lockCalls = 0,
                    unlockCalls = 0,
                }

                function button:LockHighlight()
                    self.lockCalls = self.lockCalls + 1
                end

                function button:UnlockHighlight()
                    self.unlockCalls = self.unlockCalls + 1
                end

                local originalPulseButtons = PULSEBUTTONS
                PULSEBUTTONS = { button }
                ButtonPulse_OnUpdate(0.016)
                PULSEBUTTONS = originalPulseButtons

                return button.lockCalls, button.unlockCalls, button.pulseOn, button.pulseTimeLeft
                "#,
            )
            .unwrap();

        assert_eq!(lock_calls, 1, "active pulsing button should still lock highlight");
        assert_eq!(unlock_calls, 0, "active pulsing button should not unlock in the toggle-on path");
        assert_eq!(pulse_on, 1, "active pulsing button should still flip pulseOn");
        assert!(pulse_time_left < 0.5, "active pulsing button should still consume elapsed time");
    }
}

#[test]
fn animated_shine_onupdate_still_updates_active_shines() {
    test_timeout! {
        let env = load_settled_game_ui();

        let (update_calls, last_elapsed): (i32, f64) = env
            .eval(
                r#"
                local shine = { updateCalls = 0, lastElapsed = 0 }
                function shine:Update(elapsed)
                    self.updateCalls = self.updateCalls + 1
                    self.lastElapsed = elapsed
                end

                local originalShines = SHINES_TO_ANIMATE
                SHINES_TO_ANIMATE = { shine }
                AnimatedShine_OnUpdate(0.016)
                SHINES_TO_ANIMATE = originalShines

                return shine.updateCalls, shine.lastElapsed
                "#,
            )
            .unwrap();

        assert_eq!(update_calls, 1, "active shine should still receive Update");
        assert_eq!(last_elapsed, 0.016, "active shine should receive the elapsed tick");
    }
}
