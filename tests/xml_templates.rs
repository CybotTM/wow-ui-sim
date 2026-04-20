//! Tests for XML template registration and frame creation from XML.

use std::io::Write;

use wow_ui_sim::loader::load_addon;
use wow_ui_sim::loader::{LoadTiming, create_frame_from_xml};
use wow_ui_sim::lua_api::WowLuaEnv;
use wow_ui_sim::xml::{XmlElement, clear_templates, get_template, parse_xml, register_template};

/// Parse XML and create the first frame element via the loader.
fn create_first_frame(env: &WowLuaEnv, xml: &str, widget_type: &str) {
    let ui = parse_xml(xml).unwrap();
    match &ui.elements[0] {
        XmlElement::Frame(f) | XmlElement::Button(f) | XmlElement::EditBox(f) => {
            create_frame_from_xml(
                &env.loader_env(),
                f,
                widget_type,
                None,
                None,
                None,
                &mut LoadTiming::default(),
            )
            .unwrap();
        }
        _ => panic!("Expected frame-like element"),
    }
}

fn build_strata_buckets(env: &WowLuaEnv) -> Vec<Vec<u64>> {
    let mut state = env.state().borrow_mut();
    let _ = state.get_strata_buckets();
    state.strata_buckets.as_ref().unwrap().clone()
}

fn create_test_addon(xml: &str, addon_name: &str) -> tempfile::TempDir {
    let dir = tempfile::tempdir().unwrap();
    let toc_path = dir.path().join(format!("{addon_name}.toc"));
    let xml_path = dir.path().join(format!("{addon_name}.xml"));
    let mut toc = std::fs::File::create(&toc_path).unwrap();
    writeln!(toc, "## Title: {addon_name}").unwrap();
    writeln!(toc, "{addon_name}.xml").unwrap();
    let mut xml_file = std::fs::File::create(&xml_path).unwrap();
    write!(xml_file, "{xml}").unwrap();
    dir
}

/// Parse XML and register the first element as a template.
fn register_first_template(xml: &str, name: &str, widget_type: &str) {
    let ui = parse_xml(xml).unwrap();
    match &ui.elements[0] {
        XmlElement::Frame(f) | XmlElement::Button(f) | XmlElement::EditBox(f) => {
            register_template(name, widget_type, f.clone());
        }
        _ => panic!("Expected frame-like element"),
    }
}
#[path = "xml_templates/registry.rs"]
mod registry;

#[path = "xml_templates/basic_creation.rs"]
mod basic_creation;

#[path = "xml_templates/inline_methods.rs"]
mod inline_methods;

#[path = "xml_templates/inline_flow.rs"]
mod inline_flow;

#[path = "xml_templates/inline_advanced.rs"]
mod inline_advanced;

#[path = "xml_templates/three_slice.rs"]
mod three_slice;
