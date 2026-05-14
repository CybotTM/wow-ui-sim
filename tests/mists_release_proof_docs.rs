#![cfg(feature = "client-mists")]

use std::path::PathBuf;

fn repo_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
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
