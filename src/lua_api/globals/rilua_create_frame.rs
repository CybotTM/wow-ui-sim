//! rilua RustFn equivalents for global functions from create_frame.rs,
//! global_frames.rs, and dropdown_api.rs.
//!
//! Each public function is a `rilua::RustFn` compatible signature:
//!   `fn foo(state: &mut LuaState) -> LuaResult<u32>`
//! Args start at index 1 (no self).
//!
//! `register_all` registers all globals on a rilua Lua state.

use crate::lua_api::LoaderEnv;
use crate::lua_api::rilua_methods::{
    borrow_lua, borrow_state, borrow_state_mut, create_string, create_table, extract_frame_id,
    frame_ref, get_or_create_frame_fields, registry_get, state_handle, table_get, table_set,
};
use crate::lua_api::rilua_script_helpers::protected_lua_pcall_state;
use crate::lua_bridge::FromStack;
use crate::widget::WidgetType;
use rilua::vm::state::LuaState;
use rilua::{LuaApiMut, LuaResult, Val};
use std::cell::RefCell;
use std::rc::Rc;

// ---------------------------------------------------------------------------
// CreateFrame
// ---------------------------------------------------------------------------

pub fn create_frame(state: &mut LuaState) -> LuaResult<u32> {
    let frame_type: String = FromStack::from_stack(state, 1)?;
    let name: Option<String> = FromStack::from_stack(state, 2)?;
    let parent_val: Val = FromStack::from_stack(state, 3)?;
    let inherits: Option<String> = FromStack::from_stack(state, 4)?;
    let id: Option<f64> = FromStack::from_stack(state, 5)?;

    let widget_type = WidgetType::from_str(&frame_type)
        .ok_or_else(|| rilua::runtime_error(format!("unknown frame type '{frame_type}'")))?;
    let (parent_id, parent_explicit) = resolve_parent_id(state, parent_val)?;

    let frame_id = crate::lua_api::globals::create_frame::create_frame_instance(
        state,
        widget_type,
        &frame_type,
        name,
        if parent_id == 0 {
            None
        } else {
            Some(parent_id)
        },
        parent_explicit,
        id.map(|n| n as i32),
    )?;
    let fire_on_load = {
        let sim = borrow_state(state)?;
        sim.suppress_runtime_on_load_depth == 0
    };
    apply_runtime_template_chain(state, frame_id, inherits.as_deref(), fire_on_load)?;
    let frame_val = frame_ref(state, frame_id)?;
    state.push(frame_val);
    Ok(1)
}

fn resolve_parent_id(state: &mut LuaState, parent_val: Val) -> LuaResult<(u64, bool)> {
    let parent_explicit = !matches!(parent_val, Val::Nil);
    let parent_id = if parent_explicit {
        extract_frame_id(state, parent_val)
            .ok_or_else(|| rilua::runtime_error("CreateFrame parent must be a frame or nil"))?
    } else {
        let sim = borrow_state(state)?;
        sim.widgets.get_id_by_name("UIParent").unwrap_or_default()
    };
    Ok((parent_id, parent_explicit))
}

// ---------------------------------------------------------------------------
// Global frames registration
// ---------------------------------------------------------------------------

pub fn register_global_frames(lua: &mut rilua::Lua) -> LuaResult<()> {
    let state = lua.state_mut();
    let named_frames = {
        let sim = borrow_state(state)?;
        sim.widgets
            .named_frames()
            .map(|(id, name)| (id, name.clone()))
            .collect::<Vec<_>>()
    };
    for (id, name) in named_frames {
        let frame_val = frame_ref(state, id)?;
        set_global_raw(state, &name, frame_val);
    }
    Ok(())
}

// ---------------------------------------------------------------------------
// UIDropDownMenu constants
// ---------------------------------------------------------------------------

/// Register UIDROPDOWNMENU_* global constants.
pub fn register_dropdown_constants(lua: &mut rilua::Lua) -> LuaResult<()> {
    let state = lua.state_mut();
    set_global_num(state, "UIDROPDOWNMENU_MAXBUTTONS", 1.0);
    set_global_num(state, "UIDROPDOWNMENU_MAXLEVELS", 3.0);
    set_global_num(state, "UIDROPDOWNMENU_BUTTON_HEIGHT", 16.0);
    set_global_num(state, "UIDROPDOWNMENU_BORDER_HEIGHT", 15.0);
    set_global_raw(state, "UIDROPDOWNMENU_OPEN_MENU", Val::Nil);
    set_global_raw(state, "UIDROPDOWNMENU_INIT_MENU", Val::Nil);
    set_global_num(state, "UIDROPDOWNMENU_MENU_LEVEL", 1.0);
    set_global_raw(state, "UIDROPDOWNMENU_MENU_VALUE", Val::Nil);
    set_global_num(state, "UIDROPDOWNMENU_SHOW_TIME", 2.0);
    set_global_raw(state, "UIDROPDOWNMENU_DEFAULT_TEXT_HEIGHT", Val::Nil);
    set_global_num(state, "UIDROPDOWNMENU_DEFAULT_WIDTH_PADDING", 25.0);
    let open_menus = create_table(state);
    set_global_raw(state, "OPEN_DROPDOWNMENUS", open_menus);
    Ok(())
}

// ---------------------------------------------------------------------------
// UIDropDownMenu_CreateInfo
// ---------------------------------------------------------------------------

/// `UIDropDownMenu_CreateInfo()` → returns an empty table.
pub fn ui_dropdown_menu_create_info(state: &mut LuaState) -> LuaResult<u32> {
    let t = create_table(state);
    state.push(t);
    Ok(1)
}

// ---------------------------------------------------------------------------
// UIDropDownMenu_Initialize
// ---------------------------------------------------------------------------

/// `UIDropDownMenu_Initialize(frame, initFn, displayMode, level, menuList)`
///
/// Stores the init function on the frame's fields.
/// TODO: call initFn(frame, level, menuList) once function calling from RustFn is supported.
pub fn ui_dropdown_menu_initialize(state: &mut LuaState) -> LuaResult<u32> {
    let frame: Val = FromStack::from_stack(state, 1)?;
    let init_fn: Val = FromStack::from_stack(state, 2)?;

    if let Some(id) = extract_frame_id(state, frame) {
        let fields = get_or_create_frame_fields(state, id);
        if !matches!(init_fn, Val::Nil) {
            table_set(state, fields, "initialize", init_fn);
        }
    }

    let frame2: Val = FromStack::from_stack(state, 1)?;
    set_global_raw(state, "UIDROPDOWNMENU_INIT_MENU", frame2);

    // TODO: call init_fn(frame, level, menu_list)
    Ok(0)
}

// ---------------------------------------------------------------------------
// UIDropDownMenu_AddButton
// ---------------------------------------------------------------------------

/// `UIDropDownMenu_AddButton(info, level)`
///
/// Adds a button entry to the appropriate DropDownList.
pub fn ui_dropdown_menu_add_button(state: &mut LuaState) -> LuaResult<u32> {
    let info: Val = FromStack::from_stack(state, 1)?;
    let level_val: Option<f64> = FromStack::from_stack(state, 2)?;
    let level = level_val.unwrap_or(1.0) as i32;

    let list_name = format!("DropDownList{}", level);
    let list_val = get_global(state, &list_name);
    let Some(list_id) = extract_frame_id(state, list_val) else {
        return Ok(0);
    };
    let new_index = increment_list_button_count(state, list_id);

    let btn_name = format!("DropDownList{}Button{}", level, new_index);
    let btn_val = get_global(state, &btn_name);
    let Some(btn_id) = extract_frame_id(state, btn_val) else {
        return Ok(0);
    };

    copy_info_to_button_fields(state, btn_id, info);
    apply_button_text_from_info(state, btn_id, info)?;
    Ok(0)
}

fn increment_list_button_count(state: &mut LuaState, list_id: u64) -> i32 {
    let list_fields = get_or_create_frame_fields(state, list_id);
    let num_buttons = match table_get_str(state, list_fields, "numButtons") {
        Val::Num(n) => n as i32,
        _ => 0,
    };
    let new_index = num_buttons + 1;
    table_set(state, list_fields, "numButtons", Val::Num(new_index as f64));
    new_index
}

fn copy_info_to_button_fields(state: &mut LuaState, btn_id: u64, info: Val) {
    let btn_fields = get_or_create_frame_fields(state, btn_id);
    let Val::Table(info_ref) = info else { return };
    let array_pairs: Vec<(Val, Val)> = state
        .gc
        .tables
        .get(info_ref)
        .map(|t| {
            t.array_slice()
                .iter()
                .enumerate()
                .filter(|(_, v)| !matches!(v, Val::Nil))
                .map(|(i, v)| (Val::Num((i + 1) as f64), *v))
                .collect()
        })
        .unwrap_or_default();
    let hash_pairs: Vec<(Val, Val)> = state
        .gc
        .tables
        .get(info_ref)
        .map(|t| t.hash_entries())
        .unwrap_or_default();
    if let Val::Table(fields_ref) = btn_fields {
        for (k, v) in array_pairs.into_iter().chain(hash_pairs) {
            if let Some(t) = state.gc.tables.get_mut(fields_ref) {
                let _ = t.raw_set(k, v, &state.gc.string_arena);
            }
        }
    }
}

fn apply_button_text_from_info(state: &mut LuaState, btn_id: u64, info: Val) -> LuaResult<()> {
    let text_key = state.gc.intern_string(b"text");
    let info_text = if let Val::Table(info_ref) = info {
        state
            .gc
            .tables
            .get(info_ref)
            .map(|t| t.get_str(text_key, &state.gc.string_arena))
            .unwrap_or(Val::Nil)
    } else {
        Val::Nil
    };
    let Val::Str(text_str_ref) = info_text else {
        return Ok(());
    };
    let text_bytes = state
        .gc
        .string_arena
        .get(text_str_ref)
        .map(|s| s.data().to_vec())
        .unwrap_or_default();
    let text_string = String::from_utf8_lossy(&text_bytes).into_owned();
    let mut sim = borrow_state_mut(state)?;
    let stripped = crate::render::strip_wow_markup(&text_string);
    if let Some(f) = sim.widgets.get_mut_visual(btn_id) {
        f.text_stripped = Some(stripped.clone());
        f.text = Some(text_string.clone());
    }
    let text_child = sim
        .widgets
        .get(btn_id)
        .and_then(|f| f.children_keys.get("Text").copied());
    if let Some(tc_id) = text_child {
        if let Some(tc) = sim.widgets.get_mut_visual(tc_id) {
            tc.text_stripped = Some(stripped);
            tc.text = Some(text_string);
        }
    }
    sim.set_frame_visible(btn_id, true);
    Ok(())
}

// ---------------------------------------------------------------------------
// UIDropDownMenu_SetWidth
// ---------------------------------------------------------------------------

/// `UIDropDownMenu_SetWidth(frame, width, padding?)`
pub fn ui_dropdown_menu_set_width(state: &mut LuaState) -> LuaResult<u32> {
    let frame: Val = FromStack::from_stack(state, 1)?;
    let width: f64 = FromStack::from_stack(state, 2)?;
    if let Some(id) = extract_frame_id(state, frame) {
        let mut sim = borrow_state_mut(state)?;
        if let Some(f) = sim.widgets.get_mut_visual(id) {
            f.width = width as f32;
        }
    }
    Ok(0)
}

// ---------------------------------------------------------------------------
// UIDropDownMenu_SetText / GetText
// ---------------------------------------------------------------------------

/// `UIDropDownMenu_SetText(frame, text?)`
pub fn ui_dropdown_menu_set_text(state: &mut LuaState) -> LuaResult<u32> {
    let frame: Val = FromStack::from_stack(state, 1)?;
    let text: Option<String> = FromStack::from_stack(state, 2)?;
    if let Some(id) = extract_frame_id(state, frame) {
        let mut sim = borrow_state_mut(state)?;
        if let Some(f) = sim.widgets.get_mut_visual(id) {
            f.text = text;
        }
    }
    Ok(0)
}

/// `UIDropDownMenu_GetText(frame)` → text or nil
pub fn ui_dropdown_menu_get_text(state: &mut LuaState) -> LuaResult<u32> {
    let frame: Val = FromStack::from_stack(state, 1)?;
    if let Some(id) = extract_frame_id(state, frame) {
        let text_owned: Option<String> = {
            let sim = borrow_state(state)?;
            sim.widgets.get(id).and_then(|f| f.text.clone())
        };
        if let Some(text) = text_owned {
            let val = create_string(state, &text);
            state.push(val);
            return Ok(1);
        }
    }
    state.push(Val::Nil);
    Ok(1)
}

// ---------------------------------------------------------------------------
// Selection setters/getters
// ---------------------------------------------------------------------------

/// `UIDropDownMenu_SetSelectedID(frame, id, useValue?)`
pub fn ui_dropdown_menu_set_selected_id(state: &mut LuaState) -> LuaResult<u32> {
    set_frame_field(state, "selectedID")
}

/// `UIDropDownMenu_GetSelectedID(frame)` → value or nil
pub fn ui_dropdown_menu_get_selected_id(state: &mut LuaState) -> LuaResult<u32> {
    get_frame_field(state, "selectedID")
}

/// `UIDropDownMenu_SetSelectedValue(frame, value, useValue?)`
pub fn ui_dropdown_menu_set_selected_value(state: &mut LuaState) -> LuaResult<u32> {
    set_frame_field(state, "selectedValue")
}

/// `UIDropDownMenu_GetSelectedValue(frame)` → value or nil
pub fn ui_dropdown_menu_get_selected_value(state: &mut LuaState) -> LuaResult<u32> {
    get_frame_field(state, "selectedValue")
}

/// `UIDropDownMenu_SetSelectedName(frame, name, useValue?)`
pub fn ui_dropdown_menu_set_selected_name(state: &mut LuaState) -> LuaResult<u32> {
    set_frame_field(state, "selectedName")
}

// ---------------------------------------------------------------------------
// Enable / Disable
// ---------------------------------------------------------------------------

/// `UIDropDownMenu_EnableDropDown(frame)`
pub fn ui_dropdown_menu_enable(state: &mut LuaState) -> LuaResult<u32> {
    let frame: Val = FromStack::from_stack(state, 1)?;
    if let Some(id) = extract_frame_id(state, frame) {
        let mut sim = borrow_state_mut(state)?;
        if let Some(f) = sim.widgets.get_mut_visual(id) {
            f.attributes.insert(
                "__dropdown_enabled".to_string(),
                crate::widget::AttributeValue::Boolean(true),
            );
        }
    }
    Ok(0)
}

/// `UIDropDownMenu_DisableDropDown(frame)`
pub fn ui_dropdown_menu_disable(state: &mut LuaState) -> LuaResult<u32> {
    let frame: Val = FromStack::from_stack(state, 1)?;
    if let Some(id) = extract_frame_id(state, frame) {
        let mut sim = borrow_state_mut(state)?;
        if let Some(f) = sim.widgets.get_mut_visual(id) {
            f.attributes.insert(
                "__dropdown_enabled".to_string(),
                crate::widget::AttributeValue::Boolean(false),
            );
        }
    }
    Ok(0)
}

// ---------------------------------------------------------------------------
// ToggleDropDownMenu / CloseDropDownMenus
// ---------------------------------------------------------------------------

/// `ToggleDropDownMenu(level, value, dropdownFrame, anchorName, xOffset, yOffset, menuList, button, autoHideDelay, displayMode)`
pub fn toggle_dropdown_menu(state: &mut LuaState) -> LuaResult<u32> {
    let level_val: Option<f64> = FromStack::from_stack(state, 1)?;
    let dropdown_frame: Val = FromStack::from_stack(state, 3)?;
    let level = level_val.unwrap_or(1.0) as i32;

    let list_name = format!("DropDownList{}", level);
    let list_val = get_global(state, &list_name);
    if let Some(id) = extract_frame_id(state, list_val) {
        let mut sim = borrow_state_mut(state)?;
        if let Some(f) = sim.widgets.get_mut_visual(id) {
            f.visible = !f.visible;
        }
    }
    set_global_raw(state, "UIDROPDOWNMENU_OPEN_MENU", dropdown_frame);
    Ok(0)
}

/// `CloseDropDownMenus(level?)`
pub fn close_dropdown_menus(state: &mut LuaState) -> LuaResult<u32> {
    let start_level: Option<f64> = FromStack::from_stack(state, 1)?;
    let start = start_level.unwrap_or(1.0) as i32;
    for lvl in start..=3 {
        let list_name = format!("DropDownList{}", lvl);
        let list_val = get_global(state, &list_name);
        if let Some(id) = extract_frame_id(state, list_val) {
            let mut sim = borrow_state_mut(state)?;
            sim.set_frame_visible(id, false);
        }
    }
    set_global_raw(state, "UIDROPDOWNMENU_OPEN_MENU", Val::Nil);
    Ok(0)
}

// ---------------------------------------------------------------------------
// UIDropDownMenu_SetAnchor / SetFrameStrata
// ---------------------------------------------------------------------------

/// `UIDropDownMenu_SetAnchor(dropdown, xOffset, yOffset, point, relativeTo, relativePoint)`
pub fn ui_dropdown_menu_set_anchor(state: &mut LuaState) -> LuaResult<u32> {
    let dropdown: Val = FromStack::from_stack(state, 1)?;
    let x_offset: f64 = FromStack::from_stack(state, 2)?;
    let y_offset: f64 = FromStack::from_stack(state, 3)?;
    let point: String = FromStack::from_stack(state, 4)?;
    let relative_to: Val = FromStack::from_stack(state, 5)?;
    let relative_point: Option<String> = FromStack::from_stack(state, 6)?;

    if let Some(id) = extract_frame_id(state, dropdown) {
        let fields = get_or_create_frame_fields(state, id);
        table_set(state, fields, "xOffset", Val::Num(x_offset));
        table_set(state, fields, "yOffset", Val::Num(y_offset));
        let point_val = create_string(state, &point);
        table_set(state, fields, "point", point_val);
        table_set(state, fields, "relativeTo", relative_to);
        if let Some(rp) = relative_point {
            let rp_val = create_string(state, &rp);
            table_set(state, fields, "relativePoint", rp_val);
        }
    }
    Ok(0)
}

/// `UIDropDownMenu_SetFrameStrata(frame, strata)`
pub fn ui_dropdown_menu_set_frame_strata(state: &mut LuaState) -> LuaResult<u32> {
    let frame: Val = FromStack::from_stack(state, 1)?;
    let strata: String = FromStack::from_stack(state, 2)?;
    if let Some(id) = extract_frame_id(state, frame) {
        let mut sim = borrow_state_mut(state)?;
        if let Some(f) = sim.widgets.get_mut_visual(id) {
            f.frame_strata = parse_frame_strata(&strata);
        }
    }
    Ok(0)
}

// ---------------------------------------------------------------------------
// UIDropDownMenu_AddSeparator / AddSpace
// ---------------------------------------------------------------------------

/// `UIDropDownMenu_AddSeparator(level?)` — no-op stub
///
/// TODO: build an info table and call `ui_dropdown_menu_add_button` once
/// cross-function calling from within a RustFn is supported.
pub fn ui_dropdown_menu_add_separator(_state: &mut LuaState) -> LuaResult<u32> {
    Ok(0)
}

/// `UIDropDownMenu_AddSpace(level?)` — no-op stub
///
/// TODO: same as AddSeparator
pub fn ui_dropdown_menu_add_space(_state: &mut LuaState) -> LuaResult<u32> {
    Ok(0)
}

// ---------------------------------------------------------------------------
// Query functions
// ---------------------------------------------------------------------------

/// `UIDropDownMenu_GetCurrentDropDown()` → UIDROPDOWNMENU_OPEN_MENU
pub fn ui_dropdown_menu_get_current_dropdown(state: &mut LuaState) -> LuaResult<u32> {
    let val = get_global(state, "UIDROPDOWNMENU_OPEN_MENU");
    state.push(val);
    Ok(1)
}

/// `UIDropDownMenu_IsOpen(frame?)` → bool
pub fn ui_dropdown_menu_is_open(state: &mut LuaState) -> LuaResult<u32> {
    let frame: Val = FromStack::from_stack(state, 1)?;
    let Some(target_id) = extract_frame_id(state, frame) else {
        state.push(Val::Bool(false));
        return Ok(1);
    };

    let open_menu = get_global(state, "UIDROPDOWNMENU_OPEN_MENU");
    let Some(open_id) = extract_frame_id(state, open_menu) else {
        state.push(Val::Bool(false));
        return Ok(1);
    };

    if open_id != target_id {
        state.push(Val::Bool(false));
        return Ok(1);
    }

    let list_val = get_global(state, "DropDownList1");
    let Some(list_id) = extract_frame_id(state, list_val) else {
        state.push(Val::Bool(false));
        return Ok(1);
    };

    let sim = borrow_state(state)?;
    let is_open = sim.widgets.get(list_id).is_some_and(|f| f.visible);
    drop(sim);
    state.push(Val::Bool(is_open));
    Ok(1)
}

// ---------------------------------------------------------------------------
// No-op / simple functions
// ---------------------------------------------------------------------------

/// `UIDropDownMenu_SetInitializeFunction(frame, initFn)`
pub fn ui_dropdown_menu_set_initialize_function(state: &mut LuaState) -> LuaResult<u32> {
    let frame: Val = FromStack::from_stack(state, 1)?;
    let init_fn: Val = FromStack::from_stack(state, 2)?;
    if let Some(id) = extract_frame_id(state, frame) {
        let fields = get_or_create_frame_fields(state, id);
        table_set(state, fields, "initialize", init_fn);
    }
    Ok(0)
}

/// `UIDropDownMenu_Refresh(frame, useValue?, level?)` — no-op stub
pub fn ui_dropdown_menu_refresh(_state: &mut LuaState) -> LuaResult<u32> {
    Ok(0)
}

/// `UIDropDownMenu_JustifyText(frame, justify)` — no-op stub
pub fn ui_dropdown_menu_justify_text(_state: &mut LuaState) -> LuaResult<u32> {
    Ok(0)
}

/// `UIDropDownMenu_HandleGlobalMouseEvent(button, event)` — no-op stub
pub fn ui_dropdown_menu_handle_global_mouse_event(_state: &mut LuaState) -> LuaResult<u32> {
    Ok(0)
}

// ---------------------------------------------------------------------------
// register_all
// ---------------------------------------------------------------------------

/// Register all globals from create_frame.rs, global_frames.rs, and dropdown_api.rs
/// onto the rilua Lua state.
pub fn register_all(lua: &mut rilua::Lua) -> LuaResult<()> {
    LuaApiMut::register_function(lua, "CreateFrame", create_frame)?;

    register_global_frames(lua)?;
    register_dropdown_constants(lua)?;

    LuaApiMut::register_function(
        lua,
        "UIDropDownMenu_CreateInfo",
        ui_dropdown_menu_create_info,
    )?;
    LuaApiMut::register_function(
        lua,
        "UIDropDownMenu_Initialize",
        ui_dropdown_menu_initialize,
    )?;
    LuaApiMut::register_function(lua, "UIDropDownMenu_AddButton", ui_dropdown_menu_add_button)?;
    LuaApiMut::register_function(lua, "UIDropDownMenu_SetWidth", ui_dropdown_menu_set_width)?;
    LuaApiMut::register_function(lua, "UIDropDownMenu_SetText", ui_dropdown_menu_set_text)?;
    LuaApiMut::register_function(lua, "UIDropDownMenu_GetText", ui_dropdown_menu_get_text)?;
    LuaApiMut::register_function(
        lua,
        "UIDropDownMenu_SetSelectedID",
        ui_dropdown_menu_set_selected_id,
    )?;
    LuaApiMut::register_function(
        lua,
        "UIDropDownMenu_GetSelectedID",
        ui_dropdown_menu_get_selected_id,
    )?;
    LuaApiMut::register_function(
        lua,
        "UIDropDownMenu_SetSelectedValue",
        ui_dropdown_menu_set_selected_value,
    )?;
    LuaApiMut::register_function(
        lua,
        "UIDropDownMenu_GetSelectedValue",
        ui_dropdown_menu_get_selected_value,
    )?;
    LuaApiMut::register_function(
        lua,
        "UIDropDownMenu_SetSelectedName",
        ui_dropdown_menu_set_selected_name,
    )?;
    LuaApiMut::register_function(
        lua,
        "UIDropDownMenu_EnableDropDown",
        ui_dropdown_menu_enable,
    )?;
    LuaApiMut::register_function(
        lua,
        "UIDropDownMenu_DisableDropDown",
        ui_dropdown_menu_disable,
    )?;
    LuaApiMut::register_function(lua, "ToggleDropDownMenu", toggle_dropdown_menu)?;
    LuaApiMut::register_function(lua, "CloseDropDownMenus", close_dropdown_menus)?;
    LuaApiMut::register_function(lua, "UIDropDownMenu_SetAnchor", ui_dropdown_menu_set_anchor)?;
    LuaApiMut::register_function(
        lua,
        "UIDropDownMenu_SetFrameStrata",
        ui_dropdown_menu_set_frame_strata,
    )?;
    LuaApiMut::register_function(
        lua,
        "UIDropDownMenu_AddSeparator",
        ui_dropdown_menu_add_separator,
    )?;
    LuaApiMut::register_function(lua, "UIDropDownMenu_AddSpace", ui_dropdown_menu_add_space)?;
    LuaApiMut::register_function(
        lua,
        "UIDropDownMenu_GetCurrentDropDown",
        ui_dropdown_menu_get_current_dropdown,
    )?;
    LuaApiMut::register_function(lua, "UIDropDownMenu_IsOpen", ui_dropdown_menu_is_open)?;
    LuaApiMut::register_function(
        lua,
        "UIDropDownMenu_SetInitializeFunction",
        ui_dropdown_menu_set_initialize_function,
    )?;
    LuaApiMut::register_function(lua, "UIDropDownMenu_Refresh", ui_dropdown_menu_refresh)?;
    LuaApiMut::register_function(
        lua,
        "UIDropDownMenu_JustifyText",
        ui_dropdown_menu_justify_text,
    )?;
    LuaApiMut::register_function(
        lua,
        "UIDropDownMenu_HandleGlobalMouseEvent",
        ui_dropdown_menu_handle_global_mouse_event,
    )?;

    Ok(())
}

// ---------------------------------------------------------------------------
// Private helpers
// ---------------------------------------------------------------------------

fn apply_runtime_template_chain(
    state: &mut LuaState,
    frame_id: u64,
    inherits: Option<&str>,
    fire_on_load: bool,
) -> LuaResult<()> {
    let Some(inherits) = inherits.filter(|value| !value.trim().is_empty()) else {
        return Ok(());
    };

    let chain = crate::xml::get_template_chain(inherits);
    if chain.is_empty() {
        return Ok(());
    }

    let state_rc = sim_state_rc(state)?;
    let frame_name = frame_lookup_name(state, frame_id);
    let template_parent_array = chain
        .iter()
        .find_map(|entry| entry.frame.parent_array.as_deref());
    let parent_id = borrow_state(state)?
        .widgets
        .get(frame_id)
        .and_then(|frame| frame.parent_id);
    if let Some(parent_array) = template_parent_array
        && let Some(parent_id) = parent_id
    {
        append_parent_array_entry(state, parent_id, parent_array, frame_id);
    }

    for entry in &chain {
        ensure_runtime_button_texture_slots(state, frame_id, &entry.frame)?;
        apply_frame_mixins(state, frame_id, entry.frame.combined_mixin().as_deref());
        apply_template_key_values(state, frame_id, entry.frame.all_key_values());
        if let Some(scripts) = entry.frame.scripts() {
            apply_template_scripts(state, frame_id, scripts)?;
        }
    }

    // The chain is base-to-derived. Install all parent-facing state first so
    // template child OnLoad/OnShow handlers can see derived key values and
    // mixin methods (for example ThreeSliceButtonTemplate children expect the
    // derived template's `atlasName` to already exist on the parent button).
    for entry in &chain {
        create_template_child_frames(
            state,
            &state_rc,
            frame_id,
            &frame_name,
            &frame_name,
            &entry.frame,
        )?;
    }

    apply_runtime_template_loader_effects(
        state,
        &frame_name,
        &frame_name,
        &crate::xml::FrameXml::default(),
        Some(inherits),
    )?;
    apply_runtime_template_direct_properties(&state_rc, frame_id, inherits, &frame_name);
    if fire_on_load {
        fire_frame_on_load(state, frame_id)?;
    }
    Ok(())
}

fn create_template_child_frames(
    state: &mut LuaState,
    state_rc: &Rc<RefCell<crate::lua_api::SimState>>,
    parent_id: u64,
    parent_name: &str,
    subst_parent: &str,
    frame: &crate::xml::FrameXml,
) -> LuaResult<()> {
    for child in frame.all_frame_elements() {
        create_template_child_frame(
            state,
            state_rc,
            parent_id,
            parent_name,
            subst_parent,
            &child,
        )?;
    }

    let Some(scroll_child) = frame.scroll_child() else {
        return Ok(());
    };

    let mut registered_scroll_child = false;
    for child in &scroll_child.children {
        let child_id = create_template_child_frame(
            state,
            state_rc,
            parent_id,
            parent_name,
            subst_parent,
            child,
        )?;
        if !registered_scroll_child && let Some(child_id) = child_id {
            let mut sim = borrow_state_mut(state)?;
            crate::lua_api::frame::methods::widget_scroll::assign_scroll_child(
                &mut sim, parent_id, child_id, false,
            );
            registered_scroll_child = true;
        }
    }

    Ok(())
}

fn create_template_child_frame(
    state: &mut LuaState,
    state_rc: &Rc<RefCell<crate::lua_api::SimState>>,
    parent_id: u64,
    _parent_name: &str,
    subst_parent: &str,
    child: &crate::xml::FrameElement,
) -> LuaResult<Option<u64>> {
    let Some((frame, widget_type_name, intrinsic)) = template_child_type(child) else {
        return Ok(None);
    };

    let child_name = template_child_name(frame.name.as_deref(), subst_parent);
    let child_id = crate::lua_api::globals::create_frame::create_frame_instance(
        state,
        WidgetType::from_str(widget_type_name).ok_or_else(|| {
            rilua::runtime_error(format!("unknown widget type '{widget_type_name}'"))
        })?,
        widget_type_name,
        Some(child_name.clone()),
        Some(parent_id),
        true,
        frame.xml_id,
    )?;

    let inherited_parent_key =
        resolve_inherited_string(frame, |template| template.parent_key.as_ref());
    if let Some(parent_key) = inherited_parent_key {
        crate::lua_api::globals::template::assign_parent_key(
            state,
            parent_id,
            &parent_key,
            child_id,
        )?;
    }
    if let Some(parent_array) =
        resolve_inherited_string(frame, |template| template.parent_array.as_ref())
    {
        append_parent_array_entry(state, parent_id, &parent_array, child_id);
    }

    let inherited_chain = build_child_inherits(intrinsic, frame.inherits.as_deref());
    if let Some(chain) = inherited_chain.as_deref() {
        apply_runtime_template_chain(state, child_id, Some(chain), false)?;
    }
    if let Some(intrinsic) = intrinsic {
        crate::lua_api::globals::template::set_intrinsic(state, child_id, intrinsic);
    }
    apply_frame_mixins(state, child_id, frame.combined_mixin().as_deref());
    apply_template_key_values(state, child_id, frame.all_key_values());
    if let Some(scripts) = frame.scripts() {
        apply_template_scripts(state, child_id, scripts)?;
    }

    let child_subst = if frame.name.is_some() {
        child_name.as_str()
    } else {
        subst_parent
    };
    create_template_child_frames(state, state_rc, child_id, &child_name, child_subst, frame)?;

    apply_runtime_child_direct_properties(state_rc, child_id, frame, &child_name);
    ensure_runtime_button_texture_slots(state, child_id, frame)?;
    apply_runtime_template_loader_effects(
        state,
        &child_name,
        child_subst,
        frame,
        inherited_chain.as_deref(),
    )?;
    fire_frame_on_load(state, child_id)?;
    Ok(Some(child_id))
}

fn apply_runtime_template_loader_effects(
    state: &mut LuaState,
    frame_name: &str,
    name_parent: &str,
    frame: &crate::xml::FrameXml,
    inherits: Option<&str>,
) -> LuaResult<()> {
    let loader_env = LoaderEnv::from_parts_active(borrow_lua(state)?, state_handle(state)?, state);
    let inherits = inherits.unwrap_or("");
    let mut timing = crate::loader::LoadTiming::default();

    for entry in crate::xml::get_template_chain(inherits) {
        crate::loader::xml_layer_batch::create_layer_children_batched_with_name_parent(
            &loader_env,
            &entry.frame,
            frame_name,
            name_parent,
            &mut timing,
        )
        .map_err(|error| rilua::runtime_error(error.to_string()))?;
    }
    crate::loader::xml_layer_batch::create_layer_children_batched_with_name_parent(
        &loader_env,
        frame,
        frame_name,
        name_parent,
        &mut timing,
    )
    .map_err(|error| rilua::runtime_error(error.to_string()))?;
    crate::loader::button::apply_button_textures(&loader_env, frame, frame_name, inherits)
        .map_err(|error| rilua::runtime_error(error.to_string()))?;
    crate::loader::button::apply_button_text(&loader_env, frame, frame_name, inherits)
        .map_err(|error| rilua::runtime_error(error.to_string()))?;
    crate::loader::xml_frame_extras::apply_animation_groups(
        &loader_env,
        frame,
        frame_name,
        inherits,
    )
    .map_err(|error| rilua::runtime_error(error.to_string()))?;
    crate::loader::xml_frame_extras::apply_bar_texture(&loader_env, frame, frame_name, inherits)
        .map_err(|error| rilua::runtime_error(error.to_string()))?;
    crate::loader::xml_frame_extras::init_action_bar_tables(&loader_env, frame, frame_name);
    Ok(())
}

fn ensure_runtime_button_texture_slots(
    state: &mut LuaState,
    frame_id: u64,
    frame: &crate::xml::FrameXml,
) -> LuaResult<()> {
    let is_button = {
        let sim = borrow_state(state)?;
        sim.widgets
            .get(frame_id)
            .map(|widget| {
                matches!(
                    widget.widget_type,
                    WidgetType::Button | WidgetType::CheckButton
                )
            })
            .unwrap_or(false)
    };
    if !is_button {
        return Ok(());
    }

    let slots = [
        ("NormalTexture", frame.normal_texture()),
        ("PushedTexture", frame.pushed_texture()),
        ("HighlightTexture", frame.highlight_texture()),
        ("DisabledTexture", frame.disabled_texture()),
    ];
    let mut sim = borrow_state_mut(state)?;
    for (key, texture) in slots {
        if texture.is_some() {
            crate::lua_api::frame::methods::methods_helpers::get_or_create_button_texture(
                &mut sim, frame_id, key,
            );
        }
    }
    Ok(())
}

fn apply_runtime_template_direct_properties(
    state: &Rc<RefCell<crate::lua_api::SimState>>,
    frame_id: u64,
    inherits: &str,
    frame_name: &str,
) {
    let frame = crate::xml::FrameXml::default();
    apply_runtime_child_direct_properties_with_inherits(
        state, frame_id, &frame, inherits, frame_name,
    );
}

fn apply_runtime_child_direct_properties(
    state: &Rc<RefCell<crate::lua_api::SimState>>,
    frame_id: u64,
    frame: &crate::xml::FrameXml,
    frame_name: &str,
) {
    let inherits = frame.inherits.as_deref().unwrap_or("");
    apply_runtime_child_direct_properties_with_inherits(
        state, frame_id, frame, inherits, frame_name,
    );
}

fn apply_runtime_child_direct_properties_with_inherits(
    state: &Rc<RefCell<crate::lua_api::SimState>>,
    frame_id: u64,
    frame: &crate::xml::FrameXml,
    inherits: &str,
    frame_name: &str,
) {
    crate::lua_api::globals::template::direct::apply_xml_size(state, frame_id, frame, inherits);
    crate::lua_api::globals::template::direct::apply_xml_anchors(
        state, frame_id, frame, inherits, frame_name,
    );
    crate::lua_api::globals::template::direct::apply_xml_hidden(state, frame_id, frame, inherits);
    crate::lua_api::globals::template::direct::apply_xml_clips_children(
        state, frame_id, frame, inherits,
    );
    crate::lua_api::globals::template::direct::apply_xml_set_all_points(
        state, frame_id, frame, inherits,
    );
    crate::lua_api::globals::template::direct::apply_xml_frame_level(
        state, frame_id, frame, inherits,
    );
    crate::lua_api::globals::template::direct::apply_xml_frame_strata(
        state, frame_id, frame, inherits,
    );
    crate::lua_api::globals::template::direct::apply_xml_protected(
        state, frame_id, frame, inherits,
    );
}

fn fire_frame_on_load(state: &mut LuaState, frame_id: u64) -> LuaResult<()> {
    let frame = frame_ref(state, frame_id)?;
    let intrinsic = table_get(state, frame, "OnLoad_Intrinsic");
    call_handler_with_frame(state, intrinsic, frame)?;
    if let Some(on_load) =
        crate::lua_api::rilua_script_helpers::get_script(state, frame_id, "OnLoad")
    {
        call_handler_with_frame(state, on_load, frame)?;
    }
    Ok(())
}

fn template_child_name(name: Option<&str>, subst_parent: &str) -> String {
    name.map(|name| name.replace("$parent", subst_parent))
        .unwrap_or_else(|| format!("__tpl_{}", crate::loader::helpers::rand_id()))
}

fn build_child_inherits(intrinsic: Option<&str>, inherits: Option<&str>) -> Option<String> {
    match (intrinsic, inherits.filter(|value| !value.trim().is_empty())) {
        (Some(base), Some(inherits)) => Some(format!("{base}, {inherits}")),
        (Some(base), None) => Some(base.to_string()),
        (None, Some(inherits)) => Some(inherits.to_string()),
        (None, None) => None,
    }
}

fn frame_lookup_name(state: &LuaState, frame_id: u64) -> String {
    borrow_state(state)
        .ok()
        .and_then(|sim| {
            sim.widgets
                .get(frame_id)
                .and_then(|frame| frame.name.clone())
        })
        .unwrap_or_else(|| format!("__frame_{frame_id}"))
}

fn sim_state_rc(state: &LuaState) -> LuaResult<Rc<RefCell<crate::lua_api::SimState>>> {
    state
        .app_data::<crate::lua_api::env::WowLuaAppData>()
        .map(|app| app.sim_state.clone())
        .ok_or_else(|| rilua::runtime_error("missing WowLuaAppData"))
}

fn template_child_type(
    child: &crate::xml::FrameElement,
) -> Option<(&crate::xml::FrameXml, &'static str, Option<&'static str>)> {
    let (frame, tag) = child.as_frame_data()?;
    match tag {
        "DropDownToggleButton" => Some((frame, "Button", Some("DropDownToggleButton"))),
        "EventButton" => Some((frame, "Button", Some("EventButton"))),
        _ => crate::xml::widget_type_for_tag(tag)
            .map(|(widget_type, intrinsic)| (frame, widget_type, intrinsic)),
    }
}

fn resolve_inherited_string(
    frame: &crate::xml::FrameXml,
    project: impl Fn(&crate::xml::FrameXml) -> Option<&String>,
) -> Option<String> {
    if let Some(value) = project(frame) {
        return Some(value.clone());
    }
    let inherits = frame.inherits.as_deref()?;
    crate::xml::get_template_chain(inherits)
        .into_iter()
        .find_map(|entry| project(&entry.frame).cloned())
}

fn append_parent_array_entry(state: &mut LuaState, parent_id: u64, key: &str, child_id: u64) {
    let Ok(parent) = frame_ref(state, parent_id) else {
        return;
    };
    let Ok(child) = frame_ref(state, child_id) else {
        return;
    };
    let array = match table_get(state, parent, key) {
        Val::Table(existing) => Val::Table(existing),
        _ => {
            let created = create_table(state);
            table_set(state, parent, key, created);
            created
        }
    };
    let Val::Table(array_ref) = array else {
        return;
    };
    let next_index = state
        .gc
        .tables
        .get(array_ref)
        .map(|table| table.array_slice().len() + 1)
        .unwrap_or(1);
    if let Some(table) = state.gc.tables.get_mut(array_ref) {
        let _ = table.raw_set(Val::Num(next_index as f64), child, &state.gc.string_arena);
    }
}

pub(crate) fn apply_frame_mixins(state: &mut LuaState, frame_id: u64, mixins: Option<&str>) {
    let Some(mixins) = mixins else {
        return;
    };

    for mixin_name in mixins
        .split(',')
        .map(str::trim)
        .filter(|name| !name.is_empty())
    {
        let mixin_val = resolve_global_path(state, mixin_name);
        copy_table_into_frame(state, frame_id, mixin_val);
    }
}

fn apply_template_key_values<'a>(
    state: &mut LuaState,
    frame_id: u64,
    key_values: impl Iterator<Item = &'a crate::xml::KeyValuesXml>,
) {
    let frame = frame_ref(state, frame_id).ok();
    let Some(Val::Table(frame_ref)) = frame else {
        return;
    };

    for key_block in key_values {
        for entry in &key_block.values {
            let value = template_key_value(state, &entry.value, entry.value_type.as_deref());
            let key = create_string(state, &entry.key);
            if let Some(table) = state.gc.tables.get_mut(frame_ref) {
                let _ = table.raw_set(key, value, &state.gc.string_arena);
            }
        }
    }
}

fn apply_template_scripts(
    state: &mut LuaState,
    frame_id: u64,
    scripts: &crate::xml::ScriptsXml,
) -> LuaResult<()> {
    let script_code = crate::loader::helpers::generate_scripts_code(scripts);
    if script_code.trim().is_empty() {
        return Ok(());
    }

    let chunk = format!("local frame = ...\n{script_code}");
    let func = LuaApiMut::load(state, &chunk)?;
    let frame = frame_ref(state, frame_id)?;
    match protected_lua_pcall_state(state, Val::Function(func.gc_ref()), &[frame]) {
        Ok(_) => {}
        Err(error) => return Err(rilua::runtime_error(error)),
    }
    Ok(())
}

fn template_key_value(state: &mut LuaState, value: &str, value_type: Option<&str>) -> Val {
    match value_type {
        Some("number") => value.parse::<f64>().map(Val::Num).unwrap_or(Val::Nil),
        Some("boolean") => Val::Bool(value.eq_ignore_ascii_case("true")),
        Some("global") => resolve_global_path(state, value),
        // Auto-detect numbers when type is not specified (WoW behavior)
        None if value.parse::<f64>().is_ok() => Val::Num(value.parse().unwrap()),
        _ => create_string(state, value),
    }
}

fn resolve_global_path(state: &mut LuaState, path: &str) -> Val {
    let current = resolve_table_path(state, Val::Table(state.global), path);
    if current != Val::Nil {
        return current;
    }
    let secureenv = registry_get(state, "__secureenv");
    resolve_table_path(state, secureenv, path)
}

fn resolve_table_path(state: &mut LuaState, root: Val, path: &str) -> Val {
    let mut current = root;
    for segment in path
        .split('.')
        .map(str::trim)
        .filter(|segment| !segment.is_empty())
    {
        let Val::Table(table_ref) = current else {
            return Val::Nil;
        };
        let key = state.gc.intern_string(segment.as_bytes());
        current = state
            .gc
            .tables
            .get(table_ref)
            .map(|table| table.get_str(key, &state.gc.string_arena))
            .unwrap_or(Val::Nil);
    }
    current
}

fn copy_table_into_frame(state: &mut LuaState, frame_id: u64, source: Val) {
    let Val::Table(source_ref) = source else {
        return;
    };
    let frame = frame_ref(state, frame_id).ok();
    let Some(Val::Table(frame_ref)) = frame else {
        return;
    };

    copy_table_entries_into_frame(state, frame_ref, source_ref);
    let index_key = state.gc.intern_string(b"__index");
    let index_table = state
        .gc
        .tables
        .get(source_ref)
        .and_then(|table| table.metatable())
        .and_then(|mt_ref| state.gc.tables.get(mt_ref))
        .map(|mt| mt.get_str(index_key, &state.gc.string_arena))
        .and_then(|value| match value {
            Val::Table(table_ref) => Some(table_ref),
            _ => None,
        });
    if let Some(index_ref) = index_table {
        copy_table_entries_into_frame(state, frame_ref, index_ref);
    }
}

fn copy_table_entries_into_frame(
    state: &mut LuaState,
    frame_ref: rilua::vm::gc::arena::GcRef<rilua::vm::table::Table>,
    source_ref: rilua::vm::gc::arena::GcRef<rilua::vm::table::Table>,
) {
    let array_values = state
        .gc
        .tables
        .get(source_ref)
        .map(|table| table.array_slice().to_vec())
        .unwrap_or_default();
    let hash_entries = state
        .gc
        .tables
        .get(source_ref)
        .map(|table| table.hash_entries())
        .unwrap_or_default();

    if let Some(fields_table) = state.gc.tables.get_mut(frame_ref) {
        for (index, value) in array_values.into_iter().enumerate() {
            let _ =
                fields_table.raw_set(Val::Num((index + 1) as f64), value, &state.gc.string_arena);
        }
        for (key, value) in hash_entries {
            let should_skip = match key {
                Val::Str(str_ref) => state
                    .gc
                    .string_arena
                    .get(str_ref)
                    .map(|name| {
                        matches!(
                            name.as_str(),
                            Some("RegisterCallback")
                                | Some("UnregisterCallback")
                                | Some("TriggerEvent")
                        )
                    })
                    .unwrap_or(false),
                _ => false,
            };
            if should_skip {
                continue;
            }
            let _ = fields_table.raw_set(key, value, &state.gc.string_arena);
        }
    }
}

fn call_handler_with_frame(state: &mut LuaState, handler: Val, frame: Val) -> LuaResult<()> {
    let Val::Function(_) = handler else {
        return Ok(());
    };
    match protected_lua_pcall_state(state, handler, &[frame]) {
        Ok(_) => Ok(()),
        Err(err) => Err(rilua::runtime_error(err)),
    }
}

/// Set a named field (arg 2) on the frame (arg 1)'s fields table.
fn set_frame_field(state: &mut LuaState, field_name: &str) -> LuaResult<u32> {
    let frame: Val = FromStack::from_stack(state, 1)?;
    let value: Val = FromStack::from_stack(state, 2)?;
    if let Some(id) = extract_frame_id(state, frame) {
        let fields = get_or_create_frame_fields(state, id);
        let key = create_string(state, field_name);
        if let Val::Table(fields_ref) = fields {
            if let Some(t) = state.gc.tables.get_mut(fields_ref) {
                let _ = t.raw_set(key, value, &state.gc.string_arena);
            }
        }
    }
    Ok(0)
}

/// Get a named field from the frame (arg 1)'s fields table; push result, return 1.
fn get_frame_field(state: &mut LuaState, field_name: &str) -> LuaResult<u32> {
    let frame: Val = FromStack::from_stack(state, 1)?;
    if let Some(id) = extract_frame_id(state, frame) {
        let fields_registry = registry_get(state, "__rilua_frame_fields");
        if let Val::Table(reg_ref) = fields_registry {
            let frame_fields = state
                .gc
                .tables
                .get(reg_ref)
                .map(|t| t.get_int(id as i64))
                .unwrap_or(Val::Nil);
            if let Val::Table(ff_ref) = frame_fields {
                let key_ref = state.gc.intern_string(field_name.as_bytes());
                let val = state
                    .gc
                    .tables
                    .get(ff_ref)
                    .map(|t| t.get_str(key_ref, &state.gc.string_arena))
                    .unwrap_or(Val::Nil);
                state.push(val);
                return Ok(1);
            }
        }
    }
    state.push(Val::Nil);
    Ok(1)
}

/// Get a string-keyed value from a `Val::Table`.
fn table_get_str(state: &mut LuaState, table: Val, key: &str) -> Val {
    let Val::Table(table_ref) = table else {
        return Val::Nil;
    };
    let key_ref = state.gc.intern_string(key.as_bytes());
    state
        .gc
        .tables
        .get(table_ref)
        .map(|t| t.get_str(key_ref, &state.gc.string_arena))
        .unwrap_or(Val::Nil)
}

/// Get a named global value from the Lua global table.
fn get_global(state: &mut LuaState, name: &str) -> Val {
    let key = state.gc.intern_string(name.as_bytes());
    state
        .gc
        .tables
        .get(state.global)
        .map(|g| g.get_str(key, &state.gc.string_arena))
        .unwrap_or(Val::Nil)
}

/// Set a named global to a numeric value.
fn set_global_num(state: &mut LuaState, name: &str, value: f64) {
    set_global_raw(state, name, Val::Num(value));
}

/// Set a named global to any Val.
fn set_global_raw(state: &mut LuaState, name: &str, value: Val) {
    let key = state.gc.intern_string(name.as_bytes());
    let global = state.global;
    if let Some(g) = state.gc.tables.get_mut(global) {
        let _ = g.raw_set(Val::Str(key), value, &state.gc.string_arena);
    }
}

fn parse_frame_strata(strata: &str) -> crate::widget::FrameStrata {
    match strata.to_uppercase().as_str() {
        "WORLD" | "BACKGROUND" => crate::widget::FrameStrata::Background,
        "LOW" => crate::widget::FrameStrata::Low,
        "MEDIUM" => crate::widget::FrameStrata::Medium,
        "HIGH" => crate::widget::FrameStrata::High,
        "DIALOG" => crate::widget::FrameStrata::Dialog,
        "FULLSCREEN" => crate::widget::FrameStrata::Fullscreen,
        "FULLSCREEN_DIALOG" => crate::widget::FrameStrata::FullscreenDialog,
        "TOOLTIP" => crate::widget::FrameStrata::Tooltip,
        _ => crate::widget::FrameStrata::Medium,
    }
}

#[cfg(test)]
mod tests {
    use crate::lua_api::WowLuaEnv;

    #[test]
    fn create_frame_registers_named_global_and_parent() {
        let env = WowLuaEnv::new().expect("env");

        env.exec(
            r#"
            local child = CreateFrame("Frame", "RiluaCreateFrameChild", UIParent)
            assert(child ~= nil, "CreateFrame should return a frame")
            assert(type(child) == "table", "CreateFrame should expose frames as tables")
            assert(RiluaCreateFrameChild == child, "named frame should be global")
            assert(child:GetParent() == UIParent, "parent should be assigned")
        "#,
        )
        .expect("CreateFrame should create a named child frame");

        let parent_name: Option<String> = env
            .eval("local p = RiluaCreateFrameChild:GetParent(); return p and p:GetName()")
            .expect("eval parent name");
        assert_eq!(parent_name.as_deref(), Some("UIParent"));
    }
}
