//! Private observational prototype; Rust edition 2024.
//! Extern dependencies: wow_ui_sim, serde_json.
//! Main owns launch settings, builds, runtime, commits, and verification.
//! No acceptance thresholds or claim that discovery covers all Blizzard addons.

use serde_json::{Value, json};
use std::io::{self, Write};
use std::path::Path;
use std::process::ExitCode;
use wow_ui_sim::client_profile;
use wow_ui_sim::loader::{self, LoadError, LoadResult, LoadTiming};
use wow_ui_sim::lua_api::WowLuaEnv;
use wow_ui_sim::screen::ScreenKind;

const REPO_ROOT: &str = "/home/osso/Projects/wow/wow-ui-sim";

fn initialize_game_environment(addons: &Path) -> Result<WowLuaEnv, String> {
    loader::enter_bytecode_cache_read_only_mode();
    let env = WowLuaEnv::new().map_err(|_| "environment initialization failed".to_owned())?;
    env.set_screen_size(1024.0, 768.0);
    env.set_screen_mode(ScreenKind::Game);
    env.state().borrow_mut().addon_base_paths = vec![addons.to_path_buf()];
    Ok(env)
}

// All durations are nanoseconds copied from returned fields, except total(),
// which is the loader's public returned-timing total. No exclusive-cost sums.
// lua_exec aliases compile + call; XML/frame detail fields contain subsets.
fn base_timing(t: &LoadTiming) -> Value {
    json!({
        "io": t.io_time.as_nanos(),
        "xml_parse": t.xml_parse_time.as_nanos(),
        "xml_process": t.xml_process_time.as_nanos(),
        "lua_compile": t.lua_compile_time.as_nanos(),
        "lua_call": t.lua_call_time.as_nanos(),
        "lua_exec": t.lua_exec_time.as_nanos(),
        "savedvars": t.saved_vars_time.as_nanos(),
        "total": t.total().as_nanos(),
    })
}

fn xml_frame_timing(t: &LoadTiming) -> Value {
    json!({
        "xml_frame_create": t.xml_frame_create_time.as_nanos(),
        "xml_frame_setup": t.xml_frame_setup_time.as_nanos(),
        "xml_frame_finalize": t.xml_frame_finalize_time.as_nanos(),
        "frame_exec_lua": t.frame_exec_lua_time.as_nanos(),
        "frame_apply_props": t.frame_apply_props_time.as_nanos(),
        "frame_layer_children": t.frame_layer_children_time.as_nanos(),
        "frame_lifecycle": t.frame_lifecycle_time.as_nanos(),
        "frame_code_build": t.frame_code_build_time.as_nanos(),
        "frame_anim": t.frame_anim_time.as_nanos(),
        "frame_button": t.frame_button_time.as_nanos(),
    })
}

fn creation_counts(t: &LoadTiming) -> Value {
    json!({
        "frame": t.frame_count,
        "lifecycle_fire": t.lifecycle_fire_count,
        "texture": t.texture_count,
        "fontstring": t.fontstring_count,
    })
}

fn cache_counts(t: &LoadTiming) -> Value {
    json!({
        "hits": t.cache_hits,
        "misses": t.cache_misses,
        "lookupmiss": t.cache_lookup_misses,
        "replayfail": t.cache_replay_failures,
        "store_success": t.cache_store_successes,
        "store_failure": t.cache_store_failures,
    })
}

fn result_row(ordinal: usize, expected: &str, result: &LoadResult) -> Value {
    json!({
        "kind": "load_result",
        "ordinal": ordinal,
        "expected": expected,
        "actual": result.name,
        "lua_files": result.lua_files,
        "xml_files": result.xml_files,
        "timing_ns": {
            "base": base_timing(&result.timing),
            "xml_frame": xml_frame_timing(&result.timing),
        },
        "counts": creation_counts(&result.timing),
        "cache": cache_counts(&result.timing),
        "diagnostics": {
            "warnings": result.warnings.len(),
            "nil_symbol_observations": result.nil_symbol_observations.len(),
            "missing_requirements": result.missing_requirements.len(),
        },
    })
}

// Classify hard errors without exposing Lua/XML diagnostic messages or values.
fn error_kind(error: &LoadError) -> &'static str {
    match error {
        LoadError::Io(_) => "LoadError::Io",
        LoadError::IoWithPath { .. } => "LoadError::IoWithPath",
        LoadError::Toc(_) => "LoadError::Toc",
        LoadError::Xml(_) => "LoadError::Xml",
        LoadError::Lua(_) => "LoadError::Lua",
        LoadError::DepDisabled(_) => "LoadError::DepDisabled",
    }
}

fn collect_results(env: &WowLuaEnv, addons: &Path) -> (Vec<Value>, Option<String>) {
    let discovered = loader::discover_blizzard_addons_for_screen(addons, ScreenKind::Game);
    let loader_env = env.loader_env();
    let mut rows = Vec::new();
    for (index, (expected, toc)) in discovered.into_iter().enumerate() {
        let ordinal = index + 1;
        match loader::load_addon(&loader_env, &toc) {
            Ok(result) => rows.push(result_row(ordinal, &expected, &result)),
            Err(error) => {
                let context = format!(
                    "ordinal {ordinal}, addon {expected}: {}",
                    error_kind(&error)
                );
                return (rows, Some(context));
            }
        }
    }
    (rows, None)
}

fn emit_records(rows: &[Value], error: Option<&str>) -> io::Result<()> {
    let mut summary = json!({
        "kind": "summary",
        "complete": error.is_none(),
        "rowcount": rows.len(),
    });
    if let Some(context) = error {
        summary["returned_error"] = json!(context);
    }
    let stdout = io::stdout();
    let mut output = stdout.lock();
    for record in rows.iter().chain(std::iter::once(&summary)) {
        serde_json::to_writer(&mut output, record)?;
        writeln!(output)?;
    }
    output.flush()
}

fn main() -> ExitCode {
    let root = Path::new(REPO_ROOT);
    let addons = client_profile::blizzard_ui_addons_dir_under(root);
    let (rows, error) = match initialize_game_environment(&addons) {
        Ok(env) => collect_results(&env, &addons),
        Err(context) => (Vec::new(), Some(context)),
    };
    // Empty discovery and zero-file LoadResults are preserved, not interpreted
    // as skipped or proof that every Blizzard addon loaded. Complete means only
    // that this discovered sequence ended without a returned hard error.
    if let Err(output_error) = emit_records(&rows, error.as_deref()) {
        eprintln!("probe JSON output failed: {output_error}");
        return ExitCode::FAILURE;
    }
    if error.is_some() {
        ExitCode::FAILURE
    } else {
        ExitCode::SUCCESS
    }
}
