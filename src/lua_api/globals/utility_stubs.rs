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
    local function MakeActiveEnumerator(active)
        local i = 0
        return function()
            i = i + 1
            return active[i]
        end
    end

    local function ReleaseTrackedFrame(pool, frame, resetFunc)
        if not frame then
            return
        end

        for i, activeFrame in ipairs(pool.active) do
            if activeFrame == frame then
                table.remove(pool.active, i)
                break
            end
        end

        if resetFunc then
            resetFunc(pool, frame)
        else
            frame:Hide()
            frame:ClearAllPoints()
        end
    end

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

    function CreateFramePool(frameType, parent, template, resetFunc)
        local pool = {}
        pool.active = {}

        function pool:Acquire()
            local frame = CreateFrame(frameType or "Frame", nil, parent, template)
            table.insert(self.active, frame)
            return frame
        end

        function pool:Release(frame)
            ReleaseTrackedFrame(self, frame, resetFunc)
        end

        function pool:ReleaseAll()
            while #self.active > 0 do
                self:Release(self.active[#self.active])
            end
        end

        function pool:IsActive(frame)
            for _, activeFrame in ipairs(self.active) do
                if activeFrame == frame then
                    return true
                end
            end
            return false
        end

        function pool:EnumerateActive()
            return MakeActiveEnumerator(self.active)
        end

        return pool
    end

    function CreateFramePoolCollection()
        local collection = {}
        collection.pools = {}

        local function poolKey(template)
            return template or false
        end

        function collection:CreatePool(frameType, parent, template, resetFunc)
            local pool = CreateFramePool(frameType, parent, template, resetFunc)
            self.pools[poolKey(template)] = pool
            return pool
        end

        function collection:GetPool(template)
            return self.pools[poolKey(template)]
        end

        function collection:Acquire(template)
            local pool = self:GetPool(template)
            if not pool then
                error("CreateFramePoolCollection: missing pool for template")
            end
            return pool:Acquire()
        end

        function collection:Release(frame)
            for _, pool in pairs(self.pools) do
                if pool:IsActive(frame) then
                    pool:Release(frame)
                    return
                end
            end
        end

        function collection:ReleaseAll()
            for _, pool in pairs(self.pools) do
                pool:ReleaseAll()
            end
        end

        function collection:IsActive(frame)
            for _, pool in pairs(self.pools) do
                if pool:IsActive(frame) then
                    return true
                end
            end
            return false
        end

        function collection:EnumerateActive()
            local active = {}
            for _, pool in pairs(self.pools) do
                for frame in pool:EnumerateActive() do
                    table.insert(active, frame)
                end
            end
            return MakeActiveEnumerator(active)
        end

        return collection
    end

    function CreateTexturePool(parent, layer, subLayer, template)
        local pool = {}
        pool.active = {}
        function pool:Acquire()
            local parent_frame = type(parent) == "string" and _G[parent] or parent
            if not parent_frame then parent_frame = UIParent end
            local tex = parent_frame:CreateTexture(nil, layer or "ARTWORK")
            table.insert(self.active, tex)
            return tex
        end
        function pool:Release(texture)
            if texture then texture:SetTexture(nil) end
            for i, t in ipairs(self.active) do
                if t == texture then
                    table.remove(self.active, i)
                    break
                end
            end
        end
        function pool:ReleaseAll()
            while #self.active > 0 do
                self:Release(self.active[#self.active])
            end
        end
        function pool:EnumerateActive()
            local i = 0
            return function()
                i = i + 1
                return self.active[i]
            end
        end
        return pool
    end
"##;

/// Register ScrollBox factory and utility stubs.
pub fn register_scrollbox_and_utility_stubs(lua: &Lua) -> Result<()> {
    lua.load(SCROLLBOX_GLOBALS_LUA).exec()?;
    lua.load(UTILITY_STUBS_LUA).exec()
}
