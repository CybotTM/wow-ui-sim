//! Wrapper binary for Blizzard_APIDocumentation tests.
//!
//! Cargo only auto-discovers `tests/*.rs`, so nested files under
//! `tests/blizzard_ui/blizzard_apidocumentation/` are re-exported here.

mod common;

#[path = "blizzard_ui/blizzard_apidocumentation/load.rs"]
mod load;

#[path = "blizzard_ui/blizzard_apidocumentation/surface_globals.rs"]
mod surface_globals;

#[path = "blizzard_ui/blizzard_apidocumentation/surface_mixins.rs"]
mod surface_mixins;
