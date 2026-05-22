pub(super) const GETGLOBAL_HELPER_LUA: &str = r#"
local function __wow_getglobal(name)
    return getglobal(name)
end
_G.__wow_panel_getglobal = __wow_getglobal
"#;
