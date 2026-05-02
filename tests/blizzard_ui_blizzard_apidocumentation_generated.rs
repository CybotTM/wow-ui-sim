//! Wrapper binary for Blizzard_APIDocumentationGenerated tests.
//!
//! Cargo only auto-discovers `tests/*.rs`, so nested files under
//! `tests/blizzard_ui/blizzard_apidocumentation_generated/` are re-exported
//! here.

mod common;

#[path = "blizzard_ui/blizzard_apidocumentation_generated/load.rs"]
mod load;
