use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Lua error: {0}")]
    Lua(#[from] rilua::LuaError),

    #[error("XML parse error: {0}")]
    Xml(#[from] quick_xml::DeError),

    #[error("Widget not found: {0}")]
    WidgetNotFound(String),

    #[error("Invalid widget type: expected {expected}, got {actual}")]
    InvalidWidgetType { expected: String, actual: String },

    #[error("World of Warcraft installation not found")]
    WowInstallNotFound,

    #[error("Blizzard UI sync incomplete: {missing} of {total} files missing (last error: {last_error})")]
    BlizzardUiPartial {
        missing: usize,
        total: usize,
        last_error: String,
    },

    #[error("{0}")]
    Other(String),
}

pub type Result<T> = std::result::Result<T, Error>;
