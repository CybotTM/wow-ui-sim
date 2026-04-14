mod common;

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
fn compact_raid_manager_stays_hidden_and_stops_visible_onupdate_when_solo() {
    test_timeout! {
        let env = load_settled_game_ui();
        env.exec("A_Admin.SetPartySize(0)").unwrap();

        let (button_name, manager_shown, manager_visible, button_visible, in_group): (
            String,
            bool,
            bool,
            bool,
            bool,
        ) = env
            .eval(
                r#"
                local manager = CompactRaidFrameManager
                if not manager then error("missing_manager") end

                local bottom = manager.BottomButtons
                if not bottom then error("missing_bottom_buttons") end

                local button
                for _, child in ipairs({ bottom:GetChildren() }) do
                    local name = child and child.GetName and child:GetName()
                    if name and name:find("LeaveInstanceGroupButton", 1, true) then
                        button = child
                        break
                    end
                end

                if not button then
                    error("missing_leave_instance_group_button")
                end

                return button:GetName() or "",
                    manager:IsShown(),
                    manager:IsVisible(),
                    button:IsVisible(),
                    IsInGroup()
                "#,
            )
            .unwrap();

        assert!(
            !in_group,
            "A_Admin.SetPartySize(0) should make the simulated player solo"
        );
        assert!(
            !manager_shown,
            "CompactRaidFrameManager should hide after switching the simulated player to solo"
        );
        assert!(
            !manager_visible,
            "CompactRaidFrameManager should not stay visible after switching the simulated player to solo"
        );
        assert!(
            !button_visible,
            "leave-instance button should not stay visible after switching the simulated player to solo"
        );

        env.fire_on_update(0.016).unwrap();

        let state = env.state();
        let state = state.borrow();
        let button_id = state
            .widgets
            .get_id_by_name(&button_name)
            .expect("leave-instance button should have a runtime name");

        assert!(
            state.on_update_frames.contains(&button_id),
            "leave-instance button should still register an OnUpdate handler so this test checks the visibility gate"
        );

        let visible_ids = state.visible_on_update_cache.clone().unwrap_or_default();
        assert!(
            !visible_ids.contains(&button_id),
            "leave-instance button should not stay in visible OnUpdate cache when solo"
        );
    }
}
