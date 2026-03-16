//! ScrollBox and utility factory Lua stubs.

use mlua::{Lua, Result};

const SCROLLBOX_GLOBALS_LUA: &str = r##"
    function CreateScrollBoxPadding(top, bottom, left, right, spacing)
        return {
            top = top or 0, bottom = bottom or 0, left = left or 0,
            right = right or 0, spacing = spacing or 0,
            GetSpacing = function(self) return self.spacing end,
            GetLeft = function(self) return self.left end,
            GetRight = function(self) return self.right end,
            GetTop = function(self) return self.top end,
            GetBottom = function(self) return self.bottom end,
        }
    end

    function CreateScrollBoxLinearView(top, bottom, left, right, spacing)
        return CreateAndInitFromMixin(ScrollBoxLinearViewMixin, top, bottom, left, right, spacing)
    end
"##;

const UTILITY_STUBS_LUA: &str = r##"
    function GetFinalNameFromTextureKit(kit)
        if not kit or not kit.name then return "" end
        return kit.name
    end

    NineSliceUtil = {
        GetBorderSizes = function(self, border)
            if not border then return 0, 0, 0, 0 end
            return border.top or 0, border.bottom or 0, border.left or 0, border.right or 0
        end,
        DisableSharpening = function() end,
    }

    function CreateFramePool(frameType, parent, template)
        local pool = {}
        function pool:Acquire()
            return CreateFrame(frameType or "Frame", nil, parent, template)
        end
        function pool:Release(frame)
            if frame then frame:Hide(); frame:ClearAllPoints() end
        end
        function pool:EnumerateActive() return function() end end
        return pool
    end

    function CreateTexturePool(parent, template)
        local pool = {}
        function pool:Acquire()
            local parent_frame = type(parent) == "string" and _G[parent] or parent
            if not parent_frame then parent_frame = UIParent end
            return parent_frame:CreateTexture(nil, template)
        end
        function pool:Release(texture)
            if texture then texture:SetTexture(nil) end
        end
        function pool:EnumerateActive() return function() end end
        return pool
    end
"##;

/// Register ScrollBox factory and utility stubs.
pub fn register_scrollbox_and_utility_stubs(lua: &Lua) -> Result<()> {
    lua.load(SCROLLBOX_GLOBALS_LUA).exec()?;
    lua.load(UTILITY_STUBS_LUA).exec()
}
