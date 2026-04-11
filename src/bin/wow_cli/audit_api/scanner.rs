use regex::Regex;
use std::collections::{BTreeMap, BTreeSet, HashSet};
use std::path::Path;
use walkdir::WalkDir;

use super::{AuditConfig, AuditResults, SymbolUsage};

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
            c_api: Regex::new(r"(C_\w+)[.:](\w+)\s*\(").unwrap(),
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
) -> Option<(
    std::collections::HashSet<String>,
    BTreeMap<String, BTreeSet<String>>,
)> {
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
    scan_addon_files(config, &p, &mut results);
    apply_result_filters(config, &mut results);
    results
}

fn scan_addon_files(config: &AuditConfig, p: &Patterns, results: &mut AuditResults) {
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
                    scan_lua_text(&content, &file_label, results, p);
                }
            }
            "xml" => scan_xml_file(path, &file_label, results, p),
            _ => {}
        }
    }
}

fn apply_result_filters(config: &AuditConfig, results: &mut AuditResults) {
    if let Some(ns) = &config.namespace_filter {
        results.c_api.retain(|k, _| k == ns);
    }
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
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::BTreeSet;

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
    fn run_audit_filter_startup_skips_load_on_demand_addons() {
        let dir = tempfile::tempdir().unwrap();
        let startup_addon = dir.path().join("Blizzard_StartupTest");
        let lod_addon = dir.path().join("Blizzard_LoadOnDemandTest");
        std::fs::create_dir_all(&startup_addon).unwrap();
        std::fs::create_dir_all(&lod_addon).unwrap();

        std::fs::write(
            startup_addon.join("Blizzard_StartupTest.toc"),
            "## Interface: 110000\nBlizzard_StartupTest.lua\n",
        )
        .unwrap();
        std::fs::write(
            startup_addon.join("Blizzard_StartupTest.lua"),
            "UnitName('player')\n",
        )
        .unwrap();

        std::fs::write(
            lod_addon.join("Blizzard_LoadOnDemandTest.toc"),
            "## Interface: 110000\n## LoadOnDemand: 1\nBlizzard_LoadOnDemandTest.lua\n",
        )
        .unwrap();
        std::fs::write(
            lod_addon.join("Blizzard_LoadOnDemandTest.lua"),
            "GetSpellInfo(1)\n",
        )
        .unwrap();

        let startup_only = run_audit(&AuditConfig {
            ui_path: dir.path().to_path_buf(),
            namespace_filter: None,
            filter_startup: true,
            wowless_path: None,
        });
        let all_addons = run_audit(&AuditConfig {
            ui_path: dir.path().to_path_buf(),
            namespace_filter: None,
            filter_startup: false,
            wowless_path: None,
        });

        assert_eq!(
            startup_only
                .global_functions
                .keys()
                .cloned()
                .collect::<BTreeSet<_>>(),
            BTreeSet::from(["UnitName".to_string()])
        );
        assert_eq!(
            all_addons
                .global_functions
                .keys()
                .cloned()
                .collect::<BTreeSet<_>>(),
            BTreeSet::from(["GetSpellInfo".to_string(), "UnitName".to_string()])
        );
    }

    #[test]
    fn scan_lua_text_only_counts_c_api_usages_in_call_context() {
        let patterns = Patterns::new();
        let mut used = AuditResults::default();
        let lua = r#"
            local getter = C_Spell.GetSpellInfo
            if C_Spell.GetSpellInfo then
                let maybe = C_Spell.GetSpellInfo
            end

            C_Spell.GetSpellInfo(1)
            C_Container.GetContainerNumSlots(0)
            C_Timer:After(0, function() end)
        "#;

        scan_lua_text(lua, "Blizzard_Test.lua", &mut used, &patterns);

        assert_eq!(
            used.c_api
                .get("C_Spell")
                .and_then(|methods| methods.get("GetSpellInfo"))
                .map(|usage| usage.count),
            Some(1)
        );
        assert_eq!(
            used.c_api
                .get("C_Container")
                .and_then(|methods| methods.get("GetContainerNumSlots"))
                .map(|usage| usage.count),
            Some(1)
        );
        assert_eq!(
            used.c_api
                .get("C_Timer")
                .and_then(|methods| methods.get("After"))
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

    // Silence unused import warnings for test helpers
    fn _use_helpers() {
        let _ = usage(0);
        let _ = usage_in_files(0, &[]);
    }
}
