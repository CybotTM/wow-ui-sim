use std::fs;
use std::path::PathBuf;

fn diff_constants_wrong_path() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("docs")
        .join("wow-client-diff")
        .join("diff_constants_wrong.txt")
}

#[test]
fn diff_constants_wrong_is_empty_after_constant_reconciliation() {
    let contents = fs::read_to_string(diff_constants_wrong_path())
        .expect("failed to read diff_constants_wrong.txt");
    let entries: Vec<&str> = contents
        .lines()
        .map(str::trim)
        .filter(|line| !line.is_empty())
        .collect();

    assert!(
        entries.is_empty(),
        "diff_constants_wrong.txt should be empty after reconciling wrong constants, found: {entries:?}"
    );
}
