//! Combat, color, curve, and encounter-related C_* namespace stubs.
//!
//! Split from c_stubs_api_extra.rs to keep file sizes manageable.
//! Contains: C_ColorUtil, C_CombatLog, C_CurveUtil, C_EncounterTimeline,
//! C_RestrictedActions, C_TransmogOutfitInfo, Constants.EncounterTimelineIconMasks.

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

/// Register all combat/encounter-related stubs.
pub fn register_combat_stubs(lua: &Lua) -> Result<()> {
    let g = lua.globals();
    register_c_curve_util(lua)?;
    register_c_color_util(lua, &g)?;
    register_c_combat_log(lua, &g)?;
    register_c_restricted_transmog(lua, &g)?;
    register_encounter_timeline_constants(lua, &g)?;
    register_c_damage_meter(lua, &g)?;
    register_c_combat_text(lua, &g)?;
    register_c_combat_audio_alert(lua, &g)?;
    register_c_housing_photo_sharing(lua, &g)?;
    register_nameplate_constants(lua)?;
    register_c_death_recap(lua, &g)?;
    register_c_encounter_timeline(lua, &g)?;
    Ok(())
}

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
fn interpolate(pts: &[CurvePoint], x: f64) -> f64 {
    match pts.len() {
        0 => 0.0,
        1 => pts[0].y,
        _ => {
            if x <= pts[0].x { return pts[0].y; }
            if x >= pts[pts.len() - 1].x { return pts[pts.len() - 1].y; }
            for pair in pts.windows(2) {
                if x >= pair[0].x && x <= pair[1].x {
                    let t = (x - pair[0].x) / (pair[1].x - pair[0].x);
                    return pair[0].y + t * (pair[1].y - pair[0].y);
                }
            }
            pts[pts.len() - 1].y
        }
    }
}

/// Build a __index handler for curve objects.
///
/// Checks (in order):
/// 1. Metamethod names → nil (not exposed as indexable fields)
/// 2. Per-instance user_value table (for arbitrary field storage)
/// 3. Shared methods table from the registry (for WoW API methods)
fn curve_index(lua: &Lua, ud: AnyUserData, key: String, methods_key: &'static str) -> Result<Value> {
    if METAMETHOD_NAMES.contains(&key.as_str()) {
        return Ok(Value::Nil);
    }
    // Check per-instance field storage first
    if let Ok(fields) = ud.user_value::<mlua::Table>() {
        let v: Value = fields.raw_get(key.as_str())?;
        if v != Value::Nil {
            return Ok(v);
        }
    }
    // Fall back to shared methods table
    let methods: mlua::Table = lua.named_registry_value(methods_key)?;
    methods.raw_get(key.as_str())
}

/// Build a __newindex handler for curve objects.
///
/// - Errors with "Attempted to assign to read-only key X" for method names and metamethods
/// - Stores all other key/value pairs in the per-instance user_value table
fn curve_newindex(lua: &Lua, ud: AnyUserData, key: String, value: Value, methods_key: &'static str) -> Result<()> {
    // Block metamethod assignment
    if METAMETHOD_NAMES.contains(&key.as_str()) {
        return Err(mlua::Error::runtime(format!("Attempted to assign to read-only key {}", key)));
    }
    let methods: mlua::Table = lua.named_registry_value(methods_key)?;
    let existing: Value = methods.raw_get(key.as_str())?;
    if existing != Value::Nil {
        return Err(mlua::Error::runtime(format!("Attempted to assign to read-only key {}", key)));
    }
    let fields = ud.user_value::<mlua::Table>()?;
    fields.raw_set(key, value)?;
    Ok(())
}

impl UserData for LuaCurveObject {
    fn add_methods<M: UserDataMethods<Self>>(methods: &mut M) {
        methods.add_meta_function(
            MetaMethod::Index,
            |lua, (ud, key): (AnyUserData, String)| {
                curve_index(lua, ud, key, CURVE_METHODS_KEY)
            },
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

/// Add AddPoint and ClearPoints to the LuaCurveObject methods table.
fn add_curve_add_clear(lua: &Lua, t: &mlua::Table) -> Result<()> {
    t.raw_set("AddPoint", lua.create_function(|_, (ud, x, y): (AnyUserData, f64, f64)| {
        ud.borrow::<LuaCurveObject>()?.points.borrow_mut().push(CurvePoint { x, y });
        Ok(())
    })?)?;
    t.raw_set("ClearPoints", lua.create_function(|_, ud: AnyUserData| {
        ud.borrow::<LuaCurveObject>()?.points.borrow_mut().clear();
        Ok(())
    })?)?;
    Ok(())
}

/// Add RemovePoint, SetPoints, SetToDefaults, SetType to the LuaCurveObject methods table.
fn add_curve_set_methods(lua: &Lua, t: &mlua::Table) -> Result<()> {
    t.raw_set("RemovePoint", lua.create_function(|_, (ud, index): (AnyUserData, usize)| {
        let obj = ud.borrow::<LuaCurveObject>()?;
        let mut pts = obj.points.borrow_mut();
        if index >= 1 && index <= pts.len() { pts.remove(index - 1); }
        Ok(())
    })?)?;
    t.raw_set("SetPoints", lua.create_function(|_, (ud, src): (AnyUserData, mlua::Table)| {
        let obj = ud.borrow::<LuaCurveObject>()?;
        let mut pts = obj.points.borrow_mut();
        pts.clear();
        for pair in src.sequence_values::<mlua::Table>().flatten() {
            pts.push(CurvePoint { x: pair.get("x").unwrap_or(0.0), y: pair.get("y").unwrap_or(0.0) });
        }
        Ok(())
    })?)?;
    t.raw_set("SetToDefaults", lua.create_function(|_, ud: AnyUserData| {
        let obj = ud.borrow::<LuaCurveObject>()?;
        obj.points.borrow_mut().clear();
        *obj.curve_type.borrow_mut() = 0;
        Ok(())
    })?)?;
    t.raw_set("SetType", lua.create_function(|_, (ud, ty): (AnyUserData, i32)| {
        *ud.borrow::<LuaCurveObject>()?.curve_type.borrow_mut() = ty;
        Ok(())
    })?)?;
    Ok(())
}

/// Add Copy and Evaluate to the LuaCurveObject methods table.
fn add_curve_copy_eval(lua: &Lua, t: &mlua::Table) -> Result<()> {
    t.raw_set("Copy", lua.create_function(|lua, ud: AnyUserData| {
        let obj = ud.borrow::<LuaCurveObject>()?;
        let new_obj = LuaCurveObject {
            id: NEXT_CURVE_ID.fetch_add(1, Ordering::Relaxed),
            curve_type: RefCell::new(*obj.curve_type.borrow()),
            points: RefCell::new(obj.points.borrow().iter().map(|p| CurvePoint { x: p.x, y: p.y }).collect()),
        };
        drop(obj);
        let new_ud = lua.create_userdata(new_obj)?;
        new_ud.set_user_value(lua.create_table()?)?;
        Ok(new_ud)
    })?)?;
    t.raw_set("Evaluate", lua.create_function(|_, (ud, x): (AnyUserData, f64)| {
        Ok(interpolate(&ud.borrow::<LuaCurveObject>()?.points.borrow(), x))
    })?)?;
    Ok(())
}

/// Add GetPoint, GetPointCount, GetPoints, GetType, HasSecretValues to the LuaCurveObject table.
fn add_curve_get_methods(lua: &Lua, t: &mlua::Table) -> Result<()> {
    t.raw_set("GetPoint", lua.create_function(|lua, (ud, index): (AnyUserData, usize)| {
        let obj = ud.borrow::<LuaCurveObject>()?;
        let pts = obj.points.borrow();
        if index < 1 || index > pts.len() { return Ok(Value::Nil); }
        let p = &pts[index - 1];
        let tbl = lua.create_table()?;
        tbl.raw_set("x", p.x)?;
        tbl.raw_set("y", p.y)?;
        Ok(Value::Table(tbl))
    })?)?;
    t.raw_set("GetPointCount", lua.create_function(|_, ud: AnyUserData| {
        Ok(ud.borrow::<LuaCurveObject>()?.points.borrow().len())
    })?)?;
    t.raw_set("GetPoints", lua.create_function(|lua, ud: AnyUserData| {
        let obj = ud.borrow::<LuaCurveObject>()?;
        let tbl = lua.create_table()?;
        for (i, p) in obj.points.borrow().iter().enumerate() {
            let pt = lua.create_table()?;
            pt.raw_set("x", p.x)?;
            pt.raw_set("y", p.y)?;
            tbl.raw_set(i + 1, pt)?;
        }
        Ok(tbl)
    })?)?;
    t.raw_set("GetType", lua.create_function(|_, ud: AnyUserData| {
        Ok(*ud.borrow::<LuaCurveObject>()?.curve_type.borrow())
    })?)?;
    t.raw_set("HasSecretValues", lua.create_function(|_, _: AnyUserData| Ok(false))?)?;
    Ok(())
}

/// Build the shared methods table for LuaCurveObject.
fn build_curve_methods(lua: &Lua) -> Result<mlua::Table> {
    let t = lua.create_table()?;
    add_curve_add_clear(lua, &t)?;
    add_curve_set_methods(lua, &t)?;
    add_curve_copy_eval(lua, &t)?;
    add_curve_get_methods(lua, &t)?;
    Ok(t)
}

/// Add AddPoint, ClearPoints, Copy to the LuaColorCurveObject methods table.
fn add_color_curve_basic(lua: &Lua, t: &mlua::Table) -> Result<()> {
    t.raw_set("AddPoint", lua.create_function(|_, (ud, x, y): (AnyUserData, f64, f64)| {
        ud.borrow::<LuaColorCurveObject>()?.points.borrow_mut().push(CurvePoint { x, y });
        Ok(())
    })?)?;
    t.raw_set("ClearPoints", lua.create_function(|_, ud: AnyUserData| {
        ud.borrow::<LuaColorCurveObject>()?.points.borrow_mut().clear();
        Ok(())
    })?)?;
    t.raw_set("Copy", lua.create_function(|lua, ud: AnyUserData| {
        let obj = ud.borrow::<LuaColorCurveObject>()?;
        let new_obj = LuaColorCurveObject {
            id: NEXT_CURVE_ID.fetch_add(1, Ordering::Relaxed),
            curve_type: RefCell::new(*obj.curve_type.borrow()),
            points: RefCell::new(obj.points.borrow().iter().map(|p| CurvePoint { x: p.x, y: p.y }).collect()),
        };
        drop(obj);
        let new_ud = lua.create_userdata(new_obj)?;
        new_ud.set_user_value(lua.create_table()?)?;
        Ok(new_ud)
    })?)?;
    Ok(())
}

/// Add Evaluate and EvaluateUnpacked to the LuaColorCurveObject methods table.
fn add_color_curve_evaluate(lua: &Lua, t: &mlua::Table) -> Result<()> {
    t.raw_set("Evaluate", lua.create_function(|lua, (ud, x): (AnyUserData, f64)| {
        let y = interpolate(&ud.borrow::<LuaColorCurveObject>()?.points.borrow(), x);
        let tbl = lua.create_table()?;
        tbl.raw_set("r", y)?;
        tbl.raw_set("g", y)?;
        tbl.raw_set("b", y)?;
        tbl.raw_set("a", 1.0_f64)?;
        Ok(Value::Table(tbl))
    })?)?;
    t.raw_set("EvaluateUnpacked", lua.create_function(|_, (ud, x): (AnyUserData, f64)| {
        let y = interpolate(&ud.borrow::<LuaColorCurveObject>()?.points.borrow(), x);
        Ok(MultiValue::from_iter([Value::Number(y), Value::Number(y), Value::Number(y), Value::Number(1.0)]))
    })?)?;
    Ok(())
}

/// Add GetPoint, GetPointCount, GetPoints, GetType, HasSecretValues to LuaColorCurveObject table.
fn add_color_curve_get_methods(lua: &Lua, t: &mlua::Table) -> Result<()> {
    t.raw_set("GetPoint", lua.create_function(|lua, (ud, index): (AnyUserData, usize)| {
        let obj = ud.borrow::<LuaColorCurveObject>()?;
        let pts = obj.points.borrow();
        if index < 1 || index > pts.len() { return Ok(Value::Nil); }
        let p = &pts[index - 1];
        let tbl = lua.create_table()?;
        tbl.raw_set("x", p.x)?;
        tbl.raw_set("y", p.y)?;
        Ok(Value::Table(tbl))
    })?)?;
    t.raw_set("GetPointCount", lua.create_function(|_, ud: AnyUserData| {
        Ok(ud.borrow::<LuaColorCurveObject>()?.points.borrow().len())
    })?)?;
    t.raw_set("GetPoints", lua.create_function(|lua, ud: AnyUserData| {
        let obj = ud.borrow::<LuaColorCurveObject>()?;
        let tbl = lua.create_table()?;
        for (i, p) in obj.points.borrow().iter().enumerate() {
            let pt = lua.create_table()?;
            pt.raw_set("x", p.x)?;
            pt.raw_set("y", p.y)?;
            tbl.raw_set(i + 1, pt)?;
        }
        Ok(tbl)
    })?)?;
    t.raw_set("GetType", lua.create_function(|_, ud: AnyUserData| {
        Ok(*ud.borrow::<LuaColorCurveObject>()?.curve_type.borrow())
    })?)?;
    t.raw_set("HasSecretValues", lua.create_function(|_, _: AnyUserData| Ok(false))?)?;
    Ok(())
}

/// Add RemovePoint, SetPoints, SetToDefaults, SetType to LuaColorCurveObject methods table.
fn add_color_curve_set_methods(lua: &Lua, t: &mlua::Table) -> Result<()> {
    t.raw_set("RemovePoint", lua.create_function(|_, (ud, index): (AnyUserData, usize)| {
        let obj = ud.borrow::<LuaColorCurveObject>()?;
        let mut pts = obj.points.borrow_mut();
        if index >= 1 && index <= pts.len() { pts.remove(index - 1); }
        Ok(())
    })?)?;
    t.raw_set("SetPoints", lua.create_function(|_, (ud, src): (AnyUserData, mlua::Table)| {
        let obj = ud.borrow::<LuaColorCurveObject>()?;
        let mut pts = obj.points.borrow_mut();
        pts.clear();
        for pair in src.sequence_values::<mlua::Table>().flatten() {
            pts.push(CurvePoint { x: pair.get("x").unwrap_or(0.0), y: pair.get("y").unwrap_or(0.0) });
        }
        Ok(())
    })?)?;
    t.raw_set("SetToDefaults", lua.create_function(|_, ud: AnyUserData| {
        let obj = ud.borrow::<LuaColorCurveObject>()?;
        obj.points.borrow_mut().clear();
        *obj.curve_type.borrow_mut() = 0;
        Ok(())
    })?)?;
    t.raw_set("SetType", lua.create_function(|_, (ud, ty): (AnyUserData, i32)| {
        *ud.borrow::<LuaColorCurveObject>()?.curve_type.borrow_mut() = ty;
        Ok(())
    })?)?;
    Ok(())
}

/// Build the shared methods table for LuaColorCurveObject.
fn build_color_curve_methods(lua: &Lua) -> Result<mlua::Table> {
    let t = lua.create_table()?;
    add_color_curve_basic(lua, &t)?;
    add_color_curve_evaluate(lua, &t)?;
    add_color_curve_get_methods(lua, &t)?;
    add_color_curve_set_methods(lua, &t)?;
    Ok(t)
}

/// C_CurveUtil - creates LuaCurveObject and LuaColorCurveObject for interpolation.
///
/// Both types expose the full WoW API: AddPoint, ClearPoints, Copy, Evaluate, GetPoint,
/// GetPointCount, GetPoints, GetType, HasSecretValues, RemovePoint, SetPoints, SetToDefaults,
/// SetType. LuaColorCurveObject additionally has EvaluateUnpacked.
fn register_c_curve_util(lua: &Lua) -> Result<()> {
    let curve_methods = build_curve_methods(lua)?;
    lua.set_named_registry_value(CURVE_METHODS_KEY, curve_methods)?;

    let color_curve_methods = build_color_curve_methods(lua)?;
    lua.set_named_registry_value(COLOR_CURVE_METHODS_KEY, color_curve_methods)?;

    let t = lua.create_table()?;

    t.set("CreateCurve", lua.create_function(|lua, ()| {
        let ud = lua.create_userdata(LuaCurveObject::new())?;
        ud.set_user_value(lua.create_table()?)?;
        Ok(ud)
    })?)?;

    t.set("CreateColorCurve", lua.create_function(|lua, ()| {
        let ud = lua.create_userdata(LuaColorCurveObject::new())?;
        ud.set_user_value(lua.create_table()?)?;
        Ok(ud)
    })?)?;

    lua.globals().set("C_CurveUtil", t)?;
    Ok(())
}

/// C_ColorUtil - hex color formatting for ColorMixin.
fn register_c_color_util(lua: &Lua, g: &mlua::Table) -> Result<()> {
    let cu = lua.create_table()?;
    cu.set("GenerateTextColorCode", lua.create_function(|_, color: mlua::Table| {
        let r: f64 = color.get("r").unwrap_or(1.0);
        let g: f64 = color.get("g").unwrap_or(1.0);
        let b: f64 = color.get("b").unwrap_or(1.0);
        let a: f64 = color.get("a").unwrap_or(1.0);
        Ok(format!("{:02X}{:02X}{:02X}{:02X}",
            (a * 255.0) as u8, (r * 255.0) as u8,
            (g * 255.0) as u8, (b * 255.0) as u8))
    })?)?;
    cu.set("WrapTextInColor", lua.create_function(|_, (text, color): (String, mlua::Table)| {
        let r: f64 = color.get("r").unwrap_or(1.0);
        let g: f64 = color.get("g").unwrap_or(1.0);
        let b: f64 = color.get("b").unwrap_or(1.0);
        let a: f64 = color.get("a").unwrap_or(1.0);
        let hex = format!("{:02X}{:02X}{:02X}{:02X}",
            (a * 255.0) as u8, (r * 255.0) as u8,
            (g * 255.0) as u8, (b * 255.0) as u8);
        Ok(format!("|c{hex}{text}|r"))
    })?)?;
    g.set("C_ColorUtil", cu)?;
    Ok(())
}

/// C_CombatLog - combat log API (relocated from global functions in modern WoW).
fn register_c_combat_log(lua: &Lua, g: &mlua::Table) -> Result<()> {
    let cl = lua.create_table()?;
    cl.set("DoesObjectMatchFilter", lua.create_function(|_, (unit_flags, mask): (i64, i64)| {
        Ok(unit_flags & mask != 0)
    })?)?;
    cl.set("AddEventFilter", lua.create_function(|_, (_ev, _src, _dst): (Value, Value, Value)| Ok(()))?)?;
    cl.set("ClearEntries", lua.create_function(|_, ()| Ok(()))?)?;
    cl.set("GetCurrentEntryInfo", lua.create_function(|_, ()| Ok(0i32))?)?;
    cl.set("GetCurrentEventInfo", lua.create_function(|_, ()| Ok(Value::Nil))?)?;
    cl.set("GetEntryCount", lua.create_function(|_, ()| Ok(0i32))?)?;
    cl.set("ShowCurrentEntry", lua.create_function(|_, ()| Ok(false))?)?;
    cl.set("AdvanceEntry", lua.create_function(|_, _delta: Value| Ok(false))?)?;
    cl.set("GetRetentionTime", lua.create_function(|_, ()| Ok(300.0f64))?)?;
    cl.set("SetRetentionTime", lua.create_function(|_, _time: Value| Ok(()))?)?;
    cl.set("ResetFilter", lua.create_function(|_, ()| Ok(()))?)?;
    cl.set("SetCurrentEntry", lua.create_function(|_, _index: Value| Ok(()))?)?;
    cl.set("ApplyFilterSettings", lua.create_function(|_, _settings: Value| Ok(()))?)?;
    cl.set("RefilterEntries", lua.create_function(|_, ()| Ok(()))?)?;
    g.set("C_CombatLog", cl)?;
    Ok(())
}

/// C_RestrictedActions, C_TransmogOutfitInfo stubs.
fn register_c_restricted_transmog(lua: &Lua, g: &mlua::Table) -> Result<()> {
    let ra = lua.create_table()?;
    ra.set("CheckAllowProtectedFunctions", lua.create_function(|_, ()| Ok(true))?)?;
    g.set("C_RestrictedActions", ra)?;

    let toi = lua.create_table()?;
    toi.set("GetOutfitInfoList", lua.create_function(|lua, ()| lua.create_table())?)?;
    toi.set("GetSlotSourceID", lua.create_function(|_, (_id, _slot): (Value, Value)| Ok(0i32))?)?;
    toi.set("GetAllSlotLocationInfo", lua.create_function(|lua, ()| lua.create_table())?)?;
    g.set("C_TransmogOutfitInfo", toi)?;
    Ok(())
}

/// C_DamageMeter - damage/healing meter API.
fn register_c_damage_meter(lua: &Lua, g: &mlua::Table) -> Result<()> {
    let t = lua.create_table()?;
    t.set("IsDamageMeterAvailable", lua.create_function(|_, ()| Ok((false, Value::Nil)))?)?;
    t.set("GetAvailableCombatSessions", lua.create_function(|lua, ()| lua.create_table())?)?;
    t.set("GetCombatSessionFromID", lua.create_function(|_, _a: mlua::MultiValue| Ok(Value::Nil))?)?;
    t.set("GetCombatSessionFromType", lua.create_function(|_, _a: mlua::MultiValue| Ok(Value::Nil))?)?;
    t.set("GetCombatSessionSourceFromID", lua.create_function(|_, _a: mlua::MultiValue| Ok(Value::Nil))?)?;
    t.set("GetCombatSessionSourceFromType", lua.create_function(|_, _a: mlua::MultiValue| Ok(Value::Nil))?)?;
    t.set("GetSessionDurationSeconds", lua.create_function(|_, _st: Value| Ok(0.0f64))?)?;
    t.set("ResetAllCombatSessions", lua.create_function(|_, ()| Ok(()))?)?;
    g.set("C_DamageMeter", t)?;
    Ok(())
}

/// C_CombatText - combat floating text API.
fn register_c_combat_text(lua: &Lua, g: &mlua::Table) -> Result<()> {
    let t = lua.create_table()?;
    t.set("GetCurrentEventInfo", lua.create_function(|_, ()| Ok(Value::Nil))?)?;
    t.set("SetActiveUnit", lua.create_function(|_, _unit: Value| Ok(()))?)?;
    g.set("C_CombatText", t)?;
    Ok(())
}

/// C_CombatAudioAlert - combat audio alert system.
fn register_c_combat_audio_alert(lua: &Lua, g: &mlua::Table) -> Result<()> {
    let t = lua.create_table()?;
    t.set("IsEnabled", lua.create_function(|_, ()| Ok(false))?)?;
    t.set("GetCategoryVoice", lua.create_function(|_, _cat: Value| Ok(0i32))?)?;
    t.set("GetCategoryVolume", lua.create_function(|_, _cat: Value| Ok(1.0f64))?)?;
    t.set("GetFormatSetting", lua.create_function(|_, _a: mlua::MultiValue| Ok(0i32))?)?;
    t.set("GetSpeakerSpeed", lua.create_function(|_, ()| Ok(1.0f64))?)?;
    t.set("GetSpecSetting", lua.create_function(|_, _s: Value| Ok(0i32))?)?;
    t.set("GetThrottle", lua.create_function(|_, _t: Value| Ok(0.0f64))?)?;
    t.set("SetCategoryVoice", lua.create_function(|_, _a: mlua::MultiValue| Ok(()))?)?;
    t.set("SetCategoryVolume", lua.create_function(|_, _a: mlua::MultiValue| Ok(()))?)?;
    t.set("SetFormatSetting", lua.create_function(|_, _a: mlua::MultiValue| Ok(()))?)?;
    t.set("SetSpeakerSpeed", lua.create_function(|_, _s: Value| Ok(()))?)?;
    t.set("SetSpecSetting", lua.create_function(|_, _a: mlua::MultiValue| Ok(()))?)?;
    t.set("SetThrottle", lua.create_function(|_, _a: mlua::MultiValue| Ok(()))?)?;
    t.set("SpeakText", lua.create_function(|_, _text: Value| Ok(()))?)?;
    g.set("C_CombatAudioAlert", t)?;
    Ok(())
}

/// C_HousingPhotoSharing - housing screenshot sharing.
fn register_c_housing_photo_sharing(lua: &Lua, g: &mlua::Table) -> Result<()> {
    let t = lua.create_table()?;
    t.set("IsEnabled", lua.create_function(|_, ()| Ok(false))?)?;
    t.set("IsAuthorized", lua.create_function(|_, ()| Ok(true))?)?;
    t.set("BeginAuthorizationFlow", lua.create_function(|_, ()| Ok(()))?)?;
    t.set("ClearAuthorization", lua.create_function(|_, ()| Ok(()))?)?;
    t.set("CompleteAuthorizationFlow", lua.create_function(|_, _url: Value| Ok(()))?)?;
    t.set("GetCropRatio", lua.create_function(|_, ()| Ok(1.0f64))?)?;
    t.set("GetPhotoSharingAuthURL", lua.create_function(|_, ()| Ok(Value::Nil))?)?;
    t.set("SetScreenshotPreviewTexture", lua.create_function(|_, _tex: Value| Ok(()))?)?;
    t.set("UploadPhotoToService", lua.create_function(|_, _a: mlua::MultiValue| Ok(()))?)?;
    g.set("C_HousingPhotoSharing", t)?;
    Ok(())
}

/// Build the NamePlateConstants string cvar fields sub-table.
fn nameplate_cvar_fields(lua: &Lua) -> Result<mlua::Table> {
    let t = lua.create_table()?;
    t.raw_set("INFO_DISPLAY_CVAR", "nameplateInfoDisplay")?;
    t.raw_set("CAST_BAR_DISPLAY_CVAR", "nameplateCastBarDisplay")?;
    t.raw_set("THREAT_DISPLAY_CVAR", "nameplateThreatDisplay")?;
    t.raw_set("ENEMY_NPC_AURA_DISPLAY_CVAR", "nameplateEnemyNpcAuraDisplay")?;
    t.raw_set("ENEMY_PLAYER_AURA_DISPLAY_CVAR", "nameplateEnemyPlayerAuraDisplay")?;
    t.raw_set("FRIENDLY_PLAYER_AURA_DISPLAY_CVAR", "nameplateFriendlyPlayerAuraDisplay")?;
    t.raw_set("SHOW_DEBUFFS_ON_FRIENDLY_CVAR", "nameplateShowDebuffsOnFriendly")?;
    t.raw_set("DEBUFF_PADDING_CVAR", "nameplateDebuffPadding")?;
    t.raw_set("AURA_SCALE_CVAR", "nameplateAuraScale")?;
    t.raw_set("SIZE_CVAR", "nameplateSize")?;
    t.raw_set("STYLE_CVAR", "nameplateStyle")?;
    t.raw_set("SIMPLIFIED_TYPES_CVAR", "nameplateSimplifiedTypes")?;
    t.raw_set("SOFT_TARGET_NAMEPLATE_SIZE_CVAR", "SoftTargetNameplateSize")?;
    t.raw_set("SOFT_TARGET_ICON_ENEMY_CVAR", "SoftTargetIconEnemy")?;
    t.raw_set("SOFT_TARGET_ICON_FRIEND_CVAR", "SoftTargetIconFriend")?;
    t.raw_set("SOFT_TARGET_ICON_INTERACT_CVAR", "SoftTargetIconInteract")?;
    t.raw_set("SHOW_FRIENDLY_NPCS_CVAR", "nameplateShowFriendlyNpcs")?;
    t.raw_set("SHOW_ONLY_NAME_FOR_FRIENDLY_PLAYER_UNITS_CVAR", "nameplateShowOnlyNameForFriendlyPlayerUnits")?;
    t.raw_set("USE_CLASS_COLOR_FOR_FRIENDLY_PLAYER_UNIT_NAMES_CVAR", "nameplateUseClassColorForFriendlyPlayerUnitNames")?;
    t.raw_set("PREVIEW_UNIT_TOKEN", "preview")?;
    Ok(t)
}

/// Build the NamePlateConstants numeric fields sub-table.
fn nameplate_numeric_fields(lua: &Lua) -> Result<mlua::Table> {
    let t = lua.create_table()?;
    t.raw_set("AURA_ITEM_HEIGHT", 25_i32)?;
    t.raw_set("LARGE_HEALTH_BAR_HEIGHT", 20_i32)?;
    t.raw_set("SMALL_HEALTH_BAR_HEIGHT", 10_i32)?;
    t.raw_set("HEALTH_BAR_FONT_HEIGHT", 12_i32)?;
    t.raw_set("LARGE_CAST_BAR_HEIGHT", 16_i32)?;
    t.raw_set("SMALL_CAST_BAR_HEIGHT", 10_i32)?;
    t.raw_set("CAST_BAR_FONT_HEIGHT", 10_i32)?;
    t.raw_set("CAST_BAR_ICON_HEIGHT", 12_i32)?;
    let scales = lua.create_table()?;
    for (i, v) in [0.75f64, 1.0, 1.25, 1.5, 2.0].iter().enumerate() {
        scales.raw_set(i as i32 + 1, *v)?;
    }
    t.raw_set("NAME_PLATE_SCALES", scales)?;
    Ok(t)
}

/// NamePlateConstants - global constant table for nameplate system.
fn register_nameplate_constants(lua: &Lua) -> Result<()> {
    let t = nameplate_cvar_fields(lua)?;
    for pair in nameplate_numeric_fields(lua)?.pairs::<String, Value>() {
        let (k, v) = pair?;
        t.raw_set(k, v)?;
    }
    lua.globals().set("NamePlateConstants", t)?;
    Ok(())
}

/// C_DeathRecap - death recap data.
fn register_c_death_recap(lua: &Lua, g: &mlua::Table) -> Result<()> {
    let t = lua.create_table()?;
    t.set("HasRecapEvents", lua.create_function(|_, ()| Ok(false))?)?;
    t.set("GetRecapEvents", lua.create_function(|lua, _id: Value| lua.create_table())?)?;
    t.set("GetRecapMaxHealth", lua.create_function(|_, _id: Value| Ok(0i32))?)?;
    t.set("GetRecapLink", lua.create_function(|_, _id: Value| Ok(Value::Nil))?)?;
    g.set("C_DeathRecap", t)?;
    Ok(())
}

/// Constants.EncounterTimelineIconMasks - bitmask constants for timeline icon filtering.
fn register_encounter_timeline_constants(lua: &Lua, g: &mlua::Table) -> Result<()> {
    let constants: mlua::Table = match g.get("Constants")? {
        Value::Table(t) => t,
        _ => {
            let t = lua.create_table()?;
            g.set("Constants", t.clone())?;
            t
        }
    };
    let masks = lua.create_table()?;
    masks.set("EncounterTimelineTankAlertIcons", 1i32)?;
    masks.set("EncounterTimelineHealerAlertIcons", 2i32)?;
    masks.set("EncounterTimelineDamageAlertIcons", 4i32)?;
    masks.set("EncounterTimelineDeadlyIcons", 8i32)?;
    masks.set("EncounterTimelineDispelIcons", 16i32)?;
    masks.set("EncounterTimelineEnrageIcons", 32i32)?;
    masks.set("EncounterTimelineAllIcons", 63i32)?;
    constants.set("EncounterTimelineIconMasks", masks)?;
    Ok(())
}

/// C_EncounterTimeline - encounter timeline UI data (boss ability timers).
fn register_c_encounter_timeline(lua: &Lua, g: &mlua::Table) -> Result<()> {
    let t = lua.create_table()?;
    // Feature availability / state
    t.set("IsFeatureAvailable", lua.create_function(|_, ()| Ok(false))?)?;
    t.set("IsFeatureEnabled", lua.create_function(|_, ()| Ok(false))?)?;
    // Event queries
    t.set("GetEventList", lua.create_function(|lua, ()| lua.create_table())?)?;
    t.set("GetEventInfo", lua.create_function(|_, _event_id: Value| Ok(Value::Nil))?)?;
    t.set("GetEventState", lua.create_function(|_, _event_id: Value| Ok(Value::Nil))?)?;
    t.set("GetEventTimer", lua.create_function(|_, _event_id: Value| Ok(Value::Nil))?)?;
    t.set("GetEventTrack", lua.create_function(|_, _event_id: Value| Ok(Value::Nil))?)?;
    t.set("GetEventHighlightTime", lua.create_function(|_, ()| Ok(Value::Nil))?)?;
    t.set("GetEventTimeRemaining", lua.create_function(|_, _event_id: Value| Ok(0.0f64))?)?;
    t.set("IsEventBlocked", lua.create_function(|_, _event_id: Value| Ok(false))?)?;
    t.set("HasActiveEvents", lua.create_function(|_, ()| Ok(false))?)?;
    t.set("HasPausedEvents", lua.create_function(|_, ()| Ok(false))?)?;
    t.set("HasVisibleEvents", lua.create_function(|_, ()| Ok(false))?)?;
    register_c_encounter_timeline_extra(lua, &t)?;
    g.set("C_EncounterTimeline", t)?;
    Ok(())
}

/// C_EncounterTimeline continued - tracks, view, edit mode, and icon textures.
fn register_c_encounter_timeline_extra(lua: &Lua, t: &mlua::Table) -> Result<()> {
    t.set("GetTrackList", lua.create_function(|lua, ()| lua.create_table())?)?;
    t.set("GetTrackType", lua.create_function(|_, _track: Value| Ok(0i32))?)?;
    t.set("GetViewType", lua.create_function(|_, ()| Ok(0i32))?)?;
    t.set("SetViewType", lua.create_function(|_, _view_type: Value| Ok(()))?)?;
    t.set("GetCurrentTime", lua.create_function(|_, ()| Ok(0.0f64))?)?;
    t.set("AddEditModeEvents", lua.create_function(|_, ()| Ok(0.0f64))?)?;
    t.set("CancelEditModeEvents", lua.create_function(|_, ()| Ok(()))?)?;
    t.set("SetEventIconTextures", lua.create_function(|_, _args: mlua::MultiValue| Ok(()))?)?;
    Ok(())
}

/// Global-to-namespace alias pairs: (global_name, C_CombatLog method name).
const COMBAT_LOG_ALIASES: &[(&str, &str)] = &[
    ("CombatLogAddFilter", "AddEventFilter"),
    ("CombatLogGetCurrentEntry", "GetCurrentEntryInfo"),
    ("CombatLogGetCurrentEventInfo", "GetCurrentEventInfo"),
    ("CombatLogGetNumEntries", "GetEntryCount"),
    ("CombatLogAdvanceEntry", "AdvanceEntry"),
    ("CombatLogSetCurrentEntry", "SetCurrentEntry"),
    ("CombatLogShowCurrentEntry", "ShowCurrentEntry"),
    ("CombatLogResetFilter", "ResetFilter"),
    ("CombatLogClearEntries", "ClearEntries"),
    ("CombatLogGetRetentionTime", "GetRetentionTime"),
    ("CombatLogSetRetentionTime", "SetRetentionTime"),
];

/// Re-alias CombatLog* globals to the same function objects stored in C_CombatLog.
///
/// Wowless's cfuncs uniqueChecker requires that alias pairs (e.g. "C_CombatLog.AddEventFilter"
/// and "CombatLogAddFilter") share the same underlying C function pointer. This function
/// overwrites separately-created global stubs with direct references to the namespace functions.
/// Must be called after all CombatLog registrations complete.
pub fn fixup_combat_log_aliases(lua: &Lua, g: &mlua::Table) -> Result<()> {
    let Ok(cl) = g.get::<mlua::Table>("C_CombatLog") else { return Ok(()) };
    apply_aliases(lua, g, &cl, COMBAT_LOG_ALIASES)
}

/// Apply alias pairs from a namespace table to globals, with fallback no-op on missing methods.
fn apply_aliases(lua: &Lua, g: &mlua::Table, ns: &mlua::Table, pairs: &[(&str, &str)]) -> Result<()> {
    for &(global_name, method_name) in pairs {
        let f = match ns.get::<mlua::Function>(method_name) {
            Ok(f) => f,
            Err(_) => lua.create_function(|_, _: MultiValue| Ok(()))?,
        };
        g.set(global_name, f)?;
    }
    Ok(())
}
