use regex::Regex;
use serde::Serialize;
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
pub struct PatchSourceFileIndex {
    pub schema: &'static str,
    pub addon: String,
    pub file: String,
    pub records: Vec<SourceIndexRecord>,
    pub ambiguities: Vec<SourceAmbiguity>,
}

#[derive(Debug, PartialEq, Eq, Serialize)]
pub struct SourceIndexRecord {
    pub symbol: String,
    pub addon: String,
    pub file: String,
    pub line: usize,
    pub publication: PublicationKind,
}

pub fn index_lua_file(addon: &str, path: &Path) -> Result<PatchSourceFileIndex, String> {
    let source = std::fs::read_to_string(path)
        .map_err(|error| format!("failed to read {}: {error}", path.display()))?;
    let file = path.display().to_string();
    Ok(PatchSourceFileIndex {
        schema: "framexml-source-file-index/v1",
        addon: addon.to_string(),
        file: file.clone(),
        records: index_lua_source(addon, &file, &source),
        ambiguities: find_lua_ambiguities(addon, &file, &source),
    })
}

pub fn find_lua_ambiguities(addon: &str, file: &str, source: &str) -> Vec<SourceAmbiguity> {
    let mixin = Regex::new(r"\bMixin\s*\(").expect("mixin pattern should compile");
    let dynamic_global =
        Regex::new(r#"_G\s*\[\s*[^\"']"#).expect("dynamic global pattern should compile");
    source
        .lines()
        .enumerate()
        .filter_map(|(index, line)| {
            let trimmed = line.trim_start();
            if trimmed.starts_with("--") {
                return None;
            }
            let kind = if mixin.is_match(line) {
                AmbiguityKind::Mixin
            } else if line.contains("setmetatable") || line.contains("__index") {
                AmbiguityKind::Metatable
            } else if dynamic_global.is_match(line) {
                AmbiguityKind::DynamicGlobal
            } else if line.contains("CreateFromMixins") {
                AmbiguityKind::Factory
            } else {
                return None;
            };
            Some(SourceAmbiguity {
                addon: addon.to_string(),
                file: file.to_string(),
                line: index + 1,
                kind,
            })
        })
        .collect()
}

struct PublicationPatterns {
    function: Regex,
    assignment: Regex,
    bracket: Regex,
}

impl PublicationPatterns {
    fn new() -> Self {
        Self {
            function: Regex::new(
                r"^\s*function\s+([A-Za-z_][A-Za-z0-9_]*(?:[.:][A-Za-z_][A-Za-z0-9_]*)*)\s*\(",
            )
            .expect("function definition pattern should compile"),
            assignment: Regex::new(
                r"^\s*([A-Za-z_][A-Za-z0-9_]*(?:\.[A-Za-z_][A-Za-z0-9_]*)*)\s*=\s*function\s*\(",
            )
            .expect("function assignment pattern should compile"),
            bracket: Regex::new(
                r#"^\s*([A-Za-z_][A-Za-z0-9_]*)\s*\[\s*[\"']([^\"']+)[\"']\s*\]\s*=\s*function\s*\("#,
            )
            .expect("bracket assignment pattern should compile"),
        }
    }
}

pub fn index_lua_source(addon: &str, file: &str, source: &str) -> Vec<SourceIndexRecord> {
    let patterns = PublicationPatterns::new();
    source
        .lines()
        .enumerate()
        .filter_map(|(index, line)| {
            let symbol = direct_publication_symbol(line, &patterns)?;
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

fn direct_publication_symbol(line: &str, patterns: &PublicationPatterns) -> Option<String> {
    let trimmed = line.trim_start();
    if trimmed.starts_with("--") || trimmed.starts_with("local ") {
        return None;
    }
    patterns
        .function
        .captures(line)
        .or_else(|| patterns.assignment.captures(line))
        .map(|captures| captures[1].replace(':', "."))
        .or_else(|| bracket_publication_symbol(line, &patterns.bracket))
}

fn bracket_publication_symbol(line: &str, pattern: &Regex) -> Option<String> {
    let captures = pattern.captures(line)?;
    let table = &captures[1];
    let field = &captures[2];
    if table == "_G" {
        Some(field.to_string())
    } else {
        Some(format!("{table}.{field}"))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn indexes_a_lua_file_with_source_identity() {
        let directory = tempfile::tempdir().expect("temporary directory should create");
        let path = directory.path().join("Fixture.lua");
        std::fs::write(&path, "function FileGlobal() end\nMixin(Target, Source)\n")
            .expect("fixture should write");

        let index = index_lua_file("Blizzard_Fixture", &path).expect("source file should index");

        assert_eq!(index.addon, "Blizzard_Fixture");
        assert_eq!(index.file, path.display().to_string());
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
