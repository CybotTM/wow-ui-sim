#[path = "wrath_snapshot.rs"]
mod source;
use flate2::{Compression, write::GzEncoder};
use sha2::{Digest, Sha256};
use source::extract_wrath_snapshot;
use std::{fs, path::Path};
const REV: &str = "c4e0255fc574598428ee1c25f160ab949798bc98";
const ROOT: &str = "wow-ui-source-c4e0255fc574598428ee1c25f160ab949798bc98";
const PROVENANCE: &str = ".wow-ui-sim-wrath-source.json";

fn fixture(dir: &Path, extra: Option<(&str, u8)>, omit: &str) -> Vec<u8> {
    let file = fs::File::create(dir.join("fixture.tar.gz")).unwrap();
    let mut tar = tar::Builder::new(GzEncoder::new(file, Compression::default()));
    let paths = [
        format!("{ROOT}/AddOns/Blizzard_Test/Test.lua"),
        format!("{ROOT}/FrameXML/UIParent.lua"),
        format!("{ROOT}/FrameXML/FrameXML.toc"),
        format!("{ROOT}/Other/ignored"),
    ];
    let mut entries: Vec<_> = paths
        .iter()
        .filter(|p| !p.contains(omit))
        .map(|p| (p.as_str(), b'0'))
        .collect();
    entries.insert(0, ("pax_global_header", b'g'));
    entries.insert(1, (ROOT, b'5'));
    entries.extend(extra);
    for (path, kind) in entries {
        // Construct raw names so unsafe fixtures reach the extractor, not set_path.
        let mut h = tar::Header::new_gnu();
        assert!(path.len() < 100);
        h.as_mut_bytes()[..path.len()].copy_from_slice(path.as_bytes());
        h.set_entry_type(tar::EntryType::new(kind));
        h.set_mode(0o644);
        let body = match kind {
            b'g' => format!("52 comment={REV}\n").into_bytes(),
            b'0' => b"wrath\n".to_vec(),
            _ => Vec::new(),
        };
        h.set_size(body.len() as u64);
        if matches!(kind, b'1' | b'2') {
            h.set_link_name("../../classic/sentinel").unwrap();
        }
        h.set_cksum();
        tar.append(&h, body.as_slice()).unwrap();
    }
    tar.into_inner().unwrap().finish().unwrap();
    fs::read(dir.join("fixture.tar.gz")).unwrap()
}
fn seed(dir: &Path) {
    for name in ["classic", "forever", "unrelated"] {
        fs::create_dir(dir.join(name)).unwrap();
        fs::write(dir.join(name).join("sentinel"), name).unwrap();
    }
}
fn preserved(dir: &Path) {
    for name in ["classic", "forever", "unrelated"] {
        assert_eq!(
            fs::read(dir.join(name).join("sentinel")).unwrap(),
            name.as_bytes()
        );
    }
}
#[test]
fn wrath_snapshot_extracts_only_roots_and_records_provenance() {
    let d = tempfile::tempdir().unwrap();
    seed(d.path());
    let bytes = fixture(d.path(), None, "not-present");
    let out = d.path().join("wrath");
    extract_wrath_snapshot(&bytes, &out).unwrap();
    for path in [
        "AddOns/Blizzard_Test/Test.lua",
        "FrameXML/UIParent.lua",
        "FrameXML/FrameXML.toc",
    ] {
        assert_eq!(fs::read(out.join(path)).unwrap(), b"wrath\n");
    }
    let mut names: Vec<_> = fs::read_dir(&out)
        .unwrap()
        .map(|e| e.unwrap().file_name())
        .collect();
    names.sort();
    assert_eq!(
        names,
        [PROVENANCE, "AddOns", "FrameXML"].map(std::ffi::OsString::from)
    );
    let digest = format!("{:x}", Sha256::digest(&bytes));
    let provenance: serde_json::Value =
        serde_json::from_slice(&fs::read(out.join(PROVENANCE)).unwrap()).unwrap();
    assert_eq!(provenance["schema"].as_u64(), Some(1));
    assert_eq!(provenance["profile"].as_str(), Some("wrath"));
    assert_eq!(provenance["gethe_revision"].as_str(), Some(REV));
    assert_eq!(provenance["archive_sha256"].as_str(), Some(digest.as_str()));
    assert_eq!(
        provenance["roots"],
        serde_json::json!(["AddOns", "FrameXML"])
    );
    assert_eq!(provenance["file_count"].as_u64(), Some(3));
    preserved(d.path());
}
#[test]
fn wrath_snapshot_rejects_unsafe_entries_before_publication() {
    let cases = [
        ("../escape".to_owned(), b'0'),
        ("/absolute".to_owned(), b'0'),
        ("wrong-root/FrameXML/file".to_owned(), b'0'),
        (format!("{ROOT}/AddOns/../escape"), b'0'),
        (format!("{ROOT}/AddOns\\escape"), b'0'),
        (format!("{ROOT}/C:/escape"), b'0'),
        (format!("{ROOT}/Other/link"), b'2'),
        (format!("{ROOT}/AddOns/link"), b'1'),
        (format!("{ROOT}/FrameXML/pipe"), b'6'),
        (format!("{ROOT}/Other/global"), b'g'),
    ];
    for (path, kind) in cases {
        let d = tempfile::tempdir().unwrap();
        seed(d.path());
        let bytes = fixture(d.path(), Some((&path, kind)), "not-present");
        assert!(
            extract_wrath_snapshot(&bytes, &d.path().join("wrath")).is_err(),
            "{path}"
        );
        assert!(!d.path().join("wrath").exists());
        assert!(!d.path().join("escape").exists());
        preserved(d.path());
    }
}
#[test]
fn wrath_snapshot_requires_both_roots_and_complete_gzip() {
    for omit in ["AddOns", "FrameXML", "FrameXML.toc", "not-present"] {
        let d = tempfile::tempdir().unwrap();
        let mut bytes = fixture(d.path(), None, omit);
        if omit == "not-present" {
            bytes.truncate(bytes.len() - 4);
        }
        assert!(extract_wrath_snapshot(&bytes, &d.path().join("wrath")).is_err());
        assert!(!d.path().join("wrath").exists());
    }
}
#[test]
fn wrath_snapshot_preserves_existing_and_other_profile_destinations() {
    let d = tempfile::tempdir().unwrap();
    seed(d.path());
    let bytes = fixture(d.path(), None, "not-present");
    fs::create_dir(d.path().join("wrath")).unwrap();
    fs::write(d.path().join("wrath/keep"), b"keep").unwrap();
    for name in [
        "wrath",
        "classic",
        "forever",
        "unrelated",
        "new-other-profile",
    ] {
        assert!(extract_wrath_snapshot(&bytes, &d.path().join(name)).is_err());
    }
    assert_eq!(fs::read(d.path().join("wrath/keep")).unwrap(), b"keep");
    assert!(!d.path().join("new-other-profile").exists());
    preserved(d.path());
}
