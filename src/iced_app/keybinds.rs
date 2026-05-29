//! Keyboard key mapping from iced keys to WoW key names.

use iced::keyboard::key::Named;

/// Convert an iced keyboard key to a WoW key name string.
pub(super) fn iced_key_to_wow(
    key: &iced::keyboard::Key,
    modifiers: iced::keyboard::Modifiers,
) -> Option<String> {
    use iced::keyboard::Key;
    let key_name = match key {
        Key::Named(named) => iced_named_key_to_wow(named),
        Key::Character(c) => return iced_character_key_to_wow(c, modifiers),
        _ => None,
    }?;
    Some(apply_wow_modifiers(key_name, modifiers))
}

/// Convert an iced physical key fallback to a WoW key name string.
pub(super) fn iced_physical_key_to_wow(
    physical_key: &iced::keyboard::key::Physical,
    modifiers: iced::keyboard::Modifiers,
) -> Option<String> {
    let iced::keyboard::key::Physical::Code(code) = physical_key else {
        return None;
    };
    let key_name = iced_physical_code_to_wow(*code)?;
    Some(apply_wow_modifiers(key_name.to_string(), modifiers))
}

fn iced_character_key_to_wow(key: &str, modifiers: iced::keyboard::Modifiers) -> Option<String> {
    if let Some(control_key) = crate::key_names::ascii_control_key_to_letter(key) {
        return Some(apply_wow_modifiers(
            control_key.to_string(),
            modifiers | iced::keyboard::Modifiers::CTRL,
        ));
    }
    Some(apply_wow_modifiers(key.to_uppercase(), modifiers))
}

fn apply_wow_modifiers(key_name: String, modifiers: iced::keyboard::Modifiers) -> String {
    let mut parts = Vec::new();
    if modifiers.control() {
        parts.push("CTRL");
    }
    if modifiers.alt() {
        parts.push("ALT");
    }
    if modifiers.shift() {
        parts.push("SHIFT");
    }
    parts.push(&key_name);
    parts.join("-")
}

fn iced_physical_code_to_wow(code: iced::keyboard::key::Code) -> Option<&'static str> {
    PHYSICAL_ALPHA_KEYS
        .iter()
        .find_map(|(candidate, wow_key)| (*candidate == code).then_some(*wow_key))
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

const PHYSICAL_ALPHA_KEYS: &[(iced::keyboard::key::Code, &str)] = &[
    (iced::keyboard::key::Code::KeyA, "A"),
    (iced::keyboard::key::Code::KeyB, "B"),
    (iced::keyboard::key::Code::KeyC, "C"),
    (iced::keyboard::key::Code::KeyD, "D"),
    (iced::keyboard::key::Code::KeyE, "E"),
    (iced::keyboard::key::Code::KeyF, "F"),
    (iced::keyboard::key::Code::KeyG, "G"),
    (iced::keyboard::key::Code::KeyH, "H"),
    (iced::keyboard::key::Code::KeyI, "I"),
    (iced::keyboard::key::Code::KeyJ, "J"),
    (iced::keyboard::key::Code::KeyK, "K"),
    (iced::keyboard::key::Code::KeyL, "L"),
    (iced::keyboard::key::Code::KeyM, "M"),
    (iced::keyboard::key::Code::KeyN, "N"),
    (iced::keyboard::key::Code::KeyO, "O"),
    (iced::keyboard::key::Code::KeyP, "P"),
    (iced::keyboard::key::Code::KeyQ, "Q"),
    (iced::keyboard::key::Code::KeyR, "R"),
    (iced::keyboard::key::Code::KeyS, "S"),
    (iced::keyboard::key::Code::KeyT, "T"),
    (iced::keyboard::key::Code::KeyU, "U"),
    (iced::keyboard::key::Code::KeyV, "V"),
    (iced::keyboard::key::Code::KeyW, "W"),
    (iced::keyboard::key::Code::KeyX, "X"),
    (iced::keyboard::key::Code::KeyY, "Y"),
    (iced::keyboard::key::Code::KeyZ, "Z"),
];

#[cfg(test)]
mod tests {
    use super::*;
    use iced::keyboard::Key;
    use iced::keyboard::key::{Code, Physical};

    #[test]
    fn maps_named_keys_to_wow_names() {
        assert_eq!(
            iced_key_to_wow(
                &Key::Named(Named::Escape),
                iced::keyboard::Modifiers::empty()
            ),
            Some("ESCAPE".to_string())
        );
        assert_eq!(
            iced_key_to_wow(&Key::Named(Named::F10), iced::keyboard::Modifiers::empty()),
            Some("F10".to_string())
        );
        assert_eq!(
            iced_key_to_wow(
                &Key::Named(Named::ArrowLeft),
                iced::keyboard::Modifiers::empty()
            ),
            Some("LEFT".to_string())
        );
    }

    #[test]
    fn maps_character_keys_to_uppercase() {
        assert_eq!(
            iced_key_to_wow(
                &Key::Character("b".into()),
                iced::keyboard::Modifiers::empty()
            ),
            Some("B".to_string())
        );
    }

    #[test]
    fn prefixes_control_modifier_for_wow_key_name() {
        assert_eq!(
            iced_key_to_wow(&Key::Character("q".into()), iced::keyboard::Modifiers::CTRL),
            Some("CTRL-Q".to_string())
        );
    }

    #[test]
    fn maps_control_character_for_ctrl_q_to_wow_key_name() {
        assert_eq!(
            iced_key_to_wow(
                &Key::Character("\u{11}".into()),
                iced::keyboard::Modifiers::CTRL
            ),
            Some("CTRL-Q".to_string())
        );
    }

    #[test]
    fn maps_control_character_without_modifier_to_control_key_name() {
        assert_eq!(
            iced_key_to_wow(
                &Key::Character("\u{11}".into()),
                iced::keyboard::Modifiers::empty()
            ),
            Some("CTRL-Q".to_string())
        );
    }

    #[test]
    fn maps_physical_key_q_fallback_to_control_key_name() {
        assert_eq!(
            iced_physical_key_to_wow(&Physical::Code(Code::KeyQ), iced::keyboard::Modifiers::CTRL),
            Some("CTRL-Q".to_string())
        );
    }

    #[test]
    fn returns_none_for_unmapped_named_keys() {
        assert_eq!(
            iced_key_to_wow(
                &Key::Named(Named::Shift),
                iced::keyboard::Modifiers::empty()
            ),
            None
        );
    }
}
