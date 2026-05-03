//! Wrapper binary for Blizzard_ArtifactUI tests.
//!
//! Cargo only auto-discovers `tests/*.rs`, so nested files under
//! `tests/blizzard_ui/blizzard_artifact_ui/` are re-exported here.

mod common;

#[path = "blizzard_ui/blizzard_artifact_ui/load.rs"]
mod load;
