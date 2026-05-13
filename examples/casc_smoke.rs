//! Smoke test for CASC texture resolution.

use wow_ui_sim::texture::TextureManager;

fn main() {
    unsafe {
        std::env::set_var("WOW_SIM_CASC", "1");
    }

    println!("\n--- direct asset-resolver lookups ---");
    let direct_probes = [
        "interface/buttons/ui-panel-button-up.blp",
        "Interface/Buttons/UI-Panel-Button-Up.blp",
        "interface/buttons/ui-panel-button-up.BLP",
        "Interface\\Buttons\\UI-Panel-Button-Up.blp",
        "Fonts/FRIZQT__.TTF",
        "Fonts\\FRIZQT__.TTF",
        "Fonts/ARIALN.TTF",
        "Fonts/FRIZQT___CYR.TTF",
        "fonts/trajanpro3semibold.ttf",
        "fonts/skurri.ttf",
        "fonts/morpheus.ttf",
    ];
    for probe in direct_probes {
        let fdid = asset_resolver::lookup_path(probe);
        println!("  {:60} -> {:?}", probe, fdid);
    }

    println!("\n--- TextureManager.load (full pipeline) ---");
    let mut mgr = TextureManager::new();

    let probes = [
        "Interface\\Buttons\\UI-Panel-Button-Up",
        "Interface\\DialogFrame\\UI-DialogBox-Background",
        "Interface\\Icons\\INV_Misc_QuestionMark",
    ];

    let mut failed = 0u32;
    for probe in probes {
        match mgr.load(probe) {
            Some(td) => {
                println!("  OK   {:60} -> {}x{}", probe, td.width, td.height);
            }
            None => {
                eprintln!("  MISS {}", probe);
                failed += 1;
            }
        }
    }

    if failed > 0 {
        eprintln!("\n{failed} of {} probes failed", probes.len());
        std::process::exit(1);
    }
    println!("\nall probes resolved via CASC");
}
