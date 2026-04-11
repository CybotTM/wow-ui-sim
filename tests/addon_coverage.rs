mod common;

use std::collections::{BTreeMap, HashSet};
use std::path::PathBuf;
use wow_ui_sim::loader::{discover_all_blizzard_addons, load_addon};
use wow_ui_sim::lua_api::WowLuaEnv;
use wow_ui_sim::lua_errors::grouped_errors_by_addon;

fn blizzard_ui_dir() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("Interface/BlizzardUI")
}

fn format_per_addon_report(grouped_errors: &BTreeMap<String, Vec<String>>) -> String {
    let mut rows: Vec<_> = grouped_errors.iter().collect();
    rows.sort_by(|(left_name, left_errors), (right_name, right_errors)| {
        right_errors
            .len()
            .cmp(&left_errors.len())
            .then_with(|| left_name.cmp(right_name))
    });

    rows.into_iter()
        .map(|(addon_name, errors)| {
            let sample = errors.first().map(String::as_str).unwrap_or("<no sample>");
            format!("{addon_name}: {} error(s); sample: {sample}", errors.len())
        })
        .collect::<Vec<_>>()
        .join("\n")
}

fn count_blizzard_directories() -> usize {
    std::fs::read_dir(blizzard_ui_dir())
        .expect("BlizzardUI directory should be readable")
        .flatten()
        .filter(|entry| {
            entry.path().is_dir()
                && entry
                    .file_name()
                    .to_str()
                    .is_some_and(|name| name.starts_with("Blizzard_"))
        })
        .count()
}

#[test]
fn all_blizzard_addon_load_errors_are_tracked_per_addon_name() {
    test_timeout! {
        let env = WowLuaEnv::new().expect("Failed to create Lua environment");
        env.set_screen_size(1024.0, 768.0);
        env.state().borrow_mut().addon_base_paths = vec![blizzard_ui_dir()];

        assert_eq!(
            count_blizzard_directories(),
            315,
            "expected the current Blizzard UI checkout to contain 315 Blizzard_* directories"
        );

        let addons = discover_all_blizzard_addons(&blizzard_ui_dir());
        assert_eq!(
            addons.len(),
            313,
            "expected the current Blizzard UI checkout to expose 313 loadable Blizzard addons; Blizzard_LevelUpDisplay and Blizzard_TalentUI only ship legacy Mists TOCs"
        );

        let known_addons: HashSet<_> = addons.iter().map(|(name, _)| name.clone()).collect();
        let mut load_failures = Vec::new();

        for (name, toc_path) in &addons {
            if let Err(error) = load_addon(&env.loader_env(), toc_path) {
                load_failures.push(format!("{name}: {error}"));
            }
        }

        assert!(
            load_failures.is_empty(),
            "force-loading all Blizzard addons should not have hard TOC load failures:\n{}",
            load_failures.join("\n"),
        );

        let state = env.state().borrow();
        let grouped_errors = grouped_errors_by_addon(&state);
        let unknown_count = grouped_errors.get("<unknown>").map_or(0, Vec::len);
        let invalid_addons: Vec<_> = grouped_errors
            .keys()
            .filter(|addon_name| addon_name.as_str() != "<unknown>" && !known_addons.contains(*addon_name))
            .cloned()
            .collect();

        assert!(
            unknown_count == 0,
            "full Blizzard load should attribute Lua errors to addon names, not <unknown>.\n{}",
            format_per_addon_report(&grouped_errors),
        );
        assert!(
            invalid_addons.is_empty(),
            "full Blizzard load attributed Lua errors to names outside the 315 Blizzard addons: {:?}\n{}",
            invalid_addons,
            format_per_addon_report(&grouped_errors),
        );
    }
}
