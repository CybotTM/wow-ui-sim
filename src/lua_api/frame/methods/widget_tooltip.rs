//! GameTooltip widget methods: SetOwner, AddLine, AddDoubleLine, tooltip queries, etc.

use super::super::handle::FrameRef;
use super::methods_helpers::get_mixin_override;
use crate::lua_api::frame::handle::{extract_frame_id, frame_ref, get_sim_state};
use crate::lua_api::tooltip::TooltipLine;
use crate::widget::{Anchor, AnchorPoint};
use mlua::Value;

const TOOLTIP_MULTIVALUE_STUBS: &[&str] = &["AddAtlas", "AddFontStrings"];

const TOOLTIP_VARIADIC_STUBS: &[&str] = &[
    "CopyTooltip",
    "SetAllowShowWithNoLines",
    "SetAnchorType",
    "SetCustomLineSpacing",
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
    add_tooltip_i32_stub(methods, "SetSpellByID");
    add_set_item_by_id(methods);
    add_set_hyperlink(methods);
    add_set_unit(methods);
    add_aura_tooltip_methods(methods);
    add_tooltip_multivalue_stubs(methods, TOOLTIP_MULTIVALUE_STUBS);
    add_tooltip_variadic_stubs(methods, TOOLTIP_VARIADIC_STUBS);
    add_custom_line_spacing_getter(methods);
    add_tooltip_nil_getter(methods, "GetLeftLine");
    add_tooltip_nil_getter(methods, "GetRightLine");
    add_line_count_methods(methods);
}

fn add_tooltip_i32_stub<M: mlua::UserDataMethods<FrameRef>>(methods: &mut M, name: &'static str) {
    methods.add_method(name, |_, _this, _value: i32| Ok(()));
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

fn add_tooltip_nil_getter<M: mlua::UserDataMethods<FrameRef>>(methods: &mut M, name: &'static str) {
    methods.add_method(name, |_, _this, ()| Ok(Value::Nil));
}

fn add_custom_line_spacing_getter<M: mlua::UserDataMethods<FrameRef>>(methods: &mut M) {
    methods.add_method("GetCustomLineSpacing", |_, _this, ()| Ok(0.0f64));
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

fn populate_item_tooltip(lua: &mlua::Lua, tooltip_id: u64, item_id: u32) -> mlua::Result<()> {
    let item = match crate::items::get_item(item_id) {
        Some(i) => i,
        None => return Ok(()),
    };
    let (nr, ng, nb) = quality_color_rgb(item.quality);
    let slot_label = equip_slot_label(item.inventory_type);
    {
        let state_rc = get_sim_state(lua);
        let mut state = state_rc.borrow_mut();
        if let Some(td) = state.tooltips.get_mut(&tooltip_id) {
            td.lines.clear();
            td.lines.push(TooltipLine {
                left_text: item.name.to_string(),
                left_color: (nr, ng, nb),
                right_text: None,
                right_color: (1.0, 1.0, 1.0),
                wrap: false,
            });
            td.lines.push(TooltipLine {
                left_text: format!("Item Level {}", item.item_level),
                left_color: (1.0, 0.82, 0.0),
                right_text: None,
                right_color: (1.0, 1.0, 1.0),
                wrap: false,
            });
            if item.inventory_type > 0 && !slot_label.is_empty() {
                td.lines.push(TooltipLine {
                    left_text: slot_label.to_string(),
                    left_color: (1.0, 1.0, 1.0),
                    right_text: None,
                    right_color: (1.0, 1.0, 1.0),
                    wrap: false,
                });
            }
        }
        state.set_frame_visible(tooltip_id, true);
    }
    Ok(())
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

/// Public entry point for SetUnit on tooltip frames (called from widget_model.rs).
pub fn set_unit_for_tooltip(
    lua: &mlua::Lua,
    tooltip_id: u64,
    args: mlua::MultiValue,
) -> mlua::Result<Value> {
    let unit = match args.into_iter().next() {
        Some(Value::String(s)) => s.to_string_lossy().to_string(),
        _ => return Ok(Value::Boolean(false)),
    };
    let populated = populate_unit_tooltip(lua, tooltip_id, &unit)?;
    Ok(Value::Boolean(populated))
}

fn populate_unit_tooltip(lua: &mlua::Lua, tooltip_id: u64, unit: &str) -> mlua::Result<bool> {
    let state_rc = get_sim_state(lua);
    let mut state = state_rc.borrow_mut();
    let unit_info = resolve_unit_tooltip_info(&state, unit);
    let Some(info) = unit_info else {
        return Ok(false);
    };
    if let Some(td) = state.tooltips.get_mut(&tooltip_id) {
        td.lines.clear();
        td.lines.push(TooltipLine {
            left_text: info.name,
            left_color: info.class_color,
            right_text: None,
            right_color: (1.0, 1.0, 1.0),
            wrap: false,
        });
        td.lines.push(TooltipLine {
            left_text: format!("Level {} {}", info.level, info.race),
            left_color: (1.0, 1.0, 1.0),
            right_text: None,
            right_color: (1.0, 1.0, 1.0),
            wrap: false,
        });
    }
    state.set_frame_visible(tooltip_id, true);
    Ok(true)
}

struct UnitTooltipInfo {
    name: String,
    level: i32,
    race: String,
    class_color: (f32, f32, f32),
}

fn resolve_unit_tooltip_info(
    state: &crate::lua_api::state::SimState,
    unit: &str,
) -> Option<UnitTooltipInfo> {
    match unit {
        "player" => {
            let p = &state.player;
            let class_color = class_color_rgb(p.class_index);
            let race = crate::lua_api::state::RACE_DATA
                .get(p.race_index)
                .map(|(name, _, _)| name.to_string())
                .unwrap_or_else(|| "Unknown".to_string());
            Some(UnitTooltipInfo {
                name: p.name.clone(),
                level: p.level,
                race,
                class_color,
            })
        }
        "target" => {
            let t = state.current_target.as_ref()?;
            let class_color = class_color_rgb(t.class_index);
            Some(UnitTooltipInfo {
                name: t.name.clone(),
                level: t.level,
                race: t.creature_type.clone(),
                class_color,
            })
        }
        _ => None,
    }
}

fn class_color_rgb(class_index: i32) -> (f32, f32, f32) {
    match class_index {
        1 => (0.78, 0.61, 0.43),  // Warrior
        2 => (0.96, 0.55, 0.73),  // Paladin
        3 => (0.67, 0.83, 0.45),  // Hunter
        4 => (1.0, 0.96, 0.41),   // Rogue
        5 => (1.0, 1.0, 1.0),     // Priest
        6 => (0.77, 0.12, 0.23),  // Death Knight
        7 => (0.0, 0.44, 0.87),   // Shaman
        8 => (0.25, 0.78, 0.92),  // Mage
        9 => (0.53, 0.53, 0.93),  // Warlock
        10 => (0.0, 1.0, 0.6),    // Monk
        11 => (1.0, 0.49, 0.04),  // Druid
        12 => (0.64, 0.19, 0.79), // Demon Hunter
        13 => (0.2, 0.58, 0.5),   // Evoker
        _ => (1.0, 1.0, 1.0),
    }
}

fn parse_item_id_from_hyperlink(link: &str) -> Option<u32> {
    let start = link.find("item:")?;
    let after = &link[start + 5..];
    let end = after
        .find(|c: char| c == ':' || c == '|')
        .unwrap_or(after.len());
    after[..end].parse::<u32>().ok()
}

fn quality_color_rgb(quality: u8) -> (f32, f32, f32) {
    match quality {
        0 => (0.62, 0.62, 0.62),
        1 => (1.0, 1.0, 1.0),
        2 => (0.12, 1.0, 0.0),
        3 => (0.0, 0.44, 0.87),
        4 => (0.64, 0.21, 0.93),
        5 => (1.0, 0.5, 0.0),
        6 => (0.9, 0.8, 0.5),
        7 => (0.0, 0.8, 1.0),
        _ => (1.0, 1.0, 1.0),
    }
}

fn equip_slot_label(inv_type: u8) -> &'static str {
    match inv_type {
        1 => "Head",
        2 => "Neck",
        3 => "Shoulder",
        4 => "Shirt",
        5 | 20 => "Chest",
        6 => "Waist",
        7 => "Legs",
        8 => "Feet",
        9 => "Wrist",
        10 => "Hands",
        11 => "Finger",
        12 => "Trinket",
        13 => "One-Hand",
        14 => "Off Hand",
        15 => "Ranged",
        16 => "Back",
        17 => "Two-Hand",
        21 => "Main Hand",
        22 => "Off Hand",
        23 => "Held In Off-hand",
        25 => "Thrown",
        26 => "Ranged",
        _ => "",
    }
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

fn lookup_aura_from_args(
    lua: &mlua::Lua,
    args: &mlua::MultiValue,
) -> Option<crate::lua_api::game_data::AuraInfo> {
    let mut iter = args.iter();
    let _unit = iter.next(); // unit string (e.g. "player")
    let index_or_id = match iter.next()? {
        Value::Integer(n) => *n as i32,
        Value::Number(n) => *n as i32,
        _ => return None,
    };
    let state_rc = get_sim_state(lua);
    let state = state_rc.borrow();
    // Try as 1-based index first
    if index_or_id >= 1 {
        if let Some(aura) = state.player.buffs.get((index_or_id - 1) as usize) {
            return Some(aura.clone());
        }
    }
    // Try as aura instance ID
    state
        .player
        .buffs
        .iter()
        .find(|a| a.aura_instance_id == index_or_id)
        .cloned()
}

fn populate_aura_tooltip(
    lua: &mlua::Lua,
    tooltip_id: u64,
    aura: &crate::lua_api::game_data::AuraInfo,
) -> mlua::Result<()> {
    let state_rc = get_sim_state(lua);
    let mut state = state_rc.borrow_mut();
    if let Some(td) = state.tooltips.get_mut(&tooltip_id) {
        td.lines.clear();
        td.lines.push(TooltipLine {
            left_text: aura.name.clone(),
            left_color: (1.0, 1.0, 1.0),
            right_text: None,
            right_color: (1.0, 1.0, 1.0),
            wrap: false,
        });
        if aura.duration > 0.0 {
            td.lines.push(TooltipLine {
                left_text: format_aura_duration(aura.duration),
                left_color: (1.0, 1.0, 1.0),
                right_text: None,
                right_color: (1.0, 1.0, 1.0),
                wrap: false,
            });
        }
    }
    state.set_frame_visible(tooltip_id, true);
    Ok(())
}

fn format_aura_duration(seconds: f64) -> String {
    let secs = seconds as u64;
    if secs >= 3600 {
        let hours = secs / 3600;
        let mins = (secs % 3600) / 60;
        if mins > 0 {
            format!("{hours} hr {mins} min")
        } else {
            format!("{hours} hr")
        }
    } else if secs >= 60 {
        let mins = secs / 60;
        let remaining = secs % 60;
        if remaining > 0 {
            format!("{mins} min {remaining} sec")
        } else {
            format!("{mins} min")
        }
    } else {
        format!("{secs} sec")
    }
}

fn add_tooltip_query_methods<M: mlua::UserDataMethods<FrameRef>>(methods: &mut M) {
    methods.add_method("GetUnit", |_, _this, ()| {
        Ok::<(Option<String>, Option<String>), mlua::Error>((None, None))
    });
    methods.add_method("GetSpell", |_, _this, ()| {
        Ok::<(Option<String>, Option<i32>), mlua::Error>((None, None))
    });
    methods.add_method("GetItem", |_, _this, ()| {
        Ok::<(Option<String>, Option<String>), mlua::Error>((None, None))
    });
    methods.add_method("AddTexture", |_, _this, _texture: String| Ok(()));
    add_tooltip_minwidth_methods(methods);
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
