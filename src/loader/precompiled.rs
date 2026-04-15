//! Precompiled Lua helper functions to eliminate repeated source compilation.
//!
//! The XML loader generates thousands of unique Lua code strings via `env.exec()` —
//! each compiled from source every load. This module precompiles parameterized helper
//! functions once at startup and stores them for reuse, eliminating ~12,000+ redundant
//! Lua compilation calls.

use mlua::{Function, Lua};

/// Precompiled Lua functions for the XML loader, stored in Lua app_data.
///
/// Each function is compiled once at startup and called with arguments instead of
/// generating and compiling unique Lua source strings for each frame.
pub struct PrecompiledFns {
    /// Fire OnLoad lifecycle script on a frame (by frame ref or global name).
    pub fire_onload: Function,
    /// Fire OnShow lifecycle script on a frame (by frame ref or global name).
    pub fire_onshow: Function,
    /// Increment the `__suppress_create_frame_onload` counter.
    pub suppress_push: Function,
    /// Decrement the `__suppress_create_frame_onload` counter.
    pub suppress_pop: Function,
    /// Assign `_G[parent_name][key] = _G[child_name]`.
    pub assign_parent_key: Function,
    /// Set `_G[frame_name].intrinsic = base_name`.
    pub set_intrinsic: Function,
    /// Append a child name to `__deferred_child_onloads` table.
    pub defer_onload: Function,
}

const FIRE_ONLOAD_SOURCE: &str = r#"
        local __report = debug.getregistry()["__report_script_error"]
        local reg = debug.getregistry()
        local function resolve_frame(arg)
            if type(arg) ~= "string" then
                return arg
            end
            local id = arg:match("^__frame_(%d+)$")
            if id then
                return reg.__frame_refs[tonumber(id)]
            end
            return _G[arg]
        end
        local arg = ...
        local frame = resolve_frame(arg)
        if not frame then return end
        if type(frame.OnLoad_Intrinsic) == "function" then
            local ok, err = pcall(frame.OnLoad_Intrinsic, frame)
            if not ok then
                __report("[OnLoad_Intrinsic] " .. tostring(err))
            end
        end
        local handler = frame:GetScript("OnLoad")
        if handler then
            local ok, err = pcall(handler, frame)
            if not ok then
                local name = frame.GetName and frame:GetName() or "?"
                __report("[OnLoad] " .. name .. ": " .. tostring(err))
            end
        end
    "#;

impl PrecompiledFns {
    /// Compile all helper functions once and return the struct.
    pub fn new(lua: &Lua) -> mlua::Result<Self> {
        Ok(Self {
            fire_onload: compile_fire_onload(lua)?,
            fire_onshow: compile_fire_onshow(lua)?,
            suppress_push: compile_suppress_push(lua)?,
            suppress_pop: compile_suppress_pop(lua)?,
            assign_parent_key: compile_assign_parent_key(lua)?,
            set_intrinsic: compile_set_intrinsic(lua)?,
            defer_onload: compile_defer_onload(lua)?,
        })
    }
}

fn compile_fire_onload(lua: &Lua) -> mlua::Result<Function> {
    compile_precompiled(lua, FIRE_ONLOAD_SOURCE)
}

fn compile_precompiled(lua: &Lua, source: &str) -> mlua::Result<Function> {
    lua.load(source).into_function()
}

fn compile_fire_onshow(lua: &Lua) -> mlua::Result<Function> {
    lua.load(
        r#"
        local __report = debug.getregistry()["__report_script_error"]
        local reg = debug.getregistry()
        local function resolve_frame(arg)
            if type(arg) ~= "string" then
                return arg
            end
            local id = arg:match("^__frame_(%d+)$")
            if id then
                return reg.__frame_refs[tonumber(id)]
            end
            return _G[arg]
        end
        local arg = ...
        local frame = resolve_frame(arg)
        if not frame then return end
        if frame:IsVisible() then
            local handler = frame:GetScript("OnShow")
            if handler then
                local ok, err = pcall(handler, frame)
                if not ok then
                    local name = frame.GetName and frame:GetName() or "?"
                    __report("[OnShow] " .. name .. ": " .. tostring(err))
                end
            end
            if type(frame.OnShow_Intrinsic) == "function" then
                local ok, err = pcall(frame.OnShow_Intrinsic, frame)
                if not ok then
                    __report("[OnShow_Intrinsic] " .. tostring(err))
                end
            end
        end
    "#,
    )
    .into_function()
}

fn compile_suppress_push(lua: &Lua) -> mlua::Result<Function> {
    lua.load("__suppress_create_frame_onload = (__suppress_create_frame_onload or 0) + 1")
        .into_function()
}

fn compile_suppress_pop(lua: &Lua) -> mlua::Result<Function> {
    lua.load("__suppress_create_frame_onload = __suppress_create_frame_onload - 1")
        .into_function()
}

fn compile_defer_onload(lua: &Lua) -> mlua::Result<Function> {
    lua.load(
        r#"
        local name = ...
        if not __deferred_child_onloads then
            __deferred_child_onloads = {}
        end
        __deferred_child_onloads[#__deferred_child_onloads + 1] = name
    "#,
    )
    .into_function()
}

fn compile_assign_parent_key(lua: &Lua) -> mlua::Result<Function> {
    lua.load(
        r#"
        local reg = debug.getregistry()
        local function resolve_frame(name)
            local id = name:match("^__frame_(%d+)$")
            if id then
                return reg.__frame_refs[tonumber(id)]
            end
            return _G[name]
        end
        local parent_name, key, child_name = ...
        local parent = resolve_frame(parent_name)
        local child = resolve_frame(child_name)
        if parent and child then
            if key:sub(1, 8) == "$parent." then
                parent = parent:GetParent()
                key = key:sub(9)
            end
            parent[key] = child
        end
    "#,
    )
    .into_function()
}

fn compile_set_intrinsic(lua: &Lua) -> mlua::Result<Function> {
    lua.load(
        r#"
        local reg = debug.getregistry()
        local frame_name, base = ...
        local id = frame_name:match("^__frame_(%d+)$")
        local frame = id and reg.__frame_refs[tonumber(id)] or _G[frame_name]
        if frame then
            frame.intrinsic = base
        end
    "#,
    )
    .into_function()
}

/// Initialize precompiled functions and store them in Lua app_data.
///
/// Must be called once during `WowLuaEnv::new()` after globals are registered
/// (since the functions reference `__report_script_error` etc.).
pub fn init(lua: &Lua) -> mlua::Result<()> {
    let fns = PrecompiledFns::new(lua)?;
    lua.set_app_data(fns);
    Ok(())
}

/// Retrieve the precompiled functions from Lua app_data.
///
/// Returns cloned `Function` handles (cheap Rc clone) to avoid holding
/// a `Ref<PrecompiledFns>` borrow across Lua calls.
pub fn get(lua: &Lua) -> PrecompiledFnsRef {
    let fns = lua
        .app_data_ref::<PrecompiledFns>()
        .expect("PrecompiledFns not initialized — call precompiled::init() first");
    PrecompiledFnsRef {
        fire_onload: fns.fire_onload.clone(),
        fire_onshow: fns.fire_onshow.clone(),
        suppress_push: fns.suppress_push.clone(),
        suppress_pop: fns.suppress_pop.clone(),
        assign_parent_key: fns.assign_parent_key.clone(),
        set_intrinsic: fns.set_intrinsic.clone(),
        defer_onload: fns.defer_onload.clone(),
    }
}

/// Retrieve precompiled functions when available.
pub fn try_get(lua: &Lua) -> Option<PrecompiledFnsRef> {
    let fns = lua.app_data_ref::<PrecompiledFns>()?;
    Some(PrecompiledFnsRef {
        fire_onload: fns.fire_onload.clone(),
        fire_onshow: fns.fire_onshow.clone(),
        suppress_push: fns.suppress_push.clone(),
        suppress_pop: fns.suppress_pop.clone(),
        assign_parent_key: fns.assign_parent_key.clone(),
        set_intrinsic: fns.set_intrinsic.clone(),
        defer_onload: fns.defer_onload.clone(),
    })
}

/// Owned copy of precompiled function handles (cheap Rc clones).
///
/// This avoids holding a `Ref<PrecompiledFns>` borrow across Lua calls.
pub struct PrecompiledFnsRef {
    pub fire_onload: Function,
    pub fire_onshow: Function,
    pub suppress_push: Function,
    pub suppress_pop: Function,
    pub assign_parent_key: Function,
    pub set_intrinsic: Function,
    pub defer_onload: Function,
}

#[cfg(test)]
mod tests {
    use super::*;
    use mlua::Table;

    #[test]
    fn fire_onload_precompiled_function_resolves_registry_frame_ids() -> mlua::Result<()> {
        let env = crate::lua_api::WowLuaEnv::new().expect("failed to create wow lua env");
        let lua = env.lua();
        lua.load(
            r#"
            reports = {}
            debug.getregistry()["__report_script_error"] = function(msg)
                table.insert(reports, msg)
            end
            local frame = {
                OnLoad_Intrinsic = function(self)
                    self.intrinsic_calls = (self.intrinsic_calls or 0) + 1
                end,
                GetScript = function(self, name)
                    if name == "OnLoad" then
                        return function(self)
                            self.onload_calls = (self.onload_calls or 0) + 1
                        end
                    end
                end,
                GetName = function(self)
                    return "PrecompiledTestFrame"
                end,
            }
            local reg = debug.getregistry()
            reg.__frame_refs = { [7] = frame }
            return frame
            "#,
        )
        .exec()?;

        let frame: Table = lua
            .load("return debug.getregistry().__frame_refs[7]")
            .eval()?;
        let fire_onload = compile_fire_onload(lua)?;
        fire_onload.call::<()>("__frame_7")?;

        assert_eq!(frame.get::<i64>("intrinsic_calls")?, 1);
        assert_eq!(frame.get::<i64>("onload_calls")?, 1);

        let reports: Table = lua.globals().get("reports")?;
        assert_eq!(reports.raw_len(), 0);
        Ok(())
    }
}
