//! GameTooltip widget methods: SetOwner, AddLine, AddDoubleLine, tooltip queries, etc.

use super::super::handle::FrameRef;
use super::methods_helpers::get_mixin_override;
use crate::lua_api::frame::handle::{extract_frame_id, frame_ref, get_sim_state};
use crate::lua_api::tooltip::TooltipLine;
use crate::widget::{Anchor, AnchorPoint};
use mlua::Value;

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
    methods.add_method("SetSpellByID", |_, _this, _spell_id: i32| Ok(()));
    methods.add_method("SetItemByID", |_, _this, _item_id: i32| Ok(()));
    methods.add_method("SetHyperlink", |_, _this, _link: String| Ok(()));
    methods.add_method("SetUnitBuff", |_, _this, _args: mlua::MultiValue| Ok(()));
    methods.add_method("SetUnitDebuff", |_, _this, _args: mlua::MultiValue| Ok(()));
    methods.add_method("SetUnitAura", |_, _this, _args: mlua::MultiValue| Ok(()));
    methods.add_method(
        "SetUnitBuffByAuraInstanceID",
        |_, _this, _args: mlua::MultiValue| Ok(()),
    );
    methods.add_method(
        "SetUnitDebuffByAuraInstanceID",
        |_, _this, _args: mlua::MultiValue| Ok(()),
    );
    methods.add_method("AddAtlas", |_, _this, _args: mlua::MultiValue| Ok(()));
    methods.add_method("AddFontStrings", |_, _this, _args: mlua::MultiValue| Ok(()));
    methods.add_method("CopyTooltip", |_, _this, _: mlua::Variadic<Value>| Ok(()));
    methods.add_method("GetCustomLineSpacing", |_, _this, ()| Ok(0.0f64));
    methods.add_method("GetLeftLine", |_, _this, ()| Ok(Value::Nil));
    methods.add_method("GetRightLine", |_, _this, ()| Ok(Value::Nil));
    methods.add_method(
        "SetAllowShowWithNoLines",
        |_, _this, _: mlua::Variadic<Value>| Ok(()),
    );
    methods.add_method("SetAnchorType", |_, _this, _: mlua::Variadic<Value>| Ok(()));
    methods.add_method(
        "SetCustomLineSpacing",
        |_, _this, _: mlua::Variadic<Value>| Ok(()),
    );
    methods.add_method(
        "SetCustomWordWrapMinWidth",
        |_, _this, _: mlua::Variadic<Value>| Ok(()),
    );
    methods.add_method("SetFrameStack", |_, _this, _: mlua::Variadic<Value>| Ok(()));
    methods.add_method(
        "SetObjectTooltipPosition",
        |_, _this, _: mlua::Variadic<Value>| Ok(()),
    );
    methods.add_method(
        "SetShrinkToFitWrapped",
        |_, _this, _: mlua::Variadic<Value>| Ok(()),
    );

    methods.add_method("NumLines", |lua, this, ()| {
        let state_rc = get_sim_state(lua);
        let state = state_rc.borrow();
        let count = state
            .tooltips
            .get(&this.0)
            .map(|td| td.lines.len())
            .unwrap_or(0);
        Ok(count as i32)
    });
    methods.add_method("GetNumLines", |_, _this, ()| Ok(0_i32));
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
        let id = this.0;
        if let Some((func, self_val)) = get_mixin_override(lua, id, "SetPadding") {
            let mut call_args = vec![self_val];
            call_args.extend(args);
            return func.call::<()>(mlua::MultiValue::from_iter(call_args));
        }
        let padding = args
            .into_iter()
            .next()
            .and_then(|v| match v {
                Value::Number(n) => Some(n as f32),
                Value::Integer(n) => Some(n as f32),
                _ => None,
            })
            .unwrap_or(0.0);
        let state_rc = get_sim_state(lua);
        let mut state = state_rc.borrow_mut();
        if let Some(td) = state.tooltips.get_mut(&id) {
            td.padding = padding;
        }
        Ok(())
    });

    methods.add_method("GetPadding", |lua, this, ()| {
        let id = this.0;
        if let Some((func, self_val)) = get_mixin_override(lua, id, "GetPadding") {
            return func.call::<mlua::MultiValue>(self_val);
        }
        let state_rc = get_sim_state(lua);
        let state = state_rc.borrow();
        let p = state
            .tooltips
            .get(&id)
            .map(|td| td.padding as f64)
            .unwrap_or(0.0);
        Ok(mlua::MultiValue::from_iter(std::iter::once(Value::Number(
            p,
        ))))
    });

    methods.add_method("ClearPadding", |lua, this, ()| {
        let state_rc = get_sim_state(lua);
        let mut state = state_rc.borrow_mut();
        if let Some(td) = state.tooltips.get_mut(&this.0) {
            td.padding = 0.0;
        }
        Ok(())
    });
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
