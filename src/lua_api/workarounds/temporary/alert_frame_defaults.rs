//! Temporary AlertFrame defaults.
//!
//! Blizzard_UIParent owns the real AlertFrame queue behavior. This fallback
//! keeps isolated startup and tests functional until that surface is modeled
//! without generic runtime bootstrap help.

const ALERT_FRAME_DEFAULTS_LUA: &str = r#"
local function ensure_alert_frame()
    local frame = rawget(_G, "AlertFrame")
    if frame ~= nil then
        return frame
    end

    if type(CreateFrame) == "function" then
        frame = CreateFrame("Frame", "AlertFrame", UIParent)
    else
        frame = {}
        rawset(_G, "AlertFrame", frame)
    end

    return frame
end

local function remove_array_value(values, target)
    for index = #values, 1, -1 do
        if values[index] == target then
            table.remove(values, index)
            return
        end
    end
end

local frame = ensure_alert_frame()
frame.alertFrameSubSystems = frame.alertFrameSubSystems or {}
if frame.AddQueuedAlertFrameSubSystem == nil then
    function frame:AddQueuedAlertFrameSubSystem(template, setupFn, maxAlerts, anchorSlot)
        local subsystem = {
            template = template,
            templateName = template,
            setupFn = setupFn,
            factory = setupFn,
            maxAlerts = tonumber(maxAlerts) or 0,
            anchorPriority = 1000 + ((#self.alertFrameSubSystems + 1) * 10),
            anchorSlot = anchorSlot,
            queuedAlerts = {},
        }

        function subsystem:SetCanShowMoreConditionFunc(fn)
            self.canShowMoreConditionFunc = fn
        end

        function subsystem:AddAlert(alert)
            if self.maxAlerts > 0 and #self.queuedAlerts >= self.maxAlerts then
                return false
            end
            self.queuedAlerts[#self.queuedAlerts + 1] = alert
            return true
        end

        function subsystem:RemoveAlert(alert)
            remove_array_value(self.queuedAlerts, alert)
        end

        function subsystem:ClearAllAlerts()
            self.queuedAlerts = {}
        end

        self.alertFrameSubSystems[#self.alertFrameSubSystems + 1] = subsystem
        return subsystem
    end
end
"#;

pub(crate) fn apply_bootstrap(lua: &mut rilua::Lua) -> crate::Result<()> {
    lua.exec(ALERT_FRAME_DEFAULTS_LUA)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::lua_api::WowLuaEnv;

    #[test]
    fn installs_alert_frame_defaults() {
        let env = WowLuaEnv::new().expect("lua env should initialize");

        let result: String = env
            .eval(
                r#"
                if type(AlertFrame) ~= "table" then return "frame" end
                if type(AlertFrame.alertFrameSubSystems) ~= "table" then return "subsystems" end
                if type(AlertFrame.AddQueuedAlertFrameSubSystem) ~= "function" then return "add" end

                local first = AlertFrame:AddQueuedAlertFrameSubSystem("TemplateOne", function() end, 2, 2)
                local second = AlertFrame:AddQueuedAlertFrameSubSystem("TemplateTwo", function() end, 2, 3)
                if #AlertFrame.alertFrameSubSystems ~= 2 then return "count" end
                if first.anchorPriority ~= 1010 then return "first_priority" end
                if second.anchorPriority ~= 1020 then return "second_priority" end
                if first:AddAlert("alpha") ~= true then return "add_alpha" end
                if first:AddAlert("beta") ~= true then return "add_beta" end
                if first:AddAlert("gamma") ~= false then return "add_gamma" end
                first:RemoveAlert("alpha")
                if #first.queuedAlerts ~= 1 or first.queuedAlerts[1] ~= "beta" then return "remove" end
                first:ClearAllAlerts()
                if #first.queuedAlerts ~= 0 then return "clear" end
                return "ok"
                "#,
            )
            .expect("alert frame defaults probe should run");

        assert_eq!(result, "ok");
    }
}
