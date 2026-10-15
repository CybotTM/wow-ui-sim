//! Supplied TextColumnRow contract tests; passive inputs precede the getter.
#![cfg(any(feature = "client-retail", feature = "client-ptr"))]

#[path = "widget_text_column/consumer.rs"]
mod consumer;
#[path = "widget_text_column/feed.rs"]
mod feed;
#[path = "widget_text_column/fixtures.rs"]
mod fixtures;
#[path = "widget_text_column/queries.rs"]
mod queries;
#[path = "widget_text_column/selector_security.rs"]
mod selector_security;
#[path = "widget_two_records/helpers.rs"]
mod selector_shared;
