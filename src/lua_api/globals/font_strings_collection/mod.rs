//! rilua RustFn equivalents of font_api, strings/mod, and c_collection_api globals.
//!
//! Each section mirrors the mlua original but targets the rilua VM:
//! - Plain `fn(&mut LuaState) -> LuaResult<u32>` (or non-capturing closures that
//!   coerce to that) for state-free helpers.
//! - `borrow_state` / `borrow_state_mut` for SimState access inside RustFns.
//! - `TableBuilder` for namespace tables (C_*).
//! - `LuaApiMut::register_function` for top-level globals.
//!
//! # TODOs
//! - `CreateFont`: return cached object on repeat calls (registry lookup).
//! - `CopyFontObject`: cycle-safe property copy.
//! - `SetFontObject`: cycle detection.
//! - `GetFontInfo`: resolve font object from name or table arg.
//! - `CreateFontFamily`: extract first member's file/height/flags.
//! - Keybinding functions: wire up to `keybindings` module on SimState.
//! - `register_rilua_item_quality_colors`: iterate `ITEM_QUALITY_COLORS_DATA`.
//! - `register_rilua_class_name_tables`: iterate `CLASS_NAMES_DATA`.
//! - `register_rilua_icon_list`: iterate `ICON_LIST_DATA`.
//! - `C_PetJournal::GetPetInfoByPetID / GetPetInfoBySpeciesID`: lookup by ID.

pub mod colors;
pub mod fonts;
pub mod mount_journal;
pub mod pet_journal;
mod standard_fonts;
pub mod toy_box;

// Re-export the public API so callers importing from this module still work.
pub use colors::{
    make_rilua_color_table, register_rilua_class_name_tables, register_rilua_color_globals,
    register_rilua_icon_list, register_rilua_item_quality_colors, register_rilua_tooltip_colors,
};
pub use fonts::{
    create_font, create_font_family, get_font_info, get_fonts, register_standard_font_objects,
};
pub use mount_journal::register_rilua_mount_journal;
pub use pet_journal::register_rilua_pet_journal;
pub use toy_box::register_rilua_toy_box;

use rilua::vm::state::LuaState;
use rilua::{LuaResult, Val};

// ── Shared global-table helper ────────────────────────────────────────────────

pub(crate) fn set_global_val(state: &mut LuaState, name: &str, value: Val) {
    let key = state.gc.intern_string(name.as_bytes());
    let global = state.global;
    if let Some(g) = state.gc.tables.get_mut(global) {
        let _ = g.raw_set(Val::Str(key), value, &state.gc.string_arena);
    }
    state.gc.barrier_back(global);
}

// ── Main entry point ──────────────────────────────────────────────────────────

/// Register all font, string-table, and collection API globals on the rilua VM.
pub fn register_all(lua: &mut rilua::Lua) -> LuaResult<()> {
    fonts::register_all(lua)?;

    colors::register_all(lua)?;
    super::keybindings::register_all(lua)?;

    pet_journal::register_rilua_pet_journal(lua)?;
    mount_journal::register_rilua_mount_journal(lua)?;
    toy_box::register_rilua_toy_box(lua)?;

    Ok(())
}
