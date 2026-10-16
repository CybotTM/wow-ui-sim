//! Complete supplied type27 contract; no native/catalog/assets proof.
#![cfg(any(feature = "client-retail", feature = "client-ptr"))]

#[path = "widget_item_display/consumer.rs"]
mod consumer;
#[path = "widget_item_display/feed.rs"]
mod feed;
#[path = "widget_item_display/fixtures.rs"]
mod fixtures;
#[path = "widget_two_records/helpers.rs"]
mod helpers;
#[path = "widget_item_display/queries.rs"]
mod queries;
#[cfg(feature = "retail-12-1-0")]
#[path = "widget_item_display/security.rs"]
mod security;
