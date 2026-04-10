//! Mouse event handlers for the iced application.

use iced::Point;

use super::app::App;

/// Minimum distance (in pixels) the mouse must move while held to start a drag.
const DRAG_THRESHOLD: f32 = 5.0;

impl App {
    pub(super) fn handle_mouse_move(&mut self, pos: Point) {
        self.sync_mouse_position(pos);
        self.maybe_start_drag(pos);
        self.update_hovered_frame(pos);
    }

    fn sync_mouse_position(&mut self, pos: Point) {
        self.mouse_position = Some(pos);
        {
            let env = self.env.borrow();
            env.state()
                .borrow_mut()
                .set_mouse_position(Some((pos.x, pos.y)));
        }
    }

    fn maybe_start_drag(&mut self, pos: Point) {
        // Check drag threshold while mouse is held down.
        if let (Some(down_pos), Some(down_frame), false) =
            (self.mouse_down_pos, self.mouse_down_frame, self.dragging)
        {
            let dx = pos.x - down_pos.x;
            let dy = pos.y - down_pos.y;
            if (dx * dx + dy * dy).sqrt() >= DRAG_THRESHOLD {
                self.dragging = true;
                self.fire_drag_start(down_frame);
                self.flush_post_script_updates();
            }
        }
    }

    fn update_hovered_frame(&mut self, pos: Point) {
        let new_hovered = self.hit_test(pos);
        if new_hovered == self.hovered_frame {
            self.flush_mouse_move_visual_updates();
            return;
        }

        self.fire_hover_transition(new_hovered);
        // OnEnter/OnLeave scripts may show/hide tooltips or change widget state.
        // Apply incremental HitGrid updates before the next hit_test.
        self.apply_hit_grid_changes();
        self.flush_mouse_move_visual_updates();
    }

    fn fire_hover_transition(&mut self, new_hovered: Option<u64>) {
        // Update hovered_frame in both iced_app and SimState BEFORE firing events,
        // so IsMouseMotionFocus() / GetMouseFocus() return correct values in OnEnter.
        let old_hovered = self.hovered_frame;
        self.hovered_frame = new_hovered;
        {
            let env = self.env.borrow();
            env.state().borrow_mut().hovered_frame = new_hovered;
            if let Some(old_id) = old_hovered {
                let _ = env.fire_script_handler(old_id, "OnLeave", vec![]);
            }
            if let Some(new_id) = new_hovered {
                let _ = env.fire_script_handler(new_id, "OnEnter", vec![]);
            }
        }
    }

    fn flush_mouse_move_visual_updates(&mut self) {
        let (dirty_mask, dirty_ids) = self
            .env
            .borrow()
            .state()
            .borrow()
            .widgets
            .take_render_dirty_with_ids();
        if dirty_mask != 0 {
            self.drain_console();
            self.mark_strata_dirty(dirty_mask);
            self.merge_pending_dirty_ids(dirty_ids);
        } else {
            self.drain_console();
        }
    }

    pub(super) fn handle_mouse_down(&mut self, pos: Point) {
        let hit_frame = self.hit_test(pos);

        // Focus/unfocus EditBox on click
        self.update_editbox_focus(hit_frame);

        let Some(frame_id) = hit_frame else {
            return;
        };

        if !self.is_frame_enabled(frame_id) {
            return;
        }

        self.mouse_down_frame = Some(frame_id);
        self.mouse_down_pos = Some(pos);
        self.dragging = false;
        self.pressed_frame = Some(frame_id);

        {
            let env = self.env.borrow();
            let button_val = mlua::Value::String(env.lua().create_string("LeftButton").unwrap());
            let _ = env.fire_script_handler(frame_id, "OnMouseDown", vec![button_val]);
        }
        self.flush_post_script_updates();
    }

    pub(super) fn handle_mouse_up(&mut self, pos: Point) {
        let (was_dragging, drag_source) = self.take_left_drag_state();
        let released_on = self.hit_test(pos);

        if was_dragging {
            self.finish_drag(drag_source, released_on);
        } else {
            self.dispatch_left_mouse_release(released_on);
        }
        self.mouse_down_frame = None;
        self.pressed_frame = None;
        self.flush_post_script_updates();
    }

    fn take_left_drag_state(&mut self) -> (bool, Option<u64>) {
        let was_dragging = self.dragging;
        let drag_source = self.mouse_down_frame;
        self.mouse_down_pos = None;
        self.dragging = false;
        (was_dragging, drag_source)
    }

    fn dispatch_left_mouse_release(&mut self, released_on: Option<u64>) {
        let Some(frame_id) = released_on else {
            return;
        };

        if self.cursor_holds_item() {
            self.fire_receive_drag(frame_id);
            return;
        }

        let env = self.env.borrow();
        let button_val = mlua::Value::String(env.lua().create_string("LeftButton").unwrap());

        if self.mouse_down_frame == Some(frame_id) {
            self.fire_left_click_sequence(frame_id, &env, &button_val);
        }

        let _ = env.fire_script_handler(frame_id, "OnMouseUp", vec![button_val]);
    }

    fn cursor_holds_item(&self) -> bool {
        self.env.borrow().state().borrow().cursor_item.is_some()
    }

    fn fire_left_click_sequence(
        &self,
        frame_id: u64,
        env: &crate::lua_api::WowLuaEnv,
        button_val: &mlua::Value,
    ) {
        self.toggle_checkbutton_if_needed(frame_id, env);

        let down_val = mlua::Value::Boolean(false);
        let _ = env.fire_script_handler(
            frame_id,
            "OnClick",
            vec![button_val.clone(), down_val.clone()],
        );

        // PostClick fires after OnClick (WoW secure button sequence).
        // ActionBar buttons use PostClick to call UpdateState().
        let _ = env.fire_script_handler(frame_id, "PostClick", vec![button_val.clone(), down_val]);
    }

    pub(super) fn handle_right_mouse_down(&mut self, pos: Point) {
        let Some(frame_id) = self.hit_test(pos) else {
            return;
        };
        if !self.is_frame_enabled(frame_id) {
            return;
        }
        self.right_mouse_down_frame = Some(frame_id);
        {
            let env = self.env.borrow();
            let button_val = mlua::Value::String(env.lua().create_string("RightButton").unwrap());
            let _ = env.fire_script_handler(frame_id, "OnMouseDown", vec![button_val]);
        }
        self.flush_post_script_updates();
    }

    pub(super) fn handle_right_mouse_up(&mut self, pos: Point) {
        // Right-click clears the cursor (drops held spell/action) in WoW.
        // Use the Lua ClearCursor() function so events fire properly.
        let had_cursor_item = self.env.borrow().state().borrow().cursor_item.is_some();
        if had_cursor_item {
            eprintln!("[cursor] Right-click ClearCursor");
            {
                let env = self.env.borrow();
                let lua = env.lua();
                if let Ok(clear_fn) = lua.globals().get::<mlua::Function>("ClearCursor") {
                    let _ = clear_fn.call::<()>(());
                }
            }
            self.right_mouse_down_frame = None;
            self.flush_post_script_updates();
            return;
        }

        let released_on = self.hit_test(pos);
        if let Some(frame_id) = released_on {
            {
                let env = self.env.borrow();
                let button_val =
                    mlua::Value::String(env.lua().create_string("RightButton").unwrap());

                if self.right_mouse_down_frame == Some(frame_id) {
                    let down_val = mlua::Value::Boolean(false);
                    let _ = env.fire_script_handler(
                        frame_id,
                        "OnClick",
                        vec![button_val.clone(), down_val.clone()],
                    );
                    let _ = env.fire_script_handler(
                        frame_id,
                        "PostClick",
                        vec![button_val.clone(), down_val],
                    );
                }

                let _ = env.fire_script_handler(frame_id, "OnMouseUp", vec![button_val]);
            }
            self.flush_post_script_updates();
        }
        self.right_mouse_down_frame = None;
    }

    pub(super) fn handle_middle_click(&mut self, pos: Point) {
        if let Some(frame_id) = self.hit_test(pos) {
            self.populate_inspector(frame_id);
            self.inspected_frame = Some(frame_id);
            self.inspector_visible = true;
            self.inspector_position = Point::new(pos.x + 10.0, pos.y + 10.0);
        }
    }

    pub(super) fn handle_scroll(&mut self, _dx: f32, dy: f32) {
        if self.fire_mouse_wheel(dy) {
            self.invalidate();
        } else {
            let scroll_speed = 30.0;
            self.scroll_offset -= dy * scroll_speed;
            let max_scroll = 2600.0;
            self.scroll_offset = self.scroll_offset.clamp(0.0, max_scroll);
            self.mark_all_strata_dirty();
        }
    }

    /// Fire OnDragStart on the source frame (walks up parent chain).
    fn fire_drag_start(&mut self, frame_id: u64) {
        let env = self.env.borrow();
        let lua = env.lua();
        let button_val = mlua::Value::String(lua.create_string("LeftButton").unwrap());

        // Walk up parent chain looking for a frame with OnDragStart registered.
        let mut current = Some(frame_id);
        while let Some(id) = current {
            if env.has_script_handler(id, "OnDragStart") {
                eprintln!("[drag] OnDragStart fired on frame {}", id);
                let _ = env.fire_script_handler(id, "OnDragStart", vec![button_val]);
                return;
            }
            current = env
                .state()
                .borrow()
                .widgets
                .get(id)
                .and_then(|f| f.parent_id);
        }
    }

    /// Fire OnDragStop on source and OnReceiveDrag on target.
    fn finish_drag(&mut self, source: Option<u64>, target: Option<u64>) {
        // Fire OnDragStop on the source frame (walk up parent chain).
        if let Some(src_id) = source {
            let env = self.env.borrow();
            let lua = env.lua();
            let button_val = mlua::Value::String(lua.create_string("LeftButton").unwrap());
            let mut current = Some(src_id);
            while let Some(id) = current {
                if env.has_script_handler(id, "OnDragStop") {
                    eprintln!("[drag] OnDragStop fired on frame {}", id);
                    let _ = env.fire_script_handler(id, "OnDragStop", vec![button_val]);
                    break;
                }
                current = env
                    .state()
                    .borrow()
                    .widgets
                    .get(id)
                    .and_then(|f| f.parent_id);
            }
        }

        if let Some(tgt_id) = target {
            self.fire_receive_drag(tgt_id);
        }
    }

    /// Fire OnReceiveDrag on a frame (walks up parent chain).
    /// Used both at end of drag and on click when cursor holds an item.
    fn fire_receive_drag(&mut self, frame_id: u64) {
        let env = self.env.borrow();
        let lua = env.lua();
        let button_val = mlua::Value::String(lua.create_string("LeftButton").unwrap());
        let mut current = Some(frame_id);
        while let Some(id) = current {
            if env.has_script_handler(id, "OnReceiveDrag") {
                eprintln!("[drag] OnReceiveDrag fired on frame {}", id);
                let _ = env.fire_script_handler(id, "OnReceiveDrag", vec![button_val]);
                return;
            }
            current = env
                .state()
                .borrow()
                .widgets
                .get(id)
                .and_then(|f| f.parent_id);
        }
    }

    /// Propagate OnMouseWheel up the parent chain. Returns true if handled.
    fn fire_mouse_wheel(&mut self, dy: f32) -> bool {
        let pos = match self.mouse_position {
            Some(p) => p,
            None => return false,
        };
        let start_frame = match self.hit_test(pos) {
            Some(f) => f,
            None => return false,
        };

        let env = self.env.borrow();
        let mut current = Some(start_frame);
        while let Some(frame_id) = current {
            if env.has_script_handler(frame_id, "OnMouseWheel") {
                let delta_val = mlua::Value::Number(dy as f64);
                let _ = env.fire_script_handler(frame_id, "OnMouseWheel", vec![delta_val]);
                return true;
            }
            current = env
                .state()
                .borrow()
                .widgets
                .get(frame_id)
                .and_then(|f| f.parent_id);
        }
        false
    }
}
