use super::*;

fn seed_snapshot(parent: &Path, revision: &str) -> PathBuf {
    let snapshot = parent.join("wrath");
    fs::create_dir_all(snapshot.join("AddOns")).unwrap();
    fs::create_dir(snapshot.join("FrameXML")).unwrap();
    fs::write(snapshot.join("FrameXML/FrameXML.toc"), b"UIParent.lua\n").unwrap();
    fs::write(snapshot.join("AddOns/sentinel.lua"), b"original\n").unwrap();
    let provenance = serde_json::json!({"gethe_revision": revision});
    fs::write(snapshot.join(PROVENANCE), provenance.to_string()).unwrap();
    snapshot
}

#[test]
fn matching_snapshot_is_primary() {
    let parent = tempfile::tempdir().unwrap();
    let revision = read_pinned_revision().unwrap();
    let snapshot = seed_snapshot(parent.path(), &revision);
    assert_eq!(acquire_snapshot_at(&snapshot).unwrap(), snapshot);
    assert_eq!(
        fs::read(snapshot.join("AddOns/sentinel.lua")).unwrap(),
        b"original\n"
    );
}

#[test]
fn mismatched_snapshot_is_rejected_and_preserved() {
    let parent = tempfile::tempdir().unwrap();
    let snapshot = seed_snapshot(parent.path(), "not-the-pin");
    let original = fs::read(snapshot.join(PROVENANCE)).unwrap();
    assert!(acquire_snapshot_at(&snapshot).is_err());
    assert_eq!(fs::read(snapshot.join(PROVENANCE)).unwrap(), original);
    assert_eq!(
        fs::read(snapshot.join("AddOns/sentinel.lua")).unwrap(),
        b"original\n"
    );
}

#[test]
fn incomplete_or_malformed_snapshots_are_not_replaced() {
    for missing in [
        PROVENANCE,
        "FrameXML/FrameXML.toc",
        "AddOns",
        "invalid-json",
    ] {
        let parent = tempfile::tempdir().unwrap();
        let snapshot = seed_snapshot(parent.path(), &read_pinned_revision().unwrap());
        match missing {
            "AddOns" => fs::remove_dir_all(snapshot.join(missing)).unwrap(),
            "invalid-json" => fs::write(snapshot.join(PROVENANCE), b"{").unwrap(),
            _ => fs::remove_file(snapshot.join(missing)).unwrap(),
        }
        assert!(acquire_snapshot_at(&snapshot).is_err());
        assert!(snapshot.is_dir());
        if missing == "invalid-json" {
            assert_eq!(fs::read(snapshot.join(PROVENANCE)).unwrap(), b"{");
        } else {
            assert!(!snapshot.join(missing).exists());
        }
    }
}

#[cfg(not(feature = "casc"))]
#[test]
fn absent_snapshot_requires_casc_without_creating_destination() {
    let parent = tempfile::tempdir().unwrap();
    let snapshot = parent.path().join("not-created/wrath");
    assert!(acquire_snapshot_at(&snapshot).is_err());
    assert!(!parent.path().join("not-created").exists());
}
