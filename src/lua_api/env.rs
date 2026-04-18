//! WoW Lua environment.

use super::env_convert::{FromRiluaResults, unpack_eval_results};
use super::env_init::{
    addon_taint_name, init_builtin_frames, init_lua_state, is_blizzard_addon, record_addon_time,
    update_threshold_counters,
};
use super::state::{AddonInfo, PendingTimer, SimState};
use super::timer_processing::{reschedule_timer, timer_should_wait};
use crate::Result;
use crate::font::WowFontSystem;
use crate::lua_api::methods::{
    call_function as call_rilua_function, create_string, frame_ref, registry_get, registry_set,
    table_set, val_to_string,
};
use crate::lua_api::script_helpers::{call_error_handler, get_event_listeners, get_script};
use crate::screen::ScreenKind;
use rilua::{LuaApi, LuaApiMut, Val};
use std::cell::RefCell;
use std::rc::Rc;
use std::sync::atomic::{AtomicU64, Ordering};
use std::time::{Duration, Instant};

static NEXT_TIMER_ID: AtomicU64 = AtomicU64::new(1);
const EVAL_RESULTS_REGISTRY_KEY: &str = "__wow_eval_results";

/// Generate a unique timer ID.
pub(crate) fn next_timer_id() -> u64 {
    NEXT_TIMER_ID.fetch_add(1, Ordering::Relaxed)
}

#[derive(Clone)]
pub(crate) struct WowLuaAppData {
    pub(crate) sim_state: Rc<RefCell<SimState>>,
    pub(crate) lua: Option<Rc<RefCell<rilua::Lua>>>,
    pub(crate) font_system: Option<Rc<RefCell<WowFontSystem>>>,
    /// Pre-interned handles for the hot-literal whitelist. Populated by
    /// `HotLiteralRegistry::install` during bootstrap (Track 1 sub-item 2).
    /// `None` on a fresh VM before the register-globals pass runs.
    pub(crate) hot_literals: Option<crate::lua_api::hot_literals::HotLiteralHandles>,
    /// Frozen slot vector for the Track 3 global-slot fast path.
    /// Populated by `global_slots::install` at the end of
    /// `init_lua_state`. `None` on a fresh VM before bootstrap runs.
    pub(crate) global_slots: Option<crate::lua_api::global_slots::GlobalSlotTable>,
}

impl WowLuaAppData {
    fn new(sim_state: Rc<RefCell<SimState>>) -> Self {
        Self {
            sim_state,
            lua: None,
            font_system: None,
            hot_literals: None,
            global_slots: None,
        }
    }
}

/// The WoW Lua environment.
pub struct WowLuaEnv {
    pub(crate) lua: Rc<RefCell<rilua::Lua>>,
    pub(crate) state: Rc<RefCell<SimState>>,
}

impl Drop for WowLuaEnv {
    fn drop(&mut self) {
        if let Ok(state) = self.state.try_borrow() {
            crate::lua_errors::print_suppressed_error_summary(&state);
        }
    }
}

impl WowLuaEnv {
    /// Create a new WoW Lua environment with the API initialized.
    pub fn new() -> Result<Self> {
        let state = Rc::new(RefCell::new(SimState::default()));
        let mut lua = Self::new_rilua(Rc::clone(&state));
        init_builtin_frames(&state);
        init_lua_state(&mut lua, Rc::clone(&state))?;
        let env = Self {
            lua: Rc::new(RefCell::new(lua)),
            state,
        };
        {
            let mut lua = env.lua.borrow_mut();
            let app_data = lua
                .state_mut()
                .app_data_mut::<WowLuaAppData>()
                .expect("WowLuaEnv rilua app_data should always exist");
            app_data.lua = Some(Rc::clone(&env.lua));
        }
        Ok(env)
    }

    fn new_rilua(state: Rc<RefCell<SimState>>) -> rilua::Lua {
        let mut lua = rilua::Lua::new().expect("failed to create rilua Lua state");
        lua.state_mut().set_app_data(WowLuaAppData::new(state));
        lua
    }

    /// Execute Lua code.
    pub fn exec(&self, code: &str) -> Result<()> {
        self.exec_rilua(code)?;
        Ok(())
    }

    /// Borrow the active rilua VM.
    pub fn lua(&self) -> std::cell::Ref<'_, rilua::Lua> {
        self.rilua()
    }

    /// Execute Lua code with a custom chunk name (for better error messages and debugstack).
    pub fn exec_named(&self, code: &str, name: &str) -> Result<()> {
        self.exec_rilua_named(code, name)?;
        Ok(())
    }

    /// Execute Lua code and return the result.
    pub fn eval<T: FromRiluaResults>(&self, code: &str) -> Result<T> {
        let mut lua = self.lua.borrow_mut();
        {
            let state = lua.state_mut();
            registry_set(state, EVAL_RESULTS_REGISTRY_KEY, Val::Nil);
        }
        let wrapped = format!(
            "local function __wow_eval()\n{code}\nend\ndebug.getregistry().{EVAL_RESULTS_REGISTRY_KEY} = {{ __wow_eval() }}"
        );
        let exec_result = lua.exec(&wrapped);
        let packed_results = {
            let state = lua.state_mut();
            let packed = registry_get(state, EVAL_RESULTS_REGISTRY_KEY);
            registry_set(state, EVAL_RESULTS_REGISTRY_KEY, Val::Nil);
            packed
        };
        exec_result?;
        let results = unpack_eval_results(lua.state(), packed_results)?;
        T::from_results(lua.state(), results)
    }

    /// Create a Lua string value on the active rilua VM.
    pub fn lua_string(&self, text: &str) -> Val {
        let mut lua = self.lua.borrow_mut();
        create_string(lua.state_mut(), text)
    }

    /// Run a full GC cycle then re-enable the incremental collector.
    ///
    /// Pairs with [`Self::gc_stop`] to bracket bootstrap / addon-load
    /// allocations. The full collection drops transients allocated
    /// while the collector was paused; `gc_restart` resets the debt
    /// threshold so the incremental collector resumes normally.
    pub fn gc_restart_after_bootstrap(&self) -> crate::Result<()> {
        use rilua::LuaApiMut;
        let mut lua = self.lua.borrow_mut();
        lua.gc_collect()?;
        lua.gc_restart();
        Ok(())
    }

    /// Populate the `__addon_names` registry table mapping addon index → folder name.
    pub fn sync_addon_names_to_lua(&self) {
        let addon_names = {
            let state = self.state.borrow();
            state
                .addons
                .iter()
                .map(|addon| addon.folder_name.clone())
                .collect::<Vec<_>>()
        };

        let mut lua = self.lua.borrow_mut();
        let state = lua.state_mut();
        let table = registry_get(state, "__addon_names");
        for (index, addon_name) in addon_names.iter().enumerate() {
            let addon_name_val = create_string(state, addon_name);
            table_set(state, table, &index.to_string(), addon_name_val);
            if let Val::Table(table_ref) = table
                && let Some(table) = state.gc.tables.get_mut(table_ref)
            {
                let _ = table.raw_set(
                    Val::Num(index as f64),
                    addon_name_val,
                    &state.gc.string_arena,
                );
            }
        }
    }

    /// Restore globals that EnvironmentCleanup nil'd but later addons need.
    pub fn restore_post_cleanup_globals(&self) {
        let mut lua = self.rilua_mut();
        let _ = super::globals::environment_restore::restore_post_cleanup_globals(
            &mut lua,
            Rc::clone(&self.state),
        );
    }

    /// Apply post-load workarounds for Blizzard code that depends on
    /// unimplemented engine features (AnimationGroups, EditMode, etc.).
    pub fn apply_post_load_workarounds(&self) {
        super::workarounds::apply(self);
        self.restore_post_cleanup_globals();
        let _ = self.exec(
            "rawset(_G, 'seterrorhandler', debug.newsecurefunction(rawget(_G, 'seterrorhandler')))",
        );
    }

    /// Apply workarounds that must run after startup events.
    pub fn apply_post_event_workarounds(&self) {
        super::workarounds::apply_post_event(self);
    }

    /// Fire an event to all registered frames.
    pub fn fire_event(&self, event: &str) -> Result<()> {
        self.fire_event_with_args(event, &[])
    }

    /// Fire an event with arguments to all registered frames.
    pub fn fire_event_with_args(&self, event: &str, args: &[Val]) -> Result<()> {
        let listeners = {
            let mut lua = self.lua.borrow_mut();
            get_event_listeners(lua.state_mut(), event)
        };
        for widget_id in listeners {
            self.dispatch_event_to_frame(widget_id, event, args)?;
        }
        Ok(())
    }

    fn handler_owner_addon(&self, widget_id: u64) -> Option<u16> {
        self.state
            .borrow()
            .widgets
            .get(widget_id)
            .and_then(|frame| frame.owner_addon)
    }

    fn build_event_call_args(
        &self,
        lua: &mut rilua::Lua,
        widget_id: u64,
        event: &str,
        args: &[Val],
    ) -> Result<Vec<Val>> {
        let frame = {
            let state = lua.state_mut();
            frame_ref(state, widget_id)?
        };
        let event_name = {
            let state = lua.state_mut();
            create_string(state, event)
        };
        let mut call_args = Vec::with_capacity(args.len() + 2);
        call_args.push(frame);
        call_args.push(event_name);
        call_args.extend_from_slice(args);
        Ok(call_args)
    }

    fn build_script_call_args(
        &self,
        lua: &mut rilua::Lua,
        widget_id: u64,
        extra_args: Vec<Val>,
    ) -> Result<Vec<Val>> {
        let frame = {
            let state = lua.state_mut();
            frame_ref(state, widget_id)?
        };
        let mut call_args = Vec::with_capacity(extra_args.len() + 1);
        call_args.push(frame);
        call_args.extend(extra_args);
        Ok(call_args)
    }

    fn call_widget_handler(
        &self,
        lua: &mut rilua::Lua,
        addon_idx: Option<u16>,
        handler: Val,
        call_args: &[Val],
    ) {
        let taint = addon_taint_name(&self.state, addon_idx);
        let blizzard = is_blizzard_addon(&self.state, addon_idx);
        let _ = (taint, blizzard);

        let start = Instant::now();
        self.state.borrow_mut().executing_addon_index = addon_idx;
        let call_result = call_rilua_function(lua, handler, call_args);
        self.state.borrow_mut().executing_addon_index = None;
        if let Err(error) = call_result {
            call_error_handler(lua, &error.to_string());
        }
        record_addon_time(&self.state, addon_idx, &start);
    }

    fn dispatch_event_to_frame(&self, widget_id: u64, event: &str, args: &[Val]) -> Result<()> {
        let addon_idx = self.handler_owner_addon(widget_id);
        let mut lua = self.lua.borrow_mut();
        let handler = {
            let state = lua.state_mut();
            get_script(state, widget_id, "OnEvent")
        };
        let Some(handler) = handler else {
            return Ok(());
        };
        let call_args = self.build_event_call_args(&mut lua, widget_id, event, args)?;
        self.call_widget_handler(&mut lua, addon_idx, handler, &call_args);
        Ok(())
    }

    /// Fire a script handler for a specific widget with per-addon taint restoration.
    pub fn fire_script_handler(
        &self,
        widget_id: u64,
        handler_name: &str,
        extra_args: Vec<Val>,
    ) -> Result<()> {
        let addon_idx = self.handler_owner_addon(widget_id);
        let mut lua = self.lua.borrow_mut();
        let handler = {
            let state = lua.state_mut();
            get_script(state, widget_id, handler_name)
        };
        let Some(handler) = handler else {
            return Ok(());
        };

        let call_args = self.build_script_call_args(&mut lua, widget_id, extra_args)?;
        self.call_widget_handler(&mut lua, addon_idx, handler, &call_args);
        Ok(())
    }

    /// Check if a script handler is registered for a widget.
    pub fn has_script_handler(&self, widget_id: u64, handler_name: &str) -> bool {
        let mut lua = self.lua.borrow_mut();
        get_script(lua.state_mut(), widget_id, handler_name).is_some()
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
    pub fn send_click(&self, frame_id: u64) -> Result<()> {
        let editbox_target = self.resolve_editbox_focus_target(Some(frame_id));
        let old_focus = self.state.borrow().focused_frame_id;

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

        let button_val = self.lua_string("LeftButton");
        self.fire_script_handler(frame_id, "OnMouseDown", vec![button_val])?;
        self.fire_script_handler(frame_id, "OnClick", vec![button_val, Val::Bool(false)])?;
        self.fire_script_handler(frame_id, "OnMouseUp", vec![button_val, Val::Bool(true)])?;
        Ok(())
    }

    /// Dispatch a slash command (e.g., "/wa options").
    /// Returns Ok(true) if a handler was found and called, Ok(false) if no handler matched.
    pub fn dispatch_slash_command(&self, input: &str) -> Result<bool> {
        let input = input.trim();
        if !input.starts_with('/') {
            return Ok(false);
        }

        let (cmd, msg) = match input.find(' ') {
            Some(pos) => (&input[..pos], input[pos + 1..].trim()),
            None => (input, ""),
        };
        let cmd_lower = cmd.to_lowercase();

        let mut lua = self.lua.borrow_mut();
        let slash_cmd_list = LuaApiMut::get_global_val(&mut *lua, "SlashCmdList");
        let Val::Table(slash_table_ref) = slash_cmd_list else {
            return Ok(false);
        };
        let state = lua.state_mut();
        let globals = state.global;

        for (name, handler) in matching_slash_handlers(state, globals, slash_table_ref, &cmd_lower)
        {
            if !matches!(handler, Val::Function(_)) {
                continue;
            }
            let msg_val = create_string(state, msg);
            let _ = call_rilua_function(&mut lua, handler, &[msg_val])?;
            let _ = name;
            return Ok(true);
        }

        Ok(false)
    }

    /// Call a named global Lua function.
    pub fn call_global(&self, name: &str, args: &[Val]) -> Result<Vec<Val>> {
        let mut lua = self.lua.borrow_mut();
        let func = LuaApiMut::get_global_val(&mut *lua, name);
        let Val::Function(func_ref) = func else {
            return Ok(Vec::new());
        };
        let func_handle = rilua::Function::from_gc_ref(func_ref);
        lua.call_function(&func_handle, args).map_err(Into::into)
    }

    /// Get access to the simulator state.
    pub fn state(&self) -> &Rc<RefCell<SimState>> {
        &self.state
    }

    /// Set the font system for text measurement from Lua API methods.
    pub fn set_font_system(&self, font_system: Rc<RefCell<WowFontSystem>>) {
        let mut rilua = self.rilua_mut();
        let app_data = rilua
            .state_mut()
            .app_data_mut::<WowLuaAppData>()
            .expect("WowLuaEnv rilua app_data should always exist");
        app_data.font_system = Some(font_system);
    }

    /// Update screen dimensions in SimState and resize UIParent/WorldFrame to match.
    pub fn set_screen_size(&self, width: f32, height: f32) {
        let mut state = self.state.borrow_mut();
        state.screen_width = width;
        state.screen_height = height;
        state.invalidate_strata_buckets();
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
        callback: Val,
        interval: Option<Duration>,
        iterations: Option<i32>,
    ) -> Result<u64> {
        let id = next_timer_id();
        {
            let mut lua = self.lua.borrow_mut();
            crate::lua_api::timer_layout::store_timer_callback(lua.state_mut(), id, callback);
        }
        let owner_addon = {
            let state = self.state.borrow();
            state.loading_addon_index.or(state.executing_addon_index)
        };
        let timer = PendingTimer {
            id,
            fire_at: Instant::now() + Duration::from_secs_f64(seconds),
            interval,
            remaining: iterations,
            cancelled: false,
            owner_addon,
        };
        self.state.borrow_mut().rilua_timers.push_back(timer);
        Ok(id)
    }

    /// Run ready timers and return how many callbacks fired.
    pub fn process_timers(&self) -> Result<usize> {
        let now = Instant::now();
        let mut fired = 0usize;
        let mut timers = {
            let mut state = self.state.borrow_mut();
            let mut pending = std::collections::VecDeque::new();
            std::mem::swap(&mut pending, &mut state.rilua_timers);
            pending
        };

        let mut requeue = std::collections::VecDeque::new();
        while let Some(mut timer) = timers.pop_front() {
            if timer_should_wait(&timer, now) {
                requeue.push_back(timer);
                continue;
            }

            let Some(callback) = self.timer_callback(timer.id) else {
                continue;
            };

            self.fire_timer_callback(timer.owner_addon, callback);
            fired += 1;

            if reschedule_timer(&mut timer, now) {
                requeue.push_back(timer);
                continue;
            }

            self.remove_timer_callback(timer.id);
        }

        self.state.borrow_mut().rilua_timers = requeue;
        Ok(fired)
    }

    /// Fire OnUpdate handlers for all frames that have them registered.
    pub fn fire_on_update(&self, elapsed: f64) -> Result<()> {
        super::on_update::fire(self, elapsed)
    }

    fn drain_addon_timing(&self) {
        let mut lua = self.lua.borrow_mut();
        let state = lua.state_mut();
        let timing = registry_get(state, "__addon_timing");
        let Val::Table(timing_ref) = timing else {
            return;
        };
        let entries = state
            .gc
            .tables
            .get(timing_ref)
            .map(|table| table.hash_entries())
            .unwrap_or_default();
        let mut consumed_keys = Vec::new();
        {
            let mut sim = self.state.borrow_mut();
            for (key, value) in entries {
                let Val::Num(idx) = key else {
                    continue;
                };
                let Val::Num(ms) = value else {
                    continue;
                };
                consumed_keys.push(idx);
                if let Some(addon) = sim.addons.get_mut(idx as usize) {
                    addon.runtime.current_frame_ms += ms;
                }
            }
        }
        for idx in consumed_keys {
            if let Some(table) = state.gc.tables.get_mut(timing_ref) {
                let _ = table.raw_set(Val::Num(idx), Val::Nil, &state.gc.string_arena);
            }
        }
    }

    pub(crate) fn finalize_frame_metrics(&self, frame_elapsed_ms: f64) {
        self.drain_addon_timing();
        let mut state = self.state.borrow_mut();
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
    pub fn fire_edit_mode_layouts_updated(&self) -> Result<()> {
        let Ok(true) =
            self.eval::<bool>("return C_EditMode ~= nil and C_EditMode.GetLayouts ~= nil")
        else {
            return Ok(());
        };

        let info = self.eval::<Val>(
            r#"
            local source = (EditModeManagerFrame and EditModeManagerFrame.layoutInfo) or C_EditMode.GetLayouts()
            if type(source) ~= "table" then
                return source
            end

            local filtered = {
                layouts = {},
                activeLayout = source.activeLayout or 1,
            }
            local editModeLayoutType = type(Enum) == "table" and Enum.EditModeLayoutType or nil

            if type(source.layouts) ~= "table" then
                return filtered
            end

            for _, layoutInfo in ipairs(source.layouts) do
                local layoutType = type(layoutInfo) == "table" and layoutInfo.layoutType or nil
                if editModeLayoutType == nil
                    or layoutType == editModeLayoutType.Account
                    or layoutType == editModeLayoutType.Character then
                    table.insert(filtered.layouts, layoutInfo)
                end
            end

            return filtered
            "#,
        )?;
        self.fire_event_with_args("EDIT_MODE_LAYOUTS_UPDATED", &[info, Val::Bool(true)])
    }

    /// Get the time until the next timer fires, if any.
    pub fn next_timer_delay(&self) -> Option<Duration> {
        let state = self.state.borrow();
        let now = Instant::now();
        state
            .rilua_timers
            .iter()
            .filter(|timer| !timer.cancelled)
            .map(|timer| timer.fire_at.saturating_duration_since(now))
            .min()
    }

    /// Dump all frame positions for debugging.
    pub fn dump_frames(&self) -> String {
        let state = self.state.borrow();
        super::diagnostics::dump_frames(&state)
    }

    fn timer_callback(&self, timer_id: u64) -> Option<Val> {
        let mut lua = self.lua.borrow_mut();
        let callback = crate::lua_api::timer_layout::get_timer_callback(lua.state_mut(), timer_id);
        (!matches!(callback, Val::Nil)).then_some(callback)
    }

    fn fire_timer_callback(&self, owner_addon: Option<u16>, callback: Val) {
        let start = Instant::now();
        self.state.borrow_mut().executing_addon_index = owner_addon;
        let call_result = {
            let mut lua = self.lua.borrow_mut();
            call_rilua_function(&mut lua, callback, &[])
        };
        self.state.borrow_mut().executing_addon_index = None;
        if let Err(error) = call_result {
            let mut lua = self.lua.borrow_mut();
            call_error_handler(&mut lua, &error.to_string());
        }
        record_addon_time(&self.state, owner_addon, &start);
    }

    fn remove_timer_callback(&self, timer_id: u64) {
        let mut lua = self.lua.borrow_mut();
        crate::lua_api::timer_layout::remove_timer_callback(lua.state_mut(), timer_id);
    }
}

fn global_hash_entries(
    state: &rilua::vm::state::LuaState,
    globals: rilua::vm::gc::arena::GcRef<rilua::vm::table::Table>,
) -> Vec<(Val, Val)> {
    state
        .gc
        .tables
        .get(globals)
        .map(|table| table.hash_entries())
        .unwrap_or_default()
}

fn slash_command_name(key: &str) -> Option<&str> {
    if !key.starts_with("SLASH_") {
        return None;
    }
    let suffix = &key[6..];
    let name = suffix.trim_end_matches(|c: char| c.is_ascii_digit());
    (!name.is_empty()).then_some(name)
}

fn matching_slash_handlers(
    state: &mut rilua::vm::state::LuaState,
    globals: rilua::vm::gc::arena::GcRef<rilua::vm::table::Table>,
    slash_table_ref: rilua::vm::gc::arena::GcRef<rilua::vm::table::Table>,
    command: &str,
) -> Vec<(String, Val)> {
    let entries = global_hash_entries(state, globals);
    let mut matches = Vec::new();

    for (key, value) in entries {
        let Some(key_string) = val_to_string(state, key) else {
            continue;
        };
        let Some(name) = slash_command_name(&key_string) else {
            continue;
        };
        let Some(slash_command) = val_to_string(state, value) else {
            continue;
        };
        if slash_command.to_lowercase() != command {
            continue;
        }

        let handler_key = state.gc.intern_string(name.as_bytes());
        let handler = state
            .gc
            .tables
            .get(slash_table_ref)
            .map(|table| table.get_str(handler_key, &state.gc.string_arena))
            .unwrap_or(Val::Nil);
        matches.push((name.to_string(), handler));
    }

    matches
}
