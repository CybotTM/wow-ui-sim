//! WoW client profile selection — retail, wrath, mists, era, anniversary.
//!
//! Exactly one `client-*` cargo feature must be enabled. The active profile
//! determines which profile-scoped Blizzard UI cache the addon loader reads.

use std::path::{Path, PathBuf};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ClientProfile {
    Retail,
    Wrath,
    Mists,
    Era,
    Anniversary,
}

impl ClientProfile {
    pub fn subdir(self) -> &'static str {
        match self {
            ClientProfile::Retail => "Retail",
            ClientProfile::Wrath => "Wrath",
            ClientProfile::Mists => "Mists",
            ClientProfile::Era => "Era",
            ClientProfile::Anniversary => "Anniversary",
        }
    }

    pub fn cache_subdir(self) -> &'static str {
        match self {
            ClientProfile::Retail => "retail",
            ClientProfile::Wrath => "wrath",
            ClientProfile::Mists => "mists",
            ClientProfile::Era => "era",
            ClientProfile::Anniversary => "anniversary",
        }
    }

    pub const fn interface_version(self) -> u32 {
        match self {
            ClientProfile::Retail => 120005,
            ClientProfile::Wrath => 38001,
            ClientProfile::Mists => 50504,
            ClientProfile::Era | ClientProfile::Anniversary => 11507,
        }
    }
}

#[cfg(all(
    not(feature = "client-wrath"),
    not(feature = "client-mists"),
    not(feature = "client-era"),
    not(feature = "client-anniversary"),
    feature = "client-retail",
))]
pub const ACTIVE: ClientProfile = ClientProfile::Retail;

#[cfg(all(
    not(feature = "client-retail"),
    not(feature = "client-mists"),
    not(feature = "client-era"),
    not(feature = "client-anniversary"),
    feature = "client-wrath",
))]
pub const ACTIVE: ClientProfile = ClientProfile::Wrath;

#[cfg(all(
    not(feature = "client-retail"),
    not(feature = "client-wrath"),
    not(feature = "client-era"),
    not(feature = "client-anniversary"),
    feature = "client-mists",
))]
pub const ACTIVE: ClientProfile = ClientProfile::Mists;

#[cfg(all(
    not(feature = "client-retail"),
    not(feature = "client-wrath"),
    not(feature = "client-mists"),
    not(feature = "client-anniversary"),
    feature = "client-era",
))]
pub const ACTIVE: ClientProfile = ClientProfile::Era;

#[cfg(all(
    not(feature = "client-retail"),
    not(feature = "client-wrath"),
    not(feature = "client-mists"),
    not(feature = "client-era"),
    feature = "client-anniversary",
))]
pub const ACTIVE: ClientProfile = ClientProfile::Anniversary;

#[cfg(any(
    all(feature = "client-retail", feature = "client-wrath"),
    all(feature = "client-retail", feature = "client-mists"),
    all(feature = "client-retail", feature = "client-era"),
    all(feature = "client-retail", feature = "client-anniversary"),
    all(feature = "client-wrath", feature = "client-mists"),
    all(feature = "client-wrath", feature = "client-era"),
    all(feature = "client-wrath", feature = "client-anniversary"),
    all(feature = "client-mists", feature = "client-era"),
    all(feature = "client-mists", feature = "client-anniversary"),
    all(feature = "client-era", feature = "client-anniversary"),
))]
compile_error!(
    "Exactly one of client-retail, client-wrath, client-mists, client-era, client-anniversary must be enabled"
);

#[cfg(not(any(
    feature = "client-retail",
    feature = "client-wrath",
    feature = "client-mists",
    feature = "client-era",
    feature = "client-anniversary",
)))]
compile_error!(
    "Exactly one of client-retail, client-wrath, client-mists, client-era, client-anniversary must be enabled"
);

/// Path to the AddOns directory for the active profile.
///
/// Prefer a completed cache-managed Blizzard UI source tree for every client
/// profile, otherwise return the profile-scoped default cache path so startup
/// can sync it from CASC.
pub fn blizzard_ui_addons_dir() -> PathBuf {
    blizzard_ui_addons_dir_under_with_cache(
        &PathBuf::from(env!("CARGO_MANIFEST_DIR")),
        crate::blizzard_ui_sync::cached_blizzard_ui_addons_path(),
    )
}

/// Absolute path to the AddOns directory for the active profile, anchored at `root`.
///
/// Tests typically pass `Path::new(env!("CARGO_MANIFEST_DIR"))` so the path resolves
/// regardless of the test's current working directory.
pub fn blizzard_ui_addons_dir_under(root: &Path) -> PathBuf {
    blizzard_ui_addons_dir_under_with_cache(
        root,
        crate::blizzard_ui_sync::cached_blizzard_ui_addons_path(),
    )
}

fn blizzard_ui_addons_dir_under_with_cache(root: &Path, cache_path: Option<PathBuf>) -> PathBuf {
    if let Some(cache_path) = cache_path {
        return cache_path;
    }

    crate::blizzard_ui_sync::default_cache_addons_path().unwrap_or_else(|_| {
        root.join(".cache/wow-ui-sim/blizzard-ui")
            .join(ACTIVE.cache_subdir())
            .join("AddOns")
    })
}

/// Path to the FrameXML.toc under the active profile, if it exists on disk.
///
/// Wrath ships its UI as a flat `Interface/FrameXML/` tree alongside `Interface/AddOns/`;
/// retail and mists collapsed FrameXML into `Blizzard_*` addons and have no top-level
/// FrameXML directory. Callers use this to load a synthetic "FrameXML" addon before
/// the regular Blizzard_* discovery pass.
pub fn blizzard_ui_framexml_toc() -> Option<PathBuf> {
    let addons_dir = blizzard_ui_addons_dir();
    [
        addons_dir.join("FrameXML").join("FrameXML.toc"),
        addons_dir
            .parent()
            .unwrap_or(&addons_dir)
            .join("FrameXML")
            .join("FrameXML.toc"),
    ]
    .into_iter()
    .find(|toc| toc.exists())
}

#[cfg(test)]
#[cfg_attr(not(feature = "client-mists"), allow(unused_imports))]
mod tests {
    use super::*;

    #[test]
    #[cfg(feature = "client-mists")]
    fn mists_prefers_completed_cache_over_default_cache_path() {
        let root = tempfile::tempdir().expect("tempdir");
        let cache_path = root.path().join("cache/blizzard-ui/mists/AddOns");
        let resolved =
            blizzard_ui_addons_dir_under_with_cache(root.path(), Some(cache_path.clone()));

        assert_eq!(resolved, cache_path);
    }

    #[test]
    fn missing_cache_resolves_to_profile_scoped_cache_path() {
        let root = tempfile::tempdir().expect("tempdir");
        let resolved = blizzard_ui_addons_dir_under_with_cache(root.path(), None);

        assert!(
            resolved.ends_with(Path::new(ACTIVE.cache_subdir()).join("AddOns")),
            "Blizzard UI fallback path should be profile-scoped cache AddOns root, got {}",
            resolved.display()
        );
    }
}
