//! Selected source bytes, conflict boundaries and profile isolation.
use super::{CopyCounts, copy_from_snapshot};
use serde_json::{Value, json};
use std::{
    collections::BTreeMap,
    fs,
    path::{Path, PathBuf},
};

const PIN: &str = include_str!("../../data/blizzard-ui-builds/wrath.json");
const SOURCE_PROVENANCE: &str = ".wow-ui-sim-wrath-source.json";
const MANIFEST: &str = "Blizzard_Test/Test.lua\nFrameXML/FrameXML.toc\nFrameXML/UIParent.lua\n";
const SELECTED: [(&str, &[u8]); 3] = [
    (
        "AddOns/Blizzard_Test/Test.lua",
        b"return 'selected addon'\r\n",
    ),
    ("FrameXML/FrameXML.toc", b"UIParent.lua\n"),
    ("FrameXML/UIParent.lua", b"return 'selected sibling'\n"),
];
type Tree = BTreeMap<PathBuf, Vec<u8>>;

fn write_file(path: &Path, bytes: impl AsRef<[u8]>) {
    fs::create_dir_all(path.parent().unwrap()).unwrap();
    fs::write(path, bytes).unwrap();
}

fn read_tree(root: &Path) -> Tree {
    fn visit(root: &Path, dir: &Path, out: &mut Tree) {
        for entry in fs::read_dir(dir).unwrap() {
            let path = entry.unwrap().path();
            if path.is_dir() {
                visit(root, &path, out);
            } else {
                out.insert(
                    path.strip_prefix(root).unwrap().to_owned(),
                    fs::read(path).unwrap(),
                );
            }
        }
    }
    let mut out = Tree::new();
    if root.exists() {
        visit(root, root, &mut out);
    }
    out
}

fn fixture(base: &Path) -> (PathBuf, PathBuf) {
    let source = base.join("wow-ui-source/wrath");
    let addons = base.join("blizzard-ui/wrath/AddOns");
    for (path, bytes) in SELECTED {
        write_file(&source.join(path), bytes);
    }
    write_file(&source.join("AddOns/Unselected.lua"), b"do not copy\n");
    write_file(
        &source.join("FrameXML/Unselected.lua"),
        b"do not copy sibling\n",
    );
    let pin: Value = serde_json::from_str(PIN).unwrap();
    let record = json!({"gethe_revision": pin["gethe_revision"]});
    write_file(&source.join(SOURCE_PROVENANCE), record.to_string());
    (source, addons)
}

fn selected_tree() -> Tree {
    SELECTED
        .into_iter()
        .map(|(path, bytes)| (PathBuf::from(path), bytes.to_vec()))
        .collect()
}

#[test]
fn wrath_copy_empty_profile_selected_bytes_and_repeat_present() {
    let temp = tempfile::tempdir().unwrap();
    let (source, addons) = fixture(temp.path());
    let runtime = addons.parent().unwrap();
    fs::create_dir_all(runtime).unwrap();
    let source_before = read_tree(&source);
    let copied = copy_from_snapshot(&source, &addons, MANIFEST).unwrap();
    assert_eq!(
        copied,
        CopyCounts {
            total: 3,
            copied: 3,
            present: 0
        }
    );
    assert_eq!(read_tree(runtime), selected_tree());
    let repeated = copy_from_snapshot(&source, &addons, MANIFEST).unwrap();
    assert_eq!(
        repeated,
        CopyCounts {
            total: 3,
            copied: 0,
            present: 3
        }
    );
    assert_eq!(read_tree(runtime), selected_tree());
    assert_eq!(read_tree(&source), source_before);
}

#[test]
fn wrath_copy_reuses_identical_bytes_and_preserves_unrelated_files() {
    let temp = tempfile::tempdir().unwrap();
    let (source, addons) = fixture(temp.path());
    write_file(&addons.join("Blizzard_Test/Test.lua"), SELECTED[0].1);
    write_file(&addons.join("local-keep"), b"unrelated existing file\n");
    let result = copy_from_snapshot(&source, &addons, MANIFEST).unwrap();
    assert_eq!(
        result,
        CopyCounts {
            total: 3,
            copied: 2,
            present: 1
        }
    );
    let mut expected = selected_tree();
    expected.insert(
        PathBuf::from("AddOns/local-keep"),
        b"unrelated existing file\n".to_vec(),
    );
    assert_eq!(read_tree(addons.parent().unwrap()), expected);
}

#[test]
fn wrath_copy_conflicting_existing_bytes_error_without_mutation() {
    let temp = tempfile::tempdir().unwrap();
    let (source, addons) = fixture(temp.path());
    let runtime = addons.parent().unwrap();
    write_file(
        &runtime.join("FrameXML/UIParent.lua"),
        b"existing different bytes\n",
    );
    let before = read_tree(runtime);
    assert!(copy_from_snapshot(&source, &addons, MANIFEST).is_err());
    assert_eq!(read_tree(runtime), before);
}

#[test]
fn wrath_copy_missing_source_fails_without_runtime_publication() {
    for missing in [
        "",
        "AddOns/Blizzard_Test/Test.lua",
        "FrameXML/UIParent.lua",
        SOURCE_PROVENANCE,
    ] {
        let temp = tempfile::tempdir().unwrap();
        let (source, addons) = fixture(temp.path());
        let path = source.join(missing);
        if path.is_dir() {
            fs::remove_dir_all(path).unwrap();
        } else {
            fs::remove_file(path).unwrap();
        }
        assert!(
            copy_from_snapshot(&source, &addons, MANIFEST).is_err(),
            "{missing}"
        );
        assert!(read_tree(addons.parent().unwrap()).is_empty());
    }
}

#[test]
fn wrath_copy_mismatched_source_revision_fails_without_mutation() {
    let temp = tempfile::tempdir().unwrap();
    let (source, addons) = fixture(temp.path());
    let path = source.join(SOURCE_PROVENANCE);
    let mut record: Value = serde_json::from_slice(&fs::read(&path).unwrap()).unwrap();
    record["gethe_revision"] = json!("0000000000000000000000000000000000000000");
    fs::write(path, record.to_string()).unwrap();
    let before = read_tree(&source);
    assert!(copy_from_snapshot(&source, &addons, MANIFEST).is_err());
    assert!(read_tree(addons.parent().unwrap()).is_empty());
    assert_eq!(read_tree(&source), before);
}

#[test]
fn wrath_copy_preserves_other_profile_trees() {
    let temp = tempfile::tempdir().unwrap();
    let (source, addons) = fixture(temp.path());
    for (base, profile) in [
        ("wow-ui-source", "classic"),
        ("wow-ui-source", "forever"),
        ("blizzard-ui", "mists"),
        ("blizzard-ui", "wowforever"),
    ] {
        write_file(
            &temp.path().join(base).join(profile).join("sentinel"),
            profile,
        );
    }
    copy_from_snapshot(&source, &addons, MANIFEST).unwrap();
    for (base, profile) in [
        ("wow-ui-source", "classic"),
        ("wow-ui-source", "forever"),
        ("blizzard-ui", "mists"),
        ("blizzard-ui", "wowforever"),
    ] {
        let path = temp.path().join(base).join(profile).join("sentinel");
        assert_eq!(fs::read(path).unwrap(), profile.as_bytes());
    }
}
