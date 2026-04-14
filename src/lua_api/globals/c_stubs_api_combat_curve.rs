//! Curve and color-curve table-backed proxy support for combat stubs.

use crate::lua_api::proxy_helpers::{proxy_userdata, wrap_fn_with_userdata};
use mlua::{AnyUserData, Lua, MultiValue, Result, UserData, Value};
use std::cell::RefCell;
use std::sync::atomic::{AtomicU64, Ordering};

static NEXT_CURVE_ID: AtomicU64 = AtomicU64::new(1);

/// Metamethod names that are read-only (cannot be assigned by user code).
const METAMETHOD_NAMES: &[&str] = &["__eq", "__index", "__metatable", "__newindex", "__tostring"];

/// Registry key for the shared LuaCurveObject methods table.
const CURVE_METHODS_KEY: &str = "__lua_curve_object_methods";

/// Registry key for the shared LuaColorCurveObject methods table.
const COLOR_CURVE_METHODS_KEY: &str = "__lua_color_curve_object_methods";

/// Registry key for the curve proxy metatable.
const CURVE_PROXY_MT_KEY: &str = "__curve_proxy_mt";

/// Registry key for the color curve proxy metatable.
const COLOR_CURVE_PROXY_MT_KEY: &str = "__color_curve_proxy_mt";

/// Registry key for the shared bind-method helper (shared by both proxy types).
const BIND_METHOD_KEY: &str = "__curve_bind_method_helper";

/// A curve point for piecewise linear interpolation.
struct CurvePoint {
    x: f64,
    y: f64,
}

/// LuaCurveObject: WoW curve object for interpolation (scalar y values).
struct LuaCurveObject {
    id: u64,
    curve_type: RefCell<i32>,
    points: RefCell<Vec<CurvePoint>>,
}

impl LuaCurveObject {
    fn new() -> Self {
        Self {
            id: NEXT_CURVE_ID.fetch_add(1, Ordering::Relaxed),
            curve_type: RefCell::new(0),
            points: RefCell::new(Vec::new()),
        }
    }
}

impl UserData for LuaCurveObject {}

/// LuaColorCurveObject: WoW color curve object (4-component RGBA values per point).
struct LuaColorCurveObject {
    id: u64,
    curve_type: RefCell<i32>,
    points: RefCell<Vec<CurvePoint>>,
}

impl LuaColorCurveObject {
    fn new() -> Self {
        Self {
            id: NEXT_CURVE_ID.fetch_add(1, Ordering::Relaxed),
            curve_type: RefCell::new(0),
            points: RefCell::new(Vec::new()),
        }
    }
}

impl UserData for LuaColorCurveObject {}

/// Piecewise linear interpolation over sorted points.
fn interpolate(points: &[CurvePoint], x: f64) -> f64 {
    match points.len() {
        0 => 0.0,
        1 => points[0].y,
        _ => {
            if x <= points[0].x {
                return points[0].y;
            }
            if x >= points[points.len() - 1].x {
                return points[points.len() - 1].y;
            }
            for pair in points.windows(2) {
                if x >= pair[0].x && x <= pair[1].x {
                    let t = (x - pair[0].x) / (pair[1].x - pair[0].x);
                    return pair[0].y + t * (pair[1].y - pair[0].y);
                }
            }
            points[points.len() - 1].y
        }
    }
}

// ---------------------------------------------------------------------------
// Proxy infrastructure
// ---------------------------------------------------------------------------

fn register_bind_method_helper(lua: &Lua) -> Result<()> {
    if lua
        .named_registry_value::<mlua::Function>(BIND_METHOD_KEY)
        .is_ok()
    {
        return Ok(());
    }
    lua.set_named_registry_value(
        BIND_METHOD_KEY,
        crate::lua_api::cfunc_wrap::create_bind_factory(lua)?,
    )
}

fn ensure_proxy_support(lua: &Lua) -> Result<()> {
    register_bind_method_helper(lua)?;
    install_curve_proxy_metatable(lua)?;
    install_color_curve_proxy_metatable(lua)
}

fn install_curve_proxy_metatable(lua: &Lua) -> Result<()> {
    if lua
        .named_registry_value::<mlua::Table>(CURVE_PROXY_MT_KEY)
        .is_ok()
    {
        return Ok(());
    }
    let mt = create_proxy_metatable(lua, CURVE_METHODS_KEY, "LuaCurveObject")?;
    lua.set_named_registry_value(CURVE_PROXY_MT_KEY, mt)
}

fn install_color_curve_proxy_metatable(lua: &Lua) -> Result<()> {
    if lua
        .named_registry_value::<mlua::Table>(COLOR_CURVE_PROXY_MT_KEY)
        .is_ok()
    {
        return Ok(());
    }
    let mt = create_proxy_metatable(lua, COLOR_CURVE_METHODS_KEY, "LuaColorCurveObject")?;
    lua.set_named_registry_value(COLOR_CURVE_PROXY_MT_KEY, mt)
}

fn create_proxy_metatable(
    lua: &Lua,
    methods_key: &'static str,
    type_name: &'static str,
) -> Result<mlua::Table> {
    let mt = lua.create_table()?;
    mt.raw_set("__index", create_proxy_index(lua, methods_key)?)?;
    mt.raw_set("__newindex", create_proxy_newindex(lua, methods_key)?)?;
    mt.raw_set("__tostring", create_proxy_tostring(lua, type_name)?)?;
    Ok(mt)
}

fn create_proxy_index(lua: &Lua, methods_key: &'static str) -> Result<mlua::Function> {
    lua.create_function(move |lua, (this, key): (mlua::Table, Value)| {
        // Metamethod names always return nil.
        if let Value::String(ref s) = key {
            if s.to_string_lossy().starts_with("__") {
                return Ok(Value::Nil);
            }
        }

        let proxy_value = Value::Table(this);
        let Some(userdata) = proxy_userdata(&proxy_value) else {
            return Ok(Value::Nil);
        };

        // Check per-instance fields first.
        if let Ok(fields) = userdata.user_value::<mlua::Table>() {
            let field_value: Value = fields.raw_get(key.clone())?;
            if !field_value.is_nil() {
                return Ok(field_value);
            }
        }

        // Fall back to the shared methods table.
        if let Value::String(ref name) = key {
            let methods: mlua::Table = lua.named_registry_value(methods_key)?;
            let method: Value = methods.raw_get(name.clone())?;
            if let Value::Function(function) = method {
                return Ok(Value::Function(wrap_fn_with_userdata(
                    lua, function, userdata, BIND_METHOD_KEY,
                )?));
            }
        }

        Ok(Value::Nil)
    })
}

fn create_proxy_newindex(lua: &Lua, methods_key: &'static str) -> Result<mlua::Function> {
    lua.create_function(move |lua, (this, key, value): (mlua::Table, Value, Value)| {
        if let Value::String(ref s) = key {
            let key_str = s.to_string_lossy();
            // Reject metamethod names.
            if METAMETHOD_NAMES.contains(&key_str.as_ref()) {
                return Err(mlua::Error::runtime(format!(
                    "Attempted to assign to read-only key {}",
                    key_str
                )));
            }
            // Reject method names.
            let methods: mlua::Table = lua.named_registry_value(methods_key)?;
            let existing: Value = methods.raw_get(key.clone())?;
            if existing != Value::Nil {
                return Err(mlua::Error::runtime(format!(
                    "Attempted to assign to read-only key {}",
                    key_str
                )));
            }
        }

        let proxy_value = Value::Table(this);
        let Some(userdata) = proxy_userdata(&proxy_value) else {
            return Ok(());
        };
        let fields: mlua::Table = userdata.user_value()?;
        fields.raw_set(key, value)?;
        Ok(())
    })
}

fn create_proxy_tostring(lua: &Lua, type_name: &'static str) -> Result<mlua::Function> {
    lua.create_function(move |_, this: mlua::Table| {
        let proxy_value = Value::Table(this);
        let Some(userdata) = proxy_userdata(&proxy_value) else {
            return Ok(format!("{}: 0x0000000000000000", type_name));
        };
        // Try curve first, then color curve.
        let id = userdata
            .borrow::<LuaCurveObject>()
            .map(|c| c.id)
            .or_else(|_| userdata.borrow::<LuaColorCurveObject>().map(|c| c.id))
            .unwrap_or(0);
        Ok(format!("{}: 0x{:016x}", type_name, id))
    })
}

fn create_curve_proxy(lua: &Lua, curve: LuaCurveObject) -> Result<Value> {
    let userdata = lua.create_userdata(curve)?;
    userdata.set_user_value(lua.create_table()?)?;
    let proxy = lua.create_table()?;
    proxy.raw_set("__lud", userdata)?;
    let mt: mlua::Table = lua.named_registry_value(CURVE_PROXY_MT_KEY)?;
    proxy.set_metatable(Some(mt));
    Ok(Value::Table(proxy))
}

fn create_color_curve_proxy(lua: &Lua, curve: LuaColorCurveObject) -> Result<Value> {
    let userdata = lua.create_userdata(curve)?;
    userdata.set_user_value(lua.create_table()?)?;
    let proxy = lua.create_table()?;
    proxy.raw_set("__lud", userdata)?;
    let mt: mlua::Table = lua.named_registry_value(COLOR_CURVE_PROXY_MT_KEY)?;
    proxy.set_metatable(Some(mt));
    Ok(Value::Table(proxy))
}

// ---------------------------------------------------------------------------
// Curve method table builders
// ---------------------------------------------------------------------------

fn add_curve_add_clear(lua: &Lua, table: &mlua::Table) -> Result<()> {
    table.raw_set(
        "AddPoint",
        lua.create_function(|_, (ud, x, y): (AnyUserData, f64, f64)| {
            ud.borrow::<LuaCurveObject>()?
                .points
                .borrow_mut()
                .push(CurvePoint { x, y });
            Ok(())
        })?,
    )?;
    table.raw_set(
        "ClearPoints",
        lua.create_function(|_, ud: AnyUserData| {
            ud.borrow::<LuaCurveObject>()?.points.borrow_mut().clear();
            Ok(())
        })?,
    )?;
    Ok(())
}

fn add_curve_set_methods(lua: &Lua, table: &mlua::Table) -> Result<()> {
    table.raw_set(
        "RemovePoint",
        lua.create_function(|_, (ud, index): (AnyUserData, usize)| {
            let curve = ud.borrow::<LuaCurveObject>()?;
            let mut points = curve.points.borrow_mut();
            if index >= 1 && index <= points.len() {
                points.remove(index - 1);
            }
            Ok(())
        })?,
    )?;
    table.raw_set(
        "SetPoints",
        lua.create_function(|_, (ud, src): (AnyUserData, mlua::Table)| {
            let curve = ud.borrow::<LuaCurveObject>()?;
            let mut points = curve.points.borrow_mut();
            points.clear();
            for pair in src.sequence_values::<mlua::Table>().flatten() {
                points.push(CurvePoint {
                    x: pair.get("x").unwrap_or(0.0),
                    y: pair.get("y").unwrap_or(0.0),
                });
            }
            Ok(())
        })?,
    )?;
    table.raw_set(
        "SetToDefaults",
        lua.create_function(|_, ud: AnyUserData| {
            let curve = ud.borrow::<LuaCurveObject>()?;
            curve.points.borrow_mut().clear();
            *curve.curve_type.borrow_mut() = 0;
            Ok(())
        })?,
    )?;
    table.raw_set(
        "SetType",
        lua.create_function(|_, (ud, curve_type): (AnyUserData, i32)| {
            *ud.borrow::<LuaCurveObject>()?.curve_type.borrow_mut() = curve_type;
            Ok(())
        })?,
    )?;
    Ok(())
}

fn add_curve_copy_eval(lua: &Lua, table: &mlua::Table) -> Result<()> {
    table.raw_set(
        "Copy",
        lua.create_function(|lua, ud: AnyUserData| {
            let curve = ud.borrow::<LuaCurveObject>()?;
            let new_curve = LuaCurveObject {
                id: NEXT_CURVE_ID.fetch_add(1, Ordering::Relaxed),
                curve_type: RefCell::new(*curve.curve_type.borrow()),
                points: RefCell::new(
                    curve
                        .points
                        .borrow()
                        .iter()
                        .map(|point| CurvePoint {
                            x: point.x,
                            y: point.y,
                        })
                        .collect(),
                ),
            };
            drop(curve);
            create_curve_proxy(lua, new_curve)
        })?,
    )?;
    table.raw_set(
        "Evaluate",
        lua.create_function(|_, (ud, x): (AnyUserData, f64)| {
            Ok(interpolate(
                &ud.borrow::<LuaCurveObject>()?.points.borrow(),
                x,
            ))
        })?,
    )?;
    Ok(())
}

fn add_curve_get_methods(lua: &Lua, table: &mlua::Table) -> Result<()> {
    table.raw_set(
        "GetPoint",
        lua.create_function(|lua, (ud, index): (AnyUserData, usize)| {
            let curve = ud.borrow::<LuaCurveObject>()?;
            let points = curve.points.borrow();
            if index < 1 || index > points.len() {
                return Ok(Value::Nil);
            }
            let point = &points[index - 1];
            let point_table = lua.create_table()?;
            point_table.raw_set("x", point.x)?;
            point_table.raw_set("y", point.y)?;
            Ok(Value::Table(point_table))
        })?,
    )?;
    table.raw_set(
        "GetPointCount",
        lua.create_function(|_, ud: AnyUserData| {
            Ok(ud.borrow::<LuaCurveObject>()?.points.borrow().len())
        })?,
    )?;
    table.raw_set(
        "GetPoints",
        lua.create_function(|lua, ud: AnyUserData| {
            let curve = ud.borrow::<LuaCurveObject>()?;
            let point_table = lua.create_table()?;
            for (index, point) in curve.points.borrow().iter().enumerate() {
                let entry = lua.create_table()?;
                entry.raw_set("x", point.x)?;
                entry.raw_set("y", point.y)?;
                point_table.raw_set(index + 1, entry)?;
            }
            Ok(point_table)
        })?,
    )?;
    table.raw_set(
        "GetType",
        lua.create_function(|_, ud: AnyUserData| {
            Ok(*ud.borrow::<LuaCurveObject>()?.curve_type.borrow())
        })?,
    )?;
    table.raw_set(
        "HasSecretValues",
        lua.create_function(|_, _: AnyUserData| Ok(false))?,
    )?;
    Ok(())
}

fn build_curve_methods(lua: &Lua) -> Result<mlua::Table> {
    let table = lua.create_table()?;
    add_curve_add_clear(lua, &table)?;
    add_curve_set_methods(lua, &table)?;
    add_curve_copy_eval(lua, &table)?;
    add_curve_get_methods(lua, &table)?;
    Ok(table)
}

// ---------------------------------------------------------------------------
// Color curve method table builders
// ---------------------------------------------------------------------------

fn add_color_curve_basic(lua: &Lua, table: &mlua::Table) -> Result<()> {
    table.raw_set(
        "AddPoint",
        lua.create_function(|_, (ud, x, y): (AnyUserData, f64, f64)| {
            ud.borrow::<LuaColorCurveObject>()?
                .points
                .borrow_mut()
                .push(CurvePoint { x, y });
            Ok(())
        })?,
    )?;
    table.raw_set(
        "ClearPoints",
        lua.create_function(|_, ud: AnyUserData| {
            ud.borrow::<LuaColorCurveObject>()?
                .points
                .borrow_mut()
                .clear();
            Ok(())
        })?,
    )?;
    table.raw_set(
        "Copy",
        lua.create_function(|lua, ud: AnyUserData| {
            let curve = ud.borrow::<LuaColorCurveObject>()?;
            let new_curve = LuaColorCurveObject {
                id: NEXT_CURVE_ID.fetch_add(1, Ordering::Relaxed),
                curve_type: RefCell::new(*curve.curve_type.borrow()),
                points: RefCell::new(
                    curve
                        .points
                        .borrow()
                        .iter()
                        .map(|point| CurvePoint {
                            x: point.x,
                            y: point.y,
                        })
                        .collect(),
                ),
            };
            drop(curve);
            create_color_curve_proxy(lua, new_curve)
        })?,
    )?;
    Ok(())
}

fn add_color_curve_evaluate(lua: &Lua, table: &mlua::Table) -> Result<()> {
    table.raw_set(
        "Evaluate",
        lua.create_function(|lua, (ud, x): (AnyUserData, f64)| {
            let y = interpolate(&ud.borrow::<LuaColorCurveObject>()?.points.borrow(), x);
            let color = lua.create_table()?;
            color.raw_set("r", y)?;
            color.raw_set("g", y)?;
            color.raw_set("b", y)?;
            color.raw_set("a", 1.0_f64)?;
            Ok(Value::Table(color))
        })?,
    )?;
    table.raw_set(
        "EvaluateUnpacked",
        lua.create_function(|_, (ud, x): (AnyUserData, f64)| {
            let y = interpolate(&ud.borrow::<LuaColorCurveObject>()?.points.borrow(), x);
            Ok(MultiValue::from_iter([
                Value::Number(y),
                Value::Number(y),
                Value::Number(y),
                Value::Number(1.0),
            ]))
        })?,
    )?;
    Ok(())
}

fn add_color_curve_get_methods(lua: &Lua, table: &mlua::Table) -> Result<()> {
    table.raw_set(
        "GetPoint",
        lua.create_function(|lua, (ud, index): (AnyUserData, usize)| {
            let curve = ud.borrow::<LuaColorCurveObject>()?;
            let points = curve.points.borrow();
            if index < 1 || index > points.len() {
                return Ok(Value::Nil);
            }
            let point = &points[index - 1];
            let point_table = lua.create_table()?;
            point_table.raw_set("x", point.x)?;
            point_table.raw_set("y", point.y)?;
            Ok(Value::Table(point_table))
        })?,
    )?;
    table.raw_set(
        "GetPointCount",
        lua.create_function(|_, ud: AnyUserData| {
            Ok(ud.borrow::<LuaColorCurveObject>()?.points.borrow().len())
        })?,
    )?;
    table.raw_set(
        "GetPoints",
        lua.create_function(|lua, ud: AnyUserData| {
            let curve = ud.borrow::<LuaColorCurveObject>()?;
            let point_table = lua.create_table()?;
            for (index, point) in curve.points.borrow().iter().enumerate() {
                let entry = lua.create_table()?;
                entry.raw_set("x", point.x)?;
                entry.raw_set("y", point.y)?;
                point_table.raw_set(index + 1, entry)?;
            }
            Ok(point_table)
        })?,
    )?;
    table.raw_set(
        "GetType",
        lua.create_function(|_, ud: AnyUserData| {
            Ok(*ud.borrow::<LuaColorCurveObject>()?.curve_type.borrow())
        })?,
    )?;
    table.raw_set(
        "HasSecretValues",
        lua.create_function(|_, _: AnyUserData| Ok(false))?,
    )?;
    Ok(())
}

fn add_color_curve_set_methods(lua: &Lua, table: &mlua::Table) -> Result<()> {
    table.raw_set(
        "RemovePoint",
        lua.create_function(|_, (ud, index): (AnyUserData, usize)| {
            let curve = ud.borrow::<LuaColorCurveObject>()?;
            let mut points = curve.points.borrow_mut();
            if index >= 1 && index <= points.len() {
                points.remove(index - 1);
            }
            Ok(())
        })?,
    )?;
    table.raw_set(
        "SetPoints",
        lua.create_function(|_, (ud, src): (AnyUserData, mlua::Table)| {
            let curve = ud.borrow::<LuaColorCurveObject>()?;
            let mut points = curve.points.borrow_mut();
            points.clear();
            for pair in src.sequence_values::<mlua::Table>().flatten() {
                points.push(CurvePoint {
                    x: pair.get("x").unwrap_or(0.0),
                    y: pair.get("y").unwrap_or(0.0),
                });
            }
            Ok(())
        })?,
    )?;
    table.raw_set(
        "SetToDefaults",
        lua.create_function(|_, ud: AnyUserData| {
            let curve = ud.borrow::<LuaColorCurveObject>()?;
            curve.points.borrow_mut().clear();
            *curve.curve_type.borrow_mut() = 0;
            Ok(())
        })?,
    )?;
    table.raw_set(
        "SetType",
        lua.create_function(|_, (ud, curve_type): (AnyUserData, i32)| {
            *ud.borrow::<LuaColorCurveObject>()?.curve_type.borrow_mut() = curve_type;
            Ok(())
        })?,
    )?;
    Ok(())
}

fn build_color_curve_methods(lua: &Lua) -> Result<mlua::Table> {
    let table = lua.create_table()?;
    add_color_curve_basic(lua, &table)?;
    add_color_curve_evaluate(lua, &table)?;
    add_color_curve_get_methods(lua, &table)?;
    add_color_curve_set_methods(lua, &table)?;
    Ok(table)
}

// ---------------------------------------------------------------------------
// Registration
// ---------------------------------------------------------------------------

/// C_CurveUtil - creates LuaCurveObject and LuaColorCurveObject for interpolation.
pub fn register_curve_support(lua: &Lua) -> Result<()> {
    let curve_methods = build_curve_methods(lua)?;
    lua.set_named_registry_value(CURVE_METHODS_KEY, curve_methods)?;

    let color_curve_methods = build_color_curve_methods(lua)?;
    lua.set_named_registry_value(COLOR_CURVE_METHODS_KEY, color_curve_methods)?;

    ensure_proxy_support(lua)?;

    let table = lua.create_table()?;
    table.set(
        "CreateCurve",
        lua.create_function(|lua, ()| {
            ensure_proxy_support(lua)?;
            create_curve_proxy(lua, LuaCurveObject::new())
        })?,
    )?;
    table.set(
        "CreateColorCurve",
        lua.create_function(|lua, ()| {
            ensure_proxy_support(lua)?;
            create_color_curve_proxy(lua, LuaColorCurveObject::new())
        })?,
    )?;
    lua.globals().set("C_CurveUtil", table)?;
    Ok(())
}
