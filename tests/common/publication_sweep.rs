//! Register-driven publication/absence sweep with cached deprecation aliases.
//! Publication/absence only: no signature, output, security, or behavior parity claim.

use std::collections::BTreeMap;

use wow_ui_sim::lua_api::WowLuaEnv;

#[path = "publication_sweep_core.rs"]
mod publication_sweep_core;

pub(crate) use publication_sweep_core::*;

/// Attribute only direct assignments in loaded, unmodified cached deprecation files
/// (any `*Deprecated*.lua` under a loaded addon). No hand-maintained symbol whitelist,
/// and no source rewrite; the probe still requires exact target identity at runtime.
pub(crate) fn read_deprecated_aliases(env: &WowLuaEnv) -> BTreeMap<String, (String, String)> {
    let root =
        wow_ui_sim::paths::default_blizzard_ui_addons_path().expect("resolve cached Blizzard UI");
    let mut aliases = BTreeMap::new();
    for addon in read_sorted_dir(&root) {
        let Some(name) = addon
            .file_name()
            .and_then(|n| n.to_str())
            .map(str::to_owned)
        else {
            continue;
        };
        if !addon.is_dir() || !addon_is_loaded(env, &name) {
            continue;
        }
        for file in deprecation_files(&addon) {
            let source = std::fs::read_to_string(&file)
                .unwrap_or_else(|error| panic!("read {}: {error}", file.display()));
            let label = file.to_string_lossy();
            aliases.extend(
                source
                    .lines()
                    .filter_map(|line| parse_deprecated_alias(line, &label)),
            );
        }
    }
    aliases
}

fn addon_is_loaded(env: &WowLuaEnv, name: &str) -> bool {
    env.eval(&format!("return C_AddOns.IsAddOnLoaded({name:?}) == true"))
        .expect("query addon load state")
}

fn read_sorted_dir(dir: &std::path::Path) -> Vec<std::path::PathBuf> {
    let mut entries: Vec<_> = std::fs::read_dir(dir)
        .unwrap_or_else(|error| panic!("read {}: {error}", dir.display()))
        .map(|entry| entry.expect("read dir entry").path())
        .collect();
    entries.sort();
    entries
}

fn deprecation_files(dir: &std::path::Path) -> Vec<std::path::PathBuf> {
    let mut files = Vec::new();
    for path in read_sorted_dir(dir) {
        if path.is_dir() {
            files.extend(deprecation_files(&path));
        } else if path.extension().is_some_and(|ext| ext == "lua")
            && path
                .file_name()
                .and_then(|n| n.to_str())
                .is_some_and(|n| n.contains("Deprecated"))
        {
            files.push(path);
        }
    }
    files
}

fn parse_deprecated_alias(line: &str, source: &str) -> Option<(String, (String, String))> {
    let (name, target) = line.trim().trim_end_matches(';').split_once('=')?;
    let name = name.trim();
    let target = target.trim();
    let is_identifier = |value: &str| {
        !value.is_empty()
            && value
                .bytes()
                .all(|byte| byte.is_ascii_alphanumeric() || byte == b'_')
    };
    let is_path = |value: &str| value.split('.').all(is_identifier);
    let (namespace, member) = target.split_once('.')?;
    if !is_path(name)
        || name.matches('.').count() > 1
        || !namespace.starts_with("C_")
        || !is_identifier(namespace)
        || !is_identifier(member)
    {
        return None;
    }
    Some((name.into(), (target.into(), source.into())))
}

/// Probe every register row in one cached Game environment and require the non-ok
/// ID set to equal the reviewed known-gap set exactly.
pub(crate) fn run_publication_sweep(env: &WowLuaEnv, spec: &SweepSpec) {
    run_sweep(env, spec, || read_deprecated_aliases(env));
}
