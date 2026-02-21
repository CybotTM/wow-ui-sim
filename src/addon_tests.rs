//! `run-tests` subcommand: run Lua test files from an addon's tests/ directory.
//!
//! Each test file uses a Pest-like `test("name", fn)` syntax to register tests.
//! Sync tests: `test("name", function() ... end)`
//! Async tests: `test("name", function(done) ... done() end)` — detected via arg count.
//! The runner ticks OnUpdate/timers for async tests until done() is called or timeout.

use std::path::PathBuf;
use crate::lua_api::WowLuaEnv;
use crate::startup::{fire_one_on_update_tick, process_pending_timers};

const MAX_ASYNC_TICKS: u32 = 500;

/// Lua bootstrap: injects test(), async_test(), and assertEquals().
const TEST_BOOTSTRAP: &str = r#"
__addon_tests = {}
__addon_test_results = {}
__async_done = false
__async_error = nil
function test(name, fn)
    table.insert(__addon_tests, { name = name, fn = fn, async = false })
end
function async_test(name, fn)
    table.insert(__addon_tests, { name = name, fn = fn, async = true })
end
if not assertEquals then
    function assertEquals(expected, actual)
        if expected ~= actual then
            error(string.format("expected %s, got %s", tostring(expected), tostring(actual)), 2)
        end
    end
end
if not assertNotEquals then
    function assertNotEquals(expected, actual)
        if expected == actual then
            error(string.format("expected value to differ from %s", tostring(expected)), 2)
        end
    end
end
if not assertTrue then
    function assertTrue(value)
        if not value then
            error(string.format("expected truthy, got %s", tostring(value)), 2)
        end
    end
end
if not assertFalse then
    function assertFalse(value)
        if value then
            error(string.format("expected falsy, got %s", tostring(value)), 2)
        end
    end
end
if not assertNil then
    function assertNil(value)
        if value ~= nil then
            error(string.format("expected nil, got %s", tostring(value)), 2)
        end
    end
end
if not assertNotNil then
    function assertNotNil(value)
        if value == nil then
            error("expected non-nil value, got nil", 2)
        end
    end
end
if not assertError then
    function assertError(fn)
        local ok = pcall(fn)
        if ok then
            error("expected function to throw an error", 2)
        end
    end
end
if not assertContains then
    function assertContains(haystack, needle)
        if type(haystack) == "string" then
            if not haystack:find(needle, 1, true) then
                error(string.format("expected string to contain %q", needle), 2)
            end
        elseif type(haystack) == "table" then
            for _, v in pairs(haystack) do
                if v == needle then return end
            end
            error(string.format("expected table to contain %s", tostring(needle)), 2)
        else
            error(string.format("assertContains expects string or table, got %s", type(haystack)), 2)
        end
    end
end
if not assertCount then
    function assertCount(expected, tbl)
        local count = 0
        for _ in pairs(tbl) do count = count + 1 end
        if count ~= expected then
            error(string.format("expected %d elements, got %d", expected, count), 2)
        end
    end
end
if not assertType then
    function assertType(expected, value)
        local actual = type(value)
        if actual ~= expected then
            error(string.format("expected type %s, got %s", expected, actual), 2)
        end
    end
end
if not assertAlmostEquals then
    function assertAlmostEquals(expected, actual, tolerance)
        tolerance = tolerance or 0.001
        if math.abs(expected - actual) > tolerance then
            error(string.format("expected ~%s, got %s (tolerance %s)", tostring(expected), tostring(actual), tostring(tolerance)), 2)
        end
    end
end
do
    -- Deep table comparison helper (shared by assertTableEquals and assertTableContains)
    local function deep_equal(a, b)
        if a == b then return true end
        if type(a) ~= "table" or type(b) ~= "table" then return false end
        for k, v in pairs(a) do
            if not deep_equal(v, b[k]) then return false end
        end
        for k in pairs(b) do
            if a[k] == nil then return false end
        end
        return true
    end
    local function table_to_string(t, depth)
        depth = depth or 0
        if type(t) ~= "table" then return tostring(t) end
        if depth > 3 then return "{...}" end
        local parts = {}
        for k, v in pairs(t) do
            local key = type(k) == "string" and k or "[" .. tostring(k) .. "]"
            parts[#parts + 1] = key .. " = " .. table_to_string(v, depth + 1)
        end
        return "{ " .. table.concat(parts, ", ") .. " }"
    end
    if not assertTableEquals then
        function assertTableEquals(expected, actual)
            if not deep_equal(expected, actual) then
                error(string.format("expected %s, got %s",
                    table_to_string(expected), table_to_string(actual)), 2)
            end
        end
    end
    if not assertTableContains then
        function assertTableContains(tbl, subset)
            for k, v in pairs(subset) do
                if not deep_equal(v, tbl[k]) then
                    error(string.format("key %s: expected %s, got %s",
                        tostring(k), table_to_string(v), table_to_string(tbl[k])), 2)
                end
            end
        end
    end
end
if not assertStartsWith then
    function assertStartsWith(str, prefix)
        if type(str) ~= "string" then
            error(string.format("expected string, got %s", type(str)), 2)
        end
        if str:sub(1, #prefix) ~= prefix then
            error(string.format("expected %q to start with %q", str, prefix), 2)
        end
    end
end
if not assertEndsWith then
    function assertEndsWith(str, suffix)
        if type(str) ~= "string" then
            error(string.format("expected string, got %s", type(str)), 2)
        end
        if str:sub(-#suffix) ~= suffix then
            error(string.format("expected %q to end with %q", str, suffix), 2)
        end
    end
end
if not assertMatches then
    function assertMatches(str, pattern)
        if type(str) ~= "string" then
            error(string.format("expected string, got %s", type(str)), 2)
        end
        if not str:match(pattern) then
            error(string.format("expected %q to match pattern %q", str, pattern), 2)
        end
    end
end
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

    eprintln!(
        "Running {} test file(s) from {}\n",
        test_files.len(),
        tests_dir.display()
    );

    let mut total_passed = 0u32;
    let mut total_failed = 0u32;

    for path in &test_files {
        let file_name = path.file_name().unwrap().to_string_lossy();
        match run_test_file(env, path) {
            Ok((passed, failed)) => {
                total_passed += passed;
                total_failed += failed;
                if failed == 0 {
                    eprintln!("  \x1b[32m\u{2713}\x1b[0m {file_name} ({passed} tests)");
                }
            }
            Err(e) => {
                eprintln!("  \x1b[31m\u{2717}\x1b[0m {file_name} (load error)");
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

/// Run a single test file: bootstrap, load, run sync tests, then async tests.
fn run_test_file(env: &WowLuaEnv, path: &PathBuf) -> Result<(u32, u32), String> {
    let file_name = path.file_name().unwrap().to_string_lossy();
    let code =
        std::fs::read_to_string(path).map_err(|e| format!("read error: {e}"))?;

    // Reset test registry and inject test() function
    env.exec(TEST_BOOTSTRAP)
        .map_err(|e| format!("bootstrap error: {e}"))?;

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
    // Start the async test (calls fn(done))
    let chunk = env.lua().load(ASYNC_START);
    chunk
        .call::<()>(idx)
        .map_err(|e| format!("{e}"))?;

    // Tick until done or timeout
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
    if err.is_empty() {
        Ok(())
    } else {
        Err(err)
    }
}

/// Read `__addon_test_results` from Lua and print failures.
fn read_test_results(
    env: &WowLuaEnv,
    file_name: &str,
) -> Result<(u32, u32), String> {
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
