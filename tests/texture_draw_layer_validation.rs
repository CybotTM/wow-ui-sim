//! Ordinary CreateTexture draw-layer names on Retail/PTR.
#![cfg(any(feature = "client-retail", feature = "client-ptr"))]

use wow_ui_sim::lua_api::WowLuaEnv;

fn run_draw_layer_contract(mode: &str) {
    let env = WowLuaEnv::new().expect("fresh texture draw-layer environment");
    env.exec(&format!("drawLayerContractMode = {mode:?}"))
        .expect("select texture draw-layer contract case");
    env.exec(include_str!(
        "texture_draw_layer_validation/ordinary_names.lua"
    ))
    .expect("ordinary texture draw-layer contract");
}

#[test]
fn create_texture_accepts_all_five_draw_layers() {
    run_draw_layer_contract("valid");
}

#[test]
fn create_texture_rejects_invalid_draw_layer_without_adding_region() {
    run_draw_layer_contract("invalid");
}
