//! Supplied type9 behavior; no native/catalog/asset parity.
#![cfg(any(feature = "client-retail", feature = "client-ptr"))]

#[path = "widget_horizontal_currencies/consumer.rs"]
mod consumer;
#[path = "widget_horizontal_currencies/feed.rs"]
mod feed;
#[path = "widget_horizontal_currencies/fixtures.rs"]
mod fixtures;
#[path = "widget_two_records/helpers.rs"]
mod helpers;
#[path = "widget_horizontal_currencies/ordering.rs"]
mod ordering;
#[path = "widget_horizontal_currencies/queries.rs"]
mod queries;
#[cfg(feature = "retail-12-1-0")]
#[path = "widget_horizontal_currencies/security.rs"]
mod security;
