//! Font management using cosmic-text.
//!
//! Loads WoW TTF fonts into a cosmic-text FontSystem for text shaping,
//! measurement, and glyph rasterization. Provides mapping from WoW font
//! paths (e.g. `Fonts\\FRIZQT__.TTF`) to fontdb family names.

use std::collections::HashMap;

use cosmic_text::fontdb;

/// WoW font path constants (as they appear in Lua/XML).
const WOW_FONT_FRIZ: &str = "Fonts\\FRIZQT__.TTF";
const WOW_FONT_ARIAL_NARROW: &str = "Fonts\\ARIALN.TTF";
const LINE_HEIGHT_MULTIPLIER: f32 = 1.2;

/// Default WoW font (Friz Quadrata).
pub const DEFAULT_WOW_FONT: &str = WOW_FONT_FRIZ;

pub(crate) fn line_height_for_font_size(font_size: f32) -> Option<f32> {
    if !font_size.is_finite() || font_size <= 0.0 {
        return None;
    }
    Some((font_size * LINE_HEIGHT_MULTIPLIER).ceil().max(1.0))
}

/// Font entry mapping a WoW path to a fontdb family name.
#[derive(Debug, Clone)]
struct FontEntry {
    family: String,
}

struct WowFontFile {
    filename: &'static str,
    wow_paths: &'static [&'static str],
}

const WOW_FONT_FILES: &[WowFontFile] = &[
    WowFontFile {
        filename: "FRIZQT__.TTF",
        wow_paths: &[WOW_FONT_FRIZ, "Fonts\\frizqt__.ttf"],
    },
    WowFontFile {
        filename: "ARIALN.TTF",
        wow_paths: &[WOW_FONT_ARIAL_NARROW, "Fonts\\arialn.ttf"],
    },
    WowFontFile {
        filename: "frizqt___cyr.ttf",
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

#[cfg(feature = "casc")]
use std::sync::OnceLock;

#[cfg(feature = "casc")]
static FONT_CASC_INITIALIZED: OnceLock<bool> = OnceLock::new();

#[cfg(feature = "casc")]
fn casc_enabled() -> bool {
    *FONT_CASC_INITIALIZED.get_or_init(|| {
        // Opt-out: WOW_SIM_CASC=0 disables. Anything else (or unset) enables.
        if std::env::var("WOW_SIM_CASC").ok().as_deref() == Some("0") {
            return false;
        }
        // Require a discoverable WoW install, otherwise no point trying.
        asset_resolver::wow_install_path().is_some()
    })
}

#[cfg(feature = "casc")]
fn try_casc_font_bytes(filename: &str) -> Option<Vec<u8>> {
    if !casc_enabled() {
        return None;
    }
    let path = format!("fonts/{}", filename.to_lowercase());
    let resolver = crate::asset_resolver_config::resolver();
    let fdid =
        crate::limited_listfile::lookup_path(&path).or_else(|| resolver.lookup_path(&path))?;
    resolver.resolve_bytes(fdid)
}

#[cfg(not(feature = "casc"))]
fn try_casc_font_bytes(_filename: &str) -> Option<Vec<u8>> {
    None
}

impl WowFontSystem {
    /// Create a new font system with WoW fonts loaded from CASC (or system fallback).
    pub fn new() -> Self {
        let mut db = fontdb::Database::new();
        let mut font_map = HashMap::new();

        for font_file in WOW_FONT_FILES {
            load_wow_font(font_file, &mut db, &mut font_map);
        }
        if font_map.is_empty() {
            load_system_font_fallback(&mut db, &mut font_map);
        }

        if font_map.is_empty() {
            db.load_system_fonts();
        }

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
        let Some(line_height) = line_height_for_font_size(font_size) else {
            return 0.0;
        };
        if text.is_empty() {
            return 0.0;
        }
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
        let Some(line_height) = line_height_for_font_size(font_size) else {
            return 0.0;
        };
        if text.is_empty() {
            return 0.0;
        }
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

impl Default for WowFontSystem {
    fn default() -> Self {
        Self::new()
    }
}

fn load_wow_font(
    font_file: &WowFontFile,
    db: &mut fontdb::Database,
    font_map: &mut HashMap<String, FontEntry>,
) {
    let Some(data) = try_casc_font_bytes(font_file.filename) else {
        tracing::warn!("Font {} not found in CASC, skipping", font_file.filename);
        return;
    };

    let family_name = fontdb_family_name(&data).unwrap_or_else(|| font_file.filename.to_string());
    db.load_font_data(data);
    register_font_aliases(font_file.wow_paths, &family_name, font_map);

    tracing::debug!(
        "Registered font {} from CASC -> family '{}'",
        font_file.filename,
        family_name
    );
}

fn load_system_font_fallback(db: &mut fontdb::Database, font_map: &mut HashMap<String, FontEntry>) {
    db.load_system_fonts();
    let Some(family_name) = first_system_family_name(db) else {
        tracing::warn!("No CASC or system fonts available for text shaping");
        return;
    };

    for font_file in WOW_FONT_FILES {
        register_font_aliases(font_file.wow_paths, &family_name, font_map);
    }

    tracing::warn!("Using system font '{family_name}' as WoW font fallback");
}

fn first_system_family_name(db: &fontdb::Database) -> Option<String> {
    db.faces()
        .next()
        .and_then(|face| face.families.first())
        .map(|family| family.0.clone())
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

    #[test]
    fn loads_wow_fonts() {
        let fs = WowFontSystem::new();
        // 3 font files: FRIZQT__ (2 aliases), ARIALN (2 aliases), frizqt___cyr (1 alias).
        // All come from CASC; without CASC the font_map is empty and downstream
        // shaping uses cosmic-text's default fonts.
        if asset_resolver_available() {
            assert!(
                !fs.font_map.is_empty(),
                "font_map should have CASC fonts: {:?}",
                fs.font_map.keys().collect::<Vec<_>>()
            );
        }
    }

    fn asset_resolver_available() -> bool {
        #[cfg(feature = "casc")]
        {
            asset_resolver::wow_install_path().is_some()
        }
        #[cfg(not(feature = "casc"))]
        {
            false
        }
    }

    #[test]
    fn resolves_friz_quadrata() {
        let fs = WowFontSystem::new();
        let name = fs.family_name(Some("Fonts\\FRIZQT__.TTF")).unwrap();
        assert!(!name.is_empty(), "Expected a font family fallback");
    }

    #[test]
    fn resolves_case_insensitive() {
        let fs = WowFontSystem::new();
        let upper = fs.family_name(Some("Fonts\\FRIZQT__.TTF"));
        let lower = fs.family_name(Some("Fonts\\frizqt__.ttf"));
        let mixed = fs.family_name(Some("fonts\\FrizQT__.TTF"));
        assert_eq!(upper, lower);
        assert_eq!(upper, mixed);
    }

    #[test]
    fn unknown_font_falls_back_to_default() {
        let fs = WowFontSystem::new();
        let name = fs.family_name(Some("Fonts\\NONEXISTENT.TTF")).unwrap();
        let default_name = fs.family_name(Some(DEFAULT_WOW_FONT)).unwrap();
        assert_eq!(name, default_name);
    }

    #[test]
    fn none_font_uses_default() {
        let fs = WowFontSystem::new();
        let name = fs.family_name(None).unwrap();
        let default_name = fs.family_name(Some(DEFAULT_WOW_FONT)).unwrap();
        assert_eq!(name, default_name);
    }

    #[test]
    fn can_shape_text_with_loaded_font() {
        let mut fs = WowFontSystem::new();
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
        let mut fs = WowFontSystem::new();
        let w = fs.measure_text_width("Hello", Some(WOW_FONT_FRIZ), 14.0);
        assert!(w > 0.0, "Expected positive width, got {w}");
    }

    #[test]
    fn measure_text_width_empty_is_zero() {
        let mut fs = WowFontSystem::new();
        let w = fs.measure_text_width("", Some(WOW_FONT_FRIZ), 14.0);
        assert_eq!(w, 0.0);
    }

    #[test]
    fn zero_font_size_measures_as_empty_text() {
        let mut fs = WowFontSystem::new();

        assert_eq!(fs.measure_text_width("Collections", None, 0.0), 0.0);
        assert_eq!(
            fs.measure_text_height("Collections", None, 0.0, Some(100.0)),
            0.0
        );
    }

    #[test]
    fn measure_text_width_scales_with_length() {
        let mut fs = WowFontSystem::new();
        let short = fs.measure_text_width("Hi", Some(WOW_FONT_FRIZ), 14.0);
        let long = fs.measure_text_width("Hello World", Some(WOW_FONT_FRIZ), 14.0);
        assert!(
            long > short,
            "Longer text should be wider: {long} > {short}"
        );
    }

    #[test]
    fn measure_text_height_single_line() {
        let mut fs = WowFontSystem::new();
        let h = fs.measure_text_height("Hello", Some(WOW_FONT_FRIZ), 14.0, None);
        let line_height = (14.0_f32 * 1.2).ceil();
        assert_eq!(h, line_height, "Single line should equal line_height");
    }

    #[test]
    fn measure_text_height_wraps_with_narrow_width() {
        let mut fs = WowFontSystem::new();
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
        let mut fs = WowFontSystem::new();
        let h = fs.measure_text_height("", Some(WOW_FONT_FRIZ), 14.0, Some(200.0));
        assert_eq!(h, 0.0);
    }
}
