use mlua::{Lua, Result, Value};

pub(super) fn register_missing_global_tables(lua: &Lua, g: &mlua::Table) -> Result<()> {
    register_simple_stub_tables(lua, g)?;
    register_ui_frame_manager_stub(lua, g)?;
    register_action_button_spell_alert_manager(lua, g)?;
    Ok(())
}

/// QuestUtil, ChatFrameMixin, TalentButtonUtil, SpellSearchUtil, Dispatcher stubs.
fn register_simple_stub_tables(lua: &Lua, g: &mlua::Table) -> Result<()> {
    if g.get::<Value>("QuestUtil")?.is_nil() {
        g.set("QuestUtil", lua.create_table()?)?;
    }
    if g.get::<Value>("ChatFrameMixin")?.is_nil() {
        g.set("ChatFrameMixin", lua.create_table()?)?;
    }
    if g.get::<Value>("ChatFrameEditBoxMixin")?.is_nil() {
        g.set("ChatFrameEditBoxMixin", lua.create_table()?)?;
    }
    if g.get::<Value>("TalentButtonUtil")?.is_nil() {
        g.set("TalentButtonUtil", build_talent_button_util(lua)?)?;
    }
    if g.get::<Value>("SpellSearchUtil")?.is_nil() {
        g.set("SpellSearchUtil", build_spell_search_util(lua)?)?;
    }
    if g.get::<Value>("Dispatcher")?.is_nil() {
        g.set("Dispatcher", build_dispatcher_stub(lua)?)?;
    }
    Ok(())
}

/// UIFrameManager_ManagedFrameMixin stub — needed before Blizzard_UIFrameManager loads.
/// (Blizzard_UIFrameManager loads after Blizzard_Tutorials alphabetically.)
fn register_ui_frame_manager_stub(lua: &Lua, _g: &mlua::Table) -> Result<()> {
    install_ui_frame_manager_namespace(lua)?;
    install_ui_frame_manager_managed_mixin(lua)
}

const UI_FRAME_MANAGER_NAMESPACE_LUA: &str = r#"
    if UIFrameManager == nil then
        UIFrameManager = {
            registeredFrames = {},
            registeredFrameTypeToFrames = {},
        }

        function UIFrameManager:RegisterFrameForFrameType(frame, frameType)
            if self.registeredFrames[frame] then
                return
            end

            if self.registeredFrameTypeToFrames[frameType] == nil then
                self.registeredFrameTypeToFrames[frameType] = {}
            end

            self.registeredFrameTypeToFrames[frameType][frame] = true
            self.registeredFrames[frame] = true

            frame:UpdateFrameState(C_FrameManager.GetFrameVisibilityState(frameType))
        end

        function UIFrameManager:OnEvent(event, ...)
            if event == "FRAME_MANAGER_UPDATE_ALL" then
                for frameType, frames in pairs(self.registeredFrameTypeToFrames) do
                    for frame in pairs(frames) do
                        frame:UpdateFrameState(C_FrameManager.GetFrameVisibilityState(frameType))
                    end
                end
            else
                local frameType, show = ...
                local frames = self.registeredFrameTypeToFrames[frameType]
                if frames then
                    for frame in pairs(frames) do
                        frame:UpdateFrameState(show)
                    end
                end
            end
        end
    end
"#;

fn install_ui_frame_manager_namespace(lua: &Lua) -> Result<()> {
    lua.load(UI_FRAME_MANAGER_NAMESPACE_LUA).exec()
}

const UI_FRAME_MANAGER_MANAGED_MIXIN_LUA: &str = r#"
    if UIFrameManager_ManagedFrameMixin == nil then
        UIFrameManager_ManagedFrameMixin = {}

        function UIFrameManager_ManagedFrameMixin:OnLoad()
            UIFrameManager:RegisterFrameForFrameType(self, self.frameType)
        end

        function UIFrameManager_ManagedFrameMixin:UpdateFrameState(show)
            self:SetShown(show)
        end
    end
"#;

fn install_ui_frame_manager_managed_mixin(lua: &Lua) -> Result<()> {
    lua.load(UI_FRAME_MANAGER_MANAGED_MIXIN_LUA).exec()
}

/// ActionButtonSpellAlertManager stub — referenced by PetBattleUI OnLoad before ActionBar loads.
fn register_action_button_spell_alert_manager(lua: &Lua, _g: &mlua::Table) -> Result<()> {
    install_action_button_spell_alert_manager_namespace(lua)?;
    install_action_button_spell_alert_manager_methods(lua)
}

const ACTION_BUTTON_SPELL_ALERT_MANAGER_NAMESPACE_LUA: &str = r#"
    if ActionButtonSpellAlertManager == nil then
        ActionButtonSpellAlertManager = {
            activeAlerts = {},
            SpellAlertType = { Default = 1, AssistedCombatRotation = 2 },
        }
    end

    if ActionButtonSpellAlertManager.GetAlertFrame == nil then
        function ActionButtonSpellAlertManager:GetAlertFrame(actionButton, create)
            local frame = actionButton.SpellActivationAlert
            if frame == nil and create then
                frame = CreateFrame("Frame", nil, actionButton)
                frame:SetAllPoints(actionButton)
                frame:Hide()
                actionButton.SpellActivationAlert = frame
            end
            return frame
        end
    end
"#;

fn install_action_button_spell_alert_manager_namespace(lua: &Lua) -> Result<()> {
    lua.load(ACTION_BUTTON_SPELL_ALERT_MANAGER_NAMESPACE_LUA)
        .exec()
}

const ACTION_BUTTON_SPELL_ALERT_MANAGER_METHODS_LUA: &str = r#"
    if ActionButtonSpellAlertManager and ActionButtonSpellAlertManager.ShowAlert == nil then
        function ActionButtonSpellAlertManager:ShowAlert(actionButton, skipBirth)
            local currentType = self.activeAlerts[actionButton]
            local alertType = self.SpellAlertType.Default
            if currentType == alertType then
                local alertFrame = self:GetAlertFrame(actionButton, false)
                if alertFrame then
                    alertFrame:Show()
                end
                return
            end

            self.activeAlerts[actionButton] = alertType
            local alertFrame = self:GetAlertFrame(actionButton, true)
            alertFrame:Show()
        end
    end

    if ActionButtonSpellAlertManager and ActionButtonSpellAlertManager.HideAlert == nil then
        function ActionButtonSpellAlertManager:HideAlert(actionButton)
            if self.activeAlerts[actionButton] == nil then
                return
            end

            local alertFrame = self:GetAlertFrame(actionButton, false)
            if alertFrame then
                alertFrame:Hide()
            end
            self.activeAlerts[actionButton] = nil
        end
    end

    if ActionButtonSpellAlertManager and ActionButtonSpellAlertManager.HasAlert == nil then
        function ActionButtonSpellAlertManager:HasAlert(actionButton)
            local alertType = self.activeAlerts[actionButton]
            return alertType ~= nil, alertType
        end
    end
"#;

fn install_action_button_spell_alert_manager_methods(lua: &Lua) -> Result<()> {
    lua.load(ACTION_BUTTON_SPELL_ALERT_MANAGER_METHODS_LUA)
        .exec()
}

/// TalentButtonUtil - utility table for talent button rendering.
fn build_talent_button_util(lua: &Lua) -> Result<mlua::Table> {
    let tbu = lua.create_table()?;
    tbu.set("CircleEdgeDiameterOffset", 1.2f64)?;
    tbu.set("SquareEdgeMinDiameterOffset", 1.2f64)?;
    tbu.set("SquareEdgeMaxDiameterOffset", 1.5f64)?;
    tbu.set("ChoiceEdgeMinDiameterOffset", 1.2f64)?;
    tbu.set("ChoiceEdgeMaxDiameterOffset", 1.5f64)?;
    let bvs = lua.create_table()?;
    for (i, name) in [
        "Normal",
        "Gated",
        "Disabled",
        "Locked",
        "Selectable",
        "Maxed",
        "Invisible",
        "RefundInvalid",
        "DisplayError",
    ]
    .iter()
    .enumerate()
    {
        bvs.set(*name, (i + 1) as i32)?;
    }
    tbu.set("BaseVisualState", bvs)?;
    Ok(tbu)
}

/// SpellSearchUtil - spell search utility tables.
fn build_spell_search_util(lua: &Lua) -> Result<mlua::Table> {
    let ssu = lua.create_table()?;
    let mt = lua.create_table()?;
    for (i, name) in [
        "DescriptionMatch",
        "NameMatch",
        "RelatedMatch",
        "ExactMatch",
        "NotOnActionBar",
        "OnInactiveBonusBar",
        "OnDisabledActionBar",
        "AssistedCombat",
    ]
    .iter()
    .enumerate()
    {
        mt.set(*name, (i + 1) as i32)?;
    }
    ssu.set("MatchType", mt)?;
    let st = lua.create_table()?;
    for (i, name) in ["Trait", "PvPTalent", "SpellBookItem"].iter().enumerate() {
        st.set(*name, (i + 1) as i32)?;
    }
    ssu.set("SourceType", st)?;
    let ft = lua.create_table()?;
    for (i, name) in ["Text", "ActionBar", "Name", "AssistedCombat"]
        .iter()
        .enumerate()
    {
        ft.set(*name, (i + 1) as i32)?;
    }
    ssu.set("FilterType", ft)?;
    ssu.set("ActionBarStatusTooltips", lua.create_table()?)?;
    Ok(ssu)
}

const DISPATCHER_STUB_LUA: &str = r#"
        local dispatcherFrame = CreateFrame("Frame")
        local nextID = 1
        local eventEntries = {}
        local functionHooks = {}
        local scriptHooks = {}

        local function nextToken()
            local id = nextID
            nextID = nextID + 1
            return id
        end

        local function resolveCallback(kind, key, callback)
            if type(callback) == "function" then
                return callback, callback
            end

            if type(callback) ~= "table" then
                return nil, callback
            end

            local method = callback[key]
            if type(method) == "function" then
                return function(...)
                    return method(callback, ...)
                end, callback
            end

            if kind == "event" then
                local onEvent = callback.OnEvent
                if type(onEvent) == "function" then
                    return function(...)
                        return onEvent(callback, key, ...)
                    end, callback
                end
            end

            return nil, callback
        end

        local function removeListEntry(list, match)
            for i = #list, 1, -1 do
                if match(list[i]) then
                    table.remove(list, i)
                end
            end
        end

        local function trimEvent(eventName)
            local entries = eventEntries[eventName]
            if not entries or #entries == 0 then
                eventEntries[eventName] = nil
                if eventName ~= "OnUpdate" then
                    dispatcherFrame:UnregisterEvent(eventName)
                else
                    dispatcherFrame:SetScript("OnUpdate", nil)
                end
            end
        end

        local function dispatchEntries(entries, ...)
            if not entries then
                return
            end

            local removals = {}
            for _, entry in ipairs(entries) do
                entry.callback(...)
                if entry.once then
                    table.insert(removals, entry.id)
                end
            end
            for _, id in ipairs(removals) do
                removeListEntry(entries, function(entry) return entry.id == id end)
            end
        end

        dispatcherFrame:SetScript("OnEvent", function(_, event, ...)
            local entries = eventEntries[event]
            dispatchEntries(entries, ...)
            trimEvent(event)
        end)

        local Dispatcher = {}

        function Dispatcher:RegisterEvent(eventName, callback, once)
            local cb, owner = resolveCallback("event", eventName, callback)
            if not cb then
                return nil
            end

            local entry = {
                id = nextToken(),
                owner = owner,
                callback = cb,
                once = once == true,
            }

            if not eventEntries[eventName] then
                eventEntries[eventName] = {}
                if eventName ~= "OnUpdate" then
                    dispatcherFrame:RegisterEvent(eventName)
                else
                    dispatcherFrame:SetScript("OnUpdate", function(_, elapsed)
                        local entries = eventEntries.OnUpdate
                        dispatchEntries(entries, elapsed)
                        trimEvent("OnUpdate")
                    end)
                end
            end

            table.insert(eventEntries[eventName], entry)
            return entry.id
        end

        function Dispatcher:UnregisterEvent(eventName, ownerOrToken)
            local entries = eventEntries[eventName]
            if not entries then
                return
            end
            removeListEntry(entries, function(entry)
                return entry.id == ownerOrToken or entry.owner == ownerOrToken
            end)
            trimEvent(eventName)
        end

        function Dispatcher:UnregisterAllEvents(ownerOrToken)
            for eventName, entries in pairs(eventEntries) do
                removeListEntry(entries, function(entry)
                    return entry.id == ownerOrToken or entry.owner == ownerOrToken
                end)
                trimEvent(eventName)
            end
        end

        local function functionHookKey(target, method)
            return tostring(target) .. "\31" .. method
        end

        local function trimFunctionHook(hookKey)
            local hook = functionHooks[hookKey]
            if not hook or #hook.entries > 0 then
                return
            end
            hook.target[hook.method] = hook.original
            functionHooks[hookKey] = nil
        end

        local function ensureFunctionHook(target, method)
            local hookKey = functionHookKey(target, method)
            local hook = functionHooks[hookKey]
            if hook then
                return hookKey, hook
            end

            local original = target[method]
            hook = {
                target = target,
                method = method,
                original = original,
                entries = {},
            }
            functionHooks[hookKey] = hook
            target[method] = function(...)
                if type(hook.original) == "function" then
                    hook.original(...)
                end
                local removals = {}
                for _, entry in ipairs(hook.entries) do
                    entry.callback(...)
                    if entry.once then
                        table.insert(removals, entry.id)
                    end
                end
                for _, id in ipairs(removals) do
                    removeListEntry(hook.entries, function(entry) return entry.id == id end)
                end
                trimFunctionHook(hookKey)
            end
            return hookKey, hook
        end

        function Dispatcher:RegisterFunction(targetOrName, methodOrCallback, callbackOrOnce, once)
            local target, method, callback, fireOnce
            if type(targetOrName) == "string" then
                target = _G
                method = targetOrName
                callback = methodOrCallback
                fireOnce = callbackOrOnce
            else
                target = targetOrName
                method = methodOrCallback
                callback = callbackOrOnce
                fireOnce = once
            end

            local cb, owner = resolveCallback("function", method, callback)
            if not cb then
                return nil
            end

            local _, hook = ensureFunctionHook(target, method)
            local entry = {
                id = nextToken(),
                owner = owner,
                callback = cb,
                once = fireOnce == true,
            }
            table.insert(hook.entries, entry)
            return entry.id
        end

        function Dispatcher:UnregisterFunction(targetOrName, methodOrOwner, ownerOrToken)
            local target, method, owner = nil, nil, nil
            if type(targetOrName) == "string" then
                target = _G
                method = targetOrName
                owner = methodOrOwner
            else
                target = targetOrName
                method = methodOrOwner
                owner = ownerOrToken
            end

            if type(method) ~= "string" then
                return
            end

            local hookKey = functionHookKey(target, method)
            local hook = functionHooks[hookKey]
            if not hook then
                return
            end

            removeListEntry(hook.entries, function(entry)
                return entry.id == owner or entry.owner == owner
            end)
            trimFunctionHook(hookKey)
        end

        function Dispatcher:UnregisterAllFunctions(ownerOrToken)
            for hookKey, hook in pairs(functionHooks) do
                removeListEntry(hook.entries, function(entry)
                    return entry.id == ownerOrToken or entry.owner == ownerOrToken
                end)
                trimFunctionHook(hookKey)
            end
        end

        local function scriptHookKey(frame, script)
            return tostring(frame) .. "\31" .. script
        end

        local function trimScriptHook(hookKey)
            local hook = scriptHooks[hookKey]
            if not hook or #hook.entries > 0 then
                return
            end
            hook.frame:SetScript(hook.script, hook.original)
            scriptHooks[hookKey] = nil
        end

        local function ensureScriptHook(frame, script)
            local hookKey = scriptHookKey(frame, script)
            local hook = scriptHooks[hookKey]
            if hook then
                return hookKey, hook
            end

            local original = frame:GetScript(script)
            hook = {
                frame = frame,
                script = script,
                original = original,
                entries = {},
            }
            scriptHooks[hookKey] = hook
            frame:SetScript(script, function(...)
                if type(hook.original) == "function" then
                    hook.original(...)
                end
                local removals = {}
                for _, entry in ipairs(hook.entries) do
                    entry.callback(...)
                    if entry.once then
                        table.insert(removals, entry.id)
                    end
                end
                for _, id in ipairs(removals) do
                    removeListEntry(hook.entries, function(entry) return entry.id == id end)
                end
                trimScriptHook(hookKey)
            end)
            return hookKey, hook
        end

        function Dispatcher:RegisterScript(frame, script, callback, once)
            local cb, owner = resolveCallback("script", script, callback)
            if not cb then
                return nil
            end

            local _, hook = ensureScriptHook(frame, script)
            local entry = {
                id = nextToken(),
                owner = owner,
                callback = cb,
                once = once == true,
            }
            table.insert(hook.entries, entry)
            return entry.id
        end

        function Dispatcher:UnregisterScript(frame, script, ownerOrToken)
            local hookKey = scriptHookKey(frame, script)
            local hook = scriptHooks[hookKey]
            if not hook then
                return
            end

            removeListEntry(hook.entries, function(entry)
                return entry.id == ownerOrToken or entry.owner == ownerOrToken
            end)
            trimScriptHook(hookKey)
        end

        function Dispatcher:UnregisterAllScripts(ownerOrToken)
            for hookKey, hook in pairs(scriptHooks) do
                removeListEntry(hook.entries, function(entry)
                    return entry.id == ownerOrToken or entry.owner == ownerOrToken
                end)
                trimScriptHook(hookKey)
            end
        end

        function Dispatcher:UnregisterAll(ownerOrToken)
            self:UnregisterAllEvents(ownerOrToken)
            self:UnregisterAllFunctions(ownerOrToken)
            self:UnregisterAllScripts(ownerOrToken)
        end

        return Dispatcher
        "#;

fn evaluate_dispatcher_stub(lua: &Lua) -> Result<mlua::Table> {
    lua.load(DISPATCHER_STUB_LUA).eval::<mlua::Table>()
}

/// Dispatcher - event dispatch system (real impl: Blizzard_Dispatcher addon).
fn build_dispatcher_stub(lua: &Lua) -> Result<mlua::Table> {
    evaluate_dispatcher_stub(lua)
}
