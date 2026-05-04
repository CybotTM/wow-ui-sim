//! Wrapper binary for Blizzard_AsyncRequest tests.
//!
//! Cargo only auto-discovers `tests/*.rs`, so nested files under
//! `tests/blizzard_ui/blizzard_async_request/` are re-exported here.

mod common;

#[path = "blizzard_ui/blizzard_async_request/load.rs"]
mod load;
