// Private observation-only diagnostic. Compile as Rust edition 2024.
// Main must launch with WOW_SIM_PROFILE_LUA_FILES=1; this probe never sets env.
use std::fmt::Display;
use std::io::{self, Write};
use std::path::Path;
use std::time::Instant;

use serde_json::{Value, json};
use wow_ui_sim::loader::{
    LuaFileTiming, discover_blizzard_addons_for_screen, load_addon, lua_file_timings_snapshot,
};
use wow_ui_sim::lua_api::WowLuaEnv;
use wow_ui_sim::screen::ScreenKind;
use wow_ui_sim::startup::fire_startup_events_for_screen;

const REPO_ROOT: &str = "/home/osso/Projects/wow/wow-ui-sim";
const READY_QUERY: &str = "return UIParent ~= nil and PlayerFrame ~= nil and IsLoggedIn()";

#[derive(Default)]
struct Counters {
    records: usize,
    hits: u64,
    misses: u64,
    lookup_misses: u64,
    replay_failures: u64,
}

struct Observation {
    phase: &'static str,
    addon: Option<(usize, String)>,
    duration_ns: u128,
    before: Counters,
    after: Counters,
    files: Vec<LuaFileTiming>,
    error: Option<String>,
}

fn context_error(context: impl Display, error: impl Display) -> io::Error {
    io::Error::other(format!("{context}: {error}"))
}

fn count_snapshot(snapshot: &[LuaFileTiming]) -> Counters {
    snapshot.iter().fold(
        Counters {
            records: snapshot.len(),
            ..Counters::default()
        },
        |mut counts, timing| {
            counts.hits += u64::from(timing.cache_hits);
            counts.misses += u64::from(timing.cache_misses);
            counts.lookup_misses += u64::from(timing.cache_lookup_misses);
            counts.replay_failures += u64::from(timing.cache_replay_failures);
            counts
        },
    )
}

fn measure<T>(
    rows: &mut Vec<Observation>,
    phase: &'static str,
    addon: Option<(usize, String)>,
    operation: impl FnOnce() -> io::Result<T>,
) -> io::Result<T> {
    let before = count_snapshot(&lua_file_timings_snapshot());
    let started = Instant::now();
    let result = operation();
    let duration_ns = started.elapsed().as_nanos();
    let mut snapshot = lua_file_timings_snapshot();
    if snapshot.len() < before.records {
        return Err(io::Error::other("Lua timing snapshot unexpectedly shrank"));
    }
    let after = count_snapshot(&snapshot);
    let files = snapshot.split_off(before.records);
    rows.push(Observation {
        phase,
        addon,
        duration_ns,
        before,
        after,
        files,
        error: result.as_ref().err().map(ToString::to_string),
    });
    result
}

fn initialize_game_env() -> io::Result<WowLuaEnv> {
    let env = WowLuaEnv::new()
        .map_err(|error| context_error("Lua environment initialization failed", error))?;
    env.set_screen_size(1024.0, 768.0);
    env.set_screen_mode(ScreenKind::Game);
    let ui = wow_ui_sim::client_profile::blizzard_ui_addons_dir_under(Path::new(REPO_ROOT));
    env.state().borrow_mut().addon_base_paths = vec![ui];
    Ok(env)
}

fn observe_startup(rows: &mut Vec<Observation>) -> io::Result<WowLuaEnv> {
    let env = measure(rows, "initialization", None, initialize_game_env)?;
    let ui = wow_ui_sim::client_profile::blizzard_ui_addons_dir_under(Path::new(REPO_ROOT));
    let addons = measure(rows, "discovery", None, || {
        Ok(discover_blizzard_addons_for_screen(&ui, ScreenKind::Game))
    })?;
    for (index, (name, toc_path)) in addons.iter().enumerate() {
        measure(rows, "addon_load", Some((index + 1, name.clone())), || {
            load_addon(&env.loader_env(), toc_path).map_err(|error| {
                context_error(
                    format!(
                        "Addon {} ({name}, {}) load failed",
                        index + 1,
                        toc_path.display()
                    ),
                    error,
                )
            })
        })?;
    }
    measure(rows, "postworkarounds", None, || {
        env.apply_post_load_workarounds();
        Ok(())
    })?;
    measure(rows, "startup_events", None, || {
        fire_startup_events_for_screen(&env, ScreenKind::Game);
        Ok(())
    })?;
    Ok(env)
}

fn counters_json(counts: &Counters) -> Value {
    json!({
        "records": counts.records,
        "cache_hits": counts.hits,
        "cache_misses": counts.misses,
        "cache_lookup_misses": counts.lookup_misses,
        "cache_replay_failures": counts.replay_failures,
    })
}

fn file_json(timing: &LuaFileTiming) -> Value {
    json!({
        "chunk": timing.chunk_name,
        "compile_ns": timing.compile_time.as_nanos(),
        "call_ns": timing.call_time.as_nanos(),
        "cache_hits": timing.cache_hits,
        "cache_misses": timing.cache_misses,
        "cache_lookup_misses": timing.cache_lookup_misses,
        "cache_replay_failures": timing.cache_replay_failures,
    })
}

fn observation_json(row: &Observation) -> Value {
    let ordinal = row.addon.as_ref().map(|(ordinal, _)| *ordinal);
    let name = row.addon.as_ref().map(|(_, name)| name.as_str());
    let files: Vec<Value> = row.files.iter().map(file_json).collect();
    json!({
        "kind": "phase",
        "phase": row.phase,
        "addon_ordinal": ordinal,
        "addon_name": name,
        "duration_ns": row.duration_ns,
        "snapshot_before": counters_json(&row.before),
        "snapshot_after": counters_json(&row.after),
        "new_lua_file_timings": files,
        "error": row.error,
    })
}

fn emit_jsonl(rows: &[Observation], summary: &Value) -> io::Result<()> {
    let mut output = io::BufWriter::new(io::stdout().lock());
    for row in rows {
        writeln!(output, "{}", observation_json(row))?;
    }
    writeln!(output, "{summary}")?;
    output.flush()
}

fn require_profiling() -> io::Result<()> {
    let value = std::env::var_os("WOW_SIM_PROFILE_LUA_FILES")
        .ok_or_else(|| io::Error::other("Setup failed: launch with WOW_SIM_PROFILE_LUA_FILES=1"))?;
    let value = value.to_string_lossy();
    if value == "0" || value.eq_ignore_ascii_case("false") {
        return Err(io::Error::other(
            "Setup failed: WOW_SIM_PROFILE_LUA_FILES disables profiling",
        ));
    }
    Ok(())
}

fn main() -> io::Result<()> {
    require_profiling()?;
    let mut rows = Vec::new();
    let started = Instant::now();
    let startup = observe_startup(&mut rows);
    let enclosing_ns = started.elapsed().as_nanos();
    // Readiness is queried only after the enclosing startup clock stops.
    let ready = startup.and_then(|env| {
        env.eval::<bool>(READY_QUERY)
            .map_err(|error| context_error("Post-sequence readiness eval failed", error))
    });
    let summary = json!({
        "kind": "startup_summary",
        "enclosing_startup_ns": enclosing_ns,
        "sequence_completed": rows.last().is_some_and(|row| row.phase == "startup_events" && row.error.is_none()),
        "ready": ready.as_ref().ok().copied(),
        "error": ready.as_ref().err().map(ToString::to_string),
    });
    emit_jsonl(&rows, &summary)?;
    ready.map(|_| ())
}

// Phase clocks exclude diagnostic snapshots/storage; enclosing startup includes
// that observer overhead (and path resolution). Normal simulator logging remains
// part of each measured call; diagnostics are buffered until measured work ends.
// Snapshot counters are cumulative; each phase emits only newly added records.
// File timings are dynamic-scope observations, not exclusive addon file costs.
// Nested loader activity belongs to the enclosing sequential load's scope.
// Discovery, postworkarounds and startup events expose no Result: their internal
// logged/callback errors are not promoted to probe failures. Returned load/eval
// errors retain partial observations; panics/abort cannot flush buffered rows.
// Ready=false is an observation, not failure/acceptance. No settling ticks run.
