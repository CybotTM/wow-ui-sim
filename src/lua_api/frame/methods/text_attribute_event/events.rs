//! Event registration, script handlers, and hlist data structure helpers.

use crate::lua_api::methods::{borrow_state, borrow_state_mut, frame_id_from_stack, val_to_string};
use crate::lua_api::script_helpers::{
    get_script as get_rilua_script, remove_script as remove_rilua_script,
    set_script as set_rilua_script,
};
use crate::lua_bridge::stack_val;
use crate::widget::WidgetType;
use rilua::vm::gc::arena::GcRef;
use rilua::vm::state::LuaState;
use rilua::vm::table::Table;
use rilua::{LuaApiMut, LuaResult, Val, runtime_error};

// ── Event registration ───────────────────────────────────────────────────────

pub(super) fn register_event(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id_from_stack(state, 1)?;
    let Some(event) = val_to_string(state, stack_val(state, 2)) else {
        return Err(runtime_error("RegisterEvent: event name required"));
    };
    let newly_registered = insert_registered_event_checked(state, id, &event)?;
    if newly_registered {
        rilua_hlist_register_individual(state, id, &event)?;
    }
    let restricted = crate::event::is_restricted_event(&event);
    state.push(Val::Bool(newly_registered && !restricted));
    Ok(1)
}

fn insert_registered_event_checked(state: &mut LuaState, id: u64, event: &str) -> LuaResult<bool> {
    mutate_registered_event_checked(state, id, event, RegisteredEventOp::Insert)
}

pub(super) fn register_unit_event(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id_from_stack(state, 1)?;
    let Some(event) = val_to_string(state, stack_val(state, 2)) else {
        state.push(Val::Bool(false));
        return Ok(1);
    };
    // Unit args at 3+ are intentionally ignored (unit event filtering not implemented)
    let newly_registered = {
        let mut sim = borrow_state_mut(state)?;
        sim.widgets
            .get_mut(id)
            .map(|f| f.registered_events.insert(event.clone()))
            .unwrap_or(false)
    };
    if newly_registered {
        rilua_hlist_register_individual(state, id, &event)?;
    }
    state.push(Val::Bool(newly_registered));
    Ok(1)
}

pub(super) fn unregister_event(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id_from_stack(state, 1)?;
    let Some(event) = val_to_string(state, stack_val(state, 2)) else {
        state.push(Val::Bool(false));
        return Ok(1);
    };
    let was_registered = remove_registered_event_checked(state, id, &event)?;
    if was_registered {
        rilua_hlist_unregister_individual(state, id, &event)?;
    }
    state.push(Val::Bool(was_registered));
    Ok(1)
}

fn remove_registered_event_checked(state: &mut LuaState, id: u64, event: &str) -> LuaResult<bool> {
    mutate_registered_event_checked(state, id, event, RegisteredEventOp::Remove)
}

enum RegisteredEventOp {
    Insert,
    Remove,
}

fn mutate_registered_event_checked(
    state: &mut LuaState,
    id: u64,
    event: &str,
    op: RegisteredEventOp,
) -> LuaResult<bool> {
    ensure_registerable_event(state, id, event)?;
    let mut sim = borrow_state_mut(state)?;
    Ok(sim
        .widgets
        .get_mut(id)
        .map(|frame| match op {
            RegisteredEventOp::Insert => frame.registered_events.insert(event.to_string()),
            RegisteredEventOp::Remove => frame.registered_events.remove(event),
        })
        .unwrap_or(false))
}

fn ensure_registerable_event(state: &mut LuaState, id: u64, event: &str) -> LuaResult<()> {
    if crate::event::is_registerable_event(event) {
        return Ok(());
    }

    let sim = borrow_state(state)?;
    let frame_name = sim
        .widgets
        .get(id)
        .and_then(|frame| frame.name.clone())
        .unwrap_or_else(|| "Frame".to_string());
    Err(runtime_error(format!(
        "{}:RegisterEvent(): {}:RegisterEvent(): Attempt to register unknown event \"{}\"",
        frame_name, frame_name, event
    )))
}

pub(super) fn unregister_all_events(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id_from_stack(state, 1)?;
    {
        let mut sim = borrow_state_mut(state)?;
        if let Some(frame) = sim.widgets.get_mut(id) {
            frame.registered_events.clear();
            frame.register_all_events = false;
        }
    }
    rilua_hlist_unregister_all(state, id)?;
    Ok(0)
}

pub(super) fn register_all_events(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id_from_stack(state, 1)?;
    {
        let mut sim = borrow_state_mut(state)?;
        if let Some(frame) = sim.widgets.get_mut(id) {
            frame.register_all_events = true;
        }
    }
    rilua_hlist_register_all(state, id)?;
    Ok(0)
}

pub(super) fn is_event_registered(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id_from_stack(state, 1)?;
    let event = val_to_string(state, stack_val(state, 2)).unwrap_or_default();
    let sim = borrow_state(state)?;
    let registered = sim
        .widgets
        .get(id)
        .map(|frame| frame_has_registered_event(frame, &event))
        .unwrap_or(false);
    drop(sim);
    state.push(Val::Bool(registered));
    state.push(Val::Nil);
    Ok(2)
}

fn frame_has_registered_event(frame: &crate::widget::Frame, event: &str) -> bool {
    frame.registered_events.contains(event)
}

pub(super) fn register_event_callback(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id_from_stack(state, 1)?;
    let Some(event) = val_to_string(state, stack_val(state, 2)) else {
        state.push(Val::Bool(false));
        return Ok(1);
    };
    if !crate::event::is_callback_event(&event) {
        return Err(runtime_error(format!(
            "Frame:RegisterEventCallback(): Attempt to register unknown event \"{}\"",
            event
        )));
    }
    let mut sim = borrow_state_mut(state)?;
    if let Some(f) = sim.widgets.get_mut(id) {
        f.registered_events.insert(event.clone());
    }
    let restricted = crate::event::is_restricted_event(&event);
    drop(sim);
    state.push(Val::Bool(!restricted));
    Ok(1)
}

pub(super) fn register_unit_event_callback(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id_from_stack(state, 1)?;
    let Some(event) = val_to_string(state, stack_val(state, 2)) else {
        state.push(Val::Bool(false));
        return Ok(1);
    };
    let callback = stack_val(state, 3);
    if !matches!(callback, Val::Function(_)) {
        state.push(Val::Bool(false));
        return Ok(1);
    }
    let unit_filter = val_to_string(state, stack_val(state, 4));
    let mut sim = borrow_state_mut(state)?;
    if let Some(f) = sim.widgets.get_mut(id) {
        f.registered_events.insert(event.clone());
    }
    drop(sim);
    rilua_hlist_register_individual(state, id, &event)?;
    super::callbacks::register_unit_callback(state, id, &event, callback, unit_filter.as_deref())?;
    let restricted = crate::event::is_restricted_event(&event);
    state.push(Val::Bool(!restricted));
    Ok(1)
}

pub(super) fn set_propagate_keyboard_input(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id_from_stack(state, 1)?;
    // TODO: combat lockdown check
    let propagate = matches!(stack_val(state, 2), Val::Bool(b) if b);
    let mut sim = borrow_state_mut(state)?;
    if let Some(f) = sim.widgets.get_mut(id) {
        f.propagate_keyboard_input = propagate;
    }
    Ok(0)
}

pub(super) fn get_propagate_keyboard_input(state: &mut LuaState) -> LuaResult<u32> {
    let id = frame_id_from_stack(state, 1)?;
    let sim = borrow_state(state)?;
    let val = sim
        .widgets
        .get(id)
        .map(|f| f.propagate_keyboard_input)
        .unwrap_or(false);
    drop(sim);
    state.push(Val::Bool(val));
    Ok(1)
}

// ── Script handlers ──────────────────────────────────────────────────────────

pub(super) fn set_script(state: &mut LuaState) -> LuaResult<u32> {
    let frame_id = frame_id_from_stack(state, 1)?;
    let handler_name = val_to_string(state, stack_val(state, 2))
        .ok_or_else(|| runtime_error("SetScript: handler name required"))?;
    let handler = stack_val(state, 3);
    ensure_script_supported(state, frame_id, &handler_name)?;
    if matches!(handler, Val::Nil) {
        remove_rilua_script(state, frame_id, &handler_name);
    } else {
        if !matches!(handler, Val::Function(_)) {
            return Err(runtime_error(format!(
                "SetScript: handler for '{handler_name}' must be a function or nil"
            )));
        }
        set_rilua_script(state, frame_id, &handler_name, handler);
    }
    Ok(0)
}

pub(super) fn get_script(state: &mut LuaState) -> LuaResult<u32> {
    let frame_id = frame_id_from_stack(state, 1)?;
    let handler_name = val_to_string(state, stack_val(state, 2))
        .ok_or_else(|| runtime_error("GetScript: handler name required"))?;
    let handler = get_rilua_script(state, frame_id, &handler_name).unwrap_or(Val::Nil);
    state.push(handler);
    Ok(1)
}

pub(super) fn has_script(state: &mut LuaState) -> LuaResult<u32> {
    let frame_id = frame_id_from_stack(state, 1)?;
    let handler_name = val_to_string(state, stack_val(state, 2))
        .ok_or_else(|| runtime_error("HasScript: handler name required"))?;
    let has_script = if is_animation_script_container(state, frame_id) {
        get_rilua_script(state, frame_id, &handler_name).is_some()
    } else {
        script_supported(state, frame_id, &handler_name)
    };
    state.push(Val::Bool(has_script));
    Ok(1)
}

pub(super) fn hook_script(state: &mut LuaState) -> LuaResult<u32> {
    let frame_id = frame_id_from_stack(state, 1)?;
    let handler_name = val_to_string(state, stack_val(state, 2))
        .ok_or_else(|| runtime_error("HookScript: handler name required"))?;
    ensure_script_supported(state, frame_id, &handler_name)?;
    let hook = stack_val(state, 3);
    if !matches!(hook, Val::Function(_)) {
        return Err(runtime_error(format!(
            "HookScript: hook for '{handler_name}' must be a function"
        )));
    }
    let old = get_rilua_script(state, frame_id, &handler_name).unwrap_or(Val::Nil);
    let chained = build_hooked_script(state, old, hook)?;
    set_rilua_script(state, frame_id, &handler_name, chained);
    state.push(Val::Bool(true));
    Ok(1)
}

fn build_hooked_script(state: &mut LuaState, old: Val, hook: Val) -> LuaResult<Val> {
    let func = state.load(
        r#"
        local old, hook = ...
        if old == nil then
            return hook
        end
        return function(...)
            old(...)
            hook(...)
        end
    "#,
    )?;
    let call_base = state.top;
    state.ensure_stack(call_base + 4);
    state.stack_set(call_base, Val::Function(func.gc_ref()));
    state.stack_set(call_base + 1, old);
    state.stack_set(call_base + 2, hook);
    state.top = call_base + 3;
    state.call_function(call_base, 1)?;
    let result = state.stack_get(call_base);
    state.top = call_base;
    Ok(result)
}

fn ensure_script_supported(state: &LuaState, frame_id: u64, handler_name: &str) -> LuaResult<()> {
    if script_supported(state, frame_id, handler_name) {
        return Ok(());
    }
    Err(runtime_error(format!(
        "invalid script handler '{handler_name}'"
    )))
}

fn script_supported(state: &LuaState, frame_id: u64, handler_name: &str) -> bool {
    let Ok(sim) = borrow_state(state) else {
        return false;
    };
    let Some(widget_type) = sim.widgets.get(frame_id).map(|frame| frame.widget_type) else {
        return false;
    };
    script_supported_for_widget(widget_type, handler_name)
}

fn script_supported_for_widget(widget_type: WidgetType, handler_name: &str) -> bool {
    if is_common_script_handler(handler_name)
        || is_keyboard_script_handler(handler_name)
        || is_hyperlink_script_handler(handler_name)
    {
        return true;
    }

    match handler_name {
        "OnClick" | "PreClick" | "PostClick" => {
            matches!(widget_type, WidgetType::Button | WidgetType::CheckButton)
        }
        "OnDoubleClick" => true,
        "OnEnable" | "OnDisable" => supports_enable_disable_script(widget_type),
        "OnTextChanged"
        | "OnTextSet"
        | "OnEditFocusGained"
        | "OnEditFocusLost"
        | "OnInputLanguageChanged" => widget_type == WidgetType::EditBox,
        "OnCooldownDone" => matches!(widget_type, WidgetType::Cooldown),
        "OnValueChanged" => matches!(widget_type, WidgetType::Slider | WidgetType::StatusBar),
        "OnVerticalScroll" | "OnScrollRangeChanged" => {
            matches!(widget_type, WidgetType::ScrollFrame | WidgetType::EditBox)
        }
        "OnColorSelect" => matches!(widget_type, WidgetType::ColorSelect),
        "OnTooltipCleared"
        | "OnTooltipSetSpell"
        | "OnTooltipSetItem"
        | "OnTooltipSetUnit"
        | "OnTooltipSetFramestack" => widget_type == WidgetType::GameTooltip,
        _ => false,
    }
}

fn supports_enable_disable_script(widget_type: WidgetType) -> bool {
    matches!(
        widget_type,
        WidgetType::Button
            | WidgetType::CheckButton
            | WidgetType::EditBox
            | WidgetType::Slider
            | WidgetType::ScrollFrame
    )
}

fn is_common_script_handler(handler_name: &str) -> bool {
    matches!(
        handler_name,
        "OnLoad"
            | "OnEvent"
            | "OnUpdate"
            | "OnShow"
            | "OnHide"
            | "OnEnter"
            | "OnLeave"
            | "OnMouseDown"
            | "OnMouseUp"
            | "OnMouseWheel"
            | "OnDragStart"
            | "OnDragStop"
            | "OnReceiveDrag"
            | "OnSizeChanged"
            | "OnAttributeChanged"
            | "OnPlay"
            | "OnFinished"
            | "OnStop"
            | "OnLoop"
            | "OnPause"
    )
}

fn is_keyboard_script_handler(handler_name: &str) -> bool {
    matches!(
        handler_name,
        "OnEnterPressed"
            | "OnEscapePressed"
            | "OnTabPressed"
            | "OnSpacePressed"
            | "OnChar"
            | "OnKeyDown"
            | "OnKeyUp"
    )
}

fn is_hyperlink_script_handler(handler_name: &str) -> bool {
    matches!(
        handler_name,
        "OnHyperlinkClick" | "OnHyperlinkEnter" | "OnHyperlinkLeave"
    )
}

fn is_animation_script_container(state: &LuaState, frame_id: u64) -> bool {
    let Ok(sim) = borrow_state(state) else {
        return false;
    };
    let Some(object_type_name) = sim
        .widgets
        .get(frame_id)
        .and_then(|frame| frame.object_type_name.as_deref())
    else {
        return false;
    };
    matches!(
        object_type_name,
        "AnimationGroup"
            | "Alpha"
            | "Translation"
            | "Scale"
            | "Rotation"
            | "LineTranslation"
            | "LineScale"
            | "Path"
            | "FlipBook"
            | "VertexColor"
            | "Animation"
    )
}

// ── hlist helpers ────────────────────────────────────────────────────────────

/// Insert `id` into the per-event hlist stored in registry["__event_individual"][event].
pub(super) fn rilua_hlist_register_individual(
    state: &mut LuaState,
    id: u64,
    event: &str,
) -> LuaResult<()> {
    let individual_val = crate::lua_api::methods::registry_get(state, "__event_individual");
    let Val::Table(individual) = individual_val else {
        return Ok(());
    };
    let event_key = state.gc.intern_string(event.as_bytes());
    let existing = state
        .gc
        .tables
        .get(individual)
        .map(|t| t.get_str(event_key, &state.gc.string_arena));
    let event_tbl = match existing {
        Some(Val::Table(t)) => t,
        _ => {
            let new_tbl = state.gc.alloc_table(Table::new());
            if let Some(t) = state.gc.tables.get_mut(individual) {
                let _ = t.raw_set(
                    Val::Str(event_key),
                    Val::Table(new_tbl),
                    &state.gc.string_arena,
                );
            }
            state.gc.barrier_back(individual);
            new_tbl
        }
    };
    rilua_hlist_insert(state, event_tbl, id)
}

/// Remove `id` from the per-event hlist.
pub(super) fn rilua_hlist_unregister_individual(
    state: &mut LuaState,
    id: u64,
    event: &str,
) -> LuaResult<()> {
    let individual_val = crate::lua_api::methods::registry_get(state, "__event_individual");
    let Val::Table(individual) = individual_val else {
        return Ok(());
    };
    let event_key = state.gc.intern_string(event.as_bytes());
    let existing = state
        .gc
        .tables
        .get(individual)
        .map(|t| t.get_str(event_key, &state.gc.string_arena));
    if let Some(Val::Table(event_tbl)) = existing {
        rilua_hlist_remove(state, event_tbl, id)?;
    }
    Ok(())
}

/// Insert `id` into the all-events hlist stored in registry["__event_all"].
pub(super) fn rilua_hlist_register_all(state: &mut LuaState, id: u64) -> LuaResult<()> {
    let all_val = crate::lua_api::methods::registry_get(state, "__event_all");
    if let Val::Table(all_tbl) = all_val {
        rilua_hlist_insert(state, all_tbl, id)?;
    }
    Ok(())
}

/// Remove `id` from all individual event hlists and the all-events hlist.
pub(super) fn rilua_hlist_unregister_all(state: &mut LuaState, id: u64) -> LuaResult<()> {
    remove_from_individual_hlists(state, id)?;
    remove_from_all_hlist(state, id)
}

fn remove_from_individual_hlists(state: &mut LuaState, id: u64) -> LuaResult<()> {
    let individual_val = crate::lua_api::methods::registry_get(state, "__event_individual");
    let Val::Table(individual) = individual_val else {
        return Ok(());
    };
    let sub_tables: Vec<GcRef<Table>> = state
        .gc
        .tables
        .get(individual)
        .map(|t| {
            t.hash_entries()
                .into_iter()
                .filter_map(|(_, v)| if let Val::Table(t) = v { Some(t) } else { None })
                .collect()
        })
        .unwrap_or_default();
    for event_tbl in sub_tables {
        rilua_hlist_remove(state, event_tbl, id)?;
    }
    Ok(())
}

fn remove_from_all_hlist(state: &mut LuaState, id: u64) -> LuaResult<()> {
    let all_val = crate::lua_api::methods::registry_get(state, "__event_all");
    if let Val::Table(all_tbl) = all_val {
        rilua_hlist_remove(state, all_tbl, id)?;
    }
    Ok(())
}

/// hlist insert: append id to array, record index in "_s" sub-table.
pub(super) fn rilua_hlist_insert(
    state: &mut LuaState,
    tbl: GcRef<Table>,
    id: u64,
) -> LuaResult<()> {
    let set = rilua_hlist_set(state, tbl);
    let already = state
        .gc
        .tables
        .get(set)
        .map(|t| t.get_int(id as i64) != Val::Nil)
        .unwrap_or(false);
    if already {
        return Ok(());
    }
    let n = state.gc.tables.get(tbl).map(|t| t.array_len()).unwrap_or(0) + 1;
    if let Some(t) = state.gc.tables.get_mut(tbl) {
        let _ = t.raw_set(
            Val::Num(n as f64),
            Val::Num(id as f64),
            &state.gc.string_arena,
        );
    }
    state.gc.barrier_back(tbl);
    if let Some(s) = state.gc.tables.get_mut(set) {
        let _ = s.raw_set(
            Val::Num(id as f64),
            Val::Num(n as f64),
            &state.gc.string_arena,
        );
    }
    state.gc.barrier_back(set);
    Ok(())
}

/// hlist remove: swap-remove to keep array dense.
pub(super) fn rilua_hlist_remove(
    state: &mut LuaState,
    tbl: GcRef<Table>,
    id: u64,
) -> LuaResult<()> {
    let set = rilua_hlist_set(state, tbl);
    let idx = find_hlist_index(state, set, id);
    let Some(idx) = idx else {
        return Ok(());
    };
    let n = state.gc.tables.get(tbl).map(|t| t.array_len()).unwrap_or(0);
    if idx != n {
        swap_last_to_slot(state, tbl, set, idx, n);
    }
    clear_hlist_tail(state, tbl, set, n, id);
    Ok(())
}

fn find_hlist_index(state: &LuaState, set: GcRef<Table>, id: u64) -> Option<usize> {
    state
        .gc
        .tables
        .get(set)
        .and_then(|t| match t.get_int(id as i64) {
            Val::Num(n) if n > 0.0 => Some(n as usize),
            _ => None,
        })
}

fn swap_last_to_slot(
    state: &mut LuaState,
    tbl: GcRef<Table>,
    set: GcRef<Table>,
    idx: usize,
    n: usize,
) {
    let last_id = state
        .gc
        .tables
        .get(tbl)
        .and_then(|t| match t.get_int(n as i64) {
            Val::Num(lid) => Some(lid as u64),
            _ => None,
        });
    let Some(lid) = last_id else {
        return;
    };
    if let Some(t) = state.gc.tables.get_mut(tbl) {
        let _ = t.raw_set(
            Val::Num(idx as f64),
            Val::Num(lid as f64),
            &state.gc.string_arena,
        );
    }
    state.gc.barrier_back(tbl);
    if let Some(s) = state.gc.tables.get_mut(set) {
        let _ = s.raw_set(
            Val::Num(lid as f64),
            Val::Num(idx as f64),
            &state.gc.string_arena,
        );
    }
    state.gc.barrier_back(set);
}

fn clear_hlist_tail(state: &mut LuaState, tbl: GcRef<Table>, set: GcRef<Table>, n: usize, id: u64) {
    if let Some(t) = state.gc.tables.get_mut(tbl) {
        let _ = t.raw_set(Val::Num(n as f64), Val::Nil, &state.gc.string_arena);
    }
    state.gc.barrier_back(tbl);
    if let Some(s) = state.gc.tables.get_mut(set) {
        let _ = s.raw_set(Val::Num(id as f64), Val::Nil, &state.gc.string_arena);
    }
    state.gc.barrier_back(set);
}

/// Get or create the "_s" set sub-table of a hlist table.
pub(super) fn rilua_hlist_set(state: &mut LuaState, tbl: GcRef<Table>) -> GcRef<Table> {
    let key_ref = state.gc.intern_string(b"_s");
    let existing = state
        .gc
        .tables
        .get(tbl)
        .map(|t| t.get_str(key_ref, &state.gc.string_arena));
    if let Some(Val::Table(s)) = existing {
        return s;
    }
    let new_set = state.gc.alloc_table(Table::new());
    if let Some(t) = state.gc.tables.get_mut(tbl) {
        let _ = t.raw_set(
            Val::Str(key_ref),
            Val::Table(new_set),
            &state.gc.string_arena,
        );
    }
    state.gc.barrier_back(tbl);
    new_set
}

#[cfg(test)]
mod tests {
    use super::frame_has_registered_event;
    use crate::widget::{Frame, WidgetType};

    #[test]
    fn frame_has_registered_event_uses_set_membership() {
        let mut frame = Frame::new(WidgetType::Frame, Some("EventFrame".to_string()), None);
        frame.registered_events.insert("PLAYER_LOGIN".to_string());

        assert!(frame_has_registered_event(&frame, "PLAYER_LOGIN"));
        assert!(!frame_has_registered_event(&frame, "PLAYER_LOGOUT"));
    }
}
