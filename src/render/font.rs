//! Font management using cosmic-text.
//!
//! Loads WoW TTF fonts into a cosmic-text FontSystem for text shaping,
//! measurement, and glyph rasterization. Provides mapping from WoW font
//! paths (e.g. `Fonts\\FRIZQT__.TTF`) to fontdb family names.

use std::collections::HashMap;
use std::path::Path;

use casc_lib::blte::decoder::decode_blte_with_keys;
use casc_lib::error::CascError;
use casc_lib::extract::pipeline::{CascStorage, OpenConfig};
use casc_lib::root::flags::LocaleFlags;
use cosmic_text::fontdb;

/// WoW font path constants (as they appear in Lua/XML).
const WOW_FONT_FRIZ: &str = "Fonts\\FRIZQT__.TTF";
const WOW_FONT_ARIAL_NARROW: &str = "Fonts\\ARIALN.TTF";

/// Default WoW font (Friz Quadrata).
pub const DEFAULT_WOW_FONT: &str = WOW_FONT_FRIZ;

/// Font entry mapping a WoW path to a fontdb family name.
#[derive(Debug, Clone)]
struct FontEntry {
    family: String,
}

struct WowFontFile {
    filename: &'static str,
    fdid: u32,
    wow_paths: &'static [&'static str],
}

const WOW_FONT_FILES: &[WowFontFile] = &[
    WowFontFile {
        filename: "FRIZQT__.TTF",
        fdid: 615960,
        wow_paths: &[WOW_FONT_FRIZ, "Fonts\\frizqt__.ttf"],
    },
    WowFontFile {
        filename: "ARIALN.ttf",
        fdid: 615958,
        wow_paths: &[WOW_FONT_ARIAL_NARROW, "Fonts\\arialn.ttf"],
    },
    WowFontFile {
        filename: "frizqt___cyr.ttf",
        fdid: 615971,
        wow_paths: &["Fonts\\frizqt___cyr.ttf"],
    },
];

/// Manages WoW fonts via cosmic-text.
///
/// Holds a `FontSystem` with only the WoW TTF fonts loaded (no system fonts),
/// a `SwashCache` for glyph rasterization, and a mapping from WoW font paths
/// to fontdb family names.
pub struct WowFontSystem {
    pub font_system: cosmic_text::FontSystem,
    pub swash_cache: cosmic_text::SwashCache,
    /// Map from normalized WoW font path (uppercase) to family name.
    font_map: HashMap<String, FontEntry>,
}

impl std::fmt::Debug for WowFontSystem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("WowFontSystem")
            .field("fonts", &self.font_map.keys().collect::<Vec<_>>())
            .finish()
    }
}

impl WowFontSystem {
    /// Create a new font system with WoW fonts loaded from the given directory.
    ///
    /// `fonts_dir` should point to the `fonts/` directory containing TTF files.
    pub fn new(fonts_dir: &Path) -> Self {
        let mut db = fontdb::Database::new();
        let mut font_map = HashMap::new();

        for font_file in WOW_FONT_FILES {
            load_wow_font(fonts_dir, font_file, &mut db, &mut font_map);
        }
        load_system_fallback_if_empty(&mut db, &mut font_map);

        let font_system = cosmic_text::FontSystem::new_with_locale_and_db("en-US".to_string(), db);
        let swash_cache = cosmic_text::SwashCache::new();

        Self {
            font_system,
            swash_cache,
            font_map,
        }
    }

    /// Create a font system by loading WoW fonts from local CASC storage.
    pub fn from_casc_install(install_root: &Path) -> Self {
        let mut db = fontdb::Database::new();
        let mut font_map = HashMap::new();

        match CascFontSource::open(install_root) {
            Ok(source) => {
                for font_file in WOW_FONT_FILES {
                    load_wow_font_from_casc(&source, font_file, &mut db, &mut font_map);
                }
            }
            Err(error) => {
                tracing::warn!(
                    "Failed to open CASC font source at {}: {}",
                    install_root.display(),
                    error
                );
            }
        }
        load_system_fallback_if_empty(&mut db, &mut font_map);

        let font_system = cosmic_text::FontSystem::new_with_locale_and_db("en-US".to_string(), db);
        let swash_cache = cosmic_text::SwashCache::new();

        Self {
            font_system,
            swash_cache,
            font_map,
        }
    }

    /// Create a font system from the default discovered WoW install.
    pub fn from_default_sources() -> Self {
        if let Some(install_root) = crate::paths::discover_wow_resources().install_root {
            return Self::from_casc_install(&install_root);
        }

        tracing::warn!("No WoW install found; text rendering will use cosmic-text fallback fonts");
        let mut db = fontdb::Database::new();
        let mut font_map = HashMap::new();
        load_system_fallback_if_empty(&mut db, &mut font_map);
        let font_system = cosmic_text::FontSystem::new_with_locale_and_db("en-US".to_string(), db);
        let swash_cache = cosmic_text::SwashCache::new();
        Self {
            font_system,
            swash_cache,
            font_map,
        }
    }

    /// Get the fontdb family name for a WoW font path.
    ///
    /// Falls back to the default WoW font (Friz Quadrata) if the path is
    /// unknown. Returns None only if no fonts were loaded at all.
    pub fn family_name(&self, wow_path: Option<&str>) -> Option<&str> {
        let key = normalize_wow_path(wow_path.unwrap_or(DEFAULT_WOW_FONT));
        if let Some(entry) = self.font_map.get(&key) {
            return Some(&entry.family);
        }
        // Fall back to default WoW font
        let default_key = normalize_wow_path(DEFAULT_WOW_FONT);
        self.font_map.get(&default_key).map(|e| e.family.as_str())
    }

    /// Create cosmic-text `Attrs` for a WoW font path.
    ///
    /// The returned `Attrs` borrows the family name from `self`, so it cannot
    /// be held across a `&mut self` call. For use with `Buffer::set_text`,
    /// prefer `attrs_owned()` which returns an `AttrsOwned`.
    pub fn attrs(&self, wow_path: Option<&str>) -> cosmic_text::Attrs<'_> {
        match self.family_name(wow_path) {
            Some(name) => cosmic_text::Attrs::new().family(cosmic_text::Family::Name(name)),
            None => cosmic_text::Attrs::new(),
        }
    }

    /// Create an owned `AttrsOwned` for a WoW font path.
    ///
    /// Use this when you need to pass attrs to functions that also take
    /// `&mut font_system`, since `AttrsOwned` doesn't borrow from self.
    pub fn attrs_owned(&self, wow_path: Option<&str>) -> cosmic_text::AttrsOwned {
        cosmic_text::AttrsOwned::new(&self.attrs(wow_path))
    }

    /// Measure the pixel width of a text string using cosmic-text shaping.
    ///
    /// `font_path` is the WoW font path (e.g. `Fonts\\FRIZQT__.TTF`).
    /// Returns the width of the first layout line.
    pub fn measure_text_width(
        &mut self,
        text: &str,
        font_path: Option<&str>,
        font_size: f32,
    ) -> f32 {
        if text.is_empty() {
            return 0.0;
        }
        let line_height = (font_size * 1.2).ceil();
        let metrics = cosmic_text::Metrics::new(font_size, line_height);
        let attrs = self.attrs_owned(font_path);
        let mut buffer = cosmic_text::Buffer::new(&mut self.font_system, metrics);
        buffer.set_size(&mut self.font_system, Some(10000.0), Some(line_height));
        buffer.set_text(
            &mut self.font_system,
            text,
            &attrs.as_attrs(),
            cosmic_text::Shaping::Advanced,
            None,
        );
        buffer.shape_until_scroll(&mut self.font_system, true);

        buffer
            .layout_runs()
            .map(|run| run.line_w)
            .next()
            .unwrap_or(0.0)
    }

    /// Measure the pixel height of text, accounting for word wrapping.
    ///
    /// If `wrap_width` is Some and > 0, text wraps at that width.
    /// Returns the total height of all layout lines.
    pub fn measure_text_height(
        &mut self,
        text: &str,
        font_path: Option<&str>,
        font_size: f32,
        wrap_width: Option<f32>,
    ) -> f32 {
        if text.is_empty() {
            return 0.0;
        }
        let line_height = (font_size * 1.2).ceil();
        let metrics = cosmic_text::Metrics::new(font_size, line_height);
        let attrs = self.attrs_owned(font_path);
        let shape_width = match wrap_width {
            Some(w) if w > 0.0 => w,
            _ => 10000.0,
        };
        let mut buffer = cosmic_text::Buffer::new(&mut self.font_system, metrics);
        buffer.set_size(&mut self.font_system, Some(shape_width), Some(10000.0));
        buffer.set_text(
            &mut self.font_system,
            text,
            &attrs.as_attrs(),
            cosmic_text::Shaping::Advanced,
            None,
        );
        buffer.shape_until_scroll(&mut self.font_system, true);

        let runs: Vec<_> = buffer.layout_runs().collect();
        let num_lines = runs.len();
        if num_lines <= 1 {
            line_height
        } else {
            runs.last()
                .map(|run| run.line_y + line_height)
                .unwrap_or(line_height)
        }
    }
}

struct CascFontSource {
    storage: CascStorage,
}

impl CascFontSource {
    fn open(install_root: &Path) -> Result<Self, casc_lib::error::CascError> {
        let listfile = casc_font_listfile_path();
        let config = OpenConfig {
            install_dir: install_root.to_path_buf(),
            product: Some("wow".to_string()),
            keyfile: None,
            listfile: Some(listfile),
            output_dir: None,
        };
        CascStorage::open(&config).map(|storage| Self { storage })
    }

    fn read_font(&self, font_file: &WowFontFile) -> Option<Vec<u8>> {
        match read_casc_fdid(&self.storage, font_file.fdid, LocaleFlags::ALL) {
            Ok(data) => Some(data),
            Err(error) => {
                crate::logging::eprintln_elapsed(&format!(
                    "[Font] CASC read failed for {} ({}): {}",
                    font_file.filename, font_file.fdid, error
                ));
                None
            }
        }
    }
}

fn read_casc_fdid(
    storage: &CascStorage,
    fdid: u32,
    locale: LocaleFlags,
) -> Result<Vec<u8>, CascError> {
    let root_entry =
        storage
            .root
            .find_by_fdid(fdid, locale)
            .ok_or_else(|| CascError::KeyNotFound {
                key_type: "FDID".into(),
                hash: format!("{} (locale {})", fdid, locale),
            })?;
    let enc_entry = storage
        .encoding
        .find_ekey(&root_entry.ckey)
        .ok_or_else(|| CascError::KeyNotFound {
            key_type: "CKey".into(),
            hash: hex::encode(root_entry.ckey),
        })?;
    let ekey = &enc_entry.ekeys[0];
    let idx_entry = storage
        .index
        .find(ekey)
        .ok_or_else(|| CascError::KeyNotFound {
            key_type: "EKey".into(),
            hash: hex::encode(ekey),
        })?;
    let blte_data = storage.data.read_entry(
        idx_entry.archive_number,
        idx_entry.archive_offset,
        idx_entry.size,
    )?;
    decode_blte_with_keys(blte_data, Some(&storage.keystore))
}

fn casc_font_listfile_path() -> std::path::PathBuf {
    let path = std::env::temp_dir().join("wow-ui-sim-casc-font-listfile.csv");
    let content = WOW_FONT_FILES
        .iter()
        .map(|font| format!("{};fonts/{}", font.fdid, font.filename.to_lowercase()))
        .collect::<Vec<_>>()
        .join("\n");
    let _ = std::fs::write(&path, content);
    path
}

fn load_wow_font(
    fonts_dir: &Path,
    font_file: &WowFontFile,
    db: &mut fontdb::Database,
    font_map: &mut HashMap<String, FontEntry>,
) {
    let path = fonts_dir.join(font_file.filename);
    let Some(data) = read_font_file(&path) else {
        return;
    };

    let family_name = fontdb_family_name(&data).unwrap_or_else(|| font_file.filename.to_string());
    db.load_font_data(data);
    register_font_aliases(font_file.wow_paths, &family_name, font_map);

    tracing::debug!(
        "Loaded font {} -> family '{}'",
        font_file.filename,
        family_name
    );
}

fn load_system_fallback_if_empty(
    db: &mut fontdb::Database,
    font_map: &mut HashMap<String, FontEntry>,
) {
    if !font_map.is_empty() {
        return;
    }

    db.load_system_fonts();
    let Some(family_name) = db
        .faces()
        .next()
        .and_then(|face| face.families.first().map(|(family, _)| family.to_string()))
    else {
        return;
    };

    for font_file in WOW_FONT_FILES {
        register_font_aliases(font_file.wow_paths, &family_name, font_map);
    }
}

fn load_wow_font_from_casc(
    source: &CascFontSource,
    font_file: &WowFontFile,
    db: &mut fontdb::Database,
    font_map: &mut HashMap<String, FontEntry>,
) {
    let Some(data) = source.read_font(font_file) else {
        tracing::warn!("Font not found in CASC: {}", font_file.filename);
        return;
    };

    let family_name = fontdb_family_name(&data).unwrap_or_else(|| font_file.filename.to_string());
    db.load_font_data(data);
    register_font_aliases(font_file.wow_paths, &family_name, font_map);

    tracing::debug!(
        "Loaded CASC font {} -> family '{}'",
        font_file.filename,
        family_name
    );
}

fn read_font_file(path: &Path) -> Option<Vec<u8>> {
    if !path.exists() {
        tracing::warn!("Font file not found: {}", path.display());
        return None;
    }

    match std::fs::read(path) {
        Ok(data) => Some(data),
        Err(error) => {
            tracing::warn!("Failed to read font {}: {}", path.display(), error);
            None
        }
    }
}

fn register_font_aliases(
    wow_paths: &[&str],
    family_name: &str,
    font_map: &mut HashMap<String, FontEntry>,
) {
    let entry = FontEntry {
        family: family_name.to_string(),
    };

    for wow_path in wow_paths {
        let key = normalize_wow_path(wow_path);
        font_map.insert(key, entry.clone());
    }
}

/// Normalize a WoW font path to uppercase with forward slashes for map lookup.
fn normalize_wow_path(path: &str) -> String {
    path.replace('/', "\\").to_uppercase()
}

/// Extract the font family name from raw TTF data using fontdb.
fn fontdb_family_name(data: &[u8]) -> Option<String> {
    // Parse the font to get its family name
    let mut tmp_db = fontdb::Database::new();
    tmp_db.load_font_data(data.to_vec());
    tmp_db.faces().next().map(|face| face.families[0].0.clone())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    fn fonts_dir() -> PathBuf {
        PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("fonts")
    }

    #[test]
    fn loads_wow_fonts() {
        let fs = WowFontSystem::new(&fonts_dir());
        // FRIZQT__ has two case aliases; the font map stores normalized keys.
        assert_eq!(
            fs.font_map.len(),
            3,
            "font_map: {:?}",
            fs.font_map.keys().collect::<Vec<_>>()
        );
    }

    #[test]
    fn resolves_friz_quadrata() {
        let fs = WowFontSystem::new(&fonts_dir());
        let name = fs.family_name(Some("Fonts\\FRIZQT__.TTF")).unwrap();
        assert!(
            !name.is_empty(),
            "Expected a concrete family name for FRIZQT__, got: {name}"
        );
    }

    #[test]
    fn resolves_case_insensitive() {
        let fs = WowFontSystem::new(&fonts_dir());
        let upper = fs.family_name(Some("Fonts\\FRIZQT__.TTF"));
        let lower = fs.family_name(Some("Fonts\\frizqt__.ttf"));
        let mixed = fs.family_name(Some("fonts\\FrizQT__.TTF"));
        assert_eq!(upper, lower);
        assert_eq!(upper, mixed);
    }

    #[test]
    fn unknown_font_falls_back_to_default() {
        let fs = WowFontSystem::new(&fonts_dir());
        let name = fs.family_name(Some("Fonts\\NONEXISTENT.TTF")).unwrap();
        assert!(
            !name.is_empty(),
            "Expected a fallback family name, got: {name}"
        );
    }

    #[test]
    fn none_font_uses_default() {
        let fs = WowFontSystem::new(&fonts_dir());
        let name = fs.family_name(None).unwrap();
        assert!(
            !name.is_empty(),
            "Expected a default family name, got: {name}"
        );
    }

    #[test]
    fn can_shape_text_with_loaded_font() {
        let mut fs = WowFontSystem::new(&fonts_dir());
        let attrs = fs.attrs_owned(Some("Fonts\\FRIZQT__.TTF"));
        let metrics = cosmic_text::Metrics::new(14.0, 18.0);
        let mut buffer = cosmic_text::Buffer::new(&mut fs.font_system, metrics);
        buffer.set_text(
            &mut fs.font_system,
            "Hello WoW",
            &attrs.as_attrs(),
            cosmic_text::Shaping::Advanced,
            None,
        );
        buffer.shape_until_scroll(&mut fs.font_system, true);

        // Should produce at least one layout run with glyphs
        let runs: Vec<_> = buffer.layout_runs().collect();
        assert!(!runs.is_empty(), "No layout runs produced");
        assert!(!runs[0].glyphs.is_empty(), "No glyphs in first run");
    }

    #[test]
    fn measure_text_width_returns_positive() {
        let mut fs = WowFontSystem::new(&fonts_dir());
        let w = fs.measure_text_width("Hello", Some(WOW_FONT_FRIZ), 14.0);
        assert!(w > 0.0, "Expected positive width, got {w}");
    }

    #[test]
    fn measure_text_width_empty_is_zero() {
        let mut fs = WowFontSystem::new(&fonts_dir());
        let w = fs.measure_text_width("", Some(WOW_FONT_FRIZ), 14.0);
        assert_eq!(w, 0.0);
    }

    #[test]
    fn measure_text_width_scales_with_length() {
        let mut fs = WowFontSystem::new(&fonts_dir());
        let short = fs.measure_text_width("Hi", Some(WOW_FONT_FRIZ), 14.0);
        let long = fs.measure_text_width("Hello World", Some(WOW_FONT_FRIZ), 14.0);
        assert!(
            long > short,
            "Longer text should be wider: {long} > {short}"
        );
    }

    #[test]
    fn measure_text_height_single_line() {
        let mut fs = WowFontSystem::new(&fonts_dir());
        let h = fs.measure_text_height("Hello", Some(WOW_FONT_FRIZ), 14.0, None);
        let line_height = (14.0_f32 * 1.2).ceil();
        assert_eq!(h, line_height, "Single line should equal line_height");
    }

    #[test]
    fn measure_text_height_wraps_with_narrow_width() {
        let mut fs = WowFontSystem::new(&fonts_dir());
        let long_text =
            "This is a fairly long sentence that should wrap when given a narrow width constraint";
        let single = fs.measure_text_height(long_text, Some(WOW_FONT_FRIZ), 14.0, None);
        let wrapped = fs.measure_text_height(long_text, Some(WOW_FONT_FRIZ), 14.0, Some(100.0));
        assert!(
            wrapped > single,
            "Wrapped text should be taller: {wrapped} > {single}"
        );
    }

    #[test]
    fn measure_text_height_empty_is_zero() {
        let mut fs = WowFontSystem::new(&fonts_dir());
        let h = fs.measure_text_height("", Some(WOW_FONT_FRIZ), 14.0, Some(200.0));
        assert_eq!(h, 0.0);
    }
}
