//! GameTooltip widget methods: SetOwner, AddLine, AddDoubleLine, tooltip queries, etc.

use super::super::handle::FrameRef;
use super::methods_helpers::get_mixin_override;
use super::widget_tooltip_data::{
    lookup_aura_from_args, parse_item_id_from_hyperlink, populate_aura_tooltip,
    populate_item_tooltip, populate_spell_tooltip,
};
use crate::lua_api::frame::handle::{extract_frame_id, frame_ref, get_sim_state};
use crate::lua_api::tooltip::TooltipLine;
use crate::widget::{Anchor, AnchorPoint};
use mlua::Value;

pub use super::widget_tooltip_data::set_unit_for_tooltip;

const TOOLTIP_MULTIVALUE_STUBS: &[&str] = &["AddFontStrings"];

const TOOLTIP_VARIADIC_STUBS: &[&str] = &[
    "CopyTooltip",
    "SetAllowShowWithNoLines",
    "SetAnchorType",
    "SetCustomWordWrapMinWidth",
    "SetFrameStack",
    "SetObjectTooltipPosition",
    "SetShrinkToFitWrapped",
];

pub fn add_tooltip_methods<M: mlua::UserDataMethods<FrameRef>>(methods: &mut M) {
    add_tooltip_setup_methods(methods);
    add_tooltip_addline_methods(methods);
    add_tooltip_doubleline_methods(methods);
    add_tooltip_data_query_stubs(methods);
}

fn add_tooltip_setup_methods<M: mlua::UserDataMethods<FrameRef>>(methods: &mut M) {
    add_tooltip_owner_methods(methods);
    add_tooltip_query_methods(methods);
    add_tooltip_padding_override_methods(methods);
    add_tooltip_settext_methods(methods);
    add_tooltip_info_methods(methods);
    add_tooltip_state_methods(methods);
}

fn add_tooltip_owner_methods<M: mlua::UserDataMethods<FrameRef>>(methods: &mut M) {
    methods.add_method("SetOwner", |lua, this, args: mlua::MultiValue| {
        let id = this.0;
        if let Some((func, self_val)) = get_mixin_override(lua, id, "SetOwner") {
            let mut call_args = vec![self_val];
            call_args.extend(args);
            return func
                .call::<Value>(mlua::MultiValue::from_iter(call_args))
                .map(|_| ());
        }
        set_owner_impl(lua, id, args)
    });

    methods.add_method("ClearLines", |lua, this, ()| {
        let id = this.0;
        {
            let state_rc = get_sim_state(lua);
            let mut state = state_rc.borrow_mut();
            if let Some(td) = state.tooltips.get_mut(&id) {
                td.lines.clear();
                td.spell_id = None;
            }
        }
        fire_tooltip_script(lua, id, "OnTooltipCleared")?;
        Ok(())
    });
}

fn add_tooltip_addline_methods<M: mlua::UserDataMethods<FrameRef>>(methods: &mut M) {
    methods.add_method("AddLine", |lua, this, args: mlua::MultiValue| {
        let id = this.0;
        let mut it = args.into_iter();
        let text = match it.next() {
            Some(Value::String(s)) => s.to_string_lossy().to_string(),
            Some(Value::Number(n)) => n.to_string(),
            Some(Value::Integer(n)) => n.to_string(),
            _ => return Ok(()),
        };
        let r = val_to_f32(it.next(), 1.0);
        let g = val_to_f32(it.next(), 1.0);
        let b = val_to_f32(it.next(), 1.0);
        let wrap = matches!(it.next(), Some(Value::Boolean(true)));
        let state_rc = get_sim_state(lua);
        let mut state = state_rc.borrow_mut();
        if let Some(td) = state.tooltips.get_mut(&id) {
            td.lines.push(TooltipLine {
                left_text: text,
                left_color: (r, g, b),
                right_text: None,
                right_color: (1.0, 1.0, 1.0),
                wrap,
                texture: None,
            });
        }
        Ok(())
    });
}

fn add_tooltip_doubleline_methods<M: mlua::UserDataMethods<FrameRef>>(methods: &mut M) {
    methods.add_method("AddDoubleLine", |lua, this, args: mlua::MultiValue| {
        add_double_line_impl(lua, this.0, args)
    });
}

fn add_tooltip_data_query_stubs<M: mlua::UserDataMethods<FrameRef>>(methods: &mut M) {
    add_set_spell_by_id(methods);
    add_set_item_by_id(methods);
    add_set_inventory_item(methods);
    add_set_hyperlink(methods);
    add_set_unit(methods);
    add_aura_tooltip_methods(methods);
    add_tooltip_multivalue_stubs(methods, TOOLTIP_MULTIVALUE_STUBS);
    add_tooltip_variadic_stubs(methods, TOOLTIP_VARIADIC_STUBS);
    add_line_spacing_methods(methods);
    add_get_line_methods(methods);
    add_line_count_methods(methods);
}

fn add_tooltip_multivalue_stubs<M: mlua::UserDataMethods<FrameRef>>(
    methods: &mut M,
    names: &[&'static str],
) {
    for name in names {
        methods.add_method(*name, |_, _this, _args: mlua::MultiValue| Ok(()));
    }
}

fn add_tooltip_variadic_stubs<M: mlua::UserDataMethods<FrameRef>>(
    methods: &mut M,
    names: &[&'static str],
) {
    for name in names {
        methods.add_method(*name, |_, _this, _: mlua::Variadic<Value>| Ok(()));
    }
}

fn add_line_spacing_methods<M: mlua::UserDataMethods<FrameRef>>(methods: &mut M) {
    methods.add_method("SetCustomLineSpacing", |lua, this, spacing: f64| {
        let state_rc = get_sim_state(lua);
        let mut state = state_rc.borrow_mut();
        if let Some(td) = state.tooltips.get_mut(&this.0) {
            td.line_spacing = Some(spacing as f32);
        }
        Ok(())
    });

    methods.add_method("GetCustomLineSpacing", |lua, this, ()| {
        let state_rc = get_sim_state(lua);
        let state = state_rc.borrow();
        Ok(state
            .tooltips
            .get(&this.0)
            .and_then(|td| td.line_spacing)
            .map(|s| s as f64)
            .unwrap_or(0.0))
    });
}

fn add_line_count_methods<M: mlua::UserDataMethods<FrameRef>>(methods: &mut M) {
    for name in ["NumLines", "GetNumLines"] {
        methods.add_method(name, |lua, this, ()| {
            let state_rc = get_sim_state(lua);
            let state = state_rc.borrow();
            Ok(state
                .tooltips
                .get(&this.0)
                .map(|td| td.lines.len())
                .unwrap_or(0) as i32)
        });
    }
}

fn add_set_spell_by_id<M: mlua::UserDataMethods<FrameRef>>(methods: &mut M) {
    methods.add_method("SetSpellByID", |lua, this, args: mlua::MultiValue| {
        let spell_id = match args.into_iter().next() {
            Some(Value::Integer(n)) => n as u32,
            Some(Value::Number(n)) => n as u32,
            _ => return Ok(()),
        };
        populate_spell_tooltip(lua, this.0, spell_id)?;
        fire_tooltip_script(lua, this.0, "OnTooltipSetSpell")
    });
}

fn add_set_item_by_id<M: mlua::UserDataMethods<FrameRef>>(methods: &mut M) {
    methods.add_method("SetItemByID", |lua, this, args: mlua::MultiValue| {
        let item_id = match args.into_iter().next() {
            Some(Value::Integer(n)) => n as u32,
            Some(Value::Number(n)) => n as u32,
            _ => return Ok(()),
        };
        populate_item_tooltip(lua, this.0, item_id)?;
        fire_tooltip_script(lua, this.0, "OnTooltipSetItem")
    });
}

fn add_set_inventory_item<M: mlua::UserDataMethods<FrameRef>>(methods: &mut M) {
    // SetInventoryItem("player", slot) → hasItem, hasCooldown, repairCost
    methods.add_method("SetInventoryItem", |lua, this, args: mlua::MultiValue| {
        let mut iter = args.into_iter();
        let _unit = iter.next(); // always "player" in practice
        let slot = match iter.next() {
            Some(Value::Integer(n)) => n as i32,
            Some(Value::Number(n)) => n as i32,
            _ => return Ok(mlua::MultiValue::from_vec(vec![Value::Boolean(false)])),
        };
        let state_rc = get_sim_state(lua);
        let item_id = {
            let st = state_rc.borrow();
            st.player
                .equipped_items
                .get(&slot)
                .map(|e| e.item_id)
                .filter(|&id| id > 0)
        };
        let Some(id) = item_id else {
            return Ok(mlua::MultiValue::from_vec(vec![Value::Boolean(false)]));
        };
        populate_item_tooltip(lua, this.0, id)?;
        fire_tooltip_script(lua, this.0, "OnTooltipSetItem")?;
        Ok(mlua::MultiValue::from_vec(vec![
            Value::Boolean(true),  // hasItem
            Value::Boolean(false), // hasCooldown
            Value::Integer(0),     // repairCost
        ]))
    });
}

fn add_set_hyperlink<M: mlua::UserDataMethods<FrameRef>>(methods: &mut M) {
    methods.add_method("SetHyperlink", |lua, this, args: mlua::MultiValue| {
        let link = match args.into_iter().next() {
            Some(Value::String(s)) => s.to_string_lossy().to_string(),
            _ => return Ok(()),
        };
        let item_id = parse_item_id_from_hyperlink(&link);
        if let Some(id) = item_id {
            populate_item_tooltip(lua, this.0, id)?;
            fire_tooltip_script(lua, this.0, "OnTooltipSetItem")?;
        }
        Ok(())
    });
}

fn add_set_unit<M: mlua::UserDataMethods<FrameRef>>(methods: &mut M) {
    // Note: GameTooltip:SetUnit is actually dispatched from widget_model.rs
    // because ModelScene also has SetUnit. The model method checks if the frame
    // is a tooltip and delegates here via set_unit_for_tooltip.
    // This registers a fallback for non-model tooltip frames.
    methods.add_method("SetUnit", |lua, this, args: mlua::MultiValue| {
        set_unit_for_tooltip(lua, this.0, args)
    });
}

fn add_aura_tooltip_methods<M: mlua::UserDataMethods<FrameRef>>(methods: &mut M) {
    for name in [
        "SetUnitBuff",
        "SetUnitDebuff",
        "SetUnitAura",
        "SetUnitBuffByAuraInstanceID",
        "SetUnitDebuffByAuraInstanceID",
    ] {
        methods.add_method(name, |lua, this, args: mlua::MultiValue| {
            let tooltip_id = this.0;
            let aura = lookup_aura_from_args(lua, &args);
            if let Some(aura) = aura {
                populate_aura_tooltip(lua, tooltip_id, &aura)?;
            }
            Ok(())
        });
    }
}

fn add_tooltip_query_methods<M: mlua::UserDataMethods<FrameRef>>(methods: &mut M) {
    methods.add_method("GetUnit", |_, _this, ()| {
        Ok::<(Option<String>, Option<String>), mlua::Error>((None, None))
    });
    methods.add_method("GetSpell", |lua, this, ()| {
        let state_rc = get_sim_state(lua);
        let state = state_rc.borrow();
        let spell_id = state.tooltips.get(&this.0).and_then(|td| td.spell_id);
        match spell_id {
            Some(id) => {
                let name = crate::spells::get_spell(id)
                    .map(|s| s.name.to_string())
                    .unwrap_or_else(|| format!("Spell {}", id));
                Ok::<(Option<String>, Option<i32>), mlua::Error>((Some(name), Some(id as i32)))
            }
            None => Ok((None, None)),
        }
    });
    methods.add_method("GetItem", |_, _this, ()| {
        Ok::<(Option<String>, Option<String>), mlua::Error>((None, None))
    });
    add_tooltip_texture_methods(methods);
    add_tooltip_minwidth_methods(methods);
}

fn add_tooltip_texture_methods<M: mlua::UserDataMethods<FrameRef>>(methods: &mut M) {
    use crate::lua_api::tooltip::TooltipTexture;

    methods.add_method("AddTexture", |lua, this, args: mlua::MultiValue| {
        let file_data_id = match args.into_iter().next() {
            Some(Value::Integer(n)) => n as u32,
            Some(Value::Number(n)) => n as u32,
            Some(Value::String(s)) => s.to_string_lossy().parse::<u32>().unwrap_or(0),
            _ => return Ok(()),
        };
        push_texture_line(lua, this.0, TooltipTexture::FileDataId(file_data_id))
    });

    methods.add_method("AddAtlas", |lua, this, args: mlua::MultiValue| {
        let atlas_name = match args.into_iter().next() {
            Some(Value::String(s)) => s.to_string_lossy().to_string(),
            _ => return Ok(()),
        };
        push_texture_line(lua, this.0, TooltipTexture::Atlas(atlas_name))
    });
}

fn push_texture_line(
    lua: &mlua::Lua,
    tooltip_id: u64,
    texture: crate::lua_api::tooltip::TooltipTexture,
) -> mlua::Result<()> {
    let state_rc = get_sim_state(lua);
    let mut state = state_rc.borrow_mut();
    if let Some(td) = state.tooltips.get_mut(&tooltip_id) {
        td.lines.push(TooltipLine {
            left_text: String::new(),
            left_color: (1.0, 1.0, 1.0),
            right_text: None,
            right_color: (1.0, 1.0, 1.0),
            wrap: false,
            texture: Some(texture),
        });
    }
    Ok(())
}

fn add_tooltip_minwidth_methods<M: mlua::UserDataMethods<FrameRef>>(methods: &mut M) {
    methods.add_method("SetMinimumWidth", |lua, this, width: f32| {
        let state_rc = get_sim_state(lua);
        let mut state = state_rc.borrow_mut();
        if let Some(td) = state.tooltips.get_mut(&this.0) {
            td.min_width = width;
        }
        Ok(())
    });

    methods.add_method("GetMinimumWidth", |lua, this, ()| {
        let state_rc = get_sim_state(lua);
        let state = state_rc.borrow();
        Ok(state
            .tooltips
            .get(&this.0)
            .map(|td| td.min_width)
            .unwrap_or(0.0))
    });
}

fn add_tooltip_padding_override_methods<M: mlua::UserDataMethods<FrameRef>>(methods: &mut M) {
    methods.add_method("SetPadding", |lua, this, args: mlua::MultiValue| {
        if call_tooltip_padding_override(lua, this.0, "SetPadding", args.clone())? {
            return Ok(());
        }
        let padding = parse_padding_arg(args);
        let state_rc = get_sim_state(lua);
        let mut state = state_rc.borrow_mut();
        if let Some(td) = state.tooltips.get_mut(&this.0) {
            td.padding = padding;
        }
        Ok(())
    });

    methods.add_method("GetPadding", |lua, this, ()| {
        if let Some(override_value) = get_tooltip_padding_override(lua, this.0, "GetPadding")? {
            return Ok(override_value);
        }
        Ok(padding_multi_value(read_tooltip_padding(lua, this.0)))
    });

    methods.add_method("ClearPadding", |lua, this, ()| {
        clear_tooltip_padding(lua, this.0)
    });
}

fn call_tooltip_padding_override(
    lua: &mlua::Lua,
    id: u64,
    method: &str,
    args: mlua::MultiValue,
) -> mlua::Result<bool> {
    if let Some((func, self_val)) = get_mixin_override(lua, id, method) {
        let mut call_args = vec![self_val];
        call_args.extend(args);
        func.call::<()>(mlua::MultiValue::from_iter(call_args))?;
        return Ok(true);
    }
    Ok(false)
}

fn get_tooltip_padding_override(
    lua: &mlua::Lua,
    id: u64,
    method: &str,
) -> mlua::Result<Option<mlua::MultiValue>> {
    if let Some((func, self_val)) = get_mixin_override(lua, id, method) {
        return func.call::<mlua::MultiValue>(self_val).map(Some);
    }
    Ok(None)
}

fn parse_padding_arg(args: mlua::MultiValue) -> f32 {
    args.into_iter()
        .next()
        .and_then(|value| match value {
            Value::Number(n) => Some(n as f32),
            Value::Integer(n) => Some(n as f32),
            _ => None,
        })
        .unwrap_or(0.0)
}

fn read_tooltip_padding(lua: &mlua::Lua, id: u64) -> f64 {
    let state_rc = get_sim_state(lua);
    let state = state_rc.borrow();
    state
        .tooltips
        .get(&id)
        .map(|td| td.padding as f64)
        .unwrap_or(0.0)
}

fn padding_multi_value(padding: f64) -> mlua::MultiValue {
    mlua::MultiValue::from_iter(std::iter::once(Value::Number(padding)))
}

fn clear_tooltip_padding(lua: &mlua::Lua, id: u64) -> mlua::Result<()> {
    let state_rc = get_sim_state(lua);
    let mut state = state_rc.borrow_mut();
    if let Some(td) = state.tooltips.get_mut(&id) {
        td.padding = 0.0;
    }
    Ok(())
}

fn add_tooltip_settext_methods<M: mlua::UserDataMethods<FrameRef>>(methods: &mut M) {
    methods.add_method("AppendText", |lua, this, text: String| {
        let state_rc = get_sim_state(lua);
        let mut state = state_rc.borrow_mut();
        if let Some(td) = state.tooltips.get_mut(&this.0)
            && let Some(last) = td.lines.last_mut()
        {
            last.left_text.push_str(&text);
        }
        Ok(())
    });
}

fn add_tooltip_info_methods<M: mlua::UserDataMethods<FrameRef>>(methods: &mut M) {
    methods.add_method("IsOwned", |lua, this, frame: Value| {
        let check_id = extract_frame_id(&frame);
        let state_rc = get_sim_state(lua);
        let state = state_rc.borrow();
        let owned = state
            .tooltips
            .get(&this.0)
            .is_some_and(|td| td.owner_id.is_some() && td.owner_id == check_id);
        Ok(owned)
    });

    methods.add_method("GetOwner", |lua, this, ()| {
        let owner_id = {
            let state_rc = get_sim_state(lua);
            let state = state_rc.borrow();
            state.tooltips.get(&this.0).and_then(|td| td.owner_id)
        };
        match owner_id {
            Some(oid) => frame_ref(lua, oid),
            None => Ok(Value::Nil),
        }
    });

    methods.add_method("GetAnchorType", |lua, this, ()| {
        let state_rc = get_sim_state(lua);
        let state = state_rc.borrow();
        let anchor = state
            .tooltips
            .get(&this.0)
            .map(|td| td.anchor_type.clone())
            .unwrap_or_else(|| "ANCHOR_NONE".to_string());
        Ok(anchor)
    });
}

fn add_tooltip_state_methods<M: mlua::UserDataMethods<FrameRef>>(methods: &mut M) {
    methods.add_method("FadeOut", |lua, this, ()| {
        let state_rc = get_sim_state(lua);
        let mut state = state_rc.borrow_mut();
        state.set_frame_visible(this.0, false);
        if let Some(td) = state.tooltips.get_mut(&this.0) {
            td.owner_id = None;
        }
        Ok(())
    });
}

// --- GetLeftLine / GetRightLine ---

fn add_get_line_methods<M: mlua::UserDataMethods<FrameRef>>(methods: &mut M) {
    methods.add_method("GetLeftLine", |lua, this, index: i32| {
        get_tooltip_line_fontstring(lua, this.0, index, Side::Left)
    });
    methods.add_method("GetRightLine", |lua, this, index: i32| {
        get_tooltip_line_fontstring(lua, this.0, index, Side::Right)
    });
}

enum Side {
    Left,
    Right,
}

fn get_tooltip_line_fontstring(
    lua: &mlua::Lua,
    tooltip_id: u64,
    index: i32,
    side: Side,
) -> mlua::Result<Value> {
    if index < 1 {
        return Ok(Value::Nil);
    }
    let idx = (index - 1) as usize;

    // Ensure enough FontStrings exist for the current line count.
    ensure_tooltip_fontstrings(lua, tooltip_id)?;

    let state_rc = get_sim_state(lua);
    let state = state_rc.borrow();
    let td = match state.tooltips.get(&tooltip_id) {
        Some(td) => td,
        None => return Ok(Value::Nil),
    };

    let ids = match side {
        Side::Left => &td.left_line_ids,
        Side::Right => &td.right_line_ids,
    };

    match ids.get(idx) {
        Some(&fs_id) => frame_ref(lua, fs_id),
        None => Ok(Value::Nil),
    }
}

/// Create FontString children for any tooltip lines that don't have them yet,
/// and sync text/color from `td.lines` to the FontString widgets.
fn ensure_tooltip_fontstrings(lua: &mlua::Lua, tooltip_id: u64) -> mlua::Result<()> {
    let state_rc = get_sim_state(lua);
    let mut state = state_rc.borrow_mut();

    let tooltip_name = state
        .widgets
        .get(tooltip_id)
        .and_then(|f| f.name.clone())
        .unwrap_or_default();

    let (line_count, existing_left, existing_right) = match state.tooltips.get(&tooltip_id) {
        Some(td) => (
            td.lines.len(),
            td.left_line_ids.len(),
            td.right_line_ids.len(),
        ),
        None => return Ok(()),
    };

    let new_left = create_line_fontstrings(
        &mut state,
        tooltip_id,
        &tooltip_name,
        "TextLeft",
        existing_left,
        line_count,
    );
    let new_right = create_line_fontstrings(
        &mut state,
        tooltip_id,
        &tooltip_name,
        "TextRight",
        existing_right,
        line_count,
    );

    let td = state.tooltips.get_mut(&tooltip_id).unwrap();
    td.left_line_ids.extend(&new_left);
    td.right_line_ids.extend(&new_right);
    let sync_data = collect_line_sync_data(td);

    sync_fontstring_text(&mut state, &sync_data);

    drop(state);
    register_fontstring_globals(lua, &state_rc, &new_left, &new_right)
}

/// Create FontString children for line indices `existing..target`.
fn create_line_fontstrings(
    state: &mut crate::lua_api::state::SimState,
    parent_id: u64,
    tooltip_name: &str,
    suffix: &str,
    existing: usize,
    target: usize,
) -> Vec<u64> {
    (existing..target)
        .map(|i| {
            let name = format!("{}{}{}", tooltip_name, suffix, i + 1);
            let fs = crate::widget::Frame::new(
                crate::widget::WidgetType::FontString,
                Some(name),
                Some(parent_id),
            );
            let fs_id = fs.id;
            state.widgets.register(fs);
            state.widgets.add_child(parent_id, fs_id);
            fs_id
        })
        .collect()
}

struct LineSyncEntry {
    left_id: Option<u64>,
    right_id: Option<u64>,
    left_text: String,
    left_color: (f32, f32, f32),
    right_text: Option<String>,
    right_color: (f32, f32, f32),
}

fn collect_line_sync_data(td: &crate::lua_api::tooltip::TooltipData) -> Vec<LineSyncEntry> {
    td.lines
        .iter()
        .enumerate()
        .map(|(i, line)| LineSyncEntry {
            left_id: td.left_line_ids.get(i).copied(),
            right_id: td.right_line_ids.get(i).copied(),
            left_text: line.left_text.clone(),
            left_color: line.left_color,
            right_text: line.right_text.clone(),
            right_color: line.right_color,
        })
        .collect()
}

fn sync_fontstring_text(state: &mut crate::lua_api::state::SimState, entries: &[LineSyncEntry]) {
    for entry in entries {
        if let Some(id) = entry.left_id
            && let Some(fs) = state.widgets.get_mut_visual(id)
        {
            fs.text = Some(entry.left_text.clone());
            let (r, g, b) = entry.left_color;
            fs.text_color = crate::widget::Color::new(r, g, b, 1.0);
        }
        if let Some(id) = entry.right_id
            && let Some(fs) = state.widgets.get_mut_visual(id)
        {
            fs.text = entry.right_text.clone();
            let (r, g, b) = entry.right_color;
            fs.text_color = crate::widget::Color::new(r, g, b, 1.0);
        }
    }
}

fn register_fontstring_globals(
    lua: &mlua::Lua,
    state_rc: &std::rc::Rc<std::cell::RefCell<crate::lua_api::state::SimState>>,
    new_left: &[u64],
    new_right: &[u64],
) -> mlua::Result<()> {
    for &fs_id in new_left.iter().chain(new_right.iter()) {
        let ud = frame_ref(lua, fs_id)?;
        let name = {
            let state = state_rc.borrow();
            state.widgets.get(fs_id).and_then(|f| f.name.clone())
        };
        if let Some(n) = name {
            lua.globals().raw_set(n.as_str(), ud)?;
        }
    }
    Ok(())
}

// --- Positioning ---

fn set_owner_impl(lua: &mlua::Lua, id: u64, args: mlua::MultiValue) -> mlua::Result<()> {
    let mut args_iter = args.into_iter();
    let owner_val = match args_iter.next() {
        Some(v) if extract_frame_id(&v).is_some() => v,
        _ => {
            return Err(mlua::Error::runtime(
                "Usage: GameTooltip:SetOwner(owner[, anchor])",
            ));
        }
    };
    let anchor: String = match args_iter.next() {
        Some(Value::String(s)) => {
            let s = s.to_string_lossy().to_string();
            if is_valid_anchor_type(&s) {
                s
            } else {
                "ANCHOR_LEFT".to_string()
            }
        }
        _ => "ANCHOR_LEFT".to_string(),
    };
    let owner_id = extract_frame_id(&owner_val);
    {
        let state_rc = get_sim_state(lua);
        let mut state = state_rc.borrow_mut();
        if let Some(td) = state.tooltips.get_mut(&id) {
            td.lines.clear();
            td.spell_id = None;
            td.owner_id = owner_id;
            td.anchor_type = anchor.clone();
        }
        position_tooltip(&mut state, id, owner_id, &anchor);
        state.set_frame_visible(id, true);
    }
    fire_tooltip_script(lua, id, "OnTooltipCleared")?;
    Ok(())
}

fn is_valid_anchor_type(s: &str) -> bool {
    matches!(
        s,
        "ANCHOR_LEFT"
            | "ANCHOR_RIGHT"
            | "ANCHOR_TOP"
            | "ANCHOR_BOTTOM"
            | "ANCHOR_TOPLEFT"
            | "ANCHOR_TOPRIGHT"
            | "ANCHOR_BOTTOMLEFT"
            | "ANCHOR_BOTTOMRIGHT"
            | "ANCHOR_CURSOR"
            | "ANCHOR_PRESERVE"
            | "ANCHOR_NONE"
    )
}

fn add_double_line_impl(lua: &mlua::Lua, id: u64, args: mlua::MultiValue) -> mlua::Result<()> {
    let mut it = args.into_iter();
    let left = match it.next() {
        Some(Value::String(s)) => s.to_string_lossy().to_string(),
        Some(Value::Number(n)) => n.to_string(),
        Some(Value::Integer(n)) => n.to_string(),
        _ => return Ok(()),
    };
    let right = match it.next() {
        Some(Value::String(s)) => s.to_string_lossy().to_string(),
        Some(Value::Number(n)) => n.to_string(),
        Some(Value::Integer(n)) => n.to_string(),
        _ => String::new(),
    };
    let lr = val_to_f32(it.next(), 1.0);
    let lg = val_to_f32(it.next(), 1.0);
    let lb = val_to_f32(it.next(), 1.0);
    let rr = val_to_f32(it.next(), 1.0);
    let rg = val_to_f32(it.next(), 1.0);
    let rb = val_to_f32(it.next(), 1.0);
    let state_rc = get_sim_state(lua);
    let mut state = state_rc.borrow_mut();
    if let Some(td) = state.tooltips.get_mut(&id) {
        td.lines.push(TooltipLine {
            left_text: left,
            left_color: (lr, lg, lb),
            right_text: Some(right),
            right_color: (rr, rg, rb),
            wrap: false,
            texture: None,
        });
    }
    Ok(())
}

fn position_tooltip(
    state: &mut crate::lua_api::state::SimState,
    tooltip_id: u64,
    owner_id: Option<u64>,
    anchor_type: &str,
) {
    let frame = match state.widgets.get_mut_visual(tooltip_id) {
        Some(f) => f,
        None => return,
    };
    frame.anchors.clear();
    match anchor_type {
        "ANCHOR_CURSOR" => {
            let (mx, my) = state.mouse_position.unwrap_or((0.0, 0.0));
            frame.anchors.push(Anchor {
                point: AnchorPoint::TopLeft,
                relative_to: None,
                relative_to_id: None,
                relative_point: AnchorPoint::TopLeft,
                x_offset: mx,
                y_offset: my + 20.0,
            });
        }
        "ANCHOR_NONE" => {}
        _ => {
            let owner = match owner_id {
                Some(id) => id,
                None => return,
            };
            let (tp, rp) = anchor_points_for_type(anchor_type);
            frame.anchors.push(Anchor {
                point: tp,
                relative_to: None,
                relative_to_id: Some(owner as usize),
                relative_point: rp,
                x_offset: 0.0,
                y_offset: 0.0,
            });
        }
    }
}

fn anchor_points_for_type(anchor_type: &str) -> (AnchorPoint, AnchorPoint) {
    match anchor_type {
        "ANCHOR_RIGHT" => (AnchorPoint::TopLeft, AnchorPoint::TopRight),
        "ANCHOR_LEFT" => (AnchorPoint::TopRight, AnchorPoint::TopLeft),
        "ANCHOR_TOPLEFT" => (AnchorPoint::BottomLeft, AnchorPoint::TopLeft),
        "ANCHOR_TOPRIGHT" => (AnchorPoint::BottomLeft, AnchorPoint::TopRight),
        "ANCHOR_BOTTOMLEFT" => (AnchorPoint::TopLeft, AnchorPoint::BottomLeft),
        "ANCHOR_BOTTOMRIGHT" => (AnchorPoint::TopLeft, AnchorPoint::BottomRight),
        _ => (AnchorPoint::TopLeft, AnchorPoint::TopRight),
    }
}

// --- Shared helpers (pub(super) so widget_editbox and widget_slider can use them) ---

/// Fire a script handler on a frame (e.g. OnTooltipCleared).
pub(super) fn fire_tooltip_script(
    lua: &mlua::Lua,
    frame_id: u64,
    handler: &str,
) -> mlua::Result<()> {
    if let Some(func) = crate::lua_api::script_helpers::get_script(lua, frame_id, handler)
        && let Some(frame_ud) = crate::lua_api::script_helpers::get_frame_ref(lua, frame_id)
        && let Err(e) = func.call::<()>(frame_ud)
    {
        crate::lua_api::script_helpers::call_error_handler(lua, &e.to_string());
    }
    Ok(())
}

/// Extract f32 from a Lua Value, returning default if nil/absent.
pub(super) fn val_to_f32(val: Option<Value>, default: f32) -> f32 {
    match val {
        Some(Value::Number(n)) => n as f32,
        Some(Value::Integer(n)) => n as f32,
        _ => default,
    }
}

/// Strip HTML tags from a string, returning plain text.
pub(super) fn strip_html_tags(html: &str) -> String {
    let mut result = String::with_capacity(html.len());
    let mut in_tag = false;
    for ch in html.chars() {
        match ch {
            '<' => in_tag = true,
            '>' => in_tag = false,
            _ if !in_tag => result.push(ch),
            _ => {}
        }
    }
    result
}
