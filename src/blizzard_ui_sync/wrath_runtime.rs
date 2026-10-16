//! Wrath runtime cache publication, separate from source acquisition.
use super::{
    COMPLETE_MARKER, PROVENANCE_FILE,
    wrath_runtime_copy::{self, CopyCounts},
};
use sha2::{Digest, Sha256};
use std::{
    fs,
    io::{self, Write},
    path::{Path, PathBuf},
};

const PIN: &str = include_str!("../../data/blizzard-ui-builds/wrath.json");

pub(super) fn sync_from_snapshot(
    snapshot: &Path,
    addons: &Path,
    manifest: &str,
) -> io::Result<CopyCounts> {
    let provenance = runtime_provenance(manifest)?;
    inspect_metadata(&addons.join(PROVENANCE_FILE), provenance.as_bytes())?;
    inspect_metadata(&addons.join(COMPLETE_MARKER), b"ok\n")?;
    let counts = wrath_runtime_copy::copy_from_snapshot(snapshot, addons, manifest)?;
    write_missing_metadata(&addons.join(PROVENANCE_FILE), provenance.as_bytes())?;
    write_missing_metadata(&addons.join(COMPLETE_MARKER), b"ok\n")?;
    Ok(counts)
}

pub(super) fn read_cached_addons_path(
    addons: &Path,
    manifest: &str,
) -> io::Result<Option<PathBuf>> {
    let provenance = runtime_provenance(manifest)?;
    let identity_matches =
        fs::read(addons.join(PROVENANCE_FILE)).is_ok_and(|actual| actual == provenance.as_bytes());
    let completed = fs::read(addons.join(COMPLETE_MARKER)).is_ok_and(|actual| actual == b"ok\n");
    if !identity_matches || !completed {
        return Ok(None);
    }
    let runtime = wrath_runtime_copy::inspect_runtime_parent(addons)?;
    let files_present = wrath_runtime_copy::selected_paths(manifest)?
        .iter()
        .all(|relative| runtime.join(relative).is_file());
    Ok(files_present.then(|| addons.to_owned()))
}

fn runtime_provenance(manifest: &str) -> io::Result<String> {
    let pin: serde_json::Value = serde_json::from_str(PIN)
        .map_err(|cause| io::Error::new(io::ErrorKind::InvalidData, cause))?;
    let revision = pin["gethe_revision"]
        .as_str()
        .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidData, "Wrath pin has no revision"))?;
    let hash = format!("{:x}", Sha256::digest(manifest.as_bytes()));
    Ok(format!(
        "schema=2\nprofile=wrath\ngethe_revision={revision}\nmanifest_sha256={hash}\nsource=pinned-gethe-local\nfallback=none\n"
    ))
}

fn inspect_metadata(path: &Path, expected: &[u8]) -> io::Result<bool> {
    match fs::read(path) {
        Ok(actual) if actual == expected => Ok(true),
        Ok(_) => Err(io::Error::new(
            io::ErrorKind::AlreadyExists,
            format!(
                "refusing conflicting Wrath runtime metadata {}",
                path.display()
            ),
        )),
        Err(cause) if cause.kind() == io::ErrorKind::NotFound => Ok(false),
        Err(cause) => Err(cause),
    }
}

fn write_missing_metadata(path: &Path, bytes: &[u8]) -> io::Result<()> {
    if inspect_metadata(path, bytes)? {
        return Ok(());
    }
    fs::OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(path)?
        .write_all(bytes)
}

#[cfg(test)]
#[path = "wrath_runtime_tests.rs"]
mod tests;
