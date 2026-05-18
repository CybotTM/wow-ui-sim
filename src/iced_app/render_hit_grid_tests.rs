#[cfg(test)]
mod hit_grid_tests {
    use super::*;
    use iced::Point;
    use rustc_hash::FxHashSet;

    fn dirty_mask(strata: usize) -> u16 {
        1u16 << strata
    }

    fn build_test_app() -> App {
        super::test_support::build_test_app()
    }

    #[test]
    fn layout_move_updates_cached_hit_grid_rects() {
        let app = build_test_app();
        app.env
            .borrow()
            .exec(
                r#"
                MovingHitButton = CreateFrame("Button", "MovingHitButton", UIParent)
                MovingHitButton:SetSize(100, 40)
                MovingHitButton:SetPoint("TOPLEFT", UIParent, "TOPLEFT", 20, -20)
                MovingHitButton:EnableMouse(true)
                MovingHitButton:RegisterForClicks("LeftButtonUp")
            "#,
            )
            .expect("moving hit button setup should succeed");

        let size = Size::new(800.0, 600.0);
        app.mark_all_strata_dirty();
        app.rebuild_dirty_strata(size, app.strata_dirty.get());

        let original_point = Point::new(30.0, 30.0);
        let moved_point = Point::new(330.0, 30.0);
        let initial_target = app.hit_test_mouse_button(original_point, "LeftButton");
        assert!(initial_target.is_some(), "button should start at original rect");

        app.env
            .borrow()
            .exec(
                r#"
                MovingHitButton:ClearAllPoints()
                MovingHitButton:SetPoint("TOPLEFT", UIParent, "TOPLEFT", 320, -20)
            "#,
            )
            .expect("moving hit button should succeed");
        rebuild_after_widget_dirty(&app, size);

        let stale_target = app.hit_test_mouse_button(original_point, "LeftButton");
        let moved_target = app.hit_test_mouse_button(moved_point, "LeftButton");
        assert_eq!(
            stale_target, None,
            "original rect should stop accepting clicks after layout moves"
        );
        assert_eq!(
            moved_target, initial_target,
            "moved rect should accept clicks after layout rebuild"
        );
    }

    #[test]
    fn layout_only_move_updates_alpha_zero_hit_grid_rects() {
        let app = build_test_app();
        app.env
            .borrow()
            .exec(
                r#"
                AlphaZeroMovingHitButton = CreateFrame("Button", "AlphaZeroMovingHitButton", UIParent)
                AlphaZeroMovingHitButton:SetSize(100, 40)
                AlphaZeroMovingHitButton:SetPoint("TOPLEFT", UIParent, "TOPLEFT", 20, -20)
                AlphaZeroMovingHitButton:EnableMouse(true)
                AlphaZeroMovingHitButton:RegisterForClicks("LeftButtonUp")
                AlphaZeroMovingHitButton:SetAlpha(0)
            "#,
            )
            .expect("alpha-zero moving hit button setup should succeed");

        let size = Size::new(800.0, 600.0);
        app.mark_all_strata_dirty();
        app.rebuild_dirty_strata(size, app.strata_dirty.get());

        let original_point = Point::new(30.0, 30.0);
        let moved_point = Point::new(330.0, 30.0);
        let initial_target = app.hit_test_mouse_button(original_point, "LeftButton");
        assert!(
            initial_target.is_some(),
            "alpha-zero button should start at original rect"
        );

        move_alpha_zero_button(&app);
        let (frame_id, strata_index) = alpha_zero_frame_id_and_strata(&app);
        drain_render_dirty_ids(&app);
        app.mark_strata_dirty(dirty_mask(strata_index));
        app.merge_pending_dirty_ids(Some(FxHashSet::from_iter([frame_id])));
        let rebuilt = app.rebuild_dirty_strata(size, dirty_mask(strata_index));

        let stale_target = app.hit_test_mouse_button(original_point, "LeftButton");
        let moved_target = app.hit_test_mouse_button(moved_point, "LeftButton");
        assert_eq!(
            rebuilt, 0,
            "alpha-zero hit-only movement should reproduce the pruned render path"
        );
        assert_eq!(
            stale_target, None,
            "original rect should stop accepting clicks after alpha-zero layout moves"
        );
        assert_eq!(
            moved_target, initial_target,
            "moved alpha-zero rect should accept clicks after pruned layout rebuild"
        );
    }

    #[test]
    fn hit_rect_insets_scale_with_frame_scale() {
        let app = build_test_app();
        app.env
            .borrow()
            .exec(
                r#"
                ScaledHitInsetButton = CreateFrame("Button", "ScaledHitInsetButton", UIParent)
                ScaledHitInsetButton:SetSize(50, 50)
                ScaledHitInsetButton:SetScale(0.5)
                ScaledHitInsetButton:SetHitRectInsets(6, 6, 7, 7)
                ScaledHitInsetButton:SetPoint("TOPLEFT", UIParent, "TOPLEFT", 100, -100)
                ScaledHitInsetButton:EnableMouse(true)
            "#,
            )
            .expect("scaled hit inset setup should succeed");

        let env = app.env.borrow();
        let mut state = env.state().borrow_mut();
        state.ensure_layout_rects();
        let strata_buckets = state
            .get_strata_buckets()
            .expect("visible strata buckets should exist")
            .clone();
        let frame_id = state
            .widgets
            .get_id_by_name("ScaledHitInsetButton")
            .expect("scaled hit inset button should exist");
        let collected =
            super::super::frame_collect::collect_hittable_frames(&state.widgets, &strata_buckets);
        let hittable = build_hittable_rects(&collected, &state.widgets);
        let (_, rect) = hittable
            .iter()
            .find(|(id, _)| *id == frame_id)
            .expect("scaled hit inset button should be hittable");

        assert_eq!(rect.width, 19.0);
        assert_eq!(rect.height, 18.0);
    }

    fn move_alpha_zero_button(app: &App) {
        app.env
            .borrow()
            .exec(
                r#"
                AlphaZeroMovingHitButton:ClearAllPoints()
                AlphaZeroMovingHitButton:SetPoint("TOPLEFT", UIParent, "TOPLEFT", 320, -20)
            "#,
            )
            .expect("moving alpha-zero hit button should succeed");
    }

    fn alpha_zero_frame_id_and_strata(app: &App) -> (u64, usize) {
        let env = app.env.borrow();
        let state = env.state().borrow();
        let frame_id = state
            .widgets
            .iter_ids()
            .find(|id| {
                state
                    .widgets
                    .get(*id)
                    .is_some_and(|f| f.name.as_deref() == Some("AlphaZeroMovingHitButton"))
            })
            .expect("alpha-zero moving hit button should exist");
        let strata_index = state
            .widgets
            .get(frame_id)
            .expect("alpha-zero moving hit button should be registered")
            .frame_strata
            .as_index();
        (frame_id, strata_index)
    }

    fn drain_render_dirty_ids(app: &App) {
        let _ = app
            .env
            .borrow()
            .state()
            .borrow()
            .widgets
            .take_render_dirty_with_ids();
    }

    fn rebuild_after_widget_dirty(app: &App, size: Size) -> u16 {
        let (dirty_mask, dirty_ids) = app
            .env
            .borrow()
            .state()
            .borrow()
            .widgets
            .take_render_dirty_with_ids();
        app.mark_strata_dirty(dirty_mask);
        app.merge_pending_dirty_ids(dirty_ids);
        app.rebuild_dirty_strata(size, dirty_mask)
    }
}
