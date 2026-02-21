//! `run-tests` subcommand: run Lua test files from an addon's tests/ directory.
//!
//! Each test file uses a Pest-like `test("name", fn)` syntax to register tests.
//! The runner injects the `test()` global, loads each file to collect registrations,
//! then runs each test with pcall and reports per-test pass/fail.

use std::path::PathBuf;
use crate::lua_api::WowLuaEnv;

/// Lua bootstrap: injects `test()` and `assertEquals()` globals.
const TEST_BOOTSTRAP: &str = r#"
__addon_tests = {}
__addon_test_results = {}
function test(name, fn)
    table.insert(__addon_tests, { name = name, fn = fn })
end
if not assertEquals then
    function assertEquals(expected, actual)
        if expected ~= actual then
            error(string.format("expected %s, got %s", tostring(expected), tostring(actual)), 2)
        end
    end
end
"#;

/// Run collected tests one by one via pcall, store results in `__addon_test_results`.
const TEST_RUNNER: &str = r#"
__addon_test_results = {}
for i, t in ipairs(__addon_tests) do
    local ok, err = pcall(t.fn)
    __addon_test_results[i] = {
        name = t.name,
        ok = ok,
        err = ok and "" or tostring(err),
    }
end
"#;

/// Run all .lua test files from `Interface/AddOns/<addon_name>/tests/`.
pub fn run_addon_tests(env: &WowLuaEnv, addon_name: &str, exec_lua: Option<&str>) {
    if let Some(code) = exec_lua {
        if let Err(e) = env.exec(code) {
            eprintln!("[exec-lua] error: {e}");
        }
    }

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

    eprintln!("Running {} test file(s) from {}\n", test_files.len(), tests_dir.display());

    let mut total_passed = 0u32;
    let mut total_failed = 0u32;

    for path in &test_files {
        let file_name = path.file_name().unwrap().to_string_lossy();
        match run_test_file(env, path) {
            Ok((passed, failed)) => {
                total_passed += passed;
                total_failed += failed;
                if failed == 0 {
                    eprintln!("  \x1b[32m✓\x1b[0m {file_name} ({passed} tests)");
                }
            }
            Err(e) => {
                eprintln!("  \x1b[31m✗\x1b[0m {file_name} (load error)");
                eprintln!("    {e}");
                total_failed += 1;
            }
        }
        flush_console(env);
    }

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

/// Run a single test file: bootstrap test(), load file, execute collected tests.
/// Returns (passed, failed) counts.
fn run_test_file(env: &WowLuaEnv, path: &PathBuf) -> Result<(u32, u32), String> {
    let file_name = path.file_name().unwrap().to_string_lossy();
    let code = std::fs::read_to_string(path)
        .map_err(|e| format!("read error: {e}"))?;

    // Reset test registry and inject test() function
    env.exec(TEST_BOOTSTRAP)
        .map_err(|e| format!("bootstrap error: {e}"))?;

    // Load the test file (registers tests via test() calls)
    let chunk_name = format!("@{}", path.display());
    env.exec_named(&code, &chunk_name)
        .map_err(|e| format!("{e}"))?;

    // Run collected tests
    env.exec(TEST_RUNNER)
        .map_err(|e| format!("runner error: {e}"))?;

    // Read results from __addon_test_results
    read_test_results(env, &file_name)
}

/// Read `__addon_test_results` from Lua and print failures.
fn read_test_results(env: &WowLuaEnv, file_name: &str) -> Result<(u32, u32), String> {
    let lua = env.lua();
    let globals = lua.globals();
    let results: mlua::Table = globals
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
            eprintln!("  \x1b[31m✗\x1b[0m {file_name} > {name}");
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
