//! UIDropDownMenu_* globals and registration helpers.

use super::helpers::{
    get_frame_field, get_global, parse_frame_strata, set_frame_field, set_global_num,
    set_global_raw, table_get_str,
};
use crate::lua_api::methods::{
    borrow_state, borrow_state_mut, call_function_state, create_string, create_table,
    extract_frame_id, frame_ref, get_or_create_frame_fields, table_set,
};
use crate::lua_bridge::FromStack;
use rilua::vm::state::LuaState;
use rilua::{LuaApiMut, LuaResult, Val};

const BUTTON_TEXT_CHILD_KEYS: [&str; 3] = ["Text", "text", "ButtonText"];

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

fn set_dropdown_frame_value(state: &mut LuaState, frame_id: u64, field_name: &str, value: Val) {
    let fields = get_or_create_frame_fields(state, frame_id);
    table_set(state, fields, field_name, value);
    if let Ok(frame) = frame_ref(state, frame_id) {
        table_set(state, frame, field_name, value);
    }
}

// ---------------------------------------------------------------------------
// UIDropDownMenu_Initialize
// ---------------------------------------------------------------------------

/// `UIDropDownMenu_Initialize(frame, initFn, displayMode, level, menuList)`
pub fn ui_dropdown_menu_initialize(state: &mut LuaState) -> LuaResult<u32> {
    let frame: Val = FromStack::from_stack(state, 1)?;
    let init_fn: Val = FromStack::from_stack(state, 2)?;
    let display_mode: Option<String> = FromStack::from_stack(state, 3)?;
    let level_val: Option<f64> = FromStack::from_stack(state, 4)?;
    let menu_list: Val = FromStack::from_stack(state, 5)?;
    let level = level_val.unwrap_or(1.0) as i32;

    if let Some(id) = extract_frame_id(state, frame) {
        if !matches!(init_fn, Val::Nil) {
            set_dropdown_frame_value(state, id, "initialize", init_fn);
        }
        if let Some(display_mode) = display_mode {
            let display_mode_val = create_string(state, &display_mode);
            set_dropdown_frame_value(state, id, "displayMode", display_mode_val);
        }
        set_dropdown_frame_value(state, id, "menuList", menu_list);
        set_dropdown_frame_value(state, id, "dropdownLevel", Val::Num(level as f64));
        reset_dropdown_lists(state)?;
    }

    set_global_raw(state, "UIDROPDOWNMENU_INIT_MENU", frame);
    set_global_num(state, "UIDROPDOWNMENU_MENU_LEVEL", level as f64);
    if matches!(init_fn, Val::Function(_)) {
        let _ = call_function_state(state, init_fn, &[frame, Val::Num(level as f64), menu_list])?;
    }
    Ok(0)
}

fn reset_dropdown_lists(state: &mut LuaState) -> LuaResult<()> {
    for level in 1..=3 {
        let list_name = format!("DropDownList{}", level);
        let list_val = get_global(state, &list_name);
        let Some(list_id) = extract_frame_id(state, list_val) else {
            continue;
        };

        {
            let fields = get_or_create_frame_fields(state, list_id);
            table_set(state, fields, "numButtons", Val::Num(0.0));
            table_set(state, fields, "maxWidth", Val::Num(0.0));
            table_set(state, fields, "shouldRefresh", Val::Bool(false));
        }

        {
            let button_ids: Vec<u64> = (1..=8)
                .filter_map(|button| {
                    let button_name = format!("DropDownList{}Button{}", level, button);
                    let button_val = get_global(state, &button_name);
                    extract_frame_id(state, button_val)
                })
                .collect();

            let mut sim = borrow_state_mut(state)?;
            sim.set_frame_visible(list_id, false);
            for button in 1..=8 {
                if let Some(button_id) = button_ids.get((button - 1) as usize).copied() {
                    sim.set_frame_visible(button_id, false);
                }
            }
        }
    }
    Ok(())
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
    let btn_id = find_or_create_dropdown_button(state, list_id, &btn_name, new_index)?;

    copy_info_to_button_fields(state, btn_id, info);
    apply_button_text_from_info(state, btn_id, info)?;
    Ok(0)
}

fn find_or_create_dropdown_button(
    state: &mut LuaState,
    list_id: u64,
    btn_name: &str,
    index: i32,
) -> LuaResult<u64> {
    let btn_val = get_global(state, btn_name);
    if let Some(btn_id) = extract_frame_id(state, btn_val) {
        return Ok(btn_id);
    }

    let btn_id = super::create_frame_instance(
        state,
        crate::widget::WidgetType::Button,
        "Button",
        Some(btn_name.to_string()),
        Some(list_id),
        true,
        Some(index),
    )?;
    super::apply_runtime_template_chain(state, btn_id, Some("UIDropDownMenuButtonTemplate"), true)?;
    super::replay_runtime_template_parent_links(
        state,
        btn_id,
        Some("UIDropDownMenuButtonTemplate"),
    )?;
    super::dropdown_children::ensure_dropdown_button_children(state, btn_id, btn_name)?;
    set_global_num(state, "UIDROPDOWNMENU_MAXBUTTONS", index as f64);
    Ok(btn_id)
}

fn increment_list_button_count(state: &mut LuaState, list_id: u64) -> i32 {
    let list_fields = get_or_create_frame_fields(state, list_id);
    let num_buttons = match table_get_str(state, list_fields, "numButtons") {
        Val::Num(n) => n as i32,
        _ => 0,
    };
    let new_index = num_buttons + 1;
    table_set(state, list_fields, "numButtons", Val::Num(new_index as f64));
    if let Ok(list_frame) = frame_ref(state, list_id) {
        table_set(state, list_frame, "numButtons", Val::Num(new_index as f64));
    }
    new_index
}

fn copy_info_to_button_fields(state: &mut LuaState, btn_id: u64, info: Val) {
    let Val::Table(info_ref) = info else { return };
    let btn_fields = get_or_create_frame_fields(state, btn_id);
    copy_info_array_entries(state, btn_fields, info_ref);
    copy_info_named_entries(state, btn_id, info_ref);
}

fn copy_info_array_entries(
    state: &mut LuaState,
    btn_fields: Val,
    info_ref: rilua::vm::gc::arena::GcRef<rilua::vm::table::Table>,
) {
    let array_pairs = array_info_pairs(state, info_ref);
    let Val::Table(fields_ref) = btn_fields else {
        return;
    };
    if let Some(t) = state.gc.tables.get_mut(fields_ref) {
        for (key, value) in array_pairs {
            let _ = t.raw_set(key, value, &state.gc.string_arena);
        }
    }
    state.gc.barrier_back(fields_ref);
}

fn copy_info_named_entries(
    state: &mut LuaState,
    btn_id: u64,
    info_ref: rilua::vm::gc::arena::GcRef<rilua::vm::table::Table>,
) {
    let Some(btn_frame) = frame_ref(state, btn_id).ok() else {
        return;
    };
    let named_pairs = named_info_pairs(state, info_ref);
    for (key_name, value) in named_pairs {
        table_set(state, btn_frame, &key_name, value);
    }
}

fn array_info_pairs(
    state: &LuaState,
    info_ref: rilua::vm::gc::arena::GcRef<rilua::vm::table::Table>,
) -> Vec<(Val, Val)> {
    state
        .gc
        .tables
        .get(info_ref)
        .map(|t| {
            t.array_slice()
                .iter()
                .enumerate()
                .filter(|(_, value)| !matches!(value, Val::Nil))
                .map(|(index, value)| (Val::Num((index + 1) as f64), *value))
                .collect()
        })
        .unwrap_or_default()
}

fn named_info_pairs(
    state: &LuaState,
    info_ref: rilua::vm::gc::arena::GcRef<rilua::vm::table::Table>,
) -> Vec<(String, Val)> {
    state
        .gc
        .tables
        .get(info_ref)
        .map(|t| {
            t.hash_entries()
                .into_iter()
                .filter_map(|(key, value)| match key {
                    Val::Str(str_ref) => state
                        .gc
                        .string_arena
                        .get(str_ref)
                        .map(|s| String::from_utf8_lossy(s.data()).into_owned())
                        .map(|name| (name, value)),
                    _ => None,
                })
                .collect()
        })
        .unwrap_or_default()
}

fn get_info_text_string(state: &mut LuaState, info: Val) -> Option<String> {
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
        return None;
    };
    let text_bytes = state
        .gc
        .string_arena
        .get(text_str_ref)
        .map(|s| s.data().to_vec())
        .unwrap_or_default();
    Some(String::from_utf8_lossy(&text_bytes).into_owned())
}

fn set_button_text_on_widget(sim: &mut crate::lua_api::SimState, btn_id: u64, text_string: &str) {
    let stripped = crate::render::strip_wow_markup(text_string);
    if let Some(f) = sim.widgets.get_mut_visual(btn_id) {
        f.text_stripped = Some(stripped.clone());
        f.text = Some(text_string.to_owned());
    }
    let text_child = sim.widgets.get(btn_id).and_then(|f| {
        BUTTON_TEXT_CHILD_KEYS
            .iter()
            .find_map(|key| f.children_keys.get(*key).copied())
    });
    if let Some(tc_id) = text_child {
        if let Some(tc) = sim.widgets.get_mut_visual(tc_id) {
            tc.text_stripped = Some(stripped);
            tc.text = Some(text_string.to_owned());
        }
    }
}

fn apply_button_text_from_info(state: &mut LuaState, btn_id: u64, info: Val) -> LuaResult<()> {
    let Some(text_string) = get_info_text_string(state, info) else {
        return Ok(());
    };
    let mut sim = borrow_state_mut(state)?;
    set_button_text_on_widget(&mut sim, btn_id, &text_string);
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
    set_dropdown_enabled(state, frame, true)?;
    Ok(0)
}

/// `UIDropDownMenu_DisableDropDown(frame)`
pub fn ui_dropdown_menu_disable(state: &mut LuaState) -> LuaResult<u32> {
    let frame: Val = FromStack::from_stack(state, 1)?;
    set_dropdown_enabled(state, frame, false)?;
    Ok(0)
}

fn set_dropdown_enabled(state: &mut LuaState, frame: Val, enabled: bool) -> LuaResult<()> {
    if let Some(id) = extract_frame_id(state, frame) {
        let mut sim = borrow_state_mut(state)?;
        if let Some(f) = sim.widgets.get_mut_visual(id) {
            f.attributes.insert(
                "__dropdown_enabled".to_string(),
                crate::widget::AttributeValue::Boolean(enabled),
            );
        }
    }
    Ok(())
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
        let next_visible = sim.widgets.get(id).is_some_and(|f| !f.visible);
        sim.set_frame_visible(id, next_visible);
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
        set_dropdown_frame_value(state, id, "xOffset", Val::Num(x_offset));
        set_dropdown_frame_value(state, id, "yOffset", Val::Num(y_offset));
        let point_val = create_string(state, &point);
        set_dropdown_frame_value(state, id, "point", point_val);
        set_dropdown_frame_value(state, id, "relativeTo", relative_to);
        if let Some(rp) = relative_point {
            let rp_val = create_string(state, &rp);
            set_dropdown_frame_value(state, id, "relativePoint", rp_val);
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
    add_placeholder_button(_state)
}

/// `UIDropDownMenu_AddSpace(level?)` — no-op stub
///
/// TODO: same as AddSeparator
pub fn ui_dropdown_menu_add_space(_state: &mut LuaState) -> LuaResult<u32> {
    add_placeholder_button(_state)
}

fn add_placeholder_button(state: &mut LuaState) -> LuaResult<u32> {
    let level_val: Option<f64> = FromStack::from_stack(state, 1)?;
    let level = level_val.unwrap_or(1.0) as i32;
    let list_name = format!("DropDownList{}", level);
    let list_val = get_global(state, &list_name);
    let Some(list_id) = extract_frame_id(state, list_val) else {
        return Ok(0);
    };
    let _ = increment_list_button_count(state, list_id);
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
        set_dropdown_frame_value(state, id, "initialize", init_fn);
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
// Registration
// ---------------------------------------------------------------------------

pub fn register_dropdown_mutators(lua: &mut rilua::Lua) -> LuaResult<()> {
    register_dropdown_basics(lua)?;
    register_dropdown_enable_nav(lua)
}

fn register_dropdown_basics(lua: &mut rilua::Lua) -> LuaResult<()> {
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
    Ok(())
}

fn register_dropdown_enable_nav(lua: &mut rilua::Lua) -> LuaResult<()> {
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
    Ok(())
}

pub fn register_dropdown_selections(lua: &mut rilua::Lua) -> LuaResult<()> {
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
    Ok(())
}

pub fn register_dropdown_queries(lua: &mut rilua::Lua) -> LuaResult<()> {
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
