//! Logging helpers for the workarounds module — timestamped step logs that
//! tag start and end of each `apply_step` so a reader can see ordering and
//! per-step duration in the simulator startup output.
//!
//! Extracted from `workarounds/mod.rs` as the first slice of an ongoing
//! per-domain split; see the readability follow-up in PLAN.classic.md.

use std::time::Instant;

pub(super) fn log_with_timestamp(env: &crate::lua_api::WowLuaEnv, message: &str) {
    let start_time = env.state().borrow().start_time;
    eprintln!("{} {}", crate::logging::elapsed_prefix(start_time), message);
}

pub(super) fn log_step(env: &crate::lua_api::WowLuaEnv, label: &str, apply_step: impl FnOnce()) {
    log_with_timestamp(env, &format!("[Workarounds] starting {label}"));
    let started = Instant::now();
    apply_step();
    log_with_timestamp(
        env,
        &format!(
            "[Workarounds] finished {label} in {:.2?}",
            started.elapsed()
        ),
    );
}
