//! Who query routing shared by the legacy and C_FriendList adapters.
//! Explicit response input, not a server, query parser, or native timing model.
#[derive(Debug, Default)]
pub struct WhoRouting {
    pub to_ui: bool,
    pub last_query: Option<String>,
    pub pending: Vec<String>,
    pub ui_results: Vec<String>,
}
