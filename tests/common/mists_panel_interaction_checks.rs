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

fn read_mists_panel_interaction_baseline(repo_root: &Path) -> String {
    std::fs::read_to_string(repo_root.join("docs/baselines/mists-panel-interactions.md"))
        .expect("failed to read Mists panel interaction baseline")
}
