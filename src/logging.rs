use std::time::Instant;

/// Format elapsed wall time since simulator start for log prefixes.
pub fn elapsed_prefix(start_time: Instant) -> String {
    format!("[{:7.3}s]", start_time.elapsed().as_secs_f64())
}
