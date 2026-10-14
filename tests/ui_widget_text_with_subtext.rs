//! Supplied type25 contract; unchanged profile-scoped Blizzard consumers.
#![cfg(any(feature = "client-retail", feature = "client-ptr"))]

#[path = "widget_subtext/consumer.rs"]
mod consumer;
#[path = "widget_subtext/feed.rs"]
mod feed;
#[path = "widget_subtext/fixtures.rs"]
mod fixtures;
#[path = "widget_two_records/helpers.rs"]
mod helpers;
#[path = "widget_subtext/queries.rs"]
mod queries;
#[cfg(feature = "retail-12-1-0")]
#[path = "widget_subtext/security.rs"]
mod security;
