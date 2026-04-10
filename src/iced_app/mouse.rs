//! Mouse event handlers for the iced application.

use iced::Point;

use super::app::App;
use crate::widget::{AnchorPoint, WidgetType};

/// Minimum distance (in pixels) the mouse must move while held to start a drag.
const DRAG_THRESHOLD: f32 = 5.0;

impl App {
    pub(super) fn handle_mouse_move(&mut self, pos: Point) {
        let previous_pos = self.mouse_position;
        self.sync_mouse_position(pos);
        self.maybe_start_drag(pos);
        self.maybe_move_active_drag_frame(previous_pos, pos);
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

    fn maybe_move_active_drag_frame(&mut self, previous_pos: Option<Point>, pos: Point) {
        let Some(previous_pos) = previous_pos else {
            return;
        };

        let dx = pos.x - previous_pos.x;
        let dy = pos.y - previous_pos.y;
        if dx.abs() < f32::EPSILON && dy.abs() < f32::EPSILON {
            return;
        }

        let moved = {
            let env = self.env.borrow();
            let mut state = env.state().borrow_mut();
            let Some(drag_id) = state.active_drag_frame else {
                return;
            };
            let screen_size = self.screen_size.get();

            state.ensure_layout_rects();
            let Some((parent_id, x_offset, y_offset)) = moving_drag_anchor_update(
                &state,
                drag_id,
                dx,
                dy,
                screen_size.width,
                screen_size.height,
            ) else {
                return;
            };

            reanchor_moving_drag_frame(&mut state, drag_id, parent_id, x_offset, y_offset);
            true
        };

        if moved {
            self.mark_all_strata_dirty();
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
            if let Some(old_id) = old_hovered.filter(|id| self.motion_scripts_allowed(*id)) {
                let _ = env.fire_script_handler(old_id, "OnLeave", vec![]);
            }
            if let Some(new_id) = new_hovered.filter(|id| self.motion_scripts_allowed(*id)) {
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
        let hit_frame = self.hit_test_mouse_button(pos, "LeftButton");

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
            let mut state = env.state().borrow_mut();
            let slider_drag_target = find_slider_drag_target(&state, frame_id);
            state.set_active_drag_frame(None);
            state.set_active_slider_thumb_drag_frame(slider_drag_target);
        }

        {
            let env = self.env.borrow();
            let button_val = mlua::Value::String(env.lua().create_string("LeftButton").unwrap());
            let _ = env.fire_script_handler(frame_id, "OnMouseDown", vec![button_val]);
        }
        self.flush_post_script_updates();
    }

    pub(super) fn handle_mouse_up(&mut self, pos: Point) {
        let (was_dragging, drag_source) = self.take_left_drag_state();
        let released_on = self.hit_test_mouse_button(pos, "LeftButton");

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
        let drag_source = self.env.borrow().state().borrow().active_drag_frame;
        self.mouse_down_pos = None;
        self.dragging = false;
        {
            let env = self.env.borrow();
            let mut state = env.state().borrow_mut();
            state.set_active_drag_frame(None);
            state.set_active_slider_thumb_drag_frame(None);
        }
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
        let Some(frame_id) = self.hit_test_mouse_button(pos, "RightButton") else {
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

        let released_on = self.hit_test_mouse_button(pos, "RightButton");
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
        let drag_target = {
            let env = self.env.borrow();
            find_drag_script_target(&env, frame_id, "OnDragStart")
        };
        let Some(drag_target) = drag_target else {
            return;
        };

        {
            let env = self.env.borrow();
            env.state()
                .borrow_mut()
                .set_active_drag_frame(Some(drag_target));
        }

        let env = self.env.borrow();
        let lua = env.lua();
        let button_val = mlua::Value::String(lua.create_string("LeftButton").unwrap());
        eprintln!("[drag] OnDragStart fired on frame {}", drag_target);
        let _ = env.fire_script_handler(drag_target, "OnDragStart", vec![button_val]);
    }

    /// Fire OnDragStop on source and OnReceiveDrag on target.
    fn finish_drag(&mut self, source: Option<u64>, target: Option<u64>) {
        // Fire OnDragStop on the source frame (walk up parent chain).
        if let Some(src_id) = source {
            let env = self.env.borrow();
            let lua = env.lua();
            let button_val = mlua::Value::String(lua.create_string("LeftButton").unwrap());
            if let Some(drag_stop_target) = find_drag_script_target(&env, src_id, "OnDragStop") {
                eprintln!("[drag] OnDragStop fired on frame {}", drag_stop_target);
                let _ = env.fire_script_handler(drag_stop_target, "OnDragStop", vec![button_val]);
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
        if let Some(receive_drag_target) = find_drag_script_target(&env, frame_id, "OnReceiveDrag")
        {
            eprintln!(
                "[drag] OnReceiveDrag fired on frame {}",
                receive_drag_target
            );
            let _ = env.fire_script_handler(receive_drag_target, "OnReceiveDrag", vec![button_val]);
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
            let (wheel_enabled, motion_allowed, parent_id) = {
                let state = env.state().borrow();
                state
                    .widgets
                    .get(frame_id)
                    .map(|frame| {
                        (
                            frame.mouse_wheel_enabled,
                            frame_motion_scripts_allowed(frame),
                            frame.parent_id,
                        )
                    })
                    .unwrap_or((false, false, None))
            };
            if motion_allowed && wheel_enabled && env.has_script_handler(frame_id, "OnMouseWheel") {
                let delta_val = mlua::Value::Number(dy as f64);
                let _ = env.fire_script_handler(frame_id, "OnMouseWheel", vec![delta_val]);
                return true;
            }
            current = parent_id;
        }
        false
    }

    fn motion_scripts_allowed(&self, frame_id: u64) -> bool {
        let env = self.env.borrow();
        let state = env.state().borrow();
        state
            .widgets
            .get(frame_id)
            .map(frame_motion_scripts_allowed)
            .unwrap_or(false)
    }
}

fn frame_motion_scripts_allowed(frame: &crate::widget::Frame) -> bool {
    frame_enabled(frame) || frame.motion_scripts_while_disabled
}

fn frame_enabled(frame: &crate::widget::Frame) -> bool {
    frame
        .attributes
        .get("__enabled")
        .and_then(|value| match value {
            crate::widget::AttributeValue::Boolean(enabled) => Some(*enabled),
            _ => None,
        })
        .unwrap_or(true)
}

fn moving_drag_anchor_update(
    state: &crate::lua_api::SimState,
    drag_id: u64,
    dx: f32,
    dy: f32,
    screen_width: f32,
    screen_height: f32,
) -> Option<(Option<u64>, f32, f32)> {
    let frame = state.widgets.get(drag_id)?;
    if !frame.is_moving {
        return None;
    }

    let rect = frame.layout_rect?;
    let parent_id = frame.parent_id;
    let (parent_x, parent_y) = parent_id
        .and_then(|id| state.widgets.get(id).and_then(|parent| parent.layout_rect))
        .map(|rect| (rect.x, rect.y))
        .unwrap_or((0.0, 0.0));

    let mut new_left = rect.x + dx;
    let mut new_top = rect.y + dy;
    if frame.clamped_to_screen {
        new_left = clamp_axis_to_viewport(new_left, rect.width, screen_width);
        new_top = clamp_axis_to_viewport(new_top, rect.height, screen_height);
    }
    Some((parent_id, new_left - parent_x, -(new_top - parent_y)))
}

fn clamp_axis_to_viewport(position: f32, size: f32, viewport_size: f32) -> f32 {
    if size >= viewport_size {
        0.0
    } else {
        position.clamp(0.0, viewport_size - size)
    }
}

fn reanchor_moving_drag_frame(
    state: &mut crate::lua_api::SimState,
    drag_id: u64,
    parent_id: Option<u64>,
    x_offset: f32,
    y_offset: f32,
) {
    state.widgets.remove_all_anchor_dependents_for(drag_id);
    if let Some(parent_id) = parent_id {
        state.widgets.add_anchor_dependent(parent_id, drag_id);
    }

    if let Some(frame) = state.widgets.get_mut_visual(drag_id) {
        frame.clear_all_points();
        frame.set_point(
            AnchorPoint::TopLeft,
            parent_id.map(|id| id as usize),
            AnchorPoint::TopLeft,
            x_offset,
            y_offset,
        );
    }
    state.widgets.mark_rect_dirty(drag_id);
}

fn find_drag_script_target(
    env: &crate::lua_api::WowLuaEnv,
    frame_id: u64,
    script_name: &str,
) -> Option<u64> {
    let mut current = Some(frame_id);
    while let Some(id) = current {
        if env.has_script_handler(id, script_name) {
            return Some(id);
        }
        current = env
            .state()
            .borrow()
            .widgets
            .get(id)
            .and_then(|f| f.parent_id);
    }
    None
}

fn find_slider_drag_target(state: &crate::lua_api::state::SimState, frame_id: u64) -> Option<u64> {
    let mut current = Some(frame_id);
    while let Some(id) = current {
        let frame = state.widgets.get(id)?;
        if frame.widget_type == WidgetType::Slider {
            return Some(id);
        }
        current = frame.parent_id;
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::iced_app::{build_hittable_rects, frame_collect::collect_hittable_frames};
    use crate::lua_api::WowLuaEnv;
    use crate::render::{GlyphAtlas, WowFontSystem};
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
        env.borrow().set_screen_size(800.0, 600.0);

        let texture_manager = Rc::new(RefCell::new(TextureManager::new(PathBuf::from(
            "./textures",
        ))));
        let font_system = Rc::new(RefCell::new(WowFontSystem::new(&PathBuf::from(
            super::super::app::DEFAULT_FONTS_PATH,
        ))));
        let glyph_atlas = Rc::new(RefCell::new(GlyphAtlas::new()));
        let (_cmd_tx, cmd_rx) = mpsc::channel(1);
        let (_lua_tx, lua_rx) = std::sync::mpsc::channel();

        let app = App::build_app(
            Rc::clone(&env),
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
        );
        app.screen_size.set(Size::new(800.0, 600.0));
        app
    }

    fn rebuild_hittable_cache(app: &App) {
        let env = app.env.borrow();
        let mut state = env.state().borrow_mut();
        state.ensure_layout_rects();
        let strata_buckets = state
            .get_strata_buckets()
            .expect("visible strata buckets should exist")
            .clone();
        let collected = collect_hittable_frames(&state.widgets, &strata_buckets);
        let hittable = build_hittable_rects(&collected, &state.widgets);
        let grid = super::super::hit_grid::HitGrid::new(hittable, 800.0, 600.0);
        *app.cached_hittable.borrow_mut() = Some(grid);
    }

    fn setup_pass_through_test_frames(app: &App) {
        let env = app.env.borrow();
        env.exec(
            r#"
            PassThroughParent = CreateFrame("Button", "PassThroughParent", UIParent)
            PassThroughParent:SetSize(100, 100)
            PassThroughParent:SetPoint("TOPLEFT", UIParent, "TOPLEFT", 100, -100)
            PassThroughParent:EnableMouse(true)
            PassThroughParent:SetScript("OnClick", function(_, button)
                if button == "LeftButton" then
                    __pass_parent_left = (__pass_parent_left or 0) + 1
                elseif button == "RightButton" then
                    __pass_parent_right = (__pass_parent_right or 0) + 1
                end
            end)

            PassThroughChild = CreateFrame("Button", "PassThroughChild", PassThroughParent)
            PassThroughChild:SetAllPoints(PassThroughParent)
            PassThroughChild:EnableMouse(true)
            PassThroughChild:SetScript("OnClick", function(_, button)
                if button == "LeftButton" then
                    __pass_child_left = (__pass_child_left or 0) + 1
                elseif button == "RightButton" then
                    __pass_child_right = (__pass_child_right or 0) + 1
                end
            end)

            PassThroughChild:SetPassThroughButtons("RightButton")

            __pass_parent_left = 0
            __pass_parent_right = 0
            __pass_child_left = 0
            __pass_child_right = 0
            "#,
        )
        .expect("pass-through frame setup should succeed");
    }

    fn read_pass_through_counters(app: &App) -> (f64, f64, f64, f64) {
        app.env
            .borrow()
            .eval("return __pass_parent_left, __pass_parent_right, __pass_child_left, __pass_child_right")
            .expect("pass-through counters should be readable")
    }

    fn clear_pass_through_buttons(app: &App) {
        let env = app.env.borrow();
        env.exec(
            r#"
            PassThroughChild:SetPassThroughButtons()
            __pass_parent_right = 0
            __pass_child_right = 0
            "#,
        )
        .expect("clearing pass-through buttons should succeed");
    }

    #[test]
    fn mouse_wheel_dispatch_requires_frame_mouse_wheel_enabled() {
        let mut app = build_test_app(ScreenKind::Game);

        {
            let env = app.env.borrow();
            env.exec(
                r#"
                MouseWheelDispatchFrame = CreateFrame("Frame", "MouseWheelDispatchFrame", UIParent)
                MouseWheelDispatchFrame:SetSize(100, 100)
                MouseWheelDispatchFrame:SetPoint("TOPLEFT", UIParent, "TOPLEFT", 100, -100)
                MouseWheelDispatchFrame:EnableMouse(true)
                MouseWheelDispatchFrame:SetScript("OnMouseWheel", function(_, delta)
                    __wheel_delta = (__wheel_delta or 0) + delta
                end)
                __wheel_delta = 0
                "#,
            )
            .expect("test frame setup should succeed");
        }

        rebuild_hittable_cache(&app);
        app.handle_mouse_move(Point::new(150.0, 150.0));
        app.handle_scroll(0.0, -1.0);

        let not_delivered: f64 = app
            .env
            .borrow()
            .eval("return __wheel_delta")
            .expect("wheel delta query should succeed");
        assert_eq!(
            not_delivered, 0.0,
            "mouse wheel scripts should not fire while mouse wheel is disabled"
        );

        {
            let env = app.env.borrow();
            env.exec("MouseWheelDispatchFrame:EnableMouseWheel(true)")
                .expect("enabling mouse wheel should succeed");
        }

        rebuild_hittable_cache(&app);
        app.handle_mouse_move(Point::new(150.0, 150.0));
        app.handle_scroll(0.0, -1.0);

        let delivered: f64 = app
            .env
            .borrow()
            .eval("return __wheel_delta")
            .expect("wheel delta query should succeed");
        assert_eq!(
            delivered, -1.0,
            "mouse wheel scripts should fire once the frame enables mouse wheel"
        );
    }

    #[test]
    fn drag_start_can_transfer_to_delegate_and_abort_before_mouse_up() {
        let mut app = build_test_app(ScreenKind::Game);

        {
            let env = app.env.borrow();
            env.exec(
                r#"
                DragSourceFrame = CreateFrame("Frame", "DragSourceFrame", UIParent)
                DragSourceFrame:SetSize(100, 100)
                DragSourceFrame:SetPoint("TOPLEFT", UIParent, "TOPLEFT", 100, -100)
                DragSourceFrame:EnableMouse(true)
                DragSourceFrame:SetScript("OnDragStart", function(self)
                    self:InterceptStartDrag(DragDelegateFrame)
                    DragDelegateFrame:AbortDrag()
                end)

                DragDelegateFrame = CreateFrame("Frame", "DragDelegateFrame", UIParent)
                DragDelegateFrame:SetScript("OnDragStop", function()
                    __drag_stop_calls = (__drag_stop_calls or 0) + 1
                end)

                __drag_stop_calls = 0
                "#,
            )
            .expect("drag test frame setup should succeed");
        }

        rebuild_hittable_cache(&app);
        app.handle_mouse_move(Point::new(150.0, 150.0));
        app.handle_mouse_down(Point::new(150.0, 150.0));
        app.handle_mouse_move(Point::new(170.0, 170.0));

        let active_drag_frame = app.env.borrow().state().borrow().active_drag_frame;
        assert_eq!(
            active_drag_frame, None,
            "AbortDrag during OnDragStart should clear the active drag frame immediately"
        );

        let delegate_dragging: bool = app
            .env
            .borrow()
            .eval("return DragDelegateFrame:IsDragging()")
            .expect("delegate dragging query should succeed");
        assert!(
            !delegate_dragging,
            "delegate should not report dragging after AbortDrag clears the transfer"
        );

        app.handle_mouse_up(Point::new(170.0, 170.0));

        let drag_stop_calls: f64 = app
            .env
            .borrow()
            .eval("return __drag_stop_calls")
            .expect("drag stop count query should succeed");
        assert_eq!(
            drag_stop_calls, 0.0,
            "mouse up should not fire OnDragStop after AbortDrag already cleared the drag"
        );
    }

    #[test]
    fn moving_drag_updates_frame_anchor_to_follow_mouse() {
        let mut app = build_test_app(ScreenKind::Game);

        {
            let env = app.env.borrow();
            env.exec(
                r#"
                MovingDragFrame = CreateFrame("Frame", "MovingDragFrame", UIParent)
                MovingDragFrame:SetSize(100, 100)
                MovingDragFrame:SetPoint("CENTER", UIParent, "TOPLEFT", 150, -150)
                MovingDragFrame:SetMovable(true)
                MovingDragFrame:EnableMouse(true)
                MovingDragFrame:RegisterForDrag("LeftButton")
                MovingDragFrame:SetScript("OnDragStart", function(self)
                    self:StartMoving()
                end)
                "#,
            )
            .expect("moving drag frame setup should succeed");
        }

        rebuild_hittable_cache(&app);
        app.handle_mouse_move(Point::new(150.0, 150.0));
        app.handle_mouse_down(Point::new(150.0, 150.0));
        app.handle_mouse_move(Point::new(170.0, 170.0));

        let (num_points, point, relative_to, relative_point, x, y): (
            i64,
            String,
            String,
            String,
            f64,
            f64,
        ) = app
            .env
            .borrow()
            .eval(
                r#"
                local point, relativeTo, relativePoint, x, y = MovingDragFrame:GetPoint(1)
                local relativeName = relativeTo and relativeTo:GetName() or "nil"
                return MovingDragFrame:GetNumPoints(), point, relativeName, relativePoint, x, y
            "#,
            )
            .expect("moving drag anchor query should succeed");

        assert_eq!(
            num_points, 1,
            "moving drag should collapse anchors to one TOPLEFT point"
        );
        assert_eq!(
            point, "TOPLEFT",
            "moving drag should re-anchor the frame by TOPLEFT"
        );
        assert_eq!(
            relative_to, "UIParent",
            "moving drag should anchor relative to the parent"
        );
        assert_eq!(
            relative_point, "TOPLEFT",
            "moving drag should use the parent's TOPLEFT as its target"
        );
        assert_eq!(x, 120.0, "mouse delta should advance the frame x offset");
        assert_eq!(y, -120.0, "mouse delta should advance the frame y offset");
    }

    #[test]
    fn moving_drag_clamps_frame_to_screen_when_enabled() {
        let mut app = build_test_app(ScreenKind::Game);

        {
            let env = app.env.borrow();
            env.exec(
                r#"
                ClampedMovingDragFrame = CreateFrame("Frame", "ClampedMovingDragFrame", UIParent)
                ClampedMovingDragFrame:SetSize(100, 100)
                ClampedMovingDragFrame:SetPoint("TOPLEFT", UIParent, "TOPLEFT", 10, -10)
                ClampedMovingDragFrame:SetMovable(true)
                ClampedMovingDragFrame:SetClampedToScreen(true)
                ClampedMovingDragFrame:EnableMouse(true)
                ClampedMovingDragFrame:RegisterForDrag("LeftButton")
                ClampedMovingDragFrame:SetScript("OnDragStart", function(self)
                    self:StartMoving()
                end)
                "#,
            )
            .expect("clamped moving drag frame setup should succeed");
        }

        rebuild_hittable_cache(&app);
        app.handle_mouse_move(Point::new(20.0, 20.0));
        app.handle_mouse_down(Point::new(20.0, 20.0));
        app.handle_mouse_move(Point::new(-80.0, -80.0));

        let (point, relative_to, relative_point, x, y): (String, String, String, f64, f64) = app
            .env
            .borrow()
            .eval(
                r#"
                local point, relativeTo, relativePoint, x, y = ClampedMovingDragFrame:GetPoint(1)
                local relativeName = relativeTo and relativeTo:GetName() or "nil"
                return point, relativeName, relativePoint, x, y
            "#,
            )
            .expect("clamped moving drag anchor query should succeed");

        assert_eq!(point, "TOPLEFT");
        assert_eq!(relative_to, "UIParent");
        assert_eq!(relative_point, "TOPLEFT");
        assert_eq!(
            x, 0.0,
            "clamped moving drag should not move past the left edge"
        );
        assert_eq!(
            y, 0.0,
            "clamped moving drag should not move past the top edge"
        );
    }

    #[test]
    fn slider_reports_thumb_drag_state_while_mouse_is_held() {
        let mut app = build_test_app(ScreenKind::Game);

        {
            let env = app.env.borrow();
            env.exec(
                r#"
                SliderThumbDragFrame = CreateFrame("Slider", "SliderThumbDragFrame", UIParent)
                SliderThumbDragFrame:SetSize(120, 20)
                SliderThumbDragFrame:SetPoint("TOPLEFT", UIParent, "TOPLEFT", 100, -100)
                SliderThumbDragFrame:EnableMouse(true)
                "#,
            )
            .expect("slider setup should succeed");
        }

        rebuild_hittable_cache(&app);
        let drag_pos = Point::new(150.0, 110.0);

        let before_mouse_down: bool = app
            .env
            .borrow()
            .eval("return SliderThumbDragFrame:IsDraggingThumb()")
            .expect("initial thumb drag query should succeed");
        assert!(
            !before_mouse_down,
            "fresh sliders should not report thumb dragging"
        );

        app.handle_mouse_move(drag_pos);
        app.handle_mouse_down(drag_pos);

        let during_mouse_down: bool = app
            .env
            .borrow()
            .eval("return SliderThumbDragFrame:IsDraggingThumb()")
            .expect("active thumb drag query should succeed");
        assert!(
            during_mouse_down,
            "slider should report thumb dragging while left mouse is held on it"
        );

        app.handle_mouse_up(drag_pos);

        let after_mouse_up: bool = app
            .env
            .borrow()
            .eval("return SliderThumbDragFrame:IsDraggingThumb()")
            .expect("post mouse up thumb drag query should succeed");
        assert!(
            !after_mouse_up,
            "slider thumb drag state should clear on mouse up"
        );
    }

    #[test]
    fn pass_through_buttons_reroute_clicks_and_can_be_cleared() {
        let mut app = build_test_app(ScreenKind::Game);
        setup_pass_through_test_frames(&app);

        rebuild_hittable_cache(&app);
        let click_pos = Point::new(150.0, 150.0);

        app.handle_mouse_down(click_pos);
        app.handle_mouse_up(click_pos);
        app.handle_right_mouse_down(click_pos);
        app.handle_right_mouse_up(click_pos);

        let (parent_left, parent_right, child_left, child_right) = read_pass_through_counters(&app);
        assert_eq!(parent_left, 0.0, "left click should not pass through");
        assert_eq!(
            parent_right, 1.0,
            "right click should pass through to parent"
        );
        assert_eq!(
            child_left, 1.0,
            "child should still receive non-passthrough left clicks"
        );
        assert_eq!(
            child_right, 0.0,
            "child should not receive passthrough right clicks"
        );

        clear_pass_through_buttons(&app);
        rebuild_hittable_cache(&app);
        app.handle_right_mouse_down(click_pos);
        app.handle_right_mouse_up(click_pos);

        let (_, cleared_parent_right, _, cleared_child_right) = read_pass_through_counters(&app);
        assert_eq!(
            cleared_parent_right, 0.0,
            "clearing pass-through buttons should stop rerouting right clicks"
        );
        assert_eq!(
            cleared_child_right, 1.0,
            "child should receive right clicks again after passthrough is cleared"
        );
    }

    #[test]
    fn disabled_buttons_only_run_hover_motion_scripts_when_opted_in() {
        let mut app = build_test_app(ScreenKind::Game);

        {
            let env = app.env.borrow();
            env.exec(
                r#"
                DisabledMotionButton = CreateFrame("Button", "DisabledMotionButton", UIParent)
                DisabledMotionButton:SetSize(100, 100)
                DisabledMotionButton:SetPoint("TOPLEFT", UIParent, "TOPLEFT", 100, -100)
                DisabledMotionButton:EnableMouse(true)
                DisabledMotionButton:Disable()
                DisabledMotionButton:SetScript("OnEnter", function()
                    __disabled_motion_enter = (__disabled_motion_enter or 0) + 1
                end)
                DisabledMotionButton:SetScript("OnLeave", function()
                    __disabled_motion_leave = (__disabled_motion_leave or 0) + 1
                end)
                __disabled_motion_enter = 0
                __disabled_motion_leave = 0
                "#,
            )
            .expect("disabled motion test button setup should succeed");
        }

        rebuild_hittable_cache(&app);
        let outside_pos = Point::new(20.0, 20.0);
        let inside_pos = Point::new(150.0, 150.0);

        app.handle_mouse_move(outside_pos);
        app.handle_mouse_move(inside_pos);
        app.handle_mouse_move(outside_pos);

        let (blocked_enter, blocked_leave, default_flag): (f64, f64, bool) = app
            .env
            .borrow()
            .eval(
                "return __disabled_motion_enter, __disabled_motion_leave, DisabledMotionButton:GetMotionScriptsWhileDisabled()",
            )
            .expect("default disabled motion state should be readable");
        assert_eq!(
            blocked_enter, 0.0,
            "disabled buttons should not receive OnEnter by default"
        );
        assert_eq!(
            blocked_leave, 0.0,
            "disabled buttons should not receive OnLeave by default"
        );
        assert!(
            !default_flag,
            "disabled buttons should default motion scripts while disabled to false"
        );

        {
            let env = app.env.borrow();
            env.exec(
                r#"
                DisabledMotionButton:SetMotionScriptsWhileDisabled(true)
                __disabled_motion_enter = 0
                __disabled_motion_leave = 0
                "#,
            )
            .expect("enabling motion scripts while disabled should succeed");
        }

        app.handle_mouse_move(outside_pos);
        app.handle_mouse_move(inside_pos);
        app.handle_mouse_move(outside_pos);

        let (allowed_enter, allowed_leave, enabled_flag): (f64, f64, bool) = app
            .env
            .borrow()
            .eval(
                "return __disabled_motion_enter, __disabled_motion_leave, DisabledMotionButton:GetMotionScriptsWhileDisabled()",
            )
            .expect("enabled disabled-motion state should be readable");
        assert_eq!(
            allowed_enter, 1.0,
            "disabled buttons should receive OnEnter after opting into motion scripts"
        );
        assert_eq!(
            allowed_leave, 1.0,
            "disabled buttons should receive OnLeave after opting into motion scripts"
        );
        assert!(
            enabled_flag,
            "GetMotionScriptsWhileDisabled should reflect the opt-in flag"
        );
    }
}
