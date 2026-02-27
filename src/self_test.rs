//! `self-test` subcommand: run Wowless tests headlessly and report results to terminal.

use crate::lua_api::WowLuaEnv;
use crate::lua_errors::restore_stdout;
use crate::startup::{fire_one_on_update_tick, process_pending_timers};

/// Flush Lua print() output from console_output to stderr.
fn flush_console(env: &WowLuaEnv) {
    let mut state = env.state().borrow_mut();
    for line in state.console_output.drain(..) {
        eprintln!("{line}");
    }
}

/// Check if WowlessTestsDone is true in Lua globals.
fn tests_done(env: &WowLuaEnv) -> bool {
    env.eval("WowlessTestsDone or false").unwrap_or(false)
}

/// Loop OnUpdate ticks until Wowless completes or appears stuck.
/// Returns true if tests completed, false if timed out / stuck.
fn poll_until_done(env: &WowLuaEnv, max_ticks: u32) -> bool {
    let mut idle_ticks: u32 = 0;
    let mut prev_error_count: usize = 0;

    for _tick in 0..max_ticks {
        flush_console(env);
        if tests_done(env) { return true; }

        let errors_before = env.state().borrow().lua_errors.len();
        fire_one_on_update_tick(env);
        process_pending_timers(env);
        let errors_after = env.state().borrow().lua_errors.len();

        if errors_after == prev_error_count && errors_after == errors_before {
            idle_ticks += 1;
        } else {
            idle_ticks = 0;
        }
        prev_error_count = errors_after;

        if idle_ticks >= 500 {
            eprintln!("Wowless tests appear stuck (500 idle ticks), stopping");
            return false;
        }
    }
    false
}

/// Serialize WowlessTestFailures to indented JSON via Lua and print to stdout.
const FAILURES_TO_JSON_LUA: &str = r#"
    local function to_json(v, indent)
        indent = indent or 0
        local pad = string.rep("  ", indent)
        local pad1 = string.rep("  ", indent + 1)
        if type(v) == "string" then
            local s = v:gsub('\\', '\\\\'):gsub('"', '\\"'):gsub('\n', '\\n'):gsub('\r', '\\r'):gsub('\t', '\\t')
            return '"' .. s .. '"'
        elseif type(v) == "number" or type(v) == "boolean" then
            return tostring(v)
        elseif type(v) == "table" then
            local parts = {}
            local is_array = #v > 0
            if is_array then
                for _, item in ipairs(v) do parts[#parts+1] = pad1 .. to_json(item, indent + 1) end
                return "[\n" .. table.concat(parts, ",\n") .. "\n" .. pad .. "]"
            else
                local keys = {}
                for k in pairs(v) do keys[#keys+1] = tostring(k) end
                table.sort(keys)
                for _, k in ipairs(keys) do
                    parts[#parts+1] = pad1 .. string.format("%q", k) .. ": " .. to_json(v[k], indent + 1)
                end
                return "{\n" .. table.concat(parts, ",\n") .. "\n" .. pad .. "}"
            end
        else
            return string.format("%q", tostring(v))
        end
    end
    return to_json(WowlessTestFailures)
"#;

fn print_failures(env: &WowLuaEnv) {
    let json: String = env.eval(FAILURES_TO_JSON_LUA).unwrap_or_else(|_| "{}".to_string());
    println!("{json}");
}

/// Run Wowless tests headlessly, printing output to stderr and failures as JSON to stdout.
///
/// Exit codes: 0 = pass, 1 = failures, 2 = timeout.
pub fn run_test(env: &WowLuaEnv, max_ticks: u32, exec_lua: Option<&str>, saved_stdout: Option<i32>) {
    if let Some(code) = exec_lua {
        if let Err(e) = env.exec(code) {
            eprintln!("[exec-lua] error: {e}");
        }
    }

    // Override debugprofilestop to return real elapsed milliseconds so the
    // Wowless test runner's budget check works (yields every half-frame).
    // Registered as a native C function (create_function) so the Wowless
    // globalApis.impltype test sees it as a C function, not a Lua function.
    let lua = env.lua();
    let start = std::time::Instant::now();
    let _ = lua.globals().set(
        "debugprofilestop",
        lua.create_function(move |_, ()| {
            Ok(start.elapsed().as_millis() as i64)
        }).expect("debugprofilestop override"),
    );

    let completed = poll_until_done(env, max_ticks);
    flush_console(env);

    if !completed {
        eprintln!("Wowless tests did not complete within {max_ticks} ticks");
    }

    // Restore stdout before printing JSON results
    restore_stdout(saved_stdout);

    let has_failures: bool = env.eval("next(WowlessTestFailures) ~= nil").unwrap_or(false);
    if has_failures {
        print_failures(env);
        std::process::exit(1);
    }

    if !completed {
        std::process::exit(2);
    }
}
