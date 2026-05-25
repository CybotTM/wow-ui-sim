//! Temporary dropdown list frame defaults.
//!
//! The Rust UIDropDownMenu API owns dropdown behavior, but partial Blizzard
//! loads still expect the legacy DropDownList frame tree to exist at startup.

const DROPDOWN_LIST_DEFAULTS_LUA: &str = r#"
local function ensure_named_frame(frameType, name, parent)
    local existing = rawget(_G, name)
    if existing ~= nil then
        return existing
    end
    if type(CreateFrame) ~= "function" then
        return nil
    end
    return CreateFrame(frameType or "Frame", name, parent)
end

local function ensure_named_child(parent, key, frameType, name)
    if parent == nil then
        return nil
    end
    local existing = rawget(parent, key)
    if existing ~= nil then
        return existing
    end
    local child = CreateFrame(frameType or "Frame", name, parent)
    rawset(parent, key, child)
    return child
end

local function hide_frame(frame)
    if frame ~= nil and type(frame.Hide) == "function" then
        frame:Hide()
    end
end

local function seed_dropdown_button_template_children(button, buttonName)
    local highlight = ensure_named_child(button, "Highlight", "Texture", buttonName .. "Highlight")
    hide_frame(highlight)

    local check = ensure_named_child(button, "Check", "Texture", buttonName .. "Check")
    if check ~= nil and type(check.SetTexture) == "function" then
        check:SetTexture("Interface\\Common\\UI-DropDownRadioChecks")
    end

    local uncheck = ensure_named_child(button, "UnCheck", "Texture", buttonName .. "UnCheck")
    if uncheck ~= nil and type(uncheck.SetTexture) == "function" then
        uncheck:SetTexture("Interface\\Common\\UI-DropDownRadioChecks")
    end

    local icon = ensure_named_child(button, "Icon", "Texture", buttonName .. "Icon")
    hide_frame(icon)

    local colorSwatch = ensure_named_child(button, "ColorSwatch", "Button", buttonName .. "ColorSwatch")
    if colorSwatch ~= nil then
        hide_frame(colorSwatch)
        local color = ensure_named_child(colorSwatch, "Color", "Texture", buttonName .. "ColorSwatchColor")
        if color ~= nil then
            rawset(colorSwatch, "Color", color)
        end
    end

    local expandArrow = ensure_named_child(button, "ExpandArrow", "Button", buttonName .. "ExpandArrow")
    hide_frame(expandArrow)

    local invisibleButton = ensure_named_child(button, "invisibleButton", "Button", buttonName .. "InvisibleButton")
    hide_frame(invisibleButton)

    local newFeature = ensure_named_child(button, "NewFeature", "Frame", buttonName .. "NewFeature")
    hide_frame(newFeature)

    local text = ensure_named_child(button, "Text", "FontString", buttonName .. "NormalText")
    if text ~= nil then
        if type(text.SetFontObject) == "function" then
            text:SetFontObject("GameFontHighlightSmall")
        end
        if type(text.SetText) == "function" then
            text:SetText("")
        end
    end
end

local function seed_dropdown_list(level)
    local listName = "DropDownList" .. tostring(level)
    local list = ensure_named_frame("Button", listName, UIParent)
    if list == nil then
        return
    end

    if type(list.SetFrameStrata) == "function" then
        list:SetFrameStrata("FULLSCREEN_DIALOG")
    end
    if type(list.SetClampedToScreen) == "function" then
        list:SetClampedToScreen(true)
    end
    hide_frame(list)
    list.numButtons = 0
    list.maxWidth = 0

    for index = 1, 8 do
        local buttonName = listName .. "Button" .. tostring(index)
        local button = ensure_named_child(list, "Button" .. tostring(index), "Button", buttonName)
        if button ~= nil then
            hide_frame(button)
            seed_dropdown_button_template_children(button, buttonName)
        end
    end

    if level == 1 then
        local button1 = rawget(list, "Button1")
        local normalText = button1 ~= nil and rawget(button1, "Text") or nil
        if normalText ~= nil and type(normalText.GetFont) == "function" then
            local _, fontHeight = normalText:GetFont()
            if fontHeight ~= nil then
                UIDROPDOWNMENU_DEFAULT_TEXT_HEIGHT = fontHeight
            end
        end
    end
end

for level = 1, 3 do
    seed_dropdown_list(level)
end

local function copy_mixin_methods(target, source)
    if type(target) ~= "table" or type(source) ~= "table" then
        return target
    end
    for key, value in pairs(source) do
        if rawget(target, key) == nil then
            rawset(target, key, value)
        end
    end
    return target
end

if type(DropdownSelectionTextMixin) ~= "table" then
    DropdownSelectionTextMixin = {}

    function DropdownSelectionTextMixin:SetDefaultText(text)
        self.defaultText = text
    end

    function DropdownSelectionTextMixin:SetSelectionTranslator(translator)
        self.selectionTranslator = translator
    end

    function DropdownSelectionTextMixin:SetSelectionText(selectionFunc)
        self.selectionFunc = selectionFunc
    end

    function DropdownSelectionTextMixin:UpdateToMenuSelections(_menuDescription, currentSelections)
        if self.disableSelectionText then
            return
        end
        local text = nil
        if type(self.selectionFunc) == "function" then
            text = self.selectionFunc(currentSelections or {})
        end
        if text == nil then
            text = self.defaultText
        end
        if text ~= nil and type(self.SetText) == "function" then
            self:SetText(text)
        end
    end

    function DropdownSelectionTextMixin:OnShow()
        if type(self.GenerateMenu) == "function" then
            self:GenerateMenu()
        end
    end
end

if type(WowStyle1DropdownMixin) ~= "table" then
    WowStyle1DropdownMixin = copy_mixin_methods({}, DropdownButtonMixin)

    function WowStyle1DropdownMixin:OnLoad()
        DropdownButtonMixin.OnLoad(self)
    end

    function WowStyle1DropdownMixin:OnButtonStateChanged()
    end

    function WowStyle1DropdownMixin:GetArrowAtlas()
        return nil
    end
end
copy_mixin_methods(WowStyle1DropdownMixin, DropdownSelectionTextMixin)

if type(WowStyle1FilterDropdownMixin) ~= "table" then
    WowStyle1FilterDropdownMixin = copy_mixin_methods({}, WowStyle1DropdownMixin)
end
copy_mixin_methods(WowStyle1FilterDropdownMixin, WowStyle1DropdownMixin)
copy_mixin_methods(WowStyle1FilterDropdownMixin, DropdownSelectionTextMixin)

if type(WowStyle1ArrowDropdownMixin) ~= "table" then
    WowStyle1ArrowDropdownMixin = copy_mixin_methods({}, WowStyle1DropdownMixin)
end
copy_mixin_methods(WowStyle1ArrowDropdownMixin, WowStyle1DropdownMixin)
copy_mixin_methods(WowStyle1ArrowDropdownMixin, DropdownSelectionTextMixin)

if type(WowDropdownFilterBehaviorMixin) ~= "table" then
    WowDropdownFilterBehaviorMixin = {}

    function WowDropdownFilterBehaviorMixin:OnLoad()
        if type(self.SetSelectionText) ~= "function" and DropdownButtonMixin ~= nil then
            self.SetSelectionText = DropdownButtonMixin.SetSelectionText
            self.GetSelectionText = DropdownButtonMixin.GetSelectionText
        end
    end

    function WowDropdownFilterBehaviorMixin:OnShow()
    end

    function WowDropdownFilterBehaviorMixin:SetDefaultCallback(callback)
        self.__wow_default_callback = callback
    end

    function WowDropdownFilterBehaviorMixin:SetIsDefaultCallback(callback)
        self.__wow_is_default_callback = callback
    end

    function WowDropdownFilterBehaviorMixin:SetUpdateCallback(callback)
        self.__wow_update_callback = callback
    end

    function WowDropdownFilterBehaviorMixin:NotifyUpdate(description)
        if type(self.__wow_update_callback) == "function" then
            self.__wow_update_callback(description)
        end
    end

    function WowDropdownFilterBehaviorMixin:Reset()
    end

    function WowDropdownFilterBehaviorMixin:ValidateResetState()
    end

    function WowDropdownFilterBehaviorMixin:OnMenuResponse(_menu, description)
        self:NotifyUpdate(description)
    end

    function WowDropdownFilterBehaviorMixin:OnMenuAssigned()
    end
end

if type(WowFilterButtonMixin) ~= "table" then
    WowFilterButtonMixin = copy_mixin_methods({}, WowDropdownFilterBehaviorMixin)
end
copy_mixin_methods(WowFilterButtonMixin, WowDropdownFilterBehaviorMixin)
copy_mixin_methods(WowFilterButtonMixin, DropdownSelectionTextMixin)
"#;

pub(crate) fn apply_bootstrap(lua: &mut rilua::Lua) -> crate::Result<()> {
    lua.exec(DROPDOWN_LIST_DEFAULTS_LUA)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::lua_api::WowLuaEnv;

    #[test]
    fn installs_dropdown_list_defaults() {
        let env = WowLuaEnv::new().expect("lua env should initialize");

        let result: String = env
            .eval(
                r#"
                for level = 1, 3 do
                    local list = rawget(_G, "DropDownList" .. tostring(level))
                    if type(list) ~= "table" then return "list" .. tostring(level) end
                    if list:IsVisible() then return "visible" .. tostring(level) end
                    if list.numButtons ~= 0 then return "numButtons" .. tostring(level) end
                    if list.maxWidth ~= 0 then return "maxWidth" .. tostring(level) end
                    for index = 1, 8 do
                        local buttonName = "DropDownList" .. tostring(level) .. "Button" .. tostring(index)
                        local button = rawget(_G, buttonName)
                        if type(button) ~= "table" then return buttonName end
                        if type(button.Text) ~= "table" then return buttonName .. "Text" end
                        if type(button.Icon) ~= "table" then return buttonName .. "Icon" end
                        if type(button.invisibleButton) ~= "table" then return buttonName .. "InvisibleButton" end
                        if type(button.ColorSwatch) ~= "table" then return buttonName .. "ColorSwatch" end
                        if type(button.ColorSwatch.Color) ~= "table" then return buttonName .. "Color" end
                    end
                end
                if type(DropdownSelectionTextMixin) ~= "table" then return "selection_mixin" end
                if type(WowDropdownFilterBehaviorMixin) ~= "table" then return "filter_behavior_mixin" end
                if type(WowFilterButtonMixin) ~= "table" then return "filter_button_mixin" end
                if type(WowStyle1DropdownMixin) ~= "table" then return "style1_dropdown_mixin" end
                if type(WowStyle1FilterDropdownMixin) ~= "table" then return "style1_filter_mixin" end
                if type(WowStyle1ArrowDropdownMixin) ~= "table" then return "style1_arrow_mixin" end
                return "ok"
                "#,
            )
            .expect("dropdown list defaults probe should run");

        assert_eq!(result, "ok");
    }
}
