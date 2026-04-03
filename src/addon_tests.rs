//! `run-tests` subcommand: run Lua test files from an addon's tests/ directory.
//!
//! Loads the TestFramework addon (assertions + test/async_test registration),
//! then executes each .lua file and reports per-test pass/fail.

use crate::lua_api::WowLuaEnv;
use crate::startup::{fire_one_on_update_tick, process_pending_timers};
use std::path::PathBuf;

const MAX_ASYNC_TICKS: u32 = 500;

/// Reset test registry between files (TestFramework globals stay alive).
const TEST_RESET: &str = r#"
__addon_tests = {}
__addon_test_results = {}
__async_done = false
__async_error = nil
"#;

/// Run sync tests, store results. Skips async tests (marked for Rust to handle).
const SYNC_RUNNER: &str = r#"
__addon_test_results = {}
for i, t in ipairs(__addon_tests) do
    if not t.async then
        local ok, err = pcall(t.fn)
        __addon_test_results[i] = {
            name = t.name,
            ok = ok,
            err = ok and "" or tostring(err),
        }
    end
end
"#;

/// Start a single async test by index. Sets up __async_done tracking.
const ASYNC_START: &str = r#"
local idx = ...
local t = __addon_tests[idx]
__async_done = false
__async_error = nil
local function done(assertion_fn)
    if assertion_fn then
        local ok, err = pcall(assertion_fn)
        if not ok then
            __async_error = tostring(err)
        end
    end
    __async_done = true
end
local ok, err = pcall(t.fn, done)
if not ok then
    __async_error = tostring(err)
    __async_done = true
end
"#;

/// Load the TestFramework addon (provides test/async_test/assertions).
fn load_test_framework(env: &WowLuaEnv) {
    let toc_path = PathBuf::from("./Interface/AddOns/TestFramework/TestFramework.toc");
    if !toc_path.exists() {
        eprintln!("TestFramework addon not found at {}", toc_path.display());
        std::process::exit(1);
    }
    match crate::loader::load_addon(&env.loader_env(), &toc_path) {
        Ok(r) => {
            for w in &r.warnings {
                eprintln!("  [!] {w}");
            }
        }
        Err(e) => {
            eprintln!("Failed to load TestFramework: {e}");
            std::process::exit(1);
        }
    }
}

/// Run all .lua test files from `Interface/AddOns/<addon_name>/tests/`.
pub fn run_addon_tests(env: &WowLuaEnv, addon_name: &str, exec_lua: Option<&str>) {
    run_exec_lua(env, exec_lua);
    load_test_framework(env);
    let (tests_dir, test_files) = load_test_files(addon_name);
    eprintln!(
        "Running {} test file(s) from {}\n",
        test_files.len(),
        tests_dir.display()
    );

    let mut total_passed = 0u32;
    let mut total_failed = 0u32;
    for path in &test_files {
        let (passed, failed) = report_test_file_result(env, path);
        total_passed += passed;
        total_failed += failed;
        flush_console(env);
    }
    print_summary_and_exit(total_passed, total_failed);
}

fn run_exec_lua(env: &WowLuaEnv, exec_lua: Option<&str>) {
    if let Some(code) = exec_lua {
        if let Err(e) = env.exec(code) {
            eprintln!("[exec-lua] error: {e}");
        }
    }
}

fn load_test_files(addon_name: &str) -> (PathBuf, Vec<PathBuf>) {
    let tests_dir = PathBuf::from(format!("./Interface/AddOns/{addon_name}/tests"));
    if !tests_dir.exists() {
        eprintln!("No tests directory found: {}", tests_dir.display());
        std::process::exit(1);
    }

    let mut test_files = match collect_test_files(&tests_dir) {
        Ok(files) => files,
        Err(e) => {
            eprintln!("{e}");
            std::process::exit(1);
        }
    };
    test_files.sort();
    (tests_dir, test_files)
}

fn report_test_file_result(env: &WowLuaEnv, path: &PathBuf) -> (u32, u32) {
    let file_name = path.file_name().unwrap().to_string_lossy();
    match run_test_file(env, path) {
        Ok((passed, failed)) => {
            if failed == 0 {
                eprintln!("  \x1b[32m\u{2713}\x1b[0m {file_name} ({passed} tests)");
            }
            (passed, failed)
        }
        Err(e) => {
            eprintln!("  \x1b[31m\u{2717}\x1b[0m {file_name} (load error)");
            eprintln!("    {e}");
            (0, 1)
        }
    }
}

fn print_summary_and_exit(total_passed: u32, total_failed: u32) {
    let total = total_passed + total_failed;
    eprintln!(
        "\n{total} tests, \x1b[32m{total_passed} passed\x1b[0m, \x1b[31m{total_failed} failed\x1b[0m"
    );
    if total_failed > 0 {
        std::process::exit(1);
    }
}

/// Collect .lua files from a tests directory.
fn collect_test_files(dir: &PathBuf) -> Result<Vec<PathBuf>, String> {
    let files: Vec<PathBuf> = std::fs::read_dir(dir)
        .map_err(|e| format!("Failed to read {}: {e}", dir.display()))?
        .filter_map(|entry| entry.ok())
        .map(|entry| entry.path())
        .filter(|p| p.extension().is_some_and(|ext| ext == "lua"))
        .collect();

    if files.is_empty() {
        return Err(format!("No .lua test files found in {}", dir.display()));
    }
    Ok(files)
}

/// Run a single test file: reset registry, load file, run sync then async tests.
fn run_test_file(env: &WowLuaEnv, path: &PathBuf) -> Result<(u32, u32), String> {
    let file_name = path.file_name().unwrap().to_string_lossy();
    let code = std::fs::read_to_string(path).map_err(|e| format!("read error: {e}"))?;

    // Reset test registry between files
    env.exec(TEST_RESET)
        .map_err(|e| format!("reset error: {e}"))?;

    // Load the test file (registers tests via test() calls)
    let chunk_name = format!("@{}", path.display());
    env.exec_named(&code, &chunk_name)
        .map_err(|e| format!("{e}"))?;

    // Run sync tests
    env.exec(SYNC_RUNNER)
        .map_err(|e| format!("runner error: {e}"))?;

    // Read sync results
    let (mut passed, mut failed) = read_test_results(env, &file_name)?;

    // Run async tests one by one with tick loop
    let async_indices = get_async_test_indices(env)?;
    for idx in async_indices {
        let name = get_test_name(env, idx)?;
        match run_async_test(env, idx) {
            Ok(()) => passed += 1,
            Err(e) => {
                eprintln!("  \x1b[31m\u{2717}\x1b[0m {file_name} > {name}");
                eprintln!("    {e}");
                failed += 1;
            }
        }
    }

    Ok((passed, failed))
}

/// Get indices of async tests from __addon_tests.
fn get_async_test_indices(env: &WowLuaEnv) -> Result<Vec<i64>, String> {
    let lua = env.lua();
    let tests: mlua::Table = lua
        .globals()
        .get("__addon_tests")
        .map_err(|e| format!("failed to read __addon_tests: {e}"))?;

    let mut indices = Vec::new();
    for pair in tests.pairs::<i64, mlua::Table>() {
        let (idx, entry) = pair.map_err(|e| format!("{e}"))?;
        let is_async: bool = entry.get("async").unwrap_or(false);
        if is_async {
            indices.push(idx);
        }
    }
    Ok(indices)
}

/// Get test name by index from __addon_tests.
fn get_test_name(env: &WowLuaEnv, idx: i64) -> Result<String, String> {
    let lua = env.lua();
    let tests: mlua::Table = lua
        .globals()
        .get("__addon_tests")
        .map_err(|e| format!("{e}"))?;
    let entry: mlua::Table = tests.get(idx).map_err(|e| format!("{e}"))?;
    entry.get("name").map_err(|e| format!("{e}"))
}

/// Run a single async test: call fn(done), tick until __async_done or timeout.
fn run_async_test(env: &WowLuaEnv, idx: i64) -> Result<(), String> {
    let chunk = env.lua().load(ASYNC_START);
    chunk.call::<()>(idx).map_err(|e| format!("{e}"))?;

    for _ in 0..MAX_ASYNC_TICKS {
        let done: bool = env.eval("__async_done").unwrap_or(false);
        if done {
            break;
        }
        fire_one_on_update_tick(env);
        process_pending_timers(env);
        flush_console(env);
    }

    let done: bool = env.eval("__async_done").unwrap_or(false);
    if !done {
        return Err(format!("timed out after {MAX_ASYNC_TICKS} ticks"));
    }

    let err: String = env.eval("__async_error or ''").unwrap_or_default();
    if err.is_empty() { Ok(()) } else { Err(err) }
}

/// Read `__addon_test_results` from Lua and print failures.
fn read_test_results(env: &WowLuaEnv, file_name: &str) -> Result<(u32, u32), String> {
    let lua = env.lua();
    let results: mlua::Table = lua
        .globals()
        .get("__addon_test_results")
        .map_err(|e| format!("failed to read results: {e}"))?;

    let mut passed = 0u32;
    let mut failed = 0u32;

    for pair in results.pairs::<i64, mlua::Table>() {
        let (_, entry) = pair.map_err(|e| format!("result iteration error: {e}"))?;
        let name: String = entry.get("name").unwrap_or_default();
        let ok: bool = entry.get("ok").unwrap_or(false);
        if ok {
            passed += 1;
        } else {
            let err: String = entry.get("err").unwrap_or_default();
            eprintln!("  \x1b[31m\u{2717}\x1b[0m {file_name} > {name}");
            eprintln!("    {err}");
            failed += 1;
        }
    }

    Ok((passed, failed))
}

/// Flush Lua print() output from console_output to stderr.
fn flush_console(env: &WowLuaEnv) {
    let mut state = env.state().borrow_mut();
    for line in state.console_output.drain(..) {
        eprintln!("{line}");
    }
}
