//! Explicit absolute-millisecond input; zero default is simulator policy.

#[derive(Default)]
pub struct PingCooldownInterval {
    pub start_time_ms: f64,
    pub end_time_ms: f64,
}
