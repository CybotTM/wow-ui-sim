#![cfg(feature = "client-mists")]

use std::path::PathBuf;

#[path = "common/mists_panel_artifact_checks.rs"]
mod mists_panel_artifact_checks;
#[path = "common/mists_panel_interaction_checks.rs"]
mod mists_panel_interaction_checks;

const LATEST_LOCAL_PANEL_ARTIFACT_ROOT: &str =
    "target/mists-local-completion-audit-20260515/panel-parity-with-saved-vars-240/";

fn repo_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
}

#[test]
fn latest_local_panel_logs_have_no_texture_directory_errors() {
    mists_panel_artifact_checks::assert_panel_logs_have_no_texture_directory_errors(
        &repo_root(),
        LATEST_LOCAL_PANEL_ARTIFACT_ROOT,
    );
}
