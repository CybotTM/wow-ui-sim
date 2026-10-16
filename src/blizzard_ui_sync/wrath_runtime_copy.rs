//! Copy selected pinned Wrath source bytes without replacing existing content.
//! Caller owns the trusted source/runtime parents and serializes publication.
use std::{
    fs,
    io::{self, Write},
    path::{Path, PathBuf},
};

const PIN: &str = include_str!("../../data/blizzard-ui-builds/wrath.json");
const SOURCE_PROVENANCE: &str = ".wow-ui-sim-wrath-source.json";

#[derive(Debug, PartialEq, Eq)]
pub(super) struct CopyCounts {
    pub total: usize,
    pub copied: usize,
    pub present: usize,
}

struct PendingFile {
    destination: PathBuf,
    bytes: Vec<u8>,
    present: bool,
}

pub(super) fn copy_from_snapshot(
    snapshot: &Path,
    addons: &Path,
    manifest: &str,
) -> io::Result<CopyCounts> {
    inspect_snapshot(snapshot)?;
    let runtime = inspect_runtime_parent(addons)?;
    let pending: Vec<_> = selected_paths(manifest)?
        .into_iter()
        .map(|relative| read_pending_file(snapshot, runtime, &relative))
        .collect::<io::Result<_>>()?;
    for file in &pending {
        if !file.present {
            write_new_file(file)?;
        }
    }
    let present = pending.iter().filter(|file| file.present).count();
    Ok(CopyCounts {
        total: pending.len(),
        copied: pending.len() - present,
        present,
    })
}

fn invalid(message: impl Into<String>) -> io::Error {
    io::Error::new(io::ErrorKind::InvalidData, message.into())
}

fn inspect_snapshot(snapshot: &Path) -> io::Result<()> {
    let pin: serde_json::Value = serde_json::from_str(PIN).map_err(invalid_json)?;
    let bytes = fs::read(snapshot.join(SOURCE_PROVENANCE))?;
    let provenance: serde_json::Value = serde_json::from_slice(&bytes).map_err(invalid_json)?;
    if provenance["gethe_revision"].as_str() != pin["gethe_revision"].as_str() {
        return Err(invalid(
            "Wrath source snapshot does not match its committed pin",
        ));
    }
    if !snapshot.join("AddOns").is_dir() || !snapshot.join("FrameXML/FrameXML.toc").is_file() {
        return Err(invalid(
            "Wrath source requires AddOns and FrameXML/FrameXML.toc",
        ));
    }
    Ok(())
}

fn invalid_json(cause: serde_json::Error) -> io::Error {
    invalid(format!("invalid Wrath source identity: {cause}"))
}

fn inspect_runtime_parent(addons: &Path) -> io::Result<&Path> {
    let parent = addons
        .parent()
        .ok_or_else(|| invalid("missing profile parent"))?;
    if addons.file_name() != Some(std::ffi::OsStr::new("AddOns"))
        || parent.file_name() != Some(std::ffi::OsStr::new("wrath"))
    {
        return Err(invalid("Wrath runtime destination must be wrath/AddOns"));
    }
    Ok(parent)
}

pub(super) fn selected_paths(manifest: &str) -> io::Result<Vec<PathBuf>> {
    manifest
        .lines()
        .map(str::trim)
        .filter(|line| !line.is_empty())
        .map(|entry| {
            if entry
                .split('/')
                .any(|part| matches!(part, "" | "." | "..") || part.contains(['\\', ':', '\0']))
            {
                return Err(invalid(format!("unsafe Wrath manifest entry: {entry}")));
            }
            if entry.starts_with("FrameXML/") {
                Ok(PathBuf::from(entry))
            } else {
                Ok(Path::new("AddOns").join(entry))
            }
        })
        .collect()
}

fn read_pending_file(snapshot: &Path, runtime: &Path, relative: &Path) -> io::Result<PendingFile> {
    let source = snapshot.join(relative);
    let bytes = fs::read(&source).map_err(|cause| {
        io::Error::new(
            cause.kind(),
            format!("read Wrath source {}: {cause}", source.display()),
        )
    })?;
    let destination = runtime.join(relative);
    let present = match fs::read(&destination) {
        Ok(existing) if existing == bytes => true,
        Ok(_) => {
            return Err(invalid(format!(
                "refusing conflicting Wrath file {}",
                destination.display()
            )));
        }
        Err(cause) if cause.kind() == io::ErrorKind::NotFound => false,
        Err(cause) => return Err(cause),
    };
    Ok(PendingFile {
        destination,
        bytes,
        present,
    })
}

fn write_new_file(file: &PendingFile) -> io::Result<()> {
    let parent = file
        .destination
        .parent()
        .ok_or_else(|| invalid("file has no parent"))?;
    fs::create_dir_all(parent)?;
    fs::OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(&file.destination)?
        .write_all(&file.bytes)
}

#[cfg(test)]
#[path = "wrath_runtime_copy_tests.rs"]
mod tests;
