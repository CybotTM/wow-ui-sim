//! Generator for map_art.rs from WoW DB2 CSV exports.
//!
//! Reads from data/db2/:
//!   - UiMapXMapArt.csv    (mapID → artID)
//!   - UiMapArt.csv        (artID → styleID)
//!   - UiMapArtStyleLayer.csv (styleID → layer dimensions)
//!   - UiMapArtTile.csv    (artID + layerIndex → fileDataIDs)
//!
//! Generates: data/map_art.rs

use super::csv_util::parse_csv_line;
use std::collections::{BTreeMap, HashMap};
use std::fs::File;
use std::io::{BufRead, BufReader, Write};
use std::path::Path;

/// One layer's dimension info from UiMapArtStyleLayer.
#[derive(Debug, Clone)]
struct StyleLayer {
    layer_index: u32,
    layer_width: u32,
    layer_height: u32,
    tile_width: u32,
    tile_height: u32,
    min_scale: String,
    max_scale: String,
    additional_zoom_steps: u32,
}

pub fn run() -> Result<(), Box<dyn std::error::Error>> {
    let db2_dir = Path::new("data/db2");

    println!("Loading UiMapArtStyleLayer...");
    // styleID → Vec<StyleLayer> (sorted by layer_index)
    let style_layers = load_style_layers(&db2_dir.join("UiMapArtStyleLayer.csv"))?;

    println!("Loading UiMapArt...");
    // artID → styleID
    let art_style: HashMap<u32, u32> = load_art_style(&db2_dir.join("UiMapArt.csv"))?;

    println!("Loading UiMapArtTile...");
    // (artID, layerIndex) → BTreeMap<(row,col), fileDataID>
    let tiles = load_tiles(&db2_dir.join("UiMapArtTile.csv"))?;

    println!("Loading UiMapXMapArt...");
    // mapID → artID (PhaseID=0 only)
    let map_art = load_map_art(&db2_dir.join("UiMapXMapArt.csv"))?;

    let output_path = Path::new("data/map_art.rs");
    std::fs::create_dir_all("data")?;
    let mut out = File::create(output_path)?;

    let map_count = write_map_art(&mut out, &map_art, &art_style, &style_layers, &tiles)?;

    println!("Generated {} map entries", map_count);
    println!("Output: {}", output_path.display());
    Ok(())
}

fn load_style_layers(
    path: &Path,
) -> Result<HashMap<u32, Vec<StyleLayer>>, Box<dyn std::error::Error>> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let mut map: HashMap<u32, Vec<StyleLayer>> = HashMap::new();

    for (i, line) in reader.lines().enumerate() {
        let line = line?;
        if i == 0 {
            continue;
        }
        let f = parse_csv_line(&line);
        if f.len() < 10 {
            continue;
        }
        let style_id: u32 = f[9].parse().unwrap_or(0);
        if style_id == 0 {
            continue;
        }
        map.entry(style_id).or_default().push(StyleLayer {
            layer_index: f[1].parse().unwrap_or(0),
            layer_width: f[2].parse().unwrap_or(0),
            layer_height: f[3].parse().unwrap_or(0),
            tile_width: f[4].parse().unwrap_or(0),
            tile_height: f[5].parse().unwrap_or(0),
            min_scale: f[6].clone(),
            max_scale: f[7].clone(),
            additional_zoom_steps: f[8].parse().unwrap_or(0),
        });
    }
    // Sort each style's layers by layer_index
    for layers in map.values_mut() {
        layers.sort_by_key(|l| l.layer_index);
    }
    Ok(map)
}

fn load_art_style(path: &Path) -> Result<HashMap<u32, u32>, Box<dyn std::error::Error>> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let mut map = HashMap::new();

    for (i, line) in reader.lines().enumerate() {
        let line = line?;
        if i == 0 {
            continue;
        }
        let f = parse_csv_line(&line);
        if f.len() < 4 {
            continue;
        }
        let art_id: u32 = match f[0].parse() {
            Ok(v) => v,
            Err(_) => continue,
        };
        let style_id: u32 = f[3].parse().unwrap_or(0);
        if style_id > 0 {
            map.insert(art_id, style_id);
        }
    }
    Ok(map)
}

fn load_tiles(
    path: &Path,
) -> Result<HashMap<(u32, u32), BTreeMap<(u32, u32), u32>>, Box<dyn std::error::Error>> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    // key: (artID, layerIndex), value: BTreeMap<(row, col), fileDataID>
    let mut map: HashMap<(u32, u32), BTreeMap<(u32, u32), u32>> = HashMap::new();

    for (i, line) in reader.lines().enumerate() {
        let line = line?;
        if i == 0 {
            continue;
        }
        let f = parse_csv_line(&line);
        if f.len() < 6 {
            continue;
        }
        let row: u32 = f[1].parse().unwrap_or(0);
        let col: u32 = f[2].parse().unwrap_or(0);
        let layer: u32 = f[3].parse().unwrap_or(0);
        let file_data_id: u32 = f[4].parse().unwrap_or(0);
        let art_id: u32 = f[5].parse().unwrap_or(0);

        if file_data_id == 0 || art_id == 0 {
            continue;
        }
        map.entry((art_id, layer))
            .or_default()
            .insert((row, col), file_data_id);
    }
    Ok(map)
}

fn load_map_art(path: &Path) -> Result<BTreeMap<u32, u32>, Box<dyn std::error::Error>> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let mut map = BTreeMap::new();

    for (i, line) in reader.lines().enumerate() {
        let line = line?;
        if i == 0 {
            continue;
        }
        let f = parse_csv_line(&line);
        if f.len() < 4 {
            continue;
        }
        let phase_id: u32 = f[1].parse().unwrap_or(u32::MAX);
        if phase_id != 0 {
            continue;
        }
        let art_id: u32 = f[2].parse().unwrap_or(0);
        let map_id: u32 = f[3].parse().unwrap_or(0);
        if art_id > 0 && map_id > 0 {
            map.insert(map_id, art_id);
        }
    }
    Ok(map)
}

fn write_map_art(
    out: &mut File,
    map_art: &BTreeMap<u32, u32>,
    art_style: &HashMap<u32, u32>,
    style_layers: &HashMap<u32, Vec<StyleLayer>>,
    tiles: &HashMap<(u32, u32), BTreeMap<(u32, u32), u32>>,
) -> Result<u32, Box<dyn std::error::Error>> {
    write_file_header(out)?;
    write_structs(out)?;

    // Collect all unique artIDs needed
    let mut art_ids: Vec<u32> = map_art.values().copied().collect();
    art_ids.sort_unstable();
    art_ids.dedup();

    // For each artID, emit tile slices and layer slices
    for &art_id in &art_ids {
        let style_id = match art_style.get(&art_id) {
            Some(&s) => s,
            None => continue,
        };
        let layers = match style_layers.get(&style_id) {
            Some(l) => l,
            None => continue,
        };

        for layer in layers {
            let layer_idx = layer.layer_index;
            let tile_map = tiles.get(&(art_id, layer_idx));

            let ids: Vec<u32> = if let Some(tm) = tile_map {
                // Determine grid dimensions from the max row/col seen
                let max_row = tm.keys().map(|&(r, _)| r).max().unwrap_or(0);
                let max_col = tm.keys().map(|&(_, c)| c).max().unwrap_or(0);
                let num_cols = max_col + 1;
                let num_rows = max_row + 1;
                let len = (num_rows * num_cols) as usize;
                let mut v = vec![0u32; len];
                for (&(row, col), &fdid) in tm {
                    let idx = (row * num_cols + col) as usize;
                    v[idx] = fdid;
                }
                v
            } else {
                vec![]
            };

            write!(
                out,
                "static ART_{}_LAYER_{}_TILES: [u32; {}] = [",
                art_id,
                layer_idx,
                ids.len()
            )?;
            for (i, &id) in ids.iter().enumerate() {
                if i > 0 {
                    write!(out, ",")?;
                }
                write!(out, "{}", id)?;
            }
            writeln!(out, "];")?;
        }

        // Emit the layers slice
        writeln!(
            out,
            "static ART_{}_LAYERS: [MapArtLayer; {}] = [",
            art_id,
            layers.len()
        )?;
        for layer in layers {
            writeln!(
                out,
                "    MapArtLayer {{ layer_width: {}, layer_height: {}, tile_width: {}, tile_height: {}, min_scale: {}f32, max_scale: {}f32, additional_zoom_steps: {} }},",
                layer.layer_width,
                layer.layer_height,
                layer.tile_width,
                layer.tile_height,
                layer.min_scale,
                layer.max_scale,
                layer.additional_zoom_steps,
            )?;
        }
        writeln!(out, "];")?;

        // Emit the tile slices-of-slices
        writeln!(
            out,
            "static ART_{}_TILE_SLICES: [&[u32]; {}] = [",
            art_id,
            layers.len()
        )?;
        for layer in layers {
            writeln!(
                out,
                "    &ART_{}_LAYER_{}_TILES,",
                art_id, layer.layer_index
            )?;
        }
        writeln!(out, "];")?;
    }

    writeln!(out)?;

    // Build phf map
    let mut builder = phf_codegen::Map::new();
    let mut count = 0u32;

    for (&map_id, &art_id) in map_art {
        let style_id = match art_style.get(&art_id) {
            Some(&s) => s,
            None => continue,
        };
        if style_layers.get(&style_id).is_none() {
            continue;
        }

        let value = format!(
            "MapArtInfo {{ art_id: {}, layers: &ART_{}_LAYERS, tiles: &ART_{}_TILE_SLICES }}",
            art_id, art_id, art_id
        );
        builder.entry(map_id, &value);
        count += 1;
    }

    writeln!(
        out,
        "pub static MAP_ART_DB: phf::Map<u32, MapArtInfo> = {};",
        builder.build()
    )?;
    writeln!(out)?;
    write_lookup_fn(out)?;

    Ok(count)
}

fn write_file_header(out: &mut File) -> std::io::Result<()> {
    writeln!(
        out,
        "//! Auto-generated map art tile data from WoW DB2 exports."
    )?;
    writeln!(
        out,
        "//! Do not edit manually - regenerate with: wow-cli generate map-art"
    )?;
    writeln!(out)?;
    Ok(())
}

fn write_structs(out: &mut File) -> std::io::Result<()> {
    writeln!(out, "/// Map art layer info (dimensions and scale).")?;
    writeln!(out, "#[derive(Debug, Clone, Copy)]")?;
    writeln!(out, "pub struct MapArtLayer {{")?;
    writeln!(out, "    pub layer_width: u32,")?;
    writeln!(out, "    pub layer_height: u32,")?;
    writeln!(out, "    pub tile_width: u32,")?;
    writeln!(out, "    pub tile_height: u32,")?;
    writeln!(out, "    pub min_scale: f32,")?;
    writeln!(out, "    pub max_scale: f32,")?;
    writeln!(out, "    pub additional_zoom_steps: u32,")?;
    writeln!(out, "}}")?;
    writeln!(out)?;
    writeln!(
        out,
        "/// Map art info (art ID, style layers, and tile fileDataIDs per layer)."
    )?;
    writeln!(out, "#[derive(Debug, Clone)]")?;
    writeln!(out, "pub struct MapArtInfo {{")?;
    writeln!(out, "    pub art_id: u32,")?;
    writeln!(out, "    pub layers: &'static [MapArtLayer],")?;
    writeln!(
        out,
        "    pub tiles: &'static [&'static [u32]], // per layer, ordered row-major"
    )?;
    writeln!(out, "}}")?;
    writeln!(out)?;
    Ok(())
}

fn write_lookup_fn(out: &mut File) -> std::io::Result<()> {
    writeln!(
        out,
        "pub fn get_map_art(map_id: u32) -> Option<&'static MapArtInfo> {{"
    )?;
    writeln!(out, "    MAP_ART_DB.get(&map_id)")?;
    writeln!(out, "}}")?;
    Ok(())
}
