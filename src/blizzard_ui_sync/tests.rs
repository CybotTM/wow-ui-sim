
use super::{
    copy_repo_fallback_entry_from_root, manifest_entries, manifest_entry_fdid,
    manifest_entry_is_repo_fallback_only, normalize_source_root, unpack_wow_ui_source_archive,
};
use flate2::Compression;
use flate2::write::GzEncoder;
use std::io;
use std::path::PathBuf;
use std::time::{SystemTime, UNIX_EPOCH};

fn unique_temp_dir(label: &str) -> PathBuf {
    let unique = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("system time before unix epoch")
        .as_nanos();
    std::env::temp_dir().join(format!(
        "wow-ui-sim-blizzard-ui-sync-{label}-{}-{unique}",
        std::process::id()
    ))
}

fn build_test_archive() -> io::Result<Vec<u8>> {
    let encoder = GzEncoder::new(Vec::new(), Compression::default());
    let mut builder = tar::Builder::new(encoder);
    write_tar_file(
        &mut builder,
        "wow-ui-source-live/Interface/AddOns/Blizzard_Test/Test.lua",
        b"from archive\n",
    )?;
    write_tar_file(
        &mut builder,
        "wow-ui-source-live/README.md",
        b"not an addon\n",
    )?;
    builder.into_inner()?.finish()
}

fn write_tar_file(
    builder: &mut tar::Builder<GzEncoder<Vec<u8>>>,
    path: &str,
    contents: &[u8],
) -> io::Result<()> {
    let mut header = tar::Header::new_gnu();
    header.set_size(contents.len() as u64);
    header.set_mode(0o644);
    header.set_cksum();
    builder.append_data(&mut header, path, contents)
}

#[test]
fn default_cache_addons_path_is_profile_scoped_addons_root() {
    let path = super::default_cache_addons_path().expect("cache path");

    assert!(
        path.ends_with(PathBuf::from(crate::client_profile::ACTIVE.cache_subdir()).join("AddOns")),
        "cache path should end with profile/AddOns, got {}",
        path.display()
    );
}

#[test]
fn complete_marker_writes_profile_provenance() {
    let root = unique_temp_dir("provenance");

    super::write_complete_marker(&root).expect("write complete marker");

    let provenance =
        std::fs::read_to_string(root.join(super::PROVENANCE_FILE)).expect("read provenance");
    assert!(provenance.contains(&format!(
        "profile={}",
        crate::client_profile::ACTIVE.cache_subdir()
    )));
    assert!(provenance.contains("source=casc-primary"));
    assert!(provenance.contains("fallback=wow-ui-source"));
    std::fs::remove_dir_all(root).expect("remove cache root");
}

#[test]
fn manifest_preserves_blizzard_addon_case() {
    let first = manifest_entries()
        .next()
        .expect("manifest should not be empty");
    assert!(first.starts_with("Blizzard_"));
}
#[test]
fn manifest_entries_resolve_through_limited_listfile() {
    let missing: Vec<_> = manifest_entries()
        .filter(|entry| manifest_entry_fdid(entry).is_none())
        .filter(|entry| !manifest_entry_is_repo_fallback_only(entry))
        .take(10)
        .collect();
    assert!(
        missing.is_empty(),
        "unmapped Blizzard UI files: {missing:?}"
    );
}

#[test]
#[cfg(feature = "client-mists")]
fn mists_required_cache_entries_are_in_manifest() {
    let manifest: std::collections::HashSet<_> = manifest_entries().collect();

    for entry in super::required_profile_cache_entries() {
        assert!(
            manifest.contains(entry),
            "Mists cache-required file must be synced by the Blizzard UI manifest: {entry}"
        );
    }
}
#[test]
#[cfg(feature = "client-mists")]
fn mists_cache_is_incomplete_when_required_profile_files_are_missing() {
    let root = unique_temp_dir("mists-required-files");
    std::fs::create_dir_all(&root).expect("create cache root");

    assert!(
        !super::cache_has_required_profile_files(&root),
        "Mists cache marker must not be trusted when profile-required TOC files are absent"
    );

    std::fs::remove_dir_all(root).expect("remove cache root");
}

#[test]
#[cfg(feature = "client-mists")]
fn mists_cache_rejects_old_classic_action_button_template() {
    let root = unique_temp_dir("mists-action-button-template");
    write_mists_required_cache_entries(&root);

    let action_button_template = root.join("Blizzard_ActionBar/Classic/ActionButtonTemplate.xml");
    std::fs::write(&action_button_template, "placeholder").expect("write placeholder");
    assert!(
        !super::cache_has_required_profile_files(&root),
        "Mists cache marker must not be trusted when ActionButtonTemplate.xml is the old Classic Era variant"
    );

    std::fs::write(
            action_button_template,
            r#"<CheckButton name="ActionBarButtonTemplate"><Cooldown parentKey="chargeCooldown"/></CheckButton>"#,
        )
        .expect("write Mists-compatible action button template");
    assert!(
        super::cache_has_required_profile_files(&root),
        "Mists cache should be complete when required files exist and ActionButtonTemplate.xml defines ActionBarButtonTemplate"
    );

    std::fs::remove_dir_all(root).expect("remove cache root");
}

#[test]
#[cfg(feature = "client-mists")]
fn mists_cache_rejects_mainline_nameplates_toc_without_game_type_gates() {
    let root = unique_temp_dir("mists-nameplates-toc");
    write_mists_required_cache_entries(&root);

    let nameplates_toc = root.join("Blizzard_NamePlates/Blizzard_NamePlates.toc");
    std::fs::write(&nameplates_toc, "Blizzard_ClassNameplateBar.lua\n")
        .expect("write ungated nameplates toc");
    assert!(
        !super::cache_has_required_profile_files(&root),
        "Mists cache marker must not be trusted when Blizzard_NamePlates.toc would load Mainline class bar files"
    );

    std::fs::write(
        nameplates_toc,
        "Mainline\\Blizzard_ClassNameplateBar.lua [AllowLoadGameType mainline]\n",
    )
    .expect("write Mists-compatible nameplates toc");
    assert!(
        super::cache_has_required_profile_files(&root),
        "Mists cache should be complete when Blizzard_NamePlates.toc preserves Mainline game-type gates"
    );

    std::fs::remove_dir_all(root).expect("remove cache root");
}

#[test]
#[cfg(feature = "client-mists")]
fn mists_prefers_mop_classic_repo_fallbacks() {
    assert_eq!(
        super::gethe_wow_ui_source_branches().first().copied(),
        Some("classic_ptr"),
        "Mists fallback sync must prefer the source branch matching Mists ActionButton.lua"
    );
}

#[cfg(feature = "client-mists")]
include!("blizzard_ui_sync_mists_test_fixture.rs");

#[test]
fn repo_fallback_copies_manifest_entry_from_addons_root() {
    let source_root = unique_temp_dir("source");
    let out_root = unique_temp_dir("out");
    let entry = "Blizzard_Test/Test.lua";
    let source_path = source_root.join(entry);
    let out_path = out_root.join(entry);
    std::fs::create_dir_all(source_path.parent().expect("source parent"))
        .expect("create source parent");
    std::fs::write(&source_path, "from repo\n").expect("write source fallback");
    let copied = copy_repo_fallback_entry_from_root(entry, &out_path, &source_root)
        .expect("copy fallback entry");

    assert!(copied);
    assert_eq!(
        std::fs::read_to_string(&out_path).expect("read copied fallback"),
        "from repo\n"
    );
    std::fs::remove_dir_all(source_root).expect("remove source temp dir");
    std::fs::remove_dir_all(out_root).expect("remove output temp dir");
}

#[test]
fn repo_fallback_accepts_gethe_repo_root() {
    let repo_root = unique_temp_dir("repo");
    let addons_root = repo_root.join("Interface/AddOns");
    std::fs::create_dir_all(&addons_root).expect("create addons root");

    let normalized = normalize_source_root(repo_root.clone());

    assert_eq!(normalized, Some(addons_root));
    std::fs::remove_dir_all(repo_root).expect("remove repo temp dir");
}

#[test]
fn archive_unpack_extracts_only_interface_addons() {
    let repo_root = unique_temp_dir("archive");
    let archive = build_test_archive().expect("build test archive");

    unpack_wow_ui_source_archive(&archive[..], &repo_root).expect("unpack archive");

    assert_eq!(
        std::fs::read_to_string(repo_root.join("Interface/AddOns/Blizzard_Test/Test.lua"))
            .expect("read unpacked addon file"),
        "from archive\n"
    );
    assert!(
        !repo_root.join("README.md").exists(),
        "non-addon archive file should not be unpacked"
    );
    std::fs::remove_dir_all(repo_root).expect("remove repo temp dir");
}
