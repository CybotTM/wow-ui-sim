//! Explicit RAF summon cooldown input, in seconds, shared by legacy and C_* APIs.
//! Zero timing and a disabled timer are simulator defaults, not native parity.

#[derive(Debug, Clone, Copy, Default)]
pub struct SummonFriendCooldown {
    pub start_time_seconds: f64,
    pub duration_seconds: f64,
    pub enable_cooldown_timer: bool,
}
