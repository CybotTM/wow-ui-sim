//! Instance identities passed to generated XML Lua as data, never source text.

use super::helpers::generate_set_point_code_with_refs;
use crate::lua_api::loader_env::GeneratedChunkArg;

#[derive(Default)]
pub(super) struct XmlLuaArgs {
    pub values: Vec<GeneratedChunkArg>,
}

impl XmlLuaArgs {
    pub fn bind_string(&mut self, value: &str) -> String {
        self.values
            .push(GeneratedChunkArg::String(value.to_string()));
        format!("(...)[{}]", self.values.len())
    }

    pub fn bind_frame_ref(&mut self, name: &str) -> String {
        if let Some(id) = name
            .strip_prefix("__frame_")
            .and_then(|suffix| suffix.parse::<u64>().ok())
        {
            self.values.push(GeneratedChunkArg::Number(id as f64));
            let index = self.values.len();
            return format!("debug.getregistry().__rilua_frame_refs[(...)[{index}]]");
        }
        let argument = self.bind_string(name);
        format!("_G[{argument}]")
    }

    pub fn anchor_code(
        &mut self,
        anchors: &crate::xml::AnchorsXml,
        target: &str,
        parent_name: &str,
    ) -> String {
        generate_set_point_code_with_refs(
            anchors,
            target,
            "parent",
            parent_name,
            "parent",
            |name| self.bind_frame_ref(name),
        )
    }
}
