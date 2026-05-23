//! Temporary fallbacks for small SharedXML utility globals.
//!
//! Blizzard normally supplies these from SharedXMLBase or Settings definition
//! files. Keep shallow compatibility defaults here for isolated addon closure
//! tests that do not load those providers.

const SHARED_XML_UTILITY_DEFAULTS_LUA: &str = r#"
if type(CreateAnchor) ~= "function" then
  function CreateAnchor(point, relativeTo, relativePoint, x, y)
    return {
      point = point,
      relativeTo = relativeTo,
      relativePoint = relativePoint or point,
      x = x or 0,
      y = y or 0,
    }
  end
end

if type(GetFinalNameFromTextureKit) ~= "function" then
  function GetFinalNameFromTextureKit(formatString, textureKit)
    if type(formatString) ~= "string" then
      return nil
    end
    if textureKit == nil or textureKit == "" then
      return (formatString:gsub("%%s_?", ""):gsub("_$", ""))
    end
    return formatString:gsub("%%s", textureKit)
  end
end

if type(SetClampedTextureRotation) ~= "function" then
  function SetClampedTextureRotation(texture, rotation)
    if texture and type(texture.SetRotation) == "function" then
      texture:SetRotation(rotation or 0)
    end
  end
end

if type(CopyValuesAsKeys) ~= "function" then
  function CopyValuesAsKeys(values)
    local result = {}
    if type(values) ~= "table" then
      return result
    end
    for _, value in pairs(values) do
      result[value] = true
    end
    return result
  end
end

if type(GetMicroIconForRole) ~= "function" then
  function GetMicroIconForRole(role)
    if type(role) ~= "string" then
      return "roleicon"
    end
    return "roleicon-" .. role:lower()
  end
end

if type(PingSystemInitializer) ~= "function" then
  function PingSystemInitializer(_category)
  end
end
"#;

pub(crate) fn apply_bootstrap(lua: &mut rilua::Lua) -> crate::Result<()> {
    lua.exec(SHARED_XML_UTILITY_DEFAULTS_LUA)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::lua_api::WowLuaEnv;

    #[test]
    fn installs_shared_xml_utility_defaults() {
        let env = WowLuaEnv::new().expect("lua env should initialize");
        {
            let mut lua = env.lua.borrow_mut();
            super::apply_bootstrap(&mut lua).expect("shared XML utility defaults should apply");
        }

        let result: String = env
            .eval(
                r#"
                local anchor = CreateAnchor("TOPLEFT", UIParent, nil, 10, -20)
                if anchor.point ~= "TOPLEFT" or anchor.relativePoint ~= "TOPLEFT" then return "anchor" end
                if GetFinalNameFromTextureKit("%s-topper", "ardenweald") ~= "ardenweald-topper" then return "texture_kit" end
                if GetFinalNameFromTextureKit("%s_topper", "") ~= "topper" then return "texture_kit_empty" end
                local keys = CopyValuesAsKeys({ "a", "b" })
                if keys.a ~= true or keys.b ~= true then return "copy_values" end
                if GetMicroIconForRole("TANK") ~= "roleicon-tank" then return "role_icon" end
                local rotated = 0
                SetClampedTextureRotation({ SetRotation = function(_, value) rotated = value end }, 90)
                if rotated ~= 90 then return "rotation" end
                PingSystemInitializer({})
                return "ok"
                "#,
            )
            .expect("shared XML utility defaults should be callable");

        assert_eq!(result, "ok");
    }

    #[test]
    fn preserves_existing_shared_xml_utility_members() {
        let env = WowLuaEnv::new().expect("lua env should initialize");
        env.exec(
            r#"
            function CreateAnchor() return "existing_anchor" end
            function GetFinalNameFromTextureKit() return "existing_texture" end
            function CopyValuesAsKeys() return "existing_copy" end
            "#,
        )
        .expect("fixture should install existing utility globals");

        {
            let mut lua = env.lua.borrow_mut();
            super::apply_bootstrap(&mut lua).expect("shared XML utility defaults should apply");
        }

        let result: String = env
            .eval(
                r#"
                if CreateAnchor() ~= "existing_anchor" then return "overwrote_anchor" end
                if GetFinalNameFromTextureKit() ~= "existing_texture" then return "overwrote_texture" end
                if CopyValuesAsKeys() ~= "existing_copy" then return "overwrote_copy" end
                if type(SetClampedTextureRotation) ~= "function" then return "missing_rotation" end
                if type(GetMicroIconForRole) ~= "function" then return "missing_role_icon" end
                if type(PingSystemInitializer) ~= "function" then return "missing_ping" end
                return "ok"
                "#,
            )
            .expect("shared XML utility preservation probe should run");

        assert_eq!(result, "ok");
    }
}
