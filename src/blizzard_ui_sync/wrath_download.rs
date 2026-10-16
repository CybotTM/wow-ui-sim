//! Pinned Wrath source acquisition.
use std::{
    fs, io,
    path::{Path, PathBuf},
};
const PIN: &str = include_str!("../../data/blizzard-ui-builds/wrath.json");
const PROVENANCE: &str = ".wow-ui-sim-wrath-source.json";

fn read_pinned_revision() -> io::Result<String> {
    let pin: serde_json::Value = serde_json::from_str(PIN)
        .map_err(|cause| io::Error::new(io::ErrorKind::InvalidData, cause))?;
    pin["gethe_revision"]
        .as_str()
        .map(str::to_owned)
        .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidData, "Wrath pin has no revision"))
}

fn acquire_snapshot_at(_snapshot: &Path) -> io::Result<PathBuf> {
    Err(io::Error::new(
        io::ErrorKind::Unsupported,
        "Wrath source acquisition not implemented",
    ))
}

#[cfg(test)]
#[path = "wrath_download_tests.rs"]
mod tests;
