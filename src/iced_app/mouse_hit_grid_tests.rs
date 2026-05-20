use super::test_support::*;
use super::*;
use crate::screen::ScreenKind;

#[test]
fn hit_grid_tracks_frame_moved_after_cache_build() {
    let app = build_test_app(ScreenKind::Game);

    {
        let env = app.env.borrow();
        env.exec(
            r#"
            MovedHitFrame = CreateFrame("Button", "MovedHitFrame", UIParent)
            MovedHitFrame:SetSize(100, 100)
            MovedHitFrame:SetPoint("TOPLEFT", UIParent, "TOPLEFT", 100, -100)
            MovedHitFrame:EnableMouse(true)
            "#,
        )
        .expect("moved hit frame setup should succeed");
    }

    rebuild_hittable_cache(&app);

    {
        let env = app.env.borrow();
        env.exec(
            r#"
            MovedHitFrame:ClearAllPoints()
            MovedHitFrame:SetPoint("TOPLEFT", UIParent, "TOPLEFT", 300, -100)
            "#,
        )
        .expect("moving hit frame should succeed");
    }

    let moved_id = frame_id_by_name(&app, "MovedHitFrame");
    assert_eq!(
        app.hit_test(Point::new(350.0, 150.0)),
        Some(moved_id),
        "hit grid should use the moved frame rect"
    );
    assert_ne!(
        app.hit_test(Point::new(150.0, 150.0)),
        Some(moved_id),
        "old frame rect should not remain hittable"
    );
}

#[test]
fn hit_grid_tracks_child_when_parent_moves_after_cache_build() {
    let app = build_test_app(ScreenKind::Game);

    {
        let env = app.env.borrow();
        env.exec(
            r#"
            MovedHitParent = CreateFrame("Frame", "MovedHitParent", UIParent)
            MovedHitParent:SetSize(120, 120)
            MovedHitParent:SetPoint("TOPLEFT", UIParent, "TOPLEFT", 100, -100)

            MovedHitChild = CreateFrame("Button", "MovedHitChild", MovedHitParent)
            MovedHitChild:SetSize(40, 40)
            MovedHitChild:SetPoint("TOPLEFT", MovedHitParent, "TOPLEFT", 10, -10)
            MovedHitChild:EnableMouse(true)
            "#,
        )
        .expect("moved child hit frame setup should succeed");
    }

    rebuild_hittable_cache(&app);

    {
        let env = app.env.borrow();
        env.exec(
            r#"
            MovedHitParent:ClearAllPoints()
            MovedHitParent:SetPoint("TOPLEFT", UIParent, "TOPLEFT", 300, -100)
            "#,
        )
        .expect("moving parent should succeed");
    }

    let child_id = frame_id_by_name(&app, "MovedHitChild");
    assert_eq!(
        app.hit_test(Point::new(320.0, 120.0)),
        Some(child_id),
        "child hit grid entry should move with its parent"
    );
    assert_ne!(
        app.hit_test(Point::new(120.0, 120.0)),
        Some(child_id),
        "child should not remain hittable at the old parent position"
    );
}

#[test]
fn hit_grid_tracks_moved_child_after_layout_was_resolved_elsewhere() {
    let app = build_test_app(ScreenKind::Game);

    {
        let env = app.env.borrow();
        env.exec(
            r#"
            ResolvedMoveParent = CreateFrame("Frame", "ResolvedMoveParent", UIParent)
            ResolvedMoveParent:SetSize(120, 120)
            ResolvedMoveParent:SetPoint("TOPLEFT", UIParent, "TOPLEFT", 100, -100)

            ResolvedMoveChild = CreateFrame("Button", "ResolvedMoveChild", ResolvedMoveParent)
            ResolvedMoveChild:SetSize(40, 40)
            ResolvedMoveChild:SetPoint("TOPLEFT", ResolvedMoveParent, "TOPLEFT", 10, -10)
            ResolvedMoveChild:EnableMouse(true)
            "#,
        )
        .expect("resolved move setup should succeed");
    }

    rebuild_hittable_cache(&app);

    {
        let env = app.env.borrow();
        env.exec(
            r#"
            ResolvedMoveParent:ClearAllPoints()
            ResolvedMoveParent:SetPoint("TOPLEFT", UIParent, "TOPLEFT", 300, -100)
            "#,
        )
        .expect("moving parent should succeed");
        env.state().borrow_mut().ensure_layout_rects();
    }

    let child_id = frame_id_by_name(&app, "ResolvedMoveChild");
    assert_eq!(
        app.hit_test(Point::new(320.0, 120.0)),
        Some(child_id),
        "hit grid should still update after another path resolved layout first"
    );
    assert_ne!(
        app.hit_test(Point::new(120.0, 120.0)),
        Some(child_id),
        "old child hit grid entry should be removed after layout was pre-resolved"
    );
}

fn frame_id_by_name(app: &App, name: &str) -> u64 {
    let env = app.env.borrow();
    let state = env.state().borrow();
    state
        .widgets
        .iter_ids()
        .find(|&id| state.widgets.get(id).and_then(|f| f.name.as_deref()) == Some(name))
        .expect("frame should exist")
}
