//! Supplied type22 contract; unchanged profile-scoped Blizzard consumers.
#![cfg(any(feature = "client-retail", feature = "client-ptr"))]

#[path = "widget_spacer/consumer.rs"]
mod consumer;
#[path = "widget_spacer/feed.rs"]
mod feed;
#[path = "widget_spacer/fixtures.rs"]
mod fixtures;
#[path = "widget_two_records/helpers.rs"]
mod helpers;
#[path = "widget_spacer/queries.rs"]
mod queries;
#[cfg(feature = "retail-12-1-0")]
#[path = "widget_spacer/security.rs"]
mod security;
