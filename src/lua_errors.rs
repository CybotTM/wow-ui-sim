//! `lua-errors` subcommand: load UI, collect Lua errors, output unique errors as JSON.

use crate::lua_api::{SimState, WowLuaEnv};
use crate::startup::settle_headless_startup;
use std::collections::BTreeMap;

/// A unique Lua error with its occurrence count.
#[derive(serde::Serialize)]
struct LuaError {
    message: String,
    count: usize,
}

/// Run headless startup, collect Lua errors, and print unique errors as JSON to stdout.
///
/// `saved_stdout` is the original stdout fd (redirected to stderr during loading).
/// We restore it before printing JSON so only JSON goes to stdout.
/// `exec_lua` is optional Lua code to execute after startup events.
pub fn run_lua_errors(
    env: &WowLuaEnv,
    saved_stdout: Option<i32>,
    exec_lua: Option<&str>,
    exec_lua_secure: bool,
) {
    // Suppress stderr during startup events (errors are collected in SimState)
    let saved_stderr = suppress_stderr();

    settle_headless_startup(env);

    restore_stderr(saved_stderr);

    if let Some(code) = exec_lua
        && let Err(e) = env.exec_maybe_secure(code, exec_lua_secure)
    {
        eprintln!("[exec-lua] error: {e}");
    }
    // Restore stdout so println goes to real stdout (not stderr redirect)
    restore_stdout(saved_stdout);

    let errors = collect_unique_errors(env);
    let json = serde_json::to_string_pretty(&errors).expect("JSON serialization failed");
    println!("{json}");
}

/// Deduplicate collected Lua errors by their first line. Preserves first-seen order.
fn collect_unique_errors(env: &WowLuaEnv) -> Vec<LuaError> {
    let state = env.state().borrow();
    unique_error_order(&state)
        .into_iter()
        .map(|message| LuaError {
            count: state.lua_error_counts.get(&message).copied().unwrap_or(0),
            message,
        })
        .collect()
}

pub fn grouped_errors_by_addon(state: &SimState) -> BTreeMap<String, Vec<String>> {
    let mut grouped = BTreeMap::new();

    for record in &state.lua_error_records {
        let addon_name = record
            .addon_name
            .clone()
            .unwrap_or_else(|| "<unknown>".to_string());
        let message = extract_error_message(&record.message);
        grouped
            .entry(addon_name)
            .or_insert_with(Vec::new)
            .push(message);
    }

    grouped
}

pub(crate) fn suppressed_error_summary_lines(state: &SimState) -> Vec<String> {
    unique_error_order(state)
        .into_iter()
        .filter_map(|message| {
            let count = state.lua_error_counts.get(&message).copied().unwrap_or(0);
            (count > 1).then(|| {
                format!(
                    "Lua error suppressed {} additional times: {}",
                    count - 1,
                    message
                )
            })
        })
        .collect()
}

pub(crate) fn print_suppressed_error_summary(state: &SimState) {
    for line in suppressed_error_summary_lines(state) {
        eprintln!("{line}");
    }
}

fn unique_error_order(state: &SimState) -> Vec<String> {
    let mut order = Vec::new();
    let mut seen = std::collections::HashSet::new();
    for raw in &state.lua_errors {
        let msg = extract_error_message(raw);
        if state.lua_error_counts.contains_key(&msg) && seen.insert(msg.clone()) {
            order.push(msg);
        }
    }
    order
}

/// Extract the core error message, stripping "runtime error: " prefix.
pub(crate) fn extract_error_message(raw: &str) -> String {
    let first_line = raw.lines().next().unwrap_or(raw);
    let stripped = first_line
        .strip_prefix("runtime error: ")
        .unwrap_or(first_line);
    strip_lua_location_prefix(stripped).to_string()
}

fn strip_lua_location_prefix(msg: &str) -> &str {
    let Some((prefix, body)) = msg.rsplit_once(": ") else {
        return msg;
    };
    let Some((_source, line)) = prefix.rsplit_once(':') else {
        return msg;
    };
    if line.parse::<usize>().is_ok() {
        body
    } else {
        msg
    }
}

/// Redirect stdout (fd 1) to stderr (fd 2). Returns saved original stdout fd.
pub fn redirect_stdout_to_stderr() -> Option<i32> {
    unsafe {
        let saved = libc::dup(1);
        if saved < 0 {
            return None;
        }
        libc::dup2(2, 1); // stdout now points to stderr
        Some(saved)
    }
}

/// Restore stdout from a saved fd.
pub fn restore_stdout(saved: Option<i32>) {
    if let Some(fd) = saved {
        unsafe {
            libc::dup2(fd, 1);
            libc::close(fd);
        }
    }
}

/// Redirect stderr to /dev/null. Returns saved original stderr fd.
fn suppress_stderr() -> Option<i32> {
    unsafe {
        let saved = libc::dup(2);
        if saved < 0 {
            return None;
        }
        let devnull = libc::open(b"/dev/null\0".as_ptr().cast(), libc::O_WRONLY);
        if devnull < 0 {
            libc::close(saved);
            return None;
        }
        libc::dup2(devnull, 2);
        libc::close(devnull);
        Some(saved)
    }
}

/// Restore stderr from a saved fd.
fn restore_stderr(saved: Option<i32>) {
    if let Some(fd) = saved {
        unsafe {
            libc::dup2(fd, 2);
            libc::close(fd);
        }
    }
}
