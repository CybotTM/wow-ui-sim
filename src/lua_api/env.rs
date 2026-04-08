//! WoW Lua environment.

use super::builtin_frames::create_builtin_frames;
use super::state::{AddonInfo, AddonRuntimeMetrics, PendingTimer, SimState};
use crate::Result;
use crate::render::font::WowFontSystem;
use crate::screen::ScreenKind;
use mlua::{Lua, MultiValue, Value};
use std::cell::RefCell;
use std::rc::Rc;
use std::sync::atomic::{AtomicU64, Ordering};
use std::time::Instant;

static NEXT_TIMER_ID: AtomicU64 = AtomicU64::new(1);

/// Generate a unique timer ID.
pub(crate) fn next_timer_id() -> u64 {
    NEXT_TIMER_ID.fetch_add(1, Ordering::Relaxed)
}

/// The WoW Lua environment.
pub struct WowLuaEnv {
    pub(crate) lua: Lua,
    pub(crate) state: Rc<RefCell<SimState>>,
}

impl WowLuaEnv {
    /// Create a new WoW Lua environment with the API initialized.
    pub fn new() -> Result<Self> {
        let lua = unsafe { Lua::unsafe_new() };
        let state = Rc::new(RefCell::new(SimState::default()));
        init_builtin_frames(&state);
        init_lua_state(&lua, Rc::clone(&state))?;
        Ok(Self { lua, state })
    }

    /// Execute Lua code.
    pub fn exec(&self, code: &str) -> Result<()> {
        self.lua.load(code).exec()?;
        Ok(())
    }

    /// Execute Lua code with a custom chunk name (for better error messages and debugstack).
    pub fn exec_named(&self, code: &str, name: &str) -> Result<()> {
        self.lua.load(code).set_name(name).exec()?;
        Ok(())
    }

    /// Execute Lua code with varargs (like WoW addon loading).
    /// In WoW, each addon file receives (addonName, addonTable) as varargs.
    pub fn exec_with_varargs(
        &self,
        code: &str,
        name: &str,
        addon_name: &str,
        addon_table: mlua::Table,
    ) -> Result<()> {
        let chunk = self.lua.load(code).set_name(name);
        let func: mlua::Function = chunk.into_function()?;
        func.call::<()>((addon_name.to_string(), addon_table))?;
        Ok(())
    }

    /// Create a new empty table for addon private storage.
    /// Includes a default `unpack` method that returns values at numeric indices.
    pub fn create_addon_table(&self) -> Result<mlua::Table> {
        let table = self.lua.create_table()?;
        // Add default unpack method - returns values at indices 1, 2, 3, 4
        // Addons like OmniCD use this pattern: local E, L, C = select(2, ...):unpack()
        let unpack_fn = self.lua.create_function(|_, this: mlua::Table| {
            let v1: mlua::Value = this.get(1).unwrap_or(mlua::Value::Nil);
            let v2: mlua::Value = this.get(2).unwrap_or(mlua::Value::Nil);
            let v3: mlua::Value = this.get(3).unwrap_or(mlua::Value::Nil);
            let v4: mlua::Value = this.get(4).unwrap_or(mlua::Value::Nil);
            Ok((v1, v2, v3, v4))
        })?;
        table.set("unpack", unpack_fn)?;
        Ok(table)
    }

    /// Execute Lua code and return the result.
    pub fn eval<T: mlua::FromLuaMulti>(&self, code: &str) -> Result<T> {
        let result = self.lua.load(code).eval()?;
        Ok(result)
    }

    /// Populate the `__addon_names` registry table mapping addon index → folder name.
    /// The table is pre-created in `init_registry_tables` so the OnUpdate dispatcher
    /// captures a reference to it. This method fills it after all addons are loaded.
    pub fn sync_addon_names_to_lua(&self) {
        let state = self.state.borrow();
        let Ok(t) = self
            .lua
            .named_registry_value::<mlua::Table>("__addon_names")
        else {
            return;
        };
        for (i, addon) in state.addons.iter().enumerate() {
            t.raw_set(i as i64, addon.folder_name.as_str()).ok();
        }
    }

    /// Restore globals that EnvironmentCleanup nil'd but later addons need.
    ///
    /// Call immediately after loading Blizzard_EnvironmentCleanup so that
    /// subsequent Blizzard addons (Menu, SharedXMLGame, etc.) can use
    /// `CreateSecureDelegate` at file scope.
    pub fn restore_post_cleanup_globals(&self) {
        let _ = super::globals::environment_restore::restore_post_cleanup_globals(&self.lua);
    }

    /// Apply post-load workarounds for Blizzard code that depends on
    /// unimplemented engine features (AnimationGroups, EditMode, etc.).
    /// Must be called after all addons are loaded and before firing events.
    pub fn apply_post_load_workarounds(&self) {
        super::workarounds::apply(self);
        self.restore_post_cleanup_globals();
        // Wrap seterrorhandler with newsecurefunction so coroutine.create rejects it.
        // Done here because BugGrabber overwrites it during addon loading.
        let _ = self.lua.load(
            "rawset(_G, 'seterrorhandler', debug.newsecurefunction(rawget(_G, 'seterrorhandler')))"
        ).exec();
    }

    /// Apply workarounds that must run after startup events.
    ///
    /// Some fixes (like BagsBar anchoring) get undone by event handlers
    /// (e.g. EDIT_MODE_LAYOUTS_UPDATED repositions managed frames).
    pub fn apply_post_event_workarounds(&self) {
        super::workarounds::apply_post_event(self);
    }

    /// Fire an event to all registered frames.
    pub fn fire_event(&self, event: &str) -> Result<()> {
        self.fire_event_with_args(event, &[])
    }

    /// Fire an event with arguments to all registered frames.
    pub fn fire_event_with_args(&self, event: &str, args: &[Value]) -> Result<()> {
        use super::script_helpers::{call_error_handler, get_frame_ref, get_script};

        let listeners = super::script_helpers::get_event_listeners_lua_order(&self.lua, event)?;

        for widget_id in listeners {
            if let Some(handler) = get_script(&self.lua, widget_id, "OnEvent")
                && let Some(frame) = get_frame_ref(&self.lua, widget_id)
            {
                let addon_idx = self
                    .state
                    .borrow()
                    .widgets
                    .get(widget_id)
                    .and_then(|f| f.owner_addon);
                let taint = addon_taint_name(&self.state, addon_idx);
                let blizzard = is_blizzard_addon(&self.state, addon_idx);
                let mut call_args = vec![frame, Value::String(self.lua.create_string(event)?)];
                call_args.extend(args.iter().cloned());

                let start = Instant::now();
                self.state.borrow_mut().executing_addon_index = addon_idx;
                let result = call_with_taint(&self.lua, handler, taint, blizzard, call_args);
                self.state.borrow_mut().executing_addon_index = None;
                if let Err(e) = result {
                    call_error_handler(&self.lua, &e.to_string());
                }
                record_addon_time(&self.state, addon_idx, &start);
            }
        }

        Ok(())
    }

    /// Fire a script handler for a specific widget with per-addon taint restoration.
    pub fn fire_script_handler(
        &self,
        widget_id: u64,
        handler_name: &str,
        extra_args: Vec<Value>,
    ) -> Result<()> {
        use super::script_helpers::{call_error_handler, get_script};

        if let Some(handler) = get_script(&self.lua, widget_id, handler_name) {
            let frame = super::frame::frame_ref(&self.lua, widget_id)?;
            let addon_idx = self
                .state
                .borrow()
                .widgets
                .get(widget_id)
                .and_then(|f| f.owner_addon);
            let taint = addon_taint_name(&self.state, addon_idx);
            let blizzard = is_blizzard_addon(&self.state, addon_idx);
            let mut call_args = vec![frame];
            call_args.extend(extra_args);
            self.state.borrow_mut().executing_addon_index = addon_idx;
            if let Err(e) = call_with_taint(&self.lua, handler, taint, blizzard, call_args) {
                call_error_handler(&self.lua, &e.to_string());
            }
            self.state.borrow_mut().executing_addon_index = None;
        }

        Ok(())
    }

    /// Check if a script handler is registered for a widget.
    pub fn has_script_handler(&self, widget_id: u64, handler_name: &str) -> bool {
        super::script_helpers::get_script(&self.lua, widget_id, handler_name).is_some()
    }

    /// Resolve a clicked frame to the nearest EditBox in its parent chain.
    pub(crate) fn resolve_editbox_focus_target(&self, clicked_frame: Option<u64>) -> Option<u64> {
        use crate::widget::WidgetType;

        let state = self.state.borrow();
        let mut current = clicked_frame;

        while let Some(frame_id) = current {
            let Some(frame) = state.widgets.get(frame_id) else {
                break;
            };
            if frame.widget_type == WidgetType::EditBox {
                return Some(frame_id);
            }
            current = frame.parent_id;
        }

        None
    }

    /// Simulate a left-click on a frame by ID.
    ///
    /// Handles EditBox focus management (focus/unfocus), then fires
    /// OnMouseDown, OnClick, and OnMouseUp in sequence.
    pub fn send_click(&self, frame_id: u64) -> Result<()> {
        let editbox_target = self.resolve_editbox_focus_target(Some(frame_id));
        let old_focus = self.state.borrow().focused_frame_id;

        // EditBox focus management (mirrors iced_app::update::update_editbox_focus)
        if let Some(editbox_id) = editbox_target {
            if old_focus != Some(editbox_id) {
                self.state.borrow_mut().focused_frame_id = Some(editbox_id);
                if let Some(old_id) = old_focus {
                    self.fire_script_handler(old_id, "OnEditFocusLost", vec![])?;
                }
                self.fire_script_handler(editbox_id, "OnEditFocusGained", vec![])?;
            }
        } else if let Some(old_id) = old_focus {
            self.state.borrow_mut().focused_frame_id = None;
            self.fire_script_handler(old_id, "OnEditFocusLost", vec![])?;
        }

        let button_val = Value::String(self.lua.create_string("LeftButton")?);
        self.fire_script_handler(frame_id, "OnMouseDown", vec![button_val.clone()])?;
        let down_val = Value::Boolean(false);
        self.fire_script_handler(frame_id, "OnClick", vec![button_val.clone(), down_val])?;
        self.fire_script_handler(frame_id, "OnMouseUp", vec![button_val])?;

        Ok(())
    }

    /// Dispatch a slash command (e.g., "/wa options").
    /// Returns Ok(true) if a handler was found and called, Ok(false) if no handler matched.
    pub fn dispatch_slash_command(&self, input: &str) -> Result<bool> {
        let input = input.trim();
        if !input.starts_with('/') {
            return Ok(false);
        }

        // Parse command and message: "/wa options" -> cmd="/wa", msg="options"
        let (cmd, msg) = match input.find(' ') {
            Some(pos) => (&input[..pos], input[pos + 1..].trim()),
            None => (input, ""),
        };
        let cmd_lower = cmd.to_lowercase();

        // Scan globals for SLASH_* variables to find a matching command
        let globals = self.lua.globals();
        let slash_cmd_list: mlua::Table = globals.get("SlashCmdList")?;

        // Iterate through all globals looking for SLASH_* patterns
        for pair in globals.pairs::<String, Value>() {
            let (key, value) = pair?;

            // Look for SLASH_NAME1, SLASH_NAME2, etc.
            if !key.starts_with("SLASH_") {
                continue;
            }

            // Extract the command name (e.g., "SLASH_WEAKAURAS1" -> "WEAKAURAS")
            let suffix = &key[6..]; // Skip "SLASH_"
            let name = suffix.trim_end_matches(|c: char| c.is_ascii_digit());
            if name.is_empty() {
                continue;
            }

            // Check if this SLASH_ variable matches our command
            if let Value::String(slash_str) = value
                && slash_str.to_str()?.to_lowercase() == cmd_lower
            {
                // Found a match! Look up the handler in SlashCmdList
                let handler: Option<mlua::Function> = slash_cmd_list.get(name).ok();
                if let Some(handler) = handler {
                    let msg_value = self.lua.create_string(msg)?;
                    handler.call::<()>(msg_value)?;
                    return Ok(true);
                }
            }
        }

        Ok(false)
    }

    /// Get access to the Lua state.
    pub fn lua(&self) -> &Lua {
        &self.lua
    }

    /// Get access to the simulator state.
    pub fn state(&self) -> &Rc<RefCell<SimState>> {
        &self.state
    }

    /// Create a loader environment borrowing from this environment.
    pub fn loader_env(&self) -> super::loader_env::LoaderEnv<'_> {
        super::loader_env::LoaderEnv::new(&self.lua, Rc::clone(&self.state))
    }

    /// Set the font system for text measurement from Lua API methods.
    ///
    /// This stores the font system as Lua app_data so that methods like
    /// `GetStringWidth()` can measure text accurately via cosmic-text.
    pub fn set_font_system(&self, font_system: Rc<RefCell<WowFontSystem>>) {
        self.lua.set_app_data(font_system);
    }

    /// Update screen dimensions in SimState and resize UIParent/WorldFrame to match.
    pub fn set_screen_size(&self, width: f32, height: f32) {
        let mut state = self.state.borrow_mut();
        state.screen_width = width;
        state.screen_height = height;
        // Screen resize invalidates all cached layout rects and strata buckets.
        state.strata_buckets = None;
        state.widgets.clear_all_layout_rects();
        for name in &["UIParent", "WorldFrame"] {
            if let Some(id) = state.widgets.get_id_by_name(name)
                && let Some(frame) = state.widgets.get_mut_visual(id)
            {
                frame.width = width;
                frame.height = height;
            }
        }
    }

    /// Select which UI surface should be loaded.
    pub fn set_screen_mode(&self, screen_kind: ScreenKind) {
        self.state.borrow_mut().set_screen_kind(screen_kind);
    }

    /// Toggle whether the simulated player is logged into the world.
    pub fn set_logged_in(&self, is_logged_in: bool) {
        self.state.borrow_mut().is_logged_in = is_logged_in;
    }

    /// Register an addon in the addon list.
    pub fn register_addon(&self, info: AddonInfo) {
        self.state.borrow_mut().addons.push(info);
    }

    /// Scan an addons directory and register all found addons (metadata only, no loading).
    pub fn scan_and_register_addons(&self, addons_path: &std::path::Path) {
        let mut addons = super::addon_scan::scan_addon_entries(addons_path);
        addons.sort_by(|a, b| {
            a.folder_name
                .to_lowercase()
                .cmp(&b.folder_name.to_lowercase())
        });
        let mut state = self.state.borrow_mut();
        for addon in addons {
            if !state
                .addons
                .iter()
                .any(|a| a.folder_name == addon.folder_name)
            {
                state.addons.push(addon);
            }
        }
    }

    /// Schedule a timer callback.
    pub fn schedule_timer(
        &self,
        seconds: f64,
        callback: mlua::Function,
        interval: Option<std::time::Duration>,
        iterations: Option<i32>,
    ) -> Result<u64> {
        let id = next_timer_id();
        let callback_key = self.lua.create_registry_value(callback)?;
        let fire_at = Instant::now() + std::time::Duration::from_secs_f64(seconds);

        let owner_addon = self.state.borrow().loading_addon_index;
        let timer = PendingTimer {
            id,
            fire_at,
            callback_key,
            interval,
            remaining: iterations,
            cancelled: false,
            handle_key: None,
            owner_addon,
        };

        self.state.borrow_mut().timers.push_back(timer);
        Ok(id)
    }

    /// Fire OnUpdate handlers for all frames that have them registered,
    /// then tick animation groups.
    pub fn fire_on_update(&self, elapsed: f64) -> Result<()> {
        super::on_update::fire(self, elapsed)
    }

    /// Read and clear `__addon_timing`, applying accumulated ms to each addon.
    fn drain_addon_timing(&self) {
        let Ok(timing) = self
            .lua
            .named_registry_value::<mlua::Table>("__addon_timing")
        else {
            return;
        };
        let mut keys = Vec::new();
        let mut state = self.state.borrow_mut();
        for pair in timing.pairs::<i64, f64>() {
            if let Ok((idx, ms)) = pair {
                keys.push(idx);
                if let Some(addon) = state.addons.get_mut(idx as usize) {
                    addon.runtime.current_frame_ms += ms;
                }
            }
        }
        drop(state);
        // Clear in-place (Lua dispatch holds a reference to this table).
        for key in keys {
            let _ = timing.raw_set(key, mlua::Value::Nil);
        }
    }

    pub(crate) fn finalize_frame_metrics(&self, frame_elapsed_ms: f64) {
        self.drain_addon_timing();
        let mut state = self.state.borrow_mut();
        // Update app-level frame metrics (total frame time for percentage calculations).
        let app = &mut state.app_frame_metrics;
        app.recent_frame_ms.push_back(frame_elapsed_ms);
        if app.recent_frame_ms.len() > 60 {
            app.recent_frame_ms.pop_front();
        }
        if frame_elapsed_ms > app.peak_ms {
            app.peak_ms = frame_elapsed_ms;
        }
        app.session_total_ms += frame_elapsed_ms;
        app.session_frame_count += 1;

        for addon in &mut state.addons {
            let ms = addon.runtime.current_frame_ms;
            if ms > 0.0 {
                addon.runtime.recent_frames.push_back(ms);
                if addon.runtime.recent_frames.len() > 60 {
                    addon.runtime.recent_frames.pop_front();
                }
                if ms > addon.runtime.peak_ms {
                    addon.runtime.peak_ms = ms;
                }
                addon.runtime.session_total_ms += ms;
                addon.runtime.session_frame_count += 1;
                update_threshold_counters(&mut addon.runtime, ms);
            }
            addon.runtime.current_frame_ms = 0.0;
        }
    }

    /// Fire `EDIT_MODE_LAYOUTS_UPDATED` with layout info from `C_EditMode.GetLayouts()`.
    ///
    /// Triggers `EditModeManagerFrame:UpdateLayoutInfo()` to initialize `layoutInfo`
    /// and unblock action bar positioning. No-op if EditMode addon isn't loaded.
    pub fn fire_edit_mode_layouts_updated(&self) -> Result<()> {
        let Ok(true) = self
            .lua
            .load("return C_EditMode ~= nil and C_EditMode.GetLayouts ~= nil")
            .eval::<bool>()
        else {
            return Ok(());
        };

        let Ok(info) = self
            .lua
            .load("return C_EditMode.GetLayouts()")
            .eval::<mlua::Table>()
        else {
            return Ok(());
        };

        self.fire_event_with_args(
            "EDIT_MODE_LAYOUTS_UPDATED",
            &[Value::Table(info), Value::Boolean(true)],
        )
    }

    /// Get the time until the next timer fires, if any.
    pub fn next_timer_delay(&self) -> Option<std::time::Duration> {
        let state = self.state.borrow();
        let now = Instant::now();
        state
            .timers
            .iter()
            .filter(|t| !t.cancelled)
            .map(|t| t.fire_at.saturating_duration_since(now))
            .min()
    }

    /// Dump all frame positions for debugging.
    pub fn dump_frames(&self) -> String {
        let state = self.state.borrow();
        super::diagnostics::dump_frames(&state)
    }
}

/// Increment threshold counters for a frame's addon time.
fn update_threshold_counters(rt: &mut AddonRuntimeMetrics, ms: f64) {
    if ms > 1.0 {
        rt.count_over_1ms += 1;
    }
    if ms > 5.0 {
        rt.count_over_5ms += 1;
    }
    if ms > 10.0 {
        rt.count_over_10ms += 1;
    }
    if ms > 50.0 {
        rt.count_over_50ms += 1;
    }
    if ms > 100.0 {
        rt.count_over_100ms += 1;
    }
    if ms > 500.0 {
        rt.count_over_500ms += 1;
    }
    if ms > 1000.0 {
        rt.count_over_1000ms += 1;
    }
}

/// Stamp addon taint on a handler and call it. The VM applies fixedtaint on entry.
/// For Blizzard addons (is_blizzard=true), clear the handler's taint so issecure()
/// returns true during execution, matching real WoW behavior.
fn call_with_taint(
    lua: &Lua,
    handler: mlua::Function,
    taint: Option<String>,
    is_blizzard: bool,
    args: Vec<Value>,
) -> mlua::Result<()> {
    if let Ok(sot) = lua.named_registry_value::<mlua::Function>("__setobjecttaint") {
        if is_blizzard {
            // Clear taint on Blizzard handlers so issecure() returns true.
            sot.call::<()>((handler.clone(), Value::Nil))?;
        } else if let Some(ref name) = taint {
            sot.call::<()>((handler.clone(), name.as_str()))?;
        }
    }
    handler.call(MultiValue::from_vec(args))
}

/// Look up the addon folder name for a given owner_addon index.
fn addon_taint_name(
    state: &Rc<RefCell<super::state::SimState>>,
    idx: Option<u16>,
) -> Option<String> {
    idx.and_then(|i| {
        state
            .borrow()
            .addons
            .get(i as usize)
            .map(|a| a.folder_name.clone())
    })
}

/// Check whether an addon index refers to a Blizzard addon (runs secure).
fn is_blizzard_addon(state: &Rc<RefCell<super::state::SimState>>, idx: Option<u16>) -> bool {
    idx.map(|i| {
        state
            .borrow()
            .addons
            .get(i as usize)
            .is_some_and(|a| a.folder_name.starts_with("Blizzard_"))
    })
    .unwrap_or(true)
}

/// Record per-addon timing from an Instant.
fn record_addon_time(
    state: &Rc<RefCell<super::state::SimState>>,
    idx: Option<u16>,
    start: &Instant,
) {
    if let Some(i) = idx {
        let ms = start.elapsed().as_secs_f64() * 1000.0;
        if let Some(addon) = state.borrow_mut().addons.get_mut(i as usize) {
            addon.runtime.current_frame_ms += ms;
        }
    }
}

/// Create built-in frames in the widget registry before Lua loads.
/// Registers a `__BuiltIn` pseudo-addon as their owner.
fn init_builtin_frames(state: &Rc<RefCell<SimState>>) {
    let mut s = state.borrow_mut();
    let owner = s.addons.len() as u16;
    s.addons.push(super::AddonInfo {
        folder_name: "__BuiltIn".to_string(),
        title: "Built-in Frames".to_string(),
        enabled: true,
        loaded: true,
        ..Default::default()
    });
    let (w, h) = (s.screen_width, s.screen_height);
    create_builtin_frames(&mut s.widgets, w, h, owner);
}

/// Initialize the Lua state: load Elune, register globals, patch stdlib, run keybindings.
fn init_lua_state(lua: &Lua, state: Rc<RefCell<SimState>>) -> crate::Result<()> {
    load_elune_security(lua)?;
    patch_secureexecuterange(lua)?;
    patch_elune_userdata_compat(lua)?;
    init_registry_tables(lua, &state)?;
    super::globals::register_globals(lua, Rc::clone(&state))?;
    super::secure_env::create_secure_environment(lua)?;
    enable_taint_and_wrap_loadstring(lua)?;
    super::keybindings::init_keybindings(lua)?;
    crate::loader::precompiled::init(lua)?;
    remove_sandbox_globals(lua)?;
    Ok(())
}

/// Load Elune's security library and secure call functions.
fn load_elune_security(lua: &Lua) -> crate::Result<()> {
    unsafe extern "C" {
        fn luaopen_security(state: *mut mlua::ffi::lua_State) -> std::ffi::c_int;
        fn luaopen_securecalls(state: *mut mlua::ffi::lua_State) -> std::ffi::c_int;
    }
    unsafe {
        lua.exec_raw::<()>((), |state| {
            luaopen_security(state);
        })?;
        lua.exec_raw::<()>((), |state| {
            luaopen_securecalls(state);
        })?;
    };
    Ok(())
}

/// Replace Elune's secureexecuterange with a plain Lua loop.
///
/// Elune's C implementation silently skips callbacks when taint propagation
/// interferes. The simulator doesn't enforce taint restrictions, so a plain
/// loop using securecallfunction (which swallows errors per-entry like WoW)
/// allows ContinueAfterAllEvents callbacks to fire during startup.
fn patch_secureexecuterange(lua: &Lua) -> crate::Result<()> {
    lua.load(
        r#"
        secureexecuterange = function(tbl, func, ...)
            if type(tbl) ~= "table" then return end
            for k, v in pairs(tbl) do
                securecallfunction(func, k, v, ...)
            end
        end
        "#,
    )
    .exec()?;
    Ok(())
}

/// Wrap Elune's hooksecurefunc/issecurevariable to accept userdata (FrameRef).
fn patch_elune_userdata_compat(lua: &Lua) -> crate::Result<()> {
    lua.load(include_str!("../../data/lua/elune_userdata_compat.lua"))
        .exec()?;
    Ok(())
}

/// Set up registry tables for event dispatch and taint fallback.
fn init_registry_tables(lua: &Lua, state: &Rc<RefCell<SimState>>) -> mlua::Result<()> {
    lua.set_named_registry_value("__event_individual", lua.create_table()?)?;
    lua.set_named_registry_value("__event_all", lua.create_table()?)?;
    // Persistent tables for OnUpdate profiler attribution.
    lua.set_named_registry_value("__frame_owners", lua.create_table()?)?;
    lua.set_named_registry_value("__addon_timing", lua.create_table()?)?;
    lua.set_named_registry_value("__addon_names", lua.create_table()?)?;
    let tainted_loadstring_functions = lua.create_table()?;
    let weak_meta = lua.create_table()?;
    weak_meta.set("__mode", "k")?;
    tainted_loadstring_functions.set_metatable(Some(weak_meta));
    lua.set_named_registry_value(
        "__tainted_loadstring_functions",
        tainted_loadstring_functions,
    )?;
    let taint_fallback: mlua::Function =
        lua.load("return debug.getstacktaint()").into_function()?;
    lua.set_named_registry_value("__get_stack_taint_fallback", taint_fallback)?;
    super::on_update::register(lua, state)
}

/// Enable Elune taint tracking and wrap loadstring as secure.
fn enable_taint_and_wrap_loadstring(lua: &Lua) -> mlua::Result<()> {
    lua.load("seterrorhandler(function() end); debug.settaintmode('rw')")
        .exec()?;
    // Cache setobjecttaint in registry for Rust-side and Lua-side use.
    let sot: mlua::Function = lua.load("return debug.setobjecttaint").eval()?;
    lua.set_named_registry_value("__setobjecttaint", sot)?;
    let sst: mlua::Function = lua.load("return debug.setstacktaint").eval()?;
    lua.set_named_registry_value("__setstacktaint", sst)?;
    lua.load(
        r#"
        local original_ls = loadstring
        local sst = debug.setstacktaint
        local sot = debug.setobjecttaint
        local tainted = debug.getregistry().__tainted_loadstring_functions
        loadstring = debug.newsecurefunction(function(code, name)
            sst("*** ForceTaint_Strong ***")
            local loaded, err = original_ls(code, name)
            if type(loaded) == "function" then
                sot(loaded, "*** ForceTaint_Strong ***")
                tainted[loaded] = true
            end
            return loaded, err
        end)
    "#,
    )
    .exec()?;
    Ok(())
}

/// Remove globals that WoW's sandbox doesn't expose and internal helpers
/// now stored in the Lua registry.
fn remove_sandbox_globals(lua: &Lua) -> mlua::Result<()> {
    let g = lua.globals();
    for name in &[
        "dofile",
        "load",
        "loadfile",
        "module",
        "require",
        "__original_ipairs",
        "__original_rawget",
        "__real_getmetatable",
        "__real_setmetatable",
        "__SetMixinOverride",
        "__report_script_error",
    ] {
        g.set(*name, Value::Nil)?;
    }
    lua.globals()
        .get::<mlua::Table>("string")?
        .set("dump", Value::Nil)?;
    lua.globals()
        .get::<mlua::Table>("math")?
        .set("randomseed", Value::Nil)?;
    Ok(())
}
