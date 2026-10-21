use crate::common;

use std::path::PathBuf;

use wow_ui_sim::loader::{discover_blizzard_addons_for_screen, load_addon};
use wow_ui_sim::lua_api::WowLuaEnv;
use wow_ui_sim::screen::ScreenKind;
use wow_ui_sim::startup::settle_headless_startup;

fn blizzard_ui_dir() -> PathBuf {
    wow_ui_sim::client_profile::blizzard_ui_addons_dir_under(std::path::Path::new(env!(
        "CARGO_MANIFEST_DIR"
    )))
}

fn load_settled_game_ui() -> WowLuaEnv {
    let env = WowLuaEnv::new().expect("Failed to create Lua environment");
    env.set_screen_size(1024.0, 768.0);
    env.set_screen_mode(ScreenKind::Game);
    env.state().borrow_mut().addon_base_paths = vec![blizzard_ui_dir()];

    common::load_profile_framexml(&env);
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
fn game_time_frame_set_date_same_day_does_not_mark_render_dirty() {
    test_timeout! {
        let env = load_settled_game_ui();

        let watched_ids: Vec<_> = {
            let state = env.state().borrow();
            let frame_id = state.widgets.get_id_by_name("GameTimeFrame").unwrap();
            let frame = state.widgets.get(frame_id).unwrap();
            // Native Wrath texture slots are unnamed, not modern parentKeys.
            std::iter::once(frame_id).chain(frame.children.iter().copied()).collect()
        };

        env.exec("GameTimeFrame_SetDate()").unwrap();
        let _ = env.state().borrow().widgets.take_render_dirty_with_ids();

        env.exec("GameTimeFrame_SetDate()").unwrap();

        let (dirty_mask, dirty_ids) = env.state().borrow().widgets.take_render_dirty_with_ids();
        let dirty_ids = dirty_ids.unwrap_or_default();

        assert_eq!(
            dirty_mask, 0,
            "same-day GameTimeFrame_SetDate should not trigger render dirties"
        );
        assert!(
            watched_ids.iter().all(|id| !dirty_ids.contains(id)),
            "same-day GameTimeFrame_SetDate should not dirty the button or its calendar textures (got {:?})",
            dirty_ids
        );
    }
}
