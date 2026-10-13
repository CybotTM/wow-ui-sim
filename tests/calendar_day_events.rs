//! Private candidate: host-only Calendar day rows; model policy, not native bounds/arity.
#![cfg(any(feature = "client-retail", feature = "client-ptr"))]

#[path = "calendar_day_events/consumer.rs"]
mod consumer;
#[path = "calendar_day_events/fixtures.rs"]
mod fixtures;
#[path = "calendar_day_events/queries.rs"]
mod queries;
#[cfg(feature = "retail-12-1-0")]
#[path = "calendar_day_events/security.rs"]
mod security;
