//! Immutable PTR source identity; Blizzard CDN bytes, never a Gethe content fallback.
use super::CacheProvenance;
use serde::Deserialize;
use sha2::{Digest, Sha256};
use std::path::Path;
use std::sync::OnceLock;

const INDEX: &str = include_str!("../../data/blizzard-ui-builds/ptr.json");

#[cfg(feature = "client-ptr")]
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct ClientIdentity {
    pub version: String,
    pub build: String,
}

#[derive(Deserialize)]
struct Build {
    schema: u32,
    product: String,
    version: String,
    build_key: String,
    cdn_key: String,
    install_key: String,
    gethe_revision: String,
    #[cfg(feature = "casc")]
    files: Vec<super::pinned_download::ContentEntry>,
}

fn build() -> crate::Result<&'static Build> {
    static BUILD: OnceLock<Result<Build, String>> = OnceLock::new();
    BUILD
        .get_or_init(|| {
            let build: Build = serde_json::from_str(INDEX).map_err(|cause| cause.to_string())?;
            let identity = [
                build.product.as_str(),
                build.version.as_str(),
                build.build_key.as_str(),
                build.cdn_key.as_str(),
                build.install_key.as_str(),
                build.gethe_revision.as_str(),
            ];
            let expected = [
                "wowt",
                "12.1.7.70493",
                "fc3eaa61ff0ade6bf35dbab9727bc161",
                "c13b476a09cb3262797a6fdc88103036",
                "bb46ab983520a2617d8b2119662ef38b",
                "2cdbc7ff511ec6a8c2f4f8341accd1da44a3c9af",
            ];
            if build.schema != 1 || identity != expected {
                return Err("unsupported PTR content-index schema or source identity".into());
            }
            #[cfg(feature = "casc")]
            validate_entries(&build)?;
            Ok(build)
        })
        .as_ref()
        .map_err(|cause| crate::Error::Other(cause.clone()))
}

#[cfg(feature = "client-ptr")]
pub(crate) fn client_identity() -> crate::Result<ClientIdentity> {
    let version_and_build = &build()?.version;
    let (version, build) = version_and_build.rsplit_once('.').ok_or_else(|| {
        crate::Error::Other(format!(
            "PTR version has no build suffix: {version_and_build}"
        ))
    })?;
    Ok(ClientIdentity {
        version: version.to_owned(),
        build: build.to_owned(),
    })
}

pub(super) fn provenance() -> crate::Result<CacheProvenance> {
    let build = build()?;
    let manifest_hash = format!("{:x}", Sha256::digest(super::PTR_BLIZZARD_UI_MANIFEST));
    let index_hash = format!("{:x}", Sha256::digest(INDEX));
    Ok(CacheProvenance {
        contents: format!(
            "schema=2\nprofile=ptr\nproduct={}\nversion={}\nbuild_key={}\ninstall_key={}\ncdn_key={}\ngethe_revision={}\nmanifest_sha256={manifest_hash}\ncontent_index_sha256={index_hash}\nsource=blizzard-cdn-pinned\nfallback=none\n",
            build.product,
            build.version,
            build.build_key,
            build.install_key,
            build.cdn_key,
            build.gethe_revision,
        ),
    })
}

pub(super) fn cache_matches(root: &Path) -> bool {
    let Ok(expected) = provenance() else {
        return false;
    };
    std::fs::read_to_string(root.join(super::PROVENANCE_FILE))
        .is_ok_and(|contents| contents == expected.contents())
}

#[cfg(feature = "casc")]
fn validate_entries(build: &Build) -> Result<(), String> {
    let expected: Vec<_> = super::PTR_BLIZZARD_UI_MANIFEST.lines().collect();
    let paths: Vec<_> = build
        .files
        .iter()
        .map(|entry| entry.path.as_str())
        .collect();
    if paths != expected {
        return Err("PTR content index does not match its path manifest".into());
    }
    for entry in &build.files {
        let safe_path = !entry.path.starts_with('/')
            && entry
                .path
                .split('/')
                .all(|part| !matches!(part, "" | "." | "..") && !part.contains('\\'));
        let valid_keys = [&entry.archive, &entry.content_key, &entry.encoding_key]
            .into_iter()
            .all(|key| key.len() == 32 && key.bytes().all(|b| b.is_ascii_hexdigit()));
        if !safe_path
            || !valid_keys
            || entry.size < 9
            || entry.offset.checked_add(entry.size as u64).is_none()
        {
            return Err(format!("invalid pinned CDN entry: {}", entry.path));
        }
    }
    Ok(())
}

#[cfg(feature = "casc")]
pub(super) fn sync_to(root: &Path) -> crate::Result<super::SyncSummary> {
    let build = build()?;
    let expected = provenance()?;
    let client = reqwest::blocking::Client::builder()
        .redirect(reqwest::redirect::Policy::none())
        .timeout(std::time::Duration::from_secs(30))
        .build()
        .map_err(|cause| crate::Error::Other(format!("create Blizzard CDN client: {cause}")))?;
    prepare_sync_cache(root, &expected)?;
    let roots = super::pinned_cdn::fetch_roots(&client, &build.product)?;
    let workers = 16.min(build.files.len()).max(1);
    let counts = std::thread::scope(|scope| {
        let handles: Vec<_> = (0..workers)
            .map(|worker| {
                let client = &client;
                let roots = &roots;
                scope.spawn(move || sync_worker(root, client, &build.files, roots, worker, workers))
            })
            .collect();
        handles
            .into_iter()
            .map(|handle| {
                handle
                    .join()
                    .map_err(|_| crate::Error::Other("PTR source sync worker panicked".into()))?
            })
            .collect::<crate::Result<Vec<_>>>()
    })?;
    let present = counts.iter().map(|counts| counts.0).sum();
    let extracted = counts.iter().map(|counts| counts.1).sum();
    super::write_complete_marker(root, &expected)?;
    Ok(super::SyncSummary {
        root: root.to_path_buf(),
        total: build.files.len(),
        extracted,
        present,
        missing: 0,
    })
}

#[cfg(feature = "casc")]
fn prepare_sync_cache(root: &Path, expected: &CacheProvenance) -> crate::Result<()> {
    super::invalidate_cache_if_provenance_mismatched(root, expected)?;
    super::write_cache_provenance(root, expected)
}

#[cfg(feature = "casc")]
fn sync_worker(
    root: &Path,
    client: &reqwest::blocking::Client,
    entries: &[super::pinned_download::ContentEntry],
    roots: &[String],
    worker: usize,
    workers: usize,
) -> crate::Result<(usize, usize)> {
    let mut present = 0;
    let mut extracted = 0;
    for entry in entries.iter().skip(worker).step_by(workers) {
        let destination = root.join(&entry.path);
        match std::fs::read(&destination) {
            Ok(content) if super::pinned_download::content_matches(entry, &content) => {
                present += 1;
                continue;
            }
            Ok(_) => eprintln!("Replacing mismatched PTR source {}", destination.display()),
            Err(cause) if cause.kind() == std::io::ErrorKind::NotFound => {}
            Err(cause) => {
                return Err(crate::Error::Other(format!(
                    "read {}: {cause}",
                    destination.display()
                )));
            }
        }
        let content = super::pinned_download::download_entry(client, entry, roots)?;
        super::write_extracted_casc_path(&destination, &content)?;
        extracted += 1;
    }
    Ok((present, extracted))
}

#[cfg(not(feature = "casc"))]
pub(super) fn sync_to(_root: &Path) -> crate::Result<super::SyncSummary> {
    Err(super::casc_feature_error())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[cfg(feature = "casc")]
    #[test]
    fn pinned_ptr_partial_cache_child() {
        let Some(root) = std::env::var_os("WOW_UI_SIM_PINNED_PARTIAL_CACHE_CHILD_ROOT") else {
            return;
        };
        let root = Path::new(&root);
        prepare_sync_cache(root, &provenance().unwrap()).unwrap();
        std::fs::create_dir_all(root).unwrap();
        std::fs::write(root.join("partial-source"), b"saved source bytes").unwrap();
        std::process::exit(23);
    }

    #[cfg(feature = "casc")]
    #[test]
    fn pinned_ptr_resumes_partial_cache_after_process_failure_without_completion() {
        let dir = tempfile::tempdir().unwrap();
        let root = dir.path().join("AddOns");
        let status = std::process::Command::new(std::env::current_exe().unwrap())
            .args([
                "--exact",
                "blizzard_ui_sync::pinned::tests::pinned_ptr_partial_cache_child",
                "--nocapture",
            ])
            .env("WOW_UI_SIM_PINNED_PARTIAL_CACHE_CHILD_ROOT", &root)
            .status()
            .unwrap();
        assert_eq!(status.code(), Some(23));
        assert!(!root.join(super::super::COMPLETE_MARKER).exists());
        assert_eq!(
            std::fs::read(root.join("partial-source")).unwrap(),
            b"saved source bytes"
        );

        prepare_sync_cache(&root, &provenance().unwrap()).unwrap();

        assert_eq!(
            std::fs::read(root.join("partial-source")).unwrap(),
            b"saved source bytes"
        );
        assert!(cache_matches(&root));
        assert!(!root.join(super::super::COMPLETE_MARKER).exists());
    }

    #[cfg(feature = "casc")]
    #[test]
    fn pinned_ptr_discards_stale_source_before_preparing_partial_cache() {
        let dir = tempfile::tempdir().unwrap();
        let root = dir.path();
        let stale = provenance()
            .unwrap()
            .contents()
            .replace("version=12.1.7.70493\n", "version=12.1.5.69594\n");
        std::fs::write(root.join(super::super::PROVENANCE_FILE), stale).unwrap();
        std::fs::write(root.join(super::super::COMPLETE_MARKER), b"ok\n").unwrap();
        std::fs::write(root.join("stale-source"), b"old source bytes").unwrap();

        prepare_sync_cache(root, &provenance().unwrap()).unwrap();

        assert!(!root.join("stale-source").exists());
        assert!(!root.join(super::super::COMPLETE_MARKER).exists());
        assert!(cache_matches(root));
    }

    #[test]
    fn pinned_ptr_selects_exact_70493_source_identity() {
        let selected = build().unwrap();
        assert_eq!(selected.product, "wowt");
        assert_eq!(selected.version, "12.1.7.70493");
        assert_eq!(selected.build_key, "fc3eaa61ff0ade6bf35dbab9727bc161");
        assert_eq!(selected.cdn_key, "c13b476a09cb3262797a6fdc88103036");
        assert_eq!(
            selected.gethe_revision,
            "2cdbc7ff511ec6a8c2f4f8341accd1da44a3c9af"
        );
    }

    #[cfg(feature = "casc")]
    #[test]
    fn pinned_ptr_rejects_manifest_index_mismatch() {
        let mut selected: Build = serde_json::from_str(INDEX).unwrap();
        selected.files.pop();
        assert_eq!(
            validate_entries(&selected).unwrap_err(),
            "PTR content index does not match its path manifest"
        );
    }

    #[test]
    fn pinned_ptr_rejects_previous_69594_provenance() {
        let dir = tempfile::tempdir().unwrap();
        let old_identity = provenance()
            .unwrap()
            .contents()
            .replace("product=wowt\n", "product=wowxptr\n")
            .replace("version=12.1.7.70493\n", "version=12.1.5.69594\n")
            .replace(
                "fc3eaa61ff0ade6bf35dbab9727bc161",
                "4a9973f37906f8cfb344f8a9fe6777e0",
            )
            .replace(
                "c13b476a09cb3262797a6fdc88103036",
                "3aa83893a3ce9b722a5f51328ad9a552",
            )
            .replace(
                "2cdbc7ff511ec6a8c2f4f8341accd1da44a3c9af",
                "49b69918fcdc77e109813281e4f537d45ec7dcbf",
            );
        std::fs::write(dir.path().join(super::super::PROVENANCE_FILE), old_identity).unwrap();
        assert!(!cache_matches(dir.path()));
    }

    #[test]
    fn pinned_ptr_rejects_legacy_provenance_and_accepts_exact_identity() {
        let dir = tempfile::tempdir().unwrap();
        std::fs::write(
            dir.path().join(super::super::PROVENANCE_FILE),
            "schema=1\nprofile=ptr\nproduct=wowt\nversion=12.1.0.69587\n",
        )
        .unwrap();
        assert!(!cache_matches(dir.path()));
        std::fs::write(
            dir.path().join(super::super::PROVENANCE_FILE),
            provenance().unwrap().contents(),
        )
        .unwrap();
        assert!(cache_matches(dir.path()));
    }
}
