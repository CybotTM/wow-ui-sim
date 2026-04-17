//! Template registry for virtual frames.

use super::types::FrameXml;
use std::collections::{HashMap, HashSet};
use std::sync::{Arc, OnceLock, RwLock};

/// Stores a template (virtual frame) with its widget type.
#[derive(Debug, Clone)]
pub struct TemplateEntry {
    pub name: String,
    pub widget_type: String,
    pub frame: FrameXml,
}

#[derive(Default)]
struct TemplateRegistry {
    entries: HashMap<String, TemplateEntry>,
    entries_ci: HashMap<String, String>,
    chain_cache: HashMap<String, Arc<Vec<TemplateEntry>>>,
}

/// Global registry of XML templates (virtual frames).
fn template_registry() -> &'static RwLock<TemplateRegistry> {
    static REGISTRY: OnceLock<RwLock<TemplateRegistry>> = OnceLock::new();
    REGISTRY.get_or_init(|| RwLock::new(TemplateRegistry::default()))
}

/// Register a template (virtual frame) in the global registry.
pub fn register_template(name: &str, widget_type: &str, frame: FrameXml) {
    let mut registry = template_registry().write().unwrap();
    let lower = name.to_ascii_lowercase();
    registry.entries.insert(
        name.to_string(),
        TemplateEntry {
            name: name.to_string(),
            widget_type: widget_type.to_string(),
            frame,
        },
    );
    registry.entries_ci.insert(lower, name.to_string());
    registry.chain_cache.clear();
}

/// Get a template by name from the registry (case-insensitive).
///
/// WoW's CreateFrame passes type names in various cases (e.g. "DROPDOWNBUTTON"
/// from Lua vs "DropdownButton" from XML). The registry stores the canonical
/// PascalCase name from the XML definition.
pub fn get_template(name: &str) -> Option<TemplateEntry> {
    let registry = template_registry().read().unwrap();
    if let Some(entry) = registry.entries.get(name) {
        return Some(entry.clone());
    }
    let lower = name.to_ascii_lowercase();
    registry
        .entries_ci
        .get(&lower)
        .and_then(|canonical| registry.entries.get(canonical))
        .cloned()
}

/// Template info for C_XMLUtil.GetTemplateInfo.
#[derive(Debug, Clone)]
pub struct TemplateKeyValueInfo {
    pub key: String,
    pub value: String,
    pub value_type: Option<String>,
}

#[derive(Debug, Clone)]
pub struct TemplateInfo {
    pub frame_type: String,
    pub template_name: String,
    pub width: f32,
    pub height: f32,
    pub key_values: Vec<TemplateKeyValueInfo>,
}

/// Get template info (type, width, height) by resolving inheritance chain.
pub fn get_template_info(name: &str) -> Option<TemplateInfo> {
    let chain = get_template_chain(name);
    if chain.is_empty() {
        return None;
    }
    let frame_type = resolve_frame_type(&chain);
    let (width, height) = resolve_chain_size(&chain);
    let template_name = chain
        .last()
        .map(|entry| entry.name.clone())
        .unwrap_or_else(|| name.to_string());
    let key_values = collect_key_values(&chain);
    Some(TemplateInfo {
        frame_type,
        template_name,
        width,
        height,
        key_values,
    })
}

/// Resolve the frame type from an inheritance chain.
///
/// The most derived entry's widget_type wins. The chain is base-to-derived;
/// e.g. `<Button inherits="FrameTemplate">` is a Button, not a Frame.
/// However, many templates inherit from Frame-based parents without explicitly
/// redefining their type. The last entry in the chain is the template itself
/// (most derived) — use its type if non-empty, otherwise fall back to parents.
fn resolve_frame_type(chain: &[TemplateEntry]) -> String {
    chain
        .last()
        .filter(|e| !e.widget_type.is_empty())
        .map(|e| e.widget_type.clone())
        .or_else(|| {
            chain
                .iter()
                .find(|e| !e.widget_type.is_empty())
                .map(|e| e.widget_type.clone())
        })
        .unwrap_or_else(|| "Frame".to_string())
}

/// Collect key-value pairs from all entries in the inheritance chain.
fn collect_key_values(chain: &[TemplateEntry]) -> Vec<TemplateKeyValueInfo> {
    chain
        .iter()
        .flat_map(|entry| entry.frame.all_key_values())
        .flat_map(|key_values| key_values.values.iter())
        .map(|key_value| TemplateKeyValueInfo {
            key: key_value.key.clone(),
            value: key_value.value.clone(),
            value_type: key_value.value_type.clone(),
        })
        .collect()
}

/// Resolve (width, height) across the inheritance chain.
/// Most derived entry wins. Within an entry, direct `x`/`y` attrs override `AbsDimension`.
fn resolve_chain_size(chain: &[TemplateEntry]) -> (f32, f32) {
    let mut width: f32 = 0.0;
    let mut height: f32 = 0.0;
    for entry in chain {
        let Some(size) = entry.frame.size() else {
            continue;
        };
        if let Some(ref abs) = size.abs_dimension {
            if let Some(x) = abs.x {
                width = x;
            }
            if let Some(y) = abs.y {
                height = y;
            }
        }
        if let Some(x) = size.x {
            width = x;
        }
        if let Some(y) = size.y {
            height = y;
        }
    }
    (width, height)
}

/// Get the full inheritance chain for a template (including the template itself).
/// Returns templates in order from most base to most derived.
/// Returns Arc to avoid cloning the chain on every access.
pub fn get_template_chain(names: &str) -> Arc<Vec<TemplateEntry>> {
    let key = names.trim().to_string();
    if key.is_empty() {
        return Arc::new(Vec::new());
    }

    if let Some(cached) = template_registry()
        .read()
        .unwrap()
        .chain_cache
        .get(&key)
        .cloned()
    {
        return cached;
    }

    let mut chain = Vec::new();
    let mut visited = HashSet::new();

    // Process comma-separated template names
    for name in key.split(',').map(|s| s.trim()) {
        if name.is_empty() || visited.contains(name) {
            continue;
        }
        collect_template_chain(name, &mut chain, &mut visited);
    }

    let arc_chain = Arc::new(chain);
    template_registry()
        .write()
        .unwrap()
        .chain_cache
        .insert(key, Arc::clone(&arc_chain));
    arc_chain
}

/// Recursively collect templates in the inheritance chain.
fn collect_template_chain(
    name: &str,
    chain: &mut Vec<TemplateEntry>,
    visited: &mut HashSet<String>,
) {
    if visited.contains(name) {
        return;
    }
    visited.insert(name.to_string());

    if let Some(entry) = get_template(name) {
        // First, process parent templates (if this template inherits from others)
        if let Some(ref inherits) = entry.frame.inherits {
            for parent in inherits.split(',').map(|s| s.trim()) {
                if !parent.is_empty() {
                    collect_template_chain(parent, chain, visited);
                }
            }
        }
        // Then add this template
        chain.push(entry);
    }
}

/// Register synthetic templates for C++ intrinsic frame types.
///
/// WoW has several frame types built into the C++ engine that don't have XML
/// definitions in the extracted Interface files. They behave as templates that
/// inherit from a base template and apply a mixin. Register them so the
/// template chain resolution and CreateFrame can find them.
pub fn register_intrinsic_templates() {
    let intrinsics: &[(&str, &str, &str, &str)] = &[
        // (name, widget_type, inherits, mixin)
        (
            "WoWScrollBoxList",
            "Frame",
            "ScrollBoxBaseTemplate",
            "ScrollBoxListMixin",
        ),
        (
            "WoWScrollBox",
            "Frame",
            "ScrollBoxBaseTemplate",
            "ScrollBoxBaseMixin",
        ),
        (
            "WoWTrimScrollBar",
            "EventFrame",
            "WowTrimScrollBarTemplate",
            "",
        ),
    ];

    for &(name, wtype, inherits, mixin) in intrinsics {
        let frame = FrameXml {
            inherits: Some(inherits.to_string()),
            mixin: if mixin.is_empty() {
                None
            } else {
                Some(mixin.to_string())
            },
            is_virtual: Some(true),
            ..Default::default()
        };
        register_template(name, wtype, frame);
    }
}

/// Clear the template registry (useful for testing).
pub fn clear_templates() {
    let mut registry = template_registry().write().unwrap();
    registry.entries.clear();
    registry.entries_ci.clear();
    registry.chain_cache.clear();

    texture_template_registry().write().unwrap().clear();
    anim_group_template_registry().write().unwrap().clear();
}

// ---------------------------------------------------------------------------
// Texture template registry (virtual textures with mixin/inherits)
// ---------------------------------------------------------------------------

use super::types_elements::{AnimationGroupXml, TextureXml};

/// Global registry of virtual texture templates.
fn texture_template_registry() -> &'static RwLock<HashMap<String, TextureXml>> {
    static REGISTRY: OnceLock<RwLock<HashMap<String, TextureXml>>> = OnceLock::new();
    REGISTRY.get_or_init(|| RwLock::new(HashMap::new()))
}

/// Register a virtual texture template.
pub fn register_texture_template(name: &str, texture: TextureXml) {
    let mut registry = texture_template_registry().write().unwrap();
    registry.insert(name.to_string(), texture);
}

/// Get size from a texture template by name. Returns (width, height) if found.
pub fn get_texture_template_size(name: &str) -> Option<(f32, f32)> {
    let registry = texture_template_registry().read().unwrap();
    let tex = registry.get(name)?;
    let size = tex.size.as_ref()?;
    let w = size.x.unwrap_or(0.0);
    let h = size.y.unwrap_or(0.0);
    if w > 0.0 && h > 0.0 {
        Some((w, h))
    } else {
        None
    }
}

/// Resolve texture inheritance: merge properties from the template chain.
///
/// Returns a new `TextureXml` with inherited properties filled in.
/// Instance properties override template properties (most-derived wins).
pub fn resolve_texture_inheritance(texture: &TextureXml) -> TextureXml {
    let Some(ref inherits) = texture.inherits else {
        return texture.clone();
    };

    let registry = texture_template_registry().read().unwrap();
    // Collect templates in order (base first)
    let mut templates = Vec::new();
    for parent_name in inherits.split(',').map(|s| s.trim()) {
        if let Some(parent) = registry.get(parent_name) {
            templates.push(parent.clone());
        }
    }
    drop(registry);

    if templates.is_empty() {
        return texture.clone();
    }

    // Start with first template as base, overlay subsequent templates, then instance
    let mut merged = templates[0].clone();
    for tmpl in &templates[1..] {
        merge_texture_fields(&mut merged, tmpl);
    }
    merge_texture_fields(&mut merged, texture);

    // Preserve instance identity fields
    merged.name = texture.name.clone();
    merged.parent_key = texture.parent_key.clone();
    merged.parent_array = texture.parent_array.clone();
    merged.is_virtual = texture.is_virtual;
    merged.inherits = texture.inherits.clone();
    merged.anchors = texture.anchors.clone();
    merged.animations = texture.animations.clone();
    merged.scripts = texture.scripts.clone();
    merged.masked_textures = texture.masked_textures.clone();

    merged
}

/// Overlay `src` fields onto `dst` where `src` has a value.
fn merge_texture_fields(dst: &mut TextureXml, src: &TextureXml) {
    macro_rules! merge_opt {
        ($field:ident) => {
            if src.$field.is_some() {
                dst.$field = src.$field.clone();
            }
        };
    }
    merge_opt!(file);
    merge_opt!(atlas);
    merge_opt!(use_atlas_size);
    merge_opt!(tex_coords);
    merge_opt!(size);
    merge_opt!(color);
    merge_opt!(horiz_tile);
    merge_opt!(vert_tile);
    merge_opt!(thickness);
    merge_opt!(hidden);
    merge_opt!(alpha);
    merge_opt!(set_all_points);
    merge_opt!(mixin);
    merge_texture_blend_mode(dst, src);
}

fn merge_texture_blend_mode(dst: &mut TextureXml, src: &TextureXml) {
    let Some(mode) = src.effective_blend_mode() else {
        return;
    };
    let mode = mode.to_string();
    dst.alpha_mode = Some(mode.clone());
    dst.blend_mode = Some(mode);
}

/// Collect all mixins for a texture by resolving its `inherits` chain.
pub fn collect_texture_mixins(texture: &TextureXml) -> Vec<String> {
    let registry = texture_template_registry().read().unwrap();
    collect_inherited_mixins(
        texture.inherits.as_deref(),
        texture.mixin.as_deref(),
        |name| registry.get(name).and_then(|p| p.mixin.clone()),
    )
}

// ---------------------------------------------------------------------------
// AnimationGroup template registry (virtual animation groups with mixin)
// ---------------------------------------------------------------------------

/// Global registry of virtual AnimationGroup templates.
fn anim_group_template_registry() -> &'static RwLock<HashMap<String, AnimationGroupXml>> {
    static REGISTRY: OnceLock<RwLock<HashMap<String, AnimationGroupXml>>> = OnceLock::new();
    REGISTRY.get_or_init(|| RwLock::new(HashMap::new()))
}

/// Register a virtual AnimationGroup template.
pub fn register_anim_group_template(name: &str, anim_group: AnimationGroupXml) {
    let mut registry = anim_group_template_registry().write().unwrap();
    registry.insert(name.to_string(), anim_group);
}

/// Read-lock the AnimationGroup template registry for lookups.
pub fn anim_group_template_registry_read()
-> std::sync::RwLockReadGuard<'static, HashMap<String, AnimationGroupXml>> {
    anim_group_template_registry().read().unwrap()
}

/// Collect all mixins for an AnimationGroup by resolving its `inherits` chain.
pub fn collect_anim_group_mixins(anim_group: &AnimationGroupXml) -> Vec<String> {
    let registry = anim_group_template_registry().read().unwrap();
    collect_inherited_mixins(
        anim_group.inherits.as_deref(),
        anim_group.mixin.as_deref(),
        |name| registry.get(name).and_then(|p| p.mixin.clone()),
    )
}

/// Append unique mixin names from a comma-separated string.
/// Resolve mixins from an inherits chain + own mixin attribute.
/// `lookup_parent_mixin` maps a parent template name to its mixin string (owned).
fn collect_inherited_mixins(
    inherits: Option<&str>,
    own_mixin: Option<&str>,
    lookup_parent_mixin: impl Fn(&str) -> Option<String>,
) -> Vec<String> {
    let mut mixins = Vec::new();
    if let Some(inherits) = inherits {
        for parent_name in inherits.split(',').map(|s| s.trim()) {
            if let Some(parent_mixin) = lookup_parent_mixin(parent_name) {
                append_unique_mixins(&mut mixins, Some(&parent_mixin));
            }
        }
    }
    append_unique_mixins(&mut mixins, own_mixin);
    mixins
}

fn append_unique_mixins(mixins: &mut Vec<String>, attr: Option<&str>) {
    let Some(attr) = attr else { return };
    for name in attr.split(',').map(|s| s.trim()) {
        if !name.is_empty() && !mixins.contains(&name.to_string()) {
            mixins.push(name.to_string());
        }
    }
}
