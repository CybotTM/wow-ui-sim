//! Host-supplied group eligibility observations, independent of catalog membership.

/// Missing keys have no supplied observation; `false` is an explicit observation.
pub type GroupEligibilityObservations = std::collections::HashMap<i32, bool>;
