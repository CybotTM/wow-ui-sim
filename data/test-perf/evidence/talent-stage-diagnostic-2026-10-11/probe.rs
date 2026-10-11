// Private observation-only diagnostic; compile as Rust edition 2024.
// No budgets, native parity claims, or attribution to a loaded addon suite.
use std::fmt::Display;
use std::io::{self, Write};
use std::path::Path;
use std::time::Instant;

use wow_ui_sim::loader::{discover_blizzard_addons_for_screen, load_addon};
use wow_ui_sim::lua_api::WowLuaEnv;
use wow_ui_sim::screen::ScreenKind;
use wow_ui_sim::startup::settle_headless_startup;

const REPO_ROOT: &str = "/home/osso/Projects/wow/wow-ui-sim";
const BASELINE_TICKS: usize = 20;
const TALENT_SETTLE_TICKS: usize = 10;
const TALENT_STEADY_TICKS: usize = 24;

struct Counts {
    tracked: usize,
    visible: Option<usize>,
}

struct Sample {
    phase: &'static str,
    index: usize,
    timer_callbacks: usize,
    timers_ns: u128,
    layout_ns: u128,
    onupdate_ns: u128,
    total_ns: u128,
    before: Counts,
    after: Counts,
}

fn context_error(context: impl Display, error: impl Display) -> io::Error {
    io::Error::other(format!("{context}: {error}"))
}

fn load_settled_game_ui() -> io::Result<WowLuaEnv> {
    let env = WowLuaEnv::new()
        .map_err(|error| context_error("Failed to create Lua environment", error))?;
    env.set_screen_size(1024.0, 768.0);
    env.set_screen_mode(ScreenKind::Game);
    // Explicit test repository root replaces that test's CARGO_MANIFEST_DIR.
    let addon_path = wow_ui_sim::client_profile::blizzard_ui_addons_dir_under(Path::new(REPO_ROOT));
    env.state().borrow_mut().addon_base_paths = vec![addon_path.clone()];

    let addons = discover_blizzard_addons_for_screen(&addon_path, ScreenKind::Game);
    for (name, toc_path) in &addons {
        load_addon(&env.loader_env(), toc_path).map_err(|error| {
            context_error(format!("Failed to load Blizzard addon {name}"), error)
        })?;
    }

    env.apply_post_load_workarounds();
    settle_headless_startup(&env);
    Ok(env)
}

fn open_talent_panel(env: &WowLuaEnv) -> io::Result<()> {
    env.exec("PlayerSpellsUtil.ToggleClassTalentFrame()")
        .map_err(|error| context_error("Talent panel toggle failed", error))?;
    let shown: bool = env
        .eval(
            "return PlayerSpellsFrame ~= nil and PlayerSpellsFrame:IsShown() and PlayerSpellsFrame.TalentsFrame ~= nil and PlayerSpellsFrame.TalentsFrame:IsShown()",
        )
        .map_err(|error| context_error("Talent panel visibility query failed", error))?;
    assert!(shown, "Talent panel should be visible after opening");
    Ok(())
}

fn capture_counts(env: &WowLuaEnv) -> Counts {
    let state = env.state().borrow();
    // Snapshot stored tracking/cache state without forcing a rebuild. None is
    // an absent cache, not zero visible frames; these are not callback counts.
    Counts {
        tracked: state.on_update_frames.len(),
        visible: state.visible_on_update_cache.as_ref().map(Vec::len),
    }
}

fn process_timers_timed(
    env: &WowLuaEnv,
    phase: &'static str,
    index: usize,
) -> io::Result<(usize, u128)> {
    let started = Instant::now();
    let result = env.process_timers();
    let duration_ns = started.elapsed().as_nanos();
    let callbacks = result
        .map_err(|error| context_error(format!("{phase}[{index}] process_timers failed"), error))?;
    Ok((callbacks, duration_ns))
}

fn sample_tick(env: &WowLuaEnv, phase: &'static str, index: usize) -> io::Result<Sample> {
    let before = capture_counts(env);
    let total_started = Instant::now();

    let (timer_callbacks, timers_ns) = process_timers_timed(env, phase, index)?;

    let layout_started = Instant::now();
    env.state().borrow_mut().ensure_layout_rects();
    let layout_ns = layout_started.elapsed().as_nanos();

    let onupdate_started = Instant::now();
    let onupdate_result = env.fire_on_update(0.016);
    let onupdate_ns = onupdate_started.elapsed().as_nanos();
    let total_ns = total_started.elapsed().as_nanos();
    onupdate_result
        .map_err(|error| context_error(format!("{phase}[{index}] fire_on_update failed"), error))?;

    let after = capture_counts(env);
    Ok(Sample {
        phase,
        index,
        timer_callbacks,
        timers_ns,
        layout_ns,
        onupdate_ns,
        total_ns,
        before,
        after,
    })
}

fn sample_phase(
    env: &WowLuaEnv,
    phase: &'static str,
    ticks: usize,
    rows: &mut Vec<Sample>,
) -> io::Result<()> {
    for index in 0..ticks {
        rows.push(sample_tick(env, phase, index)?);
    }
    Ok(())
}

fn optional_count_json(count: Option<usize>) -> String {
    match count {
        Some(count) => count.to_string(),
        None => "null".to_owned(),
    }
}

fn emit_jsonl(rows: &[Sample]) -> io::Result<()> {
    let mut output = io::BufWriter::new(io::stdout().lock());
    for row in rows {
        writeln!(
            output,
            "{{\"phase\":\"{}\",\"index\":{},\"timer_callbacks\":{},\"timers_ns\":{},\"layout_ns\":{},\"onupdate_ns\":{},\"total_ns\":{},\"tracked_before\":{},\"tracked_after\":{},\"visible_before\":{},\"visible_after\":{}}}",
            row.phase,
            row.index,
            row.timer_callbacks,
            row.timers_ns,
            row.layout_ns,
            row.onupdate_ns,
            row.total_ns,
            row.before.tracked,
            row.after.tracked,
            optional_count_json(row.before.visible),
            optional_count_json(row.after.visible),
        )?;
    }
    output.flush()
}

fn main() -> io::Result<()> {
    let env = load_settled_game_ui()?;
    let mut rows = Vec::with_capacity(BASELINE_TICKS + TALENT_SETTLE_TICKS + TALENT_STEADY_TICKS);
    sample_phase(&env, "baseline", BASELINE_TICKS, &mut rows)?;
    open_talent_panel(&env)?;
    sample_phase(&env, "talent_settle", TALENT_SETTLE_TICKS, &mut rows)?;
    sample_phase(&env, "talent_steady", TALENT_STEADY_TICKS, &mut rows)?;
    // All 54 ticks finish before diagnostic output. Indices are phase-local,
    // zero-based. Timers use real clock scheduling, not a synthetic 16 ms advance.
    // timer_callbacks is process_timers' returned count (including due timed
    // signals when enabled), not successful callback completions. onupdate_ns
    // covers the whole public call, including any internal layout/dispatch work.
    // total_ns includes stage measurement overhead, but excludes count snapshots
    // and row storage. No diagnostic IO occurs within measured intervals;
    // simulator callbacks may themselves log as part of the observed workload.
    emit_jsonl(&rows)
}
