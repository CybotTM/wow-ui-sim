#![cfg(feature = "client-mists")]

use std::path::PathBuf;

fn repo_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
}

fn normalize_whitespace(text: &str) -> String {
    text.split_whitespace().collect::<Vec<_>>().join(" ")
}

#[test]
fn release_proof_index_records_bounded_saved_vars_enforcement() {
    let index_path = repo_root().join("docs/baselines/mists-release-proof.md");
    let index =
        std::fs::read_to_string(index_path).expect("failed to read Mists release-proof index");

    assert!(
        !index.contains("remaining local coverage gap is enforcement"),
        "release-proof index should not describe implemented bounded sample enforcement as a remaining gap"
    );
    assert!(
        index.contains("bounded_saved_vars_addon_samples_cover_installed_mists_addons"),
        "release-proof index should cite the bounded saved-vars addon sample checker"
    );
}

#[test]
fn release_proof_index_names_full_addon_screenshot_matrix_as_scope_limit() {
    let index_path = repo_root().join("docs/baselines/mists-release-proof.md");
    let index =
        std::fs::read_to_string(index_path).expect("failed to read Mists release-proof index");
    let single_line_index = normalize_whitespace(&index);

    assert!(
        index.contains("## Validation Scope Limits"),
        "release-proof index should keep validation scope limits in a named section"
    );
    assert!(
        index.contains("full installed-addon screenshot matrix"),
        "release-proof index should name the deferred full installed-addon screenshot matrix"
    );
    assert!(
        single_line_index.contains(
            "not proof that every installed-addon panel screenshot has been exercised locally"
        ),
        "release-proof index should not let bounded samples masquerade as full local screenshot matrix proof"
    );
}
