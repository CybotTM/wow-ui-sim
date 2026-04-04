//! Keyboard key mapping from iced keys to WoW key names.

use iced::keyboard::key::Named;

/// Convert an iced keyboard key to a WoW key name string.
pub(super) fn iced_key_to_wow(key: &iced::keyboard::Key) -> Option<String> {
    use iced::keyboard::Key;
    match key {
        Key::Named(named) => iced_named_key_to_wow(named),
        Key::Character(c) => Some(c.to_uppercase()),
        _ => None,
    }
}

/// Convert an iced named key to a WoW key name.
fn iced_named_key_to_wow(named: &iced::keyboard::key::Named) -> Option<String> {
    WOW_NAMED_KEYS
        .iter()
        .find_map(|(candidate, wow_key)| (*candidate == *named).then_some(*wow_key))
        .map(str::to_string)
}

const WOW_NAMED_KEYS: &[(Named, &str)] = &[
    (Named::Escape, "ESCAPE"),
    (Named::Enter, "ENTER"),
    (Named::Tab, "TAB"),
    (Named::Space, "SPACE"),
    (Named::Backspace, "BACKSPACE"),
    (Named::Delete, "DELETE"),
    (Named::ArrowUp, "UP"),
    (Named::ArrowDown, "DOWN"),
    (Named::ArrowLeft, "LEFT"),
    (Named::ArrowRight, "RIGHT"),
    (Named::Home, "HOME"),
    (Named::End, "END"),
    (Named::PageUp, "PAGEUP"),
    (Named::PageDown, "PAGEDOWN"),
    (Named::Insert, "INSERT"),
    (Named::F1, "F1"),
    (Named::F2, "F2"),
    (Named::F3, "F3"),
    (Named::F4, "F4"),
    (Named::F5, "F5"),
    (Named::F6, "F6"),
    (Named::F7, "F7"),
    (Named::F8, "F8"),
    (Named::F9, "F9"),
    (Named::F10, "F10"),
    (Named::F11, "F11"),
    (Named::F12, "F12"),
];

#[cfg(test)]
mod tests {
    use super::*;
    use iced::keyboard::Key;

    #[test]
    fn maps_named_keys_to_wow_names() {
        assert_eq!(
            iced_key_to_wow(&Key::Named(Named::Escape)),
            Some("ESCAPE".to_string())
        );
        assert_eq!(
            iced_key_to_wow(&Key::Named(Named::F10)),
            Some("F10".to_string())
        );
        assert_eq!(
            iced_key_to_wow(&Key::Named(Named::ArrowLeft)),
            Some("LEFT".to_string())
        );
    }

    #[test]
    fn maps_character_keys_to_uppercase() {
        assert_eq!(
            iced_key_to_wow(&Key::Character("b".into())),
            Some("B".to_string())
        );
    }

    #[test]
    fn returns_none_for_unmapped_named_keys() {
        assert_eq!(iced_key_to_wow(&Key::Named(Named::Shift)), None);
    }
}
