//! XML facade; profile template selection stays above the parser/registry crate.
pub use sim_xml::*;
mod profile_templates;

fn template_policy() -> TemplatePolicy {
    TemplatePolicy {
        retain_wrath_declarations: crate::client_profile::ACTIVE
            == crate::client_profile::ClientProfile::Wrath,
    }
}

pub fn register_template(name: &str, widget_type: &str, frame: FrameXml) {
    sim_xml::register_template(name, widget_type, frame, template_policy());
}

pub fn register_template_with_local_source(
    name: &str,
    widget_type: &str,
    frame: FrameXml,
    local_source: Option<rilua::Val>,
    declaration_taint: Option<String>,
) {
    sim_xml::register_template_with_local_source(
        name,
        widget_type,
        frame,
        local_source,
        declaration_taint,
        template_policy(),
    );
}

pub fn register_intrinsic_templates() {
    sim_xml::register_intrinsic_templates(template_policy());
    profile_templates::register_all();
}

#[cfg(all(test, feature = "profile-retail"))]
mod cached_ui_tests;
#[cfg(test)]
mod template_tests;
