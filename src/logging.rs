use std::sync::OnceLock;
use std::time::Instant;

static PROCESS_START_TIME: OnceLock<Instant> = OnceLock::new();
static TEXTURE_LOAD_DEBUG_ENABLED: OnceLock<bool> = OnceLock::new();

/// Initialize the shared process start time for elapsed log prefixes.
pub fn init_process_start_time(start_time: Instant) {
    let _ = PROCESS_START_TIME.set(start_time);
}

/// Return the shared process start time, initializing lazily if needed.
pub fn process_start_time() -> Instant {
    *PROCESS_START_TIME.get_or_init(Instant::now)
}

/// Format elapsed wall time since simulator start for log prefixes.
pub fn elapsed_prefix(start_time: Instant) -> String {
    format!("[{:7.3}s]", start_time.elapsed().as_secs_f64())
}

/// Format elapsed wall time since the shared process start.
pub fn global_elapsed_prefix() -> String {
    elapsed_prefix(process_start_time())
}

/// Whether verbose texture-load profiling logs are enabled.
pub fn texture_load_debug_enabled() -> bool {
    *TEXTURE_LOAD_DEBUG_ENABLED
        .get_or_init(|| std::env::var_os("WOW_SIM_DEBUG_TEXTURE_LOADS").is_some())
}

/// Print a log line to stdout with the shared elapsed-time prefix.
pub fn println_elapsed(message: &str) {
    println!("{} {}", global_elapsed_prefix(), message);
}

/// Print a log line to stderr with the shared elapsed-time prefix.
pub fn eprintln_elapsed(message: &str) {
    eprintln!("{} {}", global_elapsed_prefix(), message);
}
