//! Simulator-policy CPU render-list coverage, not native or GPU parity.
use super::build_render_list;
use crate::LayoutRect;
use crate::lua_api::WowLuaEnv;

const EPSILON: f32 = 0.01;

#[test]
fn slider_scale_inherited_thumb_endpoints_without_query_priming() {
    for ancestor_scale in [0.5, 2.0] {
        for orientation in ["HORIZONTAL", "VERTICAL"] {
            assert_scaled_endpoints(ancestor_scale, 1.0, orientation);
        }
    }
}

#[test]
fn slider_scale_thumb_local_endpoints_without_query_priming() {
    for thumb_scale in [0.5, 1.5] {
        for orientation in ["HORIZONTAL", "VERTICAL"] {
            assert_scaled_endpoints(2.0, thumb_scale, orientation);
        }
    }
}

fn create_slider_fixture(orientation: &str) -> WowLuaEnv {
    let env = WowLuaEnv::new().unwrap();
    env.exec(
        r#"
        UIParent:SetScale(1)
        scaleAncestor = CreateFrame("Frame", "ScaleAncestor", UIParent)
        scaleAncestor:SetSize(400, 400)
        scaleAncestor:SetPoint("TOPLEFT", UIParent, "TOPLEFT", 40, -60)
        scaleSlider = CreateFrame("Slider", "ScaleSlider", scaleAncestor)
        scaleSlider:SetPoint("TOPLEFT", scaleAncestor, "TOPLEFT", 0, 0)
        scaleSlider:SetSize(160, 40)
        scaleSlider:SetMinMaxValues(10, 110)
        scaleSlider:SetValue(60)
        scaleThumb = scaleSlider:GetThumbTexture()
        scaleThumb:SetSize(18, 24)
        scaleThumb:SetColorTexture(1, 0, 1, 1)
        scaleSlider:SetThumbTexture(scaleThumb)
        "#,
    )
    .unwrap();
    if orientation == "VERTICAL" {
        env.exec("scaleSlider:SetOrientation('VERTICAL'); scaleSlider:SetSize(40, 160)")
            .unwrap();
    }
    env
}

fn capture_render_rects(
    env: &WowLuaEnv,
    ancestor_scale: f32,
    thumb_scale: f32,
) -> (LayoutRect, LayoutRect, f32) {
    let state_handle = env.state();
    let mut state = state_handle.borrow_mut();
    // Normal pre-render resolution only; no Lua geometry query primes layout.
    state.ensure_layout_rects();
    let ancestor_id = state.widgets.get_id_by_name("ScaleAncestor").unwrap();
    let slider_id = state.widgets.get_id_by_name("ScaleSlider").unwrap();
    let ancestor = state.widgets.get(ancestor_id).unwrap();
    let slider = state.widgets.get(slider_id).unwrap();
    let thumb_id = slider.children_keys["ThumbTexture"];
    let thumb = state.widgets.get(thumb_id).unwrap();
    assert_close(ancestor.effective_scale, ancestor_scale);
    assert_close(slider.scale, 1.0);
    assert_close(slider.effective_scale, ancestor_scale);
    assert_close(thumb.scale, thumb_scale);
    assert_close(thumb.effective_scale, ancestor_scale * thumb_scale);
    let list = build_render_list(
        &[slider_id, thumb_id],
        &state.widgets,
        (state.screen_width, state.screen_height),
    );
    assert_eq!(
        list.len(),
        2,
        "track and bound thumb both reach render list"
    );
    assert_eq!((list[0].0, list[1].0), (slider_id, thumb_id));
    (list[0].1, list[1].1, state.screen_height)
}

fn assert_scaled_endpoints(ancestor_scale: f32, thumb_scale: f32, orientation: &str) {
    let env = create_slider_fixture(orientation);
    // Warm stored layout before changing scales to exercise dirty propagation.
    capture_render_rects(&env, 1.0, 1.0);
    env.exec(&format!(
        "scaleAncestor:SetScale({ancestor_scale}); scaleThumb:SetScale({thumb_scale})"
    ))
    .unwrap();
    let expected_track = expected_slider_rect(ancestor_scale, orientation);
    let thumb_size = (
        18.0 * ancestor_scale * thumb_scale,
        24.0 * ancestor_scale * thumb_scale,
    );
    for (value, fraction) in [(10, 0.0), (60, 0.5), (110, 1.0)] {
        env.exec(&format!("scaleSlider:SetValue({value})")).unwrap();
        let (track, thumb, _) = capture_render_rects(&env, ancestor_scale, thumb_scale);
        assert_rect_close(track, expected_track);
        assert_thumb_position(track, thumb, thumb_size, orientation, fraction);
    }
    // All endpoint assertions finish before the first geometry query.
    assert_final_query_agreement(&env, ancestor_scale, thumb_scale);
}

fn expected_slider_rect(scale: f32, orientation: &str) -> LayoutRect {
    let (width, height) = match orientation {
        "HORIZONTAL" => (160.0, 40.0),
        "VERTICAL" => (40.0, 160.0),
        _ => panic!("unexpected fixture orientation"),
    };
    LayoutRect {
        x: 40.0 * scale,
        y: 60.0 * scale,
        width: width * scale,
        height: height * scale,
    }
}

fn assert_final_query_agreement(env: &WowLuaEnv, ancestor_scale: f32, thumb_scale: f32) {
    let (track, thumb, screen_height) = capture_render_rects(env, ancestor_scale, thumb_scale);
    assert_query_conversion(env, "scaleSlider", track, ancestor_scale, screen_height);
    assert_query_conversion(
        env,
        "scaleThumb",
        thumb,
        ancestor_scale * thumb_scale,
        screen_height,
    );
    let (after_track, after_thumb, _) = capture_render_rects(env, ancestor_scale, thumb_scale);
    assert_rect_close(after_track, track);
    assert_rect_close(after_thumb, thumb);
}

fn assert_thumb_position(
    track: LayoutRect,
    thumb: LayoutRect,
    size: (f32, f32),
    orientation: &str,
    fraction: f32,
) {
    let (width, height) = size;
    let start_x = track.x;
    let end_x = track.x + track.width - width;
    let start_y = track.y;
    let end_y = track.y + track.height - height;
    let (x_fraction, y_fraction) = match orientation {
        "HORIZONTAL" => (fraction, 0.5),
        "VERTICAL" => (0.5, fraction),
        _ => panic!("unexpected fixture orientation"),
    };
    // Interpolate endpoint positions in common top-left render space, not
    // cross-frame GetRect coordinates. No assumed native endpoint inset.
    let expected = LayoutRect {
        x: start_x * (1.0 - x_fraction) + end_x * x_fraction,
        y: start_y * (1.0 - y_fraction) + end_y * y_fraction,
        width,
        height,
    };
    assert_rect_close(thumb, expected);
    assert!(thumb.x >= track.x - EPSILON);
    assert!(thumb.y >= track.y - EPSILON);
    assert!(thumb.x + thumb.width <= track.x + track.width + EPSILON);
    assert!(thumb.y + thumb.height <= track.y + track.height + EPSILON);
}

fn assert_query_conversion(
    env: &WowLuaEnv,
    frame: &str,
    rect: LayoutRect,
    effective_scale: f32,
    screen_height: f32,
) {
    let bottom = screen_height - rect.y - rect.height;
    env.exec(&format!(
        r#"
        local frame = {frame}
        local expected = {{{left}, {bottom}, {width}, {height}}}
        local scaled = {{frame:GetScaledRect()}}
        local unscaled = {{frame:GetRect()}}
        assert(#scaled == 4 and #unscaled == 4)
        for i = 1, 4 do
            assert(math.abs(scaled[i] - expected[i]) < 0.01)
            assert(math.abs(unscaled[i] - expected[i] / {effective_scale}) < 0.01)
        end
        "#,
        left = rect.x,
        width = rect.width,
        height = rect.height,
    ))
    .unwrap();
}

fn assert_rect_close(actual: LayoutRect, expected: LayoutRect) {
    assert_close(actual.x, expected.x);
    assert_close(actual.y, expected.y);
    assert_close(actual.width, expected.width);
    assert_close(actual.height, expected.height);
}

fn assert_close(actual: f32, expected: f32) {
    assert!(
        (actual - expected).abs() < EPSILON,
        "actual {actual}, expected {expected}"
    );
}
