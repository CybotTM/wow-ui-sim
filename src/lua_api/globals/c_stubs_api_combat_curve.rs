//! Curve and color-curve userdata support for combat stubs.

use mlua::{AnyUserData, Lua, MetaMethod, MultiValue, Result, UserData, UserDataMethods, Value};
use std::cell::RefCell;
use std::sync::atomic::{AtomicU64, Ordering};

static NEXT_CURVE_ID: AtomicU64 = AtomicU64::new(1);

/// Metamethod names that should return nil when accessed via __index on curve objects.
const METAMETHOD_NAMES: &[&str] = &["__eq", "__index", "__metatable", "__newindex", "__tostring"];

/// Registry key for the shared LuaCurveObject methods table.
const CURVE_METHODS_KEY: &str = "__lua_curve_object_methods";

/// Registry key for the shared LuaColorCurveObject methods table.
const COLOR_CURVE_METHODS_KEY: &str = "__lua_color_curve_object_methods";

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

fn curve_index(
    lua: &Lua,
    ud: AnyUserData,
    key: String,
    methods_key: &'static str,
) -> Result<Value> {
    if METAMETHOD_NAMES.contains(&key.as_str()) {
        return Ok(Value::Nil);
    }
    if let Ok(fields) = ud.user_value::<mlua::Table>() {
        let value: Value = fields.raw_get(key.as_str())?;
        if value != Value::Nil {
            return Ok(value);
        }
    }
    let methods: mlua::Table = lua.named_registry_value(methods_key)?;
    methods.raw_get(key.as_str())
}

fn curve_newindex(
    lua: &Lua,
    ud: AnyUserData,
    key: String,
    value: Value,
    methods_key: &'static str,
) -> Result<()> {
    if METAMETHOD_NAMES.contains(&key.as_str()) {
        return Err(mlua::Error::runtime(format!(
            "Attempted to assign to read-only key {}",
            key
        )));
    }
    let methods: mlua::Table = lua.named_registry_value(methods_key)?;
    let existing: Value = methods.raw_get(key.as_str())?;
    if existing != Value::Nil {
        return Err(mlua::Error::runtime(format!(
            "Attempted to assign to read-only key {}",
            key
        )));
    }
    let fields = ud.user_value::<mlua::Table>()?;
    fields.raw_set(key, value)?;
    Ok(())
}

impl UserData for LuaCurveObject {
    fn add_methods<M: UserDataMethods<Self>>(methods: &mut M) {
        methods.add_meta_function(
            MetaMethod::Index,
            |lua, (ud, key): (AnyUserData, String)| curve_index(lua, ud, key, CURVE_METHODS_KEY),
        );
        methods.add_meta_function(
            MetaMethod::NewIndex,
            |lua, (ud, key, value): (AnyUserData, String, Value)| {
                curve_newindex(lua, ud, key, value, CURVE_METHODS_KEY)
            },
        );
        methods.add_meta_method(MetaMethod::ToString, |_, this, ()| {
            Ok(format!("LuaCurveObject: 0x{:016x}", this.id))
        });
    }
}

impl UserData for LuaColorCurveObject {
    fn add_methods<M: UserDataMethods<Self>>(methods: &mut M) {
        methods.add_meta_function(
            MetaMethod::Index,
            |lua, (ud, key): (AnyUserData, String)| {
                curve_index(lua, ud, key, COLOR_CURVE_METHODS_KEY)
            },
        );
        methods.add_meta_function(
            MetaMethod::NewIndex,
            |lua, (ud, key, value): (AnyUserData, String, Value)| {
                curve_newindex(lua, ud, key, value, COLOR_CURVE_METHODS_KEY)
            },
        );
        methods.add_meta_method(MetaMethod::ToString, |_, this, ()| {
            Ok(format!("LuaColorCurveObject: 0x{:016x}", this.id))
        });
    }
}

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
            let new_ud = lua.create_userdata(new_curve)?;
            new_ud.set_user_value(lua.create_table()?)?;
            Ok(new_ud)
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
            let new_ud = lua.create_userdata(new_curve)?;
            new_ud.set_user_value(lua.create_table()?)?;
            Ok(new_ud)
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

/// C_CurveUtil - creates LuaCurveObject and LuaColorCurveObject for interpolation.
pub fn register_curve_support(lua: &Lua) -> Result<()> {
    let curve_methods = build_curve_methods(lua)?;
    lua.set_named_registry_value(CURVE_METHODS_KEY, curve_methods)?;

    let color_curve_methods = build_color_curve_methods(lua)?;
    lua.set_named_registry_value(COLOR_CURVE_METHODS_KEY, color_curve_methods)?;

    let table = lua.create_table()?;
    table.set(
        "CreateCurve",
        lua.create_function(|lua, ()| {
            let ud = lua.create_userdata(LuaCurveObject::new())?;
            ud.set_user_value(lua.create_table()?)?;
            Ok(ud)
        })?,
    )?;
    table.set(
        "CreateColorCurve",
        lua.create_function(|lua, ()| {
            let ud = lua.create_userdata(LuaColorCurveObject::new())?;
            ud.set_user_value(lua.create_table()?)?;
            Ok(ud)
        })?,
    )?;
    lua.globals().set("C_CurveUtil", table)?;
    Ok(())
}
