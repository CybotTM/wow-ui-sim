//! Render order tests: strata bucket ordering and z-order correctness.

mod common;
mod render_order_support;
mod render_order_world_map;

use common::env_with_shared_xml;
use render_order_support::*;
use wow_ui_sim::iced_app::compute_frame_rect;
use wow_ui_sim::render::headless::render_to_image;

// ============================================================================
// High frame_level border must not cover lower-level content
// ============================================================================

/// Reproduces the world map quest log bug: a decorative BorderFrame at
/// frame_level 100 covers quest POI icons at level 5-7 because the DFS
/// emits the border's texture AFTER the content children.
///
/// In WoW, a border frame's textures (edges/corners) render as part of
/// that frame's draw layer — they should not occlude child content at
/// lower frame_levels in the same parent.
#[test]
fn high_level_border_does_not_cover_lower_level_content() {
    let env = env_with_shared_xml();

    // Replicate QuestScrollFrame structure:
    // - ScrollFrame with a Background texture (BACKGROUND layer)
    // - A content child with an icon texture (ARTWORK layer)
    // - A BorderFrame child at frame_level 100 with a covering texture
    env.exec(
        r#"
        local panel = CreateFrame("Frame", "TestPanel", UIParent)
        panel:SetSize(300, 400)
        panel:SetPoint("CENTER")
        panel:Show()

        -- Background texture on the panel (like QuestLog-main-background)
        local bg = panel:CreateTexture("TestPanelBg", "BACKGROUND")
        bg:SetAllPoints()
        bg:SetColorTexture(0.1, 0.1, 0.1, 1)

        -- Content child at default frame_level (like quest entries)
        local content = CreateFrame("Frame", "TestContent", panel)
        content:SetAllPoints()
        content:Show()

        -- Icon texture on content (like POI button icon at ARTWORK layer)
        local icon = content:CreateTexture("TestIcon", "ARTWORK")
        icon:SetSize(20, 20)
        icon:SetPoint("CENTER")
        icon:SetColorTexture(1, 0, 0, 1)

        -- Decorative border at high frame_level (like ScrollFrameTemplate BorderFrame)
        local border = CreateFrame("Frame", "TestBorder", panel)
        border:SetAllPoints()
        border:SetFrameLevel(100)
        border:Show()

        -- Border texture covers the whole area (like the Border texture at level 100)
        local borderTex = border:CreateTexture("TestBorderTex", "ARTWORK")
        borderTex:SetAllPoints()
        borderTex:SetColorTexture(0, 0, 0, 0.8)
    "#,
    )
    .unwrap();

    let buckets = build_strata_buckets(&env);
    let state = env.state().borrow();

    let icon_id = state.widgets.get_id_by_name("TestIcon").unwrap();
    let border_tex_id = state.widgets.get_id_by_name("TestBorderTex").unwrap();

    // Find both IDs in the strata bucket and check their order.
    // The icon MUST render AFTER the border texture so it appears on top.
    let medium_bucket = &buckets[wow_ui_sim::widget::FrameStrata::Medium.as_index()];

    let icon_pos = medium_bucket.iter().position(|&id| id == icon_id);
    let border_pos = medium_bucket.iter().position(|&id| id == border_tex_id);

    assert!(
        icon_pos.is_some(),
        "TestIcon should be in the MEDIUM strata bucket"
    );
    assert!(
        border_pos.is_some(),
        "TestBorderTex should be in the MEDIUM strata bucket"
    );

    let icon_pos = icon_pos.unwrap();
    let border_pos = border_pos.unwrap();

    assert!(
        icon_pos > border_pos,
        "Content icon (pos={icon_pos}) must render AFTER border texture (pos={border_pos}). \
         A decorative border at high frame_level should not cover lower-level content."
    );
}

#[test]
fn late_created_texture_invalidates_cached_strata_buckets() {
    let env = env_with_shared_xml();

    env.exec(
        r#"
        local parent = CreateFrame("Frame", "LateBucketParent", UIParent)
        parent:SetSize(80, 80)
        parent:SetPoint("CENTER")
        parent:Show()

        local first = parent:CreateTexture("LateBucketFirstTexture", "ARTWORK")
        first:SetSize(20, 20)
        first:SetPoint("CENTER")
        first:SetColorTexture(1, 0, 0, 1)
    "#,
    )
    .unwrap();

    let _ = build_strata_buckets(&env);

    env.exec(
        r#"
        local second = LateBucketParent:CreateTexture("LateBucketSecondTexture", "OVERLAY")
        second:SetSize(18, 18)
        second:SetPoint("CENTER")
        second:SetColorTexture(0, 1, 0, 1)
    "#,
    )
    .unwrap();

    let buckets = build_strata_buckets(&env);
    let state = env.state().borrow();
    let second_id = state
        .widgets
        .get_id_by_name("LateBucketSecondTexture")
        .expect("late-created texture should exist in widget registry");
    let medium_bucket = &buckets[wow_ui_sim::widget::FrameStrata::Medium.as_index()];

    assert!(
        medium_bucket.contains(&second_id),
        "late-created texture must appear in rebuilt strata bucket after CreateTexture"
    );
}

#[test]
fn late_created_frame_invalidates_cached_strata_buckets() {
    let env = env_with_shared_xml();

    env.exec(
        r#"
        local parent = CreateFrame("Frame", "LateBucketFrameParent", UIParent)
        parent:SetSize(80, 80)
        parent:SetPoint("CENTER")
        parent:Show()
    "#,
    )
    .unwrap();

    let _ = build_strata_buckets(&env);

    env.exec(
        r#"
        local child = CreateFrame("Frame", "LateBucketChildFrame", LateBucketFrameParent)
        child:SetSize(16, 16)
        child:SetPoint("CENTER")
        child:Show()
    "#,
    )
    .unwrap();

    let buckets = build_strata_buckets(&env);
    let state = env.state().borrow();
    let child_id = state
        .widgets
        .get_id_by_name("LateBucketChildFrame")
        .expect("late-created frame should exist in widget registry");
    let medium_bucket = &buckets[wow_ui_sim::widget::FrameStrata::Medium.as_index()];

    assert!(
        medium_bucket.contains(&child_id),
        "late-created frame must appear in rebuilt strata bucket after CreateFrame"
    );
}

#[test]
fn late_set_draw_layer_invalidates_cached_strata_buckets() {
    let env = env_with_shared_xml();

    env.exec(
        r#"
        local parent = CreateFrame("Frame", "LateLayerParent", UIParent)
        parent:SetSize(80, 80)
        parent:SetPoint("CENTER")
        parent:Show()

        local circle = parent:CreateTexture("LateLayerCircle", "BACKGROUND")
        circle:SetAllPoints()
        circle:SetColorTexture(1, 0, 0, 1)

        local map = parent:CreateTexture("LateLayerMap", "ARTWORK")
        map:SetAllPoints()
        map:SetColorTexture(0, 1, 0, 1)

        local star = parent:CreateTexture("LateLayerStar", "OVERLAY")
        star:SetAllPoints()
        star:SetColorTexture(0, 0, 1, 1)
    "#,
    )
    .unwrap();

    let _ = build_strata_buckets(&env);

    env.exec(r#"LateLayerCircle:SetDrawLayer("ARTWORK", 1)"#)
        .unwrap();

    let buckets = build_strata_buckets(&env);
    let state = env.state().borrow();
    let medium_bucket = &buckets[wow_ui_sim::widget::FrameStrata::Medium.as_index()];
    let circle_id = state
        .widgets
        .get_id_by_name("LateLayerCircle")
        .expect("circle texture should exist");
    let map_id = state
        .widgets
        .get_id_by_name("LateLayerMap")
        .expect("map texture should exist");
    let star_id = state
        .widgets
        .get_id_by_name("LateLayerStar")
        .expect("star texture should exist");

    let circle_pos = medium_bucket
        .iter()
        .position(|&id| id == circle_id)
        .expect("circle should be in MEDIUM bucket");
    let map_pos = medium_bucket
        .iter()
        .position(|&id| id == map_id)
        .expect("map should be in MEDIUM bucket");
    let star_pos = medium_bucket
        .iter()
        .position(|&id| id == star_id)
        .expect("star should be in MEDIUM bucket");

    assert!(
        map_pos < circle_pos && circle_pos < star_pos,
        "late SetDrawLayer should rebuild region ordering: expected map -> circle -> star, \
         got positions map={map_pos} circle={circle_pos} star={star_pos}"
    );
}

#[test]
fn same_draw_layer_preserves_cached_strata_buckets() {
    let env = env_with_shared_xml();

    env.exec(
        r#"
        local parent = CreateFrame("Frame", "SameLayerParent", UIParent)
        parent:SetSize(80, 80)
        parent:SetPoint("CENTER")
        parent:Show()

        local tex = parent:CreateTexture("SameLayerTexture", "ARTWORK")
        tex:SetAllPoints()
        tex:SetColorTexture(1, 0, 0, 1)
        tex:SetDrawLayer("OVERLAY", 2)
    "#,
    )
    .unwrap();

    let _ = build_strata_buckets(&env);
    assert!(
        env.state().borrow().strata_buckets.is_some(),
        "building buckets should populate the cache"
    );

    env.exec(r#"SameLayerTexture:SetDrawLayer("OVERLAY", 2)"#)
        .unwrap();

    assert!(
        env.state().borrow().strata_buckets.is_some(),
        "no-op SetDrawLayer should not invalidate cached strata buckets"
    );
}

#[test]
fn late_set_frame_level_invalidates_cached_strata_buckets() {
    let env = env_with_shared_xml();

    env.exec(
        r#"
        local parent = CreateFrame("Frame", "LateLevelParent", UIParent)
        parent:SetSize(80, 80)
        parent:SetPoint("CENTER")
        parent:Show()

        local circleFrame = CreateFrame("Frame", "LateLevelCircleFrame", parent)
        circleFrame:SetAllPoints()
        circleFrame:SetFrameLevel(1)
        circleFrame:Show()

        local circle = circleFrame:CreateTexture("LateLevelCircle", "ARTWORK")
        circle:SetAllPoints()
        circle:SetColorTexture(1, 0, 0, 1)

        local mapFrame = CreateFrame("Frame", "LateLevelMapFrame", parent)
        mapFrame:SetAllPoints()
        mapFrame:SetFrameLevel(2)
        mapFrame:Show()

        local map = mapFrame:CreateTexture("LateLevelMap", "ARTWORK")
        map:SetAllPoints()
        map:SetColorTexture(0, 1, 0, 1)

        local starFrame = CreateFrame("Frame", "LateLevelStarFrame", parent)
        starFrame:SetAllPoints()
        starFrame:SetFrameLevel(4)
        starFrame:Show()

        local star = starFrame:CreateTexture("LateLevelStar", "ARTWORK")
        star:SetAllPoints()
        star:SetColorTexture(0, 0, 1, 1)
    "#,
    )
    .unwrap();

    let _ = build_strata_buckets(&env);

    env.exec(r#"LateLevelCircleFrame:SetFrameLevel(3)"#)
        .unwrap();

    let buckets = build_strata_buckets(&env);
    let state = env.state().borrow();
    let medium_bucket = &buckets[wow_ui_sim::widget::FrameStrata::Medium.as_index()];
    let circle_frame_id = state
        .widgets
        .get_id_by_name("LateLevelCircleFrame")
        .expect("circle frame should exist");
    let map_frame_id = state
        .widgets
        .get_id_by_name("LateLevelMapFrame")
        .expect("map frame should exist");
    let star_frame_id = state
        .widgets
        .get_id_by_name("LateLevelStarFrame")
        .expect("star frame should exist");

    let circle_pos = medium_bucket
        .iter()
        .position(|&id| id == circle_frame_id)
        .expect("circle frame should be in MEDIUM bucket");
    let map_pos = medium_bucket
        .iter()
        .position(|&id| id == map_frame_id)
        .expect("map frame should be in MEDIUM bucket");
    let star_pos = medium_bucket
        .iter()
        .position(|&id| id == star_frame_id)
        .expect("star frame should be in MEDIUM bucket");

    assert!(
        map_pos < circle_pos && circle_pos < star_pos,
        "late SetFrameLevel should rebuild frame ordering: expected map -> circle -> star, \
         got positions map={map_pos} circle={circle_pos} star={star_pos}"
    );
}

#[test]
fn isolated_world_map_stack_opens_and_populates_world_quest_pins() {
    common::with_timeout(120, move || {
        let env = env_with_isolated_world_map();

        std::thread::sleep(std::time::Duration::from_millis(1200));
        wow_ui_sim::startup::process_pending_timers(&env);
        wow_ui_sim::startup::fire_one_on_update_tick(&env);

        let world_map_shown: bool = env
            .eval("return WorldMapFrame ~= nil and WorldMapFrame:IsShown() == true")
            .expect("should query WorldMapFrame visibility");
        assert!(
            world_map_shown,
            "isolated world map stack should show WorldMapFrame"
        );

        let state = env.state().borrow();
        let loaded_addons: Vec<_> = state
            .addons
            .iter()
            .filter(|addon| addon.loaded)
            .map(|addon| addon.folder_name.clone())
            .collect();
        let world_quest_pairs = world_quest_pin_pairs(&state);
        eprintln!(
            "isolated world map loaded {} addons: {:?}",
            loaded_addons.len(),
            loaded_addons
        );
        eprintln!(
            "isolated world map visible world quest pin pairs={}",
            world_quest_pairs.len()
        );

        assert!(
            !world_quest_pairs.is_empty(),
            "isolated world map stack should still populate visible world quest pins"
        );
    });
}

#[test]
fn isolated_world_map_fog_of_war_renders_only_on_unexplored_half() {
    common::with_timeout(120, move || {
        let env = env_with_isolated_world_map();

        std::thread::sleep(std::time::Duration::from_millis(1200));
        wow_ui_sim::startup::process_pending_timers(&env);
        wow_ui_sim::startup::fire_one_on_update_tick(&env);

        let fog_rect = {
            let state = env.state().borrow();
            let world_map_id = state
                .widgets
                .get_id_by_name("WorldMapFrame")
                .expect("isolated world map should create WorldMapFrame");
            let fog_pin_id = state
                .widgets
                .iter_ids()
                .find(|&id| {
                    state.widgets.get(id).is_some_and(|frame| {
                        frame
                            .object_type_name
                            .as_deref()
                            .is_some_and(|name| name.eq_ignore_ascii_case("FogOfWarFrame"))
                            && is_descendant_of(&state.widgets, id, world_map_id)
                    })
                })
                .expect("isolated world map should create a FogOfWarFrame pin");
            compute_frame_rect(&state.widgets, fog_pin_id, 1024.0, 768.0)
        };

        let mut visible_mgr = make_texture_manager().expect("texture directories should exist");
        let visible_batch = build_screenshot_like_batch(&env, 1024, 768, Some("WorldMapFrame"));
        let visible_render = render_to_image(&visible_batch, &mut visible_mgr, 1024, 768, None);

        env.exec(
            r#"
            local fogPin = WorldMapFrame:EnumeratePinsByTemplate("FogOfWarPinTemplate")()
            assert(fogPin, "missing fog pin")
            fogPin:Hide()
        "#,
        )
        .expect("failed to hide fog pin");
        wow_ui_sim::startup::process_pending_timers(&env);
        wow_ui_sim::startup::fire_one_on_update_tick(&env);

        let mut hidden_mgr = make_texture_manager().expect("texture directories should exist");
        let hidden_batch = build_screenshot_like_batch(&env, 1024, 768, Some("WorldMapFrame"));
        let hidden_render = render_to_image(&hidden_batch, &mut hidden_mgr, 1024, 768, None);
        let fog_diff =
            diff_bounds(&visible_render, &hidden_render, 12).expect("fog should change pixels");

        assert!(
            fog_diff.0 as f32 >= fog_rect.x + fog_rect.width * 0.49,
            "fog should start on the unexplored half: diff={fog_diff:?} fog_rect={fog_rect:?}"
        );
        assert!(
            fog_diff.2 as f32 >= fog_rect.x + fog_rect.width * 0.92,
            "fog should reach the right side of the fog frame: diff={fog_diff:?} fog_rect={fog_rect:?}"
        );
        assert!(
            fog_diff.2 as f32 <= fog_rect.x + fog_rect.width + 2.0,
            "fog should not extend beyond the fog frame: diff={fog_diff:?} fog_rect={fog_rect:?}"
        );
        assert!(
            fog_diff.1 as f32 <= fog_rect.y + 2.0
                && fog_diff.3 as f32 >= fog_rect.y + fog_rect.height - 2.0,
            "fog should cover the full fog-frame height: diff={fog_diff:?} fog_rect={fog_rect:?}"
        );
    });
}

#[test]
fn isolated_world_map_exploration_overlay_renders_on_explored_half() {
    common::with_timeout(120, move || {
        let env = env_with_isolated_world_map();

        std::thread::sleep(std::time::Duration::from_millis(1200));
        wow_ui_sim::startup::process_pending_timers(&env);
        wow_ui_sim::startup::fire_one_on_update_tick(&env);

        let map_rect = {
            let state = env.state().borrow();
            let world_map_id = state
                .widgets
                .get_id_by_name("WorldMapFrame")
                .expect("isolated world map should create WorldMapFrame");
            let fog_pin_id = state
                .widgets
                .iter_ids()
                .find(|&id| {
                    state.widgets.get(id).is_some_and(|frame| {
                        frame
                            .object_type_name
                            .as_deref()
                            .is_some_and(|name| name.eq_ignore_ascii_case("FogOfWarFrame"))
                            && is_descendant_of(&state.widgets, id, world_map_id)
                    })
                })
                .expect("isolated world map should create a FogOfWarFrame pin");
            compute_frame_rect(&state.widgets, fog_pin_id, 1024.0, 768.0)
        };
        let expected_overlay_bounds: (f32, f32, f32, f32) = env
            .eval(
                r#"
                local mapID = C_Map.GetCurrentMapID()
                local layer = C_Map.GetMapArtLayers(mapID)[1]
                local explored = C_MapExplorationInfo.GetExploredMapTextures(mapID)
                assert(type(explored) == "table" and #explored > 0, "missing explored overlays")

                local minLeft = math.huge
                local minTop = math.huge
                local maxRight = 0
                local maxBottom = 0

                for _, overlay in ipairs(explored) do
                    minLeft = math.min(minLeft, overlay.offsetX)
                    minTop = math.min(minTop, overlay.offsetY)
                    maxRight = math.max(maxRight, overlay.offsetX + overlay.textureWidth)
                    maxBottom = math.max(maxBottom, overlay.offsetY + overlay.textureHeight)
                end

                return minLeft / layer.layerWidth,
                    minTop / layer.layerHeight,
                    maxRight / layer.layerWidth,
                    maxBottom / layer.layerHeight
            "#,
            )
            .expect("failed to compute expected exploration bounds");

        let mut visible_mgr = make_texture_manager().expect("texture directories should exist");
        let visible_batch = build_screenshot_like_batch(&env, 1024, 768, Some("WorldMapFrame"));
        let visible_render = render_to_image(&visible_batch, &mut visible_mgr, 1024, 768, None);

        env.exec(
            r#"
            local pin = WorldMapFrame:EnumeratePinsByTemplate("MapExplorationPinTemplate")()
            assert(pin, "missing exploration pin")
            pin:Hide()
        "#,
        )
        .expect("failed to hide exploration pin");
        wow_ui_sim::startup::process_pending_timers(&env);
        wow_ui_sim::startup::fire_one_on_update_tick(&env);

        let mut hidden_mgr = make_texture_manager().expect("texture directories should exist");
        let hidden_batch = build_screenshot_like_batch(&env, 1024, 768, Some("WorldMapFrame"));
        let hidden_render = render_to_image(&hidden_batch, &mut hidden_mgr, 1024, 768, None);
        let overlay_diff = diff_bounds(&visible_render, &hidden_render, 12)
            .expect("exploration overlay should change pixels");
        let expected_left = map_rect.x + map_rect.width * expected_overlay_bounds.0;
        let expected_top = map_rect.y + map_rect.height * expected_overlay_bounds.1;
        let expected_right = map_rect.x + map_rect.width * expected_overlay_bounds.2;
        let expected_bottom = map_rect.y + map_rect.height * expected_overlay_bounds.3;
        let tolerance = 32.0;

        assert!(
            (overlay_diff.0 as f32 - expected_left).abs() <= tolerance,
            "exploration overlay should start where the API overlay data starts: diff={overlay_diff:?} expected_left={expected_left} rect={map_rect:?}"
        );
        assert!(
            (overlay_diff.1 as f32 - expected_top).abs() <= tolerance,
            "exploration overlay should start at the expected top bound: diff={overlay_diff:?} expected_top={expected_top} rect={map_rect:?}"
        );
        assert!(
            (overlay_diff.2 as f32 - expected_right).abs() <= tolerance,
            "exploration overlay should end where the API overlay data ends: diff={overlay_diff:?} expected_right={expected_right} rect={map_rect:?}"
        );
        assert!(
            (overlay_diff.3 as f32 - expected_bottom).abs() <= tolerance,
            "exploration overlay should end at the expected bottom bound: diff={overlay_diff:?} expected_bottom={expected_bottom} rect={map_rect:?}"
        );
    });
}

#[test]
fn isolated_world_map_seeded_world_quests_do_not_show_expiration_clock() {
    common::with_timeout(120, move || {
        let env = env_with_isolated_world_map();

        std::thread::sleep(std::time::Duration::from_millis(1200));
        wow_ui_sim::startup::process_pending_timers(&env);
        wow_ui_sim::startup::fire_one_on_update_tick(&env);

        let lua_probe: String = env
            .eval(
                r#"
                local pin = WorldMapFrame and WorldMapFrame:EnumeratePinsByTemplate("WorldMap_WorldQuestPinTemplate")()
                return string.format(
                    "seconds=%s low=%s critical=%s timelow=%s",
                    tostring(C_TaskQuest.GetQuestTimeLeftSeconds(90101)),
                    tostring(QuestUtils_IsQuestWithinLowTimeThreshold(90101)),
                    tostring(QuestUtils_IsQuestWithinCriticalTimeThreshold(90101)),
                    tostring(pin and pin.TimeLowFrame and pin.TimeLowFrame:IsShown())
                )
                "#,
            )
            .expect("lua probe should run");
        eprintln!("{lua_probe}");

        let state = env.state().borrow();
        let visible_clock_icons: Vec<_> = state
            .widgets
            .iter_ids()
            .filter(|&id| {
                let Some(frame) = state.widgets.get(id) else {
                    return false;
                };
                frame.effective_alpha > 0.0
                    && frame.atlas.as_deref() == Some("worldquest-icon-clock")
            })
            .collect();

        assert!(
            visible_clock_icons.is_empty(),
            "seeded world quests default to 120 minutes left, so expiration clocks should stay hidden; visible clock ids={visible_clock_icons:?}"
        );
    });
}

#[test]
fn isolated_world_map_world_quest_circle_keeps_atlas_size() {
    common::with_timeout(120, move || {
        let env = env_with_isolated_world_map();

        std::thread::sleep(std::time::Duration::from_millis(1200));
        wow_ui_sim::startup::process_pending_timers(&env);
        wow_ui_sim::startup::fire_one_on_update_tick(&env);

        let state = env.state().borrow();
        let (_circle_id, _icon_id, circle_rect, _circle_request_path, _icon_request_path) =
            world_quest_pin_pairs(&state)
                .into_iter()
                .next()
                .expect("isolated world map should have at least one world quest pair");

        assert!(
            (circle_rect.width - 32.0).abs() <= 0.1 && (circle_rect.height - 32.0).abs() <= 0.1,
            "world quest NormalTexture should keep its 32x32 atlas-sized rect, got {circle_rect:?}"
        );
    });
}
