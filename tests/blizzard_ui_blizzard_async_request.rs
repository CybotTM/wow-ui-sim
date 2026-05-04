//! Wrapper binary for Blizzard_AsyncRequest tests.
//!
//! Cargo only auto-discovers `tests/*.rs`, so nested files under
//! `tests/blizzard_ui/blizzard_async_request/` are re-exported here.

mod common;

#[path = "blizzard_ui/blizzard_async_request/load.rs"]
mod load;

#[path = "blizzard_ui/blizzard_async_request/surface_globals.rs"]
mod surface_globals;

#[path = "blizzard_ui/blizzard_async_request/surface_frames.rs"]
mod surface_frames;

#[path = "blizzard_ui/blizzard_async_request/behavior_create_requires_callbacks.rs"]
mod behavior_create_requires_callbacks;

#[path = "blizzard_ui/blizzard_async_request/behavior_create_timeout_pair_required.rs"]
mod behavior_create_timeout_pair_required;
