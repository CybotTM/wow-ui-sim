#![cfg(feature = "client-mists")]

use std::path::PathBuf;
use std::process::Command;

fn repo_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
}

#[test]
fn runner_manifest_fails_when_a_panel_row_has_no_script_case() {
    let baseline = repo_root()
        .join("target")
        .join("mists-panel-parity-test")
        .join("missing-panel.md");
    std::fs::create_dir_all(baseline.parent().expect("baseline should have parent"))
        .expect("failed to create test baseline directory");
    std::fs::write(
        &baseline,
        [
            "| Panel | Status | Screenshot | Gap notes |",
            "|---|---|---|---|",
            "| Unknown Mists Panel | Pass | test-backed | intentionally unmapped |",
        ]
        .join("\n"),
    )
    .expect("failed to write test baseline");

    let output = Command::new(repo_root().join("scripts/mists-panel-parity.sh"))
        .arg("--validate-only")
        .arg("--baseline")
        .arg(&baseline)
        .current_dir(repo_root())
        .output()
        .expect("failed to run Mists panel parity runner");

    let stderr = String::from_utf8_lossy(&output.stderr);

    assert!(
        !output.status.success(),
        "runner validation should reject unmapped rows"
    );
    assert!(
        stderr.contains("no runner case for panel row: Unknown Mists Panel"),
        "runner should explain the missing case, got:\n{stderr}"
    );
}

#[test]
fn runner_manifest_covers_every_mists_panel_baseline_row() {
    let output = Command::new(repo_root().join("scripts/mists-panel-parity.sh"))
        .arg("--validate-only")
        .current_dir(repo_root())
        .output()
        .expect("failed to run Mists panel parity runner");

    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);

    assert!(
        output.status.success(),
        "runner manifest validation failed\nstdout:\n{stdout}\nstderr:\n{stderr}"
    );
    assert!(
        stdout.contains("23 panel rows validated"),
        "runner should report all current panel rows, got:\n{stdout}"
    );
}

#[test]
fn runner_manifest_accepts_saved_vars_mode() {
    let output = Command::new(repo_root().join("scripts/mists-panel-parity.sh"))
        .arg("--validate-only")
        .arg("--with-saved-vars")
        .current_dir(repo_root())
        .output()
        .expect("failed to run Mists panel parity runner");

    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);

    assert!(
        output.status.success(),
        "saved-vars runner validation failed\nstdout:\n{stdout}\nstderr:\n{stderr}"
    );
    assert!(
        stdout.contains("23 panel rows validated"),
        "saved-vars mode should still validate all panel rows, got:\n{stdout}"
    );
}

#[test]
fn panel_baseline_references_retained_runner_artifacts() {
    let baseline_path = repo_root().join("docs/baselines/mists-panels.md");
    let baseline =
        std::fs::read_to_string(&baseline_path).expect("failed to read Mists panel baseline");

    assert!(
        !baseline.contains("test-backed:"),
        "baseline still contains test-backed placeholders"
    );
    assert!(
        baseline.contains("target/mists-panel-parity/character/screenshot.webp"),
        "baseline should reference retained screenshot artifacts"
    );
    assert!(
        baseline.contains("target/mists-panel-parity/game-menu-options/dump-tree.txt"),
        "baseline should reference retained frame-dump artifacts"
    );
}
