//! Input-only TextWithState stage; getter and Lua registration intentionally absent.
#![cfg(any(feature = "client-retail", feature = "client-ptr"))]

// Reuse existing Discrete assertion/error capture, not a second broad harness.
#[path = "widget_text_state/consumer.rs"]
mod consumer;
#[path = "widget_discrete/fixtures.rs"]
mod discrete_fixtures;
#[path = "widget_text_state/feed.rs"]
mod feed;
#[path = "widget_text_state/fixtures.rs"]
mod fixtures;
#[path = "widget_text_state/queries.rs"]
mod queries;
#[cfg(feature = "retail-12-1-0")]
#[path = "widget_text_state/security.rs"]
mod security;
