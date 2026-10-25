use super::*;
use std::process::Command;

const ENTRY: &str = "Blizzard_Test/Example.lua";
const VERSION: &str = "1.60.1.70532";
const HELLO_MD5: &str = "5d41402abc4b2a76b9719d911017c592";

fn git(root: &Path, args: &[&str]) -> String {
    let output = Command::new("git")
        .arg("-C")
        .arg(root)
        .args(args)
        .output()
        .unwrap();
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    String::from_utf8(output.stdout).unwrap().trim().to_owned()
}

fn fixture(version: &str, bytes: &[u8]) -> (tempfile::TempDir, GetheTree) {
    let dir = tempfile::tempdir().unwrap();
    let file = dir.path().join("Interface/AddOns").join(ENTRY);
    std::fs::create_dir_all(file.parent().unwrap()).unwrap();
    std::fs::write(file, bytes).unwrap();
    std::fs::write(dir.path().join("version.txt"), format!("{version}\n")).unwrap();
    git(dir.path(), &["init", "--quiet"]);
    git(dir.path(), &["add", "."]);
    git(
        dir.path(),
        &[
            "-c",
            "user.name=Fixture",
            "-c",
            "user.email=fixture@example.invalid",
            "commit",
            "--quiet",
            "-m",
            "fixture",
        ],
    );
    let revision = git(dir.path(), &["rev-parse", "HEAD"]);
    let tree = GetheTree {
        root: dir.path().to_owned(),
        revision,
        version: VERSION.to_owned(),
    };
    (dir, tree)
}

fn file(resident: bool) -> MappedFile {
    MappedFile {
        path: ENTRY.to_owned(),
        fdid: 8379893,
        content_key: HELLO_MD5.to_owned(),
        resident,
    }
}

#[test]
fn nonresident_matching_gethe_is_written_and_recorded() {
    let (_source, tree) = fixture(VERSION, b"hello");
    let output = tempfile::tempdir().unwrap();
    let destination = output.path().join(ENTRY);
    let revision = tree.revision.clone();
    let (result, record) = sync_mapped_file(
        || Ok(Some(file(false))),
        &destination,
        || unreachable!(),
        || Ok(tree),
    )
    .unwrap();
    assert!(matches!(result, super::super::EntrySyncResult::Extracted));
    assert_eq!(std::fs::read(&destination).unwrap(), b"hello");
    let record = record.unwrap();
    let mut provenance = super::super::CacheProvenance {
        contents: "schema=fixture\n".to_owned(),
    };
    append_substitution(&mut provenance, &record).unwrap();
    let line = provenance
        .contents()
        .lines()
        .nth(1)
        .unwrap()
        .strip_prefix("gethe_file=")
        .unwrap();
    assert_eq!(
        serde_json::from_str::<serde_json::Value>(line).unwrap(),
        serde_json::json!({
            "path": ENTRY, "fdid": 8379893, "ckey": HELLO_MD5, "gethe_revision": revision, "source": "gethe"
        })
    );
}

#[test]
fn nonresident_mismatching_gethe_fails_without_writing() {
    let (_source, tree) = fixture(VERSION, b"wrong");
    let output = tempfile::tempdir().unwrap();
    let destination = output.path().join(ENTRY);
    let error = sync_mapped_file(
        || Ok(Some(file(false))),
        &destination,
        || unreachable!(),
        || Ok(tree),
    )
    .err()
    .unwrap();
    assert_eq!(
        error.to_string(),
        "Gethe MD5 mismatch for Blizzard_Test/Example.lua (FDID 8379893): CKey=5d41402abc4b2a76b9719d911017c592, Gethe=2bda2998d9b0ee197da142a0447f6725"
    );
    assert!(!destination.exists());
}

#[test]
fn resident_uses_casc_with_no_gethe_tree() {
    let output = tempfile::tempdir().unwrap();
    let destination = output.path().join(ENTRY);
    let missing_tree = output.path().join("absent-gethe");
    let (result, record) = sync_mapped_file(
        || Ok(Some(file(true))),
        &destination,
        || {
            super::super::write_extracted_casc_path(&destination, b"resident CASC bytes")?;
            Ok(super::super::EntrySyncResult::Extracted)
        },
        || {
            Ok(GetheTree {
                root: missing_tree,
                revision: "0".repeat(40),
                version: VERSION.to_owned(),
            })
        },
    )
    .unwrap();
    assert!(matches!(result, super::super::EntrySyncResult::Extracted));
    assert_eq!(std::fs::read(destination).unwrap(), b"resident CASC bytes");
    assert!(record.is_none());
}

#[test]
fn mismatched_gethe_version_fails_without_writing() {
    let (_source, tree) = fixture("1.60.1.70374", b"hello");
    let output = tempfile::tempdir().unwrap();
    let destination = output.path().join(ENTRY);
    let error = sync_mapped_file(
        || Ok(Some(file(false))),
        &destination,
        || unreachable!(),
        || Ok(tree),
    )
    .err()
    .unwrap();
    assert_eq!(
        error.to_string(),
        "Gethe version mismatch: expected 1.60.1.70532, found 1.60.1.70374"
    );
    assert!(!destination.exists());
}

#[test]
fn pinned_revision_ignores_modified_working_tree_bytes() {
    let (source, tree) = fixture(VERSION, b"hello");
    std::fs::write(
        source.path().join("Interface/AddOns").join(ENTRY),
        b"uncommitted",
    )
    .unwrap();
    std::fs::write(source.path().join("version.txt"), b"wrong version").unwrap();
    let output = tempfile::tempdir().unwrap();
    let destination = output.path().join(ENTRY);
    sync_mapped_file(
        || Ok(Some(file(false))),
        &destination,
        || unreachable!(),
        || Ok(tree),
    )
    .unwrap();
    assert_eq!(std::fs::read(destination).unwrap(), b"hello");
}

#[test]
fn substitution_provenance_preserves_identity_comparison() {
    let (_source, tree) = fixture(VERSION, b"hello");
    let output = tempfile::tempdir().unwrap();
    let destination = output.path().join(ENTRY);
    let (_, record) = sync_mapped_file(
        || Ok(Some(file(false))),
        &destination,
        || unreachable!(),
        || Ok(tree),
    )
    .unwrap();
    let base = "schema=1\ngethe_revision=pinned\n";
    let mut provenance = super::super::CacheProvenance {
        contents: base.to_owned(),
    };
    append_substitution(&mut provenance, &record.unwrap()).unwrap();
    assert!(provenance_matches(provenance.contents(), base));
    assert!(!provenance_matches(
        provenance.contents(),
        "schema=1\ngethe_revision=other\n"
    ));
    assert!(!provenance_matches(
        &format!("{base}unrecognized=bad\n"),
        base
    ));
}
