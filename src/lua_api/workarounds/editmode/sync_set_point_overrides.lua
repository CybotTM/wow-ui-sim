
        if not EditModeManagerFrame then return end
        local emm = EditModeManagerFrame
        if not emm.registeredSystemFrames then return end

        local function frameFields(frame)
            local ok, env = pcall(debug.getfenv, frame)
            if ok and env then
                return env[1]
            end
        end

        local function callRustMethod(self, methodName, ...)
            local fields = frameFields(self)
            if not fields then return false end

            local override = rawget(fields, methodName)
            rawset(fields, methodName, nil)
            local method = self[methodName]
            local ok, err
            if type(method) == "function" then
                ok, err = pcall(method, self, ...)
            else
                ok, err = false, methodName .. " is not available"
            end
            rawset(fields, methodName, override)
            return ok, err
        end

        local function syncRustAnchorFromLuaPoint(self)
            local point, relativeTo, relativePoint, offsetX, offsetY = self:GetPoint(1)
            if not point then return end
            return callRustMethod(
                self,
                "SetPoint",
                point,
                relativeTo,
                relativePoint,
                offsetX or 0,
                offsetY or 0
            )
        end

        for _, frame in ipairs(emm.registeredSystemFrames) do
            local fields = frameFields(frame)
            if fields and rawget(fields, "ClearAllPoints") then
                local clearBase = rawget(fields, "ClearAllPointsBase")
                if type(clearBase) ~= "function" then
                    clearBase = rawget(fields, "ClearAllPoints")
                end
                rawset(fields, "ClearAllPoints", function(self)
                    if type(clearBase) == "function" then
                        clearBase(self)
                    end
                    callRustMethod(self, "ClearAllPoints")
                    pcall(EditModeManagerFrame.OnEditModeSystemAnchorChanged, EditModeManagerFrame)
                end)
            end

            if fields and rawget(fields, "SetPoint") then
                local base = rawget(fields, "SetPointBase") or frame.SetPointBase
                if base then
                    rawset(fields, "SetPoint", function(self, point, relativeTo, relativePoint, offsetX, offsetY)
                        if type(relativeTo) == "number" then
                            offsetX = relativeTo
                            offsetY = relativePoint
                            relativeTo = nil
                            relativePoint = nil
                        end
                        base(self, point, relativeTo, relativePoint, offsetX, offsetY)
                        pcall(syncRustAnchorFromLuaPoint, self)
                        if relativeTo then
                            pcall(self.SetSnappedToFrame, self, relativeTo)
                        end
                        pcall(EditModeManagerFrame.OnEditModeSystemAnchorChanged, EditModeManagerFrame)
                    end)
                end
            end
        end
    