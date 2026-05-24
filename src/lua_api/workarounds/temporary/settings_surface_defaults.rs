//! Temporary Settings surface defaults for partial addon loads.
//!
//! Full UI loads get the real Settings system from Blizzard_Settings. These
//! defaults keep isolated addon loads and legacy InterfaceOptions callers
//! functional without leaving the surface in the generic runtime bootstrap.

const SETTINGS_SURFACE_DEFAULTS_LUA: &str = r#"
local function settings_surface_noop()
end

local function settings_surface_namespace(fields)
    local table = fields or {}
    table.__is_wow_namespace = true
    return table
end

Settings = Settings or settings_surface_namespace({
    GetOrCreateSettingsGroup = function()
        return settings_surface_namespace({
            AddInitializer = settings_surface_noop,
            AddSetting = settings_surface_noop,
            AddCategory = settings_surface_noop,
            SetValue = settings_surface_noop,
            GetValue = function() return nil end,
        })
    end,
})

do
    local settingsPanel = rawget(_G, "SettingsPanel")
    local categories = rawget(Settings, "_categories")
    if type(categories) ~= "table" then
        categories = {}
        rawset(Settings, "_categories", categories)
    end

    local function ensure_category(id, name)
        local category = categories[id]
        if type(category) ~= "table" then
            category = {
                id = id,
                name = name,
                GetID = function(self) return self.id end,
                GetName = function(self) return self.name end,
            }
            categories[id] = category
        end
        return category
    end

    local interfaceCategory = ensure_category(1, "Interface")
    local audioCategory = ensure_category(2, "Audio")
    rawset(Settings, "INTERFACE_CATEGORY_ID", interfaceCategory:GetID())
    rawset(Settings, "AUDIO_CATEGORY_ID", audioCategory:GetID())

    if rawget(Settings, "GetCategory") == nil then
        function Settings.GetCategory(id)
            id = tonumber(id)
            if categories[id] == nil then
                if id == rawget(Settings, "INTERFACE_CATEGORY_ID") then
                    return ensure_category(id, "Interface")
                end
                if id == rawget(Settings, "AUDIO_CATEGORY_ID") then
                    return ensure_category(id, "Audio")
                end
            end
            return categories[id]
        end
    end

    if type(settingsPanel) == "table" then
        settingsPanel._layouts = settingsPanel._layouts or {}

        local function ensure_layout(category)
            local categoryID = category:GetID()
            local layout = settingsPanel._layouts[categoryID]
            if type(layout) ~= "table" then
                layout = {
                    _initializers = {},
                    GetInitializers = function(self) return self._initializers end,
                }
                settingsPanel._layouts[categoryID] = layout
            end
            return layout
        end

        if rawget(settingsPanel, "GetLayout") == nil then
            function settingsPanel:GetLayout(category)
                if type(category) ~= "table" or type(category.GetID) ~= "function" then
                    return nil
                end
                return self._layouts and self._layouts[category:GetID()] or nil
            end
        end

        if rawget(settingsPanel, "GetCurrentCategory") == nil then
            function settingsPanel:GetCurrentCategory()
                return rawget(self, "_currentCategory")
            end
        end

        local nextDynamicCategoryID = 1000

        local function set_category_frame_shown(category, shown)
            local frame = category and category.frame or nil
            if type(frame) ~= "table" then
                return
            end
            if type(frame.SetShown) == "function" then
                pcall(frame.SetShown, frame, shown)
                return
            end
            if shown and type(frame.Show) == "function" then
                pcall(frame.Show, frame)
            elseif not shown and type(frame.Hide) == "function" then
                pcall(frame.Hide, frame)
            end
        end

        local function hide_inactive_category_frames(activeCategory)
            for _, registeredCategory in pairs(categories) do
                if registeredCategory ~= activeCategory then
                    set_category_frame_shown(registeredCategory, false)
                end
            end
        end

        if rawget(Settings, "RegisterCanvasLayoutCategory") == nil then
            function Settings.RegisterCanvasLayoutCategory(frame, name, parentCategory)
                nextDynamicCategoryID = nextDynamicCategoryID + 1
                local categoryName = name
                if categoryName == nil and type(frame) == "table" then
                    categoryName = rawget(frame, "name") or rawget(frame, "Name")
                    if categoryName == nil and type(frame.GetName) == "function" then
                        categoryName = frame:GetName()
                    end
                end
                local category = ensure_category(nextDynamicCategoryID, categoryName or "AddOn")
                category.frame = frame
                category.parentCategory = parentCategory
                settingsPanel._layouts[category:GetID()] = {
                    frame = frame,
                    GetFrame = function(self) return self.frame end,
                }
                set_category_frame_shown(category, false)
                return category, settingsPanel._layouts[category:GetID()]
            end
        end

        if rawget(Settings, "RegisterCanvasLayoutSubcategory") == nil then
            function Settings.RegisterCanvasLayoutSubcategory(parentCategory, frame, name)
                local category = Settings.RegisterCanvasLayoutCategory(frame, name, parentCategory)
                return category, settingsPanel._layouts[category:GetID()]
            end
        end

        if rawget(Settings, "RegisterAddOnCategory") == nil then
            function Settings.RegisterAddOnCategory(_category) end
        end

        local audioLayout = ensure_layout(audioCategory)
        if #audioLayout:GetInitializers() == 0 then
            local setting = {
                GetVariable = function() return "Sound_OutputDriverIndex" end,
            }
            local initializer = {
                GetSetting = function() return setting end,
                GetOptions = function()
                    return function()
                        return {
                            { value = 0, label = "Silent Output Device" },
                        }
                    end
                end,
            }
            table.insert(audioLayout:GetInitializers(), initializer)
        end

        ensure_layout(interfaceCategory)

        function Settings.OpenToCategory(categoryID)
            local category = Settings.GetCategory(categoryID)
            if category == nil then
                return nil
            end
            local panel = rawget(_G, "SettingsPanel") or settingsPanel
            rawset(panel, "_currentCategory", category)
            if type(panel.SetShown) == "function" then
                pcall(panel.SetShown, panel, true)
            end
            if type(panel.Show) == "function" then
                pcall(panel.Show, panel)
            end
            hide_inactive_category_frames(category)
            set_category_frame_shown(category, true)
            return category
        end
    end
end

if rawget(_G, "InterfaceOptions_AddCategory") == nil then
    function InterfaceOptions_AddCategory(frame, addonName, position)
        if Settings and type(Settings.RegisterCanvasLayoutCategory) == "function" then
            local category = Settings.RegisterCanvasLayoutCategory(frame, addonName, position)
            if type(Settings.RegisterAddOnCategory) == "function" then
                Settings.RegisterAddOnCategory(category)
            end
            return category
        end
        return frame
    end
end
"#;

pub(crate) fn apply_bootstrap(lua: &mut rilua::Lua) -> crate::Result<()> {
    lua.exec(SETTINGS_SURFACE_DEFAULTS_LUA)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::lua_api::WowLuaEnv;

    fn apply_again(env: &WowLuaEnv) {
        let mut lua = env.lua.borrow_mut();
        super::apply_bootstrap(&mut lua).expect("Settings surface defaults should apply");
    }

    #[test]
    fn installs_minimal_settings_categories_and_group_factory() {
        let env = WowLuaEnv::new().expect("lua env should initialize");

        let result: (String, String, String, String, String, String) = env
            .eval(
                r#"
                local group = Settings.GetOrCreateSettingsGroup()
                local interface = Settings.GetCategory(Settings.INTERFACE_CATEGORY_ID)
                local audio = Settings.GetCategory(Settings.AUDIO_CATEGORY_ID)
                return type(Settings),
                       type(group.AddInitializer),
                       type(group.GetValue),
                       interface:GetName(),
                       audio:GetName(),
                       type(InterfaceOptions_AddCategory)
                "#,
            )
            .expect("Settings default probe should run");

        assert_eq!(
            result,
            (
                "table".to_string(),
                "function".to_string(),
                "function".to_string(),
                "Interface".to_string(),
                "Audio".to_string(),
                "function".to_string()
            )
        );
    }

    #[test]
    fn installs_canvas_category_helpers_when_settings_panel_exists() {
        let env = WowLuaEnv::new().expect("lua env should initialize");
        env.exec(
            r#"
            Settings = nil
            SettingsPanel = {
                shown = false,
                Show = function(self) self.shown = true end,
            }
            "#,
        )
        .expect("fixture should install SettingsPanel");

        apply_again(&env);

        let result: (String, String, bool, bool, String) = env
            .eval(
                r#"
                local frame = {
                    shown = true,
                    Hide = function(self) self.shown = false end,
                    Show = function(self) self.shown = true end,
                }
                local category, layout = Settings.RegisterCanvasLayoutCategory(frame, "Probe")
                local opened = Settings.OpenToCategory(category:GetID())
                return category:GetName(),
                       type(layout.GetFrame),
                       frame.shown,
                       SettingsPanel.shown,
                       SettingsPanel:GetCurrentCategory():GetName()
                "#,
            )
            .expect("Settings canvas category probe should run");

        assert_eq!(
            result,
            (
                "Probe".to_string(),
                "function".to_string(),
                true,
                true,
                "Probe".to_string()
            )
        );
    }

    #[test]
    fn preserves_existing_settings_get_category() {
        let env = WowLuaEnv::new().expect("lua env should initialize");
        env.exec(
            r#"
            Settings.GetCategory = function(name)
                return "existing:" .. tostring(name)
            end
            "#,
        )
        .expect("fixture should install existing Settings.GetCategory");

        apply_again(&env);

        let value: String = env
            .eval(r#"return Settings.GetCategory("KrowiTest")"#)
            .expect("Settings.GetCategory preservation probe should run");

        assert_eq!(value, "existing:KrowiTest");
    }
}
