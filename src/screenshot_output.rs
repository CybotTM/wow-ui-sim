//! Shared screenshot encoding; callers retain responsibility for image geometry and colors.

use std::path::{Path, PathBuf};

/// Save RGBA pixels as lossless PNG or lossy WebP, forcing the matching extension.
pub fn save_screenshot(
    img: &image::RgbaImage,
    output: &Path,
    png: bool,
    webp_quality: f32,
) -> Result<PathBuf, String> {
    let output = output.with_extension(if png { "png" } else { "webp" });
    if png {
        img.save_with_format(&output, image::ImageFormat::Png)
            .map_err(|error| {
                format!(
                    "Failed to save screenshot to '{}': {error}",
                    output.display()
                )
            })?;
    } else {
        let encoder = webp::Encoder::from_rgba(img.as_raw(), img.width(), img.height());
        let encoded = encoder.encode(webp_quality);
        std::fs::write(&output, &*encoded).map_err(|error| {
            format!(
                "Failed to save screenshot to '{}': {error}",
                output.display()
            )
        })?;
    }
    Ok(output)
}

#[cfg(test)]
mod tests {
    use super::save_screenshot;
    use image::{ImageFormat, RgbaImage};
    use std::fs;

    fn rgba_fixture() -> RgbaImage {
        RgbaImage::from_raw(
            2,
            2,
            vec![
                12, 34, 56, 255, 78, 90, 123, 128, 145, 167, 189, 0, 210, 232, 254, 1,
            ],
        )
        .unwrap()
    }

    #[test]
    fn png_preserves_every_rgba_byte_and_forces_extension() {
        let directory = tempfile::tempdir().unwrap();
        let image = rgba_fixture();
        for filename in ["capture.webp", "capture.jpg", "capture", "capture.png"] {
            let requested = directory.path().join(filename);
            let expected = requested.with_extension("png");
            let saved = save_screenshot(&image, &requested, true, 15.0).unwrap();

            assert_eq!(saved, expected);
            let bytes = fs::read(&saved).unwrap();
            assert!(bytes.starts_with(b"\x89PNG\r\n\x1a\n"));
            let decoded = image::load_from_memory_with_format(&bytes, ImageFormat::Png)
                .unwrap()
                .to_rgba8();
            assert_eq!(decoded.dimensions(), image.dimensions());
            assert_eq!(decoded.as_raw(), image.as_raw());
            if requested != expected {
                assert!(!requested.exists());
            }
            fs::remove_file(saved).unwrap();
        }
    }

    #[test]
    fn webp_has_riff_webp_signature_and_forces_extension() {
        let directory = tempfile::tempdir().unwrap();
        let image = rgba_fixture();
        for quality in [15.0, 65.0] {
            for filename in ["capture.png", "capture", "capture.webp"] {
                let requested = directory.path().join(filename);
                let expected = requested.with_extension("webp");
                let saved = save_screenshot(&image, &requested, false, quality).unwrap();

                assert_eq!(saved, expected);
                let bytes = fs::read(&saved).unwrap();
                assert_eq!(&bytes[..4], b"RIFF");
                assert_eq!(&bytes[8..12], b"WEBP");
                let decoded =
                    image::load_from_memory_with_format(&bytes, ImageFormat::WebP).unwrap();
                assert_eq!((decoded.width(), decoded.height()), image.dimensions());
                if requested != expected {
                    assert!(!requested.exists());
                }
                fs::remove_file(saved).unwrap();
            }
        }
    }

    #[test]
    fn missing_parent_returns_write_error_for_both_formats() {
        let directory = tempfile::tempdir().unwrap();
        let requested = directory.path().join("missing/capture.jpg");
        for png in [true, false] {
            let error = save_screenshot(&rgba_fixture(), &requested, png, 65.0).unwrap_err();
            assert!(!error.is_empty());
            assert!(!requested.with_extension("png").exists());
            assert!(!requested.with_extension("webp").exists());
        }
        assert!(!directory.path().join("missing").exists());
    }

    #[test]
    fn directory_destination_returns_write_error_for_both_formats() {
        let directory = tempfile::tempdir().unwrap();
        let requested = directory.path().join("capture.jpg");
        for (png, extension) in [(true, "png"), (false, "webp")] {
            let destination = requested.with_extension(extension);
            fs::create_dir(&destination).unwrap();
            let error = save_screenshot(&rgba_fixture(), &requested, png, 65.0).unwrap_err();
            assert!(!error.is_empty());
            assert!(destination.is_dir());
            assert_eq!(fs::read_dir(&destination).unwrap().count(), 0);
        }
    }
}
