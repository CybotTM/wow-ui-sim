mod common;

use std::path::PathBuf;

use wow_ui_sim::loader::{discover_blizzard_addons_for_screen, load_addon};
use wow_ui_sim::lua_api::WowLuaEnv;
use wow_ui_sim::screen::ScreenKind;
use wow_ui_sim::startup::settle_headless_startup;

fn blizzard_ui_dir() -> PathBuf {
    wow_ui_sim::client_profile::blizzard_ui_addons_dir_under(std::path::Path::new(env!("CARGO_MANIFEST_DIR")))
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
fn world_map_open_visible_onupdate_handlers_stay_within_initial_inventory_ceiling() {
    test_timeout! {
        let env = load_settled_game_ui();
        env.fire_on_update(0.016).unwrap();
        let before_count = {
            let state = env.state().borrow();
            state.visible_on_update_cache.clone().unwrap_or_default().len()
        };
        env.exec("ToggleWorldMap()").unwrap();

        for _ in 0..10 {
            env.fire_on_update(0.016).unwrap();
        }

        let (visible_count, visible_names): (usize, Vec<String>) = {
            let state = env.state().borrow();
            let visible_ids = state.visible_on_update_cache.clone().unwrap_or_default();
            let names = visible_ids
                .iter()
                .map(|id| {
                    state
                        .widgets
                        .get(*id)
                        .and_then(|frame| frame.name.clone().or_else(|| frame.parent_key.clone()))
                        .unwrap_or_else(|| format!("id:{id}"))
                })
                .collect::<Vec<_>>();
            (visible_ids.len(), names)
        };
        let added_count = visible_count.saturating_sub(before_count);

        assert!(
            added_count <= 32,
            "world-map open should add at most 32 visible OnUpdate handlers over the settled baseline (before {before_count}, after {visible_count}, added {added_count}: {:?})",
            visible_names
        );
    }
}
