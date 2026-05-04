use regex::Regex;
use rilua::vm::state::LuaState;
use rilua::{LuaApi, Val};
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
    let func = env
        .load_rilua(INTROSPECT_C_NAMESPACES_LUA)
        .expect("Failed to compile C_* introspection chunk");
    let results = env
        .call_rilua(&func, &[])
        .expect("C_* introspection failed");
    let table = results.into_iter().next().unwrap_or(Val::Nil);
    let lua = env.rilua();
    parse_namespace_method_table(lua.state(), table)
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

fn parse_namespace_method_table(
    state: &LuaState,
    table: Val,
) -> BTreeMap<String, BTreeSet<String>> {
    let mut map = BTreeMap::new();
    let Val::Table(table_ref) = table else {
        return map;
    };
    let Some(table) = state.gc.tables.get(table_ref) else {
        return map;
    };

    for (ns_key, methods_table) in table.hash_entries() {
        let Some(ns) = val_to_string(state, ns_key) else {
            continue;
        };
        let Val::Table(methods_ref) = methods_table else {
            continue;
        };
        let mut methods = BTreeSet::new();
        if let Some(methods_table) = state.gc.tables.get(methods_ref) {
            for method in methods_table.array_slice() {
                if let Some(method) = val_to_string(state, *method) {
                    methods.insert(method);
                }
            }
        }
        map.insert(ns, methods);
    }
    map
}

fn val_to_string(state: &LuaState, value: Val) -> Option<String> {
    let Val::Str(string_ref) = value else {
        return None;
    };
    let string = state.gc.string_arena.get(string_ref)?;
    Some(String::from_utf8_lossy(string.data()).into_owned())
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
/// `sim_methods` maps C_* namespace → registered method names.
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
mod gap_tests;
