//! Render the app icon and write `installer/wow-sim.ico` and
//! `installer/wow-sim.png`.
//!
//! Run after editing `app_icon_render.rs` to regenerate the Windows icon
//! resource embedded into `wow-sim.exe` by `build.rs` and the source PNG
//! used to build the macOS `.icns`:
//!
//! ```text
//! cargo run --example gen_app_icon
//! ```
//!
//! The ICO is a single PNG-encoded entry at `app_icon_render::SIZE`.
//! The PNG is the same raw bitmap, suitable as input to `sips`/`iconutil`.

use std::fs::File;
use std::io::{BufWriter, Write};
use std::path::PathBuf;

use image::{ImageBuffer, Rgba, codecs::png::PngEncoder};
use wow_ui_sim::app_icon_render::{SIZE, render_icon};

fn main() -> std::io::Result<()> {
    let pixels = render_icon();
    let image: ImageBuffer<Rgba<u8>, _> =
        ImageBuffer::from_raw(SIZE, SIZE, pixels).expect("renderer produced wrong buffer size");

    let mut png_bytes: Vec<u8> = Vec::with_capacity(64 * 1024);
    image
        .write_with_encoder(PngEncoder::new(&mut png_bytes))
        .expect("PNG encoding failed");

    let png_path: PathBuf = [env!("CARGO_MANIFEST_DIR"), "installer", "wow-sim.png"]
        .iter()
        .collect();
    let mut png_writer = BufWriter::new(File::create(&png_path)?);
    png_writer.write_all(&png_bytes)?;
    png_writer.flush()?;
    println!("wrote {} ({} bytes)", png_path.display(), png_bytes.len());

    let ico_bytes = wrap_png_in_ico(&png_bytes, SIZE);
    let ico_path: PathBuf = [env!("CARGO_MANIFEST_DIR"), "installer", "wow-sim.ico"]
        .iter()
        .collect();
    let mut ico_writer = BufWriter::new(File::create(&ico_path)?);
    ico_writer.write_all(&ico_bytes)?;
    ico_writer.flush()?;
    println!("wrote {} ({} bytes)", ico_path.display(), ico_bytes.len());
    Ok(())
}

/// Wrap a PNG byte stream in a single-entry ICO container.
///
/// Windows Vista+ accepts PNG-encoded ICO entries for any size, and PNG
/// preserves the renderer's full alpha channel where BMP-encoded entries
/// require a separate AND mask.
fn wrap_png_in_ico(png: &[u8], size: u32) -> Vec<u8> {
    assert!(size <= 256, "ICO entries are limited to 256x256");
    let dim = if size == 256 { 0u8 } else { size as u8 };

    let mut out = Vec::with_capacity(22 + png.len());
    // ICONDIR: reserved=0, type=1 (icon), count=1
    out.extend_from_slice(&0u16.to_le_bytes());
    out.extend_from_slice(&1u16.to_le_bytes());
    out.extend_from_slice(&1u16.to_le_bytes());
    // ICONDIRENTRY
    out.push(dim); // width  (0 == 256)
    out.push(dim); // height (0 == 256)
    out.push(0); // color count (0 for >=8bpp)
    out.push(0); // reserved
    out.extend_from_slice(&1u16.to_le_bytes()); // color planes
    out.extend_from_slice(&32u16.to_le_bytes()); // bits per pixel
    out.extend_from_slice(&(png.len() as u32).to_le_bytes()); // image size
    out.extend_from_slice(&22u32.to_le_bytes()); // image offset
    out.extend_from_slice(png);
    out
}
