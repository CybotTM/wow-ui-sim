//! App::update() method and related logic.

#[cfg(not(unix))]
use crate::inspector_server_stub::ScreenshotData;
use iced::Task;
#[cfg(unix)]
use iced_layout_inspector::server::ScreenshotData;
use rustc_hash::FxHashSet;
use std::sync::OnceLock;
use std::time::{Duration, Instant};

use crate::lua_api::WowLuaEnv;

use super::Message;
use super::app::App;
use super::state::CanvasMessage;
use super::update_helpers::{
    apply_subtree_hit_grid_change, get_checked_attribute, is_toggleable_checkbutton,
    merge_dirty_ids,
};

fn request_redraw_task<T>() -> Task<T> {
    iced_runtime::task::effect(iced_runtime::Action::Window(
        iced_runtime::window::Action::RedrawAll,
    ))
}

impl App {
    pub fn update(&mut self, message: Message) -> Task<Message> {
        let ipc_task = self.process_ipc();
        let task = match message {
            Message::CanvasEvent(cm) => self.handle_canvas_event(cm),
            Message::ProcessTimers(captured_at) => self.handle_process_timers(captured_at),
            msg => {
                self.dispatch_simple_message(msg);
                Task::none()
            }
        };
        Task::batch([task, ipc_task])
    }

    /// Handle messages that always return `Task::none()`.
    fn dispatch_simple_message(&mut self, message: Message) {
        match message {
            Message::FireEvent(event) => self.handle_fire_event(&event),
            Message::Scroll(dx, dy) => self.handle_scroll(dx, dy),
            Message::ReloadUI => self.handle_reload_ui(),
            Message::CommandInputChanged(input) => self.command_input = input,
            Message::ExecuteCommand => self.handle_execute_command(),
            Message::ScreenshotTaken(ss) => self.handle_screenshot_taken(ss),
            Message::FpsTick => {}
            Message::InspectorClose
            | Message::InspectorWidthChanged(_)
            | Message::InspectorHeightChanged(_)
            | Message::InspectorAlphaChanged(_)
            | Message::InspectorLevelChanged(_)
            | Message::InspectorVisibleToggled(_)
            | Message::InspectorMouseEnabledToggled(_)
            | Message::InspectorApply => self.handle_inspector_message(message),
            Message::ToggleFramesPanel => {
                self.frames_panel_collapsed = !self.frames_panel_collapsed
            }
            Message::XpLevelChanged(ref label) => self.handle_xp_level_changed(label),
            Message::PartySizeChanged(ref label) => self.handle_party_size_changed(label),
            Message::KeyPress(ref key, ref text, captured_at) => {
                self.handle_simple_key_press(key, text.as_deref(), captured_at)
            }
            Message::PlayerClassChanged(ref name) => self.handle_player_class_changed(name),
            Message::PlayerRaceChanged(ref name) => self.handle_player_race_changed(name),
            Message::RotDamageLevelChanged(ref label) => {
                self.handle_rot_damage_level_changed(label)
            }
            Message::ToggleOptionsModal => self.options_modal_visible = !self.options_modal_visible,
            Message::CloseOptionsModal => self.options_modal_visible = false,
            Message::MovementToggled(field, val) => self.handle_movement_toggled(field, val),
            // Handled in update() directly:
            Message::CanvasEvent(_) | Message::ProcessTimers(_) => unreachable!(),
        }
    }

    fn handle_inspector_message(&mut self, message: Message) {
        match message {
            Message::InspectorClose => self.handle_inspector_close(),
            Message::InspectorWidthChanged(v) => self.inspector_state.width = v,
            Message::InspectorHeightChanged(v) => self.inspector_state.height = v,
            Message::InspectorAlphaChanged(v) => self.inspector_state.alpha = v,
            Message::InspectorLevelChanged(v) => self.inspector_state.frame_level = v,
            Message::InspectorVisibleToggled(v) => self.inspector_state.visible = v,
            Message::InspectorMouseEnabledToggled(v) => self.inspector_state.mouse_enabled = v,
            Message::InspectorApply => self.handle_inspector_apply(),
            _ => unreachable!(),
        }
    }

    fn handle_simple_key_press(&mut self, key: &str, text: Option<&str>, captured_at: Instant) {
        let (phase, phase_elapsed) = crate::logging::blocking_phase_snapshot();
        let dropped_stale_ticks = self.dropped_stale_timer_ticks.take();
        let oldest_stale_tick_age = self.oldest_dropped_timer_tick_age.take();
        let mut message = format!(
            "[key] {key} reached app in {:.1?} (last phase={phase} for {:.1?})",
            captured_at.elapsed(),
            phase_elapsed
        );
        if dropped_stale_ticks > 0 {
            message.push_str(&format!(
                " after dropping {dropped_stale_ticks} stale timer ticks (oldest {:.1?})",
                oldest_stale_tick_age
            ));
        }
        crate::logging::eprintln_elapsed(&message);
        if key == "ESCAPE" && self.options_modal_visible {
            self.options_modal_visible = false;
            return;
        }

        self.handle_key_press(key, text, captured_at);
    }

    // ── Event handlers ──────────────────────────────────────────────────

    fn handle_fire_event(&mut self, event: &str) {
        {
            let env = self.env.borrow();
            if let Err(e) = env.fire_event(event) {
                self.log_messages.push(format!("Event error: {}", e));
            } else {
                self.log_messages.push(format!("Fired: {}", event));
            }
        }
        self.invalidate_after_lua_mutation();
    }

    fn handle_canvas_event(&mut self, canvas_msg: CanvasMessage) -> Task<Message> {
        match canvas_msg {
            CanvasMessage::MouseMove(pos) => self.handle_mouse_move(pos),
            CanvasMessage::MouseLeave => self.handle_mouse_leave(),
            CanvasMessage::MouseDown(pos) => self.handle_mouse_down(pos),
            CanvasMessage::MouseUp(pos) => self.handle_mouse_up(pos),
            CanvasMessage::RightMouseDown(pos) => self.handle_right_mouse_down(pos),
            CanvasMessage::RightMouseUp(pos) => self.handle_right_mouse_up(pos),
            CanvasMessage::MiddleClick(pos) => self.handle_middle_click(pos),
        }
        if self.strata_dirty.get() != 0 || self.textures_pending.get() {
            request_redraw_task()
        } else {
            Task::none()
        }
    }

    pub(super) fn handle_key_press(&mut self, key: &str, text: Option<&str>, captured_at: Instant) {
        let dispatch_started = Instant::now();
        let env = self.env.borrow();
        if let Err(e) = env.send_key_press(key, text) {
            self.log_messages
                .push(format!("KeyPress({}) error: {}", key, e));
        }
        drop(env);
        crate::logging::eprintln_elapsed(&format!(
            "[key] {key} app->lua dispatch took {:.1?} ({:.1?} since capture)",
            dispatch_started.elapsed(),
            captured_at.elapsed()
        ));
        self.invalidate_after_lua_mutation();
    }

    fn handle_xp_level_changed(&mut self, label: &str) {
        use crate::lua_api::state::XP_LEVELS;
        self.selected_xp_level = label.to_string();
        let fraction = XP_LEVELS
            .iter()
            .find(|(l, _)| *l == label)
            .map(|(_, f)| *f)
            .unwrap_or(0.0);
        let at_max = fraction == 0.0;
        let event = if at_max {
            "DISABLE_XP_GAIN"
        } else {
            "ENABLE_XP_GAIN"
        };
        {
            let env = self.env.borrow();
            let xp_max = 89_750i32;
            let xp_current = (xp_max as f64 * fraction) as i32;
            let lua_code = format!(
                "IsPlayerAtEffectiveMaxLevel = function() return {} end; \
                 UnitXP = function() return {} end; \
                 UnitXPMax = function() return {} end",
                at_max, xp_current, xp_max
            );
            if let Err(e) = env.exec(&lua_code) {
                self.log_messages.push(format!("XP level error: {}", e));
            }
            if let Err(e) = env.fire_event(event) {
                self.log_messages.push(format!("XP event error: {}", e));
            }
        }
        self.save_config();
        self.invalidate();
    }

    fn handle_party_size_changed(&mut self, label: &str) {
        let size = label.parse::<usize>().unwrap_or(0).min(4);
        self.selected_party_size = size.to_string();
        {
            let env = self.env.borrow();
            let mut state = env.state().borrow_mut();
            super::app::resize_party_state(&mut state, size);
            drop(state);
            crate::startup::refresh_party_frames(&env);
        }
        self.save_config();
        self.invalidate();
    }

    fn handle_player_class_changed(&mut self, class_name: &str) {
        use crate::lua_api::state::CLASS_LABELS;
        let index = CLASS_LABELS
            .iter()
            .position(|&n| n == class_name)
            .map(|i| (i + 1) as i32)
            .unwrap_or(1);
        self.selected_class = class_name.to_string();
        {
            let env = self.env.borrow();
            env.state().borrow_mut().player.class_index = index;
            self.fire_portrait_update(&env);
        }
        self.save_config();
        self.invalidate();
    }

    fn handle_player_race_changed(&mut self, race_name: &str) {
        use crate::lua_api::state::RACE_DATA;
        let index = RACE_DATA
            .iter()
            .position(|(name, _, _)| *name == race_name)
            .unwrap_or(0);
        self.selected_race = race_name.to_string();
        {
            let env = self.env.borrow();
            env.state().borrow_mut().player.race_index = index;
            self.fire_portrait_update(&env);
        }
        self.save_config();
        self.invalidate();
    }

    fn handle_rot_damage_level_changed(&mut self, label: &str) {
        use crate::lua_api::state::ROT_DAMAGE_LEVELS;
        let index = ROT_DAMAGE_LEVELS
            .iter()
            .position(|(l, _)| *l == label)
            .unwrap_or(0);
        self.selected_rot_level = label.to_string();
        self.env.borrow().state().borrow_mut().rot_damage_level = index;
        self.save_config();
    }

    fn handle_movement_toggled(&mut self, field: &str, val: bool) {
        match field {
            "moving" => self.movement.moving = val,
            "mounted" => self.movement.mounted = val,
            "flying" => self.movement.flying = val,
            "falling" => self.movement.falling = val,
            "swimming" => self.movement.swimming = val,
            _ => return,
        }
        let env = self.env.borrow();
        let mut state = env.state().borrow_mut();
        match field {
            "moving" => state.player.movement.moving = val,
            "mounted" => state.player.movement.mounted = val,
            "flying" => state.player.movement.flying = val,
            "falling" => state.player.movement.falling = val,
            "swimming" => state.player.movement.swimming = val,
            _ => {}
        }
        drop(state);
        drop(env);
        self.save_config();
    }

    fn fire_portrait_update(&self, env: &WowLuaEnv) {
        let _ = env.fire_event_with_args("UNIT_PORTRAIT_UPDATE", &[env.lua_string("player")]);
        let _ = env.fire_event_with_args(
            "PLAYER_ENTERING_WORLD",
            &[Val::Bool(false), Val::Bool(false)],
        );
    }

    fn handle_reload_ui(&mut self) {
        self.log_messages.push("Reloading UI...".to_string());
        {
            let env = self.env.borrow();
            let _ = env.fire_event_with_args("ADDON_LOADED", &[env.lua_string("WoWUISim")]);
            let _ = env.fire_event("VARIABLES_LOADED");
            let _ = env.fire_event_with_args(
                "PLAYER_ENTERING_WORLD",
                &[Val::Bool(false), Val::Bool(true)],
            );
            let _ = env.fire_event("UPDATE_BINDINGS");
            let _ = env.fire_event("DISPLAY_SIZE_CHANGED");
            let _ = env.fire_event("UI_SCALE_CHANGED");
        }
        self.drain_console();
        self.log_messages.push("UI reloaded.".to_string());
        self.mark_all_strata_dirty();
    }

    fn handle_execute_command(&mut self) {
        let cmd = self.command_input.clone();
        if cmd.is_empty() {
            return;
        }
        self.log_messages.push(format!("> {}", cmd));
        self.execute_command_inner(&cmd);
        self.command_input.clear();
        self.invalidate_after_lua_mutation();
    }

    fn execute_command_inner(&mut self, cmd: &str) {
        let cmd_lower = cmd.to_lowercase();
        if cmd_lower == "/frames" || cmd_lower == "/f" {
            let env = self.env.borrow();
            let dump = env.dump_frames();
            eprintln!("{}", dump);
            let line_count = dump.lines().count();
            self.log_messages
                .push(format!("Dumped {} frames to stderr", line_count / 2));
        } else {
            let env = self.env.borrow();
            match env.dispatch_slash_command(cmd) {
                Ok(true) => {}
                Ok(false) => self.log_messages.push(format!("Unknown command: {}", cmd)),
                Err(e) => self.log_messages.push(format!("Command error: {}", e)),
            }
        }
    }

    fn handle_process_timers(&mut self, captured_at: Instant) -> Task<Message> {
        let age = captured_at.elapsed();
        let interval = self
            .compute_tick_interval()
            .unwrap_or(std::time::Duration::from_secs(1));
        if should_drop_stale_timer_tick(age, interval) {
            let dropped = self.dropped_stale_timer_ticks.get().saturating_add(1);
            self.dropped_stale_timer_ticks.set(dropped);
            let oldest = self.oldest_dropped_timer_tick_age.get().max(age);
            self.oldest_dropped_timer_tick_age.set(oldest);
            return Task::none();
        }
        self.set_main_thread_phase("process_timers");
        let tick_started = std::time::Instant::now();
        let mut stage_timings = TickStageTimings::default();
        let mut redraw_needed = false;
        let queued_preloads_pending = self.has_queued_texture_preloads();

        if !queued_preloads_pending && !self.textures_pending.get() {
            let started = std::time::Instant::now();
            redraw_needed |=
                self.preload_visible_textures_with_budget(std::time::Duration::from_millis(10));
            stage_timings.preload += started.elapsed();
        }

        let started = std::time::Instant::now();
        self.run_pending_exec_lua();
        stage_timings.exec_lua = started.elapsed();
        redraw_needed |= self.strata_dirty.get() != 0;

        let (combined, collect_timings) = self.collect_tick_dirty();
        stage_timings.timers = collect_timings.timers;
        stage_timings.layout = collect_timings.layout;
        stage_timings.on_update = collect_timings.on_update;
        stage_timings.party_health = collect_timings.party_health;
        stage_timings.casting = collect_timings.casting;

        let started = std::time::Instant::now();
        self.drain_console();
        stage_timings.console = started.elapsed();
        if combined != 0 {
            let started = std::time::Instant::now();
            self.mark_strata_dirty(combined);
            stage_timings.mark_dirty = started.elapsed();
            redraw_needed = true;
        }
        if self.strata_dirty.get() != 0 || self.textures_pending.get() || queued_preloads_pending {
            let started = std::time::Instant::now();
            redraw_needed |= self.preload_current_render_requests_preserving_dirty(Some(
                std::time::Duration::from_millis(25),
            ));
            stage_timings.preload += started.elapsed();
        }
        let tick_elapsed = tick_started.elapsed();
        stage_timings.total = tick_elapsed;
        self.record_tick_time(tick_elapsed);
        self.update_fps_counter();
        log_slow_tick(&stage_timings, combined, self);
        if crate::logging::gui_trace_enabled() {
            let ready_count = self.cached_render_request_ready_count();
            crate::logging::eprintln_gui_trace(&format!(
                "tick redraw_request={} dirty=0x{:x} pending={} ready={ready_count}",
                redraw_needed,
                self.strata_dirty.get(),
                self.textures_pending.get()
            ));
        }
        if redraw_needed {
            request_redraw_task()
        } else {
            Task::none()
        }
    }

    /// Run timers, layout, OnUpdate, health/casting and collect dirty mask + IDs.
    fn collect_tick_dirty(&mut self) -> (u16, TickStageTimings) {
        let mut timings = TickStageTimings::default();
        let pending_before = self.pending_dirty_ids.borrow_mut().take();
        let (m0, ids0) = self.take_render_dirty_with_ids();

        let started = std::time::Instant::now();
        self.run_wow_timers();
        timings.timers = started.elapsed();
        let (m1, ids1) = self.take_render_dirty_with_ids();

        let t_layout = std::time::Instant::now();
        self.env.borrow().state().borrow_mut().ensure_layout_rects();
        timings.layout = t_layout.elapsed();

        timings.on_update = self.fire_on_update();
        let (m2, ids2) = self.take_render_dirty_with_ids();

        let started = std::time::Instant::now();
        self.tick_party_health();
        timings.party_health = started.elapsed();

        let started = std::time::Instant::now();
        self.tick_casting();
        timings.casting = started.elapsed();
        let (m3, ids3) = self.take_render_dirty_with_ids();

        *self.pending_dirty_ids.borrow_mut() =
            merge_dirty_ids([pending_before, ids0, ids1, ids2, ids3]);
        (m0 | m1 | m2 | m3, timings)
    }

    fn take_render_dirty_with_ids(&self) -> (u16, Option<FxHashSet<u64>>) {
        self.env
            .borrow()
            .state()
            .borrow()
            .widgets
            .take_render_dirty_with_ids()
    }

    pub(super) fn merge_pending_dirty_ids(&self, ids: Option<FxHashSet<u64>>) {
        let current = self.pending_dirty_ids.borrow_mut().take();
        *self.pending_dirty_ids.borrow_mut() = merge_dirty_ids([current, ids]);
    }

    fn update_fps_counter(&mut self) {
        let now = std::time::Instant::now();
        let elapsed = now.duration_since(self.fps_last_time);
        if elapsed >= std::time::Duration::from_secs(1) {
            let frames = self.frame_count.get();
            let metrics = sample_display_metrics(
                elapsed,
                frames,
                self.tick_count.get(),
                self.draw_time_accum_ms.get(),
                self.tick_time_accum_ms.get(),
            );
            self.fps = metrics.fps;
            self.tick_time_display = metrics.tick_ms;
            self.draw_time_display = metrics.draw_ms;
            self.other_time_display = metrics.other_ms;
            self.frame_count.set(0);
            self.draw_time_accum_ms.set(0.0);
            self.tick_time_accum_ms.set(0.0);
            self.tick_count.set(0);
            self.fps_last_time = now;
            let env = self.env.borrow();
            env.state().borrow_mut().fps = self.fps;
        }
    }

    fn record_tick_time(&self, elapsed: std::time::Duration) {
        let elapsed_ms = elapsed.as_secs_f32() * 1000.0;
        self.tick_time_accum_ms
            .set(self.tick_time_accum_ms.get() + elapsed_ms);
        self.tick_count.set(self.tick_count.get().saturating_add(1));
    }

    fn run_wow_timers(&self) {
        let env = self.env.borrow();
        if let Err(e) = env.process_timers() {
            eprintln!("Timer error: {}", e);
        }
    }

    fn fire_on_update(&mut self) -> crate::lua_api::on_update::OnUpdateStageTimings {
        let now = std::time::Instant::now();
        let elapsed = now.duration_since(self.last_on_update_time);
        if elapsed.as_millis() < 16 {
            return crate::lua_api::on_update::OnUpdateStageTimings::default();
        }
        self.last_on_update_time = now;
        let env = self.env.borrow();
        match env.fire_on_update_timed(elapsed.as_secs_f64()) {
            Ok(timings) => {
                self.last_on_update_time = std::time::Instant::now();
                timings
            }
            Err(e) => {
                crate::logging::eprintln_elapsed(&format!("[OnUpdate] error: {e}"));
                self.last_on_update_time = std::time::Instant::now();
                crate::lua_api::on_update::OnUpdateStageTimings::default()
            }
        }
    }

    fn tick_party_health(&mut self) {
        if self.selected_rot_level == "Off" {
            return;
        }
        let now = std::time::Instant::now();
        if now.duration_since(self.last_party_health_tick) < std::time::Duration::from_secs(2) {
            return;
        }
        self.last_party_health_tick = now;
        let env = self.env.borrow();
        let changed = {
            let mut state = env.state().borrow_mut();
            let (_, pct) = crate::lua_api::state::ROT_DAMAGE_LEVELS
                .get(state.rot_damage_level)
                .copied()
                .unwrap_or(("Light (1%)", 0.01));
            crate::lua_api::tick_party_health(&mut state.party_members, pct)
        };
        for idx in changed {
            let unit_id = format!("party{idx}");
            let _ = env.fire_event_with_args("UNIT_HEALTH", &[env.lua_string(&unit_id)]);
        }
    }

    fn tick_casting(&mut self) {
        let env = self.env.borrow();
        let completed = super::casting::extract_completed_cast(env.state());
        if let Some((cast_id, spell_id)) = completed {
            super::casting::fire_cast_complete_events(&env, cast_id, spell_id);
            super::casting::apply_spell_effect(env.state(), &env, spell_id);
            super::casting::apply_spec_change(env.state(), &env);
        }
    }

    fn run_pending_exec_lua(&mut self) {
        if let Some((code, secure)) = self.pending_exec_lua.take() {
            eprintln!(
                "[exec-lua{}] Running: {}",
                if secure { "-secure" } else { "" },
                code
            );
            let env = self.env.borrow();
            if let Err(e) = env.exec_maybe_secure(&code, secure) {
                eprintln!("[exec-lua] Error: {}", e);
            }
            drop(env);
            self.invalidate();
        }
    }

    fn handle_screenshot_taken(&mut self, screenshot: iced::window::screenshot::Screenshot) {
        if let Some(respond) = self.pending_screenshot.take() {
            let data = ScreenshotData {
                width: screenshot.size.width,
                height: screenshot.size.height,
                pixels: screenshot.rgba.to_vec(),
            };
            let _ = respond.send(Ok(data));
        }
    }

    fn handle_inspector_close(&mut self) {
        self.inspector_visible = false;
        self.inspected_frame = None;
    }

    fn handle_inspector_apply(&mut self) {
        if let Some(frame_id) = self.inspected_frame {
            self.apply_inspector_changes(frame_id);
            self.mark_all_strata_dirty();
        }
    }

    // ── Helpers ──────────────────────────────────────────────────────────

    fn save_config(&self) {
        let mut config = crate::config::SimConfig::load();
        config.player_class = self.selected_class.clone();
        config.player_race = self.selected_race.clone();
        config.rot_damage_level = self.selected_rot_level.clone();
        config.xp_level = self.selected_xp_level.clone();
        config.party_size = self.selected_party_size.parse::<u8>().unwrap_or(0).min(4);
        config.movement = self.movement.clone();
        config.save();
    }

    pub(super) fn flush_post_script_updates(&mut self) {
        self.env.borrow().state().borrow_mut().ensure_layout_rects();
        self.fire_on_update();
        self.invalidate_after_lua_mutation();
    }

    pub(super) fn invalidate_after_lua_mutation(&mut self) {
        self.drain_console();
        if !self.has_queued_texture_preloads() && !self.textures_pending.get() {
            self.preload_visible_textures();
        }
        self.clear_failed_texture_requests();
        self.merge_widget_dirty_into_render_state();
        self.preload_current_render_requests_preserving_dirty(Some(
            std::time::Duration::from_millis(25),
        ));
    }

    fn merge_widget_dirty_into_render_state(&self) {
        let (dirty_mask, dirty_ids) = self.take_render_dirty_with_ids();
        if dirty_mask == 0 {
            return;
        }
        self.mark_strata_dirty(dirty_mask);
        self.merge_pending_dirty_ids(dirty_ids);
    }

    pub(super) fn invalidate(&mut self) {
        self.drain_console();
        if !self.has_queued_texture_preloads() && !self.textures_pending.get() {
            self.preload_visible_textures();
        }
        self.clear_failed_texture_requests();
        self.mark_all_strata_dirty();
        self.preload_current_render_requests_preserving_dirty(Some(
            std::time::Duration::from_millis(25),
        ));
    }

    fn preload_visible_textures(&self) {
        let _ = self.preload_visible_textures_with_budget(std::time::Duration::from_millis(50));
    }

    fn preload_visible_textures_with_budget(&self, budget: std::time::Duration) -> bool {
        let paths = self.warmup_texture_paths();
        let deadline = std::time::Instant::now() + budget;
        let pending_before = self.textures_pending.get();
        let mut loaded = 0usize;
        {
            let mut tex_mgr = self.texture_manager.borrow_mut();
            for path in &paths {
                if texture_request_is_cached(&tex_mgr, path) {
                    continue;
                }
                super::render::preload_texture_request_source(&mut tex_mgr, path);
                if texture_request_is_cached(&tex_mgr, path) {
                    loaded += 1;
                }
                if std::time::Instant::now() >= deadline {
                    break;
                }
            }
        }
        let remaining_pending = if !self.cached_render_request_paths().is_empty() {
            self.cached_render_requests_still_pending()
        } else {
            let tex_mgr = self.texture_manager.borrow();
            let cpu_pending = paths
                .iter()
                .any(|path| !texture_request_is_cached(&tex_mgr, path));
            cpu_pending || loaded != 0 || (pending_before && !paths.is_empty())
        };
        self.textures_pending.set(remaining_pending);
        if loaded > 0 {
            crate::logging::eprintln_elapsed(&format!(
                "[texture-cache-warmup] {loaded} new textures ({} requested)",
                paths.len()
            ));
        }
        loaded != 0 || (!pending_before && remaining_pending)
    }

    fn warmup_texture_paths(&self) -> Vec<String> {
        let mut cached_paths = self.cached_render_request_paths();
        if !cached_paths.is_empty() {
            Self::sort_texture_request_paths(&mut cached_paths);
            return cached_paths;
        }

        let env = self.env.borrow();
        let mut visible_paths = env.state().borrow().widgets.visible_texture_paths();
        Self::sort_texture_request_paths(&mut visible_paths);
        visible_paths
    }

    fn has_queued_texture_preloads(&self) -> bool {
        let env = self.env.borrow();
        !env.state().borrow().pending_texture_preloads.is_empty()
    }

    fn cached_render_request_paths(&self) -> Vec<String> {
        let mut paths = FxHashSet::default();
        let strata = self.cached_strata_quads.borrow();
        for batch in strata.iter().flatten() {
            for request in batch
                .texture_requests
                .iter()
                .chain(&batch.mask_texture_requests)
            {
                paths.insert(request.path.clone());
            }
        }
        paths.into_iter().collect()
    }

    pub(super) fn apply_hit_grid_changes(&self) {
        let env = self.env.borrow();
        let mut state = env.state().borrow_mut();
        let changes = std::mem::take(&mut state.pending_hit_grid_changes);
        if changes.is_empty() {
            return;
        }
        drop(state);

        let mut grid_ref = self.cached_hittable.borrow_mut();
        let Some(grid) = grid_ref.as_mut() else {
            return;
        };
        let state = env.state().borrow();
        for (root_id, became_visible) in changes {
            apply_subtree_hit_grid_change(grid, &state.widgets, root_id, became_visible);
        }
    }

    pub(super) fn is_frame_enabled(&self, frame_id: u64) -> bool {
        let env = self.env.borrow();
        let state = env.state().borrow();
        state
            .widgets
            .get(frame_id)
            .and_then(|f| f.attributes.get("__enabled"))
            .and_then(|v| match v {
                crate::widget::AttributeValue::Boolean(b) => Some(*b),
                _ => None,
            })
            .unwrap_or(true)
    }

    pub(super) fn update_editbox_focus(&self, clicked_frame: Option<u64>) {
        let env = self.env.borrow();
        let editbox_target = env.resolve_editbox_focus_target(clicked_frame);
        let old_focus = env.state().borrow().focused_frame_id;

        if let Some(fid) = editbox_target {
            if old_focus != Some(fid) {
                {
                    let mut state = env.state().borrow_mut();
                    state.focused_frame_id = Some(fid);
                    if let Some(old_id) = old_focus {
                        if let Some(f) = state.widgets.get_mut_visual(old_id) {
                            f.editbox_focused = false;
                        }
                        state.widgets.mark_visual_dirty(old_id);
                    }
                    if let Some(f) = state.widgets.get_mut_visual(fid) {
                        f.editbox_focused = true;
                    }
                    state.widgets.mark_visual_dirty(fid);
                }
                if let Some(old_id) = old_focus {
                    let _ = env.fire_script_handler(old_id, "OnEditFocusLost", vec![]);
                }
                let _ = env.fire_script_handler(fid, "OnEditFocusGained", vec![]);
            }
        } else if let Some(old_id) = old_focus {
            {
                let mut state = env.state().borrow_mut();
                state.focused_frame_id = None;
                if let Some(f) = state.widgets.get_mut_visual(old_id) {
                    f.editbox_focused = false;
                }
                state.widgets.mark_visual_dirty(old_id);
            }
            let _ = env.fire_script_handler(old_id, "OnEditFocusLost", vec![]);
        }
    }

    pub(super) fn toggle_checkbutton_if_needed(&self, frame_id: u64, env: &WowLuaEnv) {
        let mut state = env.state().borrow_mut();
        if !is_toggleable_checkbutton(&state, frame_id) {
            return;
        }
        let new_checked = !get_checked_attribute(&state, frame_id);
        if let Some(frame) = state.widgets.get_mut(frame_id) {
            frame.attributes.insert(
                "__checked".to_string(),
                crate::widget::AttributeValue::Boolean(new_checked),
            );
        }
        let tex_id = state
            .widgets
            .get(frame_id)
            .and_then(|f| f.children_keys.get("CheckedTexture").copied());
        if let Some(tex_id) = tex_id {
            state.set_frame_visible(tex_id, new_checked);
        }
    }

    pub(crate) fn sync_screen_size_to_state(&self, size: iced::Size) {
        let env = self.env.borrow();
        let state = env.state().borrow();
        if (state.screen_width - size.width).abs() > 0.5
            || (state.screen_height - size.height).abs() > 0.5
        {
            crate::logging::println_elapsed(&format!(
                "Window size: {}x{} (was {}x{})",
                size.width as i32,
                size.height as i32,
                state.screen_width as i32,
                state.screen_height as i32
            ));
            drop(state);
            env.set_screen_size(size.width, size.height);
        }
    }

    pub(crate) fn drain_console(&mut self) {
        let env = self.env.borrow();
        let mut state = env.state().borrow_mut();
        self.log_messages.append(&mut state.console_output);
    }

    fn sort_texture_request_paths(paths: &mut [String]) {
        paths.sort_by(|a, b| {
            texture_request_priority(a)
                .cmp(&texture_request_priority(b))
                .then_with(|| a.cmp(b))
        });
    }

    fn cached_render_request_ready_count(&self) -> usize {
        self.cached_strata_quads
            .borrow()
            .iter()
            .flatten()
            .map(|batch| {
                batch
                    .texture_requests
                    .iter()
                    .chain(&batch.mask_texture_requests)
                    .filter(|request| request.handle.is_ready())
                    .count()
            })
            .sum()
    }

    fn clear_failed_texture_requests(&self) {
        let mut retried = false;
        for batch in self.cached_strata_quads.borrow().iter().flatten() {
            for request in batch
                .texture_requests
                .iter()
                .chain(&batch.mask_texture_requests)
            {
                if request.handle.is_failed() {
                    request.handle.mark_retry();
                    retried = true;
                }
            }
        }
        if retried {
            self.seed_pending_texture_paths_from_cached_strata();
        }
    }
}

#[derive(Debug, Clone, Copy, Default, PartialEq)]
struct TickStageTimings {
    total: Duration,
    exec_lua: Duration,
    timers: Duration,
    layout: Duration,
    on_update: crate::lua_api::on_update::OnUpdateStageTimings,
    party_health: Duration,
    casting: Duration,
    console: Duration,
    mark_dirty: Duration,
    preload: Duration,
}

fn log_slow_tick(stage_timings: &TickStageTimings, combined: u16, app: &App) {
    let atlas_ready = app.cached_render_request_ready_count();
    if super::perf_logging_enabled() && stage_timings.total.as_millis() > 10 {
        let n = app.pending_dirty_ids.borrow().as_ref().map(|s| s.len());
        eprintln!(
            "[tick] {:.1?} (layout={:.1?} dirty=0x{combined:x} ids={n:?} pending={})",
            stage_timings.total,
            stage_timings.layout,
            app.textures_pending.get()
        );
    }
    if tick_stage_logging_enabled() {
        let n = app.pending_dirty_ids.borrow().as_ref().map(|s| s.len());
        eprintln!(
            "{}",
            format_tick_stage_log(
                stage_timings,
                combined,
                n,
                app.textures_pending.get(),
                atlas_ready,
            )
        );
    }
}

fn tick_stage_logging_enabled() -> bool {
    static ENABLED: OnceLock<bool> = OnceLock::new();
    *ENABLED.get_or_init(|| std::env::var_os("WOW_SIM_LOG_TICK_STAGES").is_some())
}

fn format_tick_stage_log(
    stage_timings: &TickStageTimings,
    combined: u16,
    dirty_ids: Option<usize>,
    textures_pending: bool,
    atlas_ready: usize,
) -> String {
    format!(
        "[tick-stage] total={:.3}ms exec_lua={:.3} timers={:.3} layout={:.3} on_update={:.3} handlers={:.3} anim={:.3} post={:.3} metrics={:.3} gc={:.3} party={:.3} casting={:.3} console={:.3} mark={:.3} preload={:.3} dirty=0x{combined:x} ids={dirty_ids:?} pending={textures_pending} ready={atlas_ready}",
        duration_ms(stage_timings.total),
        duration_ms(stage_timings.exec_lua),
        duration_ms(stage_timings.timers),
        duration_ms(stage_timings.layout),
        duration_ms(stage_timings.on_update.total),
        duration_ms(stage_timings.on_update.dispatch_handlers),
        duration_ms(stage_timings.on_update.animation_groups),
        duration_ms(stage_timings.on_update.on_post_update),
        duration_ms(stage_timings.on_update.finalize_metrics),
        duration_ms(stage_timings.on_update.gc_step),
        duration_ms(stage_timings.party_health),
        duration_ms(stage_timings.casting),
        duration_ms(stage_timings.console),
        duration_ms(stage_timings.mark_dirty),
        duration_ms(stage_timings.preload),
    )
}

fn duration_ms(duration: Duration) -> f64 {
    duration.as_secs_f64() * 1000.0
}

fn texture_request_is_cached(tex_mgr: &crate::texture::TextureManager, path: &str) -> bool {
    if path.contains("@crop:") {
        return tex_mgr.get_cached_crop_request(path).is_some();
    }
    tex_mgr.is_cached(path)
}

fn texture_request_priority(path: &str) -> (u8, u8) {
    let is_world_map = path
        .get(..19)
        .is_some_and(|prefix| prefix.eq_ignore_ascii_case("Interface\\WorldMap\\"))
        || path
            .get(..19)
            .is_some_and(|prefix| prefix.eq_ignore_ascii_case("Interface/WorldMap/"));
    let is_crop = path.contains("@crop:");
    (u8::from(!is_world_map), u8::from(is_crop))
}

fn should_drop_stale_timer_tick(age: std::time::Duration, interval: std::time::Duration) -> bool {
    let stale_threshold = interval
        .saturating_mul(2)
        .max(std::time::Duration::from_millis(100));
    age > stale_threshold
}

#[derive(Debug, Clone, Copy, PartialEq)]
struct DisplayMetrics {
    fps: f32,
    tick_ms: f32,
    draw_ms: f32,
    other_ms: f32,
}

fn sample_display_metrics(
    elapsed: std::time::Duration,
    frames: u32,
    ticks: u32,
    draw_total_ms: f32,
    tick_total_ms: f32,
) -> DisplayMetrics {
    let elapsed_secs = elapsed.as_secs_f32();
    let fps = if elapsed_secs > 0.0 {
        frames as f32 / elapsed_secs
    } else {
        0.0
    };
    let draw_denominator = frames.max(1) as f32;
    let tick_denominator = if frames > 0 {
        draw_denominator
    } else {
        ticks.max(1) as f32
    };
    let frame_budget_ms = elapsed.as_secs_f32() * 1000.0 / draw_denominator;
    let draw_ms = draw_total_ms / draw_denominator;
    let tick_ms = tick_total_ms / tick_denominator;
    let other_ms = (frame_budget_ms - draw_ms - tick_ms).max(0.0);
    DisplayMetrics {
        fps,
        tick_ms,
        draw_ms,
        other_ms,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::iced_app::app::AppInit;
    use crate::iced_app::render;
    use crate::render::{FrameQuadSnapshot, QuadBatch, TextureRequest};
    use crate::screen::ScreenKind;
    use crate::texture::TextureManager;
    use iced::Size;
    use iced_runtime::Action;
    use iced_runtime::futures::futures::StreamExt;
    use iced_runtime::window::Action as WindowAction;
    use std::cell::RefCell;
    use std::collections::HashMap;
    use std::rc::Rc;
    use tempfile::tempdir;
    use tokio::sync::mpsc;

    fn build_test_app(screen_kind: ScreenKind) -> App {
        let env = Rc::new(RefCell::new(
            WowLuaEnv::new().expect("Failed to create Lua environment"),
        ));
        env.borrow().set_screen_mode(screen_kind);

        let texture_manager = Rc::new(RefCell::new(TextureManager::new()));
        let font_system = Rc::new(RefCell::new(crate::render::WowFontSystem::new()));
        let glyph_atlas = Rc::new(RefCell::new(crate::render::GlyphAtlas::new()));
        let (_cmd_tx, cmd_rx) = mpsc::channel(1);
        let (_lua_tx, lua_rx) = std::sync::mpsc::channel();

        App::build_app(AppInit {
            env,
            log_messages: Vec::new(),
            texture_manager,
            font_system,
            glyph_atlas,
            cmd_rx,
            lua_rx,
            debug_borders: false,
            debug_anchors: false,
            saved_vars: None,
            config: crate::config::SimConfig::default(),
        })
    }

    #[test]
    fn format_tick_stage_log_includes_nested_on_update_breakdown() {
        let log = format_tick_stage_log(
            &TickStageTimings {
                total: Duration::from_millis(100),
                exec_lua: Duration::from_millis(1),
                timers: Duration::from_millis(2),
                layout: Duration::from_millis(30),
                on_update: crate::lua_api::on_update::OnUpdateStageTimings {
                    total: Duration::from_millis(40),
                    dispatch_handlers: Duration::from_millis(10),
                    animation_groups: Duration::from_millis(11),
                    on_post_update: Duration::from_millis(12),
                    finalize_metrics: Duration::from_millis(3),
                    gc_step: Duration::from_millis(4),
                },
                party_health: Duration::from_millis(5),
                casting: Duration::from_millis(6),
                console: Duration::from_millis(7),
                mark_dirty: Duration::from_millis(8),
                preload: Duration::from_millis(9),
            },
            0x3,
            Some(4),
            true,
            42,
        );

        assert!(log.contains("total=100.000ms"));
        assert!(log.contains("layout=30.000"));
        assert!(log.contains("on_update=40.000"));
        assert!(log.contains("handlers=10.000"));
        assert!(log.contains("anim=11.000"));
        assert!(log.contains("post=12.000"));
        assert!(log.contains("metrics=3.000"));
        assert!(log.contains("gc=4.000"));
        assert!(log.contains("preload=9.000"));
        assert!(log.contains("dirty=0x3"));
        assert!(log.contains("ids=Some(4)"));
        assert!(log.contains("pending=true"));
        assert!(log.contains("ready=42"));
    }

    #[test]
    fn collect_tick_dirty_preserves_preexisting_dirty_ids() {
        let mut app = build_test_app(ScreenKind::Game);
        app.strata_dirty.set(0);
        app.selected_rot_level = "Off".to_string();
        app.screen_size.set(Size::new(1024.0, 768.0));
        *app.pending_dirty_ids.borrow_mut() = Some(FxHashSet::default());

        {
            let env = app.env.borrow();
            env.exec("TickDirtyFrame = CreateFrame('Frame', 'TickDirtyFrame', UIParent)")
                .unwrap();
        }

        let frame_id = {
            let env = app.env.borrow();
            let state = env.state().borrow();
            state.widgets.get_id_by_name("TickDirtyFrame").unwrap()
        };

        {
            let env = app.env.borrow();
            let state = env.state().borrow();
            let _ = state.widgets.take_render_dirty_with_ids();
            state.widgets.mark_visual_dirty(frame_id);
        }

        let (dirty_mask, _tick_timings) = app.collect_tick_dirty();
        let pending = app.pending_dirty_ids.borrow().clone().unwrap();

        assert_ne!(
            dirty_mask, 0,
            "pre-tick dirty should contribute to the mask"
        );
        assert!(
            pending.contains(&frame_id),
            "pre-tick dirty frame should survive collect_tick_dirty"
        );
    }

    #[test]
    fn invalidate_after_lua_mutation_keeps_incremental_dirty_ids() {
        let mut app = build_test_app(ScreenKind::Game);
        app.strata_dirty.set(0);
        *app.pending_dirty_ids.borrow_mut() = Some(FxHashSet::default());
        app.cached_frame_snapshots.borrow_mut()[0] =
            Some(HashMap::from([(1_u64, FrameQuadSnapshot::default())]));

        let frame_id = {
            let env = app.env.borrow();
            let state = env.state().borrow();
            state
                .widgets
                .get_id_by_name("UIParent")
                .expect("UIParent should exist")
        };

        {
            let env = app.env.borrow();
            let state = env.state().borrow();
            let _ = state.widgets.take_render_dirty_with_ids();
            state.widgets.mark_visual_dirty(frame_id);
        }

        app.invalidate_after_lua_mutation();

        let all_mask = (1u16 << crate::widget::FrameStrata::COUNT) - 1;
        assert_ne!(
            app.strata_dirty.get(),
            0,
            "dirty strata should be preserved"
        );
        assert_ne!(
            app.strata_dirty.get(),
            all_mask,
            "incremental invalidate should not force all strata dirty"
        );
        let pending = app.pending_dirty_ids.borrow();
        let ids = pending
            .as_ref()
            .expect("incremental invalidate should keep concrete dirty IDs");
        assert!(
            ids.contains(&frame_id),
            "pending dirty IDs should include the Lua-mutated frame"
        );
        assert!(
            app.cached_frame_snapshots.borrow()[0].is_some(),
            "incremental invalidate should not clear snapshot caches"
        );
    }

    #[test]
    fn party_size_change_updates_group_state_and_selection() {
        let mut app = build_test_app(ScreenKind::Game);

        app.dispatch_simple_message(Message::PartySizeChanged("4".to_string()));

        assert_eq!(app.selected_party_size, "4");
        let env = app.env.borrow();
        let state = env.state().borrow();
        assert_eq!(state.party_members.len(), 4);
        assert!(
            state.party_group_active,
            "party size > 0 should mark the player as grouped"
        );
        assert_eq!(
            state.party_leader_index, None,
            "GUI party-size changes should default leadership to the local player like A_Admin.SetPartySize"
        );
    }

    #[test]
    fn fast_timer_ticks_go_stale_quickly() {
        assert!(should_drop_stale_timer_tick(
            std::time::Duration::from_millis(150),
            std::time::Duration::from_millis(16),
        ));
    }

    #[test]
    fn slow_timer_ticks_get_more_slack() {
        assert!(!should_drop_stale_timer_tick(
            std::time::Duration::from_millis(150),
            std::time::Duration::from_secs(1),
        ));
        assert!(should_drop_stale_timer_tick(
            std::time::Duration::from_secs(3),
            std::time::Duration::from_secs(1),
        ));
    }

    #[test]
    fn queued_stale_timer_ticks_do_not_run_tick_work() {
        let mut app = build_test_app(ScreenKind::Game);
        app.screen_size.set(Size::new(1024.0, 768.0));
        app.textures_pending.set(true);
        app.selected_rot_level = "Off".to_string();
        let initial_on_update = app.last_on_update_time;
        let stale_captured_at = Instant::now() - std::time::Duration::from_secs(1);

        for _ in 0..8 {
            let _ = app.update(Message::ProcessTimers(stale_captured_at));
        }

        assert_eq!(app.dropped_stale_timer_ticks.get(), 8);
        assert!(
            app.oldest_dropped_timer_tick_age.get() >= std::time::Duration::from_secs(1),
            "stale tick age should track the queued backlog"
        );
        assert_eq!(
            app.last_on_update_time, initial_on_update,
            "stale timer ticks should not reach OnUpdate processing"
        );

        app.options_modal_visible = true;
        app.dispatch_simple_message(Message::KeyPress(
            "ESCAPE".to_string(),
            None,
            Instant::now(),
        ));

        assert!(
            !app.options_modal_visible,
            "escape should still be handled promptly"
        );
        assert_eq!(
            app.dropped_stale_timer_ticks.get(),
            0,
            "keypress log accounting should reset after reporting dropped ticks"
        );
        assert_eq!(
            app.oldest_dropped_timer_tick_age.get(),
            std::time::Duration::ZERO,
            "keypress log accounting should clear the recorded backlog age"
        );
    }

    #[test]
    fn tick_warmup_keeps_draw_pending_when_it_decodes_visible_textures() {
        let temp_dir = tempdir().unwrap();
        let texture_path = temp_dir.path().join("tick-warmup.png");
        let image = image::RgbaImage::from_pixel(4, 4, image::Rgba([0x44, 0x88, 0xcc, 0xff]));
        image.save(&texture_path).unwrap();

        let mut app = build_test_app(ScreenKind::Game);
        app.screen_size.set(Size::new(1024.0, 768.0));
        app.selected_rot_level = "Off".to_string();
        app.strata_dirty.set(0);
        app.textures_pending.set(false);

        {
            let env = app.env.borrow();
            env.exec(
                r#"
                local frame = CreateFrame("Frame", "TickWarmupFrame", UIParent)
                local texture = frame:CreateTexture(nil, "ARTWORK")
                texture:SetTexture("tick-warmup")
            "#,
            )
            .unwrap();
            let _ = env.state().borrow().widgets.take_render_dirty_with_ids();
        }

        let task = app.handle_process_timers(Instant::now());
        let action = pollster::block_on(async {
            iced_runtime::task::into_stream(task)
                .expect("decode progress should request a redraw")
                .next()
                .await
                .expect("task should emit a redraw action")
        });

        assert_eq!(
            app.strata_dirty.get(),
            0,
            "warmup decode should not force a full strata rebuild by itself"
        );
        assert!(
            app.textures_pending.get(),
            "decoded textures should stay pending until draw uploads and resolves them"
        );
        assert!(
            app.texture_manager.borrow().get("tick-warmup").is_some(),
            "tick-start warmup should decode the visible texture source"
        );
        assert!(
            matches!(action, Action::Window(WindowAction::RedrawAll)),
            "decode progress should request a redraw for the next draw pass"
        );
    }

    #[test]
    fn process_timers_requests_redraw_when_strata_are_dirty() {
        let mut app = build_test_app(ScreenKind::Game);
        app.screen_size.set(Size::new(1024.0, 768.0));
        app.selected_rot_level = "Off".to_string();
        app.strata_dirty.set(1);

        let task = app.handle_process_timers(Instant::now());
        let action = pollster::block_on(async {
            iced_runtime::task::into_stream(task)
                .expect("redraw task should produce a runtime action")
                .next()
                .await
                .expect("task should emit a redraw action")
        });

        assert!(
            matches!(action, Action::Window(WindowAction::RedrawAll)),
            "dirty strata should still request a redraw"
        );
    }

    #[test]
    fn canvas_event_requests_redraw_when_it_leaves_dirty_strata() {
        let mut app = build_test_app(ScreenKind::Game);
        app.strata_dirty.set(1);

        let task = app.handle_canvas_event(CanvasMessage::MouseLeave);
        let action = pollster::block_on(async {
            iced_runtime::task::into_stream(task)
                .expect("canvas event with dirty strata should request a redraw")
                .next()
                .await
                .expect("task should emit a redraw action")
        });

        assert!(
            matches!(action, Action::Window(WindowAction::RedrawAll)),
            "canvas event should request redraw when it leaves strata dirty"
        );
    }

    #[test]
    fn tick_warmup_prefers_cached_render_crop_requests() {
        let temp_dir = tempdir().unwrap();
        let texture_path = temp_dir.path().join("tick-warmup-crop.png");
        let image = image::RgbaImage::from_pixel(4, 4, image::Rgba([0x44, 0x88, 0xcc, 0xff]));
        image.save(&texture_path).unwrap();

        let app = build_test_app(ScreenKind::Game);
        app.strata_dirty.set(0);
        app.textures_pending.set(false);

        let crop_path = "tick-warmup-crop@crop:0.000000,0.500000,0.000000,0.500000";
        let mut batch = QuadBatch::new();
        batch
            .texture_requests
            .push(TextureRequest::new(crop_path, 0, 4));
        app.cached_strata_quads.borrow_mut()[0] = Some(std::sync::Arc::new(batch));

        app.preload_visible_textures_with_budget(std::time::Duration::from_millis(50));

        let tex_mgr = app.texture_manager.borrow();
        assert!(
            tex_mgr.get_cached_crop_request(crop_path).is_some(),
            "tick warmup should cache the current render crop request itself"
        );
        assert!(
            app.textures_pending.get(),
            "CPU-cached render requests must stay pending until the live draw uploads them"
        );
    }

    #[test]
    fn tick_warmup_keeps_staged_but_unprepared_requests_pending() {
        let temp_dir = tempdir().unwrap();
        let texture_path = temp_dir.path().join("staged-not-prepared.png");
        let image = image::RgbaImage::from_pixel(4, 4, image::Rgba([0x44, 0x88, 0xcc, 0xff]));
        image.save(&texture_path).unwrap();

        let app = build_test_app(ScreenKind::Game);
        app.strata_dirty.set(0);
        app.textures_pending.set(false);

        let request_path = "staged-not-prepared";
        let mut batch = QuadBatch::new();
        batch
            .texture_requests
            .push(TextureRequest::new(request_path, 0, 4));
        batch.texture_requests[0].handle.mark_staged();
        app.cached_strata_quads.borrow_mut()[0] = Some(std::sync::Arc::new(batch));

        {
            let mut tex_mgr = app.texture_manager.borrow_mut();
            render::preload_texture_request_source(&mut tex_mgr, request_path);
        }
        app.preload_visible_textures_with_budget(std::time::Duration::from_millis(50));

        assert!(
            app.textures_pending.get(),
            "draw-staged requests must stay pending until prepare uploads them into the atlas"
        );
    }

    #[test]
    fn process_timers_requests_redraw_when_pending_is_newly_discovered() {
        let temp_dir = tempdir().unwrap();
        let texture_path = temp_dir.path().join("staged-discovered.png");
        let image = image::RgbaImage::from_pixel(4, 4, image::Rgba([0x44, 0x88, 0xcc, 0xff]));
        image.save(&texture_path).unwrap();

        let mut app = build_test_app(ScreenKind::Game);
        app.screen_size.set(Size::new(1024.0, 768.0));
        app.selected_rot_level = "Off".to_string();
        app.strata_dirty.set(0);
        app.textures_pending.set(false);

        let request_path = "staged-discovered";
        let mut batch = QuadBatch::new();
        batch
            .texture_requests
            .push(TextureRequest::new(request_path, 0, 4));
        batch.texture_requests[0].handle.mark_staged();
        app.cached_strata_quads.borrow_mut()[0] = Some(std::sync::Arc::new(batch));

        {
            let mut tex_mgr = app.texture_manager.borrow_mut();
            render::preload_texture_request_source(&mut tex_mgr, request_path);
        }
        let task = app.update(Message::ProcessTimers(Instant::now()));
        let action = pollster::block_on(async {
            iced_runtime::task::into_stream(task)
                .expect("newly discovered pending draw work should request a redraw")
                .next()
                .await
                .expect("task should emit a redraw action")
        });

        assert!(
            app.textures_pending.get(),
            "staged-but-unprepared requests should become pending"
        );
        assert!(
            matches!(action, Action::Window(WindowAction::RedrawAll)),
            "first discovery of unresolved draw work should request a redraw"
        );
    }

    #[test]
    fn process_timers_skips_redraw_when_pending_state_does_not_progress() {
        let mut app = build_test_app(ScreenKind::Game);
        app.screen_size.set(Size::new(1024.0, 768.0));
        app.selected_rot_level = "Off".to_string();
        app.strata_dirty.set(0);
        app.textures_pending.set(true);
        {
            let env = app.env.borrow();
            let _ = env.state().borrow().widgets.take_render_dirty_with_ids();
        }

        let task = app.handle_process_timers(Instant::now());

        assert!(
            iced_runtime::task::into_stream(task).is_none(),
            "pending state alone should not force redraws every tick"
        );
    }

    #[test]
    fn process_timers_skips_visible_warmup_while_draw_work_is_pending() {
        let temp_dir = tempdir().unwrap();
        let texture_path = temp_dir.path().join("pending-skip-warmup.png");
        let image = image::RgbaImage::from_pixel(4, 4, image::Rgba([0x44, 0x88, 0xcc, 0xff]));
        image.save(&texture_path).unwrap();

        let mut app = build_test_app(ScreenKind::Game);
        app.screen_size.set(Size::new(1024.0, 768.0));
        app.selected_rot_level = "Off".to_string();
        app.strata_dirty.set(0);
        app.textures_pending.set(true);

        {
            let env = app.env.borrow();
            env.exec(
                r#"
                local frame = CreateFrame("Frame", "PendingSkipWarmupFrame", UIParent)
                local texture = frame:CreateTexture(nil, "ARTWORK")
                texture:SetTexture("pending-skip-warmup")
            "#,
            )
            .unwrap();
            let _ = env.state().borrow().widgets.take_render_dirty_with_ids();
        }

        let task = app.handle_process_timers(Instant::now());
        assert!(
            iced_runtime::task::into_stream(task).is_none(),
            "draw-owned pending state should not trigger another warmup decode pass"
        );
        assert!(
            app.texture_manager
                .borrow()
                .get("pending-skip-warmup")
                .is_none(),
            "visible warmup should be skipped while draw work is already pending"
        );
    }

    #[test]
    fn process_timers_prioritizes_queued_preloads_over_visible_warmup() {
        let temp_dir = tempdir().unwrap();
        let visible_texture = temp_dir.path().join("queued-visible.png");
        let queued_texture = temp_dir.path().join("queued-target.png");
        let image = image::RgbaImage::from_pixel(4, 4, image::Rgba([0x44, 0x88, 0xcc, 0xff]));
        image.save(&visible_texture).unwrap();
        image.save(&queued_texture).unwrap();

        let mut app = build_test_app(ScreenKind::Game);
        app.screen_size.set(Size::new(1024.0, 768.0));
        app.selected_rot_level = "Off".to_string();
        app.strata_dirty.set(0);
        app.textures_pending.set(false);

        {
            let env = app.env.borrow();
            env.exec(
                r#"
                local frame = CreateFrame("Frame", "QueuedPreloadVisibleFrame", UIParent)
                local texture = frame:CreateTexture(nil, "ARTWORK")
                texture:SetTexture("queued-visible")
            "#,
            )
            .unwrap();
            env.state()
                .borrow_mut()
                .enqueue_texture_preloads(["queued-target".to_string()]);
            let _ = env.state().borrow().widgets.take_render_dirty_with_ids();
        }

        let task = app.handle_process_timers(Instant::now());
        let action = pollster::block_on(async {
            iced_runtime::task::into_stream(task)
                .expect("queued preload progress should request a redraw")
                .next()
                .await
                .expect("task should emit a redraw action")
        });

        let tex_mgr = app.texture_manager.borrow();
        assert!(
            tex_mgr.get("queued-target").is_some(),
            "queued preloads should still decode during the tick"
        );
        assert!(
            tex_mgr.get("queued-visible").is_none(),
            "queued preloads should bypass tick visible warmup to avoid duplicate decode work"
        );
        assert!(
            matches!(action, Action::Window(WindowAction::RedrawAll)),
            "queued preload progress should still request a redraw"
        );
    }

    #[test]
    fn collect_tick_dirty_preserves_full_rebuild_sentinel() {
        let mut app = build_test_app(ScreenKind::Game);
        app.pending_dirty_ids.borrow_mut().take();
        app.mark_all_strata_dirty();

        {
            let env = app.env.borrow();
            let frame = env
                .state()
                .borrow()
                .widgets
                .get_id_by_name("UIParent")
                .expect("UIParent should exist");
            env.state().borrow().widgets.mark_visual_dirty(frame);
        }

        let _ = app.collect_tick_dirty();

        assert!(
            app.pending_dirty_ids.borrow().is_none(),
            "full rebuild sentinel must survive later per-frame dirty collection"
        );
    }

    #[test]
    fn sample_display_metrics_split_frame_budget() {
        let metrics =
            sample_display_metrics(std::time::Duration::from_secs_f32(1.0), 10, 10, 25.0, 15.0);

        assert!((metrics.fps - 10.0).abs() < 0.001);
        assert!((metrics.draw_ms - 2.5).abs() < 0.001);
        assert!((metrics.tick_ms - 1.5).abs() < 0.001);
        assert!((metrics.other_ms - 96.0).abs() < 0.001);
    }
}
use rilua::Val;
