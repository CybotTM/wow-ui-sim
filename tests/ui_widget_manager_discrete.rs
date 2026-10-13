//! Staged RED: input scaffolding only, no visualization producer implementation.
#![cfg(any(feature = "client-retail", feature = "client-ptr"))]

#[path = "widget_discrete/consumer.rs"]
mod consumer;
#[path = "widget_discrete/fixtures.rs"]
mod fixtures;
#[path = "widget_discrete/queries.rs"]
mod queries;
#[cfg(feature = "retail-12-1-0")]
#[path = "widget_discrete/security.rs"]
mod security;
