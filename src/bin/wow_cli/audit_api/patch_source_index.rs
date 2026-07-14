use regex::Regex;
use serde::Serialize;
use sha2::{Digest, Sha256};
use std::collections::HashMap;
use std::path::Path;

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
        schema: "framexml-source-tree-index/v1",
        files: paths.len(),
        sources,
        records,
        ambiguities,
    })
}

fn collect_lua_paths(root: &Path) -> Result<Vec<std::path::PathBuf>, String> {
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
    declarations
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
    let raw = patterns
        .function
        .captures(code)
        .or_else(|| patterns.assignment.captures(code))
        .or_else(|| patterns.alias.captures(code))
        .map(|captures| captures[1].to_string());
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

fn bracket_publication_symbol(
    line: usize,
    original: &str,
    code: &str,
    patterns: &PublicationPatterns,
    locals: &HashMap<String, usize>,
) -> Option<String> {
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
    fn indexes_lua_tree_with_relative_paths_and_addon_ownership() {
        let directory = tempfile::tempdir().expect("temporary directory should create");
        let addon = directory.path().join("Blizzard_One");
        std::fs::create_dir_all(addon.join("Nested")).expect("addon directories should create");
        std::fs::write(addon.join("Nested/One.lua"), "function TreeGlobal() end\n")
            .expect("Lua fixture should write");
        std::fs::write(addon.join("Ignored.txt"), "function Ignored() end\n")
            .expect("non-Lua fixture should write");

        let index = index_lua_tree(directory.path()).expect("tree should index");

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
            ]
        );
        assert!(
            records
                .iter()
                .all(|record| record.publication == PublicationKind::Direct)
        );
    }
}
