use std::sync::{Mutex, OnceLock};
use std::time::{Duration, Instant};

static PROCESS_START_TIME: OnceLock<Instant> = OnceLock::new();
static TEXTURE_LOAD_DEBUG_ENABLED: OnceLock<bool> = OnceLock::new();
static GUI_TRACE_ENABLED: OnceLock<bool> = OnceLock::new();
static BLOCKING_PHASE: OnceLock<Mutex<(&'static str, Instant)>> = OnceLock::new();

/// Initialize the shared process start time for elapsed log prefixes.
pub fn init_process_start_time(start_time: Instant) {
    let _ = PROCESS_START_TIME.set(start_time);
}

/// Return the shared process start time, initializing lazily if needed.
pub fn process_start_time() -> Instant {
    *PROCESS_START_TIME.get_or_init(Instant::now)
}

fn blocking_phase_cell() -> &'static Mutex<(&'static str, Instant)> {
    BLOCKING_PHASE.get_or_init(|| Mutex::new(("boot", Instant::now())))
}

/// Record the current main-thread phase that may delay input handling.
pub fn set_blocking_phase(phase: &'static str) {
    let mut current = blocking_phase_cell()
        .lock()
        .unwrap_or_else(std::sync::PoisonError::into_inner);
    *current = (phase, Instant::now());
}

/// Snapshot the most recent blocking phase and how long ago it started.
pub fn blocking_phase_snapshot() -> (&'static str, Duration) {
    let current = blocking_phase_cell()
        .lock()
        .unwrap_or_else(std::sync::PoisonError::into_inner);
    let (phase, started_at) = *current;
    (phase, started_at.elapsed())
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

/// Whether retained-GUI frame-sequence tracing is enabled.
pub fn gui_trace_enabled() -> bool {
    *GUI_TRACE_ENABLED.get_or_init(|| std::env::var_os("WOW_SIM_GUI_TRACE").is_some())
}

/// Print a log line to stdout with the shared elapsed-time prefix.
pub fn println_elapsed(message: &str) {
    println!("{} {}", global_elapsed_prefix(), message);
}

/// Print a log line to stderr with the shared elapsed-time prefix.
pub fn eprintln_elapsed(message: &str) {
    eprintln!("{} {}", global_elapsed_prefix(), message);
}

/// Print a retained-GUI trace log line to stderr when tracing is enabled.
pub fn eprintln_gui_trace(message: &str) {
    if gui_trace_enabled() {
        eprintln_elapsed(&format!("[gui-trace] {message}"));
    }
}
