use std::path::{Path, PathBuf};

use wow_ui_sim::loader::{discover_blizzard_addons, load_addon};
use wow_ui_sim::lua_api::WowLuaEnv;
use wow_ui_sim::startup::{fire_one_on_update_tick, fire_startup_events, process_pending_timers};

fn blizzard_ui_dir() -> PathBuf {
    wow_ui_sim::client_profile::blizzard_ui_addons_dir_under(std::path::Path::new(env!(
        "CARGO_MANIFEST_DIR"
    )))
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
fn groupfinder_queue_frames_keep_party_backfill_parent_keys_after_startup() {
    let env = full_game_env_after_startup();

    let result: String = env
        .eval(
            r#"
            local checks = {
                { "LFDQueueFrame", "LFDQueueFramePartyBackfill" },
                { "RaidFinderQueueFrame", "RaidFinderQueueFramePartyBackfill" },
                { "ScenarioQueueFrame", "ScenarioQueueFramePartyBackfill" },
            }

            for _, check in ipairs(checks) do
                local parent = _G[check[1]]
                local child = _G[check[2]]
                if parent == nil then
                    return "missing_parent:" .. check[1]
                end
                if child == nil then
                    return "missing_child:" .. check[2]
                end
                if parent.PartyBackfill == nil then
                    return "missing_parent_key:" .. check[1]
                end
                if parent.PartyBackfill ~= child then
                    return "wrong_child:" .. check[1]
                end
            end

            return "ok"
            "#,
        )
        .unwrap();

    assert_eq!(
        result, "ok",
        "queue frames should expose PartyBackfill via the parentKey Lua field after startup"
    );
}

#[test]
fn groupfinder_backfill_update_uses_real_party_backfill_frames_after_startup() {
    let env = full_game_env_after_startup();

    let result: String = env
        .eval(
            r#"
            local checks = {
                { "LFDQueueFrame", false },
                { "RaidFinderQueueFrame", true },
                { "ScenarioQueueFrame", true },
            }

            for _, check in ipairs(checks) do
                local parent = _G[check[1]]
                if parent == nil then
                    return "missing_parent:" .. check[1]
                end

                local backfill = parent.PartyBackfill
                if backfill == nil then
                    return "missing_backfill:" .. check[1]
                end

                local ok, err = pcall(LFGBackfillCover_Update, backfill, check[2])
                if not ok then
                    return "update_failed:" .. check[1] .. ":" .. tostring(err)
                end
            end

            return "ok"
            "#,
        )
        .unwrap();

    assert_eq!(
        result, "ok",
        "real LFGBackfillCover_Update calls should work without the removed nil-self workaround"
    );
}
