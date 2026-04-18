//! Benchmark binary: load UI then measure spellbook first-open phases.

use std::path::PathBuf;
use std::time::Instant;
use wow_ui_sim::loader::{discover_blizzard_addons, load_addon};
use wow_ui_sim::lua_api::WowLuaEnv;

fn main() {
    let env = load_base_ui();

    eprintln!("=== Spellbook demand-load path ===");
    let total_start = Instant::now();
    measure_step(
        &env,
        "LoadAddOn(Blizzard_PlayerSpells)",
        "assert(C_AddOns.LoadAddOn('Blizzard_PlayerSpells'))",
    );
    measure_step(
        &env,
        "TrySetTab(SpellBook)",
        "assert(PlayerSpellsFrame:TrySetTab(PlayerSpellsUtil.FrameTabs.SpellBook))",
    );
    measure_step(
        &env,
        "ShowUIPanel(PlayerSpellsFrame)",
        "ShowUIPanel(PlayerSpellsFrame)",
    );
    eprintln!("Demand-load total: {:.2?}", total_start.elapsed());

    measure_step(
        &env,
        "HideUIPanel(PlayerSpellsFrame)",
        "HideUIPanel(PlayerSpellsFrame)",
    );

    eprintln!("\n=== Spellbook already-loaded toggle ===");
    let reopen_start = Instant::now();
    measure_step(
        &env,
        "ToggleSpellBookFrame()",
        "PlayerSpellsUtil.ToggleSpellBookFrame()",
    );
    eprintln!("Already-loaded open total: {:.2?}", reopen_start.elapsed());
}

fn load_base_ui() -> WowLuaEnv {
    let env = WowLuaEnv::new().expect("Failed to create Lua environment");
    env.set_screen_size(1024.0, 768.0);

    let ui = PathBuf::from("./Interface/BlizzardUI");
    {
        let mut state = env.state().borrow_mut();
        state.addon_base_paths = vec![ui.clone()];
    }

    let addons = discover_blizzard_addons(&ui);
    for (name, toc_path) in &addons {
        if let Err(error) = load_addon(&env.loader_env(), toc_path) {
            eprintln!("[load {name}] FAILED: {error}");
        }
    }
    env.apply_post_load_workarounds();

    wow_ui_sim::startup::fire_startup_events(&env);
    env.apply_post_event_workarounds();
    wow_ui_sim::startup::process_pending_timers(&env);
    wow_ui_sim::startup::fire_one_on_update_tick(&env);

    env
}

fn measure_step(env: &WowLuaEnv, label: &str, lua: &str) {
    let started = Instant::now();
    env.exec(lua)
        .unwrap_or_else(|error| panic!("{label} failed: {error}"));
    eprintln!("{label}: {:.2?}", started.elapsed());
}
