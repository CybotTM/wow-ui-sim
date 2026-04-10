//! Static analysis tool for Blizzard UI API usage.
//!
//! Scans Blizzard UI Lua/XML files and cross-references against the simulator's
//! registered API. Useful for identifying gaps in coverage.

use regex::Regex;
use serde::Serialize;
use std::collections::{BTreeMap, BTreeSet, HashSet};
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

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
}

/// Script element tag names whose text content is inline Lua.
const SCRIPT_TAGS: &[&str] = &[
    "OnLoad", "OnClick", "OnShow", "OnHide", "OnEvent", "OnUpdate", "OnEnter", "OnLeave",
];

/// All compiled regex patterns (built once, reused across all files).
struct Patterns {
    c_api: Regex,
    global_call: Regex,
    le_const: Regex,
    enum_ref: Regex,
    other_const: Regex,
    line_comment: Regex,
    block_comment: Regex,
    kv_global: Regex,
    xml_inherits: Regex,
    local_fn_def: Regex,
    global_fn_def: Regex,
    local_assign: Regex,
    /// One regex per script tag (no backreference support in the `regex` crate).
    xml_script_tags: Vec<Regex>,
}

impl Patterns {
    fn new() -> Self {
        let xml_script_tags = SCRIPT_TAGS
            .iter()
            .map(|tag| Regex::new(&format!(r"(?s)<{tag}[^>]*>(.*?)</{tag}>")).unwrap())
            .collect();
        Self {
            c_api: Regex::new(r"(C_\w+)[.:](\w+)").unwrap(),
            global_call: Regex::new(r"\b([A-Za-z_][A-Za-z0-9_]*)\s*\(").unwrap(),
            le_const: Regex::new(r"\bLE_\w+").unwrap(),
            enum_ref: Regex::new(r"\bEnum\.(\w+\.\w+)").unwrap(),
            other_const: Regex::new(r"\b(ITEM_MOD|MAX|NUM|SPELL_SCHOOL|RAID_CLASS|CLASS_SORT)_\w+")
                .unwrap(),
            line_comment: Regex::new(r"--[^\n]*").unwrap(),
            block_comment: Regex::new(r"(?s)--\[\[.*?\]\]").unwrap(),
            kv_global: Regex::new(r#"type="global"[^>]*value="([^"]+)""#).unwrap(),
            xml_inherits: Regex::new(r#"inherits="([^"]+)""#).unwrap(),
            local_fn_def: Regex::new(r"\blocal\s+function\s+([A-Za-z_][A-Za-z0-9_]*)\s*\(")
                .unwrap(),
            global_fn_def: Regex::new(r"\bfunction\s+([A-Za-z_][A-Za-z0-9_]*)\s*\(").unwrap(),
            local_assign: Regex::new(
                r"\blocal\s+([A-Za-z_][A-Za-z0-9_]*(?:\s*,\s*[A-Za-z_][A-Za-z0-9_]*)*)\s*=",
            )
            .unwrap(),
            xml_script_tags,
        }
    }
}

/// Strip Lua comments from source text using pre-compiled patterns.
fn strip_comments<'a>(src: &'a str, p: &Patterns) -> std::borrow::Cow<'a, str> {
    let without_blocks = p.block_comment.replace_all(src, "");
    // replace_all returns Cow; if no block comments, it borrows — turn into owned for
    // the second pass to avoid lifetime issues.
    let without_blocks = without_blocks.into_owned();
    p.line_comment
        .replace_all(&without_blocks, "")
        .into_owned()
        .into()
}

fn record_symbol_usage(usage: &mut SymbolUsage, file_label: &str) {
    usage.count += 1;
    if !usage.files.iter().any(|f| f == file_label) {
        usage.files.push(file_label.to_string());
    }
}

fn scan_c_api_usage(clean: &str, file_label: &str, results: &mut AuditResults, p: &Patterns) {
    for cap in p.c_api.captures_iter(clean) {
        let ns = cap[1].to_string();
        let method = cap[2].to_string();
        let usage = results
            .c_api
            .entry(ns)
            .or_default()
            .entry(method)
            .or_default();
        record_symbol_usage(usage, file_label);
    }
}

fn scan_global_function_usage(
    clean: &str,
    file_label: &str,
    results: &mut AuditResults,
    p: &Patterns,
) {
    let definition_starts = collect_definition_starts(clean, p);
    for cap in p.global_call.captures_iter(clean) {
        let Some(matched_name) = cap.get(1) else {
            continue;
        };
        let name = matched_name.as_str();
        if !should_record_bare_global_call(clean, matched_name.start(), name, &definition_starts) {
            continue;
        }
        let usage = results
            .global_functions
            .entry(name.to_string())
            .or_default();
        record_symbol_usage(usage, file_label);
    }
}

fn collect_definition_starts(clean: &str, p: &Patterns) -> BTreeMap<String, usize> {
    let mut starts = BTreeMap::new();

    for cap in p.local_fn_def.captures_iter(clean) {
        record_definition_start(&mut starts, &cap[1], cap.get(1).map(|m| m.start()));
    }
    for cap in p.global_fn_def.captures_iter(clean) {
        record_definition_start(&mut starts, &cap[1], cap.get(1).map(|m| m.start()));
    }
    for cap in p.local_assign.captures_iter(clean) {
        for name in cap[1].split(',') {
            record_definition_start(&mut starts, name.trim(), cap.get(1).map(|m| m.start()));
        }
    }

    starts
}

fn record_definition_start(starts: &mut BTreeMap<String, usize>, name: &str, start: Option<usize>) {
    let Some(start) = start else {
        return;
    };
    starts
        .entry(name.to_string())
        .and_modify(|earliest| *earliest = (*earliest).min(start))
        .or_insert(start);
}

fn should_record_bare_global_call(
    clean: &str,
    name_start: usize,
    name: &str,
    definition_starts: &BTreeMap<String, usize>,
) -> bool {
    if definition_starts
        .get(name)
        .is_some_and(|definition_start| *definition_start <= name_start)
        || is_lua_keyword(name)
    {
        return false;
    }
    match previous_non_whitespace_char(clean, name_start) {
        Some('.') | Some(':') => false,
        _ => true,
    }
}

fn previous_non_whitespace_char(clean: &str, start: usize) -> Option<char> {
    clean[..start].chars().rev().find(|c| !c.is_whitespace())
}

fn is_lua_keyword(name: &str) -> bool {
    matches!(
        name,
        "and"
            | "break"
            | "do"
            | "else"
            | "elseif"
            | "end"
            | "false"
            | "for"
            | "function"
            | "if"
            | "in"
            | "local"
            | "nil"
            | "not"
            | "or"
            | "repeat"
            | "return"
            | "then"
            | "true"
            | "until"
            | "while"
    )
}

fn scan_constant_usage(clean: &str, file_label: &str, results: &mut AuditResults, p: &Patterns) {
    for cap in p.le_const.captures_iter(clean) {
        let usage = results.constants.entry(cap[0].to_string()).or_default();
        record_symbol_usage(usage, file_label);
    }
}

fn scan_enum_usage(clean: &str, file_label: &str, results: &mut AuditResults, p: &Patterns) {
    for cap in p.enum_ref.captures_iter(clean) {
        let sym = format!("Enum.{}", &cap[1]);
        let usage = results.enums.entry(sym).or_default();
        record_symbol_usage(usage, file_label);
    }
}

fn scan_other_constant_usage(
    clean: &str,
    file_label: &str,
    results: &mut AuditResults,
    p: &Patterns,
) {
    for cap in p.other_const.captures_iter(clean) {
        let usage = results
            .other_constants
            .entry(cap[0].to_string())
            .or_default();
        record_symbol_usage(usage, file_label);
    }
}

/// Scan a chunk of Lua source text and accumulate results.
fn scan_lua_text(text: &str, file_label: &str, results: &mut AuditResults, p: &Patterns) {
    let clean = strip_comments(text, p);
    scan_c_api_usage(&clean, file_label, results, p);
    scan_global_function_usage(&clean, file_label, results, p);
    scan_constant_usage(&clean, file_label, results, p);
    scan_enum_usage(&clean, file_label, results, p);
    scan_other_constant_usage(&clean, file_label, results, p);
}

/// Extract inline Lua from XML script elements and scan them.
fn scan_xml_file(path: &Path, file_label: &str, results: &mut AuditResults, p: &Patterns) {
    let content = match std::fs::read_to_string(path) {
        Ok(c) => c,
        Err(_) => return,
    };
    scan_xml_text(&content, file_label, results, p);
}

fn scan_xml_text(content: &str, file_label: &str, results: &mut AuditResults, p: &Patterns) {
    scan_xml_inherits(content, file_label, results, p);

    // Extract KeyValue globals: type="global" value="SOME_CONSTANT"
    for cap in p.kv_global.captures_iter(content) {
        let usage = results
            .other_constants
            .entry(cap[1].to_string())
            .or_default();
        record_symbol_usage(usage, file_label);
    }

    // Extract inline script bodies
    for re in &p.xml_script_tags {
        for cap in re.captures_iter(content) {
            scan_lua_text(&cap[1], file_label, results, p);
        }
    }
}

fn scan_xml_inherits(content: &str, file_label: &str, results: &mut AuditResults, p: &Patterns) {
    for cap in p.xml_inherits.captures_iter(content) {
        for template in cap[1].split(',') {
            let template = template.trim();
            if template.is_empty() {
                continue;
            }
            let usage = results
                .inherited_templates
                .entry(template.to_string())
                .or_default();
            record_symbol_usage(usage, file_label);
        }
    }
}

/// Whether to skip a directory (test suites we don't want to scan).
fn should_skip(path: &Path) -> bool {
    let s = path.to_string_lossy();
    s.contains("Interface/AddOns/Wowless") || s.contains("Interface/AddOns/WowlessData")
}

/// Whether to skip a file because it's LoadOnDemand (when filter_startup is on).
fn is_load_on_demand(addon_path: &Path, addon_dir: &Path) -> bool {
    let toc = addon_path
        .ancestors()
        .find(|p| p.parent() == Some(addon_dir))
        .and_then(|addon| {
            let name = addon.file_name()?;
            Some(addon.join(format!("{}.toc", name.to_string_lossy())))
        });

    if let Some(toc_path) = toc {
        if let Ok(toc_content) = std::fs::read_to_string(&toc_path) {
            return toc_content
                .lines()
                .any(|line| line.trim().eq_ignore_ascii_case("## LoadOnDemand: 1"));
        }
    }
    false
}

/// Load valid C_* namespaces and their methods from wowless apis.yaml.
///
/// The apis.yaml file contains flat entries like `C_Timer.After:` at the top level.
/// Returns `(namespace_set, namespace_to_methods)`, or `None` if the file doesn't exist.
pub fn load_valid_c_namespaces(
    wowless_path: &Path,
) -> Option<(HashSet<String>, BTreeMap<String, BTreeSet<String>>)> {
    let apis_path = wowless_path.join("data/products/wow/apis.yaml");
    let content = match std::fs::read_to_string(&apis_path) {
        Ok(c) => c,
        Err(_) => {
            eprintln!(
                "Warning: wowless apis.yaml not found at {}; skipping C_* namespace filtering",
                apis_path.display()
            );
            return None;
        }
    };

    let mut namespaces: HashSet<String> = HashSet::new();
    let mut methods: BTreeMap<String, BTreeSet<String>> = BTreeMap::new();

    for line in content.lines() {
        // Top-level entries look like `C_Foo.Bar:` (no leading whitespace, ends with `:`)
        if !line.starts_with("C_") || !line.ends_with(':') {
            continue;
        }
        let entry = &line[..line.len() - 1]; // strip trailing `:`
        if let Some(dot) = entry.find('.') {
            let ns = entry[..dot].to_string();
            let method = entry[dot + 1..].to_string();
            namespaces.insert(ns.clone());
            methods.entry(ns).or_default().insert(method);
        }
    }

    Some((namespaces, methods))
}

/// Run the audit and return results.
pub fn run_audit(config: &AuditConfig) -> AuditResults {
    let p = Patterns::new();
    let mut results = AuditResults::default();
    let addon_dir = &config.ui_path;

    for entry in WalkDir::new(addon_dir)
        .into_iter()
        .filter_entry(|e| !should_skip(e.path()))
        .filter_map(|e| e.ok())
        .filter(|e| e.file_type().is_file())
    {
        let path = entry.path();
        let ext = path.extension().and_then(|e| e.to_str()).unwrap_or("");
        if ext != "lua" && ext != "xml" {
            continue;
        }

        if config.filter_startup && is_load_on_demand(path, addon_dir) {
            continue;
        }

        let file_label = path
            .strip_prefix(addon_dir)
            .unwrap_or(path)
            .to_string_lossy()
            .to_string();

        match ext {
            "lua" => {
                if let Ok(content) = std::fs::read_to_string(path) {
                    scan_lua_text(&content, &file_label, &mut results, &p);
                }
            }
            "xml" => {
                scan_xml_file(path, &file_label, &mut results, &p);
            }
            _ => {}
        }
    }

    // Apply namespace filter if requested
    if let Some(ns) = &config.namespace_filter {
        results.c_api.retain(|k, _| k == ns);
    }

    // Filter C_* results against wowless allowlist to remove false positives
    if let Some(wowless_path) = &config.wowless_path {
        if let Some((valid_ns, _)) = load_valid_c_namespaces(wowless_path) {
            let before = results.c_api.len();
            results.c_api.retain(|k, _| valid_ns.contains(k));
            let filtered = before - results.c_api.len();
            if filtered > 0 {
                eprintln!(
                    "Filtered {} false-positive C_* namespace(s) using wowless allowlist ({} kept)",
                    filtered,
                    results.c_api.len()
                );
            }
        }
    }

    results
}

/// Print results in human-readable text format.
pub fn print_text(results: &AuditResults) {
    let total_ns = results.c_api.len();
    let total_methods: usize = results.c_api.values().map(|v| v.len()).sum();
    println!(
        "=== C_* API Usage ({} namespaces, {} unique methods) ===",
        total_ns, total_methods
    );
    for (ns, methods) in &results.c_api {
        let total: usize = methods.values().map(|u| u.count).sum();
        println!("{} ({} calls)", ns, total);
        let mut sorted: Vec<(&String, &SymbolUsage)> = methods.iter().collect();
        sorted.sort_by(|a, b| b.1.count.cmp(&a.1.count));
        for (method, usage) in sorted {
            println!("  .{} ({})", method, usage.count);
        }
    }

    println!();
    println!(
        "=== Global Function Calls ({} unique) ===",
        results.global_functions.len()
    );
    for (sym, usage) in &results.global_functions {
        println!("{} ({} files)", sym, usage.files.len());
    }

    println!();
    println!(
        "=== Inherited Templates ({} unique) ===",
        results.inherited_templates.len()
    );
    for (sym, usage) in &results.inherited_templates {
        println!("{} ({} files)", sym, usage.files.len());
    }

    println!();
    println!(
        "=== LE_* Constants ({} unique) ===",
        results.constants.len()
    );
    for (sym, usage) in &results.constants {
        println!("{} ({} files)", sym, usage.files.len());
    }

    println!();
    println!("=== Enum.* References ({} unique) ===", results.enums.len());
    for (sym, usage) in &results.enums {
        println!("{} ({} files)", sym, usage.files.len());
    }

    println!();
    println!(
        "=== Other Constants ({} unique) ===",
        results.other_constants.len()
    );
    for (sym, usage) in &results.other_constants {
        println!("{} ({} files)", sym, usage.files.len());
    }
}

/// Print results as JSON.
pub fn print_json(results: &AuditResults, gap: Option<&GapReport>) {
    #[derive(Serialize)]
    struct Output<'a> {
        c_api: &'a BTreeMap<String, BTreeMap<String, SymbolUsage>>,
        global_functions: &'a BTreeMap<String, SymbolUsage>,
        inherited_templates: &'a BTreeMap<String, SymbolUsage>,
        constants: &'a BTreeMap<String, SymbolUsage>,
        enums: &'a BTreeMap<String, SymbolUsage>,
        other_constants: &'a BTreeMap<String, SymbolUsage>,
        #[serde(skip_serializing_if = "Option::is_none")]
        gap: Option<&'a GapReport>,
    }
    let out = Output {
        c_api: &results.c_api,
        global_functions: &results.global_functions,
        inherited_templates: &results.inherited_templates,
        constants: &results.constants,
        enums: &results.enums,
        other_constants: &results.other_constants,
        gap,
    };
    println!("{}", serde_json::to_string_pretty(&out).unwrap());
}

// ── Simulator source scanning ────────────────────────────────────────

/// What the simulator has registered, collected from Rust source files.
#[derive(Debug, Default, Serialize)]
pub struct SimRegistered {
    /// C_* namespace names registered via `.set("C_Name", ...)`.
    pub c_namespaces: BTreeSet<String>,
    /// LE_* constant names found in Rust source + Lua data files.
    pub le_constants: BTreeSet<String>,
    /// Enum namespace names registered (e.g. "Enum.SpellBookItemType").
    pub enum_namespaces: BTreeSet<String>,
}

/// A single missing symbol with its usage count.
#[derive(Debug, Serialize)]
pub struct GapEntry {
    pub name: String,
    pub calls: usize,
}

/// Summary counts for one symbol family.
#[derive(Debug, Serialize)]
pub struct GapSummary {
    pub registered: usize,
    pub used: usize,
    pub missing: usize,
}

/// Full gap report comparing Blizzard UI usage against simulator registrations.
#[derive(Debug, Serialize)]
pub struct GapReport {
    pub c_namespaces: GapSummary,
    pub le_constants: GapSummary,
    pub enum_namespaces: GapSummary,
    pub missing_c_namespaces: Vec<GapEntry>,
    pub missing_le_constants: Vec<GapEntry>,
    pub missing_enum_namespaces: Vec<GapEntry>,
    /// C_* namespace → list of (method_name, usage_count) used by Blizzard UI but not registered
    pub missing_c_methods: BTreeMap<String, Vec<(String, usize)>>,
}

/// Introspect the simulator's Lua environment to find registered C_* methods.
///
/// Creates a `WowLuaEnv` (no addon loading — just the Rust-registered globals)
/// and iterates all `C_*` globals to collect their string keys. This is 100%
/// accurate since it reads the actual Lua state, not Rust source heuristics.
pub fn introspect_simulator_c_methods() -> BTreeMap<String, BTreeSet<String>> {
    use wow_ui_sim::lua_api::WowLuaEnv;

    let env = WowLuaEnv::new().expect("Failed to create Lua environment");
    let table: mlua::Table = env
        .lua()
        .load(
            r#"
            local result = {}
            for k, v in pairs(_G) do
                if type(k) == "string" and k:match("^C_") and type(v) == "table" then
                    local methods = {}
                    for mk, _ in pairs(v) do
                        if type(mk) == "string" then
                            methods[#methods + 1] = mk
                        end
                    end
                    result[k] = methods
                end
            end
            return result
        "#,
        )
        .eval()
        .expect("C_* introspection failed");

    let mut map = BTreeMap::new();
    for pair in table.pairs::<String, mlua::Table>() {
        let (ns, methods_table) = pair.expect("pair iteration failed");
        let mut methods = BTreeSet::new();
        for method_pair in methods_table.pairs::<i64, String>() {
            let (_, method) = method_pair.expect("method iteration failed");
            methods.insert(method);
        }
        map.insert(ns, methods);
    }
    map
}

/// Scan the simulator Rust source for C_* method registrations (static fallback).
///
/// For each Rust function, we look for `globals().set("C_Foo", var)` to identify which
/// variable holds a given namespace, then collect all `var.set("Method", ...)` in that
/// same function scope. Less accurate than `introspect_simulator_c_methods` but doesn't
/// require loading the Lua environment.
#[allow(dead_code)]
pub fn scan_simulator_c_methods(src_path: &Path) -> BTreeMap<String, BTreeSet<String>> {
    let mut result: BTreeMap<String, BTreeSet<String>> = BTreeMap::new();

    // Regex patterns for method scanning
    let globals_set_c_re =
        Regex::new(r#"\.set\("(C_[A-Za-z][A-Za-z0-9_]*)",\s*(\w+)(?:\.clone\(\))?"#).unwrap();
    let table_set_re = Regex::new(r#"(\w+)\.set\("([A-Za-z][A-Za-z0-9_]*)""#).unwrap();
    // For generated_stubs: t.get::<Value>("MethodName")
    let table_get_re = Regex::new(r#"(\w+)\.get::<[^>]+>\("([A-Za-z][A-Za-z0-9_]*)"\)"#).unwrap();
    // For generated_stubs: fn register_c_foo_N — detect which namespace owns the sub-function
    let sub_fn_re = Regex::new(r"^fn register_c_([a-z][a-z0-9_]*)_\d+\b").unwrap();
    let factory_fn_re = Regex::new(
        r"^(?:pub(?:\([^)]*\))?\s+)?(?:async\s+)?fn\s+((?:register_c_|make_c_)[a-z][a-z0-9_]*)\b",
    )
    .unwrap();

    for entry in WalkDir::new(src_path)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.file_type().is_file())
        .filter(|e| e.path().extension().and_then(|e| e.to_str()).unwrap_or("") == "rs")
    {
        let path = entry.path();
        let content = match std::fs::read_to_string(path) {
            Ok(c) => c,
            Err(_) => continue,
        };
        scan_rust_file_for_c_methods(
            &content,
            &globals_set_c_re,
            &table_set_re,
            &table_get_re,
            &sub_fn_re,
            &factory_fn_re,
            &mut result,
        );
    }

    result
}

/// Scan a single Rust file for C_* method registrations.
///
/// Strategy: split into function-level blocks, then for each block:
/// 1. Find `something.set("C_Foo", varname)` → remember varname → namespace mapping
/// 2. Collect all `varname.set("Method", ...)` for non-C_ method names
/// 3. For generated_stubs sub-functions, infer namespace from function name pattern
///
/// Phase 2 scans the full file to catch methods registered in helper sub-functions
/// that receive the namespace table as a parameter named `t`.
fn scan_rust_file_for_c_methods(
    content: &str,
    globals_set_c_re: &Regex,
    table_set_re: &Regex,
    table_get_re: &Regex,
    sub_fn_re: &Regex,
    factory_fn_re: &Regex,
    result: &mut BTreeMap<String, BTreeSet<String>>,
) {
    // Split into function blocks by finding `fn ` at start of a line (with optional pub/async)
    let fn_block_re = Regex::new(r"(?m)^(?:pub(?:\([^)]*\))?\s+)?(?:async\s+)?fn\s+\w+").unwrap();
    let fn_positions: Vec<usize> = fn_block_re.find_iter(content).map(|m| m.start()).collect();

    // Generated stubs: process each `fn register_c_foo_N` sub-function independently.
    for (i, &start) in fn_positions.iter().enumerate() {
        let end = fn_positions.get(i + 1).copied().unwrap_or(content.len());
        let block = &content[start..end];
        let first_line = block.lines().next().unwrap_or("");
        let Some(cap) = sub_fn_re.captures(first_line) else {
            continue;
        };
        let ns = snake_to_c_namespace(&cap[1]);
        for cap in table_set_re.captures_iter(block) {
            if &cap[1] != "t" || cap[2].starts_with("C_") {
                continue;
            }
            result
                .entry(ns.clone())
                .or_default()
                .insert(cap[2].to_string());
        }
        for cap in table_get_re.captures_iter(block) {
            if &cap[1] != "t" || cap[2].starts_with("C_") {
                continue;
            }
            result
                .entry(ns.clone())
                .or_default()
                .insert(cap[2].to_string());
        }
    }

    // Dedicated factory functions: `fn register_c_map(...)` or `fn make_c_dye_color(...)`
    // own exactly one namespace table inside their block, even in files that register many
    // namespaces through helper functions.
    for (i, &start) in fn_positions.iter().enumerate() {
        let end = fn_positions.get(i + 1).copied().unwrap_or(content.len());
        let block = &content[start..end];
        let first_line = block.lines().next().unwrap_or("");
        if sub_fn_re.is_match(first_line) {
            continue;
        }
        let Some(cap) = factory_fn_re.captures(first_line) else {
            continue;
        };
        let full_name = &cap[1];
        let stem = full_name
            .strip_prefix("register_c_")
            .or_else(|| full_name.strip_prefix("make_c_"))
            .unwrap_or(full_name);
        let ns = snake_to_c_namespace(stem);
        for cap in table_set_re.captures_iter(block) {
            if &cap[1] != "t" || cap[2].starts_with("C_") {
                continue;
            }
            result
                .entry(ns.clone())
                .or_default()
                .insert(cap[2].to_string());
        }
        for cap in table_get_re.captures_iter(block) {
            if &cap[1] != "t" || cap[2].starts_with("C_") {
                continue;
            }
            result
                .entry(ns.clone())
                .or_default()
                .insert(cap[2].to_string());
        }
    }

    // Phase 1: collect variable → namespace mappings from all functions in the file.
    let mut var_to_ns: std::collections::HashMap<String, String> = std::collections::HashMap::new();
    for cap in globals_set_c_re.captures_iter(content) {
        let ns = cap[1].to_string();
        let var = cap[2].to_string();
        var_to_ns.insert(var, ns);
    }

    if var_to_ns.is_empty() {
        return;
    }

    // Phase 2: for simple single-namespace files, also treat `t` as an alias
    // (helper functions receive the namespace table as parameter `t`).
    let has_named_namespace_functions = fn_positions.iter().any(|&start| {
        let first_line = content[start..].lines().next().unwrap_or("");
        sub_fn_re.is_match(first_line) || factory_fn_re.is_match(first_line)
    });
    let unique_ns: std::collections::HashSet<&String> = var_to_ns.values().collect();
    if unique_ns.len() == 1 && !has_named_namespace_functions {
        let ns = unique_ns.into_iter().next().unwrap().clone();
        var_to_ns.entry("t".to_string()).or_insert(ns);
    }

    // Scan full file for `var.set("Method", ...)` and `var.get::<T>("Method")`
    for cap in table_set_re.captures_iter(content) {
        let var = cap[1].to_string();
        let method = cap[2].to_string();
        if method.starts_with("C_") {
            continue;
        }
        if let Some(ns) = var_to_ns.get(&var) {
            result.entry(ns.clone()).or_default().insert(method);
        }
    }
    for cap in table_get_re.captures_iter(content) {
        let var = cap[1].to_string();
        let method = cap[2].to_string();
        if method.starts_with("C_") {
            continue;
        }
        if let Some(ns) = var_to_ns.get(&var) {
            result.entry(ns.clone()).or_default().insert(method);
        }
    }
}

/// Convert a snake_case C_* namespace identifier to its PascalCase form.
///
/// e.g. "container" → "C_Container", "new_items" → "C_NewItems",
///      "c_var" → "C_CVar", "add_on_profiler" → "C_AddOnProfiler"
fn snake_to_c_namespace(snake: &str) -> String {
    let parts: Vec<&str> = snake.split('_').collect();
    let pascal: String = parts
        .iter()
        .map(|p| {
            let mut c = p.chars();
            match c.next() {
                None => String::new(),
                Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
            }
        })
        .collect();
    format!("C_{}", pascal)
}

/// Scan the simulator Rust source tree and Lua data files to collect registered symbols.
pub fn scan_simulator(src_path: &Path) -> SimRegistered {
    let mut reg = SimRegistered::default();

    let set_c_re = Regex::new(r#"\.set\("(C_[A-Za-z][A-Za-z0-9_]*)""#).unwrap();
    let le_rs_re = Regex::new(r#""(LE_[A-Z][A-Z0-9_]+)""#).unwrap();
    let le_lua_re = Regex::new(r"LE_[A-Z][A-Z0-9_]+").unwrap();
    let enum_rs_re = Regex::new(r#""([A-Z][A-Za-z0-9]+)",\s*&\["#).unwrap();
    let enum_rs_explicit_re = Regex::new(r#"\("([A-Z][A-Za-z0-9]+)",\s*&\["#).unwrap();
    let enum_lua_block_re = Regex::new(r"Enum\.([A-Za-z][A-Za-z0-9]+)\s*=\s*\{").unwrap();

    for entry in WalkDir::new(src_path)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.file_type().is_file())
    {
        let path = entry.path();
        let ext = path.extension().and_then(|e| e.to_str()).unwrap_or("");

        match ext {
            "rs" => {
                let content = match std::fs::read_to_string(path) {
                    Ok(c) => c,
                    Err(_) => continue,
                };
                for cap in set_c_re.captures_iter(&content) {
                    reg.c_namespaces.insert(cap[1].to_string());
                }
                for cap in le_rs_re.captures_iter(&content) {
                    reg.le_constants.insert(cap[1].to_string());
                }
                // Enum names from SeqEnumDef/EnumDef tuple patterns
                for cap in enum_rs_re.captures_iter(&content) {
                    reg.enum_namespaces.insert(format!("Enum.{}", &cap[1]));
                }
                let _ = enum_rs_explicit_re; // same regex used above
            }
            "lua" => {
                let content = match std::fs::read_to_string(path) {
                    Ok(c) => c,
                    Err(_) => continue,
                };
                // LE_* from missing_constants.lua and similar
                for cap in le_lua_re.find_iter(&content) {
                    reg.le_constants.insert(cap.as_str().to_string());
                }
                // Enum.* namespaces from missing_enums.lua
                for cap in enum_lua_block_re.captures_iter(&content) {
                    reg.enum_namespaces.insert(format!("Enum.{}", &cap[1]));
                }
            }
            _ => {}
        }
    }

    reg
}

/// Build a gap report comparing what Blizzard UI uses vs what the simulator registers.
///
/// `sim_methods` maps C_* namespace → registered method names (from `scan_simulator_c_methods`).
pub fn build_gap_report(
    used: &AuditResults,
    registered: &SimRegistered,
    sim_methods: &BTreeMap<String, BTreeSet<String>>,
) -> GapReport {
    // C_* namespaces: compare namespace keys from usage
    let used_c: BTreeSet<String> = used.c_api.keys().cloned().collect();
    let missing_c = collect_missing_entries(&used_c, &registered.c_namespaces, |ns| {
        used.c_api
            .get(ns)
            .map(|m| m.values().map(|u| u.count).sum())
            .unwrap_or(0)
    });

    // LE_* constants
    let used_le: BTreeSet<String> = used.constants.keys().cloned().collect();
    let missing_le = collect_missing_entries(&used_le, &registered.le_constants, |sym| {
        used.constants.get(sym).map(|u| u.count).unwrap_or(0)
    });

    // Enum namespaces: extract "Enum.X" from "Enum.X.Y" usage keys
    let used_enum_ns: BTreeSet<String> = used
        .enums
        .keys()
        .map(|k| {
            // "Enum.Foo.Bar" -> "Enum.Foo"
            let parts: Vec<&str> = k.splitn(3, '.').collect();
            if parts.len() >= 2 {
                format!("Enum.{}", parts[1])
            } else {
                k.clone()
            }
        })
        .collect();
    let missing_enum = collect_missing_entries(&used_enum_ns, &registered.enum_namespaces, |ns| {
        // sum all calls to any Enum.Namespace.* key
        let prefix = format!("{}.", ns);
        used.enums
            .iter()
            .filter(|(k, _)| k.starts_with(&prefix) || *k == ns)
            .map(|(_, u)| u.count)
            .sum()
    });

    // Per-method gap: for each C_* namespace used by Blizzard UI, find methods not in sim_methods
    let missing_c_methods = build_missing_c_methods(used, registered, sim_methods);

    GapReport {
        c_namespaces: GapSummary {
            registered: registered.c_namespaces.len(),
            used: used_c.len(),
            missing: missing_c.len(),
        },
        le_constants: GapSummary {
            registered: registered.le_constants.len(),
            used: used_le.len(),
            missing: missing_le.len(),
        },
        enum_namespaces: GapSummary {
            registered: registered.enum_namespaces.len(),
            used: used_enum_ns.len(),
            missing: missing_enum.len(),
        },
        missing_c_namespaces: missing_c,
        missing_le_constants: missing_le,
        missing_enum_namespaces: missing_enum,
        missing_c_methods,
    }
}

/// For each C_* namespace known to the simulator, find methods used by Blizzard UI
/// that are not in the simulator's registered method set.
fn build_missing_c_methods(
    used: &AuditResults,
    registered: &SimRegistered,
    sim_methods: &BTreeMap<String, BTreeSet<String>>,
) -> BTreeMap<String, Vec<(String, usize)>> {
    let mut result: BTreeMap<String, Vec<(String, usize)>> = BTreeMap::new();

    for (ns, methods_used) in &used.c_api {
        // Only report on namespaces the simulator has registered
        if !registered.c_namespaces.contains(ns) {
            continue;
        }
        let registered_methods = sim_methods.get(ns);
        let mut missing: Vec<(String, usize)> = methods_used
            .iter()
            .filter(|(method, _)| {
                registered_methods
                    .map(|m| !m.contains(*method))
                    .unwrap_or(true)
            })
            .map(|(method, usage)| (method.clone(), usage.count))
            .collect();
        if !missing.is_empty() {
            missing.sort_by(|a, b| b.1.cmp(&a.1).then(a.0.cmp(&b.0)));
            result.insert(ns.clone(), missing);
        }
    }

    result
}

fn collect_missing_entries(
    used: &BTreeSet<String>,
    registered: &BTreeSet<String>,
    call_count: impl Fn(&str) -> usize,
) -> Vec<GapEntry> {
    let mut entries: Vec<GapEntry> = used
        .iter()
        .filter(|name| !registered.contains(*name))
        .map(|name| GapEntry {
            name: name.clone(),
            calls: call_count(name),
        })
        .collect();
    entries.sort_by(|a, b| b.calls.cmp(&a.calls).then(a.name.cmp(&b.name)));
    entries
}

/// Print the gap report in human-readable text format.
pub fn print_gap_text(report: &GapReport) {
    println!("=== API Gap Report ===");
    println!();
    println!(
        "C_* Namespaces: {}/{} registered ({} missing)",
        report.c_namespaces.registered, report.c_namespaces.used, report.c_namespaces.missing
    );
    println!(
        "LE_* Constants: {}/{} registered ({} missing)",
        report.le_constants.registered, report.le_constants.used, report.le_constants.missing
    );
    println!(
        "Enum.* Namespaces: {}/{} registered ({} missing)",
        report.enum_namespaces.registered,
        report.enum_namespaces.used,
        report.enum_namespaces.missing
    );

    if !report.missing_c_namespaces.is_empty() {
        println!();
        println!(
            "--- Missing C_* Namespaces ({}) ---",
            report.missing_c_namespaces.len()
        );
        for entry in &report.missing_c_namespaces {
            println!("  {} ({} calls)", entry.name, entry.calls);
        }
    }

    if !report.missing_le_constants.is_empty() {
        println!();
        println!(
            "--- Missing LE_* Constants ({}) ---",
            report.missing_le_constants.len()
        );
        for entry in &report.missing_le_constants {
            println!("  {} ({} refs)", entry.name, entry.calls);
        }
    }

    if !report.missing_enum_namespaces.is_empty() {
        println!();
        println!(
            "--- Missing Enum.* Namespaces ({}) ---",
            report.missing_enum_namespaces.len()
        );
        for entry in &report.missing_enum_namespaces {
            println!("  {} ({} refs)", entry.name, entry.calls);
        }
    }

    if !report.missing_c_methods.is_empty() {
        let total_missing: usize = report.missing_c_methods.values().map(|v| v.len()).sum();
        println!();
        println!(
            "--- Missing C_* Methods (by namespace, {} total) ---",
            total_missing
        );
        for (ns, methods) in &report.missing_c_methods {
            println!("{} ({} missing):", ns, methods.len());
            for (method, count) in methods {
                println!("    .{} ({} refs)", method, count);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn usage(count: usize) -> SymbolUsage {
        SymbolUsage {
            count,
            files: vec!["Blizzard_Test.lua".to_string()],
        }
    }

    #[test]
    fn snake_to_c_namespace_preserves_wow_style_pascal_case() {
        assert_eq!(snake_to_c_namespace("container"), "C_Container");
        assert_eq!(snake_to_c_namespace("new_items"), "C_NewItems");
        assert_eq!(snake_to_c_namespace("c_var"), "C_CVar");
        assert_eq!(snake_to_c_namespace("add_on_profiler"), "C_AddOnProfiler");
    }

    #[test]
    fn scan_rust_file_for_c_methods_collects_direct_factory_and_generated_patterns() {
        let content = r#"
fn register_namespaces(lua: &Lua) {
    let globals = lua.globals();
    let c_container = lua.create_table().unwrap();
    globals.set("C_Container", c_container.clone()).unwrap();
    c_container.set("GetContainerNumSlots", 1).unwrap();
    c_container.set("GetContainerItemInfo", 1).unwrap();
}

fn register_c_map(lua: &Lua) {
    let t = lua.create_table().unwrap();
    t.set("GetMapInfo", 1).unwrap();
}

fn register_c_tooltip_info_0(lua: &Lua) {
    let t = lua.create_table().unwrap();
    t.set("GetHyperlink", 1).unwrap();
    let _ = t.get::<Value>("GetItem");
}
"#;

        let globals_set_c_re =
            Regex::new(r#"\.set\("(C_[A-Za-z][A-Za-z0-9_]*)",\s*(\w+)(?:\.clone\(\))?"#).unwrap();
        let table_set_re = Regex::new(r#"(\w+)\.set\("([A-Za-z][A-Za-z0-9_]*)""#).unwrap();
        let table_get_re =
            Regex::new(r#"(\w+)\.get::<[^>]+>\("([A-Za-z][A-Za-z0-9_]*)"\)"#).unwrap();
        let sub_fn_re = Regex::new(r"^fn register_c_([a-z][a-z0-9_]*)_\d+\b").unwrap();
        let factory_fn_re =
            Regex::new(r"^(?:pub(?:\([^)]*\))?\s+)?(?:async\s+)?fn\s+((?:register_c_|make_c_)[a-z][a-z0-9_]*)\b").unwrap();
        let mut result = BTreeMap::new();

        scan_rust_file_for_c_methods(
            content,
            &globals_set_c_re,
            &table_set_re,
            &table_get_re,
            &sub_fn_re,
            &factory_fn_re,
            &mut result,
        );

        assert_eq!(
            result.get("C_Container"),
            Some(&BTreeSet::from([
                "GetContainerItemInfo".to_string(),
                "GetContainerNumSlots".to_string(),
            ]))
        );
        assert_eq!(
            result.get("C_Map"),
            Some(&BTreeSet::from(["GetMapInfo".to_string(),]))
        );
        assert_eq!(
            result.get("C_TooltipInfo"),
            Some(&BTreeSet::from([
                "GetHyperlink".to_string(),
                "GetItem".to_string(),
            ]))
        );
    }

    #[test]
    fn build_gap_report_includes_only_missing_methods_for_registered_namespaces() {
        let mut used = AuditResults::default();
        used.c_api.insert(
            "C_Container".to_string(),
            BTreeMap::from([
                ("GetContainerNumSlots".to_string(), usage(3)),
                ("GetContainerItemInfo".to_string(), usage(2)),
            ]),
        );
        used.c_api.insert(
            "C_Unregistered".to_string(),
            BTreeMap::from([("NeverImplemented".to_string(), usage(5))]),
        );

        let registered = SimRegistered {
            c_namespaces: BTreeSet::from(["C_Container".to_string()]),
            ..Default::default()
        };
        let sim_methods = BTreeMap::from([(
            "C_Container".to_string(),
            BTreeSet::from(["GetContainerNumSlots".to_string()]),
        )]);

        let report = build_gap_report(&used, &registered, &sim_methods);

        assert_eq!(
            report.missing_c_methods.get("C_Container"),
            Some(&vec![("GetContainerItemInfo".to_string(), 2)])
        );
        assert!(
            !report.missing_c_methods.contains_key("C_Unregistered"),
            "missing method details should only be reported for namespaces the simulator exposes"
        );
    }

    #[test]
    fn scan_xml_file_extracts_inherited_templates() {
        let patterns = Patterns::new();
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("Blizzard_Test.xml");
        std::fs::write(
            &path,
            r#"
                <Ui xmlns="http://www.blizzard.com/wow/ui/">
                    <Frame name="ExampleFrame" inherits="DefaultPanelTemplate, PortraitFrameTemplate">
                        <Frames>
                            <Frame parentKey="Inset" inherits="InsetFrameTemplate"/>
                            <Frame parentKey="NoTemplate"/>
                        </Frames>
                    </Frame>
                </Ui>
            "#,
        )
        .unwrap();
        let mut used = AuditResults::default();

        scan_xml_file(&path, "Blizzard_Test.xml", &mut used, &patterns);

        assert_eq!(
            used.inherited_templates
                .keys()
                .cloned()
                .collect::<BTreeSet<_>>(),
            BTreeSet::from([
                "DefaultPanelTemplate".to_string(),
                "InsetFrameTemplate".to_string(),
                "PortraitFrameTemplate".to_string(),
            ])
        );
        assert_eq!(
            used.inherited_templates
                .get("DefaultPanelTemplate")
                .map(|usage| usage.count),
            Some(1)
        );
        assert_eq!(
            used.inherited_templates
                .get("InsetFrameTemplate")
                .map(|usage| usage.count),
            Some(1)
        );
    }

    #[test]
    fn scan_lua_text_extracts_only_bare_global_function_calls() {
        let patterns = Patterns::new();
        let mut used = AuditResults::default();
        let lua = r#"
            UnitName("player")
            GetSpellInfo(1)
            frame:GetSpellInfo()
            frame.GetSpellInfo()
            C_Spell.GetSpellInfo(1)

            local function LocalHelper()
                return UnitName("target")
            end
            LocalHelper()

            local UnitName = function() end
            UnitName("focus")

            function GlobalHelper()
                return GetSpellInfo(2)
            end
            GlobalHelper()
        "#;

        scan_lua_text(lua, "Blizzard_Test.lua", &mut used, &patterns);

        assert_eq!(
            used.global_functions
                .keys()
                .cloned()
                .collect::<BTreeSet<_>>(),
            BTreeSet::from(["GetSpellInfo".to_string(), "UnitName".to_string()])
        );
        assert_eq!(
            used.global_functions.get("UnitName").map(|u| u.count),
            Some(2),
            "should count bare global calls but exclude locally shadowed calls"
        );
        assert_eq!(
            used.global_functions.get("GetSpellInfo").map(|u| u.count),
            Some(2),
            "should count bare global calls but exclude method calls and locally defined helpers"
        );
    }
}
