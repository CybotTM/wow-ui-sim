//! `lua-errors` subcommand: load UI, collect Lua errors, output unique errors as JSON.

use crate::lua_api::WowLuaEnv;
use crate::startup::{fire_startup_events, process_pending_timers, fire_one_on_update_tick};

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
pub fn run_lua_errors(env: &WowLuaEnv, saved_stdout: Option<i32>, exec_lua: Option<&str>) {
    // Suppress stderr during startup events (errors are collected in SimState)
    let saved_stderr = suppress_stderr();

    fire_startup_events(env);
    env.apply_post_event_workarounds();
    env.state().borrow_mut().widgets.rebuild_anchor_index();
    process_pending_timers(env);
    fire_one_on_update_tick(env);
    let _ = crate::lua_api::globals::global_frames::hide_runtime_hidden_frames(env.lua());

    restore_stderr(saved_stderr);

    if let Some(code) = exec_lua {
        if let Err(e) = env.exec(code) {
            eprintln!("[exec-lua] error: {e}");
        }
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
    let mut order: Vec<String> = Vec::new();
    let mut counts = std::collections::HashMap::<String, usize>::new();
    for raw in &state.lua_errors {
        let msg = extract_error_message(raw);
        let entry = counts.entry(msg.clone()).or_insert(0);
        if *entry == 0 { order.push(msg); }
        *entry += 1;
    }
    order.into_iter()
        .map(|message| LuaError { count: counts[&message], message })
        .collect()
}

/// Extract the core error message, stripping "runtime error: " prefix.
fn extract_error_message(raw: &str) -> String {
    let first_line = raw.lines().next().unwrap_or(raw);
    first_line
        .strip_prefix("runtime error: ")
        .unwrap_or(first_line)
        .to_string()
}

/// Redirect stdout (fd 1) to stderr (fd 2). Returns saved original stdout fd.
pub fn redirect_stdout_to_stderr() -> Option<i32> {
    unsafe {
        let saved = libc::dup(1);
        if saved < 0 { return None; }
        libc::dup2(2, 1); // stdout now points to stderr
        Some(saved)
    }
}

/// Restore stdout from a saved fd.
fn restore_stdout(saved: Option<i32>) {
    if let Some(fd) = saved {
        unsafe { libc::dup2(fd, 1); libc::close(fd); }
    }
}

/// Redirect stderr to /dev/null. Returns saved original stderr fd.
fn suppress_stderr() -> Option<i32> {
    unsafe {
        let saved = libc::dup(2);
        if saved < 0 { return None; }
        let devnull = libc::open(b"/dev/null\0".as_ptr().cast(), libc::O_WRONLY);
        if devnull < 0 { libc::close(saved); return None; }
        libc::dup2(devnull, 2);
        libc::close(devnull);
        Some(saved)
    }
}

/// Restore stderr from a saved fd.
fn restore_stderr(saved: Option<i32>) {
    if let Some(fd) = saved {
        unsafe { libc::dup2(fd, 2); libc::close(fd); }
    }
}
