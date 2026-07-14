use regex::Regex;
use serde::Deserialize;
use sha2::{Digest, Sha256};
use std::collections::{BTreeMap, HashSet};
use std::path::Path;
use std::process::{Command, Stdio};

const PATCH_MANIFEST_SCHEMA: &str = "framexml-patch-audit/v2";

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PatchAuditManifest {
    pub schema: String,
    pub patch: String,
    pub target: AuditTarget,
    pub source: PatchListSource,
    pub output: PatchAuditOutput,
    pub rows: Vec<PatchAuditRow>,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AuditTarget {
    pub flavor: AuditFlavor,
    pub build: String,
    pub cache_manifest: String,
    pub cache_hash: String,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PatchListSource {
    pub path: String,
    pub hash: String,
    pub added_count: usize,
    pub removed_count: usize,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PatchAuditOutput {
    pub checklist: String,
    pub inventory: String,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PatchAuditRow {
    pub id: String,
    pub symbol: String,
    pub change: ChangeDirection,
    pub status: Option<AuditStatus>,
    pub resolution: ResolutionKind,
    pub owner: String,
    pub load_addon: Option<String>,
    pub evidence: Vec<AuditEvidence>,
    pub tests: Vec<String>,
    pub assertions: Vec<AuditAssertion>,
    pub commit: Option<String>,
    pub approval_id: Option<String>,
    pub notes: Option<String>,
}

#[derive(Clone, Copy, Debug, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum ChangeDirection {
    Added,
    Removed,
}

impl ChangeDirection {
    fn as_str(self) -> &'static str {
        match self {
            Self::Added => "added",
            Self::Removed => "removed",
        }
    }
}

#[derive(Clone, Copy, Debug, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "kebab-case")]
pub enum AuditStatus {
    Implemented,
    BestEffort,
    ExceptionRequested,
}

impl AuditStatus {
    fn as_str(self) -> &'static str {
        match self {
            Self::Implemented => "implemented",
            Self::BestEffort => "best-effort",
            Self::ExceptionRequested => "exception-requested",
        }
    }
}

#[derive(Clone, Copy, Debug, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "kebab-case")]
pub enum ResolutionKind {
    Untriaged,
    VendorPresent,
    Compat,
    LoadOnDemand,
    Removed,
    CrossFlavor,
    StaleSnapshot,
    ReversedSnapshot,
    Unsafe,
    Impossible,
}

impl ResolutionKind {
    fn as_str(self) -> &'static str {
        match self {
            Self::Untriaged => "untriaged",
            Self::VendorPresent => "vendor-present",
            Self::Compat => "compat",
            Self::LoadOnDemand => "load-on-demand",
            Self::Removed => "removed",
            Self::CrossFlavor => "cross-flavor",
            Self::StaleSnapshot => "stale-snapshot",
            Self::ReversedSnapshot => "reversed-snapshot",
            Self::Unsafe => "unsafe",
            Self::Impossible => "impossible",
        }
    }
}

#[derive(Clone, Copy, Debug, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "kebab-case")]
pub enum AuditFlavor {
    Retail,
    Ptr,
    Wrath,
    Mists,
    Era,
    Anniversary,
}

impl AuditFlavor {
    fn as_str(self) -> &'static str {
        match self {
            Self::Retail => "retail",
            Self::Ptr => "ptr",
            Self::Wrath => "wrath",
            Self::Mists => "mists",
            Self::Era => "era",
            Self::Anniversary => "anniversary",
        }
    }
}

#[derive(Clone, Copy, Debug, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "kebab-case")]
pub enum AuditPhase {
    Initialization,
    PostCore,
    PostLoad,
    BeforeAddon,
    AfterAddon,
    PostReset,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AuditEvidence {
    pub kind: EvidenceKind,
    pub reference: String,
    pub summary: String,
    pub source_hash: Option<String>,
}

#[derive(Clone, Copy, Debug, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum EvidenceKind {
    Source,
    Runtime,
    Test,
    Manual,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AuditAssertion {
    pub flavor: AuditFlavor,
    pub phase: AuditPhase,
    pub expected: ExpectedPresence,
    pub expected_type: Option<LuaType>,
    pub addon: Option<String>,
}

#[derive(Clone, Copy, Debug, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum ExpectedPresence {
    Present,
    Absent,
}

#[derive(Clone, Copy, Debug, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum LuaType {
    Function,
    Table,
    String,
    Number,
    Boolean,
    Userdata,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ObservationSet {
    pub schema: String,
    pub manifest_hash: String,
    pub observations: Vec<Observation>,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Observation {
    pub row_id: String,
    pub flavor: AuditFlavor,
    pub phase: AuditPhase,
    pub present: bool,
    pub observed_type: Option<LuaType>,
    pub addon: Option<String>,
}

pub fn parse_manifest(json: &str) -> Result<PatchAuditManifest, String> {
    serde_json::from_str(json).map_err(|error| format!("invalid patch audit manifest: {error}"))
}

pub fn parse_observations(json: &str) -> Result<ObservationSet, String> {
    serde_json::from_str(json).map_err(|error| format!("invalid observation set: {error}"))
}

pub fn validate_manifest(manifest: &PatchAuditManifest) -> Result<(), String> {
    validate_manifest_metadata(manifest)?;
    let actual_counts = validate_manifest_rows(&manifest.rows, manifest.target.flavor)?;
    validate_direction_count("added", manifest.source.added_count, actual_counts["added"])?;
    validate_direction_count(
        "removed",
        manifest.source.removed_count,
        actual_counts["removed"],
    )
}

fn validate_manifest_metadata(manifest: &PatchAuditManifest) -> Result<(), String> {
    if manifest.schema != PATCH_MANIFEST_SCHEMA {
        return Err(format!("unsupported manifest schema: {}", manifest.schema));
    }
    require_text("patch", &manifest.patch)?;
    require_text("target.build", &manifest.target.build)?;
    require_text("target.cache_manifest", &manifest.target.cache_manifest)?;
    require_sha256("target.cache_hash", &manifest.target.cache_hash)?;
    require_text("source.path", &manifest.source.path)?;
    require_sha256("source.hash", &manifest.source.hash)?;
    require_text("output.checklist", &manifest.output.checklist)?;
    require_text("output.inventory", &manifest.output.inventory)
}

fn validate_manifest_rows(
    rows: &[PatchAuditRow],
    target_flavor: AuditFlavor,
) -> Result<BTreeMap<&'static str, usize>, String> {
    let mut row_ids = HashSet::new();
    let mut counts = BTreeMap::from([("added", 0usize), ("removed", 0usize)]);
    for row in rows {
        require_text("row.symbol", &row.symbol)?;
        let expected_id = format!("{}:{}", row.change.as_str(), row.symbol);
        if row.id != expected_id {
            return Err(format!("row {} must use id {expected_id}", row.id));
        }
        if !row_ids.insert(row.id.as_str()) {
            return Err(format!("duplicate row id: {}", row.id));
        }
        *counts
            .get_mut(row.change.as_str())
            .expect("known direction") += 1;
        require_text(&format!("{}.owner", row.id), &row.owner)?;
        validate_row(row, target_flavor)?;
    }
    Ok(counts)
}

fn validate_direction_count(direction: &str, expected: usize, actual: usize) -> Result<(), String> {
    if actual != expected {
        return Err(format!(
            "{direction} count mismatch: source={expected} rows={actual}"
        ));
    }
    Ok(())
}

fn validate_row(row: &PatchAuditRow, target_flavor: AuditFlavor) -> Result<(), String> {
    if row.resolution == ResolutionKind::Untriaged {
        return validate_untriaged_row(row);
    }
    validate_resolved_row(row, target_flavor)
}

fn validate_untriaged_row(row: &PatchAuditRow) -> Result<(), String> {
    if row.status.is_some() {
        return Err(format!("row {} untriaged status must be null", row.id));
    }
    if let Some(notes) = &row.notes {
        require_text(&format!("{}.notes", row.id), notes)?;
    }
    let has_resolution_data = !row.evidence.is_empty()
        || !row.tests.is_empty()
        || !row.assertions.is_empty()
        || row.commit.is_some()
        || row.approval_id.is_some()
        || row.load_addon.is_some();
    if has_resolution_data {
        return Err(format!("row {} untriaged fields must remain empty", row.id));
    }
    Ok(())
}

fn validate_resolved_row(row: &PatchAuditRow, target_flavor: AuditFlavor) -> Result<(), String> {
    let status = row
        .status
        .ok_or_else(|| format!("row {} resolved status must be set", row.id))?;
    validate_status_resolution(row, status)?;
    validate_load_addon_field(row)?;
    validate_evidence(row)?;
    validate_assertions(row)?;
    validate_resolution_contract(row, target_flavor)?;
    validate_focused_tests(row, status)?;
    validate_optional_metadata(row)
}

fn validate_load_addon_field(row: &PatchAuditRow) -> Result<(), String> {
    match (row.resolution, row.load_addon.as_deref()) {
        (ResolutionKind::LoadOnDemand, Some(addon)) => {
            require_text(&format!("{}.load_addon", row.id), addon)
        }
        (ResolutionKind::LoadOnDemand, None) => {
            Err(format!("row {} load-on-demand requires load_addon", row.id))
        }
        (_, Some(_)) => Err(format!(
            "row {} load_addon is only valid for load-on-demand resolution",
            row.id
        )),
        (_, None) => Ok(()),
    }
}

fn validate_evidence(row: &PatchAuditRow) -> Result<(), String> {
    if row.evidence.is_empty() {
        return Err(format!("row {} requires evidence", row.id));
    }
    for evidence in &row.evidence {
        require_text(
            &format!("{}.evidence.reference", row.id),
            &evidence.reference,
        )?;
        require_text(&format!("{}.evidence.summary", row.id), &evidence.summary)?;
        let hash = evidence.source_hash.as_deref().unwrap_or_default();
        require_sha256(&format!("{}.evidence.source_hash", row.id), hash)?;
    }
    Ok(())
}

fn validate_assertions(row: &PatchAuditRow) -> Result<(), String> {
    if row.assertions.is_empty() {
        return Err(format!("row {} requires an assertion", row.id));
    }
    for assertion in &row.assertions {
        validate_assertion(row, assertion)?;
    }
    Ok(())
}

fn validate_focused_tests(row: &PatchAuditRow, status: AuditStatus) -> Result<(), String> {
    if status == AuditStatus::ExceptionRequested {
        return Ok(());
    }
    if row.tests.is_empty() {
        return Err(format!("row {} requires a focused test", row.id));
    }
    for test in &row.tests {
        require_text(&format!("{}.tests", row.id), test)?;
    }
    Ok(())
}

fn validate_optional_metadata(row: &PatchAuditRow) -> Result<(), String> {
    for (field, value) in [
        ("commit", row.commit.as_deref()),
        ("approval_id", row.approval_id.as_deref()),
        ("notes", row.notes.as_deref()),
    ] {
        if let Some(value) = value {
            require_text(&format!("{}.{}", row.id, field), value)?;
        }
    }
    Ok(())
}

fn validate_status_resolution(row: &PatchAuditRow, status: AuditStatus) -> Result<(), String> {
    let allowed = match row.resolution {
        ResolutionKind::Untriaged => false,
        ResolutionKind::CrossFlavor
        | ResolutionKind::StaleSnapshot
        | ResolutionKind::ReversedSnapshot => status == AuditStatus::BestEffort,
        ResolutionKind::Unsafe | ResolutionKind::Impossible => {
            status == AuditStatus::ExceptionRequested
        }
        ResolutionKind::VendorPresent
        | ResolutionKind::Compat
        | ResolutionKind::LoadOnDemand
        | ResolutionKind::Removed => {
            matches!(status, AuditStatus::Implemented | AuditStatus::BestEffort)
        }
    };
    if !allowed {
        return Err(format!(
            "row {} status {} is incompatible with resolution {}",
            row.id,
            status.as_str(),
            row.resolution.as_str()
        ));
    }
    Ok(())
}

fn validate_assertion(row: &PatchAuditRow, assertion: &AuditAssertion) -> Result<(), String> {
    match (assertion.expected, assertion.expected_type) {
        (ExpectedPresence::Present, None) => {
            return Err(format!(
                "row {} present assertion requires expected_type",
                row.id
            ));
        }
        (ExpectedPresence::Absent, Some(_)) => {
            return Err(format!(
                "row {} absent assertion must not set expected_type",
                row.id
            ));
        }
        _ => {}
    }
    if let Some(addon) = &assertion.addon {
        require_text(&format!("{}.assertion.addon", row.id), addon)?;
    }
    Ok(())
}

fn validate_resolution_contract(
    row: &PatchAuditRow,
    target_flavor: AuditFlavor,
) -> Result<(), String> {
    match row.resolution {
        ResolutionKind::LoadOnDemand => validate_load_on_demand_contract(row, target_flavor),
        ResolutionKind::Removed => validate_removed_contract(row),
        ResolutionKind::CrossFlavor => validate_cross_flavor_contract(row, target_flavor),
        ResolutionKind::StaleSnapshot | ResolutionKind::ReversedSnapshot => {
            validate_snapshot_contract(row)
        }
        ResolutionKind::VendorPresent | ResolutionKind::Compat => validate_presence_contract(row),
        ResolutionKind::Unsafe | ResolutionKind::Impossible | ResolutionKind::Untriaged => Ok(()),
    }
}

fn validate_removed_contract(row: &PatchAuditRow) -> Result<(), String> {
    require_assertion(
        row,
        |assertion| {
            assertion.phase == AuditPhase::PostReset
                && assertion.expected == ExpectedPresence::Absent
        },
        "removed requires post-reset absence",
    )
}

fn validate_cross_flavor_contract(
    row: &PatchAuditRow,
    target_flavor: AuditFlavor,
) -> Result<(), String> {
    require_assertion(
        row,
        |assertion| {
            assertion.flavor == target_flavor && assertion.expected == ExpectedPresence::Absent
        },
        "cross-flavor requires target absence",
    )
}

fn validate_snapshot_contract(row: &PatchAuditRow) -> Result<(), String> {
    require_assertion(
        row,
        |assertion| assertion.expected == ExpectedPresence::Absent,
        "snapshot mismatch requires absence",
    )
}

fn validate_presence_contract(row: &PatchAuditRow) -> Result<(), String> {
    require_assertion(
        row,
        |assertion| assertion.expected == ExpectedPresence::Present,
        "requires a presence assertion",
    )
}

fn validate_load_on_demand_contract(
    row: &PatchAuditRow,
    target_flavor: AuditFlavor,
) -> Result<(), String> {
    let load_addon = row
        .load_addon
        .as_deref()
        .expect("load_addon field validated before lifecycle contract");
    let assertions_match_addon = row.assertions.iter().all(|assertion| {
        assertion.flavor == target_flavor && assertion.addon.as_deref() == Some(load_addon)
    });
    let has_before_absence = row.assertions.iter().any(|assertion| {
        assertion.phase == AuditPhase::BeforeAddon && assertion.expected == ExpectedPresence::Absent
    });
    let has_after_presence = row.assertions.iter().any(|assertion| {
        assertion.phase == AuditPhase::AfterAddon && assertion.expected == ExpectedPresence::Present
    });
    if assertions_match_addon && has_before_absence && has_after_presence {
        return Ok(());
    }
    Err(format!(
        "row {} load-on-demand requires load_addon-matched target-flavor before-absent and after-present assertions",
        row.id
    ))
}

fn require_assertion(
    row: &PatchAuditRow,
    predicate: impl Fn(&AuditAssertion) -> bool,
    failure: &str,
) -> Result<(), String> {
    if row.assertions.iter().any(predicate) {
        return Ok(());
    }
    Err(format!("row {} {failure}", row.id))
}

fn require_text(field: &str, value: &str) -> Result<(), String> {
    if value.trim().is_empty() {
        return Err(format!("{field} must not be empty"));
    }
    Ok(())
}

fn require_sha256(field: &str, value: &str) -> Result<(), String> {
    if value.len() != 64 || !value.bytes().all(|byte| byte.is_ascii_hexdigit()) {
        return Err(format!("{field} must be a 64-character SHA-256"));
    }
    Ok(())
}

pub fn validate_repository(manifest: &PatchAuditManifest, root: &Path) -> Result<(), String> {
    validate_manifest(manifest)?;
    validate_file_hash(
        root,
        &manifest.target.cache_manifest,
        &manifest.target.cache_hash,
    )?;
    validate_file_hash(root, &manifest.source.path, &manifest.source.hash)?;
    validate_source_rows(manifest, root)?;
    validate_resolved_row_artifacts(manifest, root)?;
    validate_checklist(manifest, root)?;
    validate_inventory(manifest, root)
}

fn validate_resolved_row_artifacts(
    manifest: &PatchAuditManifest,
    root: &Path,
) -> Result<(), String> {
    for row in manifest
        .rows
        .iter()
        .filter(|row| row.resolution != ResolutionKind::Untriaged)
    {
        for evidence in &row.evidence {
            let path = reference_path(&evidence.reference);
            validate_file_hash(
                root,
                path,
                evidence.source_hash.as_deref().unwrap_or_default(),
            )?;
            if evidence.kind == EvidenceKind::Test {
                validate_test_reference(root, &evidence.reference)?;
            }
        }
        for test in &row.tests {
            validate_test_reference(root, test)?;
        }
        if let Some(commit) = &row.commit {
            validate_commit(root, commit)?;
        }
    }
    Ok(())
}

fn validate_checklist(manifest: &PatchAuditManifest, root: &Path) -> Result<(), String> {
    let checklist_path = root.join(&manifest.output.checklist);
    let actual = std::fs::read_to_string(&checklist_path)
        .map_err(|error| format!("failed to read {}: {error}", checklist_path.display()))?;
    let expected = format!("{}\n", render_checklist(manifest));
    if actual != expected {
        return Err(format!(
            "generated checklist drift: {}",
            checklist_path.display()
        ));
    }
    Ok(())
}

fn validate_source_rows(manifest: &PatchAuditManifest, root: &Path) -> Result<(), String> {
    let contents = std::fs::read_to_string(root.join(&manifest.source.path))
        .map_err(|error| format!("failed to read patch source: {error}"))?;
    let source: serde_json::Value = serde_json::from_str(&contents)
        .map_err(|error| format!("invalid patch source JSON: {error}"))?;
    let expected = source_row_ids(&source)?;
    let actual: Vec<&str> = manifest.rows.iter().map(|row| row.id.as_str()).collect();
    if actual != expected {
        return Err("manifest row order/content differs from patch source".to_string());
    }
    Ok(())
}

fn source_row_ids(source: &serde_json::Value) -> Result<Vec<String>, String> {
    let symbols = |direction: &str| -> Result<Vec<String>, String> {
        source[direction]
            .as_array()
            .ok_or_else(|| format!("patch source missing {direction} array"))?
            .iter()
            .map(|value| {
                value
                    .as_str()
                    .map(str::to_string)
                    .ok_or_else(|| format!("patch source {direction} contains a non-string"))
            })
            .collect()
    };
    Ok(symbols("added")?
        .into_iter()
        .map(|symbol| format!("added:{symbol}"))
        .chain(
            symbols("removed")?
                .into_iter()
                .map(|symbol| format!("removed:{symbol}")),
        )
        .collect())
}

fn validate_inventory(manifest: &PatchAuditManifest, root: &Path) -> Result<(), String> {
    let path = root.join(&manifest.output.inventory);
    let contents = std::fs::read_to_string(&path)
        .map_err(|error| format!("failed to read {}: {error}", path.display()))?;
    let actual: Vec<(String, String)> = contents
        .lines()
        .filter_map(|line| {
            let line = line.strip_prefix("| `")?;
            let (symbol, rest) = line.split_once("` | ")?;
            let (status, _) = rest.split_once(" | ")?;
            Some((symbol.to_string(), status.trim().to_string()))
        })
        .collect();
    let expected: Vec<(String, String)> = manifest
        .rows
        .iter()
        .map(|row| {
            (
                row.symbol.clone(),
                row.status
                    .map_or("untriaged", AuditStatus::as_str)
                    .to_string(),
            )
        })
        .collect();
    if actual != expected {
        return Err(format!("inventory drift: {}", path.display()));
    }
    Ok(())
}

fn reference_path(reference: &str) -> &str {
    reference
        .split_once("::")
        .map_or(reference, |(path, _)| path)
}

fn validate_file_hash(root: &Path, relative: &str, expected: &str) -> Result<(), String> {
    let path = root.join(relative);
    let bytes = std::fs::read(&path)
        .map_err(|error| format!("failed to read {}: {error}", path.display()))?;
    let actual = format!("{:x}", Sha256::digest(bytes));
    if actual != expected {
        return Err(format!(
            "hash mismatch for {}: expected {expected}, observed {actual}",
            path.display()
        ));
    }
    Ok(())
}

fn validate_test_reference(root: &Path, reference: &str) -> Result<(), String> {
    require_text("test reference", reference)?;
    let (path, symbol) = reference
        .split_once("::")
        .map_or((reference, None), |(path, symbol)| (path, Some(symbol)));
    let contents = std::fs::read_to_string(root.join(path))
        .map_err(|error| format!("failed to read test {path}: {error}"))?;
    if let Some(symbol) = symbol {
        require_text("test symbol", symbol)?;
        let source = strip_rust_comments(&contents);
        if !defines_rust_test(&source, symbol)? {
            return Err(format!("test {path} does not define #[test] fn {symbol}"));
        }
    }
    Ok(())
}

fn defines_rust_test(source: &str, symbol: &str) -> Result<bool, String> {
    let escaped = regex::escape(symbol);
    let pattern = format!(r"^\s*(?:pub(?:\([^)]*\))?\s+)?(?:async\s+)?fn\s+{escaped}\s*\(");
    let definition = Regex::new(&pattern)
        .map_err(|error| format!("invalid test definition pattern: {error}"))?;
    let lines: Vec<&str> = source.lines().collect();
    for (index, line) in lines.iter().enumerate() {
        if definition.is_match(line) && preceding_attributes_include_test(&lines[..index]) {
            return Ok(true);
        }
    }
    Ok(false)
}

fn preceding_attributes_include_test(lines: &[&str]) -> bool {
    lines
        .iter()
        .rev()
        .map(|line| line.trim())
        .take_while(|line| line.is_empty() || line.starts_with("#["))
        .any(|line| line == "#[test]")
}

fn strip_rust_comments(contents: &str) -> String {
    let mut result = String::with_capacity(contents.len());
    let mut in_block_comment = false;
    for line in contents.lines() {
        let mut remainder = line;
        loop {
            if in_block_comment {
                let Some((_, after)) = remainder.split_once("*/") else {
                    break;
                };
                remainder = after;
                in_block_comment = false;
                continue;
            }
            let before_line_comment = remainder
                .split_once("//")
                .map_or(remainder, |(code, _)| code);
            let Some((before, after)) = before_line_comment.split_once("/*") else {
                result.push_str(before_line_comment);
                break;
            };
            result.push_str(before);
            remainder = after;
            in_block_comment = true;
        }
        result.push('\n');
    }
    result
}

fn validate_commit(root: &Path, commit: &str) -> Result<(), String> {
    let object = Command::new("git")
        .args(["cat-file", "-e", &format!("{commit}^{{commit}}")])
        .current_dir(root)
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status()
        .map_err(|error| format!("failed to inspect commit {commit}: {error}"))?;
    if !object.success() {
        return Err(format!("commit {commit} does not resolve"));
    }
    let ancestor = Command::new("git")
        .args(["merge-base", "--is-ancestor", commit, "HEAD"])
        .current_dir(root)
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status()
        .map_err(|error| format!("failed to inspect ancestry for {commit}: {error}"))?;
    if !ancestor.success() {
        return Err(format!("commit {commit} is not an ancestor of HEAD"));
    }
    Ok(())
}

pub fn validate_complete(
    manifest: &PatchAuditManifest,
    root: &Path,
    manifest_json: &str,
    observations: &ObservationSet,
) -> Result<(), String> {
    validate_repository(manifest, root)?;
    if observations.schema != "framexml-patch-observations/v1" {
        return Err(format!(
            "unsupported observation schema: {}",
            observations.schema
        ));
    }
    validate_observation_binding(manifest_json, observations)?;
    let mut approvals = HashSet::new();
    for row in &manifest.rows {
        validate_completion_row(row, &mut approvals)?;
    }
    validate_observations(manifest, &observations.observations)
}

fn validate_observation_binding(
    manifest_json: &str,
    observations: &ObservationSet,
) -> Result<(), String> {
    let manifest_hash = format!("{:x}", Sha256::digest(manifest_json.as_bytes()));
    if observations.manifest_hash != manifest_hash {
        return Err("observation manifest hash does not match audited manifest".to_string());
    }
    Ok(())
}

fn validate_completion_row<'a>(
    row: &'a PatchAuditRow,
    approvals: &mut HashSet<&'a str>,
) -> Result<(), String> {
    if row.resolution == ResolutionKind::Untriaged {
        return Err(format!("row {} remains untriaged", row.id));
    }
    let status = row.status.expect("resolved row status validated");
    if status == AuditStatus::ExceptionRequested {
        let approval = row
            .approval_id
            .as_deref()
            .ok_or_else(|| format!("row {} requires an approval_id", row.id))?;
        let prefix = format!("user-chat:{}:", row.id);
        if !approval.starts_with(&prefix) || approval.len() == prefix.len() {
            return Err(format!(
                "row {} approval_id must start with {prefix}",
                row.id
            ));
        }
        if !approvals.insert(approval) {
            return Err(format!("duplicate approval_id: {approval}"));
        }
    } else if row.commit.is_none() {
        return Err(format!("row {} requires a commit", row.id));
    }
    Ok(())
}

pub fn validate_observations(
    manifest: &PatchAuditManifest,
    observations: &[Observation],
) -> Result<(), String> {
    let expected_count: usize = manifest.rows.iter().map(|row| row.assertions.len()).sum();
    if observations.len() != expected_count {
        return Err(format!(
            "observation count mismatch: expected {expected_count}, observed {}",
            observations.len()
        ));
    }
    let mut used = HashSet::new();
    for row in &manifest.rows {
        for assertion in &row.assertions {
            let (index, observation) = find_matching_observation(row, assertion, observations)?;
            if !used.insert(index) {
                return Err(format!(
                    "observation {index} matched more than one assertion"
                ));
            }
            validate_observation(row, assertion, observation)?;
        }
    }
    Ok(())
}

fn find_matching_observation<'a>(
    row: &PatchAuditRow,
    assertion: &AuditAssertion,
    observations: &'a [Observation],
) -> Result<(usize, &'a Observation), String> {
    let matches: Vec<(usize, &Observation)> = observations
        .iter()
        .enumerate()
        .filter(|(_, observation)| observation_matches(row, assertion, observation))
        .collect();
    if matches.len() == 1 {
        return Ok(matches[0]);
    }
    Err(format!(
        "{} expected exactly one observation for {:?}/{:?}/{:?}, found {}",
        row.id,
        assertion.flavor,
        assertion.phase,
        assertion.addon,
        matches.len()
    ))
}

fn observation_matches(
    row: &PatchAuditRow,
    assertion: &AuditAssertion,
    observation: &Observation,
) -> bool {
    observation.row_id == row.id
        && observation.flavor == assertion.flavor
        && observation.phase == assertion.phase
        && observation.addon == assertion.addon
}

fn validate_observation(
    row: &PatchAuditRow,
    assertion: &AuditAssertion,
    observation: &Observation,
) -> Result<(), String> {
    let expected_present = assertion.expected == ExpectedPresence::Present;
    if observation.present != expected_present {
        return Err(format!(
            "{} expected present={expected_present}, observed present={}",
            row.id, observation.present
        ));
    }
    if assertion.expected_type != observation.observed_type {
        return Err(format!(
            "{} expected type {:?}, observed {:?}",
            row.id, assertion.expected_type, observation.observed_type
        ));
    }
    Ok(())
}

pub fn render_summary(manifest: &PatchAuditManifest) -> String {
    let mut counts = BTreeMap::from([
        ("implemented", 0usize),
        ("best-effort", 0usize),
        ("exception-requested", 0usize),
        ("untriaged", 0usize),
    ]);
    for row in &manifest.rows {
        let status = row.status.map_or("untriaged", AuditStatus::as_str);
        *counts.get_mut(status).expect("known status") += 1;
    }
    format!(
        "Patch {} for {} {}: {} rows ({} implemented, {} best-effort, {} exception-requested, {} untriaged)\nSource: {}\nCache: {} ({})",
        manifest.patch,
        manifest.target.flavor.as_str(),
        manifest.target.build,
        manifest.rows.len(),
        counts["implemented"],
        counts["best-effort"],
        counts["exception-requested"],
        counts["untriaged"],
        manifest.source.path,
        manifest.target.cache_manifest,
        manifest.target.cache_hash,
    )
}

pub fn render_checklist(manifest: &PatchAuditManifest) -> String {
    manifest
        .rows
        .iter()
        .enumerate()
        .map(|(index, row)| {
            let status = row.status.map_or("untriaged", AuditStatus::as_str);
            format!(
                "{}. [{}] `{}` — {}",
                index + 1,
                status,
                row.id,
                row.resolution.as_str()
            )
        })
        .collect::<Vec<_>>()
        .join("\n")
}

#[cfg(test)]
mod tests {
    use super::*;

    fn fixture_manifest(row: &str) -> PatchAuditManifest {
        parse_manifest(&format!(
            r#"{{
              "schema":"framexml-patch-audit/v2",
              "patch":"12.1.0",
              "target":{{"flavor":"ptr","build":"12.1.0","cache_manifest":"cache","cache_hash":"{}"}},
              "source":{{"path":"source","hash":"{}","added_count":1,"removed_count":0}},
              "output":{{"checklist":"checklist","inventory":"inventory"}},
              "rows":[{row}]
            }}"#,
            "a".repeat(64),
            "b".repeat(64)
        ))
        .expect("fixture should parse")
    }

    fn resolved_row(status: &str, resolution: &str, assertion: &str) -> String {
        format!(
            r#"{{
              "id":"added:Fixture","symbol":"Fixture","change":"added",
              "status":"{status}","resolution":"{resolution}","owner":"Blizzard_Test",
              "evidence":[{{"kind":"source","reference":"fixture.lua","summary":"evidence","source_hash":"{}"}}],
              "tests":["tests/fixture.rs::fixture_test"],"assertions":[{assertion}],
              "commit":"1234567890","approval_id":null,"notes":null
            }}"#,
            "c".repeat(64)
        )
    }

    fn load_on_demand_row(assertions: &str, load_addon: &str) -> String {
        resolved_row("best-effort", "load-on-demand", assertions).replace(
            r#""owner":"Blizzard_Test","#,
            &format!(r#""owner":"internal-module","load_addon":"{load_addon}","#),
        )
    }

    fn assertion(flavor: &str, phase: &str, expected: &str) -> String {
        let ty = if expected == "present" {
            r#", "expected_type":"function""#
        } else {
            ""
        };
        format!(r#"{{"flavor":"{flavor}","phase":"{phase}","expected":"{expected}"{ty}}}"#)
    }

    #[test]
    fn every_checked_in_patch_manifest_matches_repository_artifacts() {
        let root = Path::new(env!("CARGO_MANIFEST_DIR"));
        let directory = root.join("data/patch-api");
        let mut manifests = Vec::new();
        for entry in std::fs::read_dir(&directory).expect("patch manifest directory should read") {
            let path = entry.expect("directory entry should read").path();
            if path.extension().and_then(|extension| extension.to_str()) == Some("json") {
                manifests.push(path);
            }
        }
        manifests.sort();
        assert!(
            !manifests.is_empty(),
            "at least one patch manifest is required"
        );
        for path in manifests {
            let json = std::fs::read_to_string(&path).expect("manifest should read");
            let manifest = parse_manifest(&json).expect("manifest should parse");
            validate_repository(&manifest, root)
                .unwrap_or_else(|error| panic!("{}: {error}", path.display()));
        }
    }

    #[test]
    fn untriaged_rows_have_no_final_status() {
        let manifest = fixture_manifest(
            r#"{"id":"added:Fixture","symbol":"Fixture","change":"added","status":null,"resolution":"untriaged","owner":"unknown","evidence":[],"tests":[],"assertions":[],"commit":null,"approval_id":null,"notes":"pending"}"#,
        );
        validate_manifest(&manifest).expect("neutral draft row should validate");
        assert!(render_checklist(&manifest).contains("[untriaged]"));
    }

    #[test]
    fn untriaged_rows_reject_blank_notes() {
        let manifest = fixture_manifest(
            r#"{"id":"added:Fixture","symbol":"Fixture","change":"added","status":null,"resolution":"untriaged","owner":"unknown","evidence":[],"tests":[],"assertions":[],"commit":null,"approval_id":null,"notes":""}"#,
        );
        assert!(
            validate_manifest(&manifest)
                .unwrap_err()
                .contains("notes must not be empty")
        );
    }

    #[test]
    fn untriaged_rows_reject_exception_status() {
        let manifest = fixture_manifest(
            r#"{"id":"added:Fixture","symbol":"Fixture","change":"added","status":"exception-requested","resolution":"untriaged","owner":"unknown","evidence":[],"tests":[],"assertions":[],"commit":null,"approval_id":null,"notes":null}"#,
        );
        assert!(
            validate_manifest(&manifest)
                .unwrap_err()
                .contains("status must be null")
        );
    }

    #[test]
    fn exception_status_requires_unsafe_or_impossible_resolution() {
        let row = resolved_row(
            "exception-requested",
            "compat",
            &assertion("ptr", "post-load", "present"),
        );
        let manifest = fixture_manifest(&row);
        assert!(
            validate_manifest(&manifest)
                .unwrap_err()
                .contains("incompatible")
        );
    }

    fn assert_observation_mismatch(assertion_json: String, observation_json: &str) {
        let row = resolved_row("best-effort", "vendor-present", &assertion_json);
        let manifest = fixture_manifest(&row);
        let observations = parse_observations(&format!(
            r#"{{"schema":"framexml-patch-observations/v1","manifest_hash":"{}","observations":[{observation_json}]}}"#,
            "d".repeat(64)
        ))
        .expect("observations should parse");
        assert!(validate_observations(&manifest, &observations.observations).is_err());
    }

    #[test]
    fn vendor_present_falsifier_rejects_wrong_flavor() {
        assert_observation_mismatch(
            assertion("ptr", "post-core", "present"),
            r#"{"row_id":"added:Fixture","flavor":"retail","phase":"post-core","present":true,"observed_type":"function","addon":null}"#,
        );
    }

    #[test]
    fn load_on_demand_falsifier_rejects_wrong_phase() {
        assert_observation_mismatch(
            assertion("ptr", "after-addon", "present"),
            r#"{"row_id":"added:Fixture","flavor":"ptr","phase":"before-addon","present":true,"observed_type":"function","addon":null}"#,
        );
    }

    #[test]
    fn cross_flavor_falsifier_rejects_target_leak() {
        assert_observation_mismatch(
            assertion("ptr", "post-load", "absent"),
            r#"{"row_id":"added:Fixture","flavor":"ptr","phase":"post-load","present":true,"observed_type":"function","addon":null}"#,
        );
    }

    #[test]
    fn absent_observation_rejects_non_null_type() {
        assert_observation_mismatch(
            assertion("ptr", "post-load", "absent"),
            r#"{"row_id":"added:Fixture","flavor":"ptr","phase":"post-load","present":false,"observed_type":"function","addon":null}"#,
        );
    }

    #[test]
    fn removed_after_reset_falsifier_rejects_resurrection() {
        assert_observation_mismatch(
            assertion("ptr", "post-reset", "absent"),
            r#"{"row_id":"added:Fixture","flavor":"ptr","phase":"post-reset","present":true,"observed_type":"function","addon":null}"#,
        );
    }

    #[test]
    fn source_drift_is_rejected() {
        let root = std::env::temp_dir().join(format!(
            "wow-ui-sim-patch-source-drift-{}",
            std::process::id()
        ));
        std::fs::create_dir_all(&root).expect("temporary directory should create");
        std::fs::write(root.join("source"), r#"{"added":["Wrong"],"removed":[]}"#)
            .expect("temporary source should write");
        let manifest = fixture_manifest(
            r#"{"id":"added:Fixture","symbol":"Fixture","change":"added","status":null,"resolution":"untriaged","owner":"unknown","evidence":[],"tests":[],"assertions":[],"commit":null,"approval_id":null,"notes":null}"#,
        );
        assert!(
            validate_source_rows(&manifest, &root)
                .unwrap_err()
                .contains("differs")
        );
        std::fs::remove_dir_all(root).expect("temporary directory should remove");
    }

    #[test]
    fn mismatched_evidence_hash_is_rejected() {
        let root =
            std::env::temp_dir().join(format!("wow-ui-sim-patch-hash-{}", std::process::id()));
        std::fs::create_dir_all(&root).expect("temporary directory should create");
        std::fs::write(root.join("evidence"), "real contents")
            .expect("temporary evidence should write");
        assert!(
            validate_file_hash(&root, "evidence", &"0".repeat(64))
                .unwrap_err()
                .contains("hash mismatch")
        );
        std::fs::remove_dir_all(root).expect("temporary directory should remove");
    }

    #[test]
    fn fake_test_and_commit_references_are_rejected() {
        let root = Path::new(env!("CARGO_MANIFEST_DIR"));
        assert!(validate_test_reference(root, "tests/not-real.rs::not_real").is_err());
        assert!(validate_commit(root, "0000000000000000000000000000000000000000").is_err());
    }

    #[test]
    fn comments_and_prefixes_do_not_satisfy_named_test_references() {
        let root = std::env::temp_dir().join(format!(
            "wow-ui-sim-patch-test-reference-{}",
            std::process::id()
        ));
        std::fs::create_dir_all(&root).expect("temporary directory should create");
        std::fs::write(
            root.join("fixture.rs"),
            "// fn exact_test() {}\nfn exact_test_suffix() {}\n/* fn exact_test() {} */\n",
        )
        .expect("temporary test should write");
        assert!(validate_test_reference(&root, "fixture.rs::exact_test").is_err());
        std::fs::remove_dir_all(root).expect("temporary directory should remove");
    }

    #[test]
    fn load_on_demand_requires_declared_addon_and_target_flavor() {
        let assertions = r#"{"flavor":"ptr","phase":"before-addon","expected":"absent","addon":"Addon_A"},{"flavor":"retail","phase":"after-addon","expected":"present","expected_type":"function","addon":"Addon_A"}"#;
        let manifest = fixture_manifest(&load_on_demand_row(assertions, "Addon_A"));
        assert!(validate_manifest(&manifest).is_err());

        let assertions = r#"{"flavor":"ptr","phase":"before-addon","expected":"absent","addon":"Addon_A"},{"flavor":"ptr","phase":"after-addon","expected":"present","expected_type":"function","addon":"Addon_B"}"#;
        let manifest = fixture_manifest(&load_on_demand_row(assertions, "Addon_A"));
        assert!(validate_manifest(&manifest).is_err());

        let assertions = r#"{"flavor":"ptr","phase":"before-addon","expected":"absent","addon":"Addon_A"},{"flavor":"ptr","phase":"after-addon","expected":"present","expected_type":"function","addon":"Addon_A"}"#;
        let manifest = fixture_manifest(&load_on_demand_row(assertions, "Addon_B"));
        assert!(validate_manifest(&manifest).is_err());

        let manifest = fixture_manifest(&load_on_demand_row(assertions, "Addon_A"));
        validate_manifest(&manifest).expect("declared addon lifecycle should validate");
    }

    #[test]
    fn cross_flavor_contract_uses_manifest_target() {
        let row = resolved_row(
            "best-effort",
            "cross-flavor",
            &assertion("retail", "post-load", "absent"),
        );
        let mut manifest = fixture_manifest(&row);
        manifest.target.flavor = AuditFlavor::Retail;
        validate_manifest(&manifest).expect("target-flavor absence should validate");
    }

    #[test]
    fn completion_requires_commit_for_non_exception_rows() {
        let json = resolved_row(
            "best-effort",
            "vendor-present",
            &assertion("ptr", "post-load", "present"),
        )
        .replace(r#""commit":"1234567890""#, r#""commit":null"#);
        let manifest = fixture_manifest(&json);
        let mut approvals = HashSet::new();
        assert!(
            validate_completion_row(&manifest.rows[0], &mut approvals)
                .unwrap_err()
                .contains("requires a commit")
        );
    }

    #[test]
    fn completion_requires_item_bound_exception_approval() {
        let json = resolved_row(
            "exception-requested",
            "unsafe",
            &assertion("ptr", "post-load", "absent"),
        )
        .replace(
            r#""approval_id":null"#,
            r#""approval_id":"user-chat:added:Other:approval-1""#,
        );
        let manifest = fixture_manifest(&json);
        let mut approvals = HashSet::new();
        assert!(
            validate_completion_row(&manifest.rows[0], &mut approvals)
                .unwrap_err()
                .contains("must start")
        );
    }

    #[test]
    fn observations_must_bind_to_exact_manifest_bytes() {
        let observations = parse_observations(&format!(
            r#"{{"schema":"framexml-patch-observations/v1","manifest_hash":"{}","observations":[]}}"#,
            "0".repeat(64)
        ))
        .expect("observations should parse");
        assert!(
            validate_observation_binding("different manifest", &observations)
                .unwrap_err()
                .contains("does not match")
        );
    }

    #[test]
    fn unknown_schema_fields_are_rejected() {
        let json = format!(
            r#"{{"schema":"framexml-patch-audit/v2","patch":"x","target":{{"flavor":"ptr","build":"x","cache_manifest":"x","cache_hash":"{}","typo":true}},"source":{{"path":"x","hash":"{}","added_count":0,"removed_count":0}},"output":{{"checklist":"x","inventory":"x"}},"rows":[]}}"#,
            "a".repeat(64),
            "b".repeat(64)
        );
        assert!(parse_manifest(&json).is_err());
    }
}
