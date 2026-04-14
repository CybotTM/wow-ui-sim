use super::{
    apply_animation_groups, apply_button_text, apply_editbox_fontstring,
    apply_inline_button_textures, apply_inline_key_values, apply_mixin,
    apply_templates_from_registry, defer_child_onload, direct, elements, elements_text,
    frame_element_type, lua_global_ref, pop_suppress, push_suppress, rand_id,
};
use crate::loader::chunk_cache;
use crate::loader::helpers::generate_set_point_code;
use crate::lua_api::SimState;
use crate::lua_api::frame::{frame_ref, sync_child_to_lua};
use crate::lua_api::globals::create_frame::create_frame_instance;
use crate::widget::WidgetType;
use crate::xml::{FrameElement, FrameXml, get_template_chain};
use mlua::{Lua, Value};
use std::cell::RefCell;
use std::rc::Rc;

pub(super) fn create_child_frames(
    lua: &Lua,
    state: &Rc<RefCell<SimState>>,
    frame: &FrameXml,
    parent_name: &str,
    subst_parent: &str,
    use_direct_creation: bool,
) {
    let elements = frame.all_frame_elements();
    for child in &elements {
        let Some((child_frame, child_type, intrinsic)) = frame_element_type(child) else {
            continue;
        };
        let _ = create_child_frame_from_template(
            lua,
            state,
            child_frame,
            child_type,
            intrinsic,
            parent_name,
            subst_parent,
            use_direct_creation,
        );
    }
}

pub(super) fn use_direct_runtime_child_creation(template_name: &str) -> bool {
    // Keep nested SpellFX templates on this hot list too. Otherwise the outer
    // ActionButtonSpellFXTemplate fast path still falls back to Lua string
    // child creation when it expands Interrupt/CastingAnim descendants.
    matches!(
        template_name,
        "ActionButtonSpellFXTemplate"
            | "ActionButtonInterruptTemplate"
            | "ActionButtonCastingAnimFrameTemplate"
            | "ActionButtonTemplate"
            | "ActionBarButtonTemplate"
            | "SmallActionButtonTemplate"
            | "MainBarActionBarButtonTemplate"
            | "PetActionButtonTemplate"
            | "StanceButtonTemplate"
            | "PossessButtonTemplate"
            | "MinimalScrollBar"
    )
}

fn create_child_frame_from_template(
    lua: &Lua,
    state: &Rc<RefCell<SimState>>,
    frame: &FrameXml,
    widget_type: &str,
    intrinsic: Option<&str>,
    parent_name: &str,
    subst_parent: &str,
    use_direct_creation: bool,
) -> Option<String> {
    let is_named = frame.name.is_some();
    let child_name = frame
        .name
        .as_ref()
        .map(|name| name.replace("$parent", subst_parent))
        .unwrap_or_else(|| format!("__tpl_{}", rand_id()));

    let child_subst = if is_named { &child_name } else { subst_parent };

    push_suppress(lua);

    let create_result = if use_direct_creation {
        create_child_frame_direct(lua, state, frame, widget_type, parent_name, &child_name)
            .or_else(|error| {
                eprintln!(
                    "[template] Direct child create failed for '{}' (type={}) under '{}': {}. Falling back to Lua path.",
                    child_name, widget_type, parent_name, error
                );
                create_child_frame_via_lua(lua, frame, widget_type, parent_name, &child_name)
            })
    } else {
        create_child_frame_via_lua(lua, frame, widget_type, parent_name, &child_name)
    };
    if let Err(error) = create_result {
        eprintln!(
            "[template] Failed to create child '{}' (type={}) under '{}': {}",
            child_name, widget_type, parent_name, error
        );
        pop_suppress(lua);
        return None;
    }

    if let Some(intrinsic_name) = intrinsic {
        apply_templates_from_registry(lua, state, &child_name, intrinsic_name);
        let intrinsic_code = format!(
            "{}.intrinsic = \"{}\"",
            lua_global_ref(&child_name),
            intrinsic_name
        );
        let _ = chunk_cache::exec(lua, &intrinsic_code, "template-mod");
    }

    let inherits = frame.inherits.as_deref().unwrap_or("");
    if !inherits.is_empty() {
        apply_templates_from_registry(lua, state, &child_name, inherits);
    }

    // If the inline child definition has its own anchors, they fully replace
    // any anchors set by the inherited template (WoW behavior: most-derived
    // anchors win completely, not merged).
    if frame.anchors().is_some() {
        reapply_inline_anchors(state, frame, &child_name, parent_name);
    }

    apply_inline_frame_content(
        lua,
        state,
        frame,
        &child_name,
        child_subst,
        use_direct_creation,
    );

    pop_suppress(lua);
    defer_child_onload(lua, &child_name);
    Some(child_name)
}

fn create_child_frame_via_lua(
    lua: &Lua,
    frame: &FrameXml,
    widget_type: &str,
    parent_name: &str,
    child_name: &str,
) -> mlua::Result<()> {
    let code = build_create_child_code(frame, widget_type, parent_name, child_name);
    chunk_cache::exec(lua, &code, "template-mod")
}

fn create_child_frame_direct(
    lua: &Lua,
    state: &Rc<RefCell<SimState>>,
    frame: &FrameXml,
    widget_type_name: &str,
    parent_name: &str,
    child_name: &str,
) -> mlua::Result<()> {
    let Some(parent_id) = resolve_frame_id(state, parent_name) else {
        return Err(mlua::Error::RuntimeError(format!(
            "parent '{}' missing during direct child create",
            parent_name
        )));
    };
    let Some(widget_type) = WidgetType::from_str(widget_type_name) else {
        return Err(mlua::Error::RuntimeError(format!(
            "unknown widget type '{}'",
            widget_type_name
        )));
    };

    let child_id = create_frame_instance(
        lua,
        state,
        widget_type,
        widget_type_name,
        Some(child_name.to_string()),
        Some(parent_id),
        true,
        frame.xml_id,
    )?;
    apply_direct_child_layout(state, child_id, frame, parent_name);
    apply_direct_child_parent_refs(lua, state, parent_id, child_id, frame)?;
    Ok(())
}

fn apply_direct_child_layout(
    state: &Rc<RefCell<SimState>>,
    child_id: u64,
    frame: &FrameXml,
    parent_name: &str,
) {
    if frame.anchors().is_some() {
        direct::set_anchors(state, child_id, frame, parent_name);
    }
    if frame.set_all_points == Some(true) {
        direct::set_all_points(state, child_id, frame);
    }
    if resolve_inherited_hidden(frame) == Some(true) {
        state.borrow_mut().set_frame_visible(child_id, false);
    }
}

fn apply_direct_child_parent_refs(
    lua: &Lua,
    state: &Rc<RefCell<SimState>>,
    parent_id: u64,
    child_id: u64,
    frame: &FrameXml,
) -> mlua::Result<()> {
    if let Some(parent_key) = resolve_inherited_field(frame, |frame| frame.parent_key.as_ref()) {
        {
            let mut sim = state.borrow_mut();
            if let Some(parent) = sim.widgets.get_mut_visual(parent_id) {
                parent.children_keys.insert(parent_key.clone(), child_id);
            }
        }
        sync_child_to_lua(lua, parent_id, &parent_key, child_id)?;
    }

    let Some(parent_array) = resolve_inherited_field(frame, |frame| frame.parent_array.as_ref())
    else {
        return Ok(());
    };
    let parent_val = frame_ref(lua, parent_id)?;
    let child_val = frame_ref(lua, child_id)?;
    let Value::UserData(parent_ud) = parent_val else {
        return Ok(());
    };
    let fields: mlua::Table = parent_ud.user_value()?;
    let array = match fields.raw_get::<Value>(parent_array.as_str())? {
        Value::Table(existing) => existing,
        _ => {
            let created = lua.create_table()?;
            fields.raw_set(parent_array.as_str(), created.clone())?;
            created
        }
    };
    let next_index = array.raw_len() + 1;
    array.raw_set(next_index, child_val)?;
    Ok(())
}

fn build_create_child_code(
    frame: &FrameXml,
    widget_type: &str,
    parent_name: &str,
    child_name: &str,
) -> String {
    let mut code = format!(
        r#"
        local parent = {}
        if parent then
            local child = CreateFrame("{}", "{}", parent, nil)
        "#,
        lua_global_ref(parent_name),
        widget_type,
        super::escape_lua_string(child_name),
    );

    append_child_size_and_anchors(&mut code, frame, parent_name);
    append_child_id(&mut code, frame);
    append_child_parent_refs(&mut code, frame);
    code.push_str("        end\n");
    code
}

fn append_child_id(code: &mut String, frame: &FrameXml) {
    if let Some(id) = frame.xml_id {
        code.push_str(&format!("            child:SetID({id})\n"));
    }
}

fn resolve_inherited_hidden(frame: &FrameXml) -> Option<bool> {
    frame.hidden.or_else(|| {
        let inherits = frame.inherits.as_deref().unwrap_or("");
        if inherits.is_empty() {
            return None;
        }
        for entry in &get_template_chain(inherits) {
            if entry.frame.hidden.is_some() {
                return entry.frame.hidden;
            }
        }
        None
    })
}

fn resolve_frame_id(state: &Rc<RefCell<SimState>>, frame_name: &str) -> Option<u64> {
    state
        .borrow()
        .widgets
        .get_id_by_name(frame_name)
        .or_else(|| {
            frame_name
                .strip_prefix("__frame_")
                .and_then(|suffix| suffix.parse::<u64>().ok())
        })
}

fn append_child_size_and_anchors(code: &mut String, frame: &FrameXml, parent_name: &str) {
    if let Some(anchors) = frame.anchors() {
        code.push_str(&generate_set_point_code(
            anchors,
            "child",
            "parent",
            parent_name,
            "parent",
        ));
    }
    if frame.set_all_points == Some(true) {
        code.push_str("            child:SetAllPoints(true)\n");
    }
    let hidden = frame.hidden.or_else(|| {
        let inherits = frame.inherits.as_deref().unwrap_or("");
        if inherits.is_empty() {
            return None;
        }
        for entry in &get_template_chain(inherits) {
            if entry.frame.hidden.is_some() {
                return entry.frame.hidden;
            }
        }
        None
    });
    if hidden == Some(true) {
        code.push_str("            child:Hide()\n");
    }
}

/// Clear all template-set anchors and re-apply inline anchors from the child XML.
///
/// `$parent` in the child frame's own anchors refers to the actual parent frame,
/// not the child's resolved global name.
fn reapply_inline_anchors(
    state: &Rc<RefCell<SimState>>,
    frame: &FrameXml,
    child_name: &str,
    parent_name: &str,
) {
    let Some(anchors) = frame.anchors() else {
        return;
    };
    let frame_id = {
        let s = state.borrow();
        s.widgets.get_id_by_name(child_name).or_else(|| {
            child_name
                .strip_prefix("__frame_")
                .and_then(|suffix| suffix.parse::<u64>().ok())
        })
    };
    let Some(fid) = frame_id else { return };
    {
        let mut s = state.borrow_mut();
        if let Some(f) = s.widgets.get_mut_visual(fid) {
            f.clear_all_points();
        }
    }
    let mut s = state.borrow_mut();
    for anchor in &anchors.anchors {
        direct::set_single_anchor(&mut s, fid, anchor, parent_name);
    }
}

fn append_child_parent_refs(code: &mut String, frame: &FrameXml) {
    if let Some(parent_key) = &resolve_inherited_field(frame, |frame| frame.parent_key.as_ref()) {
        let key = super::escape_lua_string(parent_key);
        code.push_str(&format!("            parent[\"{key}\"] = child\n"));
    }
    if let Some(parent_array) = &resolve_inherited_field(frame, |frame| frame.parent_array.as_ref())
    {
        let array_name = super::escape_lua_string(parent_array);
        code.push_str(&format!(
            "            parent[\"{array_name}\"] = parent[\"{array_name}\"] or {{}}\n\
             table.insert(parent[\"{array_name}\"], child)\n"
        ));
    }
}

fn resolve_inherited_field(
    frame: &FrameXml,
    getter: impl Fn(&FrameXml) -> Option<&String>,
) -> Option<String> {
    if let Some(value) = getter(frame) {
        return Some(value.clone());
    }
    let inherits = frame.inherits.as_deref().unwrap_or("");
    if inherits.is_empty() {
        return None;
    }
    for entry in &get_template_chain(inherits) {
        if let Some(value) = getter(&entry.frame) {
            return Some(value.clone());
        }
    }
    None
}

pub(super) fn create_scroll_child_frames(
    lua: &Lua,
    state: &Rc<RefCell<SimState>>,
    children: &[FrameElement],
    parent_name: &str,
    subst_parent: &str,
    use_direct_creation: bool,
) {
    let mut registered_scroll_child = false;
    for child in children {
        let Some((child_frame, child_type, intrinsic)) = frame_element_type(child) else {
            continue;
        };
        let Some(child_name) = create_child_frame_from_template(
            lua,
            state,
            child_frame,
            child_type,
            intrinsic,
            parent_name,
            subst_parent,
            use_direct_creation,
        ) else {
            continue;
        };
        if !registered_scroll_child {
            register_scroll_child(state, parent_name, &child_name);
            registered_scroll_child = true;
        }
    }
}

fn register_scroll_child(state: &Rc<RefCell<SimState>>, parent_name: &str, child_name: &str) {
    let mut sim = state.borrow_mut();
    let Some(parent_id) = sim.widgets.get_id_by_name(parent_name) else {
        return;
    };
    let Some(child_id) = sim.widgets.get_id_by_name(child_name) else {
        return;
    };
    crate::lua_api::frame::methods::widget_scroll::assign_scroll_child(
        &mut sim, parent_id, child_id, false,
    );
}

fn apply_inline_frame_content(
    lua: &Lua,
    state: &Rc<RefCell<SimState>>,
    frame: &FrameXml,
    frame_name: &str,
    subst_parent: &str,
    use_direct_creation: bool,
) {
    apply_mixin(lua, &frame.combined_mixin(), frame_name);
    apply_inline_key_values(lua, frame, frame_name);

    let frame_id = state
        .borrow()
        .widgets
        .get_id_by_name(frame_name)
        .or_else(|| {
            frame_name
                .strip_prefix("__frame_")
                .and_then(|suffix| suffix.parse::<u64>().ok())
        });
    if let Some(frame_id) = frame_id {
        direct::set_size_partial(state, frame_id, frame);
        let inherits = frame.inherits.as_deref().unwrap_or("");
        direct::apply_xml_alpha(state, frame_id, frame, inherits);
    }

    // Create child frames before layers so that relativeKey anchors
    // from FontStrings/Textures to sibling child frames resolve correctly
    // (e.g. $parent.AccountWideIcon in ReputationEntryTemplate). Keep the
    // hot-path direct-create flag on nested inline descendants too, or shared
    // templates like MinimalScrollBar fall back to Lua CreateFrame for
    // grandchildren such as Track -> Thumb.
    create_child_frames(
        lua,
        state,
        frame,
        frame_name,
        subst_parent,
        use_direct_creation,
    );
    if let Some(scroll_child) = frame.scroll_child() {
        create_scroll_child_frames(
            lua,
            state,
            &scroll_child.children,
            frame_name,
            subst_parent,
            use_direct_creation,
        );
    }

    super::apply_layers(lua, frame, frame_name, subst_parent);

    if let Some(thumb) = frame.thumb_texture() {
        elements_text::create_thumb_texture_from_template(lua, thumb, frame_name, subst_parent);
    }
    if let Some(bar) = frame.bar_texture() {
        elements_text::create_bar_texture_from_template(lua, bar, frame_name, subst_parent);
    }

    apply_inline_button_textures(lua, frame, frame_name, subst_parent);
    apply_button_text(lua, frame, frame_name, subst_parent);
    elements_text::apply_button_text_attribute(lua, frame, frame_name);
    apply_editbox_fontstring(lua, frame, frame_name, subst_parent);
    apply_animation_groups(lua, frame, frame_name);

    if let Some(scripts) = frame.scripts() {
        elements::apply_scripts_from_template(lua, scripts, frame_name);
    }
}

#[cfg(test)]
mod tests {
    use super::use_direct_runtime_child_creation;

    #[test]
    fn action_button_template_uses_direct_runtime_child_creation() {
        assert!(use_direct_runtime_child_creation(
            "ActionButtonSpellFXTemplate"
        ));
        assert!(use_direct_runtime_child_creation(
            "ActionButtonInterruptTemplate"
        ));
        assert!(use_direct_runtime_child_creation(
            "ActionButtonCastingAnimFrameTemplate"
        ));
        assert!(use_direct_runtime_child_creation("ActionButtonTemplate"));
        assert!(use_direct_runtime_child_creation("MinimalScrollBar"));
    }

    #[test]
    fn unrelated_template_skips_direct_runtime_child_creation() {
        assert!(!use_direct_runtime_child_creation("UIPanelButtonTemplate"));
    }
}
