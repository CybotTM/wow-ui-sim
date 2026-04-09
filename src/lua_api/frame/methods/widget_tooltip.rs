//! GameTooltip widget methods: SetOwner, AddLine, AddDoubleLine, tooltip queries, etc.

use super::super::handle::FrameRef;
use super::methods_helpers::get_mixin_override;
use super::widget_tooltip_data::{
    AuraLookupKind, AuraTooltipKind, lookup_aura_from_args, populate_aura_tooltip,
    populate_item_tooltip, populate_spell_tooltip,
};
#[path = "widget_tooltip_helpers.rs"]
mod widget_tooltip_helpers;
use crate::lua_api::frame::handle::get_sim_state;
use crate::lua_api::tooltip::{
    TooltipLine, parse_item_id_from_hyperlink, parse_spell_id_from_hyperlink,
};
use mlua::Value;
use widget_tooltip_helpers::{
    add_double_line_impl, add_get_line_methods, add_tooltip_info_methods,
    add_tooltip_state_methods, copy_tooltip_impl, set_anchor_type_impl, set_frame_stack_impl,
    set_object_tooltip_position_impl, set_owner_impl,
};

pub use super::widget_tooltip_data::set_unit_for_tooltip;
pub(crate) use widget_tooltip_helpers::{fire_tooltip_script, val_to_f32};

const TOOLTIP_MULTIVALUE_STUBS: &[&str] = &["AddFontStrings"];

const TOOLTIP_VARIADIC_STUBS: &[&str] = &[];

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
    add_set_owner_method(methods);
    add_set_anchor_type_method(methods);
    add_set_object_tooltip_position_method(methods);
    add_copy_tooltip_method(methods);
    add_set_allow_show_with_no_lines_method(methods);
    add_set_custom_word_wrap_min_width_method(methods);
    add_set_shrink_to_fit_wrapped_method(methods);
    add_set_frame_stack_method(methods);
    add_clear_lines_method(methods);
}

fn add_set_owner_method<M: mlua::UserDataMethods<FrameRef>>(methods: &mut M) {
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
}

fn add_set_anchor_type_method<M: mlua::UserDataMethods<FrameRef>>(methods: &mut M) {
    methods.add_method("SetAnchorType", |lua, this, args: mlua::Variadic<Value>| {
        let id = this.0;
        if let Some((func, self_val)) = get_mixin_override(lua, id, "SetAnchorType") {
            let mut call_args = vec![self_val];
            call_args.extend(args);
            return func
                .call::<Value>(mlua::MultiValue::from_iter(call_args))
                .map(|_| ());
        }
        set_anchor_type_impl(lua, id, args)
    });
}

fn add_set_object_tooltip_position_method<M: mlua::UserDataMethods<FrameRef>>(methods: &mut M) {
    methods.add_method(
        "SetObjectTooltipPosition",
        |lua, this, args: mlua::Variadic<Value>| {
            let id = this.0;
            if let Some((func, self_val)) = get_mixin_override(lua, id, "SetObjectTooltipPosition")
            {
                let mut call_args = vec![self_val];
                call_args.extend(args);
                return func
                    .call::<Value>(mlua::MultiValue::from_iter(call_args))
                    .map(|_| ());
            }
            set_object_tooltip_position_impl(lua, id)
        },
    );
}

fn add_copy_tooltip_method<M: mlua::UserDataMethods<FrameRef>>(methods: &mut M) {
    methods.add_method("CopyTooltip", |lua, this, args: mlua::Variadic<Value>| {
        let id = this.0;
        if let Some((func, self_val)) = get_mixin_override(lua, id, "CopyTooltip") {
            let mut call_args = vec![self_val];
            call_args.extend(args);
            return func
                .call::<Value>(mlua::MultiValue::from_iter(call_args))
                .map(|_| ());
        }
        copy_tooltip_impl(lua, id, args)
    });
}

fn add_set_allow_show_with_no_lines_method<M: mlua::UserDataMethods<FrameRef>>(methods: &mut M) {
    methods.add_method("SetAllowShowWithNoLines", |lua, this, value: bool| {
        let id = this.0;
        if let Some((func, self_val)) = get_mixin_override(lua, id, "SetAllowShowWithNoLines") {
            return func.call::<Value>((self_val, value)).map(|_| ());
        }
        let state_rc = get_sim_state(lua);
        let mut state = state_rc.borrow_mut();
        if let Some(td) = state.tooltips.get_mut(&id) {
            td.allow_show_with_no_lines = value;
        }
        state.widgets.mark_rect_dirty(id);
        state.widgets.mark_visual_dirty(id);
        Ok(())
    });
}

fn add_set_custom_word_wrap_min_width_method<M: mlua::UserDataMethods<FrameRef>>(methods: &mut M) {
    methods.add_method("SetCustomWordWrapMinWidth", |lua, this, width: f64| {
        let id = this.0;
        if let Some((func, self_val)) = get_mixin_override(lua, id, "SetCustomWordWrapMinWidth") {
            return func.call::<Value>((self_val, width)).map(|_| ());
        }
        let state_rc = get_sim_state(lua);
        let mut state = state_rc.borrow_mut();
        if let Some(td) = state.tooltips.get_mut(&id) {
            td.custom_word_wrap_min_width = Some(width as f32);
        }
        state.widgets.mark_rect_dirty(id);
        state.widgets.mark_visual_dirty(id);
        Ok(())
    });
}

fn add_set_shrink_to_fit_wrapped_method<M: mlua::UserDataMethods<FrameRef>>(methods: &mut M) {
    methods.add_method("SetShrinkToFitWrapped", |lua, this, value: bool| {
        let id = this.0;
        if let Some((func, self_val)) = get_mixin_override(lua, id, "SetShrinkToFitWrapped") {
            return func.call::<Value>((self_val, value)).map(|_| ());
        }
        let state_rc = get_sim_state(lua);
        let mut state = state_rc.borrow_mut();
        if let Some(td) = state.tooltips.get_mut(&id) {
            td.shrink_to_fit_wrapped = value;
        }
        state.widgets.mark_rect_dirty(id);
        state.widgets.mark_visual_dirty(id);
        Ok(())
    });
}

fn add_set_frame_stack_method<M: mlua::UserDataMethods<FrameRef>>(methods: &mut M) {
    methods.add_method("SetFrameStack", |lua, this, args: mlua::Variadic<Value>| {
        let id = this.0;
        if let Some((func, self_val)) = get_mixin_override(lua, id, "SetFrameStack") {
            let mut call_args = vec![self_val];
            call_args.extend(args);
            return func.call::<Value>(mlua::MultiValue::from_iter(call_args));
        }
        set_frame_stack_impl(lua, id, args)
    });
}

fn add_clear_lines_method<M: mlua::UserDataMethods<FrameRef>>(methods: &mut M) {
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
    add_set_spell_book_item(methods);
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

fn add_set_spell_book_item<M: mlua::UserDataMethods<FrameRef>>(methods: &mut M) {
    methods.add_method("SetSpellBookItem", |lua, this, args: mlua::MultiValue| {
        let slot = match args.into_iter().next() {
            Some(Value::Integer(n)) => n as i32,
            Some(Value::Number(n)) => n as i32,
            _ => return Ok(()),
        };
        let Some((_, entry, _)) = crate::lua_api::globals::spellbook_data::get_spell_at_slot(slot)
        else {
            return Ok(());
        };
        populate_spell_tooltip(lua, this.0, entry.spell_id)?;
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
        if let Some(id) = parse_item_id_from_hyperlink(&link) {
            populate_item_tooltip(lua, this.0, id)?;
            fire_tooltip_script(lua, this.0, "OnTooltipSetItem")?;
            return Ok(());
        }
        if let Some(id) = parse_spell_id_from_hyperlink(&link) {
            populate_spell_tooltip(lua, this.0, id)?;
            fire_tooltip_script(lua, this.0, "OnTooltipSetSpell")?;
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
    add_index_aura_tooltip_methods(methods);
    add_aura_instance_id_tooltip_methods(methods);
}

fn add_index_aura_tooltip_methods<M: mlua::UserDataMethods<FrameRef>>(methods: &mut M) {
    add_aura_tooltip_method(
        methods,
        "SetUnitBuff",
        AuraLookupKind::Index,
        AuraTooltipKind::Buff,
    );
    add_aura_tooltip_method(
        methods,
        "SetUnitDebuff",
        AuraLookupKind::Index,
        AuraTooltipKind::Debuff,
    );
    add_aura_tooltip_method(
        methods,
        "SetUnitAura",
        AuraLookupKind::Index,
        AuraTooltipKind::Aura,
    );
}

fn add_aura_instance_id_tooltip_methods<M: mlua::UserDataMethods<FrameRef>>(methods: &mut M) {
    add_aura_tooltip_method(
        methods,
        "SetUnitBuffByAuraInstanceID",
        AuraLookupKind::AuraInstanceId,
        AuraTooltipKind::Buff,
    );
    add_aura_tooltip_method(
        methods,
        "SetUnitDebuffByAuraInstanceID",
        AuraLookupKind::AuraInstanceId,
        AuraTooltipKind::Debuff,
    );
    add_aura_tooltip_method(
        methods,
        "SetUnitAuraByAuraInstanceID",
        AuraLookupKind::AuraInstanceId,
        AuraTooltipKind::Aura,
    );
}

fn add_aura_tooltip_method<M: mlua::UserDataMethods<FrameRef>>(
    methods: &mut M,
    name: &'static str,
    lookup_kind: AuraLookupKind,
    tooltip_kind: AuraTooltipKind,
) {
    methods.add_method(name, move |lua, this, args: mlua::MultiValue| {
        let tooltip_id = this.0;
        let aura = lookup_aura_from_args(lua, &args, lookup_kind, tooltip_kind);
        if let Some(aura) = aura {
            populate_aura_tooltip(lua, tooltip_id, &aura)?;
        }
        Ok(())
    });
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
