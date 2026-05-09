fn main() {
    println!("cargo:rerun-if-changed=installer/wow-sim.ico");
    println!("cargo:rerun-if-changed=build.rs");

    let target_os = std::env::var("CARGO_CFG_TARGET_OS").unwrap_or_default();
    if target_os != "windows" {
        return;
    }

    let mut res = winresource::WindowsResource::new();
    res.set_icon("installer/wow-sim.ico");
    if let Err(error) = res.compile() {
        // On hosts without a usable resource compiler the binary should still
        // link; surface the failure as a cargo warning instead of aborting.
        println!("cargo:warning=failed to embed Windows icon resource: {error}");
    }
}
