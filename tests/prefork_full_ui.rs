#[cfg(not(target_os = "linux"))]
compile_error!("prefork_full_ui is Linux-only");

#[path = "common/prefork.rs"]
mod prefork;

use prefork::{Case, Config};
use std::env;
use std::fs::{File, OpenOptions};
use std::io::{Read, Seek, SeekFrom, Write};
use std::path::{Path, PathBuf};
use std::process::{Command, ExitCode, Output};
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::{Arc, Barrier};
use std::time::{Duration, Instant};
use tempfile::TempDir;

const DRIVER_MODE_ENV: &str = "PREFORK_CONFORMANCE_DRIVER";
const TREE_CHILD_MODE_ENV: &str = "PREFORK_CONFORMANCE_TREE_CHILD";
const START_THREAD_ENV: &str = "PREFORK_CONFORMANCE_START_THREAD";
const WORKER_STATE_ENV: &str = "PREFORK_CONFORMANCE_WORKER_STATE";
const TREE_PID_ENV: &str = "PREFORK_CONFORMANCE_TREE_PID";
const DRIVER_TIMEOUT_MS_ENV: &str = "PREFORK_CONFORMANCE_TIMEOUT_MS";
const DRIVER_TERM_GRACE_MS_ENV: &str = "PREFORK_CONFORMANCE_TERM_GRACE_MS";
const WORKER_HOLD: Duration = Duration::from_millis(40);
const PROCESS_DEATH_WAIT: Duration = Duration::from_secs(2);

struct ConformanceState {
    executable: PathBuf,
}

struct FixtureState {
    cow_value: AtomicUsize,
}

const CONFORMANCE_CASES: &[Case<ConformanceState>] = &[
    Case::new("conformance::filtering_and_listing", filtering_and_listing),
    Case::new("conformance::copy_on_write", copy_on_write),
    Case::new("conformance::panic_and_capture", panic_and_capture),
    Case::new("conformance::nocapture", nocapture),
    Case::new(
        "conformance::timeout_and_tree_cleanup",
        timeout_and_tree_cleanup,
    ),
    Case::new("conformance::exit_classification", exit_classification),
    Case::new("conformance::bounded_workers", bounded_workers),
    Case::new(
        "conformance::rejects_multithreaded_fork",
        rejects_multithreaded_fork,
    ),
    Case::new("conformance::rejects_bad_arguments", rejects_bad_arguments),
];

const FIXTURE_CASES: &[Case<FixtureState>] = &[
    Case::new("alpha::one", fixture_pass),
    Case::new("alpha::two", fixture_pass),
    Case::new("beta::one", fixture_pass),
    Case::new("cow::mutate", fixture_cow_mutate),
    Case::new("cow::observe", fixture_cow_observe),
    Case::new("output::pass", fixture_output_pass),
    Case::new("output::panic", fixture_output_panic),
    Case::new("process::signal", fixture_signal),
    Case::new("process::exit", fixture_exit),
    Case::new("process::timeout_tree", fixture_timeout_tree),
    Case::new("worker::00", fixture_worker),
    Case::new("worker::01", fixture_worker),
    Case::new("worker::02", fixture_worker),
    Case::new("worker::03", fixture_worker),
    Case::new("worker::04", fixture_worker),
    Case::new("worker::05", fixture_worker),
    Case::new("worker::06", fixture_worker),
    Case::new("worker::07", fixture_worker),
];

fn main() -> ExitCode {
    if env::var_os(TREE_CHILD_MODE_ENV).is_some() {
        run_tree_child();
    }
    if env::var_os(DRIVER_MODE_ENV).is_some() {
        return run_fixture_driver();
    }

    let state = ConformanceState {
        executable: env::current_exe().expect("resolve prefork conformance executable"),
    };
    prefork::run(&state, CONFORMANCE_CASES, Config::default())
}

fn run_fixture_driver() -> ExitCode {
    let thread_guard = start_optional_guard_thread();
    let state = FixtureState {
        cow_value: AtomicUsize::new(7),
    };
    let config = Config {
        timeout: duration_from_env(DRIVER_TIMEOUT_MS_ENV, 2_000),
        term_grace: duration_from_env(DRIVER_TERM_GRACE_MS_ENV, 100),
    };
    let result = prefork::run(&state, FIXTURE_CASES, config);
    drop(thread_guard);
    result
}

fn duration_from_env(name: &str, default_ms: u64) -> Duration {
    let milliseconds = env::var(name)
        .ok()
        .map(|value| {
            value
                .parse()
                .expect("duration environment value must be an integer")
        })
        .unwrap_or(default_ms);
    Duration::from_millis(milliseconds)
}

fn start_optional_guard_thread() -> Option<std::thread::JoinHandle<()>> {
    if env::var_os(START_THREAD_ENV).is_none() {
        return None;
    }

    let barrier = Arc::new(Barrier::new(2));
    let child_barrier = Arc::clone(&barrier);
    let handle = std::thread::spawn(move || {
        child_barrier.wait();
        loop {
            std::thread::sleep(Duration::from_secs(1));
        }
    });
    barrier.wait();
    Some(handle)
}

fn filtering_and_listing(state: &ConformanceState) {
    let substring = run_driver(state, ["--list", "alpha"], []);
    assert_success(&substring);
    assert_eq!(
        stdout(&substring),
        "alpha::one: test\nalpha::two: test\n\n2 tests, 0 benchmarks\n"
    );

    let exact = run_driver(state, ["alpha::one", "--exact", "--list"], []);
    assert_success(&exact);
    assert_eq!(stdout(&exact), "alpha::one: test\n\n1 test, 0 benchmarks\n");

    let skipped = run_driver(state, ["--list", "alpha", "--skip=two"], []);
    assert_success(&skipped);
    assert_eq!(
        stdout(&skipped),
        "alpha::one: test\n\n1 test, 0 benchmarks\n"
    );

    let split_skip = run_driver(state, ["--list", "--skip", "alpha"], []);
    assert_success(&split_skip);
    assert!(!stdout(&split_skip).contains("alpha::"));
}

fn copy_on_write(state: &ConformanceState) {
    let output = run_driver(state, ["cow::", "--test-threads", "1", "--nocapture"], []);
    assert_success(&output);
}

fn panic_and_capture(state: &ConformanceState) {
    let output = run_driver(state, ["output::panic", "--exact"], []);
    assert_failure(&output);
    let text = stdout(&output);
    assert!(text.contains("FAILED (panic: deliberate panic text)"));
    assert!(text.contains("---- output::panic stdout ----\nchild stdout marker"));
    assert!(text.contains("---- output::panic stderr ----"));
    assert!(text.contains("child stderr marker"));
}

fn nocapture(state: &ConformanceState) {
    let captured = run_driver(state, ["output::pass", "--exact"], []);
    assert_success(&captured);
    assert!(!stdout(&captured).contains("child stdout marker"));
    assert!(!stderr(&captured).contains("child stderr marker"));

    let inherited = run_driver(state, ["output::pass", "--exact", "--nocapture"], []);
    assert_success(&inherited);
    assert!(stdout(&inherited).contains("child stdout marker"));
    assert!(stderr(&inherited).contains("child stderr marker"));
}

fn timeout_and_tree_cleanup(state: &ConformanceState) {
    let temp = TempDir::new().expect("create process tree temp dir");
    let pid_path = temp.path().join("grandchild.pid");
    let timeout = [
        (TREE_PID_ENV, pid_path.as_os_str()),
        (DRIVER_TIMEOUT_MS_ENV, "120".as_ref()),
    ];
    let output = run_driver_with_os_env(state, ["process::timeout_tree", "--exact"], timeout);
    assert_failure(&output);
    assert!(stdout(&output).contains("FAILED (timeout after 120ms)"));

    let pid: libc::pid_t = std::fs::read_to_string(&pid_path)
        .expect("read grandchild pid")
        .trim()
        .parse()
        .expect("parse grandchild pid");
    assert_process_disappears(pid);
}

fn exit_classification(state: &ConformanceState) {
    let signaled = run_driver(state, ["process::signal", "--exact"], []);
    assert_failure(&signaled);
    assert!(stdout(&signaled).contains("FAILED (signal SIGUSR1)"));

    let exited = run_driver(state, ["process::exit", "--exact"], []);
    assert_failure(&exited);
    assert!(stdout(&exited).contains("FAILED (unexpected exit code 17)"));
}

fn bounded_workers(state: &ConformanceState) {
    assert_worker_limit(
        state,
        ["worker::", "--test-threads=2"],
        [("RUST_TEST_THREADS", "1")],
        2,
    );
    assert_worker_limit(state, ["worker::"], [("RUST_TEST_THREADS", "1")], 1);
    assert_worker_limit(
        state,
        ["worker::"],
        [("RUST_TEST_THREADS", "999")],
        prefork::HARD_MAX_WORKERS,
    );
    assert_worker_limit(state, ["worker::"], [], prefork::DEFAULT_WORKERS);
}

fn rejects_multithreaded_fork(state: &ConformanceState) {
    let output = run_driver(state, ["alpha::one", "--exact"], [(START_THREAD_ENV, "1")]);
    assert_failure(&output);
    assert!(
        stderr(&output).contains(
            "refusing to fork: /proc/self/task contains 2 tasks; exactly one is required"
        )
    );
}

fn rejects_bad_arguments(state: &ConformanceState) {
    let unsupported = run_driver(state, ["--ignored"], []);
    assert_failure(&unsupported);
    assert!(stderr(&unsupported).contains("unsupported argument: --ignored"));

    let conflicting = run_driver(state, ["--test-threads=2", "--test-threads", "3"], []);
    assert_failure(&conflicting);
    assert!(stderr(&conflicting).contains("--test-threads specified more than once"));

    let two_filters = run_driver(state, ["alpha", "beta"], []);
    assert_failure(&two_filters);
    assert!(stderr(&two_filters).contains("only one positional filter is supported"));
}

fn assert_worker_limit<const N: usize, const E: usize>(
    state: &ConformanceState,
    args: [&str; N],
    environment: [(&str, &str); E],
    expected_max: usize,
) {
    let temp = TempDir::new().expect("create worker count temp dir");
    let worker_path = temp.path().join("workers.txt");
    std::fs::write(&worker_path, "0 0").expect("initialize worker count file");
    let mut command = driver_command(state);
    command.args(args).env(WORKER_STATE_ENV, &worker_path);
    command.envs(environment);
    let output = command.output().expect("run worker-count fixture driver");
    assert_success(&output);

    let (_, observed_max) = read_worker_counts(&worker_path);
    assert_eq!(observed_max, expected_max);
}

fn run_driver<const N: usize, const E: usize>(
    state: &ConformanceState,
    args: [&str; N],
    environment: [(&str, &str); E],
) -> Output {
    let mut command = driver_command(state);
    command.args(args).envs(environment);
    command.output().expect("run fixture driver")
}

fn run_driver_with_os_env<const N: usize, const E: usize>(
    state: &ConformanceState,
    args: [&str; N],
    environment: [(&str, &std::ffi::OsStr); E],
) -> Output {
    let mut command = driver_command(state);
    command.args(args).envs(environment);
    command.output().expect("run fixture driver")
}

fn driver_command(state: &ConformanceState) -> Command {
    let mut command = Command::new(&state.executable);
    command
        .env(DRIVER_MODE_ENV, "1")
        .env_remove(START_THREAD_ENV)
        .env_remove(WORKER_STATE_ENV)
        .env_remove(TREE_PID_ENV)
        .env_remove(DRIVER_TIMEOUT_MS_ENV)
        .env_remove(DRIVER_TERM_GRACE_MS_ENV)
        .env_remove("RUST_TEST_THREADS");
    command
}

fn assert_success(output: &Output) {
    assert!(
        output.status.success(),
        "driver failed\nstdout:\n{}\nstderr:\n{}",
        stdout(output),
        stderr(output)
    );
}

fn assert_failure(output: &Output) {
    assert!(!output.status.success(), "driver unexpectedly succeeded");
}

fn stdout(output: &Output) -> String {
    String::from_utf8_lossy(&output.stdout).into_owned()
}

fn stderr(output: &Output) -> String {
    String::from_utf8_lossy(&output.stderr).into_owned()
}

fn assert_process_disappears(pid: libc::pid_t) {
    let deadline = Instant::now() + PROCESS_DEATH_WAIT;
    while Instant::now() < deadline {
        let result = unsafe { libc::kill(pid, 0) };
        if result == -1 && std::io::Error::last_os_error().raw_os_error() == Some(libc::ESRCH) {
            return;
        }
        std::thread::sleep(Duration::from_millis(10));
    }
    panic!("grandchild process {pid} still exists after process-group cleanup");
}

fn fixture_pass(_: &FixtureState) {}

fn fixture_cow_mutate(state: &FixtureState) {
    state.cow_value.store(99, Ordering::SeqCst);
}

fn fixture_cow_observe(state: &FixtureState) {
    assert_eq!(state.cow_value.load(Ordering::SeqCst), 7);
}

fn fixture_output_pass(_: &FixtureState) {
    println!("child stdout marker");
    eprintln!("child stderr marker");
}

fn fixture_output_panic(_: &FixtureState) {
    println!("child stdout marker");
    eprintln!("child stderr marker");
    panic!("deliberate panic text");
}

fn fixture_signal(_: &FixtureState) {
    unsafe {
        libc::raise(libc::SIGUSR1);
    }
}

fn fixture_exit(_: &FixtureState) {
    unsafe {
        libc::_exit(17);
    }
}

fn fixture_timeout_tree(_: &FixtureState) {
    unsafe {
        libc::signal(libc::SIGTERM, libc::SIG_IGN);
    }
    let pid_path = env::var_os(TREE_PID_ENV).expect("tree pid path environment");
    let child = Command::new(env::current_exe().expect("resolve current executable"))
        .env(TREE_CHILD_MODE_ENV, "1")
        .spawn()
        .expect("spawn process-tree grandchild");
    std::fs::write(pid_path, child.id().to_string()).expect("write process-tree grandchild pid");
    loop {
        std::thread::sleep(Duration::from_secs(1));
    }
}

fn run_tree_child() -> ! {
    loop {
        std::thread::sleep(Duration::from_secs(1));
    }
}

fn fixture_worker(_: &FixtureState) {
    let path = PathBuf::from(env::var_os(WORKER_STATE_ENV).expect("worker state path"));
    update_worker_counts(&path, 1);
    std::thread::sleep(WORKER_HOLD);
    update_worker_counts(&path, -1);
}

fn update_worker_counts(path: &Path, delta: isize) {
    let mut file = OpenOptions::new()
        .read(true)
        .write(true)
        .open(path)
        .expect("open worker state file");
    lock_file(&file);
    let (active, observed_max) = read_counts_from_file(&mut file);
    let next_active = active
        .checked_add_signed(delta)
        .expect("valid active worker count");
    let next_max = observed_max.max(next_active);
    write_counts_to_file(&mut file, next_active, next_max);
    unlock_file(&file);
}

fn read_worker_counts(path: &Path) -> (usize, usize) {
    let mut file = File::open(path).expect("open final worker state file");
    read_counts_from_file(&mut file)
}

fn read_counts_from_file(file: &mut File) -> (usize, usize) {
    file.seek(SeekFrom::Start(0))
        .expect("seek worker state file");
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect("read worker state file");
    let mut fields = contents.split_whitespace();
    let active = fields
        .next()
        .expect("active worker field")
        .parse()
        .expect("parse active workers");
    let observed_max = fields
        .next()
        .expect("maximum worker field")
        .parse()
        .expect("parse maximum workers");
    (active, observed_max)
}

fn write_counts_to_file(file: &mut File, active: usize, observed_max: usize) {
    file.set_len(0).expect("truncate worker state file");
    file.seek(SeekFrom::Start(0))
        .expect("rewind worker state file");
    write!(file, "{active} {observed_max}").expect("write worker state file");
    file.flush().expect("flush worker state file");
}

fn lock_file(file: &File) {
    let result = unsafe { libc::flock(std::os::fd::AsRawFd::as_raw_fd(file), libc::LOCK_EX) };
    assert_eq!(result, 0, "lock worker state file");
}

fn unlock_file(file: &File) {
    let result = unsafe { libc::flock(std::os::fd::AsRawFd::as_raw_fd(file), libc::LOCK_UN) };
    assert_eq!(result, 0, "unlock worker state file");
}
