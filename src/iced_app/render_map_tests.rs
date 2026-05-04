    #[test]
    fn request_preload_map_warms_map_art_and_overlay_textures() {
        let Some((map_id, art_path, overlay_path)) = first_map_with_art_and_overlay_paths() else {
            eprintln!("Skipping test: no map with both art and exploration overlay textures found");
            return;
        };

        let app = build_test_app();
        app.env
            .borrow()
            .exec(&format!("C_Map.RequestPreloadMap({map_id})"))
            .expect("RequestPreloadMap should succeed");

        assert!(
            app.texture_manager.borrow().get(&art_path).is_none(),
            "map art texture should not already be cached before preload runs"
        );
        assert!(
            app.texture_manager.borrow().get(&overlay_path).is_none(),
            "map overlay texture should not already be cached before preload runs"
        );

        for _ in 0..10 {
            app.preload_current_render_requests(Some(std::time::Duration::from_secs(1)));
            let tex_mgr = app.texture_manager.borrow();
            if tex_mgr.is_cached(&art_path) && tex_mgr.is_cached(&overlay_path) {
                break;
            }
        }

        let tex_mgr = app.texture_manager.borrow();
        assert!(
            tex_mgr.is_cached(&art_path),
            "RequestPreloadMap should warm map art tile texture {art_path}"
        );
        assert!(
            tex_mgr.is_cached(&overlay_path),
            "RequestPreloadMap should warm exploration overlay texture {overlay_path}"
        );
    }

    #[test]
    fn resolve_layout_and_buckets_recomputes_tooltip_layout_after_sizing() {
        let app = build_test_app();
        app.env
            .borrow()
            .exec(
                r#"
                local owner = CreateFrame("Frame", "TooltipLayoutOwner", UIParent)
                owner:SetSize(100, 50)
                owner:SetPoint("CENTER", UIParent, "CENTER", 0, 0)
                GameTooltip:SetOwner(owner, "ANCHOR_RIGHT")
                GameTooltip:AddLine("Tooltip layout must resize before render buckets lock in")
            "#,
            )
            .expect("tooltip setup should succeed");

        {
            let env = app.env.borrow();
            let mut font_sys = app.font_system.borrow_mut();
            let _ = app.resolve_layout_and_buckets(&env, &mut font_sys);
        }

        let state_ref = app.env.borrow();
        let state = state_ref.state().borrow();
        let tooltip_id = state
            .widgets
            .get_id_by_name("GameTooltip")
            .expect("GameTooltip should exist");
        let tooltip = state
            .widgets
            .get(tooltip_id)
            .expect("GameTooltip frame should exist");
        let tooltip_rect = tooltip
            .layout_rect
            .expect("render prep should resolve the tooltip layout rect");

        assert!(
            (tooltip_rect.width - tooltip.width).abs() < f32::EPSILON,
            "tooltip layout width {} should match sized width {} after render prep",
            tooltip_rect.width,
            tooltip.width
        );
        assert!(
            (tooltip_rect.height - tooltip.height).abs() < f32::EPSILON,
            "tooltip layout height {} should match sized height {} after render prep",
            tooltip_rect.height,
            tooltip.height
        );
    }
