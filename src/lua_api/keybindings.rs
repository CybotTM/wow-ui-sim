//! Minimal keybinding helpers during the rilua port.

pub fn init_keybindings<T>(_lua: &T) -> crate::Result<()> {
    Ok(())
}

pub fn get_binding_key<T>(
    _lua: &T,
    _action: &str,
) -> crate::Result<(Option<String>, Option<String>)> {
    Ok((None, None))
}

pub fn get_binding_action<T>(_lua: &T, _key: &str) -> crate::Result<Option<String>> {
    Ok(None)
}

pub fn get_binding_at<T>(
    _lua: &T,
    _index: i32,
) -> crate::Result<(
    Option<String>,
    Option<String>,
    Option<String>,
    Option<String>,
)> {
    Ok((None, None, None, None))
}

pub fn get_num_bindings<T>(_lua: &T) -> crate::Result<i32> {
    Ok(0)
}

pub fn set_binding<T>(_lua: &T, _key: &str, _action: Option<&str>) -> crate::Result<()> {
    Ok(())
}
