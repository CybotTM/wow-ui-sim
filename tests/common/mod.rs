//! Shared test helpers.

pub mod blizzard_addon_harness;
pub mod blizzard_addon_manifest;
mod event_helpers;
pub mod panel_fixtures;

use std::ops::Deref;
use std::path::PathBuf;
use std::sync::{Mutex, MutexGuard, OnceLock};
use wow_ui_sim::loader::load_addon;
use wow_ui_sim::lua_api::WowLuaEnv;

/// Per-test timeout. Exits the process if the closure doesn't complete within `secs`.
/// Default 120s — enough for full Blizzard UI load + test logic.
pub fn with_timeout<F: FnOnce() + Send + 'static>(secs: u64, f: F) {
    let (tx, rx) = std::sync::mpsc::channel();
    let handle = std::thread::spawn(move || {
        f();
        let _ = tx.send(());
    });
    match rx.recv_timeout(std::time::Duration::from_secs(secs)) {
        Ok(()) => handle.join().expect("test thread panicked"),
        Err(std::sync::mpsc::RecvTimeoutError::Timeout) => {
            eprintln!("test timed out after {secs}s");
            std::process::exit(1);
        }
        Err(std::sync::mpsc::RecvTimeoutError::Disconnected) => {
            handle
                .join()
                .expect_err("test thread panicked but join succeeded");
            eprintln!("test thread panicked (see above)");
            std::process::exit(1);
        }
    }
}

/// Serialize perf-sensitive integration tests so their thresholds measure one
/// startup/load scenario at a time instead of competing with sibling tests.
pub fn with_perf_lock<T>(f: impl FnOnce() -> T) -> T {
    let _guard = lock_perf_tests();
    f()
}

fn lock_perf_tests() -> MutexGuard<'static, ()> {
    static PERF_TEST_LOCK: OnceLock<Mutex<()>> = OnceLock::new();
    let perf_lock = PERF_TEST_LOCK.get_or_init(|| Mutex::new(()));
    match perf_lock.lock() {
        Ok(guard) => guard,
        // Coverage shards intentionally probe failing paths; a prior panic
        // must not cascade into unrelated later shards in the same process.
        Err(poisoned) => poisoned.into_inner(),
    }
}

/// Keep a `WowLuaEnv` alive under the global perf lock for the lifetime of a test.
pub struct LockedEnv {
    _guard: MutexGuard<'static, ()>,
    env: WowLuaEnv,
}

pub fn lock_env(build: impl FnOnce() -> WowLuaEnv) -> LockedEnv {
    let guard = lock_perf_tests();
    let env = build();
    LockedEnv { _guard: guard, env }
}

impl Deref for LockedEnv {
    type Target = WowLuaEnv;

    fn deref(&self) -> &Self::Target {
        &self.env
    }
}

/// Convenience macro: wraps a test body with a 120s timeout.
///
/// ```ignore
/// #[test]
/// fn my_test() {
///     test_timeout! {
///         let env = WowLuaEnv::new().unwrap();
///         // ... test body ...
///     }
/// }
/// ```
#[macro_export]
macro_rules! test_timeout {
    ($($body:tt)*) => {
        common::with_timeout(120, move || { $($body)* })
    };
}

/// Try to create a wgpu device for GPU tests.
/// Returns None if no adapter is available (e.g., headless CI).
#[cfg(feature = "gui")]
pub fn try_create_gpu_device() -> Option<(wgpu::Device, wgpu::Queue)> {
    let instance = wgpu::Instance::new(&wgpu::InstanceDescriptor {
        backends: wgpu::Backends::all(),
        ..Default::default()
    });

    let adapter = pollster::block_on(instance.request_adapter(&wgpu::RequestAdapterOptions {
        power_preference: wgpu::PowerPreference::LowPower,
        compatible_surface: None,
        force_fallback_adapter: false,
    }))
    .ok()?;

    let (device, queue) = pollster::block_on(adapter.request_device(&wgpu::DeviceDescriptor {
        label: Some("Test GPU Device"),
        ..Default::default()
    }))
    .ok()?;

    Some((device, queue))
}

fn blizzard_ui_dir() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("Interface/BlizzardUI")
}

/// Helper to load Blizzard_SharedXML templates for tests that need them.
/// Returns the environment with templates loaded.
pub fn env_with_shared_xml() -> WowLuaEnv {
    let env = WowLuaEnv::new().expect("Failed to create Lua environment");
    let ui = blizzard_ui_dir();

    let base_toc = ui.join("Blizzard_SharedXMLBase/Blizzard_SharedXMLBase.toc");
    if base_toc.exists()
        && let Err(e) = load_addon(&env.loader_env(), &base_toc)
    {
        eprintln!("Warning: Failed to load SharedXMLBase: {}", e);
    }

    let shared_toc = ui.join("Blizzard_SharedXML/Blizzard_SharedXML_Mainline.toc");
    if shared_toc.exists()
        && let Err(e) = load_addon(&env.loader_env(), &shared_toc)
    {
        eprintln!("Warning: Failed to load SharedXML: {}", e);
    }

    env
}

pub fn fire_addon_loaded(env: &WowLuaEnv, addon_name: &str) {
    let _ = env.fire_event_with_args("ADDON_LOADED", &[env.lua_string(addon_name)]);
}

pub use event_helpers::fire_player_entering_world;

pub fn call_global_if_present(env: &WowLuaEnv, function_name: &str) {
    let is_present = env
        .eval::<bool>(&format!("return type(_G[{function_name:?}]) == 'function'"))
        .unwrap_or(false);
    if is_present {
        let _ = env.exec(&format!(r#"_G[{function_name:?}]()"#));
    }
}

pub fn install_error_collector(env: &WowLuaEnv, global_name: &str) {
    env.exec(&format!(
        r#"
        local target = {global_name:?}
        _G[target] = {{}}
        seterrorhandler(function(msg)
            local trace = ""
            if type(debugstack) == "function" then
                trace = "\n" .. tostring(debugstack())
            end
            table.insert(_G[target], tostring(msg) .. trace)
        end)
        "#
    ))
    .expect("Failed to install test error handler");
}

pub fn drain_string_table(env: &WowLuaEnv, global_name: &str) -> Vec<String> {
    const SEP: char = '\u{1f}';
    let joined = env
        .eval::<String>(&format!(
            r#"
            local target = {global_name:?}
            local values = _G[target]
            if type(values) ~= "table" then
                return ""
            end
            local parts = {{}}
            for i = 1, #values do
                parts[#parts + 1] = tostring(values[i])
            end
            _G[target] = {{}}
            return table.concat(parts, string.char(31))
            "#
        ))
        .unwrap_or_default();
    if joined.is_empty() {
        return Vec::new();
    }
    joined.split(SEP).map(|entry| entry.to_string()).collect()
}

type TimeoutBody = fn();
type PerfBody = fn();
type EnvBuilder = fn() -> WowLuaEnv;

// Each integration test compiles this module separately, so a helper can be
// part of the shared test API while remaining unused in one specific target.
const _: () = {
    let _ = with_timeout::<TimeoutBody> as fn(u64, TimeoutBody);
    let _ = with_perf_lock::<()> as fn(PerfBody);
    let _ = std::mem::size_of::<LockedEnv>();
    let _ = lock_env as fn(EnvBuilder) -> LockedEnv;
    let _ = env_with_shared_xml as fn() -> WowLuaEnv;
    let _ = fire_addon_loaded as fn(&WowLuaEnv, &str);
    let _ = fire_player_entering_world as fn(&WowLuaEnv, bool, bool);
    let _ = call_global_if_present as fn(&WowLuaEnv, &str);
    let _ = install_error_collector as fn(&WowLuaEnv, &str);
    let _ = drain_string_table as fn(&WowLuaEnv, &str) -> Vec<String>;

    #[cfg(feature = "gui")]
    {
        let _ = try_create_gpu_device as fn() -> Option<(wgpu::Device, wgpu::Queue)>;
    }
};
