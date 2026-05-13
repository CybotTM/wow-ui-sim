#![cfg(feature = "gui")]

use crate::common;
#[path = "render_order_support.rs"]
mod render_order_support;

use image::RgbaImage;
use wow_ui_sim::lua_api::WowLuaEnv;
use wow_ui_sim::render::headless::render_to_image;

fn sample_rect_pixel(image: &RgbaImage, rect: (f32, f32, f32, f32), u: f32, v: f32) -> [u8; 4] {
    let max_x = image.width().saturating_sub(1) as f32;
    let max_y = image.height().saturating_sub(1) as f32;
    let x = (rect.0 + rect.2 * u).round().clamp(0.0, max_x) as u32;
    let y = (rect.1 + rect.3 * v).round().clamp(0.0, max_y) as u32;
    image.get_pixel(x, y).0
}

fn max_rgb_channel_diff(lhs: [u8; 4], rhs: [u8; 4]) -> u8 {
    (0..3)
        .map(|channel| lhs[channel].abs_diff(rhs[channel]))
        .max()
        .unwrap_or(0)
}

fn named_rect(env: &WowLuaEnv, name: &str, width: f32, height: f32) -> (f32, f32, f32, f32) {
    let state = env.state().borrow();
    let id = state
        .widgets
        .get_id_by_name(name)
        .unwrap_or_else(|| panic!("{name} should exist"));
    let rect = wow_ui_sim::iced_app::compute_frame_rect(&state.widgets, id, width, height);
    (rect.x, rect.y, rect.width, rect.height)
}

#[test]
fn masked_circular_class_texture_keeps_background_visible_in_corners() {
    if common::try_create_gpu_device().is_none() {
        eprintln!("Skipping GPU class icon atlas render test: no adapter available");
        return;
    }

    let env = WowLuaEnv::new().expect("Failed to create Lua environment");
    let width = 128u32;
    let height = 96u32;
    env.set_screen_size(width as f32, height as f32);
    env.exec(
        r#"
        local frame = CreateFrame("Frame", "ClassIconAtlasHarness", UIParent)
        frame:SetSize(96, 64)
        frame:SetPoint("TOPLEFT", UIParent, "TOPLEFT", 16, -16)

        local background = frame:CreateTexture("ClassIconAtlasHarnessBackground", "BACKGROUND")
        background:SetAllPoints()
        background:SetColorTexture(1, 0, 1, 1)

        local icon = frame:CreateTexture("ClassIconAtlasHarnessIcon", "ARTWORK")
        icon:SetSize(58, 58)
        icon:SetPoint("TOPLEFT", frame, "TOPLEFT", 3, -3)
        icon:SetTexture("Interface\\TargetingFrame\\UI-Classes-Circles")
        icon:SetTexCoord(0.25, 0.49609375, 0, 0.25)

        local mask = frame:CreateMaskTexture("ClassIconAtlasHarnessMask", "ARTWORK")
        mask:SetAllPoints(icon)
        mask:SetTexture("Interface\\Masks\\CircleMask")
        icon:AddMaskTexture(mask)
    "#,
    )
    .expect("failed to build class icon atlas harness");

    let icon_rect = named_rect(
        &env,
        "ClassIconAtlasHarnessIcon",
        width as f32,
        height as f32,
    );

    let mut tex_mgr = render_order_support::make_texture_manager();
    let batch = render_order_support::build_screenshot_like_batch(
        &env,
        width,
        height,
        Some("ClassIconAtlasHarness"),
    );
    let rendered = render_to_image(&batch, &mut tex_mgr, width, height, None);

    let expected_background = [255, 0, 255, 255];
    let center = sample_rect_pixel(&rendered, icon_rect, 0.5, 0.5);
    assert!(
        max_rgb_channel_diff(center, expected_background) >= 80,
        "icon center should not collapse to the background: center={center:?}"
    );

    for (u, v, label) in [
        (0.08, 0.08, "top-left"),
        (0.92, 0.08, "top-right"),
        (0.08, 0.92, "bottom-left"),
        (0.92, 0.92, "bottom-right"),
    ] {
        let corner = sample_rect_pixel(&rendered, icon_rect, u, v);
        assert!(
            max_rgb_channel_diff(corner, expected_background) <= 18,
            "masked circular class texture {label} corner should show the background: corner={corner:?} expected={expected_background:?}"
        );
    }
}
