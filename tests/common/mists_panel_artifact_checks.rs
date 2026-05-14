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
