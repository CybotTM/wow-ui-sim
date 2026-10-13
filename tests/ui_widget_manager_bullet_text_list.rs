//! Unexecuted positive-data RED stage. No modeled BulletTextList getter supplied.
#![cfg(any(feature = "client-retail", feature = "client-ptr"))]

#[path = "widget_bullet/consumer.rs"]
mod consumer;
#[path = "widget_bullet/feed.rs"]
mod feed;
#[path = "widget_bullet/fixtures.rs"]
mod fixtures;
#[path = "widget_bullet/queries.rs"]
mod queries;
#[cfg(feature = "retail-12-1-0")]
#[path = "widget_bullet/security.rs"]
mod security;
