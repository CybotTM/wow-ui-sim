//! Wrapper binary for Blizzard_APIDocumentation tests.
//!
//! Cargo only auto-discovers `tests/*.rs`, so nested files under
//! `tests/blizzard_ui/blizzard_apidocumentation/` are re-exported here.

mod common;

#[path = "blizzard_ui/blizzard_apidocumentation/behavior_add_documentation_table_routes_payload_kinds.rs"]
mod behavior_add_documentation_table_routes_payload_kinds;

#[path = "blizzard_ui/blizzard_apidocumentation/behavior_find_all_api_matches_returns_nil_when_empty.rs"]
mod behavior_find_all_api_matches_returns_nil_when_empty;

#[path = "blizzard_ui/blizzard_apidocumentation/behavior_handle_slash_command_help_writes_usage.rs"]
mod behavior_handle_slash_command_help_writes_usage;

#[path = "blizzard_ui/blizzard_apidocumentation/load.rs"]
mod load;

#[path = "blizzard_ui/blizzard_apidocumentation/surface_globals.rs"]
mod surface_globals;

#[path = "blizzard_ui/blizzard_apidocumentation/surface_mixins.rs"]
mod surface_mixins;
