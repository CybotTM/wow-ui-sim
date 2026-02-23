//! Tests for Blizzard addon load order.
//!
//! Verifies that transitive dependencies are loaded before the addons that
//! need them, even when the dependency chain crosses base UI addon boundaries.

use std::path::PathBuf;
use wow_ui_sim::loader::{discover_blizzard_addons, load_addon};
use wow_ui_sim::lua_api::WowLuaEnv;

fn blizzard_ui_dir() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("Interface/BlizzardUI")
}

/// Blizzard_ObjectAPI (which defines ItemMixin) must load before Blizzard_FrameXML
/// (which uses ItemMixin in EventToastManager.lua:669).
#[test]
fn test_object_api_loads_before_frame_xml() {
    let ui = blizzard_ui_dir();
    let addons = discover_blizzard_addons(&ui);

    let names: Vec<&str> = addons.iter().map(|(n, _)| n.as_str()).collect();

    let obj_api_pos = names.iter().position(|&n| n == "Blizzard_ObjectAPI");
    let frame_xml_pos = names.iter().position(|&n| n == "Blizzard_FrameXML");

    assert!(
        obj_api_pos.is_some(),
        "Blizzard_ObjectAPI should be in the addon list"
    );
    assert!(
        frame_xml_pos.is_some(),
        "Blizzard_FrameXML should be in the addon list"
    );

    assert!(
        obj_api_pos.unwrap() < frame_xml_pos.unwrap(),
        "Blizzard_ObjectAPI (pos {}) must load before Blizzard_FrameXML (pos {})\n\
         Load order: {:?}",
        obj_api_pos.unwrap(),
        frame_xml_pos.unwrap(),
        &names[..std::cmp::min(names.len(), 10)],
    );
}

/// ItemMixin (from Blizzard_ObjectAPI) must be defined when Blizzard_FrameXML loads.
/// EventToastManager.lua:669 does `CreateFromMixins(..., ItemMixin)` at file scope.
#[test]
fn test_item_mixin_available_for_event_toast_manager() {
    let env = WowLuaEnv::new().expect("Failed to create Lua environment");
    let ui = blizzard_ui_dir();
    let addons = discover_blizzard_addons(&ui);

    for (name, toc_path) in &addons {
        load_addon(&env.loader_env(), toc_path).ok();
        if name == "Blizzard_FrameXML" {
            break;
        }
    }

    let has_item_mixin: bool = env
        .eval("return type(ItemMixin) == 'table'")
        .unwrap_or(false);
    assert!(
        has_item_mixin,
        "ItemMixin should be defined before Blizzard_FrameXML finishes loading"
    );
}
