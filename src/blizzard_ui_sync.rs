//! CASC-backed Blizzard UI source synchronization.

mod profile_cache;

use self::profile_cache::{
    cache_entry_is_usable, gethe_wow_ui_source_branches, required_profile_cache_entries,
};
use std::io::{self, Read};
use std::path::{Component, Path, PathBuf};
#[cfg(feature = "casc")]
use std::sync::OnceLock;
use std::time::Duration;

const BLIZZARD_UI_MANIFEST: &str = include_str!("../data/blizzard-ui-files.txt");
const COMPLETE_MARKER: &str = ".wow-ui-sim-blizzard-ui-complete";
const PROVENANCE_FILE: &str = ".wow-ui-sim-blizzard-ui-provenance";
const GETHE_ARCHIVE_DOWNLOAD_RETRIES: usize = 3;
const GETHE_ARCHIVE_USER_AGENT: &str = concat!("wow-ui-sim/", env!("CARGO_PKG_VERSION"));
#[cfg(feature = "casc")]
static CASC_CONFIGURED: OnceLock<bool> = OnceLock::new();

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SyncSummary {
    pub root: PathBuf,
    pub total: usize,
    pub extracted: usize,
    pub present: usize,
    pub missing: usize,
}

pub fn default_cache_addons_path() -> crate::Result<PathBuf> {
    dirs::cache_dir()
        .map(|dir| {
            dir.join("wow-ui-sim/blizzard-ui")
                .join(crate::client_profile::ACTIVE.cache_subdir())
                .join("AddOns")
        })
        .ok_or_else(|| crate::Error::Other("could not determine user cache directory".to_string()))
}

pub fn cached_blizzard_ui_addons_path() -> Option<PathBuf> {
    let path = default_cache_addons_path().ok()?;
    let is_complete = path.join(COMPLETE_MARKER).is_file();
    (is_complete && cache_has_required_profile_files(&path)).then_some(path)
}

fn cache_has_required_profile_files(root: &Path) -> bool {
    required_profile_cache_entries().iter().all(|entry| {
        let path = root.join(entry);
        path.is_file() && cache_entry_is_usable(entry, &path)
    })
}

pub fn sync_blizzard_ui() -> crate::Result<SyncSummary> {
    let root = default_cache_addons_path()?;
    sync_blizzard_ui_to(&root)
}

pub fn sync_blizzard_ui_to(root: &Path) -> crate::Result<SyncSummary> {
    sync_blizzard_ui_entries(root, manifest_entries())
}

pub fn manifest_entries() -> impl Iterator<Item = &'static str> {
    BLIZZARD_UI_MANIFEST
        .lines()
        .map(str::trim)
        .filter(|line| !line.is_empty())
}

fn sync_blizzard_ui_entries<'a>(
    root: &Path,
    entries: impl Iterator<Item = &'a str>,
) -> crate::Result<SyncSummary> {
    #[cfg(feature = "casc")]
    if !casc_available() {
        return Err(crate::Error::WowInstallNotFound);
    }

    let mut summary = SyncSummary {
        root: root.to_path_buf(),
        total: 0,
        extracted: 0,
        present: 0,
        missing: 0,
    };
    let mut fallback_source = RepoFallbackSource::default();
    let mut last_missing_entry: Option<String> = None;

    for entry in entries {
        summary.total += 1;
        match sync_manifest_entry(root, entry, &mut fallback_source)? {
            EntrySyncResult::Present => summary.present += 1,
            EntrySyncResult::Extracted => summary.extracted += 1,
            EntrySyncResult::Missing => {
                summary.missing += 1;
                last_missing_entry = Some(entry.to_string());
            }
        }
    }

    if summary.missing > 0 {
        return Err(crate::Error::BlizzardUiPartial {
            missing: summary.missing,
            total: summary.total,
            last_error: last_missing_entry
                .unwrap_or_else(|| "unknown extraction failure".to_string()),
        });
    }

    write_complete_marker(root)?;
    Ok(summary)
}
enum EntrySyncResult {
    Present,
    Extracted,
    Missing,
}
fn sync_manifest_entry(
    root: &Path,
    entry: &str,
    fallback_source: &mut RepoFallbackSource,
) -> crate::Result<EntrySyncResult> {
    let out_path = root.join(entry);
    if entry_is_present_and_usable(entry, &out_path) {
        return Ok(EntrySyncResult::Present);
    }

    if extract_manifest_entry(entry, &out_path)? && entry_is_present_and_usable(entry, &out_path) {
        return Ok(EntrySyncResult::Extracted);
    }

    if fallback_source.copy_entry(entry, &out_path)? {
        return Ok(EntrySyncResult::Extracted);
    }

    if let Some(fallback) = synthesized_fallback_content_for(entry) {
        write_synthesized_fallback(&out_path, fallback)?;
        return Ok(EntrySyncResult::Extracted);
    }

    Ok(EntrySyncResult::Missing)
}

fn entry_is_present_and_usable(entry: &str, path: &Path) -> bool {
    path.is_file() && cache_entry_is_usable(entry, path)
}

fn extract_manifest_entry(entry: &str, out_path: &Path) -> crate::Result<bool> {
    match manifest_entry_fdid(entry) {
        Some(fdid) => extract_fdid(fdid, out_path),
        None => Ok(false),
    }
}

#[derive(Default)]
struct RepoFallbackSource {
    roots: Option<Vec<PathBuf>>,
}

impl RepoFallbackSource {
    fn copy_entry(&mut self, entry: &str, out_path: &Path) -> crate::Result<bool> {
        for root in self.roots()?.iter() {
            if copy_repo_fallback_entry_from_root(entry, out_path, root)?
                && entry_is_present_and_usable(entry, out_path)
            {
                return Ok(true);
            }
        }
        Ok(false)
    }
    fn roots(&mut self) -> crate::Result<&[PathBuf]> {
        if self.roots.is_none() {
            self.roots = Some(repo_fallback_roots()?);
        }
        Ok(self.roots.as_deref().expect("fallback roots initialized"))
    }
}
fn copy_repo_fallback_entry_from_root(
    entry: &str,
    out_path: &Path,
    root: &Path,
) -> crate::Result<bool> {
    if let Some(root) = normalize_source_root(root.to_path_buf()) {
        let source_path = root.join(normalize_manifest_entry(entry));
        if source_path.is_file() {
            copy_fallback_file(&source_path, out_path)?;
            return Ok(true);
        }
    }
    Ok(false)
}

fn repo_fallback_roots() -> crate::Result<Vec<PathBuf>> {
    let mut roots = Vec::new();
    if let Some(root) = std::env::var_os("WOW_SIM_BLIZZARD_UI_SOURCE_DIR").map(PathBuf::from) {
        roots.push(root);
    }

    for branch in gethe_wow_ui_source_branches() {
        roots.push(ensure_cached_wow_ui_source_branch(branch)?);
    }
    Ok(roots)
}

fn ensure_cached_wow_ui_source_branch(branch: &str) -> crate::Result<PathBuf> {
    let repo_root = cached_wow_ui_source_path(branch)?;
    if repo_root.join("Interface/AddOns").is_dir() {
        return Ok(repo_root);
    }
    download_wow_ui_source_archive(branch, &repo_root)?;
    Ok(repo_root)
}

fn cached_wow_ui_source_path(branch: &str) -> crate::Result<PathBuf> {
    dirs::cache_dir()
        .map(|dir| dir.join("wow-ui-sim/wow-ui-source").join(branch))
        .ok_or_else(|| crate::Error::Other("could not determine user cache directory".to_string()))
}

fn download_wow_ui_source_archive(branch: &str, repo_root: &Path) -> crate::Result<()> {
    let parent = repo_root.parent().ok_or_else(|| {
        crate::Error::Other(format!(
            "Blizzard UI source cache path has no parent: {}",
            repo_root.display()
        ))
    })?;
    std::fs::create_dir_all(parent).map_err(|e| {
        crate::Error::Other(format!(
            "could not create Blizzard UI source cache directory {}: {e}",
            parent.display()
        ))
    })?;

    let temp_root = repo_root.with_extension("download");
    remove_existing_path(&temp_root)?;

    let archive_url = wow_ui_source_archive_url(branch);
    let response = download_archive_response(&archive_url)?;
    unpack_wow_ui_source_archive(response, &temp_root)?;

    remove_existing_path(repo_root)?;
    std::fs::rename(&temp_root, repo_root).map_err(|e| {
        crate::Error::Other(format!(
            "could not install Blizzard UI source cache {}: {e}",
            repo_root.display()
        ))
    })
}

fn wow_ui_source_archive_url(branch: &str) -> String {
    format!("https://github.com/Gethe/wow-ui-source/archive/refs/heads/{branch}.tar.gz")
}

fn download_archive_response(url: &str) -> crate::Result<impl Read> {
    let client = reqwest::blocking::Client::builder()
        .user_agent(GETHE_ARCHIVE_USER_AGENT)
        .timeout(Duration::from_secs(60))
        .build()
        .map_err(|e| crate::Error::Other(format!("could not build HTTP client: {e}")))?;

    let mut last_error = None;
    for attempt in 1..=GETHE_ARCHIVE_DOWNLOAD_RETRIES {
        match client
            .get(url)
            .send()
            .and_then(reqwest::blocking::Response::error_for_status)
        {
            Ok(response) => return Ok(response),
            Err(error) => {
                last_error = Some(error);
                if attempt < GETHE_ARCHIVE_DOWNLOAD_RETRIES {
                    std::thread::sleep(Duration::from_millis(500 * attempt as u64));
                }
            }
        }
    }

    Err(crate::Error::Other(format!(
        "could not download Blizzard UI fallback archive {url}: {}",
        last_error
            .map(|error| error.to_string())
            .unwrap_or_else(|| "unknown error".to_string())
    )))
}

fn unpack_wow_ui_source_archive(reader: impl Read, repo_root: &Path) -> crate::Result<()> {
    let decoder = flate2::read::GzDecoder::new(reader);
    let mut archive = tar::Archive::new(decoder);
    for entry in archive.entries().map_err(|e| {
        crate::Error::Other(format!("could not read Blizzard UI fallback archive: {e}"))
    })? {
        unpack_wow_ui_source_entry(entry, repo_root)?;
    }
    Ok(())
}

fn unpack_wow_ui_source_entry<R: Read>(
    entry: io::Result<tar::Entry<'_, R>>,
    repo_root: &Path,
) -> crate::Result<()> {
    let mut entry = entry.map_err(|e| {
        crate::Error::Other(format!("could not read Blizzard UI fallback entry: {e}"))
    })?;
    let entry_path = entry.path().map_err(|e| {
        crate::Error::Other(format!(
            "could not read Blizzard UI fallback entry path: {e}"
        ))
    })?;
    let Some(relative_addon_path) = archived_addon_path(&entry_path) else {
        return Ok(());
    };
    if entry.header().entry_type().is_dir() {
        return Ok(());
    }

    let out_path = repo_root.join("Interface/AddOns").join(relative_addon_path);
    create_parent_dir(&out_path, "Blizzard UI source")?;
    entry.unpack(&out_path).map_err(|e| {
        crate::Error::Other(format!(
            "could not unpack Blizzard UI source {}: {e}",
            out_path.display()
        ))
    })?;
    Ok(())
}

fn create_parent_dir(path: &Path, label: &str) -> crate::Result<()> {
    let Some(parent) = path.parent() else {
        return Ok(());
    };
    std::fs::create_dir_all(parent).map_err(|e| {
        crate::Error::Other(format!(
            "could not create {label} directory {}: {e}",
            parent.display()
        ))
    })
}

fn archived_addon_path(path: &Path) -> Option<PathBuf> {
    let mut after_addons = false;
    let mut relative = PathBuf::new();
    for component in path.components() {
        match component {
            Component::Normal(name) if after_addons => relative.push(name),
            Component::Normal(name) if name == "AddOns" => after_addons = true,
            Component::CurDir => {}
            _ if after_addons => return None,
            _ => {}
        }
    }
    after_addons
        .then_some(relative)
        .filter(|path| !path.as_os_str().is_empty())
}

fn remove_existing_path(path: &Path) -> crate::Result<()> {
    match std::fs::metadata(path) {
        Ok(metadata) if metadata.is_dir() => std::fs::remove_dir_all(path)
            .map_err(|e| crate::Error::Other(format!("could not remove {}: {e}", path.display()))),
        Ok(_) => std::fs::remove_file(path)
            .map_err(|e| crate::Error::Other(format!("could not remove {}: {e}", path.display()))),
        Err(e) if e.kind() == io::ErrorKind::NotFound => Ok(()),
        Err(e) => Err(crate::Error::Other(format!(
            "could not inspect {}: {e}",
            path.display()
        ))),
    }
}

fn normalize_source_root(path: PathBuf) -> Option<PathBuf> {
    if path.join("Interface/AddOns").is_dir() {
        return Some(path.join("Interface/AddOns"));
    }
    if path.join("AddOns").is_dir() {
        return Some(path.join("AddOns"));
    }
    path.is_dir().then_some(path)
}

fn normalize_manifest_entry(entry: &str) -> PathBuf {
    entry.replace('\\', "/").split('/').collect()
}

fn copy_fallback_file(source_path: &Path, out_path: &Path) -> crate::Result<()> {
    remove_missing_marker(out_path);
    if let Some(parent) = out_path.parent() {
        std::fs::create_dir_all(parent).map_err(|e| {
            crate::Error::Other(format!(
                "could not create Blizzard UI fallback directory {}: {e}",
                parent.display()
            ))
        })?;
    }
    std::fs::copy(source_path, out_path).map_err(|e| {
        crate::Error::Other(format!(
            "could not copy Blizzard UI fallback {} to {}: {e}",
            source_path.display(),
            out_path.display()
        ))
    })?;
    Ok(())
}

/// Synthesized fallbacks for small, well-known Blizzard UI files.
fn synthesized_fallback_content_for(entry: &str) -> Option<&'static str> {
    match entry.replace('\\', "/").as_str() {
        "Blizzard_LoadLocale/LoadLocale.lua" => Some(concat!(
            "-- Synthesized fallback when CASC extraction misses this file.\n",
            "LOCALE_enUS = true;\n",
            "UI_LOCALE = \"enUS\";\n",
        )),
        "Blizzard_LoadLocale/Blizzard_LoadLocale.toc" => Some(concat!(
            "## Title: Blizzard_LoadLocale\n",
            "## Author: Blizzard Entertainment\n",
            "## DefaultState: enabled\n",
            "## AllowLoad: Both\n",
            "LoadLocale.lua\n",
        )),
        _ => None,
    }
}

fn write_synthesized_fallback(out_path: &Path, contents: &str) -> crate::Result<()> {
    remove_missing_marker(out_path);
    if let Some(parent) = out_path.parent() {
        std::fs::create_dir_all(parent).map_err(|e| {
            crate::Error::Other(format!(
                "could not create Blizzard UI fallback directory {}: {e}",
                parent.display()
            ))
        })?;
    }
    std::fs::write(out_path, contents).map_err(|e| {
        crate::Error::Other(format!(
            "could not write Blizzard UI fallback {}: {e}",
            out_path.display()
        ))
    })
}

fn manifest_entry_fdid(entry: &str) -> Option<u32> {
    let asset_path = format!("interface/addons/{}", entry.replace('\\', "/"));
    crate::limited_listfile::lookup_path(&asset_path)
}

#[cfg(test)]
fn manifest_entry_is_repo_fallback_only(entry: &str) -> bool {
    if profile_cache::MISTS_REQUIRED_PROFILE_CACHE_ENTRIES.contains(&entry) {
        return true;
    }

    matches!(
        entry,
        "Blizzard_CooldownBroadcaster/Blizzard_CooldownBroadcaster.lua"
            | "Blizzard_CooldownBroadcaster/Blizzard_CooldownBroadcaster.toc"
            | "Blizzard_CooldownBroadcaster/MessageQueue.lua"
            | "Blizzard_CooldownBroadcaster/TrackedCooldowns.lua"
            | "Blizzard_CombatLog/Wrath/Blizzard_CombatLog.lua"
            | "Blizzard_CombatLog/Wrath/Blizzard_CombatLog.xml"
    )
}

#[cfg(feature = "casc")]
fn extract_fdid(fdid: u32, out_path: &Path) -> crate::Result<bool> {
    if !casc_available() {
        return Err(crate::Error::Other(
            "local WoW CASC data is not available; set WOW_INSTALL_PATH or WOW_DATA_PATH, and make sure WOW_SIM_CASC is not 0".to_string(),
        ));
    }
    remove_missing_marker(out_path);
    let resolver = crate::asset_resolver_config::resolver();
    Ok(resolver.ensure_cached(fdid, out_path).is_some())
}

#[cfg(not(feature = "casc"))]
fn extract_fdid(_fdid: u32, _out_path: &Path) -> crate::Result<bool> {
    Err(crate::Error::Other(
        "Blizzard UI CASC sync requires the `casc` feature".to_string(),
    ))
}

fn write_complete_marker(root: &Path) -> crate::Result<()> {
    std::fs::create_dir_all(root).map_err(|e| {
        crate::Error::Other(format!(
            "could not create Blizzard UI cache directory {}: {e}",
            root.display()
        ))
    })?;
    write_provenance(root)?;
    std::fs::write(root.join(COMPLETE_MARKER), b"ok\n").map_err(|e| {
        crate::Error::Other(format!(
            "could not write Blizzard UI cache marker in {}: {e}",
            root.display()
        ))
    })
}

fn write_provenance(root: &Path) -> crate::Result<()> {
    let contents = format!(
        "profile={}\nsource=casc-primary\nfallback=wow-ui-source\n",
        crate::client_profile::ACTIVE.cache_subdir()
    );
    std::fs::write(root.join(PROVENANCE_FILE), contents).map_err(|e| {
        crate::Error::Other(format!(
            "could not write Blizzard UI cache provenance in {}: {e}",
            root.display()
        ))
    })
}

#[cfg(feature = "casc")]
fn casc_available() -> bool {
    *CASC_CONFIGURED.get_or_init(|| {
        if std::env::var("WOW_SIM_CASC").ok().as_deref() == Some("0") {
            return false;
        }
        asset_resolver::wow_install_path().is_some()
    })
}

fn remove_missing_marker(path: &Path) {
    let extension = path.extension().and_then(|e| e.to_str()).unwrap_or("");
    let missing_marker = path.with_extension(format!("{extension}.missing"));
    if missing_marker.is_file() {
        let _ = std::fs::remove_file(missing_marker);
    }
}

#[cfg(test)]
mod tests;
