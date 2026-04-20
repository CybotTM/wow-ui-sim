use super::env::WowLuaAppData;
use super::env::WowLuaEnv;
use super::env::next_timer_id;
use super::env_init::{record_addon_time, update_threshold_counters};
use super::state::{AddonInfo, PendingTimer, SimState};
use super::timer_processing::{reschedule_timer, timer_should_wait};
use crate::Result;
use crate::font::WowFontSystem;
use crate::lua_api::methods::{call_function as call_rilua_function, registry_get};
use crate::lua_api::script_helpers::call_error_handler;
use crate::screen::ScreenKind;
use rilua::{LuaApiMut, Val};
use std::cell::RefCell;
use std::rc::Rc;
use std::time::{Duration, Instant};

impl WowLuaEnv {
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
        let _ = self.exec(&format!(
            r#"
            function GetScreenWidth()
                return {width}
            end

            function GetScreenHeight()
                return {height}
            end

            function GetPhysicalScreenSize()
                return GetScreenWidth(), GetScreenHeight()
            end
            "#
        ));
    }

    /// Select which UI surface should be loaded.
    pub fn set_screen_mode(&self, screen_kind: ScreenKind) {
        self.state.borrow_mut().set_screen_kind(screen_kind);
        let (aurora_state, connected_to_wow, wow_connection_state, has_realm_list) =
            screen_kind.login_state();
        let is_glue = if screen_kind.is_glue() {
            "true"
        } else {
            "false"
        };
        let _ = self.exec(&format!(
            r#"
            __wow_screen_mode_is_glue = {is_glue}
            __wow_login_aurora_state = {aurora_state}
            __wow_login_connected_to_wow = {connected_to_wow}
            __wow_login_wow_connection_state = {wow_connection_state}
            __wow_login_has_realm_list = {has_realm_list}

            function InGlue()
                return {is_glue}
            end

            if C_Glue == nil then
                C_Glue = {{}}
            end

            function C_Glue.IsOnGlueScreen()
                return {is_glue}
            end
            "#
        ));
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
        self.fire_on_update_timed(elapsed).map(|_| ())
    }

    /// Fire OnUpdate handlers and return stage timings for profiling.
    pub(crate) fn fire_on_update_timed(
        &self,
        elapsed: f64,
    ) -> Result<super::on_update::OnUpdateStageTimings> {
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
