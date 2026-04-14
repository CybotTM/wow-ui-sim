//! Shared test helpers.

use std::path::PathBuf;
use std::sync::{Mutex, OnceLock};
use wow_ui_sim::loader::load_addon;
use wow_ui_sim::lua_api::WowLuaEnv;

/// Per-test timeout. Panics if the closure doesn't complete within `secs`.
/// Default 120s — enough for full Blizzard UI load + test logic.
#[allow(dead_code)]
pub fn with_timeout<F: FnOnce() + Send + 'static>(secs: u64, f: F) {
    let (tx, rx) = std::sync::mpsc::channel();
    let handle = std::thread::spawn(move || {
        f();
        let _ = tx.send(());
    });
    match rx.recv_timeout(std::time::Duration::from_secs(secs)) {
        Ok(()) => handle.join().expect("test thread panicked"),
        Err(std::sync::mpsc::RecvTimeoutError::Timeout) => {
            panic!("test timed out after {secs}s")
        }
        Err(std::sync::mpsc::RecvTimeoutError::Disconnected) => {
            handle
                .join()
                .expect_err("test thread panicked but join succeeded");
            panic!("test thread panicked (see above)")
        }
    }
}

/// Serialize perf-sensitive integration tests so their thresholds measure one
/// startup/load scenario at a time instead of competing with sibling tests.
#[allow(dead_code)]
pub fn with_perf_lock<T>(f: impl FnOnce() -> T) -> T {
    static PERF_TEST_LOCK: OnceLock<Mutex<()>> = OnceLock::new();
    let _guard = PERF_TEST_LOCK
        .get_or_init(|| Mutex::new(()))
        .lock()
        .expect("perf test lock should not be poisoned");
    f()
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
#[allow(dead_code)]
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
#[allow(dead_code)]
pub fn env_with_shared_xml() -> WowLuaEnv {
    let env = WowLuaEnv::new().expect("Failed to create Lua environment");
    let ui = blizzard_ui_dir();

    let base_toc = ui.join("Blizzard_SharedXMLBase/Blizzard_SharedXMLBase.toc");
    if base_toc.exists() {
        if let Err(e) = load_addon(&env.loader_env(), &base_toc) {
            eprintln!("Warning: Failed to load SharedXMLBase: {}", e);
        }
    }

    let shared_toc = ui.join("Blizzard_SharedXML/Blizzard_SharedXML_Mainline.toc");
    if shared_toc.exists() {
        if let Err(e) = load_addon(&env.loader_env(), &shared_toc) {
            eprintln!("Warning: Failed to load SharedXML: {}", e);
        }
    }

    env
}
