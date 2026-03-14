//! App::update() method and related logic.

use iced::Task;
use iced_layout_inspector::server::ScreenshotData;

use crate::lua_api::WowLuaEnv;

use super::Message;
use super::app::App;
use super::state::CanvasMessage;

impl App {
    pub fn update(&mut self, message: Message) -> Task<Message> {
        // Always drain IPC so commands from the inspector/REPL are processed
        // even when idle (no timer subscription active).
        let ipc_task = self.process_ipc();

        let task = match message {
            Message::FireEvent(event) => {
                self.handle_fire_event(&event);
                Task::none()
            }
            Message::CanvasEvent(canvas_msg) => self.handle_canvas_event(canvas_msg),
            Message::Scroll(dx, dy) => {
                self.handle_scroll(dx, dy);
                Task::none()
            }
            Message::ReloadUI => {
                self.handle_reload_ui();
                Task::none()
            }
            Message::CommandInputChanged(input) => {
                self.command_input = input;
                Task::none()
            }
            Message::ExecuteCommand => {
                self.handle_execute_command();
                Task::none()
            }
            Message::ProcessTimers => self.handle_process_timers(),
            Message::ScreenshotTaken(screenshot) => {
                self.handle_screenshot_taken(screenshot);
                Task::none()
            }
            Message::FpsTick => Task::none(),
            Message::InspectorClose => {
                self.handle_inspector_close();
                Task::none()
            }
            Message::InspectorWidthChanged(val) => {
                self.inspector_state.width = val;
                Task::none()
            }
            Message::InspectorHeightChanged(val) => {
                self.inspector_state.height = val;
                Task::none()
            }
            Message::InspectorAlphaChanged(val) => {
                self.inspector_state.alpha = val;
                Task::none()
            }
            Message::InspectorLevelChanged(val) => {
                self.inspector_state.frame_level = val;
                Task::none()
            }
            Message::InspectorVisibleToggled(val) => {
                self.inspector_state.visible = val;
                Task::none()
            }
            Message::InspectorMouseEnabledToggled(val) => {
                self.inspector_state.mouse_enabled = val;
                Task::none()
            }
            Message::InspectorApply => {
                self.handle_inspector_apply();
                Task::none()
            }
            Message::ToggleFramesPanel => {
                self.frames_panel_collapsed = !self.frames_panel_collapsed;
                Task::none()
            }
            Message::XpLevelChanged(ref label) => {
                self.handle_xp_level_changed(label);
                Task::none()
            }
            Message::KeyPress(ref key, ref text) => {
                if key == "ESCAPE" && self.options_modal_visible {
                    self.options_modal_visible = false;
                } else {
                    self.handle_key_press(key, text.as_deref());
                }
                Task::none()
            }
            Message::PlayerClassChanged(ref name) => {
                self.handle_player_class_changed(name);
                Task::none()
            }
            Message::PlayerRaceChanged(ref name) => {
                self.handle_player_race_changed(name);
                Task::none()
            }
            Message::RotDamageLevelChanged(ref label) => {
                self.handle_rot_damage_level_changed(label);
                Task::none()
            }
            Message::ToggleOptionsModal => {
                self.options_modal_visible = !self.options_modal_visible;
                Task::none()
            }
            Message::CloseOptionsModal => {
                self.options_modal_visible = false;
                Task::none()
            }
            Message::MovementToggled(field, val) => {
                self.handle_movement_toggled(field, val);
                Task::none()
            }
        };

        Task::batch([task, ipc_task])
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

    /// Fire UNIT_PORTRAIT_UPDATE + PLAYER_ENTERING_WORLD to refresh unit frames.
    fn fire_portrait_update(&self, env: &WowLuaEnv) {
        let _ = env.fire_event_with_args(
            "UNIT_PORTRAIT_UPDATE",
            &[mlua::Value::String(
                env.lua().create_string("player").unwrap(),
            )],
        );
        let _ = env.fire_event_with_args(
            "PLAYER_ENTERING_WORLD",
            &[mlua::Value::Boolean(false), mlua::Value::Boolean(false)],
        );
    }

    fn handle_reload_ui(&mut self) {
        self.log_messages.push("Reloading UI...".to_string());
        {
            let env = self.env.borrow();
            if let Ok(s) = env.lua().create_string("WoWUISim") {
                let _ = env.fire_event_with_args("ADDON_LOADED", &[mlua::Value::String(s)]);
            }
            let _ = env.fire_event("VARIABLES_LOADED");
            let _ = env.fire_event_with_args(
                "PLAYER_ENTERING_WORLD",
                &[mlua::Value::Boolean(false), mlua::Value::Boolean(true)],
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
                Ok(false) => {
                    self.log_messages.push(format!("Unknown command: {}", cmd));
                }
                Err(e) => {
                    self.log_messages.push(format!("Command error: {}", e));
                }
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
        if self.textures_pending.get() {
            self.mark_all_strata_dirty();
        }
        let total = t0.elapsed();
        if total.as_millis() > 10 {
            let n = self.pending_dirty_ids.borrow().as_ref().map(|s| s.len());
            eprintln!(
                "[tick] {total:.1?} (layout={layout_dur:.1?} dirty=0x{combined:x} ids={n:?} pending={})",
                self.textures_pending.get()
            );
        }
        Task::none()
    }

    /// Run timers, layout, OnUpdate, health/casting and collect dirty mask + IDs.
    fn collect_tick_dirty(&mut self) -> (u16, std::time::Duration) {
        self.env
            .borrow()
            .state()
            .borrow()
            .widgets
            .take_render_dirty();
        self.run_wow_timers();
        let (m1, ids1) = self
            .env
            .borrow()
            .state()
            .borrow()
            .widgets
            .take_render_dirty_with_ids();

        let t_layout = std::time::Instant::now();
        self.env.borrow().state().borrow_mut().ensure_layout_rects();
        let layout_dur = t_layout.elapsed();

        self.fire_on_update();
        let (m2, ids2) = self
            .env
            .borrow()
            .state()
            .borrow()
            .widgets
            .take_render_dirty_with_ids();

        self.tick_party_health();
        self.tick_casting();
        let (m3, ids3) = self
            .env
            .borrow()
            .state()
            .borrow()
            .widgets
            .take_render_dirty_with_ids();

        let combined_ids = match (ids1, ids2, ids3) {
            (Some(mut a), Some(b), Some(c)) => {
                a.extend(b);
                a.extend(c);
                Some(a)
            }
            _ => None,
        };
        *self.pending_dirty_ids.borrow_mut() = combined_ids;
        (m1 | m2 | m3, layout_dur)
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
        // Throttle: skip if less than 16ms since last completion.
        // Prevents queued ProcessTimers messages from starving draw().
        if elapsed.as_millis() < 16 {
            return;
        }
        self.last_on_update_time = now;
        let env = self.env.borrow();
        if let Err(e) = env.fire_on_update(elapsed.as_secs_f64()) {
            let ts = now.duration_since(self.fps_last_time);
            eprintln!("[{ts:.1?}] [OnUpdate] error: {e}");
        }
        // Update timestamp to after completion so throttle measures from end.
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
            let _ = env.fire_event_with_args(
                "UNIT_HEALTH",
                &[mlua::Value::String(
                    env.lua().create_string(&unit_id).unwrap(),
                )],
            );
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

    /// Save current UI settings to config file.
    fn save_config(&self) {
        let mut config = crate::config::SimConfig::load();
        config.player_class = self.selected_class.clone();
        config.player_race = self.selected_race.clone();
        config.rot_damage_level = self.selected_rot_level.clone();
        config.xp_level = self.selected_xp_level.clone();
        config.movement = self.movement.clone();
        config.save();
    }

    /// Resolve layout and fire OnUpdate after script handlers so that handlers
    /// registered during events (e.g. talent UI dirty-node processing) run in
    /// the same frame as the triggering click, then invalidate the render.
    pub(super) fn flush_post_script_updates(&mut self) {
        self.env.borrow().state().borrow_mut().ensure_layout_rects();
        self.fire_on_update();
        self.invalidate();
    }

    /// Drain console, preload textures, and mark all strata dirty.
    pub(super) fn invalidate(&mut self) {
        self.drain_console();
        self.preload_visible_textures();
        self.gpu_failed_textures.borrow_mut().clear(); // fresh upload attempt
        self.mark_all_strata_dirty();
    }

    /// Preload visible textures with a ~10ms budget to avoid freezing.
    fn preload_visible_textures(&self) {
        let env = self.env.borrow();
        let paths = env.state().borrow().widgets.visible_texture_paths();
        drop(env);
        let mut tex_mgr = self.texture_manager.borrow_mut();
        let before = tex_mgr.cache_len();
        let deadline = std::time::Instant::now() + std::time::Duration::from_millis(10);
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
            eprintln!("[preload] {loaded} new textures ({} total)", paths.len());
        }
    }

    /// Apply pending HitGrid changes from `set_frame_visible` calls.
    ///
    /// Walks the subtree of each changed root and inserts/removes hittable
    /// frames from the grid. Called after Lua handlers fire.
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
        let registry = &state.widgets;

        for (root_id, became_visible) in changes {
            // Walk subtree and update each frame in the grid.
            let mut stack = vec![root_id];
            while let Some(id) = stack.pop() {
                let Some(f) = registry.get(id) else { continue };
                if became_visible {
                    // Remove first so moved frames get old cell entries cleaned up.
                    grid.remove(id);
                    if f.visible
                        && f.effective_alpha > 0.0
                        && f.mouse_enabled
                        && !f
                            .name
                            .as_deref()
                            .is_some_and(|n| super::frame_collect::HIT_TEST_EXCLUDED.contains(&n))
                    {
                        if let Some(rect) = f.layout_rect {
                            let (il, ir, it, ib) = f.hit_rect_insets;
                            let scaled = iced::Rectangle::new(
                                iced::Point::new(
                                    (rect.x + il) * crate::render::texture::UI_SCALE,
                                    (rect.y + it) * crate::render::texture::UI_SCALE,
                                ),
                                iced::Size::new(
                                    (rect.width - il - ir).max(0.0)
                                        * crate::render::texture::UI_SCALE,
                                    (rect.height - it - ib).max(0.0)
                                        * crate::render::texture::UI_SCALE,
                                ),
                            );
                            grid.insert(id, scaled);
                        }
                    }
                } else {
                    grid.remove(id);
                }
                stack.extend_from_slice(&f.children);
            }
        }
    }

    /// Check whether a frame's `__enabled` attribute is true (default: true).
    pub(super) fn is_frame_enabled(&self, frame_id: u64) -> bool {
        let env = self.env.borrow();
        let state = env.state().borrow();
        state
            .widgets
            .get(frame_id)
            .and_then(|f| f.attributes.get("__enabled"))
            .and_then(|v| {
                if let crate::widget::AttributeValue::Boolean(b) = v {
                    Some(*b)
                } else {
                    None
                }
            })
            .unwrap_or(true)
    }

    /// Focus an EditBox on click, or clear focus when clicking elsewhere.
    pub(super) fn update_editbox_focus(&self, clicked_frame: Option<u64>) {
        let env = self.env.borrow();
        let editbox_target = env.resolve_editbox_focus_target(clicked_frame);
        let old_focus = env.state().borrow().focused_frame_id;

        if let Some(fid) = editbox_target {
            if old_focus != Some(fid) {
                // Focus the clicked EditBox via Lua SetFocus logic
                {
                    let mut state = env.state().borrow_mut();
                    state.focused_frame_id = Some(fid);
                }
                if let Some(old_id) = old_focus {
                    let _ = env.fire_script_handler(old_id, "OnEditFocusLost", vec![]);
                }
                let _ = env.fire_script_handler(fid, "OnEditFocusGained", vec![]);
            }
        } else if let Some(old_id) = old_focus {
            // Clicked on non-EditBox: clear focus
            {
                let mut state = env.state().borrow_mut();
                state.focused_frame_id = None;
            }
            let _ = env.fire_script_handler(old_id, "OnEditFocusLost", vec![]);
        }
    }

    /// Toggle CheckButton checked state before OnClick (WoW behavior).
    /// Skip action bar buttons — they manage checked state via UpdateState().
    pub(super) fn toggle_checkbutton_if_needed(&self, frame_id: u64, env: &WowLuaEnv) {
        let mut state = env.state().borrow_mut();
        let is_checkbutton = state
            .widgets
            .get(frame_id)
            .map(|f| f.widget_type == crate::widget::WidgetType::CheckButton)
            .unwrap_or(false);
        if !is_checkbutton {
            return;
        }
        // Action bar buttons registered via SetActionUIButton manage their own
        // checked state through UpdateState() — don't auto-toggle them.
        let is_action_button = state
            .action_ui_buttons
            .iter()
            .any(|(id, _)| *id == frame_id);
        if is_action_button {
            return;
        }

        let old_checked = state
            .widgets
            .get(frame_id)
            .and_then(|f| f.attributes.get("__checked"))
            .and_then(|v| {
                if let crate::widget::AttributeValue::Boolean(b) = v {
                    Some(*b)
                } else {
                    None
                }
            })
            .unwrap_or(false);
        let new_checked = !old_checked;

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

    /// Sync the iced canvas size to SimState and UIParent/WorldFrame dimensions.
    /// Called from the render path when the window is resized by the window manager.
    pub(crate) fn sync_screen_size_to_state(&self, size: iced::Size) {
        let env = self.env.borrow();
        let state = env.state().borrow();
        if (state.screen_width - size.width).abs() > 0.5
            || (state.screen_height - size.height).abs() > 0.5
        {
            println!(
                "Window size: {}x{} (was {}x{})",
                size.width as i32,
                size.height as i32,
                state.screen_width as i32,
                state.screen_height as i32
            );
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
