#![cfg(feature = "client-mists")]

use std::collections::BTreeSet;
use std::path::PathBuf;
use std::process::Command;

const MISTS_PANEL_ROW_COUNT: &str = "38 panel rows validated";

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
        stdout.contains(MISTS_PANEL_ROW_COUNT),
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
        stdout.contains(MISTS_PANEL_ROW_COUNT),
        "saved-vars mode should still validate all panel rows, got:\n{stdout}"
    );
}

#[test]
fn runner_manifest_accepts_addon_mode() {
    let output = Command::new(repo_root().join("scripts/mists-panel-parity.sh"))
        .arg("--validate-only")
        .arg("--with-addons")
        .current_dir(repo_root())
        .output()
        .expect("failed to run Mists panel parity runner");

    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);

    assert!(
        output.status.success(),
        "addon-mode runner validation failed\nstdout:\n{stdout}\nstderr:\n{stderr}"
    );
    assert!(
        stdout.contains(MISTS_PANEL_ROW_COUNT),
        "addon mode should still validate all panel rows, got:\n{stdout}"
    );
}

#[test]
fn addon_panel_matrix_validates_installed_mists_addons() {
    let output = Command::new(repo_root().join("scripts/test-mists-addon-panels.sh"))
        .arg("--validate-only")
        .current_dir(repo_root())
        .output()
        .expect("failed to run Mists addon panel matrix");

    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);

    assert!(
        output.status.success(),
        "addon panel matrix validation failed\nstdout:\n{stdout}\nstderr:\n{stderr}"
    );
    assert!(
        stdout.contains("9 installed Mists addon row(s) validated"),
        "addon panel matrix should validate the current Mists addon rows, got:\n{stdout}"
    );
}

#[test]
fn bounded_saved_vars_addon_samples_cover_installed_mists_addons() {
    let manifest_addons = mists_addon_names().into_iter().collect::<BTreeSet<_>>();
    let sample_addons = bounded_saved_vars_addon_sample_names();

    assert_eq!(
        manifest_addons, sample_addons,
        "every installed Mists addon should have at least one bounded saved-vars panel sample"
    );
}

#[test]
fn live_gui_smoke_runner_validates_micro_button_rows() {
    let output = Command::new(repo_root().join("scripts/mists-live-gui-smoke.sh"))
        .arg("--validate-only")
        .current_dir(repo_root())
        .output()
        .expect("failed to run Mists live GUI smoke validation");

    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);

    assert!(
        output.status.success(),
        "live GUI smoke validation failed\nstdout:\n{stdout}\nstderr:\n{stderr}"
    );
    assert!(
        stdout.contains("14 Mists live GUI micro-button row(s) validated"),
        "live GUI smoke should cover every Mists micro-button opener, got:\n{stdout}"
    );
}

#[test]
fn live_gui_smoke_runner_validates_hud_and_specialization_probes() {
    let output = Command::new(repo_root().join("scripts/mists-live-gui-smoke.sh"))
        .arg("--validate-only")
        .current_dir(repo_root())
        .output()
        .expect("failed to run Mists live GUI smoke validation");

    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);

    assert!(
        output.status.success(),
        "live GUI smoke validation failed\nstdout:\n{stdout}\nstderr:\n{stderr}"
    );
    assert!(
        stdout.contains("2 Mists live GUI direct probe(s) validated"),
        "live GUI smoke should validate idle HUD and specialization probes, got:\n{stdout}"
    );
}

#[test]
fn live_gui_smoke_runner_accepts_focused_button_validation() {
    let output = Command::new(repo_root().join("scripts/mists-live-gui-smoke.sh"))
        .arg("--validate-only")
        .arg("--button")
        .arg("CollectionsMicroButton")
        .current_dir(repo_root())
        .output()
        .expect("failed to run focused Mists live GUI smoke validation");

    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);

    assert!(
        output.status.success(),
        "focused live GUI smoke validation failed\nstdout:\n{stdout}\nstderr:\n{stderr}"
    );
    assert!(
        stdout.contains("1 Mists live GUI micro-button row(s) validated"),
        "focused live GUI smoke should validate exactly one row, got:\n{stdout}"
    );
}

#[test]
fn release_proof_command_validates_required_mists_lanes() {
    let output = Command::new(repo_root().join("scripts/ci-mists-release-proof.sh"))
        .arg("--validate-only")
        .current_dir(repo_root())
        .output()
        .expect("failed to run Mists release proof validation");

    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);

    assert!(
        output.status.success(),
        "release proof validation failed\nstdout:\n{stdout}\nstderr:\n{stderr}"
    );

    for expected_lane in [
        "zero-lua-errors",
        "installed-addon-matrix",
        "panel-parity",
        "installed-addon-panel-matrix",
        "interaction-audit",
        "visual-comparison",
        "artifact-completeness",
    ] {
        assert!(
            stdout.contains(expected_lane),
            "release proof validation should list {expected_lane}, got:\n{stdout}"
        );
    }
}

#[test]
fn github_actions_runs_mists_release_proof_by_default() {
    let workflow_path = repo_root().join(".github/workflows/test.yml");
    let workflow = std::fs::read_to_string(&workflow_path).expect("failed to read workflow");
    let proof_job = workflow_job_block(&workflow, "mists-release-proof");

    assert!(
        !proof_job.contains("RUN_MISTS_RELEASE_PROOF"),
        "Mists release proof should not be gated by a repository variable"
    );
    assert!(
        !proof_job.contains("run_mists_release_proof"),
        "Mists release proof should not be gated by workflow_dispatch input"
    );
}

#[test]
fn release_proof_artifact_validation_rejects_missing_panel_screenshots() {
    let out_dir = release_proof_artifact_test_dir("missing-panel-screenshot");
    if out_dir.exists() {
        std::fs::remove_dir_all(&out_dir).expect("failed to clean prior artifact fixture");
    }
    create_release_proof_artifact_fixture(&out_dir);

    let missing_screenshot = out_dir
        .join("panel-parity")
        .join("character")
        .join("screenshot.webp");
    std::fs::remove_file(&missing_screenshot).expect("failed to remove screenshot fixture");

    let output = run_release_proof_artifact_validation(&out_dir);

    let stderr = String::from_utf8_lossy(&output.stderr);

    assert!(
        !output.status.success(),
        "artifact validation should reject missing panel screenshots"
    );
    assert!(
        stderr.contains("missing artifact file"),
        "artifact validation should explain the missing file, got:\n{stderr}"
    );
    assert!(
        stderr.contains("panel-parity/character/screenshot.webp"),
        "artifact validation should identify the missing screenshot, got:\n{stderr}"
    );
}

#[test]
fn release_proof_artifact_validation_accepts_complete_artifacts() {
    let out_dir = release_proof_artifact_test_dir("complete");
    if out_dir.exists() {
        std::fs::remove_dir_all(&out_dir).expect("failed to clean prior artifact fixture");
    }
    create_release_proof_artifact_fixture(&out_dir);

    let output = run_release_proof_artifact_validation(&out_dir);

    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);

    assert!(
        output.status.success(),
        "complete artifact validation failed\nstdout:\n{stdout}\nstderr:\n{stderr}"
    );
    assert!(
        stdout.contains("Mists release proof artifacts are complete"),
        "artifact validation should report success, got:\n{stdout}"
    );
}

#[test]
fn lod_audit_documents_every_mists_load_on_demand_addon() {
    let audit_path = repo_root().join("docs/baselines/mists-lod-audit.md");
    let audit = std::fs::read_to_string(&audit_path).expect("failed to read Mists LoD audit");
    let audited_addons = audited_lod_addons(&audit);
    let addon_root = repo_root().join("Interface/BlizzardUI/Mists/AddOns");
    let addon_entries =
        std::fs::read_dir(&addon_root).expect("failed to read Mists Blizzard addons");
    let missing: Vec<_> = addon_entries
        .map(|entry| entry.expect("failed to read addon directory entry"))
        .filter(|entry| {
            let path = entry.path();
            path.is_dir() && addon_has_load_on_demand_toc(&path)
        })
        .map(|entry| entry.file_name().to_string_lossy().into_owned())
        .filter(|addon_name| !audited_addons.contains(addon_name.as_str()))
        .collect();

    assert!(
        missing.is_empty(),
        "Mists LoD audit is missing addon rows: {missing:?}"
    );
}

#[test]
fn raid_and_arena_lod_addons_are_panel_parity_rows() {
    let audit_path = repo_root().join("docs/baselines/mists-lod-audit.md");
    let audit = std::fs::read_to_string(&audit_path).expect("failed to read Mists LoD audit");

    assert!(
        audit.contains("| Blizzard_ArenaUI | Added |"),
        "Blizzard_ArenaUI should be promoted into the panel parity matrix"
    );
    assert!(
        audit.contains("| Blizzard_RaidUI | Added |"),
        "Blizzard_RaidUI should be promoted into the panel parity matrix"
    );
}

fn audited_lod_addons(audit: &str) -> BTreeSet<&str> {
    audit
        .lines()
        .filter_map(|line| {
            let mut columns = line.split('|').map(str::trim);
            columns.next()?;
            let addon_name = columns.next()?;
            addon_name.starts_with("Blizzard_").then_some(addon_name)
        })
        .collect()
}

fn addon_has_load_on_demand_toc(addon_dir: &std::path::Path) -> bool {
    let Ok(entries) = std::fs::read_dir(addon_dir) else {
        return false;
    };

    entries.filter_map(Result::ok).any(|entry| {
        let path = entry.path();
        path.extension().is_some_and(|ext| ext == "toc")
            && std::fs::read_to_string(&path)
                .map(|toc| toc.contains("## LoadOnDemand: 1"))
                .unwrap_or(false)
    })
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

fn release_proof_artifact_test_dir(name: &str) -> PathBuf {
    repo_root()
        .join("target")
        .join("mists-release-proof-artifact-tests")
        .join(format!("{name}-{}", std::process::id()))
}

fn workflow_job_block(workflow: &str, job_name: &str) -> String {
    let job_header = format!("  {job_name}:");
    let mut in_job = false;
    let mut lines = Vec::new();

    for line in workflow.lines() {
        if line == job_header {
            in_job = true;
            lines.push(line);
            continue;
        }
        if in_job && starts_next_workflow_job(line) {
            break;
        }
        if in_job {
            lines.push(line);
        }
    }

    assert!(in_job, "workflow should define {job_name} job");
    lines.join("\n")
}

fn starts_next_workflow_job(line: &str) -> bool {
    let is_job_indent = line.starts_with("  ") && !line.starts_with("    ");
    let has_job_name = !line.trim().is_empty();
    is_job_indent && has_job_name
}

fn run_release_proof_artifact_validation(out_dir: &std::path::Path) -> std::process::Output {
    Command::new(repo_root().join("scripts/ci-mists-release-proof.sh"))
        .arg("--validate-artifacts-only")
        .arg("--out-dir")
        .arg(out_dir)
        .current_dir(repo_root())
        .output()
        .expect("failed to run Mists release proof artifact validation")
}

fn create_release_proof_artifact_fixture(out_dir: &std::path::Path) {
    create_required_lane_logs(out_dir);
    write_file(out_dir.join("mists-release-lua-errors.json"), "[]\n");

    let slugs = mists_panel_slugs();
    create_panel_matrix_fixture(&out_dir.join("panel-parity"), &slugs);
    create_panel_matrix_fixture(&out_dir.join("panel-parity-with-saved-vars"), &slugs);

    let addons = mists_addon_names();
    create_addon_panel_matrix_fixture(&out_dir.join("addon-panel-parity"), &addons, &slugs);
    create_addon_panel_matrix_fixture(
        &out_dir.join("addon-panel-parity-with-saved-vars"),
        &addons,
        &slugs,
    );
}

fn create_required_lane_logs(out_dir: &std::path::Path) {
    for log_name in [
        "build-release.log",
        "zero-lua-errors.log",
        "installed-addon-matrix.log",
        "panel-parity-and-visual-comparison.log",
        "installed-addon-panel-matrix.log",
        "panel-parity-with-saved-vars.log",
        "installed-addon-panel-matrix-with-saved-vars.log",
        "live-gui-smoke.log",
        "interaction-audit.log",
        "artifact-completeness.log",
    ] {
        write_file(out_dir.join("logs").join(log_name), "ok\n");
    }
}

fn create_addon_panel_matrix_fixture(root: &std::path::Path, addons: &[String], slugs: &[String]) {
    for addon in addons {
        create_panel_matrix_fixture(&root.join(addon), slugs);
    }
}

fn create_panel_matrix_fixture(root: &std::path::Path, slugs: &[String]) {
    for slug in slugs {
        let panel_dir = root.join(slug);
        write_file(panel_dir.join("screenshot.webp"), "webp");
        write_file(panel_dir.join("dump-tree.txt"), "frame tree\n");
        write_file(panel_dir.join("lua-errors.json"), "[]\n");
    }
}

fn mists_panel_slugs() -> Vec<String> {
    let baseline_path = repo_root().join("docs/baselines/mists-panels.md");
    let baseline =
        std::fs::read_to_string(&baseline_path).expect("failed to read Mists panel baseline");

    baseline
        .lines()
        .filter(|line| line.starts_with('|') && line.contains(" | Pass | "))
        .filter_map(|line| {
            line.split("target/mists-panel-parity/")
                .nth(1)
                .and_then(|path| path.split('/').next())
        })
        .map(str::to_owned)
        .collect()
}

fn mists_addon_names() -> Vec<String> {
    let manifest_path = repo_root().join("tools/classic-addon-manifest.tsv");
    let manifest = std::fs::read_to_string(&manifest_path).expect("failed to read addon manifest");

    manifest
        .lines()
        .filter_map(|line| {
            let mut columns = line.split('\t');
            let name = columns.next()?;
            let profile = columns.next()?;
            is_mists_addon_row(name, profile).then_some(name.to_owned())
        })
        .collect()
}

fn bounded_saved_vars_addon_sample_names() -> BTreeSet<String> {
    let index_path = repo_root().join("docs/baselines/mists-release-proof.md");
    let index = std::fs::read_to_string(&index_path)
        .expect("failed to read Mists release-proof artifact index");

    index
        .lines()
        .filter_map(saved_vars_sample_addon_name)
        .collect()
}

fn saved_vars_sample_addon_name(line: &str) -> Option<String> {
    let has_sample_artifact = line.contains("target/mists-local-addon-panel-sample/");
    let is_saved_vars_row = line.contains("` + normal SavedVariables |");
    if !has_sample_artifact || !is_saved_vars_row {
        return None;
    }

    line.strip_prefix("| `")?
        .split('`')
        .next()
        .map(str::to_owned)
}

fn is_mists_addon_row(name: &str, profile: &str) -> bool {
    let is_data_row = !name.starts_with('#') && !name.is_empty() && name != "name";
    let is_mists_profile = profile == "mists";
    is_data_row && is_mists_profile
}

fn write_file(path: PathBuf, contents: &str) {
    let parent = path.parent().expect("fixture path should have a parent");
    std::fs::create_dir_all(parent).expect("failed to create fixture directory");
    std::fs::write(path, contents).expect("failed to write fixture file");
}
