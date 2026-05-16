//! Temporary Lua proxy-object factories for unmodeled C API userdata-like objects.
//!
//! `C_CurveUtil` and `C_FunctionContainers` should eventually be backed by real
//! simulator-side object types. Until then, keep the table-shaped Lua
//! compatibility objects in the workaround layer instead of central runtime
//! bootstrap.

const PROXY_OBJECT_FACTORIES_LUA: &str = r#"
C_CurveUtil = C_CurveUtil or __wow_namespace({
  CreateCurve = nil,
  CreateColorCurve = nil,
})

C_FunctionContainers = C_FunctionContainers or __wow_namespace({
  CreateCallback = nil,
})

ProxyUtil = ProxyUtil or {}
ProxyConvertableMixin = ProxyConvertableMixin or {}
ProxyUtil.CreateProxy = ProxyUtil.CreateProxy or function(value) return value end
ProxyUtil.CreateProxyMixin = ProxyUtil.CreateProxyMixin or function() return {} end
ProxyUtil.SetPrivateReference = ProxyUtil.SetPrivateReference or __wow_noop
ProxyUtil.ReleasePrivateReference = ProxyUtil.ReleasePrivateReference or __wow_noop

if type(ProxyConvertableMixin.Init) ~= "function" then
  function ProxyConvertableMixin:Init(proxy, proxies, permitOverwrite)
    self.proxy = proxy or self
    if proxies and type(proxies.AddProxy) == "function" then
      proxies:AddProxy(self, permitOverwrite)
    end
    self.__proxy_tags = self.__proxy_tags or {}
    return self.__proxy_tags
  end
end

if type(ProxyConvertableMixin.ToProxy) ~= "function" then
  function ProxyConvertableMixin:ToProxy()
    return self.proxy or self
  end
end

if type(ProxyUtil.CreateProxyDirectory) ~= "function"
  or type(ProxyUtil.CreateProxyDirectory().AddProxy) ~= "function"
then
  function ProxyUtil.CreateProxyDirectory()
    local proxies = {
      __private_by_public = setmetatable({}, { __mode = "k" }),
      __public_by_private = setmetatable({}, { __mode = "k" }),
    }

    function proxies:AddProxy(object, _permitOverwrite)
      local public = object and type(object.ToProxy) == "function" and object:ToProxy() or object
      if public ~= nil then
        self.__private_by_public[public] = object
        self.__public_by_private[object] = public
      end
    end

    function proxies:RemoveProxy(public)
      local private = self.__private_by_public[public]
      self.__private_by_public[public] = nil
      if private ~= nil then
        self.__public_by_private[private] = nil
      end
    end

    function proxies:ToPrivate(public)
      return self.__private_by_public[public] or public
    end

    function proxies:ToPublic(private)
      return self.__public_by_private[private] or private
    end

    return proxies
  end
end

local __wow_proxy_object_id = 1

local function __wow_next_proxy_label(prefix)
  local label = prefix .. ":" .. tostring(__wow_proxy_object_id)
  __wow_proxy_object_id = __wow_proxy_object_id + 1
  return label
end

local function __wow_make_proxy_object(prefix, methods, initial_state)
  local object = initial_state or {}
  local label = __wow_next_proxy_label(prefix)
  return setmetatable(object, {
    __index = function(t, key)
      local value = rawget(t, key)
      if value ~= nil then
        return value
      end
      return methods[key]
    end,
    __newindex = function(t, key, value)
      if methods[key] ~= nil then
        error("read-only key: " .. tostring(key), 2)
      end
      rawset(t, key, value)
    end,
    __tostring = function()
      return label
    end,
  })
end

local function __wow_clone_proxy_points(points)
  local copy = {}
  for index = 1, #(points or {}) do
    local point = points[index]
    copy[index] = {
      x = point.x,
      y = point.y,
    }
  end
  return copy
end

local function __wow_copy_proxy_table(source)
  local copy = {}
  if type(source) ~= "table" then
    return copy
  end
  for key, value in pairs(source) do
    copy[key] = value
  end
  return copy
end

local function __wow_curve_methods(prefix)
  local methods = {}

  function methods:AddPoint(x, y)
    self.points[#self.points + 1] = { x = x or 0, y = y or 0 }
  end

  function methods:ClearPoints()
    self.points = {}
  end

  function methods:SetType(curveType)
    self.curveType = curveType or 0
  end

  function methods:GetPointCount()
    return #self.points
  end

  function methods:Evaluate(x)
    local points = self.points
    if #points == 0 then
      return 0
    end
    if #points == 1 then
      return points[1].y
    end

    local target = x or 0
    for index = 1, #points - 1 do
      local left = points[index]
      local right = points[index + 1]
      if target <= right.x then
        local dx = right.x - left.x
        if dx == 0 then
          return right.y
        end
        local fraction = (target - left.x) / dx
        return left.y + (right.y - left.y) * fraction
      end
    end

    return points[#points].y
  end

  function methods:Copy()
    return __wow_make_proxy_object(prefix, methods, {
      points = __wow_clone_proxy_points(self.points),
      curveType = self.curveType,
    })
  end

  return methods
end

if rawget(C_CurveUtil, "CreateCurve") == nil then
  local curveMethods = __wow_curve_methods("LuaCurveObject")
  function C_CurveUtil.CreateCurve()
    return __wow_make_proxy_object("LuaCurveObject", curveMethods, {
      points = {},
      curveType = 0,
    })
  end
end

if rawget(C_CurveUtil, "CreateColorCurve") == nil then
  local colorCurveMethods = __wow_curve_methods("LuaColorCurveObject")
  function C_CurveUtil.CreateColorCurve()
    return __wow_make_proxy_object("LuaColorCurveObject", colorCurveMethods, {
      points = {},
      curveType = 0,
    })
  end
end

if rawget(C_FunctionContainers, "CreateCallback") == nil then
  local functionContainerMethods = {}

  function functionContainerMethods:Cancel()
    self._cancelled = true
  end

  function functionContainerMethods:IsCancelled()
    return self._cancelled == true
  end

  function functionContainerMethods:Invoke(...)
    if self._cancelled or type(self._callback) ~= "function" then
      return nil
    end
    return self._callback(...)
  end

  function C_FunctionContainers.CreateCallback(fn)
    return __wow_make_proxy_object("LuaFunctionContainer", functionContainerMethods, {
      _callback = fn,
      _cancelled = false,
    })
  end
end

if CreateAbbreviateConfig == nil then
  local abbreviateMethods = {}

  function abbreviateMethods:GetAbbreviateNumberData()
    return self._abbreviateNumberData
  end

  function abbreviateMethods:SetAbbreviateNumberData(data)
    self._abbreviateNumberData = data
  end

  function CreateAbbreviateConfig(initial)
    local state = __wow_copy_proxy_table(initial)
    state._abbreviateNumberData = state._abbreviateNumberData
    return __wow_make_proxy_object("AbbreviateConfig", abbreviateMethods, state)
  end
end

if CreateUnitHealPredictionCalculator == nil then
  local healPredictionMethods = {}

  function healPredictionMethods:Reset()
    self._damageAbsorbClampMode = 0
    self._incomingHeals = 0
  end

  function healPredictionMethods:GetIncomingHeals()
    return self._incomingHeals or 0
  end

  function healPredictionMethods:GetDamageAbsorbClampMode()
    return self._damageAbsorbClampMode or 0
  end

  function healPredictionMethods:SetDamageAbsorbClampMode(mode)
    self._damageAbsorbClampMode = mode or 0
  end

  function CreateUnitHealPredictionCalculator()
    return __wow_make_proxy_object("UnitHealPredictionCalculator", healPredictionMethods, {
      _damageAbsorbClampMode = 0,
      _incomingHeals = 0,
    })
  end
end
"#;

pub(crate) fn apply_bootstrap(lua: &mut rilua::Lua) -> crate::Result<()> {
    lua.exec(PROXY_OBJECT_FACTORIES_LUA)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::lua_api::WowLuaEnv;

    #[test]
    fn installs_proxy_factories() {
        let env = WowLuaEnv::new().expect("lua env should initialize");

        let result: String = env
            .eval(
                r#"
                if type(C_CurveUtil.CreateCurve) ~= "function" then return "curve" end
                if type(C_CurveUtil.CreateColorCurve) ~= "function" then return "color_curve" end
                if type(C_FunctionContainers.CreateCallback) ~= "function" then return "callback" end
                if type(ProxyUtil.CreateProxy) ~= "function" then return "proxy" end
                if type(ProxyUtil.CreateProxyMixin) ~= "function" then return "proxy_mixin" end
                if type(ProxyUtil.CreateProxyDirectory) ~= "function" then return "proxy_directory" end
                if type(ProxyUtil.CreateProxyDirectory().AddProxy) ~= "function" then return "proxy_directory_add" end
                if type(ProxyConvertableMixin) ~= "table" then return "convertable_mixin" end
                if type(ProxyConvertableMixin.Init) ~= "function" then return "convertable_init" end
                if type(ProxyConvertableMixin.ToProxy) ~= "function" then return "convertable_to_proxy" end
                if type(CreateAbbreviateConfig) ~= "function" then return "abbreviate" end
                if type(CreateUnitHealPredictionCalculator) ~= "function" then return "heal_prediction" end
                local curve = C_CurveUtil.CreateCurve()
                curve:AddPoint(0, 10)
                curve:AddPoint(10, 20)
                if curve:Evaluate(5) ~= 15 then return "evaluate" end
                local callback = C_FunctionContainers.CreateCallback(function(value) return value + 1 end)
                if callback:Invoke(41) ~= 42 then return "invoke" end
                local value = { name = "proxy-value" }
                if ProxyUtil.CreateProxy(value) ~= value then return "proxy_identity" end
                local directory = ProxyUtil.CreateProxyDirectory()
                if directory:ToPrivate(value) ~= value then return "to_private" end
                if directory:ToPublic(value) ~= value then return "to_public" end
                local private = {}
                local public = {}
                private.ToProxy = ProxyConvertableMixin.ToProxy
                ProxyConvertableMixin.Init(private, public, directory)
                if directory:ToPrivate(public) ~= private then return "registered_private" end
                if directory:ToPublic(private) ~= public then return "registered_public" end
                local config = CreateAbbreviateConfig({})
                config:SetAbbreviateNumberData({ value = 1 })
                if config:GetAbbreviateNumberData().value ~= 1 then return "config" end
                local prediction = CreateUnitHealPredictionCalculator()
                prediction:SetDamageAbsorbClampMode(2)
                if prediction:GetDamageAbsorbClampMode() ~= 2 then return "prediction" end
                return "ok"
                "#,
            )
            .expect("proxy factory probe should run");

        assert_eq!(result, "ok");
    }
}
