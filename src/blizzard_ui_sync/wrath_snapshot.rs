use std::{io, path::Path};

pub(super) fn extract_wrath_snapshot(_archive: &[u8], _snapshot: &Path) -> io::Result<()> {
    Err(io::Error::new(
        io::ErrorKind::Unsupported,
        "Wrath extraction not implemented",
    ))
}
