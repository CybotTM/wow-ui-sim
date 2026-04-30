//! TOC file parser for WoW addons.
//!
//! Parses `.toc` files to extract addon metadata and file load order.

use crate::paths::find_case_insensitive;
use crate::screen::ScreenKind;
use std::collections::HashMap;
use std::path::{Path, PathBuf};

/// Parsed TOC file contents.
#[derive(Debug, Clone)]
pub struct TocFile {
    /// Addon directory path
    pub addon_dir: PathBuf,
    /// Addon name (from directory or Title metadata)
    pub name: String,
    /// Metadata key-value pairs (## Key: Value)
    pub metadata: HashMap<String, String>,
    /// Files to load in order (relative paths)
    pub files: Vec<PathBuf>,
    /// Per-file environment override from `[LoadIntoEnvironment ...]` annotations.
    /// `None` means inherit the addon's default environment.
    pub file_env_overrides: Vec<Option<bool>>,
}

/// Strip inline annotations like `[AllowLoadEnvironment Global]` from a TOC line.
fn strip_annotations(line: &str) -> &str {
    if let Some(pos) = line.find(" [") {
        line[..pos].trim()
    } else if line.ends_with(']') {
        if let Some(pos) = line.find('[') {
            line[..pos].trim()
        } else {
            line.trim()
        }
    } else {
        line.trim()
    }
}

/// Check if an inline `[AllowLoadGameType ...]` annotation includes a game type
/// compatible with mainline retail WoW (standard mode).
fn is_allowed_game_type(line: &str) -> bool {
    let Some(start) = line.find("[AllowLoadGameType") else {
        return true;
    };
    let rest = &line[start + "[AllowLoadGameType".len()..];
    let Some(end) = rest.find(']') else {
        return true;
    };
    let types = &rest[..end];
    types
        .split(',')
        .any(|t| matches!(t.trim(), "mainline" | "standard"))
}

/// Resolve addon name from Title metadata or directory name.
fn resolve_addon_name(metadata: &HashMap<String, String>, addon_dir: &Path) -> String {
    metadata.get("Title").cloned().unwrap_or_else(|| {
        addon_dir
            .file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("Unknown")
            .to_string()
    })
}

/// Check if a metadata value consists entirely of unresolved packager template tokens.
///
/// Returns true when every comma-separated token looks like `@something@`.
/// Example: `@toc-version-retail@, @toc-version-cata@` → true.
/// Mixed: `@toc-version-retail@, 110000` → false.
fn is_all_template_versions(value: &str) -> bool {
    if !value.contains("@toc-version-") {
        return false;
    }
    value
        .split(',')
        .map(|v| v.trim())
        .all(|v| v.starts_with('@') && v.ends_with('@'))
}

/// Replace packager template variables in a metadata value.
/// - `@project-version@` → `dev`
fn replace_template_vars(value: &str) -> String {
    value.replace("@project-version@", "dev")
}

/// Process a `## Key: Value` metadata line into the map.
///
/// Skips `Interface` lines whose value consists entirely of unresolved
/// `@toc-version-*@` packager tokens — the `#@debug@` block in the TOC
/// provides the real fallback version for source-form TOC files.
fn insert_metadata(metadata: &mut HashMap<String, String>, rest: &str) {
    let Some((key, value)) = rest.split_once(':') else {
        return;
    };
    let key = key.trim();
    let value = value.trim();
    if key == "Interface" && is_all_template_versions(value) {
        return;
    }
    metadata.insert(key.to_string(), replace_template_vars(value));
}

fn parse_load_into_environment(line: &str) -> Option<bool> {
    let lower = line.to_ascii_lowercase();
    if lower.contains("[loadintoenvironment secure]") {
        Some(true)
    } else if lower.contains("[loadintoenvironment global]") {
        Some(false)
    } else {
        None
    }
}

fn split_metadata_list(value: &str) -> Vec<String> {
    if value.contains(',') {
        value
            .split(',')
            .map(str::trim)
            .filter(|item| !item.is_empty())
            .map(ToString::to_string)
            .collect()
    } else {
        value.split_whitespace().map(ToString::to_string).collect()
    }
}

/// Process a non-metadata, non-comment TOC line as a file path entry.
fn push_file_entry(
    files: &mut Vec<PathBuf>,
    file_env_overrides: &mut Vec<Option<bool>>,
    line: &str,
) {
    if line.contains("[AllowLoadTextLocale") && !line.contains("enUS") {
        return;
    }
    if line.contains("[AllowLoadGameType") && !is_allowed_game_type(line) {
        return;
    }
    let line = line.replace("[TextLocale]", "enUS");
    let line = line.replace("[Family]", "Mainline");
    let line = line.replace("[Game]", "Standard");
    let file_path = strip_annotations(&line).replace('\\', "/");
    if !file_path.is_empty() {
        files.push(PathBuf::from(file_path));
        file_env_overrides.push(parse_load_into_environment(&line));
    }
}

impl TocFile {
    /// Parse a TOC file from its contents.
    ///
    /// Handles CurseForge/BigWigs packager template tags in source form:
    /// - `#@debug@` / `#@end-debug@` block markers: skipped as `#` comments;
    ///   inner lines like `## Interface: 120000` are active.
    /// - `## Interface: @toc-version-*@, ...` with only template tokens: skipped
    ///   so the `#@debug@` block entry takes precedence.
    /// - `@project-version@` in any value: replaced with `dev`.
    pub fn parse(addon_dir: &Path, contents: &str) -> Self {
        let mut metadata = HashMap::new();
        let mut files = Vec::new();
        let mut file_env_overrides = Vec::new();

        for line in contents.lines() {
            let line = line.trim();
            if line.is_empty() {
                continue;
            }
            if let Some(rest) = line.strip_prefix("##") {
                insert_metadata(&mut metadata, rest.trim());
                continue;
            }
            if line.starts_with('#') {
                continue;
            }
            push_file_entry(&mut files, &mut file_env_overrides, line);
        }

        TocFile {
            addon_dir: addon_dir.to_path_buf(),
            name: resolve_addon_name(&metadata, addon_dir),
            metadata,
            files,
            file_env_overrides,
        }
    }

    /// Parse a TOC file from disk.
    pub fn from_file(toc_path: &Path) -> std::io::Result<Self> {
        let contents = std::fs::read_to_string(toc_path)?;
        let addon_dir = toc_path.parent().unwrap_or(Path::new("."));
        Ok(Self::parse(addon_dir, &contents))
    }

    /// Get interface version(s) from metadata.
    pub fn interface_versions(&self) -> Vec<u32> {
        self.metadata
            .get("Interface")
            .map(|s| s.split(',').filter_map(|v| v.trim().parse().ok()).collect())
            .unwrap_or_default()
    }

    /// Get required dependencies.
    ///
    /// WoW TOC files use three variant keys: `RequiredDep`, `RequiredDeps`, `Dependencies`.
    pub fn dependencies(&self) -> Vec<String> {
        self.metadata
            .get("RequiredDep")
            .or_else(|| self.metadata.get("Dependencies"))
            .or_else(|| self.metadata.get("RequiredDeps"))
            .map(|s| split_metadata_list(s))
            .unwrap_or_default()
    }

    /// Get `LoadWith` triggers — addon names that, when loaded, should trigger
    /// loading this addon immediately inline.
    pub fn load_with(&self) -> Vec<String> {
        self.metadata
            .get("LoadWith")
            .map(|s| split_metadata_list(s))
            .unwrap_or_default()
    }

    /// Get optional dependencies.
    pub fn optional_deps(&self) -> Vec<String> {
        self.metadata
            .get("OptionalDeps")
            .map(|s| split_metadata_list(s))
            .unwrap_or_default()
    }

    /// Check if addon uses the secure Lua environment (UseSecureEnvironment: 1).
    pub fn is_secure_env(&self) -> bool {
        self.metadata
            .get("UseSecureEnvironment")
            .map(|v| v == "1")
            .unwrap_or(false)
    }

    /// Get the per-file environment override for a TOC entry.
    pub fn file_use_secure_env(&self, index: usize) -> Option<bool> {
        self.file_env_overrides.get(index).copied().flatten()
    }

    /// Default enabled state — `## DefaultState: disabled` ships the addon
    /// disabled out of the box; any other value (or absence) ships it enabled.
    pub fn default_enabled(&self) -> bool {
        self.metadata
            .get("DefaultState")
            .map(|v| !v.eq_ignore_ascii_case("disabled"))
            .unwrap_or(true)
    }

    /// Check if addon is load-on-demand.
    pub fn is_load_on_demand(&self) -> bool {
        self.metadata
            .get("LoadOnDemand")
            .map(|v| v == "1" || v.eq_ignore_ascii_case("true"))
            .unwrap_or(false)
    }

    /// Check if addon requests early loading via `## LoadFirst: 1`.
    pub fn is_load_first(&self) -> bool {
        self.metadata
            .get("LoadFirst")
            .map(|v| v == "1" || v.eq_ignore_ascii_case("true"))
            .unwrap_or(false)
    }

    /// Check if addon is glue-only (login/character-select screen).
    /// These addons have `AllowLoad: Glue` and should not load in game mode.
    pub fn is_glue_only(&self) -> bool {
        self.metadata
            .get("AllowLoad")
            .map(|v| v.eq_ignore_ascii_case("glue"))
            .unwrap_or(false)
    }

    /// Check if addon is PTR/Beta-only (e.g. Blizzard_PTRFeedback).
    /// These addons have `OnlyBetaAndPTR: 1` and should not load on live clients.
    pub fn is_ptr_only(&self) -> bool {
        self.metadata
            .get("OnlyBetaAndPTR")
            .map(|v| v == "1")
            .unwrap_or(false)
    }

    /// Check if addon is restricted to a game type incompatible with the active client profile.
    /// Tocs with `AllowLoadGameType: <type>` only load when that type matches the active profile.
    pub fn is_game_type_restricted(&self) -> bool {
        let allowed: &[&str] = match crate::client_profile::ACTIVE {
            crate::client_profile::ClientProfile::Retail => &["mainline", "standard"],
            crate::client_profile::ClientProfile::Wrath => &["wrath", "wrath_classic", "classic"],
            crate::client_profile::ClientProfile::Mists => &["mists", "mists_classic", "classic"],
        };
        self.metadata
            .get("AllowLoadGameType")
            .map(|v| {
                !v.split(',')
                    .any(|t| allowed.contains(&t.trim()))
            })
            .unwrap_or(false)
    }

    /// Whether this addon should load for the requested screen kind.
    pub fn allows_screen(&self, screen: ScreenKind) -> bool {
        match self.metadata.get("AllowLoad").map(|v| v.trim()) {
            Some(v) if v.eq_ignore_ascii_case("both") => true,
            Some(v) if v.eq_ignore_ascii_case("game") => screen == ScreenKind::Game,
            Some(v) if v.eq_ignore_ascii_case("glue") => screen.is_glue(),
            Some(_) => screen == ScreenKind::Game,
            None => screen == ScreenKind::Game,
        }
    }

    /// Get saved variables names (account-wide + machine-specific).
    pub fn saved_variables(&self) -> Vec<String> {
        let mut vars: Vec<String> = Vec::new();
        for key in ["SavedVariables", "SavedVariablesMachine"] {
            if let Some(s) = self.metadata.get(key) {
                vars.extend(
                    s.split(',')
                        .map(|v| v.trim().to_string())
                        .filter(|v| !v.is_empty()),
                );
            }
        }
        vars
    }

    /// Get saved variables per character names.
    pub fn saved_variables_per_character(&self) -> Vec<String> {
        self.metadata
            .get("SavedVariablesPerCharacter")
            .map(|s| {
                s.split(',')
                    .map(|v| v.trim().to_string())
                    .filter(|v| !v.is_empty())
                    .collect()
            })
            .unwrap_or_default()
    }

    /// Get absolute paths for all files to load.
    /// Uses case-insensitive matching for compatibility with WoW (Windows/macOS).
    pub fn file_paths(&self) -> Vec<PathBuf> {
        self.files
            .iter()
            .map(|f| resolve_path_case_insensitive(&self.addon_dir, f))
            .collect()
    }
}

/// Resolve a path with case-insensitive matching (WoW is case-insensitive on Windows/macOS).
fn resolve_path_case_insensitive(base: &Path, path: &Path) -> PathBuf {
    let path_str = path.to_string_lossy().replace('\\', "/");
    let components: Vec<&str> = path_str.split('/').collect();
    let mut current = base.to_path_buf();

    for component in &components {
        if component.is_empty() {
            continue;
        }
        // Try exact match first
        let exact = current.join(component);
        if exact.exists() {
            current = exact;
        } else if let Some(entry) = find_case_insensitive(&current, component) {
            current = entry;
        } else {
            // Fall back to exact path (will fail later with proper error)
            current = exact;
        }
    }
    current
}

impl TocFile {
    /// Check if this is a Blizzard addon (AllowLoad metadata present).
    pub fn is_blizzard_addon(&self) -> bool {
        self.metadata.contains_key("AllowLoad")
    }

    /// Check if this TOC should execute without addon taint.
    ///
    /// Most Blizzard UI TOCs advertise `AllowLoad`; a few secure helper TOCs
    /// only advertise `UseSecureEnvironment`; other internal Blizzard addons
    /// rely on the signed `Blizzard_` folder-name convention that also drives
    /// `C_AddOns.GetAddOnSecurity`.
    pub fn loads_as_blizzard_code(&self) -> bool {
        self.is_blizzard_addon() || self.is_secure_env() || self.folder_name_starts_with_blizzard()
    }

    fn folder_name_starts_with_blizzard(&self) -> bool {
        self.addon_dir
            .file_name()
            .and_then(|name| name.to_str())
            .is_some_and(|name| name.starts_with("Blizzard_"))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_simple_toc() {
        let contents = r#"
## Title: MyAddon
## Interface: 110000
## Dependencies: Ace3, LibStub

Core.lua
UI/Main.lua
UI/Options.xml
"#;
        let toc = TocFile::parse(Path::new("/addons/MyAddon"), contents);

        assert_eq!(toc.name, "MyAddon");
        assert_eq!(toc.interface_versions(), vec![110000]);
        assert_eq!(toc.dependencies(), vec!["Ace3", "LibStub"]);
        assert_eq!(toc.files.len(), 3);
        assert_eq!(toc.files[0], PathBuf::from("Core.lua"));
        assert_eq!(toc.files[1], PathBuf::from("UI/Main.lua"));
        assert_eq!(toc.files[2], PathBuf::from("UI/Options.xml"));
    }

    #[test]
    fn test_parse_space_separated_dependencies() {
        let contents = r#"
## Title: Blizzard_BattlefieldMap
## Dependencies: Blizzard_MapCanvas Blizzard_SharedMapDataProviders Blizzard_ObjectiveTracker
BattlefieldMap.lua
"#;
        let toc = TocFile::parse(Path::new("/addons/Blizzard_BattlefieldMap"), contents);

        assert_eq!(
            toc.dependencies(),
            vec![
                "Blizzard_MapCanvas",
                "Blizzard_SharedMapDataProviders",
                "Blizzard_ObjectiveTracker",
            ]
        );
    }

    #[test]
    fn test_parse_blizzard_toc() {
        let contents = r#"
## Title: Blizzard_SharedXMLBase
## AllowLoad: Both
Compat.lua
Mixin.lua
TableUtil.lua
"#;
        let toc = TocFile::parse(
            Path::new("/Interface/AddOns/Blizzard_SharedXMLBase"),
            contents,
        );

        assert_eq!(toc.name, "Blizzard_SharedXMLBase");
        assert!(toc.is_blizzard_addon());
        assert_eq!(toc.files.len(), 3);
    }

    #[test]
    fn test_parse_with_comments() {
        let contents = r#"
## Title: TestAddon
# This is a comment
#@no-lib-strip@
Libs/LibStub.lua
#@end-no-lib-strip@
Core.lua
"#;
        let toc = TocFile::parse(Path::new("/addons/TestAddon"), contents);

        // Comments and directives should be skipped
        assert_eq!(toc.files.len(), 2);
        assert_eq!(toc.files[0], PathBuf::from("Libs/LibStub.lua"));
        assert_eq!(toc.files[1], PathBuf::from("Core.lua"));
    }

    #[test]
    fn test_parse_backslash_paths() {
        let contents = r#"
## Title: TestAddon
Libs\LibStub\LibStub.lua
Core\Init.lua
"#;
        let toc = TocFile::parse(Path::new("/addons/TestAddon"), contents);

        // Backslashes should be normalized to forward slashes
        assert_eq!(toc.files[0], PathBuf::from("Libs/LibStub/LibStub.lua"));
        assert_eq!(toc.files[1], PathBuf::from("Core/Init.lua"));
    }

    #[test]
    fn test_optional_deps() {
        let contents = r#"
## Title: TestAddon
## OptionalDeps: Ace3, LibDBIcon-1.0, LibSharedMedia-3.0
Core.lua
"#;
        let toc = TocFile::parse(Path::new("/addons/TestAddon"), contents);

        assert_eq!(
            toc.optional_deps(),
            vec!["Ace3", "LibDBIcon-1.0", "LibSharedMedia-3.0"]
        );
    }

    #[test]
    fn test_load_first_metadata() {
        let contents = r#"
## Title: TestAddon
## LoadFirst: 1
Core.lua
"#;
        let toc = TocFile::parse(Path::new("/addons/TestAddon"), contents);

        assert!(toc.is_load_first());
    }

    #[test]
    fn test_saved_variables() {
        let contents = r#"
## Title: TestAddon
## SavedVariables: TestAddonDB, TestAddonPerCharDB
Core.lua
"#;
        let toc = TocFile::parse(Path::new("/addons/TestAddon"), contents);

        assert_eq!(
            toc.saved_variables(),
            vec!["TestAddonDB", "TestAddonPerCharDB"]
        );
    }

    #[test]
    fn test_multiple_interface_versions() {
        let contents = r#"
## Title: TestAddon
## Interface: 110107, 50500, 11507
Core.lua
"#;
        let toc = TocFile::parse(Path::new("/addons/TestAddon"), contents);

        assert_eq!(toc.interface_versions(), vec![110107, 50500, 11507]);
    }

    /// Wrath profile vendors (andrew6180/WoTLK-3.3.5-UI-Source) write
    /// `## Interface: 30300`. Parser must accept the single legacy value.
    #[test]
    fn test_wrath_interface_version() {
        let contents = r#"
## Title: WrathAddon
## Interface: 30300
WrathCore.lua
"#;
        let toc = TocFile::parse(Path::new("/addons/WrathAddon"), contents);

        assert_eq!(toc.interface_versions(), vec![30300]);
        assert!(!toc.is_game_type_restricted());
    }

    /// Mists profile vendors write `## Interface: 50500` for MoP-Classic
    /// addons. Parser must accept the single value (the existing
    /// `test_multiple_interface_versions` covers 50500 only as a list element).
    #[test]
    fn test_mists_interface_version() {
        let contents = r#"
## Title: MistsAddon
## Interface: 50500
MistsCore.lua
"#;
        let toc = TocFile::parse(Path::new("/addons/MistsAddon"), contents);

        assert_eq!(toc.interface_versions(), vec![50500]);
        assert!(!toc.is_game_type_restricted());
    }

    #[test]
    fn test_parse_inline_annotations() {
        let contents = r#"
## Title: TestAddon
Core.lua
Dump.lua [AllowLoadEnvironment Global]
Debug.lua [AllowLoadEnvironment Global, SomeFlag]
"#;
        let toc = TocFile::parse(Path::new("/addons/TestAddon"), contents);

        // Annotations should be stripped, only filenames kept
        assert_eq!(toc.files.len(), 3);
        assert_eq!(toc.files[0], PathBuf::from("Core.lua"));
        assert_eq!(toc.files[1], PathBuf::from("Dump.lua"));
        assert_eq!(toc.files[2], PathBuf::from("Debug.lua"));
        assert_eq!(toc.file_use_secure_env(0), None);
        assert_eq!(toc.file_use_secure_env(1), None);
        assert_eq!(toc.file_use_secure_env(2), None);
    }

    #[test]
    fn test_parse_load_into_environment_annotations() {
        let contents = r#"
## Title: TestAddon
Core.lua
Restricted.lua [LoadIntoEnvironment secure]
Public.lua [LoadIntoEnvironment global]
"#;
        let toc = TocFile::parse(Path::new("/addons/TestAddon"), contents);

        assert_eq!(toc.files.len(), 3);
        assert_eq!(toc.files[0], PathBuf::from("Core.lua"));
        assert_eq!(toc.files[1], PathBuf::from("Restricted.lua"));
        assert_eq!(toc.files[2], PathBuf::from("Public.lua"));
        assert_eq!(toc.file_use_secure_env(0), None);
        assert_eq!(toc.file_use_secure_env(1), Some(true));
        assert_eq!(toc.file_use_secure_env(2), Some(false));
    }

    #[test]
    fn test_family_placeholder_resolves_to_mainline() {
        let contents = r#"
## Title: Blizzard_Colors
Shared\ColorOverrides.lua
[Family]\ColorConstants.lua
[Family]\ColorManager.lua
"#;
        let toc = TocFile::parse(Path::new("/addons/Blizzard_Colors"), contents);

        assert_eq!(toc.files.len(), 3);
        assert_eq!(toc.files[0], PathBuf::from("Shared/ColorOverrides.lua"));
        assert_eq!(toc.files[1], PathBuf::from("Mainline/ColorConstants.lua"));
        assert_eq!(toc.files[2], PathBuf::from("Mainline/ColorManager.lua"));
    }

    #[test]
    fn test_game_type_filter_skips_plunderstorm() {
        let contents = r#"
## Title: Blizzard_FrameXMLBase
Constants.lua
[Game]\GameModeConstants.lua [AllowLoadGameType plunderstorm]
"#;
        let toc = TocFile::parse(Path::new("/addons/Blizzard_FrameXMLBase"), contents);

        assert_eq!(toc.files.len(), 1);
        assert_eq!(toc.files[0], PathBuf::from("Constants.lua"));
    }

    #[test]
    fn test_game_type_filter_allows_mainline_and_standard() {
        let contents = r#"
## Title: TestAddon
Core.lua
Mainline\Override.lua [AllowLoadGameType mainline]
Standard\Mode.lua [AllowLoadGameType standard]
Standard\Multi.lua [AllowLoadGameType standard, wowhack, plunderstorm]
WoWLabs\Mode.lua [AllowLoadGameType plunderstorm]
Classic\Mode.lua [AllowLoadGameType classic]
Cata\Mode.lua [AllowLoadGameType wrath, cata, mists]
"#;
        let toc = TocFile::parse(Path::new("/addons/TestAddon"), contents);

        assert_eq!(toc.files.len(), 4);
        assert_eq!(toc.files[0], PathBuf::from("Core.lua"));
        assert_eq!(toc.files[1], PathBuf::from("Mainline/Override.lua"));
        assert_eq!(toc.files[2], PathBuf::from("Standard/Mode.lua"));
        assert_eq!(toc.files[3], PathBuf::from("Standard/Multi.lua"));
    }

    #[test]
    fn test_is_allowed_game_type() {
        assert!(is_allowed_game_type("Core.lua"));
        assert!(is_allowed_game_type(
            "File.lua [AllowLoadGameType mainline]"
        ));
        assert!(is_allowed_game_type(
            "File.lua [AllowLoadGameType standard]"
        ));
        assert!(is_allowed_game_type(
            "File.lua [AllowLoadGameType standard, wowhack]"
        ));
        assert!(!is_allowed_game_type(
            "File.lua [AllowLoadGameType plunderstorm]"
        ));
        assert!(!is_allowed_game_type(
            "File.lua [AllowLoadGameType classic]"
        ));
        assert!(!is_allowed_game_type(
            "File.lua [AllowLoadGameType wrath, cata, mists]"
        ));
    }

    #[test]
    fn test_is_game_type_restricted() {
        let plunderstorm = TocFile::parse(
            Path::new("/addons/Test"),
            "## AllowLoadGameType: plunderstorm\nCore.lua",
        );
        assert!(plunderstorm.is_game_type_restricted());

        let mainline = TocFile::parse(
            Path::new("/addons/Test"),
            "## AllowLoadGameType: mainline\nCore.lua",
        );
        assert!(!mainline.is_game_type_restricted());

        let standard = TocFile::parse(
            Path::new("/addons/Test"),
            "## AllowLoadGameType: standard\nCore.lua",
        );
        assert!(!standard.is_game_type_restricted());

        let mixed = TocFile::parse(
            Path::new("/addons/Test"),
            "## AllowLoadGameType: plunderstorm, wowhack\nCore.lua",
        );
        assert!(mixed.is_game_type_restricted());

        let no_restriction =
            TocFile::parse(Path::new("/addons/Test"), "## Title: TestAddon\nCore.lua");
        assert!(!no_restriction.is_game_type_restricted());
    }

    #[test]
    fn test_packager_debug_block_interface_version() {
        // BlizzMove-style TOC: template Interface line skipped, debug block wins
        let contents = r#"
## Interface: @toc-version-midnight@, @toc-version-retail@, @toc-version-classic@
#@debug@
## Interface: 120000
#@end-debug@
## Title: BlizzMove
## Version: @project-version@
Core.lua
"#;
        let toc = TocFile::parse(Path::new("/addons/BlizzMove"), contents);
        // Template-only Interface line is skipped; debug block provides version
        assert_eq!(toc.interface_versions(), vec![120000]);
        // @project-version@ replaced with "dev"
        assert_eq!(toc.metadata.get("Version").map(|s| s.as_str()), Some("dev"));
        assert_eq!(toc.files.len(), 1);
    }

    #[test]
    fn test_packager_mixed_interface_version_kept() {
        // If Interface has at least one plain number alongside templates, keep it
        let contents = r#"
## Interface: @toc-version-retail@, 110000
Core.lua
"#;
        let toc = TocFile::parse(Path::new("/addons/TestAddon"), contents);
        // Mixed value retained; non-numeric tokens dropped by interface_versions()
        assert_eq!(toc.interface_versions(), vec![110000]);
    }

    #[test]
    fn test_is_all_template_versions() {
        assert!(is_all_template_versions(
            "@toc-version-retail@, @toc-version-cata@"
        ));
        assert!(is_all_template_versions("@toc-version-retail@"));
        assert!(!is_all_template_versions("110000"));
        assert!(!is_all_template_versions("@toc-version-retail@, 110000"));
        assert!(!is_all_template_versions(""));
        assert!(!is_all_template_versions("@project-version@"));
    }

    #[test]
    fn test_allows_screen_modes() {
        use crate::screen::ScreenKind;

        let both = TocFile::parse(Path::new("/addons/Both"), "## AllowLoad: Both\nCore.lua");
        assert!(both.allows_screen(ScreenKind::Game));
        assert!(both.allows_screen(ScreenKind::Login));
        assert!(both.allows_screen(ScreenKind::CharacterSelect));

        let game = TocFile::parse(Path::new("/addons/Game"), "## AllowLoad: Game\nCore.lua");
        assert!(game.allows_screen(ScreenKind::Game));
        assert!(!game.allows_screen(ScreenKind::Login));
        assert!(!game.allows_screen(ScreenKind::CharacterSelect));

        let glue = TocFile::parse(Path::new("/addons/Glue"), "## AllowLoad: Glue\nCore.lua");
        assert!(!glue.allows_screen(ScreenKind::Game));
        assert!(glue.allows_screen(ScreenKind::Login));
        assert!(glue.allows_screen(ScreenKind::CharacterSelect));

        let unrestricted = TocFile::parse(
            Path::new("/addons/Unrestricted"),
            "## Title: TestAddon\nCore.lua",
        );
        assert!(unrestricted.allows_screen(ScreenKind::Game));
        assert!(!unrestricted.allows_screen(ScreenKind::Login));
        assert!(!unrestricted.allows_screen(ScreenKind::CharacterSelect));
    }
}
