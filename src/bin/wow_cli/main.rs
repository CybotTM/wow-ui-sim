//! WoW UI Simulator CLI - thin client for a running wow-sim server.
//!
//! All commands except extract-textures, convert-texture, and generate require a running wow-sim
//! instance.
//!
//! Usage:
//!   wow-cli lua                      # Interactive Lua REPL
//!   wow-cli lua -e "print('hi')"     # Execute code and exit
//!   wow-cli dump-tree                # Dump frame tree from running server
//!   wow-cli dump-quads               # Dump cached live GUI quads from running server
//!   wow-cli screenshot -o out.webp   # Render screenshot via running server
//!   wow-cli extract-textures         # Extract textures to WebP (standalone)
//!   wow-cli convert-texture foo.BLP  # Convert single BLP to WebP (standalone)
//!   wow-cli generate spells          # Regenerate data/spells.rs from CSVs
//!   wow-cli startup-intern-stats      # Measure startup intern-string churn
//!   wow-cli global-slot-coverage      # Report Track 3 slot coverage after bootstrap

mod audit_api;
mod csv_util;
mod gen_atlas;
mod gen_encounter_journal;
mod gen_global_strings;
mod gen_items;
mod gen_manifest;
mod gen_map_art;
mod gen_quest_poi;
mod gen_spells;
mod gen_spells_power;
mod gen_traits;
mod gen_traits_emit;
mod gen_traits_load;
mod gen_zones;
mod global_slot_coverage;
mod startup_intern_stats;

use clap::{Parser, Subcommand};
use std::io::{self, BufRead, Write};
use std::path::{Path, PathBuf};
use wow_ui_sim::lua_server::client;

#[derive(Parser)]
#[command(name = "wow-cli")]
#[command(about = "WoW UI Simulator CLI tools (requires running wow-sim)")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Lua REPL - connect to running wow-sim and execute Lua code
    Lua {
        /// Execute code and exit
        #[arg(short = 'e', long)]
        exec: Option<String>,

        /// Execute file and exit
        #[arg(short = 'f', long)]
        file: Option<PathBuf>,

        /// List running servers
        #[arg(short = 'l', long)]
        list: bool,
    },

    /// Dump the rendered frame tree (requires running server)
    DumpTree {
        /// Filter by frame name (substring match)
        #[arg(short, long)]
        filter: Option<String>,

        /// Filter by frame name and print the full subtree of matches
        #[arg(long)]
        filter_key: Option<String>,

        /// Show only visible frames
        #[arg(long)]
        visible_only: bool,

        /// Show verbose texture detail lines, including rect and UV coords
        #[arg(short, long)]
        verbose: bool,
    },

    /// Dump cached live GUI quads from the running server
    DumpQuads {
        /// Filter by texture path substring
        #[arg(short, long)]
        filter: Option<String>,

        /// Show per-vertex detail lines
        #[arg(short, long)]
        verbose: bool,
    },

    /// Render UI to an image file (requires running server)
    Screenshot {
        /// Output file path (always lossy WebP at quality 50, extension forced to .webp)
        #[arg(short, long, default_value = "screenshot.webp")]
        output: PathBuf,

        /// Image width in pixels
        #[arg(long, default_value_t = 1600)]
        width: u32,

        /// Image height in pixels
        #[arg(long, default_value_t = 1200)]
        height: u32,

        /// Render only this frame subtree (name substring match)
        #[arg(short, long)]
        filter: Option<String>,

        /// Crop the output image to WxH+X+Y (e.g., 700x150+400+650)
        #[arg(long, value_name = "WxH+X+Y")]
        crop: Option<String>,
    },

    /// Extract textures referenced by addons to WebP format (standalone)
    ExtractTextures {
        /// Path to addons directory to scan
        #[arg(long, default_value_os_t = default_addons_path())]
        addons: PathBuf,

        /// Path to WoW Interface directory (for BLP textures)
        #[arg(long, default_value_os_t = dirs::home_dir().unwrap_or_default().join("Projects/wow/Interface"))]
        interface: PathBuf,

        /// Output directory for WebP textures
        #[arg(long, short, default_value = "./textures")]
        output: PathBuf,
    },

    /// Convert a BLP texture file to WebP format (standalone)
    ConvertTexture {
        /// Input BLP file path
        input: PathBuf,

        /// Output WebP file path (defaults to input with .webp extension)
        #[arg(short, long)]
        output: Option<PathBuf>,
    },

    /// Generate static data files from WoW CSV exports (standalone)
    Generate {
        #[command(subcommand)]
        what: GenerateTarget,
    },

    /// CASC cache and extraction commands (standalone)
    Casc {
        #[command(subcommand)]
        what: CascTarget,
    },

    /// Statically audit Blizzard UI API usage (standalone)
    AuditApi(AuditApiArgs),

    /// Measure headless startup intern-string churn.
    StartupInternStats,

    /// Report Track 3 global-slot coverage after headless bootstrap.
    GlobalSlotCoverage,
}

#[derive(clap::Args)]
struct AuditApiArgs {
    /// Output format
    #[arg(long, default_value = "text")]
    format: String,
    /// Only scan non-LoadOnDemand addons
    #[arg(long)]
    filter_startup: bool,
    /// Drill into a specific C_* namespace
    #[arg(long)]
    namespace: Option<String>,
    /// Override Blizzard UI path
    #[arg(long, default_value_os_t = default_blizzard_ui_path())]
    ui_path: PathBuf,
    /// Also scan simulator source and print a gap report
    #[arg(long)]
    gaps: bool,
    /// Path to simulator src directory (for gap analysis)
    #[arg(long, default_value = "./src")]
    sim_path: PathBuf,
    /// Path to wowless repo (for C_* namespace allowlist filtering)
    #[arg(long, default_value_os_t = default_wowless_path())]
    wowless_path: PathBuf,
}

#[derive(Subcommand)]
enum GenerateTarget {
    /// Generate data/spells.rs from SpellName/Spell/SpellMisc CSVs
    Spells,
    /// Generate data/items.rs from ItemSparse CSV
    Items,
    /// Generate data/atlas.rs from UiTextureAtlas CSVs
    Atlas,
    /// Generate data/global_strings.rs from GlobalStrings CSV
    GlobalStrings,
    /// Generate data/manifest_interface_data.rs from ManifestInterfaceData CSV
    Manifest,
    /// Generate data/traits.rs from Trait* CSVs
    Traits,
    /// Generate data/zones.rs from AreaTable CSV
    Zones,
    /// Generate data/map_art.rs from UiMap* DB2 CSVs
    MapArt,
    /// Generate data/quest_ui_map.rs from QuestPOIBlob CSV
    QuestPoi,
    /// Generate data/encounter_journal.rs from Journal* CSVs
    EncounterJournal,
}

#[derive(Subcommand)]
enum CascTarget {
    /// Extract Blizzard UI source files from local CASC into ~/.cache/wow-ui-sim
    SyncBlizzardUi,
}

fn default_addons_path() -> PathBuf {
    PathBuf::from("./Interface/AddOns")
}

fn default_blizzard_ui_path() -> PathBuf {
    PathBuf::from("./vendor/wow-ui-source/Interface/AddOns")
}

fn default_wowless_path() -> PathBuf {
    dirs::home_dir().unwrap_or_default().join("Repos/wowless")
}

fn main() {
    let cli = Cli::parse();
    handle_command(cli.command);
}

fn handle_command(command: Commands) {
    match command {
        Commands::Lua { exec, file, list } => handle_lua_command(exec, file, list),
        Commands::DumpTree {
            filter,
            filter_key,
            visible_only,
            verbose,
        } => dump_tree(filter, filter_key, visible_only, verbose),
        Commands::DumpQuads { filter, verbose } => dump_quads(filter, verbose),
        Commands::Screenshot {
            output,
            width,
            height,
            filter,
            crop,
        } => take_screenshot(&output, width, height, filter, crop),
        Commands::ExtractTextures {
            addons,
            interface,
            output,
        } => handle_extract_textures_command(addons, interface, output),
        Commands::ConvertTexture { input, output } => convert_texture(&input, output.as_ref()),
        Commands::Generate { what } => run_generator(what),
        Commands::Casc { what } => run_casc_command(what),
        Commands::AuditApi(args) => handle_audit_api(args),
        Commands::StartupInternStats => startup_intern_stats::run(),
        Commands::GlobalSlotCoverage => global_slot_coverage::run(),
    }
}

fn handle_lua_command(exec: Option<String>, file: Option<PathBuf>, list: bool) {
    if list {
        list_servers();
    } else if let Some(code) = exec {
        execute_and_exit(&code);
    } else if let Some(path) = file {
        execute_file_and_exit(&path);
    } else {
        run_repl();
    }
}

fn handle_extract_textures_command(addons: PathBuf, interface: PathBuf, output: PathBuf) {
    let (found, missing) =
        wow_ui_sim::extract_textures::extract_textures(&addons, &interface, &output);
    println!("\nSummary: {} converted, {} missing", found, missing);
}

fn run_generator(target: GenerateTarget) {
    let result = match target {
        GenerateTarget::Spells => gen_spells::run(),
        GenerateTarget::Items => gen_items::run(),
        GenerateTarget::Atlas => gen_atlas::run(),
        GenerateTarget::GlobalStrings => gen_global_strings::run(),
        GenerateTarget::Manifest => gen_manifest::run(),
        GenerateTarget::Traits => gen_traits::run(),
        GenerateTarget::Zones => gen_zones::run(),
        GenerateTarget::MapArt => gen_map_art::run(),
        GenerateTarget::QuestPoi => gen_quest_poi::run(),
        GenerateTarget::EncounterJournal => gen_encounter_journal::run(),
    };
    if let Err(e) = result {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    }
}

fn run_casc_command(target: CascTarget) {
    let result = match target {
        CascTarget::SyncBlizzardUi => wow_ui_sim::blizzard_ui_sync::sync_blizzard_ui().map(|s| {
            println!(
                "Synced {} Blizzard UI files to {} ({} extracted, {} already present)",
                s.total,
                s.root.display(),
                s.extracted,
                s.present
            );
        }),
    };
    if let Err(e) = result {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    }
}

fn handle_audit_api(args: AuditApiArgs) {
    let fmt = parse_output_format(&args.format);
    let config = build_audit_config(
        args.ui_path,
        args.namespace,
        args.filter_startup,
        args.wowless_path,
    );
    let results = audit_api::run_audit(&config);
    let gap_report = args
        .gaps
        .then(|| build_gap_report(&args.sim_path, &results));
    print_audit_output(fmt, &results, gap_report.as_ref());
}

fn parse_output_format(format: &str) -> audit_api::OutputFormat {
    match format {
        "json" => audit_api::OutputFormat::Json,
        "plan" => audit_api::OutputFormat::Plan,
        _ => audit_api::OutputFormat::Text,
    }
}

fn build_audit_config(
    ui_path: PathBuf,
    namespace: Option<String>,
    filter_startup: bool,
    wowless_path: PathBuf,
) -> audit_api::AuditConfig {
    let wowless_opt = wowless_path
        .join("data/products/wow/apis.yaml")
        .exists()
        .then_some(wowless_path);
    audit_api::AuditConfig {
        ui_path,
        namespace_filter: namespace,
        filter_startup,
        wowless_path: wowless_opt,
    }
}

fn build_gap_report(sim_path: &Path, results: &audit_api::AuditResults) -> audit_api::GapReport {
    let registered = audit_api::scan_simulator(sim_path);
    let sim_methods = audit_api::introspect_simulator_c_methods();
    audit_api::build_gap_report(results, &registered, &sim_methods)
}

fn print_audit_output(
    fmt: audit_api::OutputFormat,
    results: &audit_api::AuditResults,
    gap_report: Option<&audit_api::GapReport>,
) {
    match fmt {
        audit_api::OutputFormat::Json => audit_api::print_json(results, gap_report),
        audit_api::OutputFormat::Plan => match gap_report {
            Some(report) => audit_api::print_gap_plan(report),
            None => eprintln!("--format plan requires --gaps"),
        },
        audit_api::OutputFormat::Text => {
            audit_api::print_text(results);
            if let Some(report) = gap_report {
                println!();
                audit_api::print_gap_text(report);
            }
        }
    }
}

// ── IPC helpers ─────────────────────────────────────────────────────

fn resolve_socket() -> PathBuf {
    match std::env::var("WOW_LUA_SOCKET") {
        Ok(s) => PathBuf::from(s),
        Err(_) => match find_server() {
            Some(s) => s,
            None => std::process::exit(1),
        },
    }
}

fn find_server() -> Option<PathBuf> {
    let servers = client::find_servers();
    if servers.is_empty() {
        eprintln!("Error: No wow-sim server found.");
        eprintln!("Start wow-sim first, then run wow-cli.");
        return None;
    }
    if servers.len() > 1 {
        eprintln!("Multiple servers found. Using first one.");
        eprintln!("Use --list to see all, or set WOW_LUA_SOCKET to specify.");
    }
    Some(servers.into_iter().next().unwrap())
}

fn list_servers() {
    let servers = client::find_servers();
    if servers.is_empty() {
        println!("No wow-sim servers found.");
        println!("Start wow-sim first, then run wow-cli lua.");
    } else {
        println!("Running servers:");
        for server in &servers {
            let status = match client::ping(server) {
                Ok(()) => "OK",
                Err(_) => "ERROR",
            };
            println!("  {} [{}]", server.display(), status);
        }
    }
}

// ── Subcommand handlers ─────────────────────────────────────────────

fn execute_and_exit(code: &str) {
    let socket = resolve_socket();
    match client::exec(&socket, code) {
        Ok(output) => {
            if !output.is_empty() {
                println!("{}", output);
            }
        }
        Err(e) => {
            eprintln!("Error: {}", e);
            std::process::exit(1);
        }
    }
}

fn execute_file_and_exit(path: &PathBuf) {
    let code = match std::fs::read_to_string(path) {
        Ok(c) => c,
        Err(e) => {
            eprintln!("Error reading {}: {}", path.display(), e);
            std::process::exit(1);
        }
    };
    execute_and_exit(&code);
}

fn dump_tree(
    filter: Option<String>,
    filter_key: Option<String>,
    visible_only: bool,
    verbose: bool,
) {
    let socket = resolve_socket();
    match client::dump_tree(&socket, filter, filter_key, visible_only, verbose) {
        Ok(tree) => println!("{}", tree),
        Err(e) => {
            eprintln!("Error: {}", e);
            std::process::exit(1);
        }
    }
}

fn dump_quads(filter: Option<String>, verbose: bool) {
    let socket = resolve_socket();
    match client::dump_quads(&socket, filter, verbose) {
        Ok(dump) => println!("{}", dump),
        Err(e) => {
            eprintln!("Error: {}", e);
            std::process::exit(1);
        }
    }
}

fn take_screenshot(
    output: &PathBuf,
    width: u32,
    height: u32,
    filter: Option<String>,
    crop: Option<String>,
) {
    let socket = resolve_socket();
    // Canonicalize output path so the server can write to the right location
    let abs_output = std::env::current_dir()
        .map(|cwd| cwd.join(output))
        .unwrap_or_else(|_| output.clone());
    match client::screenshot(
        &socket,
        &abs_output.to_string_lossy(),
        width,
        height,
        filter,
        crop,
    ) {
        Ok(msg) => println!("{}", msg),
        Err(e) => {
            eprintln!("Error: {}", e);
            std::process::exit(1);
        }
    }
}

fn convert_texture(input: &PathBuf, output: Option<&PathBuf>) {
    use image_blp::{convert::blp_to_image, parser::load_blp};

    let output_path = match output {
        Some(p) => p.clone(),
        None => input.with_extension("webp"),
    };

    let blp = match load_blp(input) {
        Ok(b) => b,
        Err(e) => {
            eprintln!("Error loading BLP {}: {}", input.display(), e);
            std::process::exit(1);
        }
    };

    let img = match blp_to_image(&blp, 0) {
        Ok(i) => i,
        Err(e) => {
            eprintln!("Error converting BLP: {}", e);
            std::process::exit(1);
        }
    };

    let mut rgba = img.to_rgba8();
    // Fix 1-bit alpha: image-blp decodes 1-bit alpha as literal 0/1 byte values
    wow_ui_sim::texture::fix_1bit_alpha(rgba.as_mut());
    if let Err(e) = rgba.save(&output_path) {
        eprintln!("Error saving {}: {}", output_path.display(), e);
        std::process::exit(1);
    }

    println!(
        "Converted {} -> {} ({}x{})",
        input.display(),
        output_path.display(),
        rgba.width(),
        rgba.height()
    );
}

// ── REPL ────────────────────────────────────────────────────────────

fn run_repl() {
    let mut socket = resolve_socket();

    println!("Connected to {}", socket.display());
    println!("Type Lua code to execute. Use .exit to quit.");
    println!();

    let stdin = io::stdin();
    let mut stdout = io::stdout();

    loop {
        print!("> ");
        stdout.flush().unwrap();

        let mut line = String::new();
        match stdin.lock().read_line(&mut line) {
            Ok(0) => break,
            Ok(_) => {}
            Err(e) => {
                eprintln!("Read error: {}", e);
                break;
            }
        }

        let line = line.trim();
        if line.is_empty() {
            continue;
        }

        if !handle_repl_command(line, &mut socket) {
            break;
        }
    }

    println!("Goodbye!");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn dump_tree_accepts_filter_key_flag() {
        let cli = Cli::try_parse_from(["wow-cli", "dump-tree", "--filter-key", "PartyFrame"])
            .expect("dump-tree should parse --filter-key");
        match cli.command {
            Commands::DumpTree { filter_key, .. } => {
                assert_eq!(filter_key.as_deref(), Some("PartyFrame"));
            }
            _ => panic!("expected dump-tree command"),
        }
    }

    #[test]
    fn dump_quads_accepts_filter_and_verbose_flags() {
        let cli = Cli::try_parse_from([
            "wow-cli",
            "dump-quads",
            "--filter",
            "uigroupmanager",
            "--verbose",
        ])
        .expect("dump-quads should parse flags");
        match cli.command {
            Commands::DumpQuads { filter, verbose } => {
                assert_eq!(filter.as_deref(), Some("uigroupmanager"));
                assert!(verbose);
            }
            _ => panic!("expected dump-quads command"),
        }
    }

    #[test]
    fn casc_sync_blizzard_ui_parses() {
        let cli = Cli::try_parse_from(["wow-cli", "casc", "sync-blizzard-ui"])
            .expect("casc sync-blizzard-ui should parse");
        match cli.command {
            Commands::Casc {
                what: CascTarget::SyncBlizzardUi,
            } => {}
            _ => panic!("expected casc sync-blizzard-ui command"),
        }
    }
}

/// Handle a REPL input line. Returns false to exit the loop.
fn handle_repl_command(line: &str, socket: &mut PathBuf) -> bool {
    if line.starts_with('.') {
        match line {
            ".exit" | ".quit" | ".q" => return false,
            ".servers" => list_servers(),
            cmd if cmd.starts_with(".connect ") => {
                let path = cmd.strip_prefix(".connect ").unwrap().trim();
                *socket = PathBuf::from(path);
                println!("Switched to {}", socket.display());
            }
            _ => eprintln!("Unknown command: {}", line),
        }
        return true;
    }

    match client::exec(socket, line) {
        Ok(output) => {
            if !output.is_empty() {
                println!("{}", output);
            }
        }
        Err(e) => eprintln!("Error: {}", e),
    }
    true
}
