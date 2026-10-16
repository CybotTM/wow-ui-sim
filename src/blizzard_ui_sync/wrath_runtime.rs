//! Wrath runtime cache publication, separate from source acquisition.
use super::wrath_runtime_copy::CopyCounts;
use std::{
    io,
    path::{Path, PathBuf},
};

pub(super) fn sync_from_snapshot(
    _snapshot: &Path,
    _addons: &Path,
    _manifest: &str,
) -> io::Result<CopyCounts> {
    Err(io::Error::new(
        io::ErrorKind::Unsupported,
        "Wrath runtime publication not implemented",
    ))
}

pub(super) fn read_cached_addons_path(
    _addons: &Path,
    _manifest: &str,
) -> io::Result<Option<PathBuf>> {
    Err(io::Error::new(
        io::ErrorKind::Unsupported,
        "Wrath cache acceptance not implemented",
    ))
}

#[cfg(test)]
#[path = "wrath_runtime_tests.rs"]
mod tests;
