//! Temporary `C_Texture` file-data lookup defaults.
//!
//! Atlas lookups are state-backed in `c_api::c_texture`. FileDataID-to-filename
//! lookup is not modeled yet, so keep the nil result as an explicit temporary
//! compatibility default outside the C API implementation.

const TEXTURE_FILE_DATA_DEFAULTS_LUA: &str = r#"
C_Texture = C_Texture or __wow_namespace()
if rawget(C_Texture, "GetFilenameFromFileDataID") == nil then
    function C_Texture.GetFilenameFromFileDataID(_fileDataID)
    end
end
"#;

pub(crate) fn apply_bootstrap(lua: &mut rilua::Lua) -> crate::Result<()> {
    lua.exec(TEXTURE_FILE_DATA_DEFAULTS_LUA)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::lua_api::WowLuaEnv;

    #[test]
    fn installs_nil_file_data_lookup_default() {
        let env = WowLuaEnv::new().expect("lua env should initialize");

        let result: bool = env
            .eval("return C_Texture.GetFilenameFromFileDataID(12345) == nil")
            .expect("file data lookup default should be callable");

        assert!(result);
    }

    #[test]
    fn preserves_existing_file_data_lookup() {
        let env = WowLuaEnv::new().expect("lua env should initialize");
        env.exec(
            r#"
            function C_Texture.GetFilenameFromFileDataID()
                return "Interface\\Icons\\INV_Misc_QuestionMark"
            end
            "#,
        )
        .expect("fixture should install existing function");

        super::apply_bootstrap(&mut env.rilua_mut()).expect("workaround should apply");

        let result: String = env
            .eval("return C_Texture.GetFilenameFromFileDataID(12345)")
            .expect("existing file data lookup should remain callable");

        assert_eq!(result, "Interface\\Icons\\INV_Misc_QuestionMark");
    }
}
