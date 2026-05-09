//! Procedural app icon renderer.
//!
//! Produces an RGBA pixel buffer used by both the iced runtime window icon
//! and the build-time generator that writes `installer/wow-sim.ico`.

pub const SIZE: u32 = 128;

pub fn render_icon() -> Vec<u8> {
    let mut pixels = vec![0; (SIZE * SIZE * 4) as usize];

    draw_background(&mut pixels);
    draw_outer_frame(&mut pixels);
    draw_grid_panel(&mut pixels);
    draw_sigil(&mut pixels);
    draw_anchor_nodes(&mut pixels);

    pixels
}

fn draw_background(pixels: &mut [u8]) {
    for y in 0..SIZE {
        for x in 0..SIZE {
            let px = x as f32 + 0.5;
            let py = y as f32 + 0.5;
            let base_alpha = rounded_rect_alpha(px, py, 7.0, 7.0, 114.0, 114.0, 25.0);
            if base_alpha <= 0.0 {
                continue;
            }

            let glow = radial(px, py, 40.0, 30.0, 18.0, 104.0);
            let edge = radial(px, py, 64.0, 64.0, 10.0, 82.0);
            let r = lerp(18.0, 30.0, glow) + edge * 8.0;
            let g = lerp(21.0, 39.0, glow) + edge * 5.0;
            let b = lerp(31.0, 52.0, glow) + edge * 12.0;
            blend(
                pixels,
                x,
                y,
                [r as u8, g as u8, b as u8, (255.0 * base_alpha) as u8],
            );
        }
    }
}

fn draw_outer_frame(pixels: &mut [u8]) {
    draw_rounded_rect(pixels, 12.0, 12.0, 104.0, 104.0, 21.0, [7, 8, 13, 120]);
    draw_rounded_rect_stroke(
        pixels,
        Rect::new(10.0, 10.0, 108.0, 108.0),
        23.0,
        3.0,
        [224, 172, 68, 245],
    );
    draw_rounded_rect_stroke(
        pixels,
        Rect::new(15.0, 15.0, 98.0, 98.0),
        18.0,
        1.4,
        [95, 69, 32, 210],
    );
}

fn draw_grid_panel(pixels: &mut [u8]) {
    draw_rounded_rect(pixels, 25.0, 29.0, 78.0, 60.0, 8.0, [14, 33, 54, 245]);
    draw_rounded_rect_stroke(
        pixels,
        Rect::new(24.0, 28.0, 80.0, 62.0),
        9.0,
        2.0,
        [82, 181, 217, 170],
    );
    draw_grid_lines(pixels);
}

fn draw_grid_lines(pixels: &mut [u8]) {
    draw_line(pixels, (33.0, 44.0), (94.0, 44.0), 1.2, [80, 185, 214, 90]);
    draw_line(pixels, (33.0, 60.0), (94.0, 60.0), 1.2, [80, 185, 214, 75]);
    draw_line(pixels, (46.0, 34.0), (46.0, 83.0), 1.2, [80, 185, 214, 65]);
    draw_line(pixels, (64.0, 34.0), (64.0, 83.0), 1.2, [80, 185, 214, 75]);
    draw_line(pixels, (82.0, 34.0), (82.0, 83.0), 1.2, [80, 185, 214, 65]);
}

fn draw_sigil(pixels: &mut [u8]) {
    let shadow = [
        (34.0, 49.0),
        (44.0, 82.0),
        (55.0, 61.0),
        (64.0, 80.0),
        (73.0, 61.0),
        (84.0, 82.0),
        (94.0, 49.0),
    ];
    draw_polyline(pixels, &shadow, 8.8, [0, 0, 0, 120]);
    let mark = [
        (34.0, 47.0),
        (44.0, 80.0),
        (55.0, 59.0),
        (64.0, 78.0),
        (73.0, 59.0),
        (84.0, 80.0),
        (94.0, 47.0),
    ];
    draw_polyline(pixels, &mark, 6.3, [247, 197, 82, 255]);
    draw_polyline(pixels, &mark, 2.0, [255, 236, 156, 235]);
}

fn draw_anchor_nodes(pixels: &mut [u8]) {
    draw_line(pixels, (32.0, 98.0), (96.0, 98.0), 2.0, [72, 199, 168, 185]);
    draw_line(pixels, (32.0, 98.0), (32.0, 76.0), 2.0, [72, 199, 168, 145]);
    draw_line(pixels, (96.0, 98.0), (96.0, 76.0), 2.0, [72, 199, 168, 145]);
    for &(cx, cy) in &[(32.0, 98.0), (96.0, 98.0), (32.0, 76.0), (96.0, 76.0)] {
        draw_circle(pixels, cx, cy, 4.2, [112, 235, 192, 240]);
        draw_circle(pixels, cx, cy, 2.0, [18, 39, 45, 230]);
    }
}

#[derive(Clone, Copy)]
struct Rect {
    x: f32,
    y: f32,
    w: f32,
    h: f32,
}

impl Rect {
    const fn new(x: f32, y: f32, w: f32, h: f32) -> Self {
        Self { x, y, w, h }
    }
}

fn draw_rounded_rect(
    pixels: &mut [u8],
    x: f32,
    y: f32,
    w: f32,
    h: f32,
    radius: f32,
    color: [u8; 4],
) {
    for py in 0..SIZE {
        for px in 0..SIZE {
            let alpha = rounded_rect_alpha(px as f32 + 0.5, py as f32 + 0.5, x, y, w, h, radius);
            if alpha > 0.0 {
                let mut c = color;
                c[3] = ((c[3] as f32) * alpha) as u8;
                blend(pixels, px, py, c);
            }
        }
    }
}

fn draw_rounded_rect_stroke(
    pixels: &mut [u8],
    rect: Rect,
    radius: f32,
    width: f32,
    color: [u8; 4],
) {
    for py in 0..SIZE {
        for px in 0..SIZE {
            let x = px as f32 + 0.5;
            let y = py as f32 + 0.5;
            let outer = rounded_rect_alpha(x, y, rect.x, rect.y, rect.w, rect.h, radius);
            let inner = rounded_rect_alpha(
                x,
                y,
                rect.x + width,
                rect.y + width,
                rect.w - width * 2.0,
                rect.h - width * 2.0,
                (radius - width).max(0.0),
            );
            let alpha = (outer - inner).clamp(0.0, 1.0);
            if alpha > 0.0 {
                let mut c = color;
                c[3] = ((c[3] as f32) * alpha) as u8;
                blend(pixels, px, py, c);
            }
        }
    }
}

fn draw_polyline(pixels: &mut [u8], points: &[(f32, f32)], width: f32, color: [u8; 4]) {
    for pair in points.windows(2) {
        draw_line(pixels, pair[0], pair[1], width, color);
    }
}

fn draw_line(pixels: &mut [u8], a: (f32, f32), b: (f32, f32), width: f32, color: [u8; 4]) {
    let min_x = (a.0.min(b.0) - width - 2.0).floor().max(0.0) as u32;
    let max_x = (a.0.max(b.0) + width + 2.0).ceil().min((SIZE - 1) as f32) as u32;
    let min_y = (a.1.min(b.1) - width - 2.0).floor().max(0.0) as u32;
    let max_y = (a.1.max(b.1) + width + 2.0).ceil().min((SIZE - 1) as f32) as u32;

    for y in min_y..=max_y {
        for x in min_x..=max_x {
            let dist = distance_to_segment((x as f32 + 0.5, y as f32 + 0.5), a, b);
            let alpha = (width / 2.0 + 0.8 - dist).clamp(0.0, 1.0);
            if alpha > 0.0 {
                let mut c = color;
                c[3] = ((c[3] as f32) * alpha) as u8;
                blend(pixels, x, y, c);
            }
        }
    }
}

fn draw_circle(pixels: &mut [u8], cx: f32, cy: f32, radius: f32, color: [u8; 4]) {
    let min_x = (cx - radius - 1.0).floor().max(0.0) as u32;
    let max_x = (cx + radius + 1.0).ceil().min((SIZE - 1) as f32) as u32;
    let min_y = (cy - radius - 1.0).floor().max(0.0) as u32;
    let max_y = (cy + radius + 1.0).ceil().min((SIZE - 1) as f32) as u32;
    for y in min_y..=max_y {
        for x in min_x..=max_x {
            let dx = x as f32 + 0.5 - cx;
            let dy = y as f32 + 0.5 - cy;
            let alpha = (radius + 0.8 - (dx * dx + dy * dy).sqrt()).clamp(0.0, 1.0);
            if alpha > 0.0 {
                let mut c = color;
                c[3] = ((c[3] as f32) * alpha) as u8;
                blend(pixels, x, y, c);
            }
        }
    }
}

fn rounded_rect_alpha(px: f32, py: f32, x: f32, y: f32, w: f32, h: f32, radius: f32) -> f32 {
    let cx = (px - (x + w / 2.0)).abs() - w / 2.0 + radius;
    let cy = (py - (y + h / 2.0)).abs() - h / 2.0 + radius;
    let outside_x = cx.max(0.0);
    let outside_y = cy.max(0.0);
    let outside = (outside_x * outside_x + outside_y * outside_y).sqrt();
    let inside = cx.max(cy).min(0.0);
    let dist = outside + inside - radius;
    (0.8 - dist).clamp(0.0, 1.0)
}

fn distance_to_segment(p: (f32, f32), a: (f32, f32), b: (f32, f32)) -> f32 {
    let vx = b.0 - a.0;
    let vy = b.1 - a.1;
    let wx = p.0 - a.0;
    let wy = p.1 - a.1;
    let len_sq = vx * vx + vy * vy;
    let t = if len_sq > 0.0 {
        ((wx * vx + wy * vy) / len_sq).clamp(0.0, 1.0)
    } else {
        0.0
    };
    let dx = p.0 - (a.0 + t * vx);
    let dy = p.1 - (a.1 + t * vy);
    (dx * dx + dy * dy).sqrt()
}

fn radial(px: f32, py: f32, cx: f32, cy: f32, inner: f32, outer: f32) -> f32 {
    let dx = px - cx;
    let dy = py - cy;
    let dist = (dx * dx + dy * dy).sqrt();
    (1.0 - ((dist - inner) / (outer - inner)).clamp(0.0, 1.0)).powf(1.4)
}

fn blend(pixels: &mut [u8], x: u32, y: u32, src: [u8; 4]) {
    let idx = ((y * SIZE + x) * 4) as usize;
    let src_a = src[3] as f32 / 255.0;
    let dst_a = pixels[idx + 3] as f32 / 255.0;
    let out_a = src_a + dst_a * (1.0 - src_a);
    if out_a <= 0.0 {
        return;
    }
    for channel in 0..3 {
        let src_c = src[channel] as f32 / 255.0;
        let dst_c = pixels[idx + channel] as f32 / 255.0;
        pixels[idx + channel] =
            (((src_c * src_a + dst_c * dst_a * (1.0 - src_a)) / out_a) * 255.0) as u8;
    }
    pixels[idx + 3] = (out_a * 255.0) as u8;
}

fn lerp(a: f32, b: f32, t: f32) -> f32 {
    a + (b - a) * t
}
