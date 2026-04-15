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
    parent_key: String,
}

fn exec_batch(env: &LoaderEnv<'_>, batch: &str, parent_name: &str) -> Result<(), LoadError> {
    if batch.is_empty() {
        return Ok(());
    }
    env.exec(batch)
        .map_err(|e| LoadError::Lua(format!("layer children on {}: {}", parent_name, e)))
}

fn collect_textures<'a>(frame: &'a xml::FrameXml) -> Vec<CollectedTexture<'a>> {
    let mut result = Vec::new();
    for layers in frame.layers() {
        for layer in &layers.layers {
            let draw_layer = layer.level.as_deref().unwrap_or("ARTWORK");
            let sub_level = layer.texture_sub_level.unwrap_or(0);
            for (texture, is_mask, is_line) in layer.textures() {
                result.push(CollectedTexture {
                    texture,
                    is_mask,
                    is_line,
                    draw_layer: draw_layer.to_string(),
                    sub_level,
                });
            }
        }
    }
    result
}

fn append_texture_code<'a>(
    textures: &[CollectedTexture<'a>],
    parent_name: &str,
    batch: &mut String,
    attachments: &mut Vec<ParentKeyAttachment>,
    anim_entries: &mut Vec<AnimEntry<'a>>,
    timing: &mut LoadTiming,
) {
    for ct in textures {
        if ct.texture.is_virtual == Some(true) {
            if let Some(ref name) = ct.texture.name {
                xml::register_texture_template(name, ct.texture.clone());
            }
            continue;
        }
        let resolved = xml::resolve_texture_inheritance(ct.texture);
        let tex_name = resolve_child_name(resolved.name.as_deref(), parent_name, "__tex_");
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
        if let Some(parent_key) = resolved.parent_key.as_ref() {
            attachments.push(ParentKeyAttachment {
                child_name: tex_name.clone(),
                parent_key: parent_key.clone(),
            });
        }
        if ct.texture.animations.is_some() {
            anim_entries.push(AnimEntry {
                texture: ct.texture,
                tex_name,
            });
        }
        timing.texture_count += 1;
    }
}

fn append_fontstring_code(
    frame: &xml::FrameXml,
    parent_name: &str,
    batch: &mut String,
    attachments: &mut Vec<ParentKeyAttachment>,
    text_syncs: &mut Vec<TextSync>,
    timing: &mut LoadTiming,
) {
    for layers in frame.layers() {
        for layer in &layers.layers {
            let draw_layer = layer.level.as_deref().unwrap_or("ARTWORK");
            let sub_level = layer.texture_sub_level.unwrap_or(0);
            for fs in layer.font_strings() {
                append_single_fontstring(
                    fs,
                    parent_name,
                    draw_layer,
                    sub_level,
                    batch,
                    attachments,
                    text_syncs,
                    timing,
                );
            }
        }
    }
}

fn append_single_fontstring(
    fontstring: &xml::FontStringXml,
    parent_name: &str,
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
    let fs_name = resolve_child_name(fontstring.name.as_deref(), parent_name, "__fs_");
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
    if let Some(parent_key) = fontstring.parent_key.as_ref() {
        attachments.push(ParentKeyAttachment {
            child_name: fs_name.clone(),
            parent_key: parent_key.clone(),
        });
    }
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
    attachments: &[ParentKeyAttachment],
) -> Result<(), LoadError> {
    if attachments.is_empty() {
        return Ok(());
    }

    env.with_state(|state| {
        for attachment in attachments {
            let ids = {
                let sim = crate::lua_api::rilua_methods::borrow_state(state)
                    .map_err(|error| LoadError::Lua(error.to_string()))?;
                (
                    sim.widgets.get_id_by_name(parent_name),
                    sim.widgets.get_id_by_name(&attachment.child_name),
                )
            };
            let (Some(parent_id), Some(child_id)) = ids else {
                continue;
            };
            crate::lua_api::globals::template::assign_parent_key(
                state,
                parent_id,
                &attachment.parent_key,
                child_id,
            )
            .map_err(|error| LoadError::Lua(error.to_string()))?;
        }
        Ok(())
    })
}

fn apply_fontstring_syncs(env: &LoaderEnv<'_>, syncs: &[TextSync]) {
    for sync in syncs {
        sync_fontstring_text_to_rust(env, &sync.name, &sync.text);
    }
}

/// Create all textures and fontstrings for a frame in a single batched Lua exec.
pub fn create_layer_children_batched(
    env: &LoaderEnv<'_>,
    frame: &xml::FrameXml,
    parent_name: &str,
    timing: &mut LoadTiming,
) -> Result<(), LoadError> {
    let mut batch = String::with_capacity(4096);
    let mut attachments: Vec<ParentKeyAttachment> = Vec::new();
    let mut text_syncs: Vec<TextSync> = Vec::new();
    let mut anim_entries: Vec<AnimEntry<'_>> = Vec::new();
    let all_textures = collect_textures(frame);

    append_texture_code(
        &all_textures,
        parent_name,
        &mut batch,
        &mut attachments,
        &mut anim_entries,
        timing,
    );
    append_fontstring_code(
        frame,
        parent_name,
        &mut batch,
        &mut attachments,
        &mut text_syncs,
        timing,
    );
    exec_batch(env, &batch, parent_name)?;
    apply_parent_key_attachments(env, parent_name, &attachments)?;
    apply_texture_anims(env, &anim_entries);
    apply_fontstring_syncs(env, &text_syncs);
    Ok(())
}
