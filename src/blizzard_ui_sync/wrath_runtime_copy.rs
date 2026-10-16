//! Copy the selected pinned Wrath source into its separate runtime profile.
use std::{io, path::Path};

#[derive(Debug, PartialEq, Eq)]
pub(super) struct CopyCounts {
    pub total: usize,
    pub copied: usize,
    pub present: usize,
}

pub(super) fn copy_from_snapshot(
    _snapshot: &Path,
    _addons: &Path,
    _manifest: &str,
) -> io::Result<CopyCounts> {
    Err(io::Error::new(
        io::ErrorKind::Unsupported,
        "Wrath runtime source copying not implemented",
    ))
}

#[cfg(test)]
#[path = "wrath_runtime_copy_tests.rs"]
mod tests;
