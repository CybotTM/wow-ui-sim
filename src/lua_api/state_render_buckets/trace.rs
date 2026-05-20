use std::sync::OnceLock;
use std::time::{Duration, Instant};

#[derive(Clone, Copy)]
enum StrataInvalidationTrace {
    Disabled,
    Enabled { after: Option<Duration> },
}

pub(crate) fn should_trace_strata_invalidations(start_time: &Instant) -> bool {
    match strata_invalidation_trace() {
        StrataInvalidationTrace::Disabled => false,
        StrataInvalidationTrace::Enabled { after: None } => true,
        StrataInvalidationTrace::Enabled { after: Some(after) } => start_time.elapsed() >= after,
    }
}

fn strata_invalidation_trace() -> StrataInvalidationTrace {
    static TRACE: OnceLock<StrataInvalidationTrace> = OnceLock::new();
    *TRACE.get_or_init(read_strata_invalidation_trace)
}

fn read_strata_invalidation_trace() -> StrataInvalidationTrace {
    if std::env::var_os("WOW_SIM_TRACE_STRATA_INVALIDATIONS").is_none() {
        return StrataInvalidationTrace::Disabled;
    }

    let after = std::env::var("WOW_SIM_TRACE_STRATA_INVALIDATIONS_AFTER_MS")
        .ok()
        .and_then(|value| value.parse::<u64>().ok())
        .map(Duration::from_millis);
    StrataInvalidationTrace::Enabled { after }
}
