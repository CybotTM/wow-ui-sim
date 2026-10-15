//! Supplied type5 query, isolation, security and cached-consumer contracts.
#![cfg(feature = "client-retail")]

#[path = "widget_double_icon/consumer.rs"]
mod consumer;
#[path = "widget_double_icon/distinct.rs"]
mod distinct;
#[path = "widget_double_icon/feed.rs"]
mod feed;
#[path = "widget_double_icon/fixtures.rs"]
mod fixtures;
#[path = "widget_two_records/helpers.rs"]
mod helpers;
#[path = "widget_double_icon/queries.rs"]
mod queries;
#[cfg(feature = "retail-12-1-0")]
#[path = "widget_double_icon/security.rs"]
mod security;
