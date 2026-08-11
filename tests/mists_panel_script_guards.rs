use std::path::PathBuf;

fn repo_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
}

fn read_repo_file(path: &str) -> String {
    std::fs::read_to_string(repo_root().join(path)).expect("failed to read repository file")
}

fn cargo_profile<'a>(manifest: &'a str, profile: &str) -> &'a str {
    let header = format!("[profile.{profile}]");
    let section_start = manifest
        .find(&header)
        .unwrap_or_else(|| panic!("missing {header}"))
        + header.len();
    let section = &manifest[section_start..];

    section
        .split_once("\n[")
        .map_or(section, |(profile, _)| profile)
}

#[test]
fn mists_panel_scripts_inherit_incremental_dev_profile() {
    let manifest = read_repo_file("Cargo.toml");
    let dev_profile = cargo_profile(&manifest, "dev");
    let release_profile = cargo_profile(&manifest, "release");

    assert!(
        dev_profile
            .lines()
            .any(|line| line.trim() == "incremental = true"),
        "development profile should enable incremental compilation"
    );
    assert!(
        !release_profile
            .lines()
            .any(|line| line.trim_start().starts_with("incremental =")),
        "release profile should preserve Cargo's incremental default"
    );

    for script_path in [
        "scripts/mists-panel-parity.sh",
        "scripts/test-mists-addon-panels.sh",
    ] {
        let script = read_repo_file(script_path);

        assert!(
            script.contains("MISTS_CARGO_TARGET_DIR"),
            "{script_path} should allow a Mists-specific Cargo target override"
        );
        assert!(
            script.contains("wow-ui-sim/cargo-targets/mists-panel-parity"),
            "{script_path} should default scripted Mists builds outside repo target/"
        );
        assert!(
            !script.contains("CARGO_INCREMENTAL"),
            "{script_path} should preserve Cargo profile and caller environment precedence"
        );
    }
}

#[test]
fn installed_addon_panel_runner_separates_exit_and_interrupt_cleanup() {
    let script = read_repo_file("scripts/test-mists-addon-panels.sh");

    assert!(
        script.contains("cleanup_active_addon_on_exit"),
        "addon panel runner should have a quiet EXIT cleanup path"
    );
    assert!(
        script.contains("cleanup_active_addon_on_interrupt"),
        "addon panel runner should have an interrupt-specific cleanup path"
    );
    assert!(
        !script.contains("trap cleanup_active_addon EXIT"),
        "normal EXIT cleanup should not use the interrupt/error-reporting cleanup path"
    );
    assert!(
        !script.contains("trap 'cleanup_active_addon; exit 130' INT TERM"),
        "INT/TERM cleanup should use the interrupt-specific path"
    );
}

#[test]
fn mists_panel_runner_fails_on_texture_manager_load_errors() {
    let script = read_repo_file("scripts/mists-panel-parity.sh");

    assert!(
        script.contains("fail_if_runtime_log_error"),
        "panel runner should centralize runtime stderr/stdout failure checks"
    );
    assert!(
        script.contains("TexMgr") && script.contains("Load error"),
        "panel runner should fail when a panel emits texture-manager load errors"
    );
}
