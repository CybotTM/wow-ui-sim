#![cfg(target_os = "linux")]

use std::io::Write;
use std::path::{Path, PathBuf};
use std::process::{Command, Output};
use std::thread;
use std::time::{Duration, Instant};
use tempfile::TempDir;

const FIXTURE_MODE_ENV: &str = "WOW_SIM_TIMEOUT_REEXEC_FIXTURE";
const SIBLING_MARKER_ENV: &str = "WOW_SIM_TIMEOUT_REEXEC_SIBLING_MARKER";
const DESCENDANT_PID_ENV: &str = "WOW_SIM_TIMEOUT_REEXEC_DESCENDANT_PID";
const PROCESS_DEATH_WAIT: Duration = Duration::from_secs(2);

#[test]
fn panic_failure_is_aggregated_and_later_sibling_runs() {
    let temp = TempDir::new().expect("create panic fixture tempdir");
    let sibling_marker = temp.path().join("panic-sibling-complete");
    let output = run_fixture(
        "timeout_reexec::fixtures::panic_case::",
        "panic",
        &sibling_marker,
        None,
    );
    let stdout = stdout(&output);
    let stderr = stderr(&output);

    assert!(!output.status.success(), "panic fixture must report one failure");
    assert!(
        sibling_marker.exists(),
        "later panic sibling did not execute\nstdout:\n{stdout}\nstderr:\n{stderr}"
    );
    assert!(stdout.contains("timeout reexec panic stdout marker"));
    assert!(stderr.contains("timeout reexec panic stderr marker"));
    assert!(stdout.contains("timeout_reexec::fixtures::panic_case::b_records_completion ... ok"));
    assert!(
        stdout.contains("test result: FAILED. 1 passed; 1 failed; 0 ignored; 0 measured;")
    );
    assert_final_failure_list(
        &stdout,
        "timeout_reexec::fixtures::panic_case::a_panics",
    );
}

#[test]
fn timeout_kills_descendant_and_later_sibling_runs() {
    let temp = TempDir::new().expect("create timeout fixture tempdir");
    let sibling_marker = temp.path().join("timeout-sibling-complete");
    let descendant_pid_path = temp.path().join("descendant.pid");
    let output = run_fixture(
        "timeout_reexec::fixtures::timeout_case::",
        "timeout",
        &sibling_marker,
        Some(&descendant_pid_path),
    );
    let stdout = stdout(&output);
    let stderr = stderr(&output);
    let descendant_pid = read_pid(&descendant_pid_path);
    let descendant_disappeared = wait_for_process_exit(descendant_pid);
    if !descendant_disappeared {
        kill_process(descendant_pid);
    }

    assert!(!output.status.success(), "timeout fixture must report one failure");
    assert!(
        sibling_marker.exists(),
        "later timeout sibling did not execute\nstdout:\n{stdout}\nstderr:\n{stderr}"
    );
    assert!(stdout.contains("timeout reexec timeout stdout marker"));
    assert!(stderr.contains("timeout reexec timeout stderr marker"));
    assert!(stdout.contains("timeout_reexec::fixtures::timeout_case::b_records_completion ... ok"));
    assert!(
        stdout.contains("test result: FAILED. 1 passed; 1 failed; 0 ignored; 0 measured;")
    );
    assert!(
        stderr.contains("timed out after 1s"),
        "timeout reason was not forwarded\nstdout:\n{stdout}\nstderr:\n{stderr}"
    );
    assert!(
        descendant_disappeared,
        "timeout descendant {descendant_pid} survived process-group cleanup"
    );
    assert_final_failure_list(
        &stdout,
        "timeout_reexec::fixtures::timeout_case::a_times_out_with_descendant",
    );
}

#[test]
fn nested_guards_execute_each_closure_once() {
    let temp = TempDir::new().expect("create nested fixture tempdir");
    let marker = temp.path().join("nested-closures");
    let output = run_fixture(
        "timeout_reexec::fixtures::nested_case::",
        "nested",
        &marker,
        None,
    );
    let stdout = stdout(&output);
    let stderr = stderr(&output);

    assert!(
        output.status.success(),
        "nested timeout guards failed\nstdout:\n{stdout}\nstderr:\n{stderr}"
    );
    assert_eq!(
        std::fs::read_to_string(&marker).expect("read nested closure marker"),
        "first\nsecond\n"
    );
}

fn run_fixture(
    filter: &str,
    mode: &str,
    sibling_marker: &Path,
    descendant_pid_path: Option<&Path>,
) -> Output {
    let mut command = Command::new(std::env::current_exe().expect("resolve integration test binary"));
    command
        .args([filter, "--test-threads=1", "--nocapture"])
        .env(FIXTURE_MODE_ENV, mode)
        .env(SIBLING_MARKER_ENV, sibling_marker);
    if let Some(path) = descendant_pid_path {
        command.env(DESCENDANT_PID_ENV, path);
    }
    command.output().expect("run timeout re-exec fixture")
}

fn assert_final_failure_list(stdout: &str, expected_failure: &str) {
    let failures = stdout
        .rsplit_once("failures:\n")
        .map(|(_, failures)| failures)
        .expect("normal libtest failures section");
    assert!(failures.contains(expected_failure));
    assert_eq!(
        failures
            .lines()
            .filter(|line| line.trim_start().starts_with("timeout_reexec::fixtures::"))
            .count(),
        1,
        "expected exactly one named fixture failure in final summary:\n{failures}"
    );
}

fn stdout(output: &Output) -> String {
    String::from_utf8_lossy(&output.stdout).into_owned()
}

fn stderr(output: &Output) -> String {
    String::from_utf8_lossy(&output.stderr).into_owned()
}

fn read_pid(path: &Path) -> libc::pid_t {
    std::fs::read_to_string(path)
        .unwrap_or_else(|error| panic!("read descendant pid from {}: {error}", path.display()))
        .trim()
        .parse()
        .expect("parse descendant pid")
}

fn wait_for_process_exit(pid: libc::pid_t) -> bool {
    let deadline = Instant::now() + PROCESS_DEATH_WAIT;
    while Instant::now() < deadline {
        if !process_exists(pid) {
            return true;
        }
        thread::sleep(Duration::from_millis(10));
    }
    false
}

fn process_exists(pid: libc::pid_t) -> bool {
    let result = unsafe { libc::kill(pid, 0) };
    result == 0 || std::io::Error::last_os_error().raw_os_error() != Some(libc::ESRCH)
}

fn kill_process(pid: libc::pid_t) {
    unsafe {
        libc::kill(pid, libc::SIGKILL);
    }
}

mod fixtures {
    use super::*;

    pub(super) mod panic_case {
        use super::*;

        #[test]
        fn a_panics() {
            if fixture_mode() != Some("panic") {
                return;
            }
            crate::common::with_timeout(5, || {
                println!("timeout reexec panic stdout marker");
                eprintln!("timeout reexec panic stderr marker");
                panic!("deliberate timeout reexec panic");
            });
        }

        #[test]
        fn b_records_completion() {
            if fixture_mode() != Some("panic") {
                return;
            }
            record_sibling_completion();
        }
    }

    pub(super) mod timeout_case {
        use super::*;

        #[test]
        fn a_times_out_with_descendant() {
            if fixture_mode() != Some("timeout") {
                return;
            }
            crate::common::with_timeout(1, || {
                println!("timeout reexec timeout stdout marker");
                eprintln!("timeout reexec timeout stderr marker");
                let descendant = Command::new("sleep")
                    .arg("60")
                    .spawn()
                    .expect("spawn timeout descendant");
                let pid_path = PathBuf::from(
                    std::env::var_os(DESCENDANT_PID_ENV)
                        .expect("timeout descendant pid path environment"),
                );
                std::fs::write(&pid_path, descendant.id().to_string())
                    .expect("write timeout descendant pid");
                drop(descendant);
                thread::sleep(Duration::from_secs(30));
            });
        }

        #[test]
        fn b_records_completion() {
            if fixture_mode() != Some("timeout") {
                return;
            }
            record_sibling_completion();
        }
    }

    pub(super) mod nested_case {
        use super::*;

        #[test]
        fn nested_guards_run_both_closures() {
            if fixture_mode() != Some("nested") {
                return;
            }
            crate::common::with_timeout(5, || {
                record_nested_closure("first");
                crate::common::with_timeout(5, || record_nested_closure("second"));
            });
        }
    }

    fn fixture_mode() -> Option<&'static str> {
        match std::env::var(FIXTURE_MODE_ENV).as_deref() {
            Ok("panic") => Some("panic"),
            Ok("timeout") => Some("timeout"),
            Ok("nested") => Some("nested"),
            _ => None,
        }
    }

    fn record_sibling_completion() {
        let marker = PathBuf::from(
            std::env::var_os(SIBLING_MARKER_ENV).expect("sibling marker path environment"),
        );
        std::fs::write(marker, "complete\n").expect("record later sibling completion");
    }

    fn record_nested_closure(label: &str) {
        let marker = PathBuf::from(
            std::env::var_os(SIBLING_MARKER_ENV).expect("nested closure marker environment"),
        );
        let mut marker = std::fs::OpenOptions::new()
            .create(true)
            .append(true)
            .open(marker)
            .expect("open nested closure marker");
        writeln!(marker, "{label}").expect("record nested closure");
    }
}
