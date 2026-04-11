use regex::Regex;
use serde::Serialize;
use std::collections::{BTreeMap, BTreeSet};
use std::path::Path;
use walkdir::WalkDir;

use super::{AuditResults, SymbolUsage};

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
#[derive(Debug, Serialize, PartialEq, Eq)]
pub struct GapEntry {
    pub name: String,
    pub calls: usize,
    pub files: Vec<String>,
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
    /// C_* namespace → missing methods used by Blizzard UI but not registered
    pub missing_c_methods: BTreeMap<String, Vec<GapEntry>>,
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
        .load(INTROSPECT_C_NAMESPACES_LUA)
        .eval()
        .expect("C_* introspection failed");
    parse_namespace_method_table(table)
}

const INTROSPECT_C_NAMESPACES_LUA: &str = r#"
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
"#;

fn parse_namespace_method_table(table: mlua::Table) -> BTreeMap<String, BTreeSet<String>> {
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
    let fn_positions = find_fn_positions(content);
    scan_named_function_blocks(content, &fn_positions, table_set_re, table_get_re, sub_fn_re, factory_fn_re, result);
    scan_variable_mapped_methods(content, &fn_positions, globals_set_c_re, table_set_re, table_get_re, sub_fn_re, factory_fn_re, result);
}

/// Find byte offsets of all `fn` declarations in a Rust file.
fn find_fn_positions(content: &str) -> Vec<usize> {
    let fn_block_re = Regex::new(r"(?m)^(?:pub(?:\([^)]*\))?\s+)?(?:async\s+)?fn\s+\w+").unwrap();
    fn_block_re.find_iter(content).map(|m| m.start()).collect()
}

/// Collect methods from `t.set("Method", ...)` and `t.get::<T>("Method")` in a block,
/// filtering to a specific variable name and skipping C_* entries.
fn collect_methods_from_block(
    block: &str,
    var_name: &str,
    table_set_re: &Regex,
    table_get_re: &Regex,
    methods: &mut BTreeSet<String>,
) {
    for cap in table_set_re.captures_iter(block).chain(table_get_re.captures_iter(block)) {
        if &cap[1] == var_name && !cap[2].starts_with("C_") {
            methods.insert(cap[2].to_string());
        }
    }
}

/// Scan generated stub sub-functions (`register_c_foo_N`) and factory functions
/// (`register_c_map`, `make_c_dye_color`) for method registrations.
fn scan_named_function_blocks(
    content: &str,
    fn_positions: &[usize],
    table_set_re: &Regex,
    table_get_re: &Regex,
    sub_fn_re: &Regex,
    factory_fn_re: &Regex,
    result: &mut BTreeMap<String, BTreeSet<String>>,
) {
    for (i, &start) in fn_positions.iter().enumerate() {
        let end = fn_positions.get(i + 1).copied().unwrap_or(content.len());
        let block = &content[start..end];
        let first_line = block.lines().next().unwrap_or("");

        let ns = if let Some(cap) = sub_fn_re.captures(first_line) {
            snake_to_c_namespace(&cap[1])
        } else if !sub_fn_re.is_match(first_line) {
            if let Some(cap) = factory_fn_re.captures(first_line) {
                let full_name = &cap[1];
                let stem = full_name
                    .strip_prefix("register_c_")
                    .or_else(|| full_name.strip_prefix("make_c_"))
                    .unwrap_or(full_name);
                snake_to_c_namespace(stem)
            } else {
                continue;
            }
        } else {
            continue;
        };

        collect_methods_from_block(block, "t", table_set_re, table_get_re,
            result.entry(ns).or_default());
    }
}

/// Scan full file for `var.set("Method", ...)` using variable→namespace mappings.
fn scan_variable_mapped_methods(
    content: &str,
    fn_positions: &[usize],
    globals_set_c_re: &Regex,
    table_set_re: &Regex,
    table_get_re: &Regex,
    sub_fn_re: &Regex,
    factory_fn_re: &Regex,
    result: &mut BTreeMap<String, BTreeSet<String>>,
) {
    let mut var_to_ns: std::collections::HashMap<String, String> = std::collections::HashMap::new();
    for cap in globals_set_c_re.captures_iter(content) {
        var_to_ns.insert(cap[2].to_string(), cap[1].to_string());
    }
    if var_to_ns.is_empty() {
        return;
    }

    // For single-namespace files without named sub-functions, treat `t` as an alias.
    let has_named_fns = fn_positions.iter().any(|&start| {
        let first_line = content[start..].lines().next().unwrap_or("");
        sub_fn_re.is_match(first_line) || factory_fn_re.is_match(first_line)
    });
    let unique_ns: std::collections::HashSet<&String> = var_to_ns.values().collect();
    if unique_ns.len() == 1 && !has_named_fns {
        let ns = unique_ns.into_iter().next().unwrap().clone();
        var_to_ns.entry("t".to_string()).or_insert(ns);
    }

    for cap in table_set_re.captures_iter(content).chain(table_get_re.captures_iter(content)) {
        let method = cap[2].to_string();
        if method.starts_with("C_") {
            continue;
        }
        if let Some(ns) = var_to_ns.get(&cap[1]) {
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
    let rs_patterns = RustScanPatterns::new();
    let lua_patterns = LuaScanPatterns::new();

    for entry in WalkDir::new(src_path)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.file_type().is_file())
    {
        let path = entry.path();
        let Ok(content) = std::fs::read_to_string(path) else {
            continue;
        };
        match path.extension().and_then(|e| e.to_str()) {
            Some("rs") => scan_rust_symbols(&content, &rs_patterns, &mut reg),
            Some("lua") => scan_lua_symbols(&content, &lua_patterns, &mut reg),
            _ => {}
        }
    }
    reg
}

struct RustScanPatterns {
    set_c: Regex,
    le: Regex,
    enum_def: Regex,
}

impl RustScanPatterns {
    fn new() -> Self {
        Self {
            set_c: Regex::new(r#"\.set\("(C_[A-Za-z][A-Za-z0-9_]*)""#).unwrap(),
            le: Regex::new(r#""(LE_[A-Z][A-Z0-9_]+)""#).unwrap(),
            enum_def: Regex::new(r#""([A-Z][A-Za-z0-9]+)",\s*&\["#).unwrap(),
        }
    }
}

struct LuaScanPatterns {
    le: Regex,
    enum_block: Regex,
}

impl LuaScanPatterns {
    fn new() -> Self {
        Self {
            le: Regex::new(r"LE_[A-Z][A-Z0-9_]+").unwrap(),
            enum_block: Regex::new(r"Enum\.([A-Za-z][A-Za-z0-9]+)\s*=\s*\{").unwrap(),
        }
    }
}

fn scan_rust_symbols(content: &str, p: &RustScanPatterns, reg: &mut SimRegistered) {
    for cap in p.set_c.captures_iter(content) {
        reg.c_namespaces.insert(cap[1].to_string());
    }
    for cap in p.le.captures_iter(content) {
        reg.le_constants.insert(cap[1].to_string());
    }
    for cap in p.enum_def.captures_iter(content) {
        reg.enum_namespaces.insert(format!("Enum.{}", &cap[1]));
    }
}

fn scan_lua_symbols(content: &str, p: &LuaScanPatterns, reg: &mut SimRegistered) {
    for cap in p.le.find_iter(content) {
        reg.le_constants.insert(cap.as_str().to_string());
    }
    for cap in p.enum_block.captures_iter(content) {
        reg.enum_namespaces.insert(format!("Enum.{}", &cap[1]));
    }
}

/// Build a gap report comparing what Blizzard UI uses vs what the simulator registers.
///
/// `sim_methods` maps C_* namespace → registered method names (from `scan_simulator_c_methods`).
pub fn build_gap_report(
    used: &AuditResults,
    registered: &SimRegistered,
    sim_methods: &BTreeMap<String, BTreeSet<String>>,
) -> GapReport {
    let (used_c, missing_c) = find_missing_c_namespaces(used, registered);
    let (used_le, missing_le) = find_missing_le_constants(used, registered);
    let (used_enum_ns, missing_enum) = find_missing_enum_namespaces(used, registered);
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

fn find_missing_c_namespaces(
    used: &AuditResults,
    registered: &SimRegistered,
) -> (BTreeSet<String>, Vec<GapEntry>) {
    let used_c: BTreeSet<String> = used.c_api.keys().cloned().collect();
    let missing = collect_missing_entries(
        &used_c,
        &registered.c_namespaces,
        |ns| {
            used.c_api
                .get(ns)
                .map(|m| m.values().map(|u| u.count).sum())
                .unwrap_or(0)
        },
        |ns| {
            used.c_api
                .get(ns)
                .map(collect_method_usage_files)
                .unwrap_or_default()
        },
    );
    (used_c, missing)
}

fn find_missing_le_constants(
    used: &AuditResults,
    registered: &SimRegistered,
) -> (BTreeSet<String>, Vec<GapEntry>) {
    let used_le: BTreeSet<String> = used.constants.keys().cloned().collect();
    let missing = collect_missing_entries(
        &used_le,
        &registered.le_constants,
        |sym| used.constants.get(sym).map(|u| u.count).unwrap_or(0),
        |sym| {
            used.constants
                .get(sym)
                .map(|usage| normalize_file_list(usage.files.clone()))
                .unwrap_or_default()
        },
    );
    (used_le, missing)
}

fn find_missing_enum_namespaces(
    used: &AuditResults,
    registered: &SimRegistered,
) -> (BTreeSet<String>, Vec<GapEntry>) {
    let used_enum_ns: BTreeSet<String> = used
        .enums
        .keys()
        .map(|k| {
            let parts: Vec<&str> = k.splitn(3, '.').collect();
            if parts.len() >= 2 {
                format!("Enum.{}", parts[1])
            } else {
                k.clone()
            }
        })
        .collect();
    let missing = collect_missing_entries(
        &used_enum_ns,
        &registered.enum_namespaces,
        |ns| {
            let prefix = format!("{}.", ns);
            used.enums
                .iter()
                .filter(|(k, _)| k.starts_with(&prefix) || *k == ns)
                .map(|(_, u)| u.count)
                .sum()
        },
        |ns| collect_prefixed_usage_files(&used.enums, &format!("{ns}."), ns),
    );
    (used_enum_ns, missing)
}

/// For each C_* namespace known to the simulator, find methods used by Blizzard UI
/// that are not in the simulator's registered method set.
fn build_missing_c_methods(
    used: &AuditResults,
    registered: &SimRegistered,
    sim_methods: &BTreeMap<String, BTreeSet<String>>,
) -> BTreeMap<String, Vec<GapEntry>> {
    let mut result: BTreeMap<String, Vec<GapEntry>> = BTreeMap::new();

    for (ns, methods_used) in &used.c_api {
        // Only report on namespaces the simulator has registered
        if !registered.c_namespaces.contains(ns) {
            continue;
        }
        let registered_methods = sim_methods.get(ns);
        let mut missing: Vec<GapEntry> = methods_used
            .iter()
            .filter(|(method, _)| {
                registered_methods
                    .map(|m| !m.contains(*method))
                    .unwrap_or(true)
            })
            .map(|(method, usage)| GapEntry {
                name: method.clone(),
                calls: usage.count,
                files: normalize_file_list(usage.files.clone()),
            })
            .collect();
        if !missing.is_empty() {
            missing.sort_by(|a, b| b.calls.cmp(&a.calls).then(a.name.cmp(&b.name)));
            result.insert(ns.clone(), missing);
        }
    }

    result
}

fn collect_missing_entries(
    used: &BTreeSet<String>,
    registered: &BTreeSet<String>,
    call_count: impl Fn(&str) -> usize,
    files_for: impl Fn(&str) -> Vec<String>,
) -> Vec<GapEntry> {
    let mut entries: Vec<GapEntry> = used
        .iter()
        .filter(|name| !registered.contains(*name))
        .map(|name| GapEntry {
            name: name.clone(),
            calls: call_count(name),
            files: files_for(name),
        })
        .collect();
    entries.sort_by(|a, b| b.calls.cmp(&a.calls).then(a.name.cmp(&b.name)));
    entries
}

fn collect_method_usage_files(methods: &BTreeMap<String, SymbolUsage>) -> Vec<String> {
    let mut files = BTreeSet::new();
    for usage in methods.values() {
        files.extend(usage.files.iter().cloned());
    }
    files.into_iter().collect()
}

fn collect_prefixed_usage_files(
    usages: &BTreeMap<String, SymbolUsage>,
    prefix: &str,
    exact_name: &str,
) -> Vec<String> {
    let mut files = BTreeSet::new();
    for (name, usage) in usages {
        if name.starts_with(prefix) || name == exact_name {
            files.extend(usage.files.iter().cloned());
        }
    }
    files.into_iter().collect()
}

fn normalize_file_list(files: Vec<String>) -> Vec<String> {
    let mut files = files;
    files.sort();
    files.dedup();
    files
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

    fn usage_in_files(count: usize, files: &[&str]) -> SymbolUsage {
        SymbolUsage {
            count,
            files: files.iter().map(|file| file.to_string()).collect(),
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
                (
                    "GetContainerItemInfo".to_string(),
                    usage_in_files(2, &["Blizzard_Container.lua", "Blizzard_Bags.lua"]),
                ),
            ]),
        );
        used.c_api.insert(
            "C_Unregistered".to_string(),
            BTreeMap::from([(
                "NeverImplemented".to_string(),
                usage_in_files(5, &["Blizzard_Unregistered.lua"]),
            )]),
        );
        used.constants.insert(
            "LE_TEST_CONSTANT".to_string(),
            usage_in_files(4, &["Blizzard_Constants.lua"]),
        );
        used.enums.insert(
            "Enum.TestNamespace.Value".to_string(),
            usage_in_files(7, &["Blizzard_Enum.lua"]),
        );

        let registered = SimRegistered {
            c_namespaces: BTreeSet::from(["C_Container".to_string()]),
            le_constants: BTreeSet::new(),
            enum_namespaces: BTreeSet::new(),
            ..Default::default()
        };
        let sim_methods = BTreeMap::from([(
            "C_Container".to_string(),
            BTreeSet::from(["GetContainerNumSlots".to_string()]),
        )]);

        let report = build_gap_report(&used, &registered, &sim_methods);

        assert_eq!(
            report.missing_c_methods.get("C_Container"),
            Some(&vec![GapEntry {
                name: "GetContainerItemInfo".to_string(),
                calls: 2,
                files: vec![
                    "Blizzard_Bags.lua".to_string(),
                    "Blizzard_Container.lua".to_string(),
                ],
            }])
        );
        assert!(
            !report.missing_c_methods.contains_key("C_Unregistered"),
            "missing method details should only be reported for namespaces the simulator exposes"
        );
        assert_eq!(
            report.missing_c_namespaces,
            vec![GapEntry {
                name: "C_Unregistered".to_string(),
                calls: 5,
                files: vec!["Blizzard_Unregistered.lua".to_string()],
            }]
        );
        assert_eq!(
            report.missing_le_constants,
            vec![GapEntry {
                name: "LE_TEST_CONSTANT".to_string(),
                calls: 4,
                files: vec!["Blizzard_Constants.lua".to_string()],
            }]
        );
        assert_eq!(
            report.missing_enum_namespaces,
            vec![GapEntry {
                name: "Enum.TestNamespace".to_string(),
                calls: 7,
                files: vec!["Blizzard_Enum.lua".to_string()],
            }]
        );
    }
}
