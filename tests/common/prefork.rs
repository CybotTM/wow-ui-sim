use std::env;
use std::ffi::OsString;
use std::fs;
use std::io::{self, Write};
use std::os::fd::RawFd;
use std::panic::{self, AssertUnwindSafe};
use std::process::ExitCode;
use std::time::{Duration, Instant};

pub const DEFAULT_WORKERS: usize = 2;
pub const HARD_MAX_WORKERS: usize = 4;

const CHILD_POLL_INTERVAL: Duration = Duration::from_millis(2);
const MAX_PANIC_TEXT_BYTES: usize = 4_096;
const CHILD_PASS: u8 = 0;
const CHILD_PANIC: u8 = 1;

pub struct Case<S> {
    pub name: &'static str,
    pub test: fn(&S),
}

impl<S> Case<S> {
    pub const fn new(name: &'static str, test: fn(&S)) -> Self {
        Self { name, test }
    }
}

#[derive(Clone, Copy)]
pub struct Config {
    pub timeout: Duration,
    pub term_grace: Duration,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            timeout: Duration::from_secs(30),
            term_grace: Duration::from_millis(100),
        }
    }
}

#[derive(Default)]
struct Options {
    exact: bool,
    list: bool,
    nocapture: bool,
    filter: Option<String>,
    skips: Vec<String>,
    test_threads: Option<usize>,
}

struct SelectedCase<'a, S> {
    case: &'a Case<S>,
}

struct Pipe {
    read_fd: RawFd,
    write_fd: RawFd,
}

struct RunningChild<'a, S> {
    selected: SelectedCase<'a, S>,
    pid: libc::pid_t,
    started: Instant,
    termination: Termination,
    stdout_fd: Option<RawFd>,
    stderr_fd: Option<RawFd>,
    status_fd: RawFd,
    stdout: Vec<u8>,
    stderr: Vec<u8>,
}

#[derive(Clone, Copy)]
enum Termination {
    Running,
    TermSent(Instant),
    KillSent,
}

struct ChildResult {
    passed: bool,
    detail: String,
}

pub fn run<S>(state: &S, cases: &[Case<S>], config: Config) -> ExitCode {
    match run_with_args(state, cases, config, env::args_os().skip(1)) {
        Ok(true) => ExitCode::SUCCESS,
        Ok(false) | Err(()) => ExitCode::FAILURE,
    }
}

fn run_with_args<S>(
    state: &S,
    cases: &[Case<S>],
    config: Config,
    args: impl Iterator<Item = OsString>,
) -> Result<bool, ()> {
    let options = parse_args(args).map_err(report_cli_error)?;
    let selected = select_cases(cases, &options);
    if options.list {
        print_list(&selected);
        return Ok(true);
    }

    let worker_count = resolve_worker_count(&options).map_err(report_cli_error)?;
    run_selected(state, selected, config, worker_count, options.nocapture)
        .map_err(report_runner_error)
}

fn parse_args(args: impl Iterator<Item = OsString>) -> Result<Options, String> {
    let mut options = Options::default();
    let mut args = args;
    while let Some(argument) = args.next() {
        let argument = argument
            .into_string()
            .map_err(|_| "arguments must be valid UTF-8".to_string())?;
        parse_argument(&mut options, &mut args, argument)?;
    }
    if options.exact && options.filter.is_none() {
        return Err("--exact requires a positional filter".to_string());
    }
    Ok(options)
}

fn parse_argument(
    options: &mut Options,
    args: &mut impl Iterator<Item = OsString>,
    argument: String,
) -> Result<(), String> {
    match argument.as_str() {
        "--list" => set_once(&mut options.list, "--list"),
        "--exact" => set_once(&mut options.exact, "--exact"),
        "--nocapture" => set_once(&mut options.nocapture, "--nocapture"),
        "--test-threads" => {
            let value = next_option_value(args, "--test-threads")?;
            set_test_threads(options, &value)
        }
        "--skip" => {
            let value = next_option_value(args, "--skip")?;
            options.skips.push(value);
            Ok(())
        }
        _ => parse_compact_or_positional_argument(options, argument),
    }
}

fn parse_compact_or_positional_argument(
    options: &mut Options,
    argument: String,
) -> Result<(), String> {
    if let Some(value) = argument.strip_prefix("--test-threads=") {
        return set_test_threads(options, value);
    }
    if let Some(value) = argument.strip_prefix("--skip=") {
        require_nonempty(value, "--skip")?;
        options.skips.push(value.to_string());
        return Ok(());
    }
    if argument.starts_with('-') {
        return Err(format!("unsupported argument: {argument}"));
    }
    set_filter(options, argument)
}

fn set_once(value: &mut bool, name: &str) -> Result<(), String> {
    if *value {
        return Err(format!("{name} specified more than once"));
    }
    *value = true;
    Ok(())
}

fn next_option_value(
    args: &mut impl Iterator<Item = OsString>,
    name: &str,
) -> Result<String, String> {
    args.next()
        .ok_or_else(|| format!("{name} requires a value"))?
        .into_string()
        .map_err(|_| format!("{name} value must be valid UTF-8"))
}

fn set_test_threads(options: &mut Options, value: &str) -> Result<(), String> {
    if options.test_threads.is_some() {
        return Err("--test-threads specified more than once".to_string());
    }
    options.test_threads = Some(parse_worker_count(value, "--test-threads")?);
    Ok(())
}

fn set_filter(options: &mut Options, filter: String) -> Result<(), String> {
    if options.filter.is_some() {
        return Err("only one positional filter is supported".to_string());
    }
    options.filter = Some(filter);
    Ok(())
}

fn require_nonempty(value: &str, name: &str) -> Result<(), String> {
    if value.is_empty() {
        return Err(format!("{name} requires a non-empty value"));
    }
    Ok(())
}

fn parse_worker_count(value: &str, source: &str) -> Result<usize, String> {
    let count = value
        .parse::<usize>()
        .map_err(|_| format!("{source} must be a positive integer"))?;
    if count == 0 {
        return Err(format!("{source} must be a positive integer"));
    }
    Ok(count.min(HARD_MAX_WORKERS))
}

fn select_cases<'a, S>(cases: &'a [Case<S>], options: &Options) -> Vec<SelectedCase<'a, S>> {
    cases
        .iter()
        .filter(|case| matches_filter(case.name, options))
        .filter(|case| !options.skips.iter().any(|skip| case.name.contains(skip)))
        .map(|case| SelectedCase { case })
        .collect()
}

fn matches_filter(name: &str, options: &Options) -> bool {
    let Some(filter) = options.filter.as_deref() else {
        return true;
    };
    if options.exact {
        name == filter
    } else {
        name.contains(filter)
    }
}

fn print_list<S>(selected: &[SelectedCase<'_, S>]) {
    for selected_case in selected {
        println!("{}: test", selected_case.case.name);
    }
    println!();
    let noun = if selected.len() == 1 { "test" } else { "tests" };
    println!("{} {noun}, 0 benchmarks", selected.len());
}

fn resolve_worker_count(options: &Options) -> Result<usize, String> {
    if let Some(count) = options.test_threads {
        return Ok(count);
    }
    let Some(value) = env::var_os("RUST_TEST_THREADS") else {
        return Ok(DEFAULT_WORKERS);
    };
    let value = value
        .into_string()
        .map_err(|_| "RUST_TEST_THREADS must be valid UTF-8".to_string())?;
    parse_worker_count(&value, "RUST_TEST_THREADS")
}

fn run_selected<S>(
    state: &S,
    selected: Vec<SelectedCase<'_, S>>,
    config: Config,
    worker_count: usize,
    nocapture: bool,
) -> Result<bool, String> {
    println!("running {} tests", selected.len());
    let total = selected.len();
    let mut pending = selected.into_iter();
    let mut running = Vec::new();
    let mut passed = 0;
    let mut failed = 0;

    loop {
        while running.len() < worker_count {
            let Some(selected_case) = pending.next() else {
                break;
            };
            running.push(spawn_child(state, selected_case, nocapture)?);
        }
        if running.is_empty() {
            break;
        }
        collect_finished_children(&mut running, config, &mut passed, &mut failed)?;
        std::thread::sleep(CHILD_POLL_INTERVAL);
    }

    println!();
    let outcome = if failed == 0 { "ok" } else { "FAILED" };
    println!("test result: {outcome}. {passed} passed; {failed} failed; {total} total");
    Ok(failed == 0)
}

fn spawn_child<'a, S>(
    state: &S,
    selected: SelectedCase<'a, S>,
    nocapture: bool,
) -> Result<RunningChild<'a, S>, String> {
    let status_pipe = create_pipe()?;
    let stdout_pipe = (!nocapture).then(create_pipe).transpose()?;
    let stderr_pipe = (!nocapture).then(create_pipe).transpose()?;

    assert_single_threaded_parent()?;
    let pid = unsafe { libc::fork() };
    if pid == -1 {
        close_pipe(status_pipe);
        stdout_pipe.map(close_pipe);
        stderr_pipe.map(close_pipe);
        return Err(format!("fork failed: {}", io::Error::last_os_error()));
    }
    if pid == 0 {
        run_child(state, &selected, status_pipe, stdout_pipe, stderr_pipe);
    }

    prepare_parent_child(pid, status_pipe, stdout_pipe, stderr_pipe, selected)
}

fn prepare_parent_child<'a, S>(
    pid: libc::pid_t,
    status_pipe: Pipe,
    stdout_pipe: Option<Pipe>,
    stderr_pipe: Option<Pipe>,
    selected: SelectedCase<'a, S>,
) -> Result<RunningChild<'a, S>, String> {
    close_fd(status_pipe.write_fd);
    let stdout_fd = prepare_capture_pipe(stdout_pipe)?;
    let stderr_fd = prepare_capture_pipe(stderr_pipe)?;
    set_child_process_group(pid)?;
    Ok(RunningChild {
        selected,
        pid,
        started: Instant::now(),
        termination: Termination::Running,
        stdout_fd,
        stderr_fd,
        status_fd: status_pipe.read_fd,
        stdout: Vec::new(),
        stderr: Vec::new(),
    })
}

fn run_child<S>(
    state: &S,
    selected: &SelectedCase<'_, S>,
    status_pipe: Pipe,
    stdout_pipe: Option<Pipe>,
    stderr_pipe: Option<Pipe>,
) -> ! {
    close_fd(status_pipe.read_fd);
    configure_child_process_group(status_pipe.write_fd);
    redirect_child_output(stdout_pipe, libc::STDOUT_FILENO, status_pipe.write_fd);
    redirect_child_output(stderr_pipe, libc::STDERR_FILENO, status_pipe.write_fd);

    let result = panic::catch_unwind(AssertUnwindSafe(|| (selected.case.test)(state)));
    flush_child_output();
    match result {
        Ok(()) => write_child_status_and_exit(status_pipe.write_fd, CHILD_PASS, "", 0),
        Err(payload) => {
            let panic_text = panic_payload_text(payload);
            write_child_status_and_exit(status_pipe.write_fd, CHILD_PANIC, &panic_text, 101)
        }
    }
}

fn configure_child_process_group(status_fd: RawFd) {
    let result = unsafe { libc::setpgid(0, 0) };
    if result == -1 {
        let detail = format!("setpgid failed: {}", io::Error::last_os_error());
        write_child_status_and_exit(status_fd, CHILD_PANIC, &detail, 101);
    }
}

fn redirect_child_output(pipe: Option<Pipe>, target_fd: RawFd, status_fd: RawFd) {
    let Some(pipe) = pipe else {
        return;
    };
    close_fd(pipe.read_fd);
    let result = unsafe { libc::dup2(pipe.write_fd, target_fd) };
    if result == -1 {
        let detail = format!("dup2 failed: {}", io::Error::last_os_error());
        write_child_status_and_exit(status_fd, CHILD_PANIC, &detail, 101);
    }
    if pipe.write_fd != target_fd {
        close_fd(pipe.write_fd);
    }
}

fn flush_child_output() {
    let _ = io::stdout().flush();
    let _ = io::stderr().flush();
}

fn panic_payload_text(payload: Box<dyn std::any::Any + Send>) -> String {
    let text = if let Some(text) = payload.downcast_ref::<&str>() {
        (*text).to_string()
    } else if let Some(text) = payload.downcast_ref::<String>() {
        text.clone()
    } else {
        "non-string panic payload".to_string()
    };
    text.chars().take(MAX_PANIC_TEXT_BYTES).collect()
}

fn write_child_status_and_exit(status_fd: RawFd, kind: u8, detail: &str, code: i32) -> ! {
    let mut status = Vec::with_capacity(detail.len() + 1);
    status.push(kind);
    status.extend_from_slice(detail.as_bytes());
    write_all_fd(status_fd, &status);
    close_fd(status_fd);
    unsafe {
        libc::_exit(code);
    }
}

fn write_all_fd(fd: RawFd, mut bytes: &[u8]) {
    while !bytes.is_empty() {
        let written = unsafe { libc::write(fd, bytes.as_ptr().cast(), bytes.len()) };
        if written > 0 {
            bytes = &bytes[written as usize..];
            continue;
        }
        if written == -1 && io::Error::last_os_error().kind() == io::ErrorKind::Interrupted {
            continue;
        }
        return;
    }
}

fn collect_finished_children<S>(
    running: &mut Vec<RunningChild<'_, S>>,
    config: Config,
    passed: &mut usize,
    failed: &mut usize,
) -> Result<(), String> {
    let mut index = 0;
    while index < running.len() {
        drain_child_output(&mut running[index])?;
        enforce_timeout(&mut running[index], config)?;
        let wait_status = poll_child(running[index].pid)?;
        let Some(wait_status) = wait_status else {
            index += 1;
            continue;
        };

        let mut child = running.swap_remove(index);
        drain_child_output(&mut child)?;
        let result = classify_child(&mut child, wait_status, config.timeout)?;
        report_child(&child, &result);
        if result.passed {
            *passed += 1;
        } else {
            *failed += 1;
        }
        close_child_fds(&mut child);
    }
    Ok(())
}

fn drain_child_output<S>(child: &mut RunningChild<'_, S>) -> Result<(), String> {
    if let Some(fd) = child.stdout_fd {
        drain_fd(fd, &mut child.stdout)?;
    }
    if let Some(fd) = child.stderr_fd {
        drain_fd(fd, &mut child.stderr)?;
    }
    Ok(())
}

fn drain_fd(fd: RawFd, destination: &mut Vec<u8>) -> Result<(), String> {
    let mut buffer = [0_u8; 4_096];
    loop {
        let read = unsafe { libc::read(fd, buffer.as_mut_ptr().cast(), buffer.len()) };
        if read > 0 {
            destination.extend_from_slice(&buffer[..read as usize]);
            continue;
        }
        if read == 0 {
            return Ok(());
        }
        let error = io::Error::last_os_error();
        match error.kind() {
            io::ErrorKind::WouldBlock => return Ok(()),
            io::ErrorKind::Interrupted => continue,
            _ => return Err(format!("read child output failed: {error}")),
        }
    }
}

fn enforce_timeout<S>(child: &mut RunningChild<'_, S>, config: Config) -> Result<(), String> {
    match child.termination {
        Termination::Running if child.started.elapsed() >= config.timeout => {
            signal_process_group(child.pid, libc::SIGTERM)?;
            child.termination = Termination::TermSent(Instant::now());
        }
        Termination::TermSent(sent_at) if sent_at.elapsed() >= config.term_grace => {
            signal_process_group(child.pid, libc::SIGKILL)?;
            child.termination = Termination::KillSent;
        }
        Termination::Running | Termination::TermSent(_) | Termination::KillSent => {}
    }
    Ok(())
}

fn signal_process_group(pid: libc::pid_t, signal: i32) -> Result<(), String> {
    let result = unsafe { libc::kill(-pid, signal) };
    if result == 0 {
        return Ok(());
    }
    let error = io::Error::last_os_error();
    if error.raw_os_error() == Some(libc::ESRCH) {
        return Ok(());
    }
    Err(format!("signal process group {pid} failed: {error}"))
}

fn poll_child(pid: libc::pid_t) -> Result<Option<i32>, String> {
    let mut status = 0;
    let result = unsafe { libc::waitpid(pid, &mut status, libc::WNOHANG) };
    if result == 0 {
        return Ok(None);
    }
    if result == pid {
        return Ok(Some(status));
    }
    Err(format!(
        "waitpid for {pid} failed: {}",
        io::Error::last_os_error()
    ))
}

fn classify_child<S>(
    child: &mut RunningChild<'_, S>,
    wait_status: i32,
    timeout: Duration,
) -> Result<ChildResult, String> {
    if !matches!(child.termination, Termination::Running) {
        return Ok(ChildResult {
            passed: false,
            detail: format!("timeout after {}ms", timeout.as_millis()),
        });
    }

    let status = read_status(child.status_fd)?;
    if status.first() == Some(&CHILD_PANIC) {
        let panic_text = String::from_utf8_lossy(&status[1..]);
        return Ok(ChildResult {
            passed: false,
            detail: format!("panic: {panic_text}"),
        });
    }
    if libc::WIFSIGNALED(wait_status) {
        let signal = libc::WTERMSIG(wait_status);
        return Ok(ChildResult {
            passed: false,
            detail: format!("signal {}", signal_name(signal)),
        });
    }
    if libc::WIFEXITED(wait_status) {
        let code = libc::WEXITSTATUS(wait_status);
        let passed = code == 0 && status.first() == Some(&CHILD_PASS);
        let detail = if passed {
            String::new()
        } else {
            format!("unexpected exit code {code}")
        };
        return Ok(ChildResult { passed, detail });
    }
    Ok(ChildResult {
        passed: false,
        detail: "unclassified child status".to_string(),
    })
}

fn read_status(fd: RawFd) -> Result<Vec<u8>, String> {
    let mut status = Vec::new();
    drain_fd(fd, &mut status)?;
    Ok(status)
}

fn signal_name(signal: i32) -> String {
    match signal {
        libc::SIGTERM => "SIGTERM".to_string(),
        libc::SIGKILL => "SIGKILL".to_string(),
        libc::SIGUSR1 => "SIGUSR1".to_string(),
        _ => signal.to_string(),
    }
}

fn report_child<S>(child: &RunningChild<'_, S>, result: &ChildResult) {
    if result.passed {
        println!("test {} ... ok", child.selected.case.name);
        return;
    }
    println!(
        "test {} ... FAILED ({})",
        child.selected.case.name, result.detail
    );
    print_capture(child.selected.case.name, "stdout", &child.stdout);
    print_capture(child.selected.case.name, "stderr", &child.stderr);
}

fn print_capture(test_name: &str, stream_name: &str, bytes: &[u8]) {
    if bytes.is_empty() {
        return;
    }
    println!("---- {test_name} {stream_name} ----");
    let text = String::from_utf8_lossy(bytes);
    print!("{text}");
    if !text.ends_with('\n') {
        println!();
    }
}

fn close_child_fds<S>(child: &mut RunningChild<'_, S>) {
    child.stdout_fd.take().map(close_fd);
    child.stderr_fd.take().map(close_fd);
    close_fd(child.status_fd);
}

fn create_pipe() -> Result<Pipe, String> {
    let mut fds = [0; 2];
    let result = unsafe { libc::pipe2(fds.as_mut_ptr(), libc::O_CLOEXEC) };
    if result == -1 {
        return Err(format!("pipe2 failed: {}", io::Error::last_os_error()));
    }
    Ok(Pipe {
        read_fd: fds[0],
        write_fd: fds[1],
    })
}

fn prepare_capture_pipe(pipe: Option<Pipe>) -> Result<Option<RawFd>, String> {
    let Some(pipe) = pipe else {
        return Ok(None);
    };
    close_fd(pipe.write_fd);
    set_nonblocking(pipe.read_fd)?;
    Ok(Some(pipe.read_fd))
}

fn set_nonblocking(fd: RawFd) -> Result<(), String> {
    let flags = unsafe { libc::fcntl(fd, libc::F_GETFL) };
    if flags == -1 {
        return Err(format!(
            "fcntl F_GETFL failed: {}",
            io::Error::last_os_error()
        ));
    }
    let result = unsafe { libc::fcntl(fd, libc::F_SETFL, flags | libc::O_NONBLOCK) };
    if result == -1 {
        return Err(format!(
            "fcntl F_SETFL failed: {}",
            io::Error::last_os_error()
        ));
    }
    Ok(())
}

fn close_pipe(pipe: Pipe) {
    close_fd(pipe.read_fd);
    close_fd(pipe.write_fd);
}

fn close_fd(fd: RawFd) {
    unsafe {
        libc::close(fd);
    }
}

fn set_child_process_group(pid: libc::pid_t) -> Result<(), String> {
    let result = unsafe { libc::setpgid(pid, pid) };
    if result == 0 {
        return Ok(());
    }
    let error = io::Error::last_os_error();
    if error.raw_os_error() == Some(libc::EACCES) {
        return Ok(());
    }
    Err(format!("setpgid for child {pid} failed: {error}"))
}

fn assert_single_threaded_parent() -> Result<(), String> {
    let task_count = fs::read_dir("/proc/self/task")
        .map_err(|error| format!("read /proc/self/task failed: {error}"))?
        .count();
    if task_count == 1 {
        return Ok(());
    }
    Err(format!(
        "refusing to fork: /proc/self/task contains {task_count} tasks; exactly one is required"
    ))
}

fn report_cli_error(error: String) {
    eprintln!("prefork_full_ui: {error}");
}

fn report_runner_error(error: String) {
    eprintln!("prefork_full_ui: {error}");
}
