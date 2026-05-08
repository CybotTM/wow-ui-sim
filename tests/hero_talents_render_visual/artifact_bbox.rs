use super::*;

#[test]
fn historical_bottom_right_artifact_point_is_only_background_marble_in_current_render() {
    let env = setup_full_ui();
    open_class_talent_frame(&env);
    env.set_screen_size(1600.0, 1200.0);

    let buckets = {
        let mut state = env.state().borrow_mut();
        state.ensure_layout_rects();
        let _ = state.get_strata_buckets();
        state.strata_buckets.as_ref().unwrap().clone()
    };
    let state = env.state().borrow();
    let batch = build_quad_batch_for_registry(RegistryQuadBatchParams::new(
        &state.widgets,
        (1600.0, 1200.0),
        &buckets,
    ));

    let matches: Vec<_> = batch
        .texture_requests
        .iter()
        .filter(|request| request_contains_point(&batch, request, 1000.0, 610.0))
        .map(|request| request.path.as_str())
        .collect();

    assert_eq!(
        matches,
        vec!["framegeneral/ui-background-marble"],
        "Historical bottom-right artifact point should only intersect the class-talent marble background in the current render"
    );
}

#[test]
fn lower_right_artifact_bbox_matches_background_marble_in_raw_player_spells_render() {
    if common::try_create_gpu_device().is_none() {
        eprintln!("Skipping GPU render test: no adapter available");
        return;
    }

    let env = setup_full_ui();
    open_class_talent_frame(&env);
    let batch = build_screenshot_like_batch(&env, 1600, 1200, Some("PlayerSpellsFrame"));

    let artifact_rect = (1134.0, 664.0, 40.0, 45.0);
    let texture_matches: Vec<_> = batch
        .texture_requests
        .iter()
        .filter(|request| {
            request.path != "framegeneral/ui-background-marble"
                && request_overlaps_rect(&batch, request, artifact_rect)
        })
        .map(|request| request.path.as_str())
        .collect();
    assert!(
        texture_matches.is_empty(),
        "artifact bbox should not overlap non-background texture requests: {texture_matches:#?}"
    );

    let mask_matches: Vec<_> = batch
        .mask_texture_requests
        .iter()
        .filter(|request| request_overlaps_rect(&batch, request, artifact_rect))
        .map(|request| request.path.as_str())
        .collect();
    assert!(
        mask_matches.is_empty(),
        "artifact bbox should not overlap mask requests: {mask_matches:#?}"
    );

    let solid_matches: Vec<_> = batch
        .vertices
        .chunks_exact(4)
        .enumerate()
        .filter_map(|(quad_idx, verts)| {
            if verts[0].tex_index != -1 {
                return None;
            }
            let vertex_start = quad_idx * 4;
            let bounds = vertex_range_bounds(&batch, vertex_start, 4);
            bounds_overlap_rect(bounds, artifact_rect).then_some((
                quad_idx,
                bounds,
                verts[0].color,
                verts[1].color,
                verts[2].color,
                verts[3].color,
            ))
        })
        .collect();
    assert!(
        solid_matches.is_empty(),
        "artifact bbox should not overlap solid-color quads: {solid_matches:#?}"
    );

    let mut raw_mgr = make_texture_manager();
    let raw_render = render_to_image(&batch, &mut raw_mgr, 1600, 1200, None);
    let mut marble_mgr = make_texture_manager();
    let marble_render = render_to_image(
        &marble_only_batch(1600, 1200),
        &mut marble_mgr,
        1600,
        1200,
        None,
    );
    assert_images_match_rect(
        &raw_render,
        &marble_render,
        (1134, 664, 40, 45),
        "lower-right artifact bbox",
    );
}
