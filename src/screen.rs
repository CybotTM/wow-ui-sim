use clap::ValueEnum;

/// High-level UI surface to boot.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, ValueEnum)]
pub enum ScreenKind {
    /// Normal in-game UI.
    #[default]
    Game,
    /// Glue login screen.
    Login,
    /// Glue character selection screen.
    #[value(
        alias = "character-select",
        alias = "charselect",
        alias = "character_select"
    )]
    CharacterSelect,
}

impl ScreenKind {
    pub const fn is_glue(self) -> bool {
        matches!(self, Self::Login | Self::CharacterSelect)
    }

    pub const fn glue_screen_name(self) -> Option<&'static str> {
        match self {
            Self::Game => None,
            Self::Login => Some("login"),
            Self::CharacterSelect => Some("charselect"),
        }
    }

    /// Return the values expected by `C_Login.GetState()`.
    pub const fn login_state(self) -> (i32, bool, i32, bool) {
        const LE_AURORA_STATE_NONE: i32 = 1;
        match self {
            Self::Game => (LE_AURORA_STATE_NONE, true, 0, false),
            Self::Login => (LE_AURORA_STATE_NONE, false, 0, false),
            Self::CharacterSelect => (LE_AURORA_STATE_NONE, true, 0, false),
        }
    }
}
