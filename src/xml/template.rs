//! Template registry for virtual frames.

use super::types::FrameXml;
use std::collections::{HashMap, HashSet};
use std::sync::{OnceLock, RwLock};

/// Stores a template (virtual frame) with its widget type.
#[derive(Debug, Clone)]
pub struct TemplateEntry {
    pub name: String,
    pub widget_type: String,
    pub frame: FrameXml,
}

/// Global registry of XML templates (virtual frames).
fn template_registry() -> &'static RwLock<HashMap<String, TemplateEntry>> {
    static REGISTRY: OnceLock<RwLock<HashMap<String, TemplateEntry>>> = OnceLock::new();
    REGISTRY.get_or_init(|| RwLock::new(HashMap::new()))
}

/// Register a template (virtual frame) in the global registry.
pub fn register_template(name: &str, widget_type: &str, frame: FrameXml) {
    let mut registry = template_registry().write().unwrap();
    registry.insert(
        name.to_string(),
        TemplateEntry {
            name: name.to_string(),
            widget_type: widget_type.to_string(),
            frame,
        },
    );
}

/// Get a template by name from the registry (case-insensitive).
///
/// WoW's CreateFrame passes type names in various cases (e.g. "DROPDOWNBUTTON"
/// from Lua vs "DropdownButton" from XML). The registry stores the canonical
/// PascalCase name from the XML definition.
pub fn get_template(name: &str) -> Option<TemplateEntry> {
    let registry = template_registry().read().unwrap();
    if let Some(entry) = registry.get(name) {
        return Some(entry.clone());
    }
    // Case-insensitive fallback
    let lower = name.to_ascii_lowercase();
    registry
        .values()
        .find(|e| e.name.to_ascii_lowercase() == lower)
        .cloned()
}

/// Template info for C_XMLUtil.GetTemplateInfo.
pub struct TemplateInfo {
    pub frame_type: String,
    pub width: f32,
    pub height: f32,
}

/// Get template info (type, width, height) by resolving inheritance chain.
pub fn get_template_info(name: &str) -> Option<TemplateInfo> {
    let chain = get_template_chain(name);
    if chain.is_empty() {
        return None;
    }

    // Get the widget type from the first entry that defines it
    let frame_type = chain
        .iter()
        .find(|e| !e.widget_type.is_empty())
        .map(|e| e.widget_type.clone())
        .unwrap_or_else(|| "Frame".to_string());

    // Resolve size by looking through inheritance chain (most derived wins)
    let mut width: f32 = 0.0;
    let mut height: f32 = 0.0;

    for entry in &chain {
        if let Some(size) = entry.frame.size() {
            // Check AbsDimension first, then direct attributes
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
    }

    Some(TemplateInfo {
        frame_type,
        width,
        height,
    })
}

/// Get the full inheritance chain for a template (including the template itself).
/// Returns templates in order from most base to most derived.
pub fn get_template_chain(names: &str) -> Vec<TemplateEntry> {
    let mut chain = Vec::new();
    let mut visited = HashSet::new();

    // Process comma-separated template names
    for name in names.split(',').map(|s| s.trim()) {
        if name.is_empty() || visited.contains(name) {
            continue;
        }
        collect_template_chain(name, &mut chain, &mut visited);
    }

    chain
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
#[allow(dead_code)]
pub fn clear_templates() {
    let mut registry = template_registry().write().unwrap();
    registry.clear();
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
    merge_opt!(alpha_mode);
    merge_opt!(set_all_points);
    merge_opt!(mixin);
}

/// Collect all mixins for a texture by resolving its `inherits` chain.
pub fn collect_texture_mixins(texture: &TextureXml) -> Vec<String> {
    let mut mixins = Vec::new();

    // Collect mixins from inherited templates
    if let Some(ref inherits) = texture.inherits {
        let registry = texture_template_registry().read().unwrap();
        for parent_name in inherits.split(',').map(|s| s.trim()) {
            if let Some(parent) = registry.get(parent_name)
                && let Some(ref m) = parent.mixin
            {
                for mixin in m.split(',').map(|s| s.trim()) {
                    if !mixin.is_empty() && !mixins.contains(&mixin.to_string()) {
                        mixins.push(mixin.to_string());
                    }
                }
            }
        }
    }

    // Collect direct mixins on the texture itself
    if let Some(ref m) = texture.mixin {
        for mixin in m.split(',').map(|s| s.trim()) {
            if !mixin.is_empty() && !mixins.contains(&mixin.to_string()) {
                mixins.push(mixin.to_string());
            }
        }
    }

    mixins
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
    let mut mixins = Vec::new();

    // Collect mixins from inherited templates
    if let Some(ref inherits) = anim_group.inherits {
        let registry = anim_group_template_registry().read().unwrap();
        for parent_name in inherits.split(',').map(|s| s.trim()) {
            if let Some(parent) = registry.get(parent_name)
                && let Some(ref m) = parent.mixin
            {
                for mixin in m.split(',').map(|s| s.trim()) {
                    if !mixin.is_empty() && !mixins.contains(&mixin.to_string()) {
                        mixins.push(mixin.to_string());
                    }
                }
            }
        }
    }

    // Collect direct mixins on the animation group itself
    if let Some(ref m) = anim_group.mixin {
        for mixin in m.split(',').map(|s| s.trim()) {
            if !mixin.is_empty() && !mixins.contains(&mixin.to_string()) {
                mixins.push(mixin.to_string());
            }
        }
    }

    mixins
}
