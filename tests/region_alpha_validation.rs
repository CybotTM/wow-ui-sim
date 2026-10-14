//! Ordinary numeric Region:SetAlpha contract from the 10.0.0 change notes.
#![cfg(any(feature = "client-retail", feature = "client-ptr"))]

use wow_ui_sim::lua_api::WowLuaEnv;

fn run_alpha_contract(mode: &str) {
    let env = WowLuaEnv::new().expect("fresh region-alpha environment");
    env.exec(&format!("alphaContractMode = {mode:?}"))
        .expect("select alpha contract case");
    env.exec(include_str!("region_alpha_validation/ordinary_range.lua"))
        .expect("ordinary numeric region-alpha contract");
}

#[test]
fn region_alpha_accepts_numeric_boundaries_and_interior_values() {
    run_alpha_contract("valid");
}

#[test]
fn region_alpha_rejects_out_of_range_numbers_without_mutation() {
    run_alpha_contract("invalid");
}
