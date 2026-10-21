//! Wrath file failures use retained loader warnings, not the Lua error sink.

use std::path::{Component, Path};

use super::addon::AddonContext;

pub(super) fn uses_wrath_file_diagnostics() -> bool {
    crate::client_profile::ACTIVE == crate::client_profile::ClientProfile::Wrath
}

pub(super) fn is_recorded_xml_failure(error: &super::LoadError) -> bool {
    uses_wrath_file_diagnostics() && matches!(error, super::LoadError::Xml(_))
}

pub(super) fn record_file_diagnostic(
    warnings: &mut Vec<String>,
    ctx: &AddonContext<'_>,
    path: &Path,
    message: &str,
) {
    let virtual_path = client_virtual_path(path, ctx);
    warnings.push(format!("{message} {virtual_path}"));
}

fn client_virtual_path(path: &Path, ctx: &AddonContext<'_>) -> String {
    // INFERRED mapping: host paths are tracked, native virtual paths are not.
    // Keep the addon-root-relative spelling, including nonexistent children.
    let root_parts: Vec<_> = ctx.addon_root.components().collect();
    let path_parts: Vec<_> = path.components().collect();
    let shared = root_parts
        .iter()
        .zip(&path_parts)
        .take_while(|(a, b)| a == b)
        .count();
    let mut components = if ctx.name == "FrameXML" {
        vec!["Interface".to_string(), "FrameXML".to_string()]
    } else {
        vec![
            "Interface".to_string(),
            "AddOns".to_string(),
            ctx.name.to_string(),
        ]
    };
    components.truncate(components.len().saturating_sub(root_parts.len() - shared));
    for component in &path_parts[shared..] {
        match component {
            Component::ParentDir => {
                components.pop();
            }
            Component::Normal(name) => components.push(name.to_string_lossy().into_owned()),
            _ => {}
        }
    }
    components.join("\\")
}
