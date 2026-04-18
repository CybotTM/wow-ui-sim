//! Error types for addon loading.

/// Error type for addon loading.
#[derive(Debug)]
pub enum LoadError {
    Io(std::io::Error),
    Toc(std::io::Error),
    Xml(crate::xml::XmlLoadError),
    Lua(String),
    /// Caller asked to load an addon whose dependency chain contains
    /// a disabled addon. Renders to exactly `"DEP_DISABLED"` so it can
    /// flow through `LoadAddOn`'s reason string unchanged. Tuple holds
    /// the disabled dependency name for diagnostics (logged but not
    /// surfaced through `Display`, matching retail's flat reason code).
    DepDisabled(String),
}

impl From<std::io::Error> for LoadError {
    fn from(e: std::io::Error) -> Self {
        LoadError::Io(e)
    }
}

impl From<crate::xml::XmlLoadError> for LoadError {
    fn from(e: crate::xml::XmlLoadError) -> Self {
        LoadError::Xml(e)
    }
}

impl std::fmt::Display for LoadError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LoadError::Io(e) => write!(f, "IO error: {}", e),
            LoadError::Toc(e) => write!(f, "TOC error: {}", e),
            LoadError::Xml(e) => write!(f, "XML error: {}", e),
            LoadError::Lua(e) => write!(f, "Lua error: {}", e),
            LoadError::DepDisabled(_) => write!(f, "DEP_DISABLED"),
        }
    }
}

impl std::error::Error for LoadError {}
