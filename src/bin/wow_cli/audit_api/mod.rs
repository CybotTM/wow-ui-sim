//! Static analysis tool for Blizzard UI API usage.
//!
//! Scans Blizzard UI Lua/XML files and cross-references against the simulator's
//! registered API. Useful for identifying gaps in coverage.

mod gap;
mod output;
mod patch_manifest;
mod patch_source_index;
mod scanner;

use serde::Serialize;
use std::collections::BTreeMap;
use std::path::PathBuf;

pub use gap::{GapReport, build_gap_report, introspect_simulator_c_methods, scan_simulator};
pub use output::{print_gap_plan, print_gap_text, print_json, print_text};
pub use patch_manifest::{
    generate_initialization_observations, parse_manifest, parse_observations, render_checklist,
    render_summary, validate_complete, validate_repository,
};
pub use patch_source_index::{index_active_lua_tree, index_lua_file, index_lua_tree};
pub use scanner::run_audit;

/// Per-symbol occurrence data.
#[derive(Debug, Default, Serialize)]
pub struct SymbolUsage {
    pub count: usize,
    pub files: Vec<String>,
}

/// Collected results from scanning Blizzard UI sources.
#[derive(Debug, Default, Serialize)]
pub struct AuditResults {
    /// C_Namespace -> method -> usage
    pub c_api: BTreeMap<String, BTreeMap<String, SymbolUsage>>,
    /// Bare global function calls like `UnitName(...)`
    pub global_functions: BTreeMap<String, SymbolUsage>,
    /// XML template inheritance references like `inherits="ButtonFrameTemplate"`
    pub inherited_templates: BTreeMap<String, SymbolUsage>,
    /// LE_* constants
    pub constants: BTreeMap<String, SymbolUsage>,
    /// Enum.Namespace.Value references
    pub enums: BTreeMap<String, SymbolUsage>,
    /// Other constant families (ITEM_MOD_, MAX_, etc.)
    pub other_constants: BTreeMap<String, SymbolUsage>,
}

/// Configuration for the audit command.
pub struct AuditConfig {
    pub ui_path: PathBuf,
    pub namespace_filter: Option<String>,
    pub filter_startup: bool,
    /// Path to wowless repo (for C_* namespace allowlist from apis.yaml).
    pub wowless_path: Option<PathBuf>,
}

#[derive(Clone, Copy, PartialEq)]
pub enum OutputFormat {
    Text,
    Json,
    /// PLAN.md-ready markdown checkboxes (gaps only, no usage dump).
    Plan,
}
