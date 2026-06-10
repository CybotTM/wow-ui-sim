use std::io;
use std::path::Path;

use crate::lua_api::sim_substates::Keybindings;
use crate::saved_variables::WtfConfig;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BindingCacheEntry {
    pub key: String,
    pub action: Option<String>,
}

pub fn load_wtf_account_bindings(config: &WtfConfig) -> io::Result<Vec<BindingCacheEntry>> {
    read_bindings_cache(&config.account_bindings_cache_file())
}

pub fn read_bindings_cache(path: &Path) -> io::Result<Vec<BindingCacheEntry>> {
    let text = std::fs::read_to_string(path)?;
    Ok(parse_bindings_cache(&text))
}

pub fn parse_bindings_cache(text: &str) -> Vec<BindingCacheEntry> {
    text.lines().filter_map(parse_binding_line).collect()
}

pub fn apply_bindings_cache(bindings: &mut Keybindings, entries: &[BindingCacheEntry]) {
    for entry in entries {
        let action = entry.action.as_deref().unwrap_or_default();
        bindings.set(&entry.key, action);
    }
}

fn parse_binding_line(line: &str) -> Option<BindingCacheEntry> {
    let mut parts = line.split_whitespace();
    let command = parts.next()?;
    if command != "bind" {
        return None;
    }
    let key = parts.next()?.to_string();
    let action = parts.next()?.to_string();
    let action = (action != "NONE").then_some(action);
    Some(BindingCacheEntry { key, action })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_bindings_cache_reads_bound_and_unbound_keys() {
        let entries = parse_bindings_cache(
            "bind F1 TARGETSELF\n\
             bind 1 NONE\n\
             ignored F2 TARGETPARTYMEMBER1\n",
        );

        assert_eq!(
            entries,
            vec![
                BindingCacheEntry {
                    key: "F1".to_string(),
                    action: Some("TARGETSELF".to_string()),
                },
                BindingCacheEntry {
                    key: "1".to_string(),
                    action: None,
                },
            ]
        );
    }

    #[test]
    fn apply_bindings_cache_imports_none_as_explicit_unbind() {
        let mut keybindings = Keybindings::default();
        let entries = parse_bindings_cache("bind F1 NONE\nbind F2 TARGETPARTYMEMBER1\n");

        apply_bindings_cache(&mut keybindings, &entries);

        assert!(keybindings.shadows_default_key("F1"));
        assert_eq!(keybindings.action_for_key("F1"), "");
        assert_eq!(keybindings.action_for_key("F2"), "TARGETPARTYMEMBER1");
    }
}
