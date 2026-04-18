//! Benchmark binary: measure spellbook open on the real GUI app path.

use std::path::PathBuf;

use wow_ui_sim::iced_app::{BenchmarkPhase, benchmark_spellbook_open_in_gui};
use wow_ui_sim::loader::{discover_blizzard_addons, load_addon};
use wow_ui_sim::lua_api::WowLuaEnv;

fn main() {
    let env = load_base_ui();
    let report = benchmark_spellbook_open_in_gui(env).expect("spellbook GUI benchmark failed");

    print_phase(&report.startup_idle);
    print_phase(&report.first_open);
    print_phase(&report.first_close);
    print_phase(&report.second_open);
}

fn load_base_ui() -> WowLuaEnv {
    let env = WowLuaEnv::new().expect("failed to create Lua environment");
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
    env
}

fn print_phase(phase: &BenchmarkPhase) {
    eprintln!("=== {} ===", phase.name);
    eprintln!("keypress: {:?}", phase.keypress_elapsed);
    eprintln!("settle:   {:?}", phase.settle_elapsed);
    eprintln!("ticks:    {:?}", phase.tick_elapsed);
    eprintln!("draws:    {:?}", phase.draw_elapsed);
    eprintln!("frames:   {}", phase.frames);
    eprintln!(
        "textures: rgba={} bc={} max_pending_dirty_ids={}",
        phase.textures_loaded, phase.bc_textures_loaded, phase.max_pending_dirty_ids
    );
    eprintln!("shown:    {}", phase.spellbook_shown);
    eprintln!();
}
