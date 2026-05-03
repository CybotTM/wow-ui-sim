//! Wrapper binary for Blizzard_APIDocumentationGenerated tests.
//!
//! Cargo only auto-discovers `tests/*.rs`, so nested files under
//! `tests/blizzard_ui/blizzard_apidocumentation_generated/` are re-exported
//! here.

mod common;

#[path = "blizzard_ui/blizzard_apidocumentation_generated/load.rs"]
mod load;

#[path = "blizzard_ui/blizzard_apidocumentation_generated/surface_globals.rs"]
mod surface_globals;

#[path = "blizzard_ui/blizzard_apidocumentation_generated/behavior_find_all_api_matches_returns_corpus_results.rs"]
mod behavior_find_all_api_matches_returns_corpus_results;
