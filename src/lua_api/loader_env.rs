//! Lightweight loader environment for addon loading.

use super::env::WowLuaEnv;
use super::globals::security::mark_secure_state;
use super::methods::create_string;
use crate::Result;
use crate::lua_api::methods::create_table;
use crate::lua_api::script_helpers::call_error_handler_state;
use crate::lua_bridge::table_set_rust_fn_static;
use rilua::LuaApiMut;
use rilua::Val;
use rilua::vm::state::LuaState;
use std::cell::{Ref, RefMut};
use std::marker::PhantomData;
use std::ptr::NonNull;
use std::rc::Rc;

use super::state::SimState;

/// Post-load patch for Blizzard_UIParent's managed-frame mixin. Blizzard
/// wires `layoutParent` via a template `<KeyValue type="global">` that
/// resolves eagerly against `_G`; in the sim the container may not
/// exist yet when a child frame fires OnHide, so guard both methods.
const MANAGED_FRAME_MIXIN_PATCH_LUA: &str = r#"
if UIParentManagedFrameMixin ~= nil then
    function UIParentManagedFrameMixin:OnShow()
        if self.layoutParent and self.layoutParent.AddManagedFrame then
            self.layoutParent:AddManagedFrame(self)
        end
    end
    function UIParentManagedFrameMixin:OnHide()
        if self.layoutParent and self.layoutParent.RemoveManagedFrame then
            self.layoutParent:RemoveManagedFrame(self)
        end
    end
end
"#;

const UNIT_POSITION_FRAME_MIXIN_PATCH_LUA: &str = r#"
if UnitPositionFrameMixin ~= nil then
    local orig = UnitPositionFrameMixin.OnHide
    UnitPositionFrameMixin.OnHide = function(self, ...)
        if self.dataProvider then
            return orig(self, ...)
        end
    end
end
if GroupMembersPinMixin ~= nil then
    local orig = GroupMembersPinMixin.OnHide
    GroupMembersPinMixin.OnHide = function(self, ...)
        if self.dataProvider then
            return orig(self, ...)
        end
    end
end
"#;

const QUEST_LOG_MIXIN_PATCH_LUA: &str = r#"
local function SafeGetCurrentMapID(self)
    local parent = self:GetParent()
    if parent and parent:IsShown() then
        return parent:GetMapID()
    end
    return C_Map.GetBestMapForUnit("player")
end
-- Patch the mixin for future frames
if QuestLogMixin ~= nil then
    QuestLogMixin.GetCurrentMapID = SafeGetCurrentMapID
end
-- Patch the existing QuestMapFrame instance directly
if QuestMapFrame then
    QuestMapFrame.GetCurrentMapID = SafeGetCurrentMapID
end

if type(QuestMapFrame_UpdateAll) == "function" and not rawget(_G, "__wow_quest_map_update_all_patched") then
    local originalUpdateAll = QuestMapFrame_UpdateAll
    QuestMapFrame_UpdateAll = function(numPOIs)
        local parent = QuestMapFrame and QuestMapFrame:GetParent() or nil
        if parent == nil then
            QuestMapFrame.UpdatePOIs(QuestMapFrame)
            if not numPOIs then
                QuestMapUpdateAllQuests()
            end
            return
        end
        return originalUpdateAll(numPOIs)
    end
    rawset(_G, "__wow_quest_map_update_all_patched", true)
end
"#;

/// Permissive dropdown descriptor installed when Blizzard_Menu fails to
/// define `Menu.CreateRootMenuDescription`. Every unknown method returns
/// the table itself so method chains (e.g.
/// `rootDescription:CreateRadio(...):SetEnabled(false)`) don't blow up,
/// matching the shape downstream code expects.
const MENU_DESCRIPTOR_FALLBACK_LUA: &str = r#"
if Menu == nil then Menu = {} end
local function __wow_menu_empty_iterator() return nil end
local __wow_menu_iterator_methods = {
    EnumerateElementDescriptions = true,
    EnumerateActiveElementDescriptions = true,
    EnumerateChildren = true,
    EnumerateInitializers = true,
    EnumerateFrames = true,
}
local function __wow_menu_descriptor_stub()
    local desc = { __wow_elements = {} }
    local function add_child(kind, text, ...)
        local child = __wow_menu_descriptor_stub()
        child.kind = kind
        child.text = text
        child.args = { ... }
        table.insert(desc.__wow_elements, child)
        return child
    end
    function desc:SetTag(tag)
        self.tag = tag
        return self
    end
    function desc:CreateRadio(text, ...)
        return add_child("radio", text, ...)
    end
    function desc:CreateButton(text, ...)
        return add_child("button", text, ...)
    end
    function desc:CreateTitle(text, ...)
        return add_child("title", text, ...)
    end
    function desc:CreateDivider(...)
        return add_child("divider", nil, ...)
    end
    setmetatable(desc, {
        __index = function(_, key)
            if __wow_menu_iterator_methods[key] then
                return function()
                    return __wow_menu_empty_iterator
                end
            end
            return function(self)
                return self
            end
        end,
    })
    return desc
end

local function __wow_wrap_menu_descriptor(desc)
    if type(desc) ~= "table" then
        return __wow_menu_descriptor_stub()
    end
    if type(desc.__wow_elements) ~= "table" then
        desc.__wow_elements = {}
    end
    local previous_mt = getmetatable(desc)
    local previous_index = previous_mt and previous_mt.__index or nil
    local function add_child(kind, text, ...)
        local child = __wow_wrap_menu_descriptor({})
        child.kind = kind
        child.text = text
        child.args = { ... }
        table.insert(desc.__wow_elements, child)
        return child
    end
    if rawget(desc, "SetTag") == nil then
        desc.SetTag = function(self, tag)
            self.tag = tag
            return self
        end
    end
    if rawget(desc, "CreateRadio") == nil then
        desc.CreateRadio = function(_, text, ...)
            return add_child("radio", text, ...)
        end
    end
    if rawget(desc, "CreateButton") == nil then
        desc.CreateButton = function(_, text, ...)
            return add_child("button", text, ...)
        end
    end
    if rawget(desc, "CreateTitle") == nil then
        desc.CreateTitle = function(_, text, ...)
            return add_child("title", text, ...)
        end
    end
    if rawget(desc, "CreateDivider") == nil then
        desc.CreateDivider = function(_, ...)
            return add_child("divider", nil, ...)
        end
    end
    setmetatable(desc, {
        __index = function(self, key)
            local existing = rawget(self, key)
            if existing ~= nil then
                return existing
            end
            if previous_index ~= nil then
                if type(previous_index) == "function" then
                    existing = previous_index(self, key)
                else
                    existing = previous_index[key]
                end
                if existing ~= nil then
                    return existing
                end
            end
            if __wow_menu_iterator_methods[key] then
                return function()
                    return __wow_menu_empty_iterator
                end
            end
            return function(inner_self)
                return inner_self
            end
        end,
    })
    return desc
end

if not rawget(_G, "__wow_menu_fallback_installed") then
    rawset(_G, "__wow_menu_fallback_installed", true)

    local __wow_existing_create_root = Menu.CreateRootMenuDescription
    function Menu.CreateRootMenuDescription(menuMixin)
        if type(__wow_existing_create_root) == "function" then
            local ok, desc = pcall(__wow_existing_create_root, menuMixin)
            if ok then
                return __wow_wrap_menu_descriptor(desc)
            end
        end
        return __wow_menu_descriptor_stub()
    end

    local __wow_existing_create_element = Menu.CreateMenuElementDescription
    function Menu.CreateMenuElementDescription(...)
        if type(__wow_existing_create_element) == "function" then
            local ok, desc = pcall(__wow_existing_create_element, ...)
            if ok then
                return __wow_wrap_menu_descriptor(desc)
            end
        end
        return __wow_menu_descriptor_stub()
    end

    function Menu.PopulateDescription(menuGenerator, ownerRegion, description)
        local wrapped = __wow_wrap_menu_descriptor(description)
        if type(menuGenerator) == "function" then
            pcall(menuGenerator, ownerRegion, wrapped)
        end
    end

    if MenuUtil == nil then MenuUtil = {} end
    function MenuUtil.CreateRootMenuDescription(menuMixin)
        return Menu.CreateRootMenuDescription(menuMixin)
    end
end

local function __wow_dropdown_button_name(owner, index)
    if type(owner.GetName) ~= "function" then
        return nil
    end
    local ok, name = pcall(owner.GetName, owner)
    if ok and type(name) == "string" and name ~= "" then
        return name .. "MenuButton" .. tostring(index)
    end
    return nil
end

local function __wow_dropdown_width(owner)
    if type(owner.GetWidth) == "function" then
        local ok, width = pcall(owner.GetWidth, owner)
        if ok and type(width) == "number" and width > 0 then
            return width
        end
    end
    return 160
end

local function __wow_dropdown_materialize_menu(owner, description)
    local previous = owner.__wow_menu_buttons
    if type(previous) == "table" then
        for _, button in ipairs(previous) do
            if button and type(button.Hide) == "function" then
                button:Hide()
            end
        end
    end

    local elements = type(description) == "table" and description.__wow_elements or nil
    if type(elements) ~= "table" then
        owner.__wow_menu_buttons = {}
        return
    end

    local buttons = {}
    local visible_index = 0
    local width = __wow_dropdown_width(owner)
    for _, element in ipairs(elements) do
        local text = type(element) == "table" and element.text or nil
        if type(text) == "string" and text ~= "" then
            visible_index = visible_index + 1
            local name = __wow_dropdown_button_name(owner, visible_index)
            local button = name and rawget(_G, name) or nil
            if button == nil then
                button = CreateFrame("Button", name, UIParent)
            end
            button:SetSize(width, 20)
            button:ClearAllPoints()
            button:SetPoint("TOPLEFT", owner, "BOTTOMLEFT", 0, -((visible_index - 1) * 20))
            button:SetText(text)
            button:Show()
            buttons[visible_index] = button
        end
    end
    owner.__wow_menu_buttons = buttons
end

local function __wow_dropdown_radio_is_selected(element)
    local args = type(element) == "table" and element.args or nil
    if type(args) ~= "table" or type(args[1]) ~= "function" then
        return false
    end
    local ok, selected = pcall(args[1], args[3])
    return ok and selected == true
end

local function __wow_dropdown_update_selection_text(owner, description)
    local elements = type(description) == "table" and description.__wow_elements or nil
    local text = nil
    if type(elements) == "table" then
        for _, element in ipairs(elements) do
            if type(element) == "table"
                and type(element.text) == "string"
                and element.text ~= ""
                and __wow_dropdown_radio_is_selected(element) then
                text = element.text
                break
            end
        end
        if text == nil
            and type(description) == "table"
            and description.tag == "MENU_COMMUNITIES_LIST" then
            for _, element in ipairs(elements) do
                if type(element) == "table" and type(element.text) == "string" and element.text ~= "" then
                    text = element.text
                    break
                end
            end
        end
    end
    if text == nil and type(owner.GetSelectionText) == "function" then
        local ok, selection_text = pcall(owner.GetSelectionText, owner)
        if ok and type(selection_text) == "string" and selection_text ~= "" then
            text = selection_text
        end
    end
    if text ~= nil and type(owner.SetText) == "function" then
        owner:SetText(text)
    end
    if text ~= nil and owner.Text ~= nil and type(owner.Text.SetText) == "function" then
        owner.Text:SetText(text)
    end
end

local function __wow_generate_dropdown_menu(owner)
    local description = nil
    if type(owner.__wow_menu_generator) == "function" then
        description = MenuUtil.CreateRootMenuDescription({})
        pcall(owner.__wow_menu_generator, owner, description)
    elseif type(owner.menuGenerator) == "function" then
        description = MenuUtil.CreateRootMenuDescription({})
        pcall(owner.menuGenerator, owner, description)
    elseif type(owner.__wow_menu_description) == "table" then
        description = owner.__wow_menu_description
    elseif type(owner.menuDescription) == "table" then
        description = owner.menuDescription
    end
    if type(description) == "table" then
        owner.__wow_menu_description = description
        owner.menuDescription = description
    end
    return description
end

local function __wow_patch_dropdown_button_mixin()
    if type(DropdownButtonMixin) ~= "table" then
        return
    end
    if rawget(DropdownButtonMixin, "__wow_menu_fallback_open_menu") == DropdownButtonMixin.OpenMenu
        and rawget(DropdownButtonMixin, "__wow_menu_fallback_generate_menu") == DropdownButtonMixin.GenerateMenu
        and rawget(DropdownButtonMixin, "__wow_menu_fallback_setup_menu") == DropdownButtonMixin.SetupMenu then
        return
    end

    local existing_setup_menu = DropdownButtonMixin.SetupMenu
    local existing_generate_menu = DropdownButtonMixin.GenerateMenu
    local existing_open_menu = DropdownButtonMixin.OpenMenu

    function DropdownButtonMixin:SetupMenu(generator)
        self.__wow_menu_generator = generator
        self.menuGenerator = generator
        if type(existing_setup_menu) == "function" then
            pcall(existing_setup_menu, self, generator)
        end
    end

    function DropdownButtonMixin:GenerateMenu()
        local description = nil
        if type(existing_generate_menu) == "function" then
            local ok, generated = pcall(existing_generate_menu, self)
            if ok and type(generated) == "table" then
                description = generated
            elseif ok and type(self.menuDescription) == "table" then
                description = self.menuDescription
            end
        end
        if type(description) ~= "table" then
            description = __wow_generate_dropdown_menu(self)
        end
        if type(description) == "table" then
            self.__wow_menu_description = description
            self.menuDescription = description
            __wow_dropdown_update_selection_text(self, description)
        end
        return description
    end

    function DropdownButtonMixin:OpenMenu()
        if type(existing_open_menu) == "function" then
            pcall(existing_open_menu, self)
        else
            self.__wow_menu_open = true
        end
        local description = self:GenerateMenu()
        __wow_dropdown_materialize_menu(self, description)
        self.__wow_menu_open = true
    end

    DropdownButtonMixin.__wow_menu_fallback_setup_menu = DropdownButtonMixin.SetupMenu
    DropdownButtonMixin.__wow_menu_fallback_generate_menu = DropdownButtonMixin.GenerateMenu
    DropdownButtonMixin.__wow_menu_fallback_open_menu = DropdownButtonMixin.OpenMenu
end

__wow_patch_dropdown_button_mixin()

if type(__wow_install_dropdown_button_mixin_patch) == "function" then
    __wow_install_dropdown_button_mixin_patch()
end
"#;

pub struct LoaderEnv<'a> {
    lua: Rc<std::cell::RefCell<rilua::Lua>>,
    state: Rc<std::cell::RefCell<SimState>>,
    current_state: Option<NonNull<LuaState>>,
    _marker: PhantomData<&'a WowLuaEnv>,
}

impl<'a> LoaderEnv<'a> {
    pub fn new(env: &'a WowLuaEnv) -> Self {
        Self {
            lua: Rc::clone(&env.lua),
            state: Rc::clone(&env.state),
            current_state: None,
            _marker: PhantomData,
        }
    }

    pub fn from_parts(
        lua: Rc<std::cell::RefCell<rilua::Lua>>,
        state: Rc<std::cell::RefCell<SimState>>,
    ) -> LoaderEnv<'static> {
        LoaderEnv {
            lua,
            state,
            current_state: None,
            _marker: PhantomData,
        }
    }

    pub fn from_parts_active(
        lua: Rc<std::cell::RefCell<rilua::Lua>>,
        state: Rc<std::cell::RefCell<SimState>>,
        current_state: &mut LuaState,
    ) -> LoaderEnv<'static> {
        LoaderEnv {
            lua,
            state,
            current_state: Some(NonNull::from(current_state)),
            _marker: PhantomData,
        }
    }

    fn load_dynamic_chunk_without_slots(
        state: &mut LuaState,
        code: &str,
        tag: &str,
    ) -> Result<rilua::Function> {
        let saved_slots = state.global_slots.take();
        let cache_tag = format!("{tag}-no-global-slots");
        let result = crate::loader::chunk_cache::load_chunk(state, code, &cache_tag)
            .map_err(|e| crate::Error::Other(e.to_string()));
        state.global_slots = saved_slots;
        result
    }

    pub fn with_state<T, E>(
        &self,
        f: impl FnOnce(&mut LuaState) -> std::result::Result<T, E>,
    ) -> std::result::Result<T, E> {
        match self.current_state {
            Some(mut current_state) => {
                let state = unsafe { current_state.as_mut() };
                f(state)
            }
            None => {
                let mut lua = self.lua.borrow_mut();
                f(lua.state_mut())
            }
        }
    }

    fn loading_addon_uses_secure_env(&self) -> bool {
        let state = self.state.borrow();
        state
            .loading_addon_index
            .and_then(|idx| state.addons.get(idx as usize))
            .map(|addon| addon.use_secure_env)
            .unwrap_or(false)
    }

    pub fn exec(&self, code: &str) -> Result<()> {
        self.with_state(|state| {
            let func = Self::load_dynamic_chunk_without_slots(state, code, "loader-exec")?;
            if self.loading_addon_uses_secure_env() {
                mark_secure_state(state, &func)?;
            }
            crate::lua_api::script_helpers::call_void_function_with_fallback_state(
                state,
                Val::Function(func.gc_ref()),
                &[],
            )
            .map_err(crate::Error::Other)?;
            Ok(())
        })
    }

    pub fn exec_with_varargs(
        &self,
        code: &str,
        name: &str,
        addon_name: &str,
        addon_table: Val,
    ) -> Result<()> {
        self.with_state(|state| {
            let saved_slots = state.global_slots.take();
            let func = LuaApiMut::load_bytes(state, code.as_bytes(), name)?;
            state.global_slots = saved_slots;
            let addon_name = create_string(state, addon_name);
            crate::lua_api::methods::call_function_state(
                state,
                Val::Function(func.gc_ref()),
                &[addon_name, addon_table],
            )?;
            Ok(())
        })
    }

    pub fn fire_event_with_args(&self, event: &str, args: &[Val]) -> Result<()> {
        let listeners = self.with_state(|state| {
            Ok::<Vec<u64>, crate::Error>(crate::lua_api::script_helpers::get_event_listeners(
                state, event,
            ))
        })?;
        for widget_id in listeners {
            let result: std::result::Result<(), crate::Error> = self.with_state(|state| {
                let handler =
                    crate::lua_api::script_helpers::get_script(state, widget_id, "OnEvent");
                let Some(handler) = handler else {
                    return Ok(());
                };
                let frame = crate::lua_api::methods::frame_ref(state, widget_id)?;
                let event_name = crate::lua_api::methods::create_string(state, event);
                let mut call_args = Vec::with_capacity(args.len() + 2);
                call_args.push(frame);
                call_args.push(event_name);
                call_args.extend_from_slice(args);
                if let Err(error) =
                    crate::lua_api::script_helpers::call_void_function_with_fallback_state(
                        state, handler, &call_args,
                    )
                {
                    call_error_handler_state(state, &error);
                }
                Ok(())
            });
            if let Err(error) = result {
                self.with_state(|state| {
                    call_error_handler_state(state, &error.to_string());
                    Ok::<(), crate::Error>(())
                })?;
            }
        }
        Ok(())
    }

    pub fn restore_post_cleanup_globals(&self) -> crate::Result<()> {
        let mut lua = self.lua.borrow_mut();
        super::globals::environment_restore::restore_post_cleanup_globals(
            &mut lua,
            Rc::clone(&self.state),
        )
    }

    /// Patch `UIParentManagedFrameMixin:OnShow` / `OnHide` to no-op when
    /// `self.layoutParent` is nil. Blizzard wires `layoutParent` via a
    /// `<KeyValue type="global">` that resolves eagerly against `_G`;
    /// in the sim that container may not exist yet when a frame
    /// inheriting the mixin fires OnHide, producing
    /// `attempt to index field 'layoutParent'`. Guarding the methods
    /// post-load lets those OnHide passes succeed silently.
    pub fn patch_managed_frame_mixin(&self) -> crate::Result<()> {
        self.exec(MANAGED_FRAME_MIXIN_PATCH_LUA)
    }

    /// Patch `UnitPositionFrameMixin:OnHide` to no-op when `self.dataProvider`
    /// is nil. `OnHide` fires before `SetDataProvider`/`OnAcquired` when a
    /// frame inheriting the mixin is hidden during initial load, producing
    /// `attempt to index field 'dataProvider' (a nil value)` at
    /// GroupMembersDataProvider.lua:90.
    pub fn patch_unit_position_frame_mixin(&self) -> crate::Result<()> {
        self.exec(UNIT_POSITION_FRAME_MIXIN_PATCH_LUA)
    }

    /// If `Blizzard_Menu` left `Menu.CreateRootMenuDescription` undefined
    /// (its top-level `do ... end` touches subsystems the sim doesn't
    /// fully implement), install a permissive descriptor fallback so
    /// downstream `MenuUtil.CreateRootMenuDescription(...)` doesn't blow
    /// up every dropdown-bearing frame.
    pub fn ensure_menu_descriptor_fallback(&self) -> crate::Result<()> {
        self.exec(MENU_DESCRIPTOR_FALLBACK_LUA)
    }

    /// Patch `QuestLogMixin:GetCurrentMapID` to guard against nil parent.
    /// During startup, `QUEST_LOG_UPDATE` fires before the QuestLog is
    /// parented to WorldMapFrame, causing `self:GetParent():IsShown()`
    /// at QuestMapFrame.lua:279 to error.
    pub fn patch_quest_log_mixin(&self) -> crate::Result<()> {
        self.exec(QUEST_LOG_MIXIN_PATCH_LUA)
    }

    pub fn create_addon_table(&self) -> Result<Val> {
        self.with_state(create_addon_table_state)
    }

    pub fn lua(&self) -> &Rc<std::cell::RefCell<rilua::Lua>> {
        &self.lua
    }

    pub fn rilua(&self) -> Ref<'_, rilua::Lua> {
        self.lua.borrow()
    }

    pub fn rilua_mut(&self) -> RefMut<'_, rilua::Lua> {
        self.lua.borrow_mut()
    }

    pub fn state(&self) -> &Rc<std::cell::RefCell<SimState>> {
        &self.state
    }
}

pub(crate) fn create_addon_table(lua: &mut rilua::Lua) -> Result<Val> {
    create_addon_table_state(lua.state_mut())
}

pub(crate) fn create_addon_table_state(state: &mut LuaState) -> Result<Val> {
    let table = create_table(state);
    let Val::Table(table_ref) = table else {
        unreachable!("create_table must return a table");
    };
    table_set_rust_fn_static(state, table_ref, "unpack", addon_table_unpack)?;
    Ok(table)
}

fn addon_table_unpack(state: &mut LuaState) -> rilua::LuaResult<u32> {
    let table = state.stack_get(state.base);
    let values = addon_table_values(state, table);
    for value in values {
        state.push(value);
    }
    Ok(4)
}

fn addon_table_values(state: &LuaState, table: Val) -> [Val; 4] {
    let Val::Table(table_ref) = table else {
        return [Val::Nil, Val::Nil, Val::Nil, Val::Nil];
    };
    let Some(table) = state.gc.tables.get(table_ref) else {
        return [Val::Nil, Val::Nil, Val::Nil, Val::Nil];
    };
    let values = table.array_slice();
    [
        values.first().copied().unwrap_or(Val::Nil),
        values.get(1).copied().unwrap_or(Val::Nil),
        values.get(2).copied().unwrap_or(Val::Nil),
        values.get(3).copied().unwrap_or(Val::Nil),
    ]
}
