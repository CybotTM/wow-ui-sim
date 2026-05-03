use wow_ui_sim::lua_api::state::SimState;
use wow_ui_sim::widget::{DrawLayer, Frame, FrameStrata, WidgetType};

fn visible_root_texture(id: u64, name: &str) -> Frame {
    Frame {
        id,
        name: Some(name.to_string()),
        widget_type: WidgetType::Texture,
        frame_strata: FrameStrata::Medium,
        draw_layer: DrawLayer::Artwork,
        visible: true,
        alpha: 1.0,
        effective_alpha: 1.0,
        width: 10.0,
        height: 10.0,
        layout_rect: Some(wow_ui_sim::LayoutRect {
            x: 0.0,
            y: 0.0,
            width: 10.0,
            height: 10.0,
        }),
        ..Default::default()
    }
}

#[test]
fn later_created_root_regions_render_after_earlier_root_regions() {
    let mut state = SimState::default();
    state.widgets.register(visible_root_texture(101, "Earlier"));
    state.widgets.register(visible_root_texture(102, "Later"));

    let bucket = state
        .get_strata_buckets()
        .expect("strata buckets")
        .get(FrameStrata::Medium.as_index())
        .expect("medium bucket");

    assert_eq!(
        bucket,
        &[101, 102],
        "later-created root regions should render later/on top in the same draw layer"
    );
}
