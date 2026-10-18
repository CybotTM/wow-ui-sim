//! Subprocesses isolate the once-cached timing environment and capture real stderr.
use std::process::Command;
use wow_ui_sim::lua_api::WowLuaEnv;

const CHILD: &str = "WOW_SIM_TIMER_PROVENANCE_TEST_CHILD";
const TIMINGS: &str = "WOW_SIM_LOG_HANDLER_TIMINGS";
const SOURCE: &str = "@Interface/AddOns/TimerProvenance/Callbacks.lua";

#[test]
fn after_success_logs_original_source_before_body_and_correlated_return() {
    let Some(stderr) = run_fixture(Some("0")) else { return };
    assert_invocation(&stderr, 2, "timer-provenance-success-body", "ok");
}

#[test]
fn after_error_logs_original_source_before_body_and_correlated_return() {
    let Some(stderr) = run_fixture(Some("0")) else { return };
    assert_invocation(&stderr, 7, "timer-provenance-error-body", "error");
    assert!(stderr.contains("timer-provenance-sentinel"));
}

#[test]
fn timer_provenance_is_silent_without_environment_gate() {
    let Some(stderr) = run_fixture(None) else { return };
    assert!(stderr.contains("timer-provenance-success-body"));
    assert!(!stderr.contains("[handler-entry]"), "{stderr}");
    assert!(!stderr.contains("timer_id="), "{stderr}");
    assert!(!stderr.contains("callback_source="), "{stderr}");
}

#[test]
fn entries_ignore_duration_threshold_and_ticker_reuses_id() {
    let Some(stderr) = run_fixture(Some("1000000000")) else { return };
    let entries: Vec<_> = stderr
        .lines()
        .filter(|line| line.starts_with("[handler-entry]") && line.contains("handler=C_Timer"))
        .collect();
    assert_eq!(entries.len(), 4, "{stderr}");
    let ticker: Vec<_> = entries
        .iter()
        .filter(|line| line.contains(&format!("callback_source={SOURCE}:14")))
        .collect();
    assert_eq!(ticker.len(), 2, "{stderr}");
    assert_eq!(field(ticker[0], "timer_id"), field(ticker[1], "timer_id"));
    assert!(!stderr.lines().any(|line| line.starts_with("[handler]")), "{stderr}");
}

fn assert_invocation(stderr: &str, line: u32, body: &str, outcome: &str) {
    let label = format!("callback_source={SOURCE}:{line}");
    let entries: Vec<_> = stderr
        .lines()
        .filter(|record| record.starts_with("[handler-entry]") && record.contains(&label))
        .collect();
    assert_eq!(entries.len(), 1, "missing original callback entry: {stderr}");
    let entry = entries[0];
    let id = field(entry, "timer_id");
    assert!(id.parse::<u64>().is_ok(), "{entry}");
    let returns: Vec<_> = stderr
        .lines()
        .filter(|record| {
            record.starts_with("[handler]")
                && record.contains(&label)
                && field(record, "timer_id") == id
        })
        .collect();
    assert_eq!(returns.len(), 1, "missing correlated return: {stderr}");
    let returned = returns[0];
    for record in [entry, returned] {
        assert!(record.contains("handler=C_Timer frame=#0"), "{record}");
        assert!(record.contains(" source=="), "wrapper source lost: {record}");
    }
    assert!(returned.contains("duration_ms="), "{returned}");
    assert_eq!(field(returned, "outcome"), outcome);
    assert!(stderr.find(entry).unwrap() < stderr.find(body).unwrap(), "{stderr}");
    assert!(stderr.find(body).unwrap() < stderr.find(returned).unwrap(), "{stderr}");
}

fn field<'a>(record: &'a str, name: &str) -> &'a str {
    let prefix = format!("{name}=");
    record
        .split_whitespace()
        .find_map(|part| part.strip_prefix(&prefix))
        .unwrap_or_else(|| panic!("missing {name}: {record}"))
}

#[test]
fn completed_callback_handle_cycle_does_not_root_provenance() {
    run_isolated(Some("0"), exercise_completed_handle_cycle);
}

fn exercise_completed_handle_cycle() {
    let env = WowLuaEnv::new().unwrap();
    env.exec(
        r#"
        local originals = debug.getregistry().__wow_timer_original_callbacks
        assert(type(originals) == "table", "diagnostics must be enabled")
        local handle = {}
        local callback = function() handle.fired = true end
        TimerLifetimeContainer = C_FunctionContainers.CreateCallback(callback)
        C_Timer.After(0, TimerLifetimeContainer)
        -- Model a callback-owned handle reaching its invoker. Registry inspection
        -- exposes the wrapper, but neither it nor the original is kept globally.
        handle.invoker = next(originals)
        assert(originals[handle.invoker] == callback)
        "#,
    )
    .unwrap();
    env.process_timers().unwrap();
    env.exec(
        r#"
        -- Remove the pre-existing container backing map's independent strong
        -- callback edge, so this test isolates diagnostic ownership only.
        TimerLifetimeContainer._callback = nil
        TimerLifetimeContainer = nil
        collectgarbage("collect")
        collectgarbage("collect")
        assert(next(debug.getregistry().__wow_timer_original_callbacks) == nil,
            "completed callback/handle cycle retained by provenance association")
        "#,
    )
    .unwrap();
}

fn run_fixture(threshold: Option<&str>) -> Option<String> {
    run_isolated(threshold, exercise_addon)
}

fn run_isolated(threshold: Option<&str>, exercise: fn()) -> Option<String> {
    if std::env::var_os(CHILD).is_some() {
        exercise();
        return None;
    }
    let thread = std::thread::current();
    let mut command = Command::new(std::env::current_exe().unwrap());
    command
        .args([thread.name().unwrap(), "--exact", "--test-threads=1", "--nocapture"])
        .env(CHILD, "1")
        .env_remove(TIMINGS);
    if let Some(threshold) = threshold {
        command.env(TIMINGS, threshold);
    }
    let output = command.output().expect("run timer provenance fixture child");
    let stderr = String::from_utf8(output.stderr).unwrap();
    assert!(output.status.success(), "fixture child failed: {stderr}\n{}", String::from_utf8_lossy(&output.stdout));
    Some(stderr)
}

fn exercise_addon() {
    let temp = tempfile::tempdir().unwrap();
    let addon = temp.path().join("Interface/AddOns/TimerProvenance");
    std::fs::create_dir_all(&addon).unwrap();
    std::fs::write(addon.join("TimerProvenance.toc"), include_str!("fixtures/TimerProvenance/TimerProvenance.toc")).unwrap();
    std::fs::write(addon.join("Callbacks.lua"), include_str!("fixtures/TimerProvenance/Callbacks.lua")).unwrap();
    let root = addon.join("TimerProvenance.toc");
    let env = WowLuaEnv::new().unwrap();
    env.exec("seterrorhandler(function(message) TimerProvenance.errors = (TimerProvenance.errors or 0) + 1; print(message) end)").unwrap();
    let loaded = wow_ui_sim::loader::load_addon(&env.loader_env(), &root).unwrap();
    assert!(loaded.warnings.is_empty(), "{:?}", loaded.warnings);
    env.process_timers().unwrap();
    env.process_timers().unwrap();
    env.exec("assert(TimerProvenance.calls == 4); assert(TimerProvenance.errors == 1)").unwrap();
}
