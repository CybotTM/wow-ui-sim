//! Supplied type4 query, isolation, security and cached-consumer contracts.
#![cfg(any(feature = "client-retail", feature = "client-ptr"))]

#[path = "widget_icon_background/consumer.rs"]
mod consumer;
#[path = "widget_icon_background/distinct.rs"]
mod distinct;
#[path = "widget_icon_background/feed.rs"]
mod feed;
#[path = "widget_icon_background/fixtures.rs"]
mod fixtures;
#[path = "widget_two_records/helpers.rs"]
mod helpers;
#[path = "widget_icon_background/ordering.rs"]
mod ordering;
#[path = "widget_icon_background/queries.rs"]
mod queries;
#[cfg(feature = "retail-12-1-0")]
#[path = "widget_icon_background/security.rs"]
mod security;
