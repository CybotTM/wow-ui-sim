//! Batched creation of layer children (textures and fontstrings).
//!
//! Instead of one `env.exec()` per texture/fontstring, collects all Lua code
//! into a single chunk with `do...end` scoping, executes once, then applies
//! post-exec side effects (texture animations, fontstring text sync).

use crate::lua_api::LoaderEnv;
use crate::xml;

use super::LoadTiming;
use super::error::LoadError;
use super::helpers::resolve_child_name;
use super::xml_fontstring::{
    build_fontstring_lua, resolve_fontstring_text, sync_fontstring_text_to_rust,
};
use super::xml_texture::{apply_texture_animations_xml, build_texture_lua};

struct CollectedTexture<'a> {
    texture: &'a xml::TextureXml,
    is_mask: bool,
    is_line: bool,
    draw_layer: String,
    sub_level: i32,
}

struct AnimEntry<'a> {
    texture: &'a xml::TextureXml,
    tex_name: String,
}

struct TextSync {
    name: String,
    text: String,
}

struct ParentKeyAttachment {
    child_name: String,
    parent_key: Option<String>,
    parent_array: Option<String>,
}

fn exec_batch(env: &LoaderEnv<'_>, batch: &str, parent_name: &str) -> Result<(), LoadError> {
    if batch.is_empty() {
        return Ok(());
    }
    env.exec(batch)
        .map_err(|e| LoadError::Lua(format!("layer children on {}: {}", parent_name, e)))
}

fn collected_texture_for<'a>(
    element: &'a xml::LayerElement,
    draw_layer: &str,
    sub_level: i32,
) -> Option<CollectedTexture<'a>> {
    let (texture, is_mask, is_line) = match element {
        xml::LayerElement::Texture(t) => (t, false, false),
        xml::LayerElement::Line(t) => (t, false, true),
        xml::LayerElement::MaskTexture(t) => (t, true, false),
        xml::LayerElement::FontString(_) => return None,
    };
    Some(CollectedTexture {
        texture,
        is_mask,
        is_line,
        draw_layer: draw_layer.to_string(),
        sub_level,
    })
}

fn push_parent_key_attachment(
    attachments: &mut Vec<ParentKeyAttachment>,
    child_name: String,
    parent_key: Option<String>,
    parent_array: Option<String>,
) {
    if parent_key.is_some() || parent_array.is_some() {
        attachments.push(ParentKeyAttachment {
            child_name,
            parent_key,
            parent_array,
        });
    }
}

fn append_single_texture<'a>(
    ct: &CollectedTexture<'a>,
    parent_name: &str,
    name_parent: &str,
    batch: &mut String,
    attachments: &mut Vec<ParentKeyAttachment>,
    anim_entries: &mut Vec<AnimEntry<'a>>,
    timing: &mut LoadTiming,
) {
    let resolved = xml::resolve_texture_inheritance(ct.texture);
    let tex_name = resolve_child_name(resolved.name.as_deref(), name_parent, "__tex_");
    let code = build_texture_lua(
        &tex_name,
        &resolved,
        parent_name,
        &ct.draw_layer,
        ct.is_mask,
        ct.is_line,
        ct.sub_level,
    );
    batch.push_str("do ");
    batch.push_str(&code);
    batch.push_str(" end\n");
    push_parent_key_attachment(
        attachments,
        tex_name.clone(),
        resolved.parent_key.clone(),
        resolved.parent_array.clone(),
    );
    if ct.texture.animations.is_some() {
        anim_entries.push(AnimEntry {
            texture: ct.texture,
            tex_name,
        });
    }
    timing.texture_count += 1;
}

/// Append code for all layer children (textures + fontstrings) in XML document order.
///
/// Order matters: an element's anchor `relativeKey="$parent.SiblingKey"` resolves at
/// SetPoint time by reading the sibling from the parent table, so the sibling must
/// be created before any element that anchors to it. Iterating `layer.elements`
/// directly preserves the XML order Blizzard authors rely on.
fn append_layer_children_code<'a>(
    frame: &'a xml::FrameXml,
    parent_name: &str,
    name_parent: &str,
    batch: &mut String,
    attachments: &mut Vec<ParentKeyAttachment>,
    anim_entries: &mut Vec<AnimEntry<'a>>,
    text_syncs: &mut Vec<TextSync>,
    timing: &mut LoadTiming,
) {
    for layers in frame.layers() {
        for layer in &layers.layers {
            let draw_layer = layer.level.as_deref().unwrap_or("ARTWORK");
            let sub_level = layer.texture_sub_level.unwrap_or(0);
            for element in &layer.elements {
                append_layer_element_code(
                    element,
                    parent_name,
                    name_parent,
                    draw_layer,
                    sub_level,
                    batch,
                    attachments,
                    anim_entries,
                    text_syncs,
                    timing,
                );
            }
        }
    }
}

fn append_layer_element_code<'a>(
    element: &'a xml::LayerElement,
    parent_name: &str,
    name_parent: &str,
    draw_layer: &str,
    sub_level: i32,
    batch: &mut String,
    attachments: &mut Vec<ParentKeyAttachment>,
    anim_entries: &mut Vec<AnimEntry<'a>>,
    text_syncs: &mut Vec<TextSync>,
    timing: &mut LoadTiming,
) {
    if let xml::LayerElement::FontString(fs) = element {
        append_single_fontstring(
            fs,
            parent_name,
            name_parent,
            draw_layer,
            sub_level,
            batch,
            attachments,
            text_syncs,
            timing,
        );
        return;
    }
    let Some(ct) = collected_texture_for(element, draw_layer, sub_level) else {
        return;
    };
    if ct.texture.is_virtual == Some(true) {
        if let Some(ref name) = ct.texture.name {
            xml::register_texture_template(name, ct.texture.clone());
        }
        return;
    }
    append_single_texture(
        &ct,
        parent_name,
        name_parent,
        batch,
        attachments,
        anim_entries,
        timing,
    );
}

fn append_single_fontstring(
    fontstring: &xml::FontStringXml,
    parent_name: &str,
    name_parent: &str,
    draw_layer: &str,
    sub_level: i32,
    batch: &mut String,
    attachments: &mut Vec<ParentKeyAttachment>,
    text_syncs: &mut Vec<TextSync>,
    timing: &mut LoadTiming,
) {
    if fontstring.is_virtual == Some(true) {
        return;
    }
    let fs_name = resolve_child_name(fontstring.name.as_deref(), name_parent, "__fs_");
    let resolved_text = resolve_fontstring_text(fontstring.text.as_deref());
    let code = build_fontstring_lua(
        fontstring,
        parent_name,
        draw_layer,
        sub_level,
        &fs_name,
        &resolved_text,
    );
    batch.push_str("do ");
    batch.push_str(&code);
    batch.push_str(" end\n");
    push_parent_key_attachment(
        attachments,
        fs_name.clone(),
        fontstring.parent_key.clone(),
        fontstring.parent_array.clone(),
    );
    if let Some(text) = resolved_text {
        text_syncs.push(TextSync {
            name: fs_name,
            text,
        });
    }
    timing.fontstring_count += 1;
}

fn apply_texture_anims(env: &LoaderEnv<'_>, entries: &[AnimEntry<'_>]) {
    for entry in entries {
        apply_texture_animations_xml(env, entry.texture, &entry.tex_name);
    }
}

fn apply_parent_key_attachments(
    env: &LoaderEnv<'_>,
    parent_name: &str,
    target_parent_name: &str,
    attachments: &[ParentKeyAttachment],
) -> Result<(), LoadError> {
    if attachments.is_empty() {
        return Ok(());
    }

    env.with_state(|state| {
        for attachment in attachments {
            apply_parent_key_attachment(state, parent_name, target_parent_name, attachment)?;
        }
        Ok(())
    })
}

fn apply_parent_key_attachment(
    state: &mut rilua::vm::state::LuaState,
    parent_name: &str,
    target_parent_name: &str,
    attachment: &ParentKeyAttachment,
) -> Result<(), LoadError> {
    let (parent_id, child_id) =
        find_attachment_ids(state, parent_name, target_parent_name, attachment)?;
    let (Some(parent_id), Some(child_id)) = (parent_id, child_id) else {
        return Ok(());
    };
    apply_attachment_links(state, parent_id, child_id, attachment)
}

fn apply_attachment_links(
    state: &mut rilua::vm::state::LuaState,
    parent_id: u64,
    child_id: u64,
    attachment: &ParentKeyAttachment,
) -> Result<(), LoadError> {
    if let Some(parent_key) = attachment.parent_key.as_deref() {
        attach_parent_key(state, parent_id, parent_key, child_id)?;
    }
    if let Some(parent_array) = attachment.parent_array.as_deref() {
        append_parent_array_entry(state, parent_id, parent_array, child_id)?;
    }
    Ok(())
}

fn find_attachment_ids(
    state: &mut rilua::vm::state::LuaState,
    parent_name: &str,
    target_parent_name: &str,
    attachment: &ParentKeyAttachment,
) -> Result<(Option<u64>, Option<u64>), LoadError> {
    let sim = crate::lua_api::methods::borrow_state(state)
        .map_err(|error| LoadError::Lua(error.to_string()))?;
    let child_id = sim.widgets.get_id_by_name(&attachment.child_name);
    let named_parent_id = sim
        .widgets
        .get_id_by_name(target_parent_name)
        .or_else(|| sim.widgets.get_id_by_name(parent_name));
    let fallback_parent_id =
        child_id.and_then(|child_id| sim.widgets.get(child_id).and_then(|child| child.parent_id));

    Ok((named_parent_id.or(fallback_parent_id), child_id))
}

fn attach_parent_key(
    state: &mut rilua::vm::state::LuaState,
    parent_id: u64,
    parent_key: &str,
    child_id: u64,
) -> Result<(), LoadError> {
    if child_is_direct(state, parent_id, child_id)? {
        crate::lua_api::globals::template::assign_parent_key(
            state, parent_id, parent_key, child_id,
        )
        .map_err(|error| LoadError::Lua(error.to_string()))?;
    } else {
        crate::lua_api::methods::sync_child_to_rilua(state, parent_id, parent_key, child_id)
            .map_err(|error| LoadError::Lua(error.to_string()))?;
    }

    if let Some(target_parent_id) = transparent_wrapper_parent_id(state, parent_id)? {
        crate::lua_api::methods::sync_child_to_rilua(state, target_parent_id, parent_key, child_id)
            .map_err(|error| LoadError::Lua(error.to_string()))?;
    }

    Ok(())
}

fn child_is_direct(
    state: &mut rilua::vm::state::LuaState,
    parent_id: u64,
    child_id: u64,
) -> Result<bool, LoadError> {
    let sim = crate::lua_api::methods::borrow_state(state)
        .map_err(|error| LoadError::Lua(error.to_string()))?;
    Ok(sim.widgets.get(child_id).and_then(|child| child.parent_id) == Some(parent_id))
}

fn transparent_wrapper_parent_id(
    state: &mut rilua::vm::state::LuaState,
    parent_id: u64,
) -> Result<Option<u64>, LoadError> {
    let sim = crate::lua_api::methods::borrow_state(state)
        .map_err(|error| LoadError::Lua(error.to_string()))?;
    Ok(sim.widgets.get(parent_id).and_then(|parent| {
        let synthetic_name = parent
            .name
            .as_deref()
            .is_some_and(|name| name.starts_with("__tpl_"));
        (synthetic_name && parent.parent_key.is_none())
            .then_some(parent.parent_id)
            .flatten()
    }))
}

fn append_parent_array_entry(
    state: &mut rilua::vm::state::LuaState,
    parent_id: u64,
    key: &str,
    child_id: u64,
) -> Result<(), LoadError> {
    use crate::lua_api::methods::{create_table, frame_ref, table_get, table_set};
    use rilua::Val;

    let parent = frame_ref(state, parent_id).map_err(|error| LoadError::Lua(error.to_string()))?;
    let child = frame_ref(state, child_id).map_err(|error| LoadError::Lua(error.to_string()))?;
    let array = match table_get(state, parent, key) {
        Val::Table(existing) => Val::Table(existing),
        _ => {
            let created = create_table(state);
            table_set(state, parent, key, created);
            created
        }
    };
    let Val::Table(array_ref) = array else {
        return Ok(());
    };
    let next_index = next_table_array_index(state, array_ref);
    if let Some(table) = state.gc.tables.get_mut(array_ref) {
        let _ = table.raw_set(Val::Num(next_index as f64), child, &state.gc.string_arena);
    }
    state.gc.barrier_back(array_ref);
    Ok(())
}

fn next_table_array_index(
    state: &rilua::vm::state::LuaState,
    table_ref: rilua::vm::gc::arena::GcRef<rilua::vm::table::Table>,
) -> i64 {
    let mut index = 1_i64;
    while state
        .gc
        .tables
        .get(table_ref)
        .map(|table| !matches!(table.get_int(index), rilua::Val::Nil))
        .unwrap_or(false)
    {
        index += 1;
    }
    index
}

fn apply_fontstring_syncs(env: &LoaderEnv<'_>, syncs: &[TextSync]) {
    for sync in syncs {
        sync_fontstring_text_to_rust(env, &sync.name, &sync.text);
    }
}

/// Create all textures and fontstrings for a frame in a single batched Lua exec.
pub fn create_layer_children_batched_with_name_parent(
    env: &LoaderEnv<'_>,
    frame: &xml::FrameXml,
    parent_name: &str,
    name_parent: &str,
    timing: &mut LoadTiming,
) -> Result<(), LoadError> {
    let mut batch = String::with_capacity(4096);
    let mut attachments: Vec<ParentKeyAttachment> = Vec::new();
    let mut text_syncs: Vec<TextSync> = Vec::new();
    let mut anim_entries: Vec<AnimEntry<'_>> = Vec::new();

    append_layer_children_code(
        frame,
        parent_name,
        name_parent,
        &mut batch,
        &mut attachments,
        &mut anim_entries,
        &mut text_syncs,
        timing,
    );
    exec_batch(env, &batch, parent_name)?;
    let attachment_parent_name =
        if frame.name.is_none() && frame.parent_key.is_none() && frame.parent_array.is_none() {
            name_parent
        } else {
            parent_name
        };
    apply_parent_key_attachments(env, parent_name, attachment_parent_name, &attachments)?;
    apply_texture_anims(env, &anim_entries);
    apply_fontstring_syncs(env, &text_syncs);
    Ok(())
}
