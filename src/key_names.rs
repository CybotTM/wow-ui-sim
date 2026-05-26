pub(crate) fn ascii_control_key_to_letter(key: &str) -> Option<char> {
    let mut chars = key.chars();
    let key = chars.next()?;
    if chars.next().is_some() || !key.is_ascii_control() {
        return None;
    }

    let code = key as u32;
    (1..=26)
        .contains(&code)
        .then(|| char::from_u32((b'A' as u32) + code - 1))
        .flatten()
}
