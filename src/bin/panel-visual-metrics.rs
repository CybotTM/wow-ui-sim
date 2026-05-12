use std::env;
use std::fs;
use std::path::Path;

use image::{DynamicImage, GenericImageView, ImageReader};

const ACTIVE_PIXEL_MIN_RGB_SUM: u16 = 8;
const ACTIVE_PIXEL_MIN_ALPHA: u8 = 8;
const ACTIVE_PIXEL_TOLERANCE_PERCENT: u64 = 30;
const MIN_STDDEV_PERCENT: u64 = 50;
const MAX_AHASH_DISTANCE: u32 = 18;

#[derive(Debug, Clone)]
struct Metrics {
    width: u32,
    height: u32,
    active_pixels: u64,
    luma_stddev_milli: u64,
    ahash: u64,
}

fn main() {
    if let Err(err) = run() {
        eprintln!("ERROR: {err}");
        std::process::exit(1);
    }
}

fn run() -> Result<(), String> {
    let args = env::args().collect::<Vec<_>>();
    match args.get(1).map(String::as_str) {
        Some("record") if args.len() == 4 => record(&args[2], Path::new(&args[3])),
        Some("compare") if args.len() == 5 => {
            compare(Path::new(&args[2]), &args[3], Path::new(&args[4]))
        }
        _ => Err(format!(
            "usage: {} record SLUG IMAGE | compare BASELINE SLUG IMAGE",
            args.first()
                .cloned()
                .unwrap_or_else(|| "panel-visual-metrics".to_string())
        )),
    }
}

fn record(slug: &str, image_path: &Path) -> Result<(), String> {
    let metrics = measure_image(image_path)?;
    println!(
        "{slug}\t{}\t{}\t{}\t{}\t{:016x}",
        metrics.width,
        metrics.height,
        metrics.active_pixels,
        metrics.luma_stddev_milli,
        metrics.ahash
    );
    Ok(())
}

fn compare(baseline_path: &Path, slug: &str, image_path: &Path) -> Result<(), String> {
    let expected = read_baseline_metrics(baseline_path, slug)?;
    let actual = measure_image(image_path)?;
    validate_metrics(slug, &expected, &actual)
}

fn measure_image(image_path: &Path) -> Result<Metrics, String> {
    let image = ImageReader::open(image_path)
        .map_err(|err| format!("opening {}: {err}", image_path.display()))?
        .decode()
        .map_err(|err| format!("decoding {}: {err}", image_path.display()))?;
    let (width, height) = image.dimensions();
    let stats = collect_active_pixel_stats(&image);

    Ok(Metrics {
        width,
        height,
        active_pixels: stats.active_pixels,
        luma_stddev_milli: stats.luma_stddev_milli(),
        ahash: average_hash(&image),
    })
}

struct ActivePixelStats {
    active_pixels: u64,
    luma_sum: f64,
    luma_sq_sum: f64,
}

impl ActivePixelStats {
    fn luma_stddev_milli(&self) -> u64 {
        if self.active_pixels == 0 {
            return 0;
        }
        let count = self.active_pixels as f64;
        let mean = self.luma_sum / count;
        let variance = (self.luma_sq_sum / count) - (mean * mean);
        (variance.max(0.0).sqrt() * 1000.0).round() as u64
    }
}

fn collect_active_pixel_stats(image: &DynamicImage) -> ActivePixelStats {
    let mut stats = ActivePixelStats {
        active_pixels: 0,
        luma_sum: 0.0,
        luma_sq_sum: 0.0,
    };
    for pixel in image.to_rgba8().pixels() {
        let [r, g, b, a] = pixel.0;
        if is_active_pixel(r, g, b, a) {
            stats.active_pixels += 1;
            let luma = luma(r, g, b);
            stats.luma_sum += luma;
            stats.luma_sq_sum += luma * luma;
        }
    }
    stats
}

fn is_active_pixel(r: u8, g: u8, b: u8, a: u8) -> bool {
    a > ACTIVE_PIXEL_MIN_ALPHA
        && u16::from(r) + u16::from(g) + u16::from(b) > ACTIVE_PIXEL_MIN_RGB_SUM
}

fn luma(r: u8, g: u8, b: u8) -> f64 {
    0.2126 * f64::from(r) + 0.7152 * f64::from(g) + 0.0722 * f64::from(b)
}

fn average_hash(image: &DynamicImage) -> u64 {
    let gray = image
        .resize_exact(8, 8, image::imageops::FilterType::Triangle)
        .to_luma8();
    let values = gray
        .pixels()
        .map(|pixel| u64::from(pixel.0[0]))
        .collect::<Vec<_>>();
    let average = values.iter().sum::<u64>() / values.len() as u64;
    values
        .iter()
        .enumerate()
        .fold(0_u64, |hash, (index, value)| {
            if *value >= average {
                hash | (1_u64 << index)
            } else {
                hash
            }
        })
}

fn read_baseline_metrics(baseline_path: &Path, slug: &str) -> Result<Metrics, String> {
    let contents = fs::read_to_string(baseline_path)
        .map_err(|err| format!("reading {}: {err}", baseline_path.display()))?;
    for line in contents.lines() {
        if line.starts_with('#') || line.trim().is_empty() {
            continue;
        }
        let fields = line.split('\t').collect::<Vec<_>>();
        if fields.first() == Some(&slug) {
            return parse_metrics_line(&fields);
        }
    }
    Err(format!(
        "no visual baseline row for slug '{slug}' in {}",
        baseline_path.display()
    ))
}

fn parse_metrics_line(fields: &[&str]) -> Result<Metrics, String> {
    if fields.len() != 6 {
        return Err(format!(
            "visual baseline row has {} fields, expected 6",
            fields.len()
        ));
    }
    Ok(Metrics {
        width: parse_field(fields[1], "width")?,
        height: parse_field(fields[2], "height")?,
        active_pixels: parse_field(fields[3], "active_pixels")?,
        luma_stddev_milli: parse_field(fields[4], "luma_stddev_milli")?,
        ahash: u64::from_str_radix(fields[5], 16)
            .map_err(|err| format!("parsing ahash '{}': {err}", fields[5]))?,
    })
}

fn parse_field<T>(value: &str, name: &str) -> Result<T, String>
where
    T: std::str::FromStr,
    T::Err: std::fmt::Display,
{
    value
        .parse::<T>()
        .map_err(|err| format!("parsing {name} '{value}': {err}"))
}

fn validate_metrics(slug: &str, expected: &Metrics, actual: &Metrics) -> Result<(), String> {
    if expected.width != actual.width || expected.height != actual.height {
        return Err(format!(
            "{slug}: dimensions changed from {}x{} to {}x{}",
            expected.width, expected.height, actual.width, actual.height
        ));
    }
    if !within_percent(
        expected.active_pixels,
        actual.active_pixels,
        ACTIVE_PIXEL_TOLERANCE_PERCENT,
    ) {
        return Err(format!(
            "{slug}: active pixels changed from {} to {}",
            expected.active_pixels, actual.active_pixels
        ));
    }
    let min_stddev = expected.luma_stddev_milli * MIN_STDDEV_PERCENT / 100;
    if actual.luma_stddev_milli < min_stddev {
        return Err(format!(
            "{slug}: luminance contrast fell from {} to {}",
            expected.luma_stddev_milli, actual.luma_stddev_milli
        ));
    }
    let distance = (expected.ahash ^ actual.ahash).count_ones();
    if distance > MAX_AHASH_DISTANCE {
        return Err(format!(
            "{slug}: visual hash distance {distance} exceeded {MAX_AHASH_DISTANCE}"
        ));
    }
    Ok(())
}

fn within_percent(expected: u64, actual: u64, tolerance_percent: u64) -> bool {
    let tolerance = expected * tolerance_percent / 100;
    actual >= expected.saturating_sub(tolerance) && actual <= expected.saturating_add(tolerance)
}
