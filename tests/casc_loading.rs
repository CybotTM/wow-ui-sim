//! Integration tests for CASC asset loading (textures + fonts).
//!
//! These tests require a real WoW install discoverable by
//! [`asset_resolver::wow_install_path`] (`WOW_INSTALL_PATH` /
//! `WOW_DATA_PATH` env, otherwise the built-in candidate list). When no
//! install is found they print a skip message and pass — CI without CASC
//! is expected to skip rather than fail.
//!
//! Tests are gated on the `casc` Cargo feature; with `--no-default-features`
//! the file compiles to nothing.

#![cfg(feature = "casc")]

use wow_ui_sim::texture::TextureManager;

fn casc_available() -> bool {
    if std::env::var("WOW_SIM_CASC").ok().as_deref() == Some("0") {
        eprintln!("skipping: WOW_SIM_CASC=0");
        return false;
    }
    let Some(install) = asset_resolver::wow_install_path() else {
        eprintln!("skipping: no WoW install discovered");
        return false;
    };
    eprintln!("CASC tests using install at {}", install.display());
    true
}

fn casc_resolution_contains_fdid(fdid: u32) -> bool {
    let Some(install) = asset_resolver::wow_install_path() else {
        return false;
    };
    asset_resolver::casc_resolver::open_resolution_cache_for_install(install)
        .ok()
        .is_some_and(|cache| cache.resolve_fdid(fdid).is_some())
}

#[test]
fn casc_resolves_baseline_textures() {
    if !casc_available() {
        return;
    }

    let mut mgr = TextureManager::new();
    let probes = [
        ("Interface\\Buttons\\UI-Panel-Button-Up", 128, 32),
        ("Interface\\DialogFrame\\UI-DialogBox-Background", 64, 64),
        ("Interface\\Icons\\INV_Misc_QuestionMark", 64, 64),
    ];

    for (path, want_w, want_h) in probes {
        let td = mgr
            .load(path)
            .unwrap_or_else(|| panic!("CASC failed to resolve {path}"));
        assert_eq!(
            (td.width, td.height),
            (want_w, want_h),
            "{path}: dims mismatch"
        );
    }
}

#[test]
fn casc_resolves_legacy_encoding_key_fallback_textures() {
    if !casc_available() {
        return;
    }

    let mut mgr = TextureManager::new();
    let probes = [
        "Interface\\Common\\CurrencyWindow",
        "Interface\\FrameGeneral\\UIFrameMetal2x",
        "Interface\\FrameGeneral\\UIFrameMetalHorizontal2x",
        "Interface\\FrameGeneral\\UIFrameMetalVertical2x",
        "Interface\\FrameGeneral\\UIFrameTabs",
        "Interface\\Options\\OptionsExpandListButton",
        "Interface\\PaperDollInfoFrame\\PaperDollInfoPart1",
    ];

    for path in probes {
        let td = mgr
            .load(path)
            .unwrap_or_else(|| panic!("CASC fallback failed to resolve {path}"));
        assert!(td.width > 0, "{path}: expected non-zero width");
        assert!(td.height > 0, "{path}: expected non-zero height");
    }
}

#[test]
fn casc_resolves_baseline_fonts() {
    if !casc_available() {
        return;
    }

    let probes = [
        "Fonts/FRIZQT__.TTF",
        "Fonts/ARIALN.TTF",
        "Fonts/FRIZQT___CYR.TTF",
    ];
    let resolver = wow_ui_sim::asset_resolver_config::resolver();
    for path in probes {
        let fdid = resolver
            .lookup_path(path)
            .unwrap_or_else(|| panic!("listfile miss for {path}"));
        if !casc_resolution_contains_fdid(fdid) {
            eprintln!("skipping {path}: fdid {fdid} missing from CASC resolution cache");
            continue;
        }
        let bytes = resolver
            .resolve_bytes(fdid)
            .unwrap_or_else(|| panic!("resolve_bytes failed for {path} (fdid {fdid})"));
        assert!(
            !bytes.is_empty(),
            "{path} (fdid {fdid}) returned empty bytes"
        );
    }
}
