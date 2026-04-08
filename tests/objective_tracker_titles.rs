mod common;

use std::path::PathBuf;
use wow_ui_sim::loader::{discover_blizzard_addons, load_addon};
use wow_ui_sim::lua_api::WowLuaEnv;

fn blizzard_ui_dir() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("Interface/BlizzardUI")
}

fn setup_full_ui() -> WowLuaEnv {
    let env = WowLuaEnv::new().expect("Failed to create Lua environment");
    env.set_screen_size(1024.0, 768.0);

    let ui = blizzard_ui_dir();
    {
        let mut state = env.state().borrow_mut();
        state.addon_base_paths = vec![ui.clone()];
    }

    for (name, toc_path) in &discover_blizzard_addons(&ui) {
        if let Err(err) = load_addon(&env.loader_env(), toc_path) {
            eprintln!("[load {name}] FAILED: {err}");
        }
    }

    env.apply_post_load_workarounds();
    wow_ui_sim::startup::settle_headless_startup(&env);
    env
}

#[test]
fn quest_objective_tracker_populates_titles_via_normal_async_load_path() {
    let env = setup_full_ui();

    let titles: String = env
        .eval(
            r#"
            local titles = {}
            for _, blocks in pairs(QuestObjectiveTracker.usedBlocks or {}) do
                for _, block in pairs(blocks) do
                    local text = block.HeaderText and block.HeaderText:GetText()
                    if text and text ~= "" then
                        table.insert(titles, text)
                    end
                end
            end
            table.sort(titles)
            return table.concat(titles, "|")
            "#,
        )
        .unwrap();

    assert_eq!(titles, "Defending the Gates|Supply Run|The Lost Expedition");
}
