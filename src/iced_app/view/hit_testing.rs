use crate::iced_app::app::App;

impl App {
    /// Hit test to find frame under cursor (uses cached rects from render pass).
    ///
    /// After finding the topmost frame at the cursor position (highest strata/level),
    /// drills down through child frames to find the deepest mouse-enabled descendant.
    /// This matches WoW behavior where child frames always receive clicks over parents,
    /// regardless of the parent's frame level. Descent walks through any visible child
    /// whose visual rect contains the point — even non-mouse-enabled containers — so
    /// hittable descendants of a transparent panning/scroll container are still reachable.
    pub(crate) fn hit_test(&self, pos: iced::Point) -> Option<u64> {
        self.apply_hit_grid_changes();
        let cache = self.cached_hittable.borrow();
        let grid = cache.as_ref()?;

        let env = self.env.borrow();
        let state = env.state().borrow();
        let initial_id = grid.topmost_matching_at(pos, |_| true)?;
        deepest_hover_target_through_visible_children(&state.widgets, grid, initial_id, pos)
    }

    pub(crate) fn hit_test_mouse_button(
        &self,
        pos: iced::Point,
        button_name: &str,
        down: bool,
    ) -> Option<u64> {
        self.apply_hit_grid_changes();
        let cache = self.cached_hittable.borrow();
        let grid = cache.as_ref()?;

        let env = self.env.borrow();
        let state = env.state().borrow();
        let initial_id = grid.topmost_matching_at(pos, |id| {
            deepest_click_target_through_visible_children(
                &state.widgets,
                &env,
                grid,
                id,
                pos,
                button_name,
                down,
            )
            .is_some()
        })?;
        deepest_click_target_through_visible_children(
            &state.widgets,
            &env,
            grid,
            initial_id,
            pos,
            button_name,
            down,
        )
    }
}

fn deepest_hover_target_through_visible_children(
    widgets: &crate::widget::WidgetRegistry,
    grid: &crate::iced_app::hit_grid::HitGrid,
    frame_id: u64,
    pos: iced::Point,
) -> Option<u64> {
    let frame = widgets.get(frame_id)?;
    for child_id in visible_descendants_at_point_by_z_order(widgets, frame, pos) {
        if let Some(target) =
            deepest_hover_target_through_visible_children(widgets, grid, child_id, pos)
        {
            return Some(target);
        }
    }

    grid.contains(frame_id, pos).then_some(frame_id)
}

fn deepest_click_target_through_visible_children(
    widgets: &crate::widget::WidgetRegistry,
    env: &crate::lua_api::WowLuaEnv,
    grid: &crate::iced_app::hit_grid::HitGrid,
    frame_id: u64,
    pos: iced::Point,
    button_name: &str,
    down: bool,
) -> Option<u64> {
    let frame = widgets.get(frame_id)?;
    for child_id in visible_descendants_at_point_by_z_order(widgets, frame, pos) {
        if let Some(target) = deepest_click_target_through_visible_children(
            widgets,
            env,
            grid,
            child_id,
            pos,
            button_name,
            down,
        ) {
            return Some(target);
        }
    }

    if grid.contains(frame_id, pos)
        && frame_has_mouse_button_action(frame, env, frame_id, button_name, down)
    {
        Some(frame_id)
    } else {
        None
    }
}

fn visible_descendants_at_point_by_z_order(
    widgets: &crate::widget::WidgetRegistry,
    frame: &crate::widget::Frame,
    pos: iced::Point,
) -> Vec<u64> {
    let mut child_ids: Vec<_> = frame
        .children
        .iter()
        .copied()
        .filter(|&child_id| {
            widgets
                .get(child_id)
                .is_some_and(|child| child_visually_contains(child, pos))
        })
        .collect();
    child_ids.sort_by_key(|&child_id| {
        widgets
            .get(child_id)
            .map(|child| hit_sort_key(child, child_id))
            .unwrap_or_default()
    });
    child_ids.reverse();
    child_ids
}

/// Whether the child's visual bounds (visible+layout_rect, scaled by UI_SCALE)
/// contain the screen-space point. Used for hit-test descent through any
/// visible frame, regardless of mouse-enabled status.
fn child_visually_contains(child: &crate::widget::Frame, pos: iced::Point) -> bool {
    if !child.visible {
        return false;
    }
    let Some(rect) = child.layout_rect else {
        return false;
    };
    let scale = crate::render::texture::UI_SCALE;
    let x = rect.x * scale;
    let y = rect.y * scale;
    let w = rect.width * scale;
    let h = rect.height * scale;
    pos.x >= x && pos.x < x + w && pos.y >= y && pos.y < y + h
}

fn hit_sort_key(
    frame: &crate::widget::Frame,
    id: u64,
) -> (crate::widget::FrameStrata, i32, i32, u64) {
    (
        frame.frame_strata,
        frame.frame_level.saturating_add(frame.raise_order),
        frame.frame_level,
        id,
    )
}

fn frame_has_mouse_button_action(
    frame: &crate::widget::Frame,
    env: &crate::lua_api::WowLuaEnv,
    frame_id: u64,
    button_name: &str,
    down: bool,
) -> bool {
    if !crate::iced_app::frame_collect::frame_accepts_mouse_button(frame, button_name) {
        return false;
    }

    if crate::iced_app::mouse::frame_click_registration_accepts_button(frame, button_name) {
        return true;
    }

    let script = if down { "OnMouseDown" } else { "OnMouseUp" };
    crate::iced_app::frame_collect::frame_mouse_registration_matches(frame, button_name, down)
        && env.has_script_handler(frame_id, script)
}
