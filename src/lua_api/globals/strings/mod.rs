//! Minimal UI string registration bridge during the rilua migration.

pub mod string_data;

pub fn register_all_ui_strings<T, U>(_lua: &T, _globals: &U) -> crate::Result<()> {
    Ok(())
}
