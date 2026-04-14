//! App::update() method and related logic.

use iced::Task;
use iced_layout_inspector::server::ScreenshotData;

use crate::lua_api::WowLuaEnv;

use super::Message;
use super::app::App;
use super::state::CanvasMessage;
use super::update_helpers::{
    apply_subtree_hit_grid_change, get_checked_attribute, is_toggleable_checkbutton,
    merge_dirty_ids,
};

impl App {
    pub fn update(&mut self, message: Message) -> Task<Message> {
        let ipc_task = self.process_ipc();
        let task = match message {
            Message::CanvasEvent(cm) => self.handle_canvas_event(cm),
            Message::ProcessTimers => self.handle_process_timers(),
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
            Message::KeyPress(ref key, ref text) => {
                self.handle_simple_key_press(key, text.as_deref())
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
            Message::CanvasEvent(_) | Message::ProcessTimers => unreachable!(),
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

    fn handle_simple_key_press(&mut self, key: &str, text: Option<&str>) {
        if key == "ESCAPE" && self.options_modal_visible {
            self.options_modal_visible = false;
            return;
        }

        self.handle_key_press(key, text);
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
        self.invalidate();
    }

    fn handle_canvas_event(&mut self, canvas_msg: CanvasMessage) -> Task<Message> {
        match canvas_msg {
            CanvasMessage::MouseMove(pos) => self.handle_mouse_move(pos),
            CanvasMessage::MouseDown(pos) => self.handle_mouse_down(pos),
            CanvasMessage::MouseUp(pos) => self.handle_mouse_up(pos),
            CanvasMessage::RightMouseDown(pos) => self.handle_right_mouse_down(pos),
            CanvasMessage::RightMouseUp(pos) => self.handle_right_mouse_up(pos),
            CanvasMessage::MiddleClick(pos) => self.handle_middle_click(pos),
        }
        Task::none()
    }

    pub(super) fn handle_key_press(&mut self, key: &str, text: Option<&str>) {
        let env = self.env.borrow();
        if let Err(e) = env.send_key_press(key, text) {
            self.log_messages
                .push(format!("KeyPress({}) error: {}", key, e));
        }
        drop(env);
        self.invalidate();
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
        self.drain_console();
        self.command_input.clear();
        self.mark_all_strata_dirty();
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

    fn handle_process_timers(&mut self) -> Task<Message> {
        let t0 = std::time::Instant::now();
        self.update_fps_counter();
        self.run_pending_exec_lua();

        let (combined, layout_dur) = self.collect_tick_dirty();
        self.drain_console();
        if combined != 0 {
            self.mark_strata_dirty(combined);
        }
        if self.strata_dirty.get() != 0 || self.textures_pending.get() {
            self.preload_current_render_requests_preserving_dirty(Some(
                std::time::Duration::from_millis(25),
            ));
        }
        log_slow_tick(t0.elapsed(), layout_dur, combined, self);
        Task::none()
    }

    /// Run timers, layout, OnUpdate, health/casting and collect dirty mask + IDs.
    fn collect_tick_dirty(&mut self) -> (u16, std::time::Duration) {
        let (m0, ids0) = self.take_render_dirty_with_ids();
        self.run_wow_timers();
        let (m1, ids1) = self.take_render_dirty_with_ids();

        let t_layout = std::time::Instant::now();
        self.env.borrow().state().borrow_mut().ensure_layout_rects();
        let layout_dur = t_layout.elapsed();

        self.fire_on_update();
        let (m2, ids2) = self.take_render_dirty_with_ids();

        self.tick_party_health();
        self.tick_casting();
        let (m3, ids3) = self.take_render_dirty_with_ids();

        *self.pending_dirty_ids.borrow_mut() = merge_dirty_ids([ids0, ids1, ids2, ids3]);
        (m0 | m1 | m2 | m3, layout_dur)
    }

    fn take_render_dirty_with_ids(&self) -> (u16, Option<std::collections::HashSet<u64>>) {
        self.env
            .borrow()
            .state()
            .borrow()
            .widgets
            .take_render_dirty_with_ids()
    }

    pub(super) fn merge_pending_dirty_ids(&self, ids: Option<std::collections::HashSet<u64>>) {
        let current = self.pending_dirty_ids.borrow_mut().take();
        *self.pending_dirty_ids.borrow_mut() = merge_dirty_ids([current, ids]);
    }

    fn update_fps_counter(&mut self) {
        let now = std::time::Instant::now();
        let elapsed = now.duration_since(self.fps_last_time);
        if elapsed >= std::time::Duration::from_secs(1) {
            let frames = self.frame_count.get();
            self.fps = frames as f32 / elapsed.as_secs_f32();
            self.frame_time_display = self.frame_time_avg.get();
            self.frame_count.set(0);
            self.fps_last_time = now;
            let env = self.env.borrow();
            env.state().borrow_mut().fps = self.fps;
        }
    }

    fn run_wow_timers(&self) {
        let env = self.env.borrow();
        if let Err(e) = env.process_timers() {
            eprintln!("Timer error: {}", e);
        }
    }

    fn fire_on_update(&mut self) {
        let now = std::time::Instant::now();
        let elapsed = now.duration_since(self.last_on_update_time);
        if elapsed.as_millis() < 16 {
            return;
        }
        self.last_on_update_time = now;
        let env = self.env.borrow();
        if let Err(e) = env.fire_on_update(elapsed.as_secs_f64()) {
            crate::logging::eprintln_elapsed(&format!("[OnUpdate] error: {e}"));
        }
        self.last_on_update_time = std::time::Instant::now();
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
        if let Some(code) = self.pending_exec_lua.take() {
            eprintln!("[exec-lua] Running: {}", code);
            let env = self.env.borrow();
            if let Err(e) = env.exec(&code) {
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
        config.movement = self.movement.clone();
        config.save();
    }

    pub(super) fn flush_post_script_updates(&mut self) {
        self.env.borrow().state().borrow_mut().ensure_layout_rects();
        self.fire_on_update();
        self.invalidate();
    }

    pub(super) fn invalidate(&mut self) {
        self.drain_console();
        self.preload_visible_textures();
        self.gpu_failed_textures.borrow_mut().clear();
        self.mark_all_strata_dirty();
        self.preload_current_render_requests_preserving_dirty(Some(
            std::time::Duration::from_millis(25),
        ));
    }

    fn preload_visible_textures(&self) {
        let env = self.env.borrow();
        let paths = env.state().borrow().widgets.visible_texture_paths();
        drop(env);
        let mut tex_mgr = self.texture_manager.borrow_mut();
        let before = tex_mgr.cache_len();
        let deadline = std::time::Instant::now() + std::time::Duration::from_millis(50);
        let mut remaining_uncached = false;
        for path in &paths {
            if tex_mgr.get(path).is_some() {
                continue;
            }
            tex_mgr.load(path);
            if std::time::Instant::now() >= deadline {
                remaining_uncached = paths.iter().any(|p| tex_mgr.get(p).is_none());
                break;
            }
        }
        self.textures_pending.set(remaining_uncached);
        let loaded = tex_mgr.cache_len() - before;
        if loaded > 0 {
            crate::logging::eprintln_elapsed(&format!(
                "[preload] {loaded} new textures ({} total)",
                paths.len()
            ));
        }
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
                env.state().borrow_mut().focused_frame_id = Some(fid);
                if let Some(old_id) = old_focus {
                    let _ = env.fire_script_handler(old_id, "OnEditFocusLost", vec![]);
                }
                let _ = env.fire_script_handler(fid, "OnEditFocusGained", vec![]);
            }
        } else if let Some(old_id) = old_focus {
            env.state().borrow_mut().focused_frame_id = None;
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
}

fn log_slow_tick(
    total: std::time::Duration,
    layout_dur: std::time::Duration,
    combined: u16,
    app: &App,
) {
    if super::perf_logging_enabled() && total.as_millis() > 10 {
        let n = app.pending_dirty_ids.borrow().as_ref().map(|s| s.len());
        eprintln!(
            "[tick] {total:.1?} (layout={layout_dur:.1?} dirty=0x{combined:x} ids={n:?} pending={})",
            app.textures_pending.get()
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::screen::ScreenKind;
    use crate::texture::TextureManager;
    use iced::Size;
    use std::cell::RefCell;
    use std::path::PathBuf;
    use std::rc::Rc;
    use tokio::sync::mpsc;

    fn build_test_app(screen_kind: ScreenKind) -> App {
        let env = Rc::new(RefCell::new(
            WowLuaEnv::new().expect("Failed to create Lua environment"),
        ));
        env.borrow().set_screen_mode(screen_kind);

        let texture_manager = Rc::new(RefCell::new(TextureManager::new(PathBuf::from(
            "./textures",
        ))));
        let font_system = Rc::new(RefCell::new(crate::render::WowFontSystem::new(
            &PathBuf::from(super::super::app::DEFAULT_FONTS_PATH),
        )));
        let glyph_atlas = Rc::new(RefCell::new(crate::render::GlyphAtlas::new()));
        let (_cmd_tx, cmd_rx) = mpsc::channel(1);
        let (_lua_tx, lua_rx) = std::sync::mpsc::channel();

        App::build_app(
            env,
            Vec::new(),
            texture_manager,
            font_system,
            glyph_atlas,
            cmd_rx,
            lua_rx,
            false,
            false,
            None,
            crate::config::SimConfig::default(),
        )
    }

    #[test]
    fn collect_tick_dirty_preserves_preexisting_dirty_ids() {
        let mut app = build_test_app(ScreenKind::Game);
        app.strata_dirty.set(0);
        app.selected_rot_level = "Off".to_string();
        app.screen_size.set(Size::new(1024.0, 768.0));

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

        let (dirty_mask, _layout_dur) = app.collect_tick_dirty();
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
}
use rilua::Val;
