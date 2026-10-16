use super::{read_cached_addons_path, sync_from_snapshot};
use std::{
    fs,
    path::{Path, PathBuf},
};

const MANIFEST: &str = "Blizzard_Test/Test.lua\nFrameXML/FrameXML.toc\nFrameXML/UIParent.lua\n";
const COMPLETE: &str = ".wow-ui-sim-blizzard-ui-complete";
const PROVENANCE: &str = ".wow-ui-sim-blizzard-ui-provenance";

fn fixture(base: &Path) -> (PathBuf, PathBuf) {
    let source = base.join("source/wrath");
    let addons = base.join("runtime/wrath/AddOns");
    for path in [
        "AddOns/Blizzard_Test/Test.lua",
        "FrameXML/FrameXML.toc",
        "FrameXML/UIParent.lua",
    ] {
        let file = source.join(path);
        fs::create_dir_all(file.parent().unwrap()).unwrap();
        fs::write(file, path).unwrap();
    }
    let pin: serde_json::Value =
        serde_json::from_str(include_str!("../../data/blizzard-ui-builds/wrath.json")).unwrap();
    let provenance = serde_json::json!({"gethe_revision": pin["gethe_revision"]});
    fs::write(
        source.join(".wow-ui-sim-wrath-source.json"),
        provenance.to_string(),
    )
    .unwrap();
    (source, addons)
}

#[test]
fn complete_runtime_accepts_without_source_and_repeat_preserves_bytes() {
    let temp = tempfile::tempdir().unwrap();
    let (source, addons) = fixture(temp.path());
    fs::create_dir_all(addons.parent().unwrap()).unwrap();
    assert_eq!(read_cached_addons_path(&addons, MANIFEST).unwrap(), None);
    let first = sync_from_snapshot(&source, &addons, MANIFEST).unwrap();
    assert_eq!((first.total, first.copied, first.present), (3, 3, 0));
    assert_eq!(fs::read(addons.join(COMPLETE)).unwrap(), b"ok\n");
    let provenance = fs::read(addons.join(PROVENANCE)).unwrap();
    let repeated = sync_from_snapshot(&source, &addons, MANIFEST).unwrap();
    assert_eq!(
        (repeated.total, repeated.copied, repeated.present),
        (3, 0, 3)
    );
    assert_eq!(fs::read(addons.join(PROVENANCE)).unwrap(), provenance);
    fs::remove_dir_all(&source).unwrap();
    assert_eq!(
        read_cached_addons_path(&addons, MANIFEST).unwrap(),
        Some(addons)
    );
}

#[test]
fn missing_or_wrong_source_cannot_publish_runtime_completion() {
    for missing in [
        "FrameXML/UIParent.lua",
        ".wow-ui-sim-wrath-source.json",
        "revision",
    ] {
        let temp = tempfile::tempdir().unwrap();
        let (source, addons) = fixture(temp.path());
        if missing == "revision" {
            fs::write(
                source.join(".wow-ui-sim-wrath-source.json"),
                b"{\"gethe_revision\":\"wrong\"}",
            )
            .unwrap();
        } else {
            fs::remove_file(source.join(missing)).unwrap();
        }
        assert!(sync_from_snapshot(&source, &addons, MANIFEST).is_err());
        assert!(!addons.join(COMPLETE).exists());
        assert!(!addons.join(PROVENANCE).exists());
    }
}

#[test]
fn runtime_reader_rejects_stale_identity_and_missing_sibling_files() {
    let temp = tempfile::tempdir().unwrap();
    let (source, addons) = fixture(temp.path());
    sync_from_snapshot(&source, &addons, MANIFEST).unwrap();
    assert_eq!(
        read_cached_addons_path(&addons, "Blizzard_Test/Test.lua\n").unwrap(),
        None
    );
    fs::remove_file(addons.parent().unwrap().join("FrameXML/UIParent.lua")).unwrap();
    assert_eq!(read_cached_addons_path(&addons, MANIFEST).unwrap(), None);
    fs::write(addons.join(PROVENANCE), b"stale identity\n").unwrap();
    assert!(sync_from_snapshot(&source, &addons, MANIFEST).is_err());
    assert_eq!(
        fs::read(addons.join(PROVENANCE)).unwrap(),
        b"stale identity\n"
    );
    assert!(
        !addons
            .parent()
            .unwrap()
            .join("FrameXML/UIParent.lua")
            .exists()
    );
}
