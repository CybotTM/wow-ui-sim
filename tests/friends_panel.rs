use crate::common;

use std::path::{Path, PathBuf};

use wow_ui_sim::loader::{discover_blizzard_addons, load_addon};
use wow_ui_sim::lua_api::WowLuaEnv;
use wow_ui_sim::startup::{fire_one_on_update_tick, fire_startup_events, process_pending_timers};

fn blizzard_ui_dir() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("Interface/BlizzardUI")
}

fn full_game_env_after_startup() -> WowLuaEnv {
    let env = WowLuaEnv::new().expect("Failed to create Lua environment");
    env.set_screen_size(1600.0, 1200.0);

    {
        let mut state = env.state().borrow_mut();
        state.addon_base_paths = vec![blizzard_ui_dir()];
    }

    load_all_blizzard_addons(&env, &blizzard_ui_dir());
    env.apply_post_load_workarounds();
    fire_startup_events(&env);
    env.apply_post_event_workarounds();
    env.state().borrow_mut().widgets.rebuild_anchor_index();
    process_pending_timers(&env);
    fire_one_on_update_tick(&env);

    env
}

fn load_all_blizzard_addons(env: &WowLuaEnv, ui: &Path) {
    for (name, toc_path) in &discover_blizzard_addons(ui) {
        if let Err(err) = load_addon(&env.loader_env(), toc_path) {
            panic!("[load {name}] FAILED: {err}");
        }
    }
}

#[test]
fn social_panel_toggle_realizes_online_and_offline_friend_rows() {
    common::with_timeout(120, || {
        common::with_perf_lock(|| {
            let env = full_game_env_after_startup();

            let result: String = env
                .eval(
                    r#"
                    ToggleSocialPanel()

                    local rows = {}
                    FriendsListFrame.ScrollBox:ForEachFrame(function(frame, elementData)
                        if elementData.buttonType == FRIENDS_BUTTON_TYPE_WOW then
                            table.insert(rows, frame.name:GetText() .. "|" .. frame.info:GetText())
                        end
                    end)

                    return table.concat(rows, "\n")
                    "#,
                )
                .unwrap();

            assert!(
                result.contains("Alyth"),
                "friends panel should show the online friend row, got: {result:?}"
            );
            assert!(
                result.contains("Brennor"),
                "friends panel should show the offline friend row, got: {result:?}"
            );

            let (wow_alpha, offline_alpha): (f64, f64) = env
                .eval(
                    r#"
                    return FRIENDS_WOW_BACKGROUND_COLOR.a, FRIENDS_OFFLINE_BACKGROUND_COLOR.a
                    "#,
                )
                .unwrap();

            assert!(
                (wow_alpha - 0.05).abs() < 0.001,
                "online friend row background should be translucent, got alpha {wow_alpha}"
            );
            assert!(
                (offline_alpha - 0.05).abs() < 0.001,
                "offline friend row background should be translucent, got alpha {offline_alpha}"
            );
        });
    });
}
