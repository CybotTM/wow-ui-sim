//! Pinned Wrath source extraction only. Caller owns downloading and cache routing.
use flate2::read::GzDecoder;
use sha2::{Digest, Sha256};
use std::{
    fs,
    io::{self, Read},
    path::{Path, PathBuf},
};

const PIN: &str = include_str!("../../data/blizzard-ui-builds/wrath.json");
const PROVENANCE: &str = ".wow-ui-sim-wrath-source.json";

/// Writes a fresh `wrath` snapshot, never replaces one. Parent must already exist,
/// be trusted and remain exclusively owned by the caller throughout extraction.
/// Provenance is published last; this is not an atomic whole-directory transaction.
pub(super) fn extract_wrath_snapshot(archive: &[u8], snapshot: &Path) -> io::Result<()> {
    require_fresh_wrath_destination(snapshot)?;
    let parent = snapshot
        .parent()
        .ok_or_else(|| invalid("snapshot has no parent"))?;
    let staging = parent.join(".wrath-source-extracting");
    fs::create_dir(&staging)?;
    let result = write_snapshot_staging(archive, &staging)
        .and_then(|()| publish_snapshot(&staging, snapshot));
    let cleanup = fs::remove_dir_all(&staging);
    match (result, cleanup) {
        (result, Ok(())) => result,
        (Ok(()), Err(error)) => Err(error),
        (Err(error), Err(cleanup)) => Err(io::Error::other(format!(
            "Wrath extraction: {error}; remove {}: {cleanup}",
            staging.display()
        ))),
    }
}

fn invalid(message: impl Into<String>) -> io::Error {
    io::Error::new(io::ErrorKind::InvalidData, message.into())
}

fn require_fresh_wrath_destination(snapshot: &Path) -> io::Result<()> {
    if snapshot.file_name() != Some(std::ffi::OsStr::new("wrath")) {
        return Err(invalid("destination must be a fresh wrath snapshot"));
    }
    match fs::symlink_metadata(snapshot) {
        Err(error) if error.kind() == io::ErrorKind::NotFound => Ok(()),
        Err(error) => Err(error),
        Ok(_) => Err(io::Error::new(
            io::ErrorKind::AlreadyExists,
            format!("refusing existing snapshot {}", snapshot.display()),
        )),
    }
}

fn write_snapshot_staging(bytes: &[u8], staging: &Path) -> io::Result<()> {
    let revision = pinned_revision()?;
    let file_count = extract_archive_roots(bytes, staging, &revision)?;
    fs::write(
        staging.join(PROVENANCE),
        archive_provenance(bytes, &revision, file_count),
    )
}

fn pinned_revision() -> io::Result<String> {
    let pin: serde_json::Value = serde_json::from_str(PIN)
        .map_err(|cause| invalid(format!("invalid Wrath source pin: {cause}")))?;
    let revision = pin["gethe_revision"]
        .as_str()
        .ok_or_else(|| invalid("Wrath source pin has no Gethe revision"))?;
    if revision.len() != 40 || !revision.bytes().all(|byte| byte.is_ascii_hexdigit()) {
        return Err(invalid("invalid Wrath Gethe revision"));
    }
    Ok(revision.to_owned())
}

fn extract_archive_roots(bytes: &[u8], staging: &Path, revision: &str) -> io::Result<usize> {
    let archive_root = format!("wow-ui-source-{revision}");
    let mut archive = tar::Archive::new(GzDecoder::new(bytes));
    let mut populated = [false; 2];
    let mut file_count = 0;
    for entry in archive.entries()? {
        let mut entry = entry?;
        if entry.header().entry_type().is_pax_global_extensions() {
            validate_revision_comment(&mut entry, revision)?;
            continue;
        }
        let relative = select_runtime_path(&entry, &archive_root)?;
        if let Some((path, root_index)) = relative {
            write_runtime_entry(&mut entry, &staging.join(path))?;
            let is_file = entry.header().entry_type().is_file();
            populated[root_index] |= is_file;
            file_count += usize::from(is_file);
        }
    }
    // tar ends before the gzip trailer. Drain to validate CRC and truncation too.
    let mut decoder = archive.into_inner();
    io::copy(&mut decoder, &mut io::sink())?;
    validate_snapshot_roots(populated, staging)?;
    Ok(file_count)
}

fn validate_snapshot_roots(populated: [bool; 2], staging: &Path) -> io::Result<()> {
    if !populated.into_iter().all(|present| present) {
        return Err(invalid("archive must populate both AddOns and FrameXML"));
    }
    if !staging.join("FrameXML/FrameXML.toc").is_file() {
        return Err(invalid("Wrath snapshot has no FrameXML/FrameXML.toc"));
    }
    Ok(())
}

fn validate_revision_comment<R: Read>(
    entry: &mut tar::Entry<'_, R>,
    revision: &str,
) -> io::Result<()> {
    if entry.path_bytes().as_ref() != b"pax_global_header" {
        return Err(invalid("unexpected global PAX header"));
    }
    let expected = format!("52 comment={revision}\n");
    let mut comment = Vec::new();
    entry.read_to_end(&mut comment)?;
    if comment != expected.as_bytes() {
        return Err(invalid(
            "global PAX metadata must contain only the pinned revision comment",
        ));
    }
    Ok(())
}

fn select_runtime_path<R: Read>(
    entry: &tar::Entry<'_, R>,
    archive_root: &str,
) -> io::Result<Option<(PathBuf, usize)>> {
    let kind = entry.header().entry_type();
    if !(kind.is_file() || kind.is_dir()) || entry.link_name_bytes().is_some() {
        return Err(invalid(
            "archive links and non-file/directory entries are forbidden",
        ));
    }
    let bytes = entry.path_bytes();
    let name = std::str::from_utf8(&bytes).map_err(|_| invalid("non-UTF-8 archive path"))?;
    let name = name.strip_suffix('/').unwrap_or(name);
    let parts = split_archive_path(name, archive_root)?;
    if parts.len() == 1 && !kind.is_dir() {
        return Err(invalid("archive root is not a directory"));
    }
    if parts.len() == 1 {
        return Ok(None);
    }
    let root_index = match parts[1] {
        "AddOns" => 0,
        "FrameXML" => 1,
        _ => return Ok(None),
    };
    if parts.len() == 2 && !kind.is_dir() {
        return Err(invalid("runtime root is not a directory"));
    }
    Ok(Some((parts[1..].iter().collect(), root_index)))
}

fn split_archive_path<'a>(name: &'a str, archive_root: &str) -> io::Result<Vec<&'a str>> {
    let parts: Vec<_> = name.split('/').collect();
    if parts.iter().any(|part| unsafe_component(part)) {
        return Err(invalid(format!("unsafe archive path: {name}")));
    }
    if parts[0] != archive_root {
        return Err(invalid(format!(
            "entry outside pinned archive root: {name}"
        )));
    }
    Ok(parts)
}

fn unsafe_component(part: &str) -> bool {
    matches!(part, "" | "." | "..") || part.contains(['\\', ':', '\0'])
}

fn write_runtime_entry<R: Read>(entry: &mut tar::Entry<'_, R>, output: &Path) -> io::Result<()> {
    if entry.header().entry_type().is_dir() {
        return fs::create_dir_all(output);
    }
    let parent = output
        .parent()
        .ok_or_else(|| invalid("entry has no parent"))?;
    fs::create_dir_all(parent)?;
    let mut file = fs::OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(output)?;
    io::copy(entry, &mut file)?;
    Ok(())
}

fn archive_provenance(bytes: &[u8], revision: &str, file_count: usize) -> String {
    serde_json::json!({
        "schema": 1,
        "profile": "wrath",
        "gethe_revision": revision,
        "source_url": format!("https://codeload.github.com/Gethe/wow-ui-source/tar.gz/{revision}"),
        "archive_sha256": format!("{:x}", Sha256::digest(bytes)),
        "roots": ["AddOns", "FrameXML"],
        "file_count": file_count,
    })
    .to_string()
}

fn publish_snapshot(staging: &Path, snapshot: &Path) -> io::Result<()> {
    // Atomic reservation refuses all pre-existing destinations, including symlinks.
    fs::create_dir(snapshot)?;
    let result = ["AddOns", "FrameXML", PROVENANCE]
        .into_iter()
        .try_for_each(|name| fs::rename(staging.join(name), snapshot.join(name)));
    if let Err(error) = result {
        fs::remove_dir_all(snapshot).map_err(|cleanup| {
            io::Error::other(format!(
                "publish {}: {error}; remove incomplete snapshot: {cleanup}",
                snapshot.display()
            ))
        })?;
        return Err(error);
    }
    Ok(())
}
