use std::collections::BTreeSet;
use std::path::Path;

pub fn assert_interaction_test_references_exist(repo_root: &Path) {
    let missing_references = interaction_test_references(repo_root)
        .into_iter()
        .filter(|reference| !interaction_test_reference_exists(repo_root, reference))
        .collect::<Vec<_>>();

    assert!(
        missing_references.is_empty(),
        "interaction baseline references missing tests: {missing_references:?}"
    );
}

pub fn assert_interaction_rows_have_strong_evidence(repo_root: &Path) {
    let baseline = read_mists_panel_interaction_baseline(repo_root);
    let weak_rows = baseline
        .lines()
        .filter(|line| line.starts_with('|'))
        .filter(|line| interaction_row_has_weak_evidence_name(line))
        .collect::<Vec<_>>();

    assert!(
        weak_rows.is_empty(),
        "interaction baseline rows cite load-only or no-lua-error evidence names: {weak_rows:?}"
    );
}

pub fn assert_covered_rows_cite_mists_tests(repo_root: &Path) {
    let baseline = read_mists_panel_interaction_baseline(repo_root);
    let weak_rows = baseline
        .lines()
        .filter(|line| row_requires_mists_test_reference(line))
        .filter(|line| !row_cites_mists_test_reference(line))
        .collect::<Vec<_>>();

    assert!(
        weak_rows.is_empty(),
        "covered interaction rows should cite at least one tests/mists_*.rs reference: {weak_rows:?}"
    );
}

fn interaction_test_references(repo_root: &Path) -> BTreeSet<String> {
    read_mists_panel_interaction_baseline(repo_root)
        .split('`')
        .filter(|part| part.starts_with("tests/mists_") && part.contains(".rs::"))
        .map(str::to_owned)
        .collect()
}

fn interaction_test_reference_exists(repo_root: &Path, reference: &str) -> bool {
    let Some((file_path, function_name)) = reference.split_once("::") else {
        return false;
    };
    let test_path = repo_root.join(file_path);
    let test_contents = std::fs::read_to_string(test_path).unwrap_or_default();
    let function_signature = format!("fn {function_name}(");

    test_contents.contains(&function_signature)
}

fn interaction_row_has_weak_evidence_name(line: &str) -> bool {
    line.split('`')
        .filter(|part| part.starts_with("tests/mists_") && part.contains(".rs::"))
        .any(test_reference_name_is_weak)
}

fn test_reference_name_is_weak(reference: &str) -> bool {
    let weak_fragments = [
        "without_lua_errors",
        "open_cleanly",
        "render_without_lua_errors",
        "load_ui_creates_renderable",
    ];

    weak_fragments
        .iter()
        .any(|fragment| reference.contains(fragment))
}

fn row_requires_mists_test_reference(line: &str) -> bool {
    let columns = markdown_table_columns(line);
    let Some(status) = columns.get(3) else {
        return false;
    };

    matches!(*status, "Covered" | "Mists-specific")
}

fn row_cites_mists_test_reference(line: &str) -> bool {
    line.split('`')
        .any(|part| part.starts_with("tests/mists_") && part.contains(".rs::"))
}

pub fn markdown_table_columns(line: &str) -> Vec<&str> {
    if !line.starts_with('|') {
        return Vec::new();
    }

    line.trim_matches('|').split('|').map(str::trim).collect()
}

fn read_mists_panel_interaction_baseline(repo_root: &Path) -> String {
    std::fs::read_to_string(repo_root.join("docs/baselines/mists-panel-interactions.md"))
        .expect("failed to read Mists panel interaction baseline")
}
