//! Reusable directory listings, scoped to one manager's ordered roots and binary profile.
//! Requests still run the original case-preserving exact probes before indexed CI probes.
//! No request-to-missing result is retained between searches.
use std::collections::{HashMap, HashSet};
use std::path::{Path, PathBuf};

#[derive(Debug, Default)]
pub(super) struct SourceIndex {
    directories: HashMap<PathBuf, DirectorySnapshot>,
}

#[derive(Debug)]
struct DirectorySnapshot {
    stamp: DirectoryStamp,
    complete: bool,
    // Preserve first read_dir match (including directories), just like paths.rs.
    entries: HashMap<String, PathBuf>,
}

#[derive(Debug, PartialEq, Eq)]
struct DirectoryStamp {
    modified: Option<std::time::SystemTime>,
    length: u64,
    #[cfg(unix)]
    identity: (u64, u64, i64, i64, i64, i64),
}

impl SourceIndex {
    pub(super) fn clear(&mut self) {
        self.directories.clear();
    }

    pub(super) fn search(&mut self) -> DirectorySearch<'_> {
        DirectorySearch {
            index: self,
            refreshed: HashSet::new(),
        }
    }
}

pub(super) struct DirectorySearch<'a> {
    index: &'a mut SourceIndex,
    refreshed: HashSet<PathBuf>,
}

impl DirectorySearch<'_> {
    pub(super) fn read_match(&mut self, dir: &Path, name: &str) -> Option<PathBuf> {
        let stamp = read_directory_stamp(dir)?;
        let stale = self
            .index
            .directories
            .get(dir)
            .is_none_or(|snapshot| !snapshot.complete || snapshot.stamp != stamp);
        if stale {
            self.refresh_directory(dir)?;
        }
        let key = name.to_lowercase();
        let found = self.lookup(dir, &key);
        // A missing name always refreshes once per resolution, even if timestamps
        // compare equal. Only this synchronous search can reuse negative evidence.
        let absent = found.as_ref().is_none_or(|path| !path.exists());
        if absent && !self.refreshed.contains(dir) {
            self.refresh_directory(dir)?;
        }
        self.lookup(dir, &key)
    }

    fn lookup(&self, dir: &Path, key: &str) -> Option<PathBuf> {
        self.index.directories.get(dir)?.entries.get(key).cloned()
    }

    fn refresh_directory(&mut self, dir: &Path) -> Option<()> {
        self.index.directories.remove(dir);
        self.refreshed.insert(dir.to_path_buf());
        let stamp = read_directory_stamp(dir)?;
        let (entries, complete) = read_directory_entries(dir)?;
        if read_directory_stamp(dir)? != stamp {
            return None;
        }
        self.index.directories.insert(
            dir.to_path_buf(),
            DirectorySnapshot {
                stamp,
                complete,
                entries,
            },
        );
        Some(())
    }
}

fn read_directory_entries(dir: &Path) -> Option<(HashMap<String, PathBuf>, bool)> {
    let mut entries = HashMap::new();
    let mut complete = true;
    for entry in std::fs::read_dir(dir).ok()? {
        let Ok(entry) = entry else {
            complete = false;
            continue;
        };
        let key = entry.file_name().to_string_lossy().to_lowercase();
        entries.entry(key).or_insert_with(|| entry.path());
    }
    Some((entries, complete))
}

fn read_directory_stamp(dir: &Path) -> Option<DirectoryStamp> {
    let metadata = std::fs::metadata(dir).ok()?;
    if !metadata.is_dir() {
        return None;
    }
    #[cfg(unix)]
    use std::os::unix::fs::MetadataExt;
    Some(DirectoryStamp {
        modified: metadata.modified().ok(),
        length: metadata.len(),
        #[cfg(unix)]
        identity: (
            metadata.dev(),
            metadata.ino(),
            metadata.mtime(),
            metadata.mtime_nsec(),
            metadata.ctime(),
            metadata.ctime_nsec(),
        ),
    })
}
