use regex::Regex;
use serde::Serialize;
use sha2::{Digest, Sha256};
use std::collections::{HashMap, HashSet};
use std::path::{Path, PathBuf};
use wow_ui_sim::screen::ScreenKind;
use wow_ui_sim::toc::TocFile;
use wow_ui_sim::xml::XmlElement;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum PublicationKind {
    Direct,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum AmbiguityKind {
    Mixin,
    Metatable,
    DynamicGlobal,
    Factory,
}

#[derive(Debug, PartialEq, Eq, Serialize)]
pub struct SourceAmbiguity {
    pub addon: String,
    pub file: String,
    pub line: usize,
    pub kind: AmbiguityKind,
}

#[derive(Debug, PartialEq, Eq, Serialize)]
pub struct PatchSourceTreeIndex {
    pub schema: &'static str,
    pub files: usize,
    pub sources: Vec<SourceIdentity>,
    pub records: Vec<SourceIndexRecord>,
    pub ambiguities: Vec<SourceAmbiguity>,
    pub missing: Vec<String>,
}

#[derive(Debug, PartialEq, Eq, Serialize)]
pub struct PatchSourceFileIndex {
    pub schema: &'static str,
    pub addon: String,
    pub file: String,
    pub source_hash: String,
    pub records: Vec<SourceIndexRecord>,
    pub ambiguities: Vec<SourceAmbiguity>,
}

#[derive(Debug, PartialEq, Eq, Serialize)]
pub struct SourceIdentity {
    pub file: String,
    pub sha256: String,
}

#[derive(Debug, PartialEq, Eq, Serialize)]
pub struct SourceIndexRecord {
    pub symbol: String,
    pub addon: String,
    pub file: String,
    pub line: usize,
    pub publication: PublicationKind,
}

pub fn index_lua_tree(root: &Path) -> Result<PatchSourceTreeIndex, String> {
    let paths = collect_lua_paths(root)?;
    index_lua_paths(root, paths, Vec::new())
}

pub fn index_active_lua_tree(root: &Path) -> Result<PatchSourceTreeIndex, String> {
    let reachable = collect_active_lua_paths(root)?;
    index_lua_paths(root, reachable.paths, reachable.missing)
}

fn index_lua_paths(
    root: &Path,
    paths: Vec<PathBuf>,
    missing: Vec<String>,
) -> Result<PatchSourceTreeIndex, String> {
    let mut sources = Vec::new();
    let mut records = Vec::new();
    let mut ambiguities = Vec::new();
    for path in &paths {
        let indexed = index_tree_file(root, path)?;
        sources.push(indexed.source);
        records.extend(indexed.records);
        ambiguities.extend(indexed.ambiguities);
    }
    Ok(PatchSourceTreeIndex {
        schema: "framexml-source-tree-index/v2",
        files: paths.len(),
        sources,
        records,
        ambiguities,
        missing,
    })
}

fn collect_lua_paths(root: &Path) -> Result<Vec<PathBuf>, String> {
    if !root.is_dir() {
        return Err(format!("source tree does not exist: {}", root.display()));
    }
    let mut paths = Vec::new();
    for entry in walkdir::WalkDir::new(root) {
        let entry = entry.map_err(|error| format!("failed to walk {}: {error}", root.display()))?;
        let path = entry.path();
        if entry.file_type().is_file()
            && path.extension().and_then(|extension| extension.to_str()) == Some("lua")
        {
            paths.push(entry.into_path());
        }
    }
    paths.sort();
    Ok(paths)
}

struct ReachableSourcePaths {
    paths: Vec<PathBuf>,
    missing: Vec<String>,
}

fn collect_active_lua_paths(root: &Path) -> Result<ReachableSourcePaths, String> {
    if !root.is_dir() {
        return Err(format!("source tree does not exist: {}", root.display()));
    }
    let mut paths = HashSet::new();
    let mut missing = HashSet::new();
    for entry in std::fs::read_dir(root)
        .map_err(|error| format!("failed to read {}: {error}", root.display()))?
    {
        let addon_dir = entry
            .map_err(|error| format!("failed to read {} entry: {error}", root.display()))?
            .path();
        collect_addon_lua_paths(root, &addon_dir, &mut paths, &mut missing)?;
    }
    let mut paths = paths.into_iter().collect::<Vec<_>>();
    paths.sort();
    let mut missing = missing.into_iter().collect::<Vec<_>>();
    missing.sort();
    Ok(ReachableSourcePaths { paths, missing })
}

fn collect_addon_lua_paths(
    root: &Path,
    addon_dir: &Path,
    paths: &mut HashSet<PathBuf>,
    missing: &mut HashSet<String>,
) -> Result<(), String> {
    if !addon_dir.is_dir() {
        return Ok(());
    }
    let addon_name = addon_dir
        .file_name()
        .and_then(|name| name.to_str())
        .unwrap_or_default();
    if wow_ui_sim::loader::is_addon_excluded_for_active_profile(addon_name) {
        return Ok(());
    }
    let Some(toc_path) = wow_ui_sim::loader::find_toc_file(addon_dir) else {
        return Ok(());
    };
    let toc = TocFile::from_file(&toc_path)
        .map_err(|error| format!("failed to parse {}: {error}", toc_path.display()))?;
    if toc.is_ptr_only() || toc.is_game_type_restricted() || !toc.allows_screen(ScreenKind::Game) {
        return Ok(());
    }
    collect_toc_lua_paths(root, &toc, paths, missing)
}

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
enum SourceKind {
    Lua,
    Xml,
}

#[derive(Clone, Debug, Hash, PartialEq, Eq)]
struct SourceReference {
    path: PathBuf,
    kind: SourceKind,
    fatal_if_missing: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum ReferenceOutcome {
    Continue,
    Abort,
}

fn collect_toc_lua_paths(
    root: &Path,
    toc: &TocFile,
    paths: &mut HashSet<PathBuf>,
    missing: &mut HashSet<String>,
) -> Result<(), String> {
    let mut visited_xml = HashSet::new();
    for reference in toc_source_references(toc) {
        collect_source_reference(
            root,
            &toc.addon_dir,
            reference,
            paths,
            missing,
            &mut visited_xml,
        )?;
    }
    Ok(())
}

fn toc_source_references(toc: &TocFile) -> Vec<SourceReference> {
    let folder_name = toc
        .addon_dir
        .file_name()
        .and_then(|name| name.to_str())
        .unwrap_or_default();
    toc.files
        .iter()
        .zip(toc.file_paths())
        .enumerate()
        .filter(|(index, (relative, _))| {
            !wow_ui_sim::loader::is_addon_toc_file_excluded_for_active_profile(toc, relative)
                && wow_ui_sim::loader::is_addon_toc_file_loaded_for_active_profile(
                    folder_name,
                    toc,
                    *index,
                )
        })
        .filter_map(|(_, (_, absolute))| {
            toc_source_kind(&absolute).map(|kind| SourceReference {
                path: absolute,
                kind,
                fatal_if_missing: false,
            })
        })
        .collect()
}

fn toc_source_kind(path: &Path) -> Option<SourceKind> {
    match path.extension().and_then(|extension| extension.to_str()) {
        Some("lua") => Some(SourceKind::Lua),
        Some("xml") => Some(SourceKind::Xml),
        _ => None,
    }
}

fn collect_source_reference(
    root: &Path,
    addon_root: &Path,
    reference: SourceReference,
    paths: &mut HashSet<PathBuf>,
    missing: &mut HashSet<String>,
    visited_xml: &mut HashSet<PathBuf>,
) -> Result<ReferenceOutcome, String> {
    if !reference.path.is_file() {
        missing.insert(relative_source_path(root, &reference.path));
        return Ok(if reference.fatal_if_missing {
            ReferenceOutcome::Abort
        } else {
            ReferenceOutcome::Continue
        });
    }
    if reference.kind == SourceKind::Lua {
        paths.insert(reference.path);
        return Ok(ReferenceOutcome::Continue);
    }
    if !visited_xml.insert(reference.path.clone()) {
        return Ok(ReferenceOutcome::Continue);
    }
    for child in read_xml_references(&reference.path, addon_root)? {
        if collect_source_reference(root, addon_root, child, paths, missing, visited_xml)?
            == ReferenceOutcome::Abort
        {
            return Ok(ReferenceOutcome::Abort);
        }
    }
    Ok(ReferenceOutcome::Continue)
}

fn relative_source_path(root: &Path, path: &Path) -> String {
    path.strip_prefix(root)
        .unwrap_or(path)
        .to_string_lossy()
        .replace('\\', "/")
}

fn read_xml_references(path: &Path, addon_root: &Path) -> Result<Vec<SourceReference>, String> {
    let xml = wow_ui_sim::xml::parse_xml_file(path)
        .map_err(|error| format!("failed to parse {}: {error}", path.display()))?;
    let mut references = Vec::new();
    collect_xml_references(&xml.elements, path, addon_root, &mut references);
    Ok(references)
}

fn collect_xml_references(
    elements: &[XmlElement],
    xml_path: &Path,
    addon_root: &Path,
    references: &mut Vec<SourceReference>,
) {
    for element in elements {
        match element {
            XmlElement::Script(script) | XmlElement::ScriptLower(script) => {
                if let Some(file) = &script.file {
                    push_xml_reference(
                        references,
                        xml_path,
                        addon_root,
                        file,
                        SourceKind::Lua,
                        true,
                    );
                }
            }
            XmlElement::Include(include) | XmlElement::IncludeLower(include) => {
                let kind = if include.file.ends_with(".lua") {
                    SourceKind::Lua
                } else {
                    SourceKind::Xml
                };
                push_xml_reference(
                    references,
                    xml_path,
                    addon_root,
                    &include.file,
                    kind,
                    kind == SourceKind::Xml,
                );
            }
            XmlElement::ScopedModifier(scoped) => {
                collect_xml_references(&scoped.elements, xml_path, addon_root, references);
            }
            _ => {}
        }
    }
}

fn push_xml_reference(
    references: &mut Vec<SourceReference>,
    xml_path: &Path,
    addon_root: &Path,
    file: &str,
    kind: SourceKind,
    fatal_if_missing: bool,
) {
    references.push(SourceReference {
        path: wow_ui_sim::loader::resolve_xml_include_path(xml_path, addon_root, file),
        kind,
        fatal_if_missing,
    });
}

struct IndexedTreeFile {
    source: SourceIdentity,
    records: Vec<SourceIndexRecord>,
    ambiguities: Vec<SourceAmbiguity>,
}

fn index_tree_file(root: &Path, path: &Path) -> Result<IndexedTreeFile, String> {
    let relative = path
        .strip_prefix(root)
        .map_err(|error| format!("failed to relativize {}: {error}", path.display()))?;
    let addon = relative
        .components()
        .next()
        .ok_or_else(|| format!("missing addon for {}", path.display()))?
        .as_os_str()
        .to_string_lossy()
        .into_owned();
    let contents = std::fs::read_to_string(path)
        .map_err(|error| format!("failed to read {}: {error}", path.display()))?;
    let file = relative.to_string_lossy().replace('\\', "/");
    Ok(IndexedTreeFile {
        source: SourceIdentity {
            file: file.clone(),
            sha256: sha256(contents.as_bytes()),
        },
        records: index_lua_source(&addon, &file, &contents),
        ambiguities: find_lua_ambiguities(&addon, &file, &contents),
    })
}

pub fn index_lua_file(addon: &str, path: &Path) -> Result<PatchSourceFileIndex, String> {
    let source = std::fs::read_to_string(path)
        .map_err(|error| format!("failed to read {}: {error}", path.display()))?;
    let file = path.display().to_string();
    Ok(PatchSourceFileIndex {
        schema: "framexml-source-file-index/v1",
        addon: addon.to_string(),
        file: file.clone(),
        source_hash: sha256(source.as_bytes()),
        records: index_lua_source(addon, &file, &source),
        ambiguities: find_lua_ambiguities(addon, &file, &source),
    })
}

fn sha256(bytes: &[u8]) -> String {
    format!("{:x}", Sha256::digest(bytes))
}

pub fn find_lua_ambiguities(addon: &str, file: &str, source: &str) -> Vec<SourceAmbiguity> {
    let masked = mask_lua_non_code(source);
    masked
        .lines()
        .enumerate()
        .flat_map(|(index, line)| {
            ambiguity_kinds(line)
                .into_iter()
                .map(move |kind| SourceAmbiguity {
                    addon: addon.to_string(),
                    file: file.to_string(),
                    line: index + 1,
                    kind,
                })
        })
        .collect()
}

fn ambiguity_kinds(line: &str) -> Vec<AmbiguityKind> {
    let mixin = Regex::new(r"\bMixin\s*\(").expect("mixin pattern should compile");
    let dynamic_global =
        Regex::new(r"_G\s*\[\s*[^\s\]]").expect("dynamic global pattern should compile");
    let mut kinds = Vec::new();
    if mixin.is_match(line) {
        kinds.push(AmbiguityKind::Mixin);
    }
    if line.contains("setmetatable") || line.contains("__index") {
        kinds.push(AmbiguityKind::Metatable);
    }
    if dynamic_global.is_match(line) {
        kinds.push(AmbiguityKind::DynamicGlobal);
    }
    if line.contains("CreateFromMixins") {
        kinds.push(AmbiguityKind::Factory);
    }
    kinds
}

struct PublicationPatterns {
    function: Regex,
    assignment: Regex,
    alias: Regex,
    bracket: Regex,
    bracket_shape: Regex,
}

impl PublicationPatterns {
    fn new() -> Self {
        let name = r"[A-Za-z_][A-Za-z0-9_]*(?:[.:][A-Za-z_][A-Za-z0-9_]*)*";
        Self {
            function: Regex::new(&format!(r"^\s*function\s+({name})\s*\("))
                .expect("function definition pattern should compile"),
            assignment: Regex::new(&format!(r"^\s*({name})\s*=\s*function\s*\("))
                .expect("function assignment pattern should compile"),
            alias: Regex::new(&format!(r"^\s*({name})\s*=\s*{name}\s*;?\s*$"))
                .expect("alias assignment pattern should compile"),
            bracket: Regex::new(
                r#"^\s*([A-Za-z_][A-Za-z0-9_]*)\s*\[\s*[\"']([^\"']+)[\"']\s*\]\s*="#,
            )
            .expect("bracket assignment pattern should compile"),
            bracket_shape: Regex::new(&format!(
                r"^\s*[A-Za-z_][A-Za-z0-9_]*\s*\[\s*\]\s*=\s*(?:function\s*\(|{name}\s*;?\s*$)"
            ))
            .expect("bracket shape pattern should compile"),
        }
    }
}

pub fn index_lua_source(addon: &str, file: &str, source: &str) -> Vec<SourceIndexRecord> {
    let patterns = PublicationPatterns::new();
    let masked = mask_lua_non_code(source);
    let locals = collect_local_declarations(&masked);
    source
        .lines()
        .zip(masked.lines())
        .enumerate()
        .filter_map(|(index, (original, code))| {
            let symbol = direct_publication_symbol(index, original, code, &patterns, &locals)?;
            Some(SourceIndexRecord {
                symbol,
                addon: addon.to_string(),
                file: file.to_string(),
                line: index + 1,
                publication: PublicationKind::Direct,
            })
        })
        .collect()
}

fn collect_local_declarations(source: &str) -> HashMap<String, usize> {
    let mut declarations = HashMap::new();
    let mut pending = None::<(usize, String)>;
    for (line_index, line) in source.lines().enumerate() {
        if let Some((start, text)) = &mut pending {
            text.push(' ');
            text.push_str(line.trim());
            if local_declaration_complete(text) {
                record_local_declaration(&mut declarations, *start, text);
                pending = None;
            }
            continue;
        }
        let Some(declaration) = line.trim_start().strip_prefix("local ") else {
            continue;
        };
        let mut text = declaration.to_string();
        if local_declaration_complete(&text) {
            record_local_declaration(&mut declarations, line_index, &text);
        } else {
            pending = Some((line_index, std::mem::take(&mut text)));
        }
    }
    collect_function_parameters(source, &mut declarations);
    declarations
}

fn collect_function_parameters(source: &str, declarations: &mut HashMap<String, usize>) {
    let pattern = Regex::new(r"(?s)\bfunction(?:\s+([A-Za-z_][A-Za-z0-9_:.]*))?\s*\(([^)]*)\)")
        .expect("function parameter pattern should compile");
    for captures in pattern.captures_iter(source) {
        let Some(parameter_list) = captures.get(2) else {
            continue;
        };
        let line = source[..parameter_list.start()]
            .bytes()
            .filter(|byte| *byte == b'\n')
            .count();
        if captures
            .get(1)
            .is_some_and(|function_name| function_name.as_str().contains(':'))
        {
            declarations.entry("self".to_string()).or_insert(line);
        }
        for parameter in parameter_list
            .as_str()
            .split(',')
            .map(str::trim)
            .filter(|parameter| is_identifier(parameter))
        {
            declarations.entry(parameter.to_string()).or_insert(line);
        }
    }
}

fn local_declaration_complete(declaration: &str) -> bool {
    declaration.contains('=') || declaration.contains(';') || !declaration.trim_end().ends_with(',')
}

fn record_local_declaration(
    declarations: &mut HashMap<String, usize>,
    line: usize,
    declaration: &str,
) {
    if let Some(function) = declaration.strip_prefix("function ") {
        let name = function.split('(').next().unwrap_or_default().trim();
        if is_identifier(name) {
            declarations.entry(name.to_string()).or_insert(line);
        }
        return;
    }
    let names = declaration.split(['=', ';']).next().unwrap_or(declaration);
    for name in names
        .split(',')
        .map(str::trim)
        .filter(|name| is_identifier(name))
    {
        declarations.entry(name.to_string()).or_insert(line);
    }
}

fn is_identifier(name: &str) -> bool {
    let mut characters = name.chars();
    characters
        .next()
        .is_some_and(|first| first == '_' || first.is_ascii_alphabetic())
        && characters.all(|character| character == '_' || character.is_ascii_alphanumeric())
}

fn direct_publication_symbol(
    line: usize,
    original: &str,
    code: &str,
    patterns: &PublicationPatterns,
    locals: &HashMap<String, usize>,
) -> Option<String> {
    let direct = patterns
        .function
        .captures(code)
        .or_else(|| patterns.assignment.captures(code));
    let alias = (!assignment_rhs_is_lua_keyword(code))
        .then(|| patterns.alias.captures(code))
        .flatten();
    let raw = direct.or(alias).map(|captures| captures[1].to_string());
    if let Some(raw) = raw {
        return is_global_at_line(&raw, line, locals).then(|| normalize_global_name(&raw));
    }
    bracket_publication_symbol(line, original, code, patterns, locals)
}

fn is_global_at_line(name: &str, line: usize, locals: &HashMap<String, usize>) -> bool {
    let normalized = name.replace(':', ".");
    let root = normalized.split('.').next().unwrap_or_default();
    locals
        .get(root)
        .is_none_or(|declaration| *declaration > line)
}

fn normalize_global_name(name: &str) -> String {
    let normalized = name.replace(':', ".");
    normalized
        .strip_prefix("_G.")
        .unwrap_or(&normalized)
        .to_string()
}

fn assignment_rhs_is_lua_keyword(code: &str) -> bool {
    let Some((_, value)) = code.split_once('=') else {
        return false;
    };
    matches!(value.trim().trim_end_matches(';'), "nil" | "true" | "false")
}

fn bracket_publication_symbol(
    line: usize,
    original: &str,
    code: &str,
    patterns: &PublicationPatterns,
    locals: &HashMap<String, usize>,
) -> Option<String> {
    (!assignment_rhs_is_lua_keyword(code)).then_some(())?;
    patterns.bracket_shape.is_match(code).then_some(())?;
    let captures = patterns.bracket.captures(original)?;
    let table = &captures[1];
    is_global_at_line(table, line, locals).then_some(())?;
    let field = &captures[2];
    Some(if table == "_G" {
        field.to_string()
    } else {
        format!("{table}.{field}")
    })
}

#[derive(Clone, Copy)]
enum MaskState {
    Code,
    ShortString { quote: u8, escaped: bool },
    LineComment,
    Long { equals: usize },
}

fn mask_lua_non_code(source: &str) -> String {
    let bytes = source.as_bytes();
    let mut output = vec![b' '; bytes.len()];
    let mut state = MaskState::Code;
    let mut index = 0;
    while index < bytes.len() {
        if bytes[index] == b'\n' {
            output[index] = b'\n';
            if matches!(state, MaskState::LineComment) {
                state = MaskState::Code;
            }
            index += 1;
            continue;
        }
        let consumed = mask_next(bytes, &mut output, index, &mut state);
        index += consumed;
    }
    String::from_utf8_lossy(&output).into_owned()
}

fn mask_next(bytes: &[u8], output: &mut [u8], index: usize, state: &mut MaskState) -> usize {
    match *state {
        MaskState::Code => mask_code_byte(bytes, output, index, state),
        MaskState::ShortString { quote, escaped } => {
            mask_short_string_byte(bytes, index, state, quote, escaped)
        }
        MaskState::LineComment => 1,
        MaskState::Long { equals } => mask_long_byte(bytes, index, state, equals),
    }
}

fn mask_code_byte(bytes: &[u8], output: &mut [u8], index: usize, state: &mut MaskState) -> usize {
    if bytes[index..].starts_with(b"--") {
        if let Some((equals, length)) = long_bracket_open(bytes, index + 2) {
            *state = MaskState::Long { equals };
            return length + 2;
        }
        *state = MaskState::LineComment;
        return 2;
    }
    if matches!(bytes[index], b'\'' | b'\"') {
        *state = MaskState::ShortString {
            quote: bytes[index],
            escaped: false,
        };
        return 1;
    }
    if let Some((equals, length)) = long_bracket_open(bytes, index) {
        *state = MaskState::Long { equals };
        return length;
    }
    output[index] = bytes[index];
    1
}

fn mask_short_string_byte(
    bytes: &[u8],
    index: usize,
    state: &mut MaskState,
    quote: u8,
    escaped: bool,
) -> usize {
    if escaped {
        *state = MaskState::ShortString {
            quote,
            escaped: false,
        };
    } else if bytes[index] == b'\\' {
        *state = MaskState::ShortString {
            quote,
            escaped: true,
        };
    } else if bytes[index] == quote {
        *state = MaskState::Code;
    }
    1
}

fn mask_long_byte(bytes: &[u8], index: usize, state: &mut MaskState, equals: usize) -> usize {
    let length = equals + 2;
    if long_bracket_close(bytes, index, equals) {
        *state = MaskState::Code;
        length
    } else {
        1
    }
}

fn long_bracket_open(bytes: &[u8], index: usize) -> Option<(usize, usize)> {
    if bytes.get(index) != Some(&b'[') {
        return None;
    }
    let equals = bytes[index + 1..]
        .iter()
        .take_while(|byte| **byte == b'=')
        .count();
    (bytes.get(index + equals + 1) == Some(&b'[')).then_some((equals, equals + 2))
}

fn long_bracket_close(bytes: &[u8], index: usize, equals: usize) -> bool {
    bytes.get(index) == Some(&b']')
        && bytes[index + 1..]
            .iter()
            .take(equals)
            .all(|byte| *byte == b'=')
        && bytes.get(index + equals + 1) == Some(&b']')
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn excludes_block_comments_long_strings_and_local_namespaces() {
        let source = r#"
            --[[ function BlockComment() end ]]
            function BeforeLaterLocal() end
            local BeforeLaterLocal
            local LocalNamespace = {}
            function LocalNamespace.Method() end
            local FirstLocal,
                SecondLocal = {}, {}
            function SecondLocal.Method() end
            function _G.RealGlobal() end
            _G.AliasGlobal = ExistingFunction
            local _G = {}
            function _G.ShadowedGlobal() end
            local text = [=[
                function LongString() end
                Mixin(Fake, Source)
            ]=]
            Mixin(Target, Source); setmetatable(Target, Meta)
        "#;

        let records = index_lua_source("Blizzard_Test", "Scope.lua", source);
        let names: Vec<&str> = records
            .iter()
            .map(|record| record.symbol.as_str())
            .collect();
        let ambiguities = find_lua_ambiguities("Blizzard_Test", "Scope.lua", source);
        let kinds: Vec<AmbiguityKind> = ambiguities.iter().map(|item| item.kind).collect();

        assert_eq!(names, vec!["BeforeLaterLocal", "RealGlobal", "AliasGlobal"]);
        assert_eq!(kinds, vec![AmbiguityKind::Mixin, AmbiguityKind::Metatable]);
    }

    #[test]
    fn bracket_publications_require_function_or_alias_values() {
        let source = r#"
            _G["FunctionValue"] = function() end
            _G["AliasValue"] = ExistingFunction
            _G["Constant"] = 42
            _G["TableValue"] = {}
            RemovedGlobal = nil
            _G["RemovedBracket"] = nil
            FalseGlobal = false
            TrueGlobal = true
        "#;

        let records = index_lua_source("Blizzard_Test", "Bracket.lua", source);
        let names: Vec<&str> = records
            .iter()
            .map(|record| record.symbol.as_str())
            .collect();

        assert_eq!(names, vec!["FunctionValue", "AliasValue"]);
    }

    #[test]
    fn missing_lua_tree_is_an_error() {
        let path = std::env::temp_dir().join("wow-ui-sim-missing-source-tree");
        let error = index_lua_tree(&path).expect_err("missing tree must fail");
        assert!(error.contains("source tree"));
    }

    #[test]
    fn active_toc_index_excludes_other_flavors_and_follows_xml_scripts() {
        let directory = tempfile::tempdir().expect("temporary directory should create");
        let addon = directory.path().join("ExampleAddon");
        std::fs::create_dir_all(&addon).expect("addon directory should create");
        std::fs::write(
            addon.join("ExampleAddon_Mainline.toc"),
            "Mainline.lua\nMainline/Shared.xml\n",
        )
        .expect("mainline TOC should write");
        std::fs::write(addon.join("ExampleAddon_Mists.toc"), "Mists.lua\n")
            .expect("Mists TOC should write");
        std::fs::create_dir_all(addon.join("Mainline")).expect("mainline directory should create");
        std::fs::write(
            addon.join("Mainline/Shared.xml"),
            r#"<Ui><ScopedModifier><script file="MAINLINE\Nested.lua"/></ScopedModifier><include file="Missing.xml"/><Script file="NeverLoaded.lua"/></Ui>"#,
        )
        .expect("XML fixture should write");
        std::fs::write(addon.join("Mainline.lua"), "function MainlineOnly() end\n")
            .expect("mainline Lua should write");
        std::fs::write(
            addon.join("Mainline/Nested.lua"),
            "function NestedReachable() end\n",
        )
        .expect("nested Lua should write");
        std::fs::write(addon.join("Mists.lua"), "function MistsOnly() end\n")
            .expect("Mists Lua should write");
        std::fs::write(
            addon.join("Mainline/NeverLoaded.lua"),
            "function NeverLoadedAfterFatalInclude() end\n",
        )
        .expect("post-failure Lua should write");

        let index = index_active_lua_tree(directory.path()).expect("active TOCs should index");
        let symbols = index
            .records
            .iter()
            .map(|record| record.symbol.as_str())
            .collect::<Vec<_>>();

        assert!(symbols.contains(&"MainlineOnly"));
        assert!(symbols.contains(&"NestedReachable"));
        assert!(!symbols.contains(&"MistsOnly"));
        assert!(!symbols.contains(&"NeverLoadedAfterFatalInclude"));
        assert_eq!(index.files, 2);
        assert_eq!(index.missing, vec!["ExampleAddon/Mainline/Missing.xml"]);
    }

    #[cfg(feature = "client-mists")]
    #[test]
    fn active_toc_index_applies_profile_specific_toc_file_exclusions() {
        let directory = tempfile::tempdir().expect("temporary directory should create");
        let addon = directory.path().join("Blizzard_Collections");
        std::fs::create_dir_all(&addon).expect("addon directory should create");
        std::fs::write(
            addon.join("Blizzard_Collections_Mists.toc"),
            "Wardrobe.lua\nCollections.lua\n",
        )
        .expect("Mists TOC should write");
        std::fs::write(addon.join("Wardrobe.lua"), "function WardrobeLeak() end\n")
            .expect("Wardrobe Lua should write");
        std::fs::write(
            addon.join("Collections.lua"),
            "function CollectionsReachable() end\n",
        )
        .expect("Collections Lua should write");

        let index = index_active_lua_tree(directory.path()).expect("active TOCs should index");
        let symbols = index
            .records
            .iter()
            .map(|record| record.symbol.as_str())
            .collect::<Vec<_>>();

        assert!(symbols.contains(&"CollectionsReachable"));
        assert!(!symbols.contains(&"WardrobeLeak"));
    }

    #[test]
    fn active_toc_index_applies_toc_environment_filters() {
        let directory = tempfile::tempdir().expect("temporary directory should create");
        let addon = directory.path().join("EnvironmentFiltered");
        std::fs::create_dir_all(&addon).expect("addon directory should create");
        std::fs::write(
            addon.join("EnvironmentFiltered_Mainline.toc"),
            "Restricted.lua [AllowLoadEnvironment secure]\n",
        )
        .expect("environment-filtered TOC should write");
        std::fs::write(
            addon.join("Restricted.lua"),
            "function FalsePositiveGlobal() end\n",
        )
        .expect("restricted Lua should write");

        let index = index_active_lua_tree(directory.path()).expect("active TOCs should index");

        assert!(index.records.is_empty());
        assert_eq!(index.files, 0);
    }

    #[cfg(not(feature = "client-ptr"))]
    #[test]
    fn active_toc_index_excludes_ptr_only_addons_on_live_profiles() {
        let directory = tempfile::tempdir().expect("temporary directory should create");
        let addon = directory.path().join("PtrOnlyAddon");
        std::fs::create_dir_all(&addon).expect("addon directory should create");
        std::fs::write(
            addon.join("PtrOnlyAddon.toc"),
            "## OnlyBetaAndPTR: 1\nPtrOnly.lua\n",
        )
        .expect("PTR-only TOC should write");
        std::fs::write(addon.join("PtrOnly.lua"), "function PtrOnlyGlobal() end\n")
            .expect("PTR-only Lua should write");

        let index = index_active_lua_tree(directory.path()).expect("active TOCs should index");

        assert!(index.records.is_empty());
        assert_eq!(index.files, 0);
    }

    #[test]
    fn indexes_lua_tree_with_relative_paths_and_addon_ownership() {
        let directory = tempfile::tempdir().expect("temporary directory should create");
        let addon = directory.path().join("Blizzard_One");
        std::fs::create_dir_all(addon.join("Nested")).expect("addon directories should create");
        std::fs::write(addon.join("Nested/One.lua"), "function TreeGlobal() end\n")
            .expect("Lua fixture should write");
        std::fs::write(addon.join("Ignored.txt"), "function Ignored() end\n")
            .expect("non-Lua fixture should write");

        let index = index_lua_tree(directory.path()).expect("tree should index");

        assert_eq!(index.schema, "framexml-source-tree-index/v2");
        assert_eq!(index.files, 1);
        assert_eq!(index.sources.len(), 1);
        assert_eq!(index.sources[0].file, "Blizzard_One/Nested/One.lua");
        assert_eq!(index.sources[0].sha256.len(), 64);
        assert_eq!(index.records[0].addon, "Blizzard_One");
        assert_eq!(index.records[0].file, "Blizzard_One/Nested/One.lua");
        assert_eq!(index.records[0].symbol, "TreeGlobal");
    }

    #[test]
    fn indexes_a_lua_file_with_source_identity() {
        let directory = tempfile::tempdir().expect("temporary directory should create");
        let path = directory.path().join("Fixture.lua");
        std::fs::write(&path, "function FileGlobal() end\nMixin(Target, Source)\n")
            .expect("fixture should write");

        let index = index_lua_file("Blizzard_Fixture", &path).expect("source file should index");

        assert_eq!(index.addon, "Blizzard_Fixture");
        assert_eq!(index.file, path.display().to_string());
        assert_eq!(index.source_hash.len(), 64);
        assert_eq!(index.records[0].symbol, "FileGlobal");
        assert_eq!(index.ambiguities[0].kind, AmbiguityKind::Mixin);
    }

    #[test]
    fn flags_dynamic_publication_constructs_as_ambiguities() {
        let source = r#"
            Mixin(Target, SourceMixin)
            setmetatable(Target, { __index = SourceMixin })
            _G[name] = factory()
            CreateFromMixins(SourceMixin)
        "#;

        let ambiguities = find_lua_ambiguities("Blizzard_Test", "Dynamic.lua", source);
        let kinds: Vec<AmbiguityKind> = ambiguities.iter().map(|item| item.kind).collect();

        assert_eq!(
            kinds,
            vec![
                AmbiguityKind::Mixin,
                AmbiguityKind::Metatable,
                AmbiguityKind::DynamicGlobal,
                AmbiguityKind::Factory,
            ]
        );
    }

    #[test]
    fn indexes_direct_publications_without_promoting_locals_comments_or_strings() {
        let source = r#"
            function DirectGlobal() end
            function Namespace.TableMethod() end
            Namespace.Assigned = function() end
            Namespace["Bracketed"] = function() end
            _G["IndexedGlobal"] = function() end
            local function LocalOnly() end
            local LocalAssigned = function() end
            local function PublishIntoParameter(Namespace)
                Namespace.NotGlobal = function() end
            end
            local MultilineParameter = function(
                OtherNamespace
            )
                OtherNamespace.AlsoNotGlobal = function() end
            end
            function Object:Publish()
                self.ImplicitParameterIsNotGlobal = function() end
            end
            -- function CommentedOut() end
            local text = "function StringContent() end"
        "#;

        let records = index_lua_source("Blizzard_Test", "Test.lua", source);
        let names: Vec<&str> = records
            .iter()
            .map(|record| record.symbol.as_str())
            .collect();

        assert_eq!(
            names,
            vec![
                "DirectGlobal",
                "Namespace.TableMethod",
                "Namespace.Assigned",
                "Namespace.Bracketed",
                "IndexedGlobal",
                "Object.Publish",
            ]
        );
        assert!(
            records
                .iter()
                .all(|record| record.publication == PublicationKind::Direct)
        );
    }
}
