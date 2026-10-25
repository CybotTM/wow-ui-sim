use std::collections::{BTreeMap, BTreeSet};
use std::path::{Path, PathBuf};

use serde_json::{Value, json};
use sha2::{Digest, Sha256};
use wow_ui_sim::client_profile::{ACTIVE, ClientProfile};
use wow_ui_sim::loader::{discover_blizzard_addon_closure_for_screen_with_overrides, load_addon};
use wow_ui_sim::lua_api::WowLuaEnv;
use wow_ui_sim::screen::ScreenKind;

const INVENTORY: &str = include_str!("../../data/patch-api/sources/12.1.7-inherited-1.60.1.json");
const PROBE: &str = include_str!("probe.lua");
const CONSUMERS: &str = include_str!("consumers.lua");

fn profile() -> &'static str {
    match ACTIVE {
        ClientProfile::Ptr => "ptr",
        ClientProfile::Retail => "retail",
        ClientProfile::WowForever => "wowforever",
        _ => panic!("I4 selectors are restricted to PTR/Retail/Forever"),
    }
}

fn read_register() -> Value {
    let source = match std::env::var_os("P1217_SWEEP_REGISTER") {
        Some(path) => std::fs::read_to_string(path).expect("read explicit I4 profile register"),
        None => match ACTIVE {
            ClientProfile::Ptr => include_str!("../data/p1217-i4/ptr.json").into(),
            ClientProfile::Retail => include_str!("../data/p1217-i4/retail.json").into(),
            ClientProfile::WowForever => include_str!("../data/p1217-i4/wowforever.json").into(),
            _ => unreachable!(),
        },
    };
    let register: Value = serde_json::from_str(&source).expect("parse I4 register");
    assert_eq!(register["profile"], profile(), "wrong profile register");
    register
}

fn authenticate_consumer_files(root: &Path, register: &Value) {
    for file in register["files"].as_array().expect("source-bound files") {
        let relative = file["path"].as_str().expect("file path");
        let bytes = std::fs::read(root.join(relative)).expect("read original consumer bytes");
        let digest = format!("{:x}", Sha256::digest(&bytes));
        assert_eq!(file["sha256"], digest, "wrong source bytes: {relative}");
    }
}

fn read_cache_root(register: &Value) -> PathBuf {
    let root = wow_ui_sim::paths::default_blizzard_ui_addons_path().expect("resolve profile cache");
    if ACTIVE == ClientProfile::Ptr {
        let provenance = std::fs::read_to_string(root.join(".wow-ui-sim-blizzard-ui-provenance"))
            .expect("read PTR cache provenance");
        for expected in register["provenance"].as_array().expect("PTR identity") {
            assert!(
                provenance
                    .lines()
                    .any(|line| Some(line) == expected.as_str()),
                "PTR cache identity mismatch: {expected}"
            );
        }
        assert_eq!(
            std::fs::read(root.join(".wow-ui-sim-blizzard-ui-complete")).unwrap(),
            b"ok\n"
        );
    }
    authenticate_consumer_files(&root, register);
    root
}

fn load_consumers(root: &Path, register: &Value) -> (WowLuaEnv, Value) {
    let env = crate::common::blizzard_addon_harness::new_blizzard_addon_env(root);
    let roots: Vec<&str> = register["roots"]
        .as_array()
        .unwrap()
        .iter()
        .map(|value| value.as_str().unwrap())
        .collect();
    let selected = discover_blizzard_addon_closure_for_screen_with_overrides(
        root,
        ScreenKind::Game,
        &roots,
        &[],
    );
    let loads: BTreeMap<_, _> = selected
        .iter()
        .map(|(name, toc)| {
            let outcome = load_addon(&env.loader_env(), toc);
            (
                name.clone(),
                json!({"toc": toc.strip_prefix(root).unwrap(),
            "error": outcome.err().map(|error| error.to_string())}),
            )
        })
        .collect();
    for name in roots {
        assert!(
            loads.contains_key(name),
            "original root not runtime-selected: {name}"
        );
    }
    (env, json!(loads))
}

fn observe_row(env: &WowLuaEnv, row: &Value, register: &Value) -> Value {
    let id = row["id"].as_str().unwrap();
    let Some(expectation) = register["entries"].get(id) else {
        return json!({"proof": "static-only", "native": "unknown"});
    };
    let symbol = serde_json::to_string(row["symbol"].as_str().unwrap()).unwrap();
    let section = serde_json::to_string(row["section"].as_str().unwrap()).unwrap();
    let code = format!("return (function(symbol, section) {PROBE} end)({symbol}, {section})");
    let observation = env.eval::<String>(&code);
    match observation {
        Ok(observed) => json!({"proof": "publication-shape-only", "native": "unknown",
            "expected": expectation, "observed": observed}),
        Err(error) => json!({"proof": "probe-error", "native": "unknown",
            "expected": expectation, "error": error.to_string()}),
    }
}

fn observe_consumers(env: &WowLuaEnv, register: &Value) -> BTreeMap<String, Value> {
    register["consumers"]
        .as_array()
        .unwrap()
        .iter()
        .map(|case| {
            let name = case["name"].as_str().unwrap();
            let source = format!("return (function(case) {CONSUMERS} end)({name:?})");
            let result = env.eval::<String>(&source);
            let value = match result {
                Ok(observation) => json!({"ok": true, "observation": observation,
                "proof": "original-consumer-shape-only", "native": "unknown"}),
                Err(error) => json!({"ok": false, "error": error.to_string(),
                "proof": "consumer-failure-unclassified", "native": "unknown"}),
            };
            (name.into(), value)
        })
        .collect()
}

fn collect_observations() -> Value {
    let register = read_register();
    let root = read_cache_root(&register);
    let (env, loads) = load_consumers(&root, &register);
    let inventory: Value = serde_json::from_str(INVENTORY).unwrap();
    let rows = inventory["entries"].as_array().unwrap();
    let ids: BTreeSet<_> = rows.iter().map(|row| row["id"].as_str().unwrap()).collect();
    assert_eq!(
        (rows.len(), ids.len()),
        (1876, 1876),
        "literal row identity loss"
    );
    let observations: BTreeMap<_, _> = rows
        .iter()
        .map(|row| {
            (
                row["id"].as_str().unwrap().to_owned(),
                observe_row(&env, row, &register),
            )
        })
        .collect();
    json!({"schema": "p1217-i4-runtime-v1", "profile": profile(),
        "source_revision": std::env::var("P1217_SOURCE_SHA").expect("bind test source SHA"),
        "elf_sha256": std::env::var("P1217_ELF_SHA256").expect("bind sealed ELF"),
        "register_sha256": format!("{:x}", Sha256::digest(serde_json::to_vec(&register).unwrap())),
        "loads": loads, "observations": observations,
        "consumers": observe_consumers(&env, &register),
        "runtime_complete": false, "native_behavior": "unknown"})
}

fn write_observations(result: &Value) {
    let path = std::env::var_os("P1217_SWEEP_OUT").expect("P1217_SWEEP_OUT is mandatory");
    std::fs::write(path, serde_json::to_vec_pretty(result).unwrap())
        .expect("persist I4 results before assertions");
}

fn assert_ptr_shapes(result: &Value) {
    let failures: Vec<_> = result["observations"]
        .as_object()
        .unwrap()
        .iter()
        .filter_map(|(id, row)| {
            let expected = row["expected"]["shape"].as_str()?;
            (row["observed"].as_str() != Some(expected)).then_some(id)
        })
        .collect();
    let consumer_failures: Vec<_> = result["consumers"]
        .as_object()
        .unwrap()
        .iter()
        .filter(|(_, case)| case["ok"] != true)
        .map(|(name, _)| name)
        .collect();
    assert!(
        failures.is_empty() && consumer_failures.is_empty(),
        "PTR shape gaps {failures:?}; original-consumer failures {consumer_failures:?}. See persisted results; failures are not yet producer RED classification."
    );
}

fn assert_preservation(result: &Value) {
    let path = std::env::var_os("P1217_SWEEP_BASELINE").expect(
        "pre-repair own-profile baseline is required; never infer preservation from one run",
    );
    let bytes = std::fs::read(path).expect("read pre-repair baseline");
    let baseline: Value = serde_json::from_slice(&bytes).expect("parse baseline");
    for field in [
        "schema",
        "profile",
        "register_sha256",
        "observations",
        "consumers",
        "loads",
    ] {
        assert_eq!(
            result[field], baseline[field],
            "own-profile preservation changed: {field}"
        );
    }
}

pub(super) fn capture() {
    let result = collect_observations();
    write_observations(&result);
    eprintln!("I4 baseline measured only; no API/consumer/native acceptance asserted");
}

pub(super) fn run(preservation: bool) {
    let result = collect_observations();
    write_observations(&result);
    if ACTIVE == ClientProfile::Ptr {
        assert_ptr_shapes(&result);
    }
    if preservation {
        assert_preservation(&result);
    }
}
