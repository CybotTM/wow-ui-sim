//! Temporary `TransmogUtil` location helpers.
//!
//! These are table-shaped compatibility defaults for Blizzard collection and
//! wardrobe code. Real appearance/illusion state remains in the C_Transmog*
//! surfaces; this module only supplies the small Lua utility object shape.

const TRANSMOG_UTIL_DEFAULTS_LUA: &str = r#"
local function __wow_make_transmog_util_location(slotName, slotID, transmogType, modification)
  local location = {
    slotName = tostring(slotName or ""),
    slotID = tonumber(slotID) or 0,
    transmogType = tonumber(transmogType) or 0,
    modification = tonumber(modification) or 0,
  }

  function location:IsAppearance()
    return true
  end

  function location:IsIllusion()
    return false
  end

  function location:IsEitherHand()
    return self.slotName == "MAINHANDSLOT" or self.slotName == "SECONDHANDSLOT"
  end

  function location:IsSecondary()
    return self.slotName == "SECONDHANDSLOT"
  end

  function location:IsMainHand()
    return self.slotName == "MAINHANDSLOT"
  end

  function location:GetSlotName()
    return self.slotName
  end

  function location:IsEqual(other)
    return type(other) == "table"
      and self.slotName == other.slotName
      and self.slotID == other.slotID
      and self.transmogType == other.transmogType
      and self.modification == other.modification
  end

  return location
end

TransmogUtil = type(TransmogUtil) == "table" and TransmogUtil or {}
if rawget(TransmogUtil, "GetTransmogLocation") == nil then
  function TransmogUtil.GetTransmogLocation(slotName, transmogType, modification)
    return __wow_make_transmog_util_location(slotName, 0, transmogType, modification)
  end
end
if rawget(TransmogUtil, "CreateTransmogLocation") == nil then
  function TransmogUtil.CreateTransmogLocation(slotID, transmogType, modification)
    return __wow_make_transmog_util_location("", slotID, transmogType, modification)
  end
end
if rawget(TransmogUtil, "GetBestItemModifiedAppearanceID") == nil then
  function TransmogUtil.GetBestItemModifiedAppearanceID(_itemID)
    return nil
  end
end
"#;

pub(crate) fn apply_bootstrap(lua: &mut rilua::Lua) -> crate::Result<()> {
    lua.exec(TRANSMOG_UTIL_DEFAULTS_LUA)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::lua_api::WowLuaEnv;

    #[test]
    fn installs_transmog_util_location_defaults() {
        let env = WowLuaEnv::new().expect("lua env should initialize");
        {
            let mut lua = env.lua.borrow_mut();
            super::apply_bootstrap(&mut lua).expect("transmog defaults should apply");
        }

        let result: String = env
            .eval(
                r#"
                local head = TransmogUtil.GetTransmogLocation("HEADSLOT", 0, false)
                local head2 = TransmogUtil.GetTransmogLocation("HEADSLOT", 0, false)
                local mainHand = TransmogUtil.GetTransmogLocation("MAINHANDSLOT", 0, false)
                local created = TransmogUtil.CreateTransmogLocation(1, 0, 0)
                if head:GetSlotName() ~= "HEADSLOT" then return "slot_name" end
                if not head:IsAppearance() or head:IsIllusion() then return "kind" end
                if mainHand:IsEitherHand() ~= true or mainHand:IsMainHand() ~= true then return "hand" end
                if not head:IsEqual(head2) then return "equal" end
                if created.slotID ~= 1 then return "created_slot" end
                if TransmogUtil.GetBestItemModifiedAppearanceID(6948) ~= nil then return "appearance_id" end
                return "ok"
                "#,
            )
            .expect("transmog defaults should be callable");

        assert_eq!(result, "ok");
    }
}
