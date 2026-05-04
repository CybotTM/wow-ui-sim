use super::*;
use regex::Regex;
use std::collections::{BTreeMap, BTreeSet, HashMap, HashSet};

struct MethodScanPatterns {
    globals_set_c: Regex,
    table_set: Regex,
    table_get: Regex,
    sub_fn: Regex,
    factory_fn: Regex,
}

impl MethodScanPatterns {
    fn new() -> Self {
        Self {
            globals_set_c: Regex::new(
                r#"\.set\("(C_[A-Za-z][A-Za-z0-9_]*)",\s*(\w+)(?:\.clone\(\))?"#,
            )
            .unwrap(),
            table_set: Regex::new(r#"(\w+)\.set\("([A-Za-z][A-Za-z0-9_]*)""#).unwrap(),
            table_get: Regex::new(r#"(\w+)\.get::<[^>]+>\("([A-Za-z][A-Za-z0-9_]*)"\)"#)
                .unwrap(),
            sub_fn: Regex::new(r"^fn register_c_([a-z][a-z0-9_]*)_\d+\b").unwrap(),
            factory_fn: Regex::new(
                r"^(?:pub(?:\([^)]*\))?\s+)?(?:async\s+)?fn\s+((?:register_c_|make_c_)[a-z][a-z0-9_]*)\b",
            )
            .unwrap(),
        }
    }
}

fn scan_rust_file_for_c_methods(
    content: &str,
    patterns: &MethodScanPatterns,
    result: &mut BTreeMap<String, BTreeSet<String>>,
) {
    let fn_positions = find_fn_positions(content);
    scan_named_function_blocks(content, &fn_positions, patterns, result);
    scan_variable_mapped_methods(content, &fn_positions, patterns, result);
}

fn find_fn_positions(content: &str) -> Vec<usize> {
    let fn_block_re = Regex::new(r"(?m)^(?:pub(?:\([^)]*\))?\s+)?(?:async\s+)?fn\s+\w+").unwrap();
    fn_block_re.find_iter(content).map(|m| m.start()).collect()
}

fn scan_named_function_blocks(
    content: &str,
    fn_positions: &[usize],
    patterns: &MethodScanPatterns,
    result: &mut BTreeMap<String, BTreeSet<String>>,
) {
    for (i, &start) in fn_positions.iter().enumerate() {
        let end = fn_positions.get(i + 1).copied().unwrap_or(content.len());
        let block = &content[start..end];
        let first_line = block.lines().next().unwrap_or("");

        let Some(namespace) = function_namespace(first_line, patterns) else {
            continue;
        };

        collect_methods_from_block(
            block,
            "t",
            &patterns.table_set,
            &patterns.table_get,
            result.entry(namespace).or_default(),
        );
    }
}

fn function_namespace(first_line: &str, patterns: &MethodScanPatterns) -> Option<String> {
    if let Some(cap) = patterns.sub_fn.captures(first_line) {
        return Some(snake_to_c_namespace(&cap[1]));
    }

    let cap = patterns.factory_fn.captures(first_line)?;
    let full_name = &cap[1];
    let stem = full_name
        .strip_prefix("register_c_")
        .or_else(|| full_name.strip_prefix("make_c_"))
        .unwrap_or(full_name);

    Some(snake_to_c_namespace(stem))
}

fn collect_methods_from_block(
    block: &str,
    var_name: &str,
    table_set_re: &Regex,
    table_get_re: &Regex,
    methods: &mut BTreeSet<String>,
) {
    for cap in table_set_re
        .captures_iter(block)
        .chain(table_get_re.captures_iter(block))
    {
        if &cap[1] == var_name && !cap[2].starts_with("C_") {
            methods.insert(cap[2].to_string());
        }
    }
}

fn scan_variable_mapped_methods(
    content: &str,
    fn_positions: &[usize],
    patterns: &MethodScanPatterns,
    result: &mut BTreeMap<String, BTreeSet<String>>,
) {
    let mut var_to_ns = variable_namespace_map(content, &patterns.globals_set_c);
    if var_to_ns.is_empty() {
        return;
    }

    add_single_namespace_alias(content, fn_positions, patterns, &mut var_to_ns);
    collect_variable_mapped_methods(content, patterns, &var_to_ns, result);
}

fn variable_namespace_map(content: &str, globals_set_c_re: &Regex) -> HashMap<String, String> {
    let mut var_to_ns = HashMap::new();
    for cap in globals_set_c_re.captures_iter(content) {
        var_to_ns.insert(cap[2].to_string(), cap[1].to_string());
    }
    var_to_ns
}

fn add_single_namespace_alias(
    content: &str,
    fn_positions: &[usize],
    patterns: &MethodScanPatterns,
    var_to_ns: &mut HashMap<String, String>,
) {
    if has_named_functions(content, fn_positions, patterns) {
        return;
    }

    let unique_ns: HashSet<&String> = var_to_ns.values().collect();
    if unique_ns.len() == 1 {
        let namespace = unique_ns.into_iter().next().unwrap().clone();
        var_to_ns.entry("t".to_string()).or_insert(namespace);
    }
}

fn has_named_functions(
    content: &str,
    fn_positions: &[usize],
    patterns: &MethodScanPatterns,
) -> bool {
    fn_positions.iter().any(|&start| {
        let first_line = content[start..].lines().next().unwrap_or("");
        patterns.sub_fn.is_match(first_line) || patterns.factory_fn.is_match(first_line)
    })
}

fn collect_variable_mapped_methods(
    content: &str,
    patterns: &MethodScanPatterns,
    var_to_ns: &HashMap<String, String>,
    result: &mut BTreeMap<String, BTreeSet<String>>,
) {
    for cap in patterns
        .table_set
        .captures_iter(content)
        .chain(patterns.table_get.captures_iter(content))
    {
        let method = cap[2].to_string();
        if method.starts_with("C_") {
            continue;
        }
        if let Some(ns) = var_to_ns.get(&cap[1]) {
            result.entry(ns.clone()).or_default().insert(method);
        }
    }
}

fn snake_to_c_namespace(snake: &str) -> String {
    let pascal: String = snake.split('_').map(capitalize_first).collect();
    format!("C_{}", pascal)
}

fn capitalize_first(part: &str) -> String {
    let mut chars = part.chars();
    match chars.next() {
        Some(first) => first.to_uppercase().collect::<String>() + chars.as_str(),
        None => String::new(),
    }
}

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

    let patterns = MethodScanPatterns::new();
    let mut result = BTreeMap::new();

    scan_rust_file_for_c_methods(content, &patterns, &mut result);

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
