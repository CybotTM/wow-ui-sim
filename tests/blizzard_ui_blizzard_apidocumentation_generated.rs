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

#[path = "blizzard_ui/blizzard_apidocumentation_generated/behavior_handle_slash_command_stats_writes_real_counts.rs"]
mod behavior_handle_slash_command_stats_writes_real_counts;

#[path = "blizzard_ui/blizzard_apidocumentation_generated/behavior_handle_slash_command_system_list_lists_corpus_systems.rs"]
mod behavior_handle_slash_command_system_list_lists_corpus_systems;

#[path = "blizzard_ui/blizzard_apidocumentation_generated/behavior_handle_slash_command_system_search_dispatches_to_corpus_system.rs"]
mod behavior_handle_slash_command_system_search_dispatches_to_corpus_system;
