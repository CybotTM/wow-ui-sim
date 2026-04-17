//! Timer-tick helpers: predicate and rescheduling logic for `WowLuaEnv::process_timers`.

use super::state::PendingTimer;
use std::time::Instant;

/// Returns true when the timer should not fire yet (either cancelled or scheduled for later).
pub(super) fn timer_should_wait(timer: &PendingTimer, now: Instant) -> bool {
    timer.cancelled || timer.fire_at > now
}

/// Reschedule a repeating timer. Returns true if the timer was re-queued.
pub(super) fn reschedule_timer(timer: &mut PendingTimer, now: Instant) -> bool {
    let Some(interval) = timer.interval else {
        return false;
    };

    let should_repeat = match timer.remaining {
        Some(remaining) if remaining <= 1 => false,
        Some(remaining) => {
            timer.remaining = Some(remaining - 1);
            true
        }
        None => true,
    };
    if should_repeat {
        timer.fire_at = now + interval;
    }
    should_repeat
}
