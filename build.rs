use std::path::PathBuf;

#[cfg(feature = "native-duration-formatting")]
#[path = "build/intl_native.rs"]
mod intl_native;
#[path = "build/locked_rilua.rs"]
mod locked_rilua;

fn main() {
    publish_rilua_compiler_revision();
    #[cfg(feature = "native-duration-formatting")]
    intl_native::build();

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

fn publish_rilua_compiler_revision() {
    let manifest_dir = PathBuf::from(std::env::var("CARGO_MANIFEST_DIR").expect("manifest dir"));
    let path = manifest_dir.join("Cargo.lock");
    println!("cargo:rerun-if-changed={}", path.display());
    println!("cargo:rerun-if-changed=build/locked_rilua.rs");
    let lock = std::fs::read_to_string(&path)
        .unwrap_or_else(|error| panic!("read {} for compiler identity: {error}", path.display()));
    let revision = locked_rilua::parse_locked_rilua_revision(&lock)
        .unwrap_or_else(|error| panic!("{} compiler identity: {error}", path.display()));
    println!("cargo:rustc-env=WOW_SIM_RILUA_COMPILER_REVISION={revision}");
}
