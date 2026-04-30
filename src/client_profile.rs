//! WoW client profile selection — retail, wrath, mists, era, anniversary.
//!
//! Exactly one `client-*` cargo feature must be enabled. The active profile
//! determines which `Interface/BlizzardUI/<Profile>/` subdir the addon loader
//! reads vendor sources from.

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

/// Path to `Interface/BlizzardUI/<Profile>` relative to the repo root.
pub fn blizzard_ui_root() -> PathBuf {
    PathBuf::from("./Interface/BlizzardUI").join(ACTIVE.subdir())
}

/// Path to the AddOns directory under the active profile root.
pub fn blizzard_ui_addons_dir() -> PathBuf {
    blizzard_ui_root().join("AddOns")
}

/// Absolute path to the AddOns directory under the active profile, anchored at `root`.
///
/// Tests typically pass `Path::new(env!("CARGO_MANIFEST_DIR"))` so the path resolves
/// regardless of the test's current working directory.
pub fn blizzard_ui_addons_dir_under(root: &Path) -> PathBuf {
    root.join("Interface/BlizzardUI")
        .join(ACTIVE.subdir())
        .join("AddOns")
}

/// Path to the FrameXML.toc under the active profile, if it exists on disk.
///
/// Wrath ships its UI as a flat `Interface/FrameXML/` tree alongside `Interface/AddOns/`;
/// retail and mists collapsed FrameXML into `Blizzard_*` addons and have no top-level
/// FrameXML directory. Callers use this to load a synthetic "FrameXML" addon before
/// the regular Blizzard_* discovery pass.
pub fn blizzard_ui_framexml_toc() -> Option<PathBuf> {
    let toc = blizzard_ui_root().join("FrameXML").join("FrameXML.toc");
    toc.exists().then_some(toc)
}
