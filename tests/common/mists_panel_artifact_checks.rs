use std::collections::BTreeSet;
use std::path::Path;

pub fn panel_slugs(repo_root: &Path, artifact_root: &str) -> Vec<String> {
    read_panel_baseline(repo_root)
        .lines()
        .filter(|line| line.starts_with('|') && line.contains(" | Pass | "))
        .filter_map(|line| panel_slug_from_baseline_row(line, artifact_root))
        .map(str::to_owned)
        .collect()
}

pub fn retained_panel_artifacts(repo_root: &Path) -> Vec<String> {
    read_panel_baseline(repo_root)
        .lines()
        .filter(|line| line.starts_with('|') && line.contains(" | Pass | "))
        .flat_map(panel_artifacts_from_baseline_row)
        .collect()
}

pub fn retained_lua_error_artifacts(repo_root: &Path, artifact_root: &str) -> Vec<String> {
    panel_slugs(repo_root, artifact_root)
        .into_iter()
        .map(|slug| format!("{artifact_root}{slug}/lua-errors.json"))
        .collect()
}

pub fn retained_frame_dump_artifacts(repo_root: &Path, artifact_root: &str) -> Vec<String> {
    panel_slugs(repo_root, artifact_root)
        .into_iter()
        .map(|slug| format!("{artifact_root}{slug}/dump-tree.txt"))
        .collect()
}

pub fn retained_screenshot_artifacts(repo_root: &Path, artifact_root: &str) -> Vec<String> {
    panel_slugs(repo_root, artifact_root)
        .into_iter()
        .map(|slug| format!("{artifact_root}{slug}/screenshot.webp"))
        .collect()
}

pub fn assert_empty_lua_error_artifact(repo_root: &Path, artifact: &str) {
    let path = repo_root.join(artifact);
    assert!(
        path.is_file(),
        "lua-error artifact should exist: {artifact}"
    );

    let contents = std::fs::read_to_string(&path)
        .unwrap_or_else(|error| panic!("failed to read {artifact}: {error}"));
    let errors: Vec<serde_json::Value> = serde_json::from_str(&contents)
        .unwrap_or_else(|error| panic!("failed to parse {artifact}: {error}"));

    assert!(
        errors.is_empty(),
        "lua-error artifact should contain an empty array: {artifact}"
    );
}

pub fn assert_frame_dump_has_visible_root(repo_root: &Path, artifact: &str) {
    let path = repo_root.join(artifact);
    assert!(
        path.is_file(),
        "frame dump artifact should exist: {artifact}"
    );

    let contents = std::fs::read_to_string(&path)
        .unwrap_or_else(|error| panic!("failed to read {artifact}: {error}"));
    assert!(
        !contents.trim().is_empty(),
        "frame dump artifact should not be empty: {artifact}"
    );
    assert!(
        contents.lines().any(is_visible_root_frame_dump_line),
        "frame dump artifact should contain a visible root frame marker: {artifact}"
    );
}

pub fn assert_non_empty_screenshot_artifact(repo_root: &Path, artifact: &str) {
    let path = repo_root.join(artifact);
    assert!(
        path.is_file(),
        "screenshot artifact should exist: {artifact}"
    );

    let metadata = std::fs::metadata(&path)
        .unwrap_or_else(|error| panic!("failed to stat {artifact}: {error}"));
    assert!(
        metadata.len() > 0,
        "screenshot artifact should not be zero bytes: {artifact}"
    );
}

pub fn assert_webp_screenshot_artifact(repo_root: &Path, artifact: &str) {
    let path = repo_root.join(artifact);
    assert!(
        path.is_file(),
        "screenshot artifact should exist: {artifact}"
    );

    let bytes = std::fs::read(&path)
        .unwrap_or_else(|error| panic!("failed to read screenshot artifact {artifact}: {error}"));
    assert!(
        bytes.len() >= 12,
        "screenshot artifact should include a WebP header: {artifact}"
    );
    assert_eq!(
        &bytes[0..4],
        b"RIFF",
        "screenshot artifact should start with RIFF: {artifact}"
    );
    assert_eq!(
        &bytes[8..12],
        b"WEBP",
        "screenshot artifact should be a WebP RIFF container: {artifact}"
    );
}

pub fn assert_panel_artifact_slug_sets_match(repo_root: &Path, artifact_root: &str) {
    let expected_slugs = expected_panel_slug_set(repo_root, artifact_root);
    let screenshot_slugs = artifact_slugs_with_file(repo_root, artifact_root, "screenshot.webp");
    let frame_dump_slugs = artifact_slugs_with_file(repo_root, artifact_root, "dump-tree.txt");
    let lua_error_slugs = artifact_slugs_with_file(repo_root, artifact_root, "lua-errors.json");

    assert_eq!(
        screenshot_slugs, expected_slugs,
        "latest retained screenshot slug set should match the panel baseline"
    );
    assert_eq!(
        frame_dump_slugs, expected_slugs,
        "latest retained frame-dump slug set should match the panel baseline"
    );
    assert_eq!(
        lua_error_slugs, expected_slugs,
        "latest retained lua-error slug set should match the panel baseline"
    );
}

pub fn assert_panel_baseline_schema_is_valid(repo_root: &Path) {
    let baseline = read_panel_baseline(repo_root);
    let rows = panel_table_rows(&baseline);

    assert_panel_rows_have_four_columns(&rows);
    assert_panel_rows_have_known_statuses(&rows);
    assert_panel_rows_have_no_empty_fields(&rows);
    assert_panel_rows_have_unique_panel_names(&rows);
    assert_panel_rows_have_screenshot_and_dump_refs(&rows);
}

fn expected_panel_slug_set(repo_root: &Path, artifact_root: &str) -> BTreeSet<String> {
    panel_slugs(repo_root, artifact_root).into_iter().collect()
}

fn artifact_slugs_with_file(
    repo_root: &Path,
    artifact_root: &str,
    artifact_name: &str,
) -> BTreeSet<String> {
    let root = repo_root.join(artifact_root);
    let entries = std::fs::read_dir(&root)
        .unwrap_or_else(|error| panic!("failed to read artifact root {artifact_root}: {error}"));

    entries
        .filter_map(Result::ok)
        .filter(|entry| entry.path().join(artifact_name).is_file())
        .filter_map(|entry| entry.file_name().into_string().ok())
        .collect()
}

fn panel_table_rows(baseline: &str) -> Vec<&str> {
    let mut found_header = false;
    let mut rows = Vec::new();

    for line in baseline.lines() {
        if !found_header {
            found_header = crate::mists_panel_interaction_checks::markdown_table_columns(line)
                .first()
                == Some(&"Panel");
            continue;
        }
        if line.trim().is_empty() {
            break;
        }

        let columns = crate::mists_panel_interaction_checks::markdown_table_columns(line);
        if line.starts_with('|') && !is_separator_row(&columns) {
            rows.push(line);
        }
    }

    rows
}

fn assert_panel_rows_have_four_columns(rows: &[&str]) {
    let malformed_rows = rows
        .iter()
        .copied()
        .filter(|row| crate::mists_panel_interaction_checks::markdown_table_columns(row).len() != 4)
        .collect::<Vec<_>>();

    assert!(
        malformed_rows.is_empty(),
        "panel baseline rows should have exactly four columns: {malformed_rows:?}"
    );
}

fn assert_panel_rows_have_known_statuses(rows: &[&str]) {
    let unknown_status_rows = rows
        .iter()
        .copied()
        .filter(|row| {
            let columns = crate::mists_panel_interaction_checks::markdown_table_columns(row);
            !is_known_panel_status(columns[1])
        })
        .collect::<Vec<_>>();

    assert!(
        unknown_status_rows.is_empty(),
        "panel baseline rows should use known statuses: {unknown_status_rows:?}"
    );
}

fn assert_panel_rows_have_no_empty_fields(rows: &[&str]) {
    let rows_with_empty_fields = rows
        .iter()
        .copied()
        .filter(|row| {
            crate::mists_panel_interaction_checks::markdown_table_columns(row)
                .iter()
                .any(|column| column.is_empty())
        })
        .collect::<Vec<_>>();

    assert!(
        rows_with_empty_fields.is_empty(),
        "panel baseline rows should not contain empty fields: {rows_with_empty_fields:?}"
    );
}

fn assert_panel_rows_have_unique_panel_names(rows: &[&str]) {
    let mut seen_panel_names = BTreeSet::new();
    let duplicate_panel_names = rows
        .iter()
        .filter_map(|row| {
            crate::mists_panel_interaction_checks::markdown_table_columns(row)
                .first()
                .copied()
                .map(str::to_owned)
        })
        .filter(|panel| !seen_panel_names.insert(panel.clone()))
        .collect::<Vec<_>>();

    assert!(
        duplicate_panel_names.is_empty(),
        "panel baseline panel names should be unique: {duplicate_panel_names:?}"
    );
}

fn assert_panel_rows_have_screenshot_and_dump_refs(rows: &[&str]) {
    let rows_missing_artifacts = rows
        .iter()
        .copied()
        .filter(|row| {
            let artifacts = panel_artifacts_from_baseline_row(row);
            !has_exact_artifact_count(&artifacts, "/screenshot.webp", 1)
                || !has_exact_artifact_count(&artifacts, "/dump-tree.txt", 1)
        })
        .collect::<Vec<_>>();

    assert!(
        rows_missing_artifacts.is_empty(),
        "panel baseline rows should reference one screenshot and one dump: {rows_missing_artifacts:?}"
    );
}

fn has_exact_artifact_count(artifacts: &[String], suffix: &str, expected_count: usize) -> bool {
    artifacts
        .iter()
        .filter(|path| path.ends_with(suffix))
        .count()
        == expected_count
}

fn is_separator_row(columns: &[&str]) -> bool {
    columns
        .iter()
        .all(|column| column.chars().all(|ch| ch == '-'))
}

fn is_known_panel_status(status: &str) -> bool {
    matches!(status, "Pass" | "Watch" | "Fail")
}

fn read_panel_baseline(repo_root: &Path) -> String {
    std::fs::read_to_string(repo_root.join("docs/baselines/mists-panels.md"))
        .expect("failed to read Mists panel baseline")
}

fn panel_slug_from_baseline_row<'a>(line: &'a str, artifact_root: &str) -> Option<&'a str> {
    line.split(artifact_root)
        .nth(1)
        .and_then(|path| path.split('/').next())
}

fn panel_artifacts_from_baseline_row(line: &str) -> Vec<String> {
    line.split('`')
        .filter(|part| part.starts_with("target/") && is_panel_artifact_path(part))
        .map(str::to_owned)
        .collect()
}

fn is_panel_artifact_path(path: &str) -> bool {
    path.ends_with("/screenshot.webp") || path.ends_with("/dump-tree.txt")
}

fn is_visible_root_frame_dump_line(line: &str) -> bool {
    let is_top_level_widget = !line.starts_with(' ') && line.contains('[');
    is_top_level_widget && line.contains(" visible ")
}
