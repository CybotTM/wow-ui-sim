use serde_json::Value;
use std::{collections::HashSet, fs, path::PathBuf};

fn repo_path(relative: &str) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join(relative)
}

#[test]
fn probe_register_has_itemized_checked_in_evidence() {
    let manifest: Value = serde_json::from_str(
        &fs::read_to_string(repo_path("data/patch-api/12.0.5-probes.json"))
            .expect("12.0.5 probe manifest should be readable"),
    )
    .expect("12.0.5 probe manifest should be valid JSON");
    let rows = manifest["rows"]
        .as_array()
        .expect("probe manifest rows should be an array");
    assert_eq!(rows.len(), 38);

    let ids = rows
        .iter()
        .map(|row| row["id"].as_str().expect("probe row id should be a string"))
        .collect::<HashSet<_>>();
    assert_eq!(ids.len(), rows.len(), "probe occurrence IDs must be unique");

    for row in rows {
        let symbol = row["symbol"]
            .as_str()
            .expect("probe symbol should be present");
        let addon = symbol
            .split('.')
            .next()
            .expect("probe symbol should be qualified");
        let source_dir = repo_path(&format!("docs/addons/{addon}"));
        assert!(
            source_dir.is_dir(),
            "missing probe source directory: {addon}"
        );
        let notes = row["notes"].as_str().unwrap_or_default();
        assert!(
            !notes.is_empty(),
            "probe row should retain an evidence note: {symbol}"
        );
    }
}
