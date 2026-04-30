//! Wrapper binary that pulls every per-aspect Blizzard_AccountSaveUI
//! test file under `tests/blizzard_ui/blizzard_accountsaveui/` into a
//! single `cargo test --test ...` target. Cargo only auto-discovers
//! `tests/*.rs`, so the nested `load.rs` / `surface_*.rs` / `behavior_*.rs`
//! files declared by the per-addon plan template need a flat re-export here
//! to be reachable.

mod common;

#[path = "blizzard_ui/blizzard_accountsaveui/load.rs"]
mod load;

#[path = "blizzard_ui/blizzard_accountsaveui/surface_globals.rs"]
mod surface_globals;
