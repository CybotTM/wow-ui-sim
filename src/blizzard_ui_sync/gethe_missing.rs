//! Contract boundary for local CASC residency and pinned Gethe source bytes.
use super::{CacheProvenance, EntrySyncResult};
use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};

struct MappedFile {
    path: String,
    fdid: u32,
    content_key: String,
    resident: bool,
}

struct GetheTree {
    root: PathBuf,
    revision: String,
    version: String,
}

#[derive(Deserialize, Serialize)]
struct Substitution {
    path: String,
    fdid: u32,
    ckey: String,
    gethe_revision: String,
    source: String,
}

fn sync_mapped_file(
    file: &MappedFile,
    _destination: &Path,
    read_local: impl FnOnce() -> crate::Result<EntrySyncResult>,
    _load_gethe: impl FnOnce() -> crate::Result<GetheTree>,
) -> crate::Result<(EntrySyncResult, Option<Substitution>)> {
    if file.resident {
        return Ok((read_local()?, None));
    }
    Err(crate::Error::Other(format!(
        "local CASC encoding key is not resident for {} (FDID {})",
        file.path, file.fdid
    )))
}

fn append_substitution(
    provenance: &mut CacheProvenance,
    record: &Substitution,
) -> crate::Result<()> {
    let json =
        serde_json::to_string(record).map_err(|error| crate::Error::Other(error.to_string()))?;
    provenance
        .contents
        .push_str(&format!("gethe_file={json}\n"));
    Ok(())
}

pub(super) fn provenance_matches(actual: &str, expected: &str) -> bool {
    let Some(records) = actual.strip_prefix(expected) else {
        return false;
    };
    records.lines().all(|line| {
        line.strip_prefix("gethe_file=")
            .is_some_and(|json| serde_json::from_str::<Substitution>(json).is_ok())
    })
}

#[cfg(test)]
#[path = "gethe_missing_tests.rs"]
mod tests;
